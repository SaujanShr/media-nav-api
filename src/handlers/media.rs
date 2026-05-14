use actix_web::{get, post, HttpResponse, Responder};
use actix_web::web::{Data, Json, Path, Query, ServiceConfig, scope};
use actix_governor::{Governor, GovernorConfigBuilder};
use serde::Deserialize;
use serde_json::json;

use plugin_sdk::query::Query as PluginQuery;

use crate::services::media as media_service;
use crate::services::plugin::PluginError;
use crate::state::AppState;

// ── Config ────────────────────────────────────────────────────────────────────

const REQUESTS_PER_MINUTE: u64 = 60;

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
        _ =>
            HttpResponse::InternalServerError().json(json!({ "error": "internal server error" })),
    }
}

// ── Handlers ──────────────────────────────────────────────────────────────────

/// `POST /media/{plugin_id}/fetch?page={page}&pageSize={page_size}`
///
/// Body (optional): `{ "query": { "search_fields": { "search": "foo" }, ... } }`
#[post("/{plugin_id}/fetch")]
async fn fetch(
    state: Data<AppState>,
    path: Path<String>,
    qs: Query<FetchQuery>,
    body: Json<FetchBody>,
) -> impl Responder {
    let plugin_id = path.into_inner();

    match media_service::fetch(
        &state.plugins,
        &plugin_id,
        qs.page,
        qs.page_size,
        body.into_inner().query,
    ) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => error_response(e),
    }
}

/// `GET /api/media/{plugin_id}/enrich/{item_id}`
#[get("/{plugin_id}/enrich/{item_id}")]
async fn enrich(
    state: Data<AppState>,
    path: Path<(String, String)>,
) -> impl Responder {
    let (plugin_id, item_id) = path.into_inner();

    match media_service::enrich(&state.plugins, &plugin_id, &item_id) {
        Ok(Some(detail)) => HttpResponse::Ok().json(detail),
        Ok(None) => HttpResponse::NotFound().json(json!({ "error": "item not found" })),
        Err(e) => error_response(e),
    }
}

/// `GET /api/media/{plugin_id}/library/{library_item_id}/media
#[get("/{plugin_id}/library/{library_item_id}/media")]
async fn media(
    state: Data<AppState>,
    path: Path<(String, String)>,
) -> impl Responder {
    let (plugin_id, library_item_id) = path.into_inner();

    match media_service::media(&state.plugins, &plugin_id, &library_item_id) {
        Ok(media) => HttpResponse::Ok().json(media),
        Err(e) => error_response(e),
    }
}

// ── Public ────────────────────────────────────────────────────────────────────

pub fn public_routes(cfg: &mut ServiceConfig) {
    let governor_conf = GovernorConfigBuilder::default()
        .requests_per_minute(REQUESTS_PER_MINUTE)
        .finish()
        .unwrap();

    cfg.service(
        scope("/media")
            .wrap(Governor::new(&governor_conf))
            .service(fetch)
            .service(enrich)
            .service(media),
    );
}

