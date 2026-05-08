use actix_web::{get, put, HttpRequest, HttpResponse, Responder};
use actix_web::web::{Data, Json, ServiceConfig};
use serde::Deserialize;
use serde_json::json;

use crate::auth::extractor;
use crate::services::user::{self as user_settings_service, UserSettingsError};
use crate::state::AppState;

// ── Types ─────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct SetNsfwRequest {
    enabled: bool,
}

// ── Private ───────────────────────────────────────────────────────────────────

fn error_response(err: UserSettingsError) -> HttpResponse {
    match err {
        UserSettingsError::NotFound =>
            HttpResponse::NotFound().json(json!({ "error": "settings not found" })),
        UserSettingsError::Internal =>
            HttpResponse::InternalServerError().json(json!({ "error": "internal server error" })),
    }
}

// ── Handlers ──────────────────────────────────────────────────────────────────

/// `GET /api/settings`
#[get("/settings")]
async fn get_settings(state: Data<AppState>, req: HttpRequest) -> impl Responder {
    let claims = assert_ok!(extractor::claims(&req));

    match user_settings_service::get(&state.db, &claims.sub).await {
        Ok(settings) => HttpResponse::Ok().json(settings),
        Err(e) => error_response(e),
    }
}

/// `PUT /api/settings/nsfw`
///
/// Body: `{ "enabled": true }`
#[put("/settings/nsfw")]
async fn set_nsfw(state: Data<AppState>, req: HttpRequest, body: Json<SetNsfwRequest>) -> impl Responder {
    let claims = assert_ok!(extractor::claims(&req));

    match user_settings_service::set_nsfw_enabled(&state.db, &claims.sub, body.enabled).await {
        Ok(settings) => HttpResponse::Ok().json(settings),
        Err(e) => error_response(e),
    }
}

// ── Public ────────────────────────────────────────────────────────────────────

pub fn protected_routes(cfg: &mut ServiceConfig) {
    cfg
        .service(get_settings)
        .service(set_nsfw);
}
