use actix_web::{delete, get, patch, post, HttpRequest, HttpResponse, Responder};
use actix_web::http::StatusCode;
use actix_web::web::{Data, Json, Path, ServiceConfig, scope};
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
        PluginError::ValidationError(msg) =>
            HttpResponse::BadRequest().json(json!({ "error": msg })),
        PluginError::UpstreamError(err) => {
            let status = StatusCode::from_u16(err.status).unwrap_or(StatusCode::BAD_GATEWAY);
            HttpResponse::build(status).json(json!({ "error": err.message }))
        }
        PluginError::Internal =>
            HttpResponse::InternalServerError().json(json!({ "error": "internal server error" })),
    }
}

// ── Handlers ──────────────────────────────────────────────────────────────────

#[get("/all")]
async fn list_all(state: Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(plugin_service::list_all(&state.plugins))
}

#[get("")]
async fn list(req: HttpRequest, state: Data<AppState>) -> impl Responder {
    let user_id = assert_ok!(user_id(&req));

    plugin_service::list(&state.db, &user_id)
        .await
        .map(|plugins| HttpResponse::Ok().json(plugins))
        .unwrap_or_else(error_response)
}

#[post("")]
async fn install(req: HttpRequest, state: Data<AppState>, body: Json<InstallRequest>) -> impl Responder {
    let user_id = assert_ok!(user_id(&req));

    plugin_service::install(&state.db, &state.plugins, &user_id, &body.plugin_id)
        .await
        .map(|plugin| HttpResponse::Created().json(plugin))
        .unwrap_or_else(error_response)
}

#[delete("/{plugin_id}")]
async fn uninstall(req: HttpRequest, state: Data<AppState>, path: Path<String>) -> impl Responder {
    let plugin_id = path.into_inner();
    let user_id = assert_ok!(user_id(&req));

    plugin_service::uninstall(&state.db, &user_id, &plugin_id)
        .await
        .map(|_| HttpResponse::NoContent().finish())
        .unwrap_or_else(error_response)
}

#[patch("/{plugin_id}")]
async fn set_enabled(req: HttpRequest, state: Data<AppState>, path: Path<String>, body: Json<SetEnabledRequest>) -> impl Responder {
    let plugin_id = path.into_inner();
    let user_id = assert_ok!(user_id(&req));

    plugin_service::set_enabled(&state.db, &user_id, &plugin_id, body.enabled)
        .await
        .map(|plugin| HttpResponse::Ok().json(plugin))
        .unwrap_or_else(error_response)
}

// ── Public ────────────────────────────────────────────────────────────────────

pub fn public_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/plugins")
            .service(list_all),
    );
}

pub fn protected_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/plugins")
            .service(list)
            .service(install)
            .service(uninstall)
            .service(set_enabled),
    );
}
