use actix_web::{get, put, HttpRequest, HttpResponse, Responder};
use actix_web::web::{Data, Json, ServiceConfig, scope};
use serde::Deserialize;
use serde_json::json;

use crate::auth::extractor;
use crate::models::user::Theme;
use crate::services::user::{self as user_settings_service, UserSettingsError};
use crate::state::AppState;

#[cfg(test)]
#[path = "tests/settings.rs"]
mod tests;

// ── Types ─────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct UpdateSettingsRequest {
    pub nsfw_enabled: Option<bool>,
    pub theme: Option<Theme>,
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

#[get("")]
async fn get_settings(state: Data<AppState>, req: HttpRequest) -> impl Responder {
    let claims = assert_ok!(extractor::claims(&req));

    user_settings_service::get(&state.db, &claims.sub)
        .await
        .map(|settings| HttpResponse::Ok().json(settings))
        .unwrap_or_else(error_response)
}

#[put("")]
async fn update_settings(state: Data<AppState>, req: HttpRequest, body: Json<UpdateSettingsRequest>) -> impl Responder {
    let claims = assert_ok!(extractor::claims(&req));

    user_settings_service::update(&state.db, &claims.sub, body.into_inner())
        .await
        .map(|settings| HttpResponse::Ok().json(settings))
        .unwrap_or_else(error_response)
}

// ── Public ────────────────────────────────────────────────────────────────────

pub fn protected_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/settings")
            .service(get_settings)
            .service(update_settings),
    );
}
