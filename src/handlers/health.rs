use actix_web::{get, web::ServiceConfig, HttpResponse, Responder};
use serde_json::json;

// ── Handlers ──────────────────────────────────────────────────────────────────

/// `GET /health`
#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(json!({ "status": "ok" }))
}

// ── Public ────────────────────────────────────────────────────────────────────

pub fn public_routes(cfg: &mut ServiceConfig) {
    cfg.service(health);
}
