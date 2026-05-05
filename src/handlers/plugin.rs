use actix_web::{delete, get, patch, post, HttpRequest, HttpResponse, Responder};
use actix_web::web::{Data, Json, Path, ServiceConfig};
use serde::Deserialize;
use serde_json::json;

use crate::auth::user_id;
use crate::services::plugin::{self as plugin_service, PluginError};
use crate::state::AppState;

// ── Types ─────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct InstallRequest {
    plugin_id: String,
}

#[derive(Deserialize)]
struct SetEnabledRequest {
    enabled: bool,
}

// ── Private ───────────────────────────────────────────────────────────────────

fn error_response(err: PluginError) -> HttpResponse {
    match err {
        PluginError::AlreadyInstalled =>
            HttpResponse::Conflict().json(json!({ "error": "plugin already installed" })),
        PluginError::NotFound =>
            HttpResponse::NotFound().json(json!({ "error": "plugin not found" })),
        PluginError::Internal =>
            HttpResponse::InternalServerError().json(json!({ "error": "internal server error" })),
    }
}

// ── Handlers ──────────────────────────────────────────────────────────────────

/// `GET /plugins/all`
#[get("/plugins/all")]
async fn list_all(state: Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(plugin_service::list_all(&state.plugins))
}

/// `GET /api/plugins`
#[get("/plugins")]
async fn list(req: HttpRequest, state: Data<AppState>) -> impl Responder {
    let user_id = assert_ok!(user_id(&req));

    match plugin_service::list(&state.db, &user_id).await {
        Ok(plugins) => HttpResponse::Ok().json(plugins),
        Err(e) => error_response(e),
    }
}

/// `POST /api/plugins`
///
/// Body: `{ "plugin_id": "abc" }`
#[post("/plugins")]
async fn install(req: HttpRequest, state: Data<AppState>, body: Json<InstallRequest>) -> impl Responder {
    let user_id = assert_ok!(user_id(&req));

    match plugin_service::install(&state.db, &user_id, &body.plugin_id).await {
        Ok(plugin) => HttpResponse::Created().json(plugin),
        Err(e) => error_response(e),
    }
}

/// `DELETE /api/plugins/{plugin_id}`
#[delete("/plugins/{plugin_id}")]
async fn uninstall(req: HttpRequest, state: Data<AppState>, path: Path<String>) -> impl Responder {
    let plugin_id = path.into_inner();
    let user_id = assert_ok!(user_id(&req));

    match plugin_service::uninstall(&state.db, &user_id, &plugin_id).await {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(e) => error_response(e),
    }
}

/// `PATCH /api/plugins/{plugin_id}`
///
/// Body: `{ "enabled": true }`
#[patch("/plugins/{plugin_id}")]
async fn set_enabled(req: HttpRequest, state: Data<AppState>, path: Path<String>, body: Json<SetEnabledRequest>) -> impl Responder {
    let plugin_id = path.into_inner();
    let user_id = assert_ok!(user_id(&req));

    match plugin_service::set_enabled(&state.db, &user_id, &plugin_id, body.enabled).await {
        Ok(plugin) => HttpResponse::Ok().json(plugin),
        Err(e) => error_response(e),
    }
}

// ── Public ────────────────────────────────────────────────────────────────────

pub fn public_routes(cfg: &mut ServiceConfig) {
    cfg.service(list_all);
}

pub fn protected_routes(cfg: &mut ServiceConfig) {
    cfg
        .service(list)
        .service(install)
        .service(uninstall)
        .service(set_enabled);
}
