use actix_web::{
    delete, get, post, Responder,
    HttpRequest, HttpResponse
};
use actix_web::web::{Data, Json, ServiceConfig, scope};
use actix_governor::{Governor, GovernorConfigBuilder};
use serde::Deserialize;
use serde_json::json;

use crate::auth::extractor;
use crate::services::auth::{self as auth_service, AuthError};
use crate::state::AppState;

// ── Config ────────────────────────────────────────────────────────────────────

const REQUESTS_PER_MINUTE: u64 = 10;

// ── Types ─────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct AuthRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct DeleteAccountRequest {
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

#[post("/register")]
async fn register(state: Data<AppState>, body: Json<AuthRequest>) -> impl Responder {
    auth_service::register(&state.db, &body.username, &body.password, &state.jwt_secret)
        .await
        .map(|result| HttpResponse::Created().json(result))
        .unwrap_or_else(error_response)
}

#[post("/login")]
async fn login(state: Data<AppState>, body: Json<AuthRequest>) -> impl Responder {
    auth_service::login(&state.db, &body.username, &body.password, &state.jwt_secret)
        .await
        .map(|result| HttpResponse::Ok().json(result))
        .unwrap_or_else(error_response)
}

#[get("")]
async fn me(req: HttpRequest) -> impl Responder {
    let claims = assert_ok!(extractor::claims(&req));
    HttpResponse::Ok().json(claims)
}

#[delete("")]
async fn delete(req: HttpRequest, state: Data<AppState>, body: Json<DeleteAccountRequest>) -> impl Responder {
    let claims = assert_ok!(extractor::claims(&req));

    auth_service::delete(&state.db, &claims.sub, &body.password)
        .await
        .map(|_| HttpResponse::NoContent().finish())
        .unwrap_or_else(error_response)
}

// ── Public ────────────────────────────────────────────────────────────────────

pub fn public_routes(cfg: &mut ServiceConfig) {
    let governor_conf = GovernorConfigBuilder::default()
        .requests_per_minute(REQUESTS_PER_MINUTE)
        .finish()
        .unwrap();

    cfg.service(
        scope("/account")
            .wrap(Governor::new(&governor_conf))
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
