use actix_web::{
    get, post, Responder,
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
        AuthError::Internal =>
            HttpResponse::InternalServerError().json(json!({ "error": "internal server error" })),
    }
}

// ── Handlers ──────────────────────────────────────────────────────────────────

/// `POST /auth/register`
///
/// Body: `{ "username": "alice", "password": "secret" }`
#[post("/register")]
async fn register(state: Data<AppState>, body: Json<AuthRequest>) -> impl Responder {
    match auth_service::register(&state.db, &body.username, &body.password, &state.jwt_secret).await {
        Ok(result) => HttpResponse::Created().json(result),
        Err(e) => error_response(e),
    }
}

/// `POST /auth/login`
///
/// Body: `{ "username": "alice", "password": "secret" }`
#[post("/login")]
async fn login(state: Data<AppState>, body: Json<AuthRequest>) -> impl Responder {
    match auth_service::login(&state.db, &body.username, &body.password, &state.jwt_secret).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => error_response(e),
    }
}

/// `GET /api/auth/me`
#[get("/me")]
async fn me(req: HttpRequest) -> impl Responder {
    let claims = assert_ok!(extractor::claims(&req));
    HttpResponse::Ok().json(claims)
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
        scope("/auth")
            .service(me),
    );
}
