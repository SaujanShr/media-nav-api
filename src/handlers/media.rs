use std::sync::OnceLock;

use actix_web::{get, post, HttpResponse, Responder};
use actix_web::http::StatusCode;
use actix_web::web::{Data, Json, Path, Query, ServiceConfig, scope};
use actix_governor::governor::middleware::NoOpMiddleware;
use actix_governor::{Governor, GovernorConfig, GovernorConfigBuilder, PeerIpKeyExtractor};
use serde::Deserialize;
use serde_json::json;

use plugin_sdk::query::Query as PluginQuery;

use crate::services::media as media_service;
use crate::services::plugin::PluginError;
use crate::state::AppState;

// ── Config ────────────────────────────────────────────────────────────────────

const REQUESTS_PER_MINUTE: u64 = 60;

fn governor_conf() -> &'static GovernorConfig<PeerIpKeyExtractor, NoOpMiddleware> {
    static CONF: OnceLock<GovernorConfig<PeerIpKeyExtractor, NoOpMiddleware>> = OnceLock::new();

    CONF.get_or_init(|| {
        GovernorConfigBuilder::default()
            .requests_per_minute(REQUESTS_PER_MINUTE)
            .finish()
            .unwrap()
    })
}

// ── Types ─────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct FetchQuery {
    page:      u32,
    #[serde(rename = "pageSize")]
    page_size: u32,
}

#[derive(Deserialize)]
struct FetchBody {
    #[serde(default)]
    query: PluginQuery,
}

// ── Private ───────────────────────────────────────────────────────────────────

fn error_response(err: PluginError) -> HttpResponse {
    match err {
        PluginError::NotFound =>
            HttpResponse::NotFound().json(json!({ "error": "plugin not found" })),
        PluginError::ValidationError(msg) =>
            HttpResponse::BadRequest().json(json!({ "error": msg })),
        PluginError::UpstreamError(err) => {
            let status = StatusCode::from_u16(err.status).unwrap_or(StatusCode::BAD_GATEWAY);
            HttpResponse::build(status).json(json!({ "error": err.message }))
        }
        PluginError::AlreadyInstalled | PluginError::Internal =>
            HttpResponse::InternalServerError().json(json!({ "error": "internal server error" })),
    }
}

// ── Handlers ──────────────────────────────────────────────────────────────────

/// Body (optional): `{ "query": { "search_fields": { "search": "foo" }, ... } }`
#[post("/{plugin_id}/fetch")]
async fn fetch(
    state: Data<AppState>,
    path: Path<String>,
    qs: Query<FetchQuery>,
    body: Json<FetchBody>,
) -> impl Responder {
    let plugin_id = path.into_inner();

    media_service::fetch(&state.plugins, &plugin_id, qs.page, qs.page_size, body.into_inner().query)
        .await
        .map(|result| HttpResponse::Ok().json(result))
        .unwrap_or_else(error_response)
}

#[get("/{plugin_id}/schema")]
async fn schema(state: Data<AppState>, path: Path<String>) -> impl Responder {
    let plugin_id = path.into_inner();

    media_service::schema(&state.plugins, &plugin_id)
        .map(|schema| HttpResponse::Ok().json(schema))
        .unwrap_or_else(error_response)
}

#[get("/{plugin_id}/enrich/{item_id}")]
async fn enrich(
    state: Data<AppState>,
    path: Path<(String, String)>,
) -> impl Responder {
    let (plugin_id, item_id) = path.into_inner();

    media_service::enrich(&state.plugins, &plugin_id, &item_id)
        .await
        .map(|detail| match detail {
            Some(d) => HttpResponse::Ok().json(d),
            None => HttpResponse::NotFound().json(json!({ "error": "item not found" })),
        })
        .unwrap_or_else(error_response)
}

// ── Public ────────────────────────────────────────────────────────────────────

pub fn public_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/media")
            .wrap(Governor::new(governor_conf()))
            .service(schema)
            .service(fetch)
            .service(enrich),
    );
}
