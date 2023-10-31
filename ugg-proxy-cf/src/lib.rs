use reqwest::{
    header::{HeaderMap, HeaderValue, AGE, CACHE_CONTROL, ETAG, LAST_MODIFIED},
    StatusCode,
};
use serde::de::DeserializeOwned;
use ugg_types::{
    mappings::{self, Region, Role},
    matchups::{GroupedMatchupData, Matchups, WrappedMatchupData},
    nested_data::{GroupedData, NestedData},
    overview::{ChampOverview, GroupedOverviewData, WrappedOverviewData},
};
use worker::*;

fn get_cache_headers(headers: &HeaderMap) -> Headers {
    let mut worker_headers = Headers::new();

    for header in [CACHE_CONTROL, ETAG, LAST_MODIFIED, AGE] {
        if let Some(Ok(value)) = headers.get(&header).map(HeaderValue::to_str) {
            let _ = worker_headers.set(header.as_str(), value);
        }
    }

    worker_headers
}

async fn retrieve_from_ugg<
    V: DeserializeOwned,
    G: DeserializeOwned + GroupedData<V>,
    T: DeserializeOwned + NestedData<serde_json::Value>,
>(
    kind: &str,
    data_path: &str,
    region: mappings::Region,
    role: mappings::Role,
) -> core::result::Result<(Headers, V), Result<Response>> {
    console_log!("1");
    let ugg_response = reqwest::get(format!(
        "https://stats2.u.gg/lol/1.5/{kind}/{data_path}.json"
    ))
    .await
    .map_err(|e| {
        console_error!("Could not fetch {} data: {}", kind, e);
        Response::error(
            format!("Could not fetch {kind} data."),
            StatusCode::INTERNAL_SERVER_ERROR.into(),
        )
    })?;

    console_log!("2");
    let response_headers = get_cache_headers(ugg_response.headers());

    console_log!("3");

    let json_data = ugg_response.json::<T>().await.map_err(|e| {
        console_error!("Could not parse {} data: {}", kind, e);
        Response::error(
            format!("Could not parse {kind} data."),
            StatusCode::INTERNAL_SERVER_ERROR.into(),
        )
    })?;

    console_log!("4");
    let region_query = if json_data.is_region_valid(&region) {
        region
    } else {
        mappings::Region::World
    };

    console_log!("5");
    let rank_query = if json_data.is_rank_valid(&region_query, &mappings::Rank::PlatinumPlus) {
        mappings::Rank::PlatinumPlus
    } else {
        mappings::Rank::Overall
    };

    console_log!("6");
    let grouped_data = if let Some(d) = json_data.get_wrapped_data(&region_query, &rank_query) {
        serde_json::from_value::<G>(d).map_err(|e| {
            console_error!("Could not parse grouped {} data: {}", kind, e);
            Response::error(
                format!("Could not parse grouped {kind} data."),
                StatusCode::INTERNAL_SERVER_ERROR.into(),
            )
        })?
    } else {
        console_error!("Could not parse grouped {} data.", kind);
        return Err(Response::error(
            format!("Could not parse grouped {kind} data."),
            StatusCode::INTERNAL_SERVER_ERROR.into(),
        ));
    };

    console_log!("7");
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

    console_log!("8");
    if let Some(wrapped) = grouped_data.get_wrapped_data(&role_query) {
        Ok((response_headers, wrapped))
    } else {
        console_log!("8-e");
        Err(Response::error(
            format!("Could not find {kind} data for your query."),
            StatusCode::INTERNAL_SERVER_ERROR.into(),
        ))
    }
}

fn get_region_and_role(url: Url) -> (Region, Role) {
    let region = url
        .query_pairs()
        .find(|(k, _)| k == "region")
        .map(|(_, v)| v.parse::<Region>());
    let role = url
        .query_pairs()
        .find(|(k, _)| k == "role")
        .map(|(_, v)| v.parse::<Role>());

    (
        region.unwrap_or(Ok(Region::World)).unwrap_or(Region::World),
        role.unwrap_or(Ok(Role::Automatic))
            .unwrap_or(Role::Automatic),
    )
}

async fn retrieve_handler<
    V: DeserializeOwned + serde::Serialize,
    G: DeserializeOwned + GroupedData<V>,
    T: DeserializeOwned + NestedData<serde_json::Value>,
>(
    req: Request,
    ctx: RouteContext<()>,
    kind: &str,
) -> Result<Response> {
    console_log!("got to the handler {:?}", req.url());
    if let (Some(patch), Some(mode), Some(champ), Some(api_version)) = (
        ctx.param("patch"),
        ctx.param("mode"),
        ctx.param("champ"),
        ctx.param("api_version"),
    ) {
        console_log!("hi {} {} {} {}", patch, mode, champ, api_version);
        let (region, role) = get_region_and_role(req.url().unwrap());

        let data_path = format!("{patch}/{mode}/{champ}/{api_version}");
        let cache_key = format!("{kind}/{data_path}/{region}/{role}");

        if let Ok(Some(cached)) = Cache::default().get(&cache_key, false).await {
            return Ok(cached);
        }

        let (headers, wrapped) = retrieve_from_ugg::<V, G, T>(kind, &data_path, region, role)
            .await
            .unwrap();

        let mut r = Response::from_json(&wrapped).map(|r| r.with_headers(headers))?;

        if let Ok(cloned) = r.cloned() {
            let _ = Cache::default().put(cache_key, cloned).await;
        }

        Ok(r)
    } else {
        console_error!("failed to parse url");
        Response::error("Missing parameters", 500)
    }
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    Router::new()
        .get_async(
            "/:patch/:mode/:champ/:api_version/overview.json",
            |req, ctx| {
                retrieve_handler::<WrappedOverviewData, GroupedOverviewData, ChampOverview>(
                    req, ctx, "overview",
                )
            },
        )
        .get_async(
            "/:patch/:mode/:champ/:api_version/matchups.json",
            |req, ctx| {
                retrieve_handler::<WrappedMatchupData, GroupedMatchupData, Matchups>(
                    req, ctx, "overview",
                )
            },
        )
        .run(req, env)
        .await
}
