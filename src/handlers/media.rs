use actix_web::{get, post, HttpResponse, Responder};
use actix_web::web::{Data, Json, Path, Query, ServiceConfig, scope};
use actix_governor::{Governor, GovernorConfigBuilder};
use serde::Deserialize;
use serde_json::json;

use plugin_sdk::query::Query as PluginQuery;

use crate::services::plugin::{self as plugin_service, PluginError};
use crate::state::AppState;

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

    match plugin_service::fetch(
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

    match plugin_service::enrich(&state.plugins, &plugin_id, &item_id) {
        Ok(Some(detail)) => HttpResponse::Ok().json(detail),
        Ok(None) => HttpResponse::NotFound().json(json!({ "error": "item not found" })),
        Err(e) => error_response(e),
    }
}

// ── Public ────────────────────────────────────────────────────────────────────

pub fn public_routes(cfg: &mut ServiceConfig) {
    let governor_conf = GovernorConfigBuilder::default()
        .requests_per_minute(60)
        .finish()
        .unwrap();

    cfg.service(
        scope("/media")
            .wrap(Governor::new(&governor_conf))
            .service(fetch)
            .service(enrich),
    );
}

