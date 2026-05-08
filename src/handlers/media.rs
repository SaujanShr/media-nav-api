use actix_web::{get, post, HttpRequest, HttpResponse, Responder};
use actix_web::web::{Data, Json, Path, Query, ServiceConfig};
use serde::Deserialize;
use serde_json::json;

use plugin_sdk::query::Query as PluginQuery;

use crate::auth::user_id;
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

/// `POST /api/media/{plugin_id}/fetch?page={page}&pageSize={page_size}`
///
/// Body (optional): `{ "query": { "search_fields": { "search": "foo" }, ... } }`
#[post("/media/{plugin_id}/fetch")]
async fn fetch(
    req: HttpRequest,
    state: Data<AppState>,
    path: Path<String>,
    qs: Query<FetchQuery>,
    body: Json<FetchBody>,
) -> impl Responder {
    assert_ok!(user_id(&req));
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
#[get("/media/{plugin_id}/enrich/{item_id}")]
async fn enrich(
    req: HttpRequest,
    state: Data<AppState>,
    path: Path<(String, String)>,
) -> impl Responder {
    assert_ok!(user_id(&req));
    let (plugin_id, item_id) = path.into_inner();

    match plugin_service::enrich(&state.plugins, &plugin_id, &item_id) {
        Ok(Some(detail)) => HttpResponse::Ok().json(detail),
        Ok(None) => HttpResponse::NotFound().json(json!({ "error": "item not found" })),
        Err(e) => error_response(e),
    }
}

// ── Public ────────────────────────────────────────────────────────────────────

pub fn protected_routes(cfg: &mut ServiceConfig) {
    cfg
        .service(fetch)
        .service(enrich);
}

