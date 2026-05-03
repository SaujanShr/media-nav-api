use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;

use crate::auth::Claims;
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
            HttpResponse::Conflict().json(serde_json::json!({ "error": "username already taken" })),
        AuthError::InvalidCredentials =>
            HttpResponse::Unauthorized().json(serde_json::json!({ "error": "invalid credentials" })),
        AuthError::Internal =>
            HttpResponse::InternalServerError().json(serde_json::json!({ "error": "internal server error" })),
    }
}

/// `POST /auth/register`
///
/// Body: `{ "username": "alice", "password": "secret" }`
#[post("/auth/register")]
async fn register(state: web::Data<AppState>, body: web::Json<AuthRequest>) -> impl Responder {
    match auth_service::register(&state.db, &body.username, &body.password, &state.jwt_secret).await {
        Ok(result) => HttpResponse::Created().json(result),
        Err(e) => error_response(e),
    }
}

/// `POST /auth/login`
///
/// Body: `{ "username": "alice", "password": "secret" }`
#[post("/auth/login")]
async fn login(state: web::Data<AppState>, body: web::Json<AuthRequest>) -> impl Responder {
    match auth_service::login(&state.db, &body.username, &body.password, &state.jwt_secret).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => error_response(e),
    }
}

/// `GET /auth/me` – returns the currently authenticated user's info.
#[get("/me")]
async fn me(req: HttpRequest) -> impl Responder {
    let extensions = req.extensions();
    let claims = extensions.get::<Claims>().unwrap();

    HttpResponse::Ok().json(serde_json::json!({
        "user_id": claims.sub,
        "username": claims.username,
    }))
}

// ── Public ────────────────────────────────────────────────────────────────────

/// Registers public (unauthenticated) auth routes.
pub fn public_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(register)
        .service(login);
}

/// Registers protected (authenticated) auth routes.
/// Mount these inside the app's protected scope.
pub fn protected_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(me);
}
