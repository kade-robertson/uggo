use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, HeaderValue, Method, StatusCode},
    response::{AppendHeaders, IntoResponse, Json},
    routing::get,
    Router, Server,
};
use config::get_config;
use http_cache_reqwest::{Cache, CacheMode, HttpCache, MokaCache, MokaManager};
use reqwest::header::{HeaderName, AGE, CACHE_CONTROL, ETAG, LAST_MODIFIED};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use serde::{de::DeserializeOwned, Deserialize};
use std::{net::SocketAddr, sync::Arc};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};
use ugg_types::{
    mappings,
    matchups::{GroupedMatchupData, Matchups, WrappedMatchupData},
    nested_data::{GroupedData, NestedData},
    overview::{ChampOverview, GroupedOverviewData, WrappedOverviewData},
};

mod config;

#[derive(Clone)]
struct AppState {
    client: Arc<ClientWithMiddleware>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = get_config();

    let logger = tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(config.log_level))
        .with(tracing_subscriber::fmt::layer());
    logger.init();

    let state = AppState {
        client: Arc::new(
            ClientBuilder::new(reqwest::Client::new())
                .with(Cache(HttpCache {
                    mode: CacheMode::Default,
                    manager: MokaManager::new(MokaCache::new(500)),
                    options: None,
                }))
                .build(),
        ),
    };

    let app = Router::new()
        .route(
            "/:patch/:mode/:champ/:api_version/overview.json",
            get(overview),
        )
        .route(
            "/:patch/:mode/:champ/:api_version/matchups.json",
            get(matchups),
        )
        .with_state(state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(
                    DefaultMakeSpan::new()
                        .level(Level::INFO)
                        .include_headers(false),
                )
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(
            CorsLayer::new()
                .allow_headers(Any)
                .allow_methods(vec![Method::GET])
                .allow_origin(Any),
        );

    let socket = SocketAddr::new(config.bind_address.parse()?, config.bind_port);

    tracing::info!("Starting server on {socket}");
    Server::bind(&socket).serve(app.into_make_service()).await?;

    Ok(())
}

#[derive(Deserialize)]
struct UggParams {
    patch: String,
    mode: String,
    champ: String,
    api_version: String,
}

#[derive(Deserialize)]
struct UggOptions {
    #[serde(default)]
    region: mappings::Region,

    #[serde(default)]
    role: mappings::Role,
}

fn get_cache_headers(headers: &HeaderMap) -> Vec<(HeaderName, HeaderValue)> {
    vec![CACHE_CONTROL, ETAG, LAST_MODIFIED, AGE]
        .iter()
        .filter_map(|header| {
            headers
                .get(header)
                .map(|value| (header.clone(), value.clone()))
        })
        .collect()
}

async fn retrieve_from_ugg<
    V: DeserializeOwned,
    G: DeserializeOwned + GroupedData<V>,
    T: DeserializeOwned + NestedData<serde_json::Value>,
>(
    State(state): State<AppState>,
    kind: &str,
    data_path: &str,
    region: mappings::Region,
    role: mappings::Role,
) -> Result<(Vec<(HeaderName, HeaderValue)>, V), (StatusCode, String)> {
    let ugg_response = state
        .client
        .get(format!(
            "https://stats2.u.gg/lol/1.5/{}/{}.json",
            kind, data_path
        ))
        .send()
        .await
        .map_err(|e| {
            tracing::error!("Could not fetch {} data: {}", kind, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Could not fetch {} data.", kind),
            )
        })?;

    let response_headers = get_cache_headers(ugg_response.headers());

    let json_data = ugg_response.json::<T>().await.map_err(|e| {
        tracing::error!("Could not parse {} data: {}", kind, e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Could not parse {} data.", kind),
        )
    })?;

    let region_query = if json_data.is_region_valid(&region) {
        region
    } else {
        mappings::Region::World
    };

    let rank_query = if json_data.is_rank_valid(&region_query, &mappings::Rank::PlatinumPlus) {
        mappings::Rank::PlatinumPlus
    } else {
        mappings::Rank::Overall
    };

    let grouped_data = if let Some(d) = json_data.get_wrapped_data(&region_query, &rank_query) {
        serde_json::from_value::<G>(d).map_err(|e| {
            tracing::error!("Could not parse grouped {} data: {}", kind, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Could not parse grouped {} data.", kind),
            )
        })?
    } else {
        tracing::error!("Could not parse grouped {} data.", kind);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Could not parse grouped {} data.", kind),
        ));
    };

    let mut role_query = role;
    if !grouped_data.is_role_valid(&role_query) {
        if role_query == mappings::Role::Automatic {
            // Go through each role and pick the one with most matches played
            role_query = grouped_data.get_most_popular_role().unwrap_or(role_query)
        } else {
            // This should only happen in ARAM
            role_query = mappings::Role::None;
        }
    }

    if let Some(wrapped) = grouped_data.get_wrapped_data(&role_query) {
        Ok((response_headers, wrapped))
    } else {
        Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Could not find {} data for your query.", kind),
        ))
    }
}

async fn overview(
    Path(UggParams {
        patch,
        mode,
        champ,
        api_version,
    }): Path<UggParams>,
    Query(UggOptions { region, role }): Query<UggOptions>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let actual_mode = mappings::Mode::from_api_string(&mode).to_api_string();
    let data_path = format!("{}/{}/{}/{}", patch, actual_mode, champ, api_version);

    let (headers, wrapped) = retrieve_from_ugg::<
        WrappedOverviewData,
        GroupedOverviewData,
        ChampOverview,
    >(State(state), "overview", &data_path, region, role)
    .await?;

    Ok((AppendHeaders(headers), Json(wrapped.data)))
}

async fn matchups(
    Path(UggParams {
        patch,
        mode,
        champ,
        api_version,
    }): Path<UggParams>,
    Query(UggOptions { region, role }): Query<UggOptions>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let actual_mode = mappings::Mode::from_api_string(&mode).to_api_string();
    let data_path = format!("{}/{}/{}/{}", patch, actual_mode, champ, api_version);

    let (headers, wrapped) = retrieve_from_ugg::<WrappedMatchupData, GroupedMatchupData, Matchups>(
        State(state),
        "matchups",
        &data_path,
        region,
        role,
    )
    .await?;

    Ok((AppendHeaders(headers), Json(wrapped.data)))
}
