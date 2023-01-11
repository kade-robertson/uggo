use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, HeaderValue, Method, StatusCode},
    response::{AppendHeaders, IntoResponse, Json},
    routing::get,
    Router, Server,
};
use config::get_config;
use http_cache_reqwest::{Cache, CacheMode, HttpCache, MokaCache, MokaManager};
use reqwest::header::{HeaderName, CACHE_CONTROL, ETAG, LAST_MODIFIED};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use serde::Deserialize;
use std::{net::SocketAddr, sync::Arc};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};
use ugg_types::{mappings, matchups::Matchups, overview::ChampOverview};

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
    vec![CACHE_CONTROL, ETAG, LAST_MODIFIED]
        .iter()
        .filter_map(|header| {
            headers
                .get(header)
                .map(|value| (header.clone(), value.clone()))
        })
        .collect()
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
    let actual_mode: mappings::Mode = mode.as_str().into();

    let overview_data_response = state
        .client
        .get(format!(
            "https://stats2.u.gg/lol/1.5/overview/{}/{}/{}/{}.json",
            patch,
            actual_mode.to_api_string(),
            champ,
            api_version
        ))
        .send()
        .await
        .map_err(|e| {
            tracing::error!("Could not fetch overview data: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not fetch overview data.".to_owned(),
            )
        })?;

    let response_headers = get_cache_headers(overview_data_response.headers());

    let overview_data = overview_data_response
        .json::<ChampOverview>()
        .await
        .map_err(|e| {
            tracing::error!("Could not parse overview data: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not parse overview data.".to_owned(),
            )
        })?;

    let region_query = if overview_data.contains_key(&region) {
        region
    } else {
        mappings::Region::World
    };

    let rank_query = if overview_data[&region_query].contains_key(&mappings::Rank::PlatinumPlus) {
        mappings::Rank::PlatinumPlus
    } else {
        mappings::Rank::Overall
    };

    let mut role_query = role;
    if !overview_data[&region_query][&rank_query].contains_key(&role_query) {
        if role_query == mappings::Role::Automatic {
            // Go through each role and pick the one with most matches played
            let mut most_games = 0;
            let mut used_role = role;
            for (role_key, role_stats) in &overview_data[&region_query][&rank_query] {
                if role_stats.data.matches > most_games {
                    most_games = role_stats.data.matches;
                    used_role = *role_key;
                }
            }
            role_query = used_role;
        } else {
            // This should only happen in ARAM
            role_query = mappings::Role::None;
        }
    }

    let overview = overview_data
        .get(&region_query)
        .ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Could not find data for region {:}.", region_query),
            )
        })?
        .get(&rank_query)
        .ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Could not find data for rank {:}.", rank_query),
            )
        })?
        .get(&role_query)
        .ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Could not find data for role {:}.", role),
            )
        })?;

    Ok((AppendHeaders(response_headers), Json(overview.data.clone())))
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
    let actual_mode: mappings::Mode = mode.as_str().into();

    let matchup_data_response = state
        .client
        .get(format!(
            "https://stats2.u.gg/lol/1.5/matchups/{}/{}/{}/{}.json",
            patch,
            actual_mode.to_api_string(),
            champ,
            api_version
        ))
        .send()
        .await
        .map_err(|e| {
            tracing::error!("Could not fetch matchup data: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not fetch matchup data.".to_owned(),
            )
        })?;

    let response_headers = get_cache_headers(matchup_data_response.headers());

    let matchup_data = matchup_data_response
        .json::<Matchups>()
        .await
        .map_err(|e| {
            tracing::error!("Could not parse matchup data: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not parse matchup data.".to_owned(),
            )
        })?;

    let region_query = if matchup_data.contains_key(&region) {
        region
    } else {
        mappings::Region::World
    };

    let rank_query = if matchup_data[&region_query].contains_key(&mappings::Rank::PlatinumPlus) {
        mappings::Rank::PlatinumPlus
    } else {
        mappings::Rank::Overall
    };

    let mut role_query = role;
    if !matchup_data[&region_query][&rank_query].contains_key(&role_query) {
        if role_query == mappings::Role::Automatic {
            // Go through each role and pick the one with most matches played
            let mut most_games = 0;
            let mut used_role = role;
            for (role_key, role_stats) in &matchup_data[&region_query][&rank_query] {
                if role_stats.data.total_matches > most_games {
                    most_games = role_stats.data.total_matches;
                    used_role = *role_key;
                }
            }
            role_query = used_role;
        } else {
            // This should only happen in ARAM
            role_query = mappings::Role::None;
        }
    }

    let matchup = matchup_data
        .get(&region_query)
        .ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Could not find data for region {:}.", region_query),
            )
        })?
        .get(&rank_query)
        .ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Could not find data for rank {:}.", rank_query),
            )
        })?
        .get(&role_query)
        .ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Could not find data for role {:}.", role),
            )
        })?;

    Ok((AppendHeaders(response_headers), Json(matchup.data.clone())))
}
