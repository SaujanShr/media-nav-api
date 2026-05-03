use actix_web::{delete, get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;

use crate::auth::Claims;
use crate::services::plugin::{self as plugin_service, PluginError};
use crate::state::AppState;

// ── Types ─────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct InstallRequest {
    plugin_id: String,
    version_id: String,
}

// ── Private ───────────────────────────────────────────────────────────────────

fn error_response(err: PluginError) -> HttpResponse {
    match err {
        PluginError::AlreadyInstalled =>
            HttpResponse::Conflict().json(serde_json::json!({ "error": "plugin already installed" })),
        PluginError::NotFound =>
            HttpResponse::NotFound().json(serde_json::json!({ "error": "plugin not installed" })),
        PluginError::Internal =>
            HttpResponse::InternalServerError().json(serde_json::json!({ "error": "internal server error" })),
    }
}

fn user_id(req: &HttpRequest) -> String {
    req.extensions().get::<Claims>().unwrap().sub.clone()
}

/// `GET /api/plugins` – list all installed plugins for the current user.
#[get("/plugins")]
async fn list(req: HttpRequest, state: web::Data<AppState>) -> impl Responder {
    match plugin_service::list(&state.db, &user_id(&req)).await {
        Ok(plugins) => HttpResponse::Ok().json(plugins),
        Err(e) => error_response(e),
    }
}

/// `POST /api/plugins` – install a plugin.
///
/// Body: `{ "plugin_id": "abc", "version_id": "1.0.0" }`
#[post("/plugins")]
async fn install(req: HttpRequest, state: web::Data<AppState>, body: web::Json<InstallRequest>) -> impl Responder {
    match plugin_service::install(&state.db, &user_id(&req), &body.plugin_id, &body.version_id).await {
        Ok(plugin) => HttpResponse::Created().json(plugin),
        Err(e) => error_response(e),
    }
}

/// `DELETE /api/plugins/{plugin_id}` – uninstall a plugin.
#[delete("/plugins/{plugin_id}")]
async fn uninstall(req: HttpRequest, state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let plugin_id = path.into_inner();
    match plugin_service::uninstall(&state.db, &user_id(&req), &plugin_id).await {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(e) => error_response(e),
    }
}

// ── Route config ──────────────────────────────────────────────────────────────

/// Registers protected plugin routes. Mount inside the app's protected scope.
pub fn protected_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(list)
        .service(install)
        .service(uninstall);
}

