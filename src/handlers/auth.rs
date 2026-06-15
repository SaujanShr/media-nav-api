use actix_web::{
    delete, get, post, Responder,
    HttpRequest, HttpResponse
};
use actix_web::web::{Data, Json, ServiceConfig, scope};
use serde::Deserialize;
use serde_json::json;

use crate::auth::extractor;
use crate::services::auth::{self as auth_service, AuthError};
use crate::state::AppState;

// ── Types ─────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct AuthRequest {
    username: String,
    password: String,
}

// ── Private ───────────────────────────────────────────────────────────────────

fn error_response(err: AuthError) -> HttpResponse {
    match err {
        AuthError::UsernameTaken =>
            HttpResponse::Conflict().json(json!({ "error": "username already taken" })),
        AuthError::InvalidCredentials =>
            HttpResponse::Unauthorized().json(json!({ "error": "invalid credentials" })),
        AuthError::ValidationError(msg) =>
            HttpResponse::BadRequest().json(json!({ "error": msg })),
        AuthError::Internal =>
            HttpResponse::InternalServerError().json(json!({ "error": "internal server error" })),
    }
}

// ── Handlers ──────────────────────────────────────────────────────────────────

/// `POST /auth/register`
#[post("/register")]
async fn register(state: Data<AppState>, body: Json<AuthRequest>) -> impl Responder {
    auth_service::register(&state.db, &body.username, &body.password, &state.jwt_secret)
        .await
        .map(|result| HttpResponse::Created().json(result))
        .unwrap_or_else(error_response)
}

/// `POST /auth/login`
#[post("/login")]
async fn login(state: Data<AppState>, body: Json<AuthRequest>) -> impl Responder {
    auth_service::login(&state.db, &body.username, &body.password, &state.jwt_secret)
        .await
        .map(|result| HttpResponse::Ok().json(result))
        .unwrap_or_else(error_response)
}

/// `GET /api/account`
#[get("")]
async fn me(req: HttpRequest) -> impl Responder {
    let claims = assert_ok!(extractor::claims(&req));
    HttpResponse::Ok().json(claims)
}

/// `DELETE /api/account`
#[delete("")]
async fn delete(req: HttpRequest, state: Data<AppState>) -> impl Responder {
    let claims = assert_ok!(extractor::claims(&req));

    auth_service::delete(&state.db, &claims.sub)
        .await
        .map(|_| HttpResponse::NoContent().finish())
        .unwrap_or_else(error_response)
}

// ── Public ────────────────────────────────────────────────────────────────────

pub fn public_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/auth")
            .service(register)
            .service(login),
    );
}

pub fn protected_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/account")
            .service(me)
            .service(delete),
    );
}
