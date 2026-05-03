use actix_web::{get, web, HttpResponse, Responder};

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "status": "ok" }))
}

pub fn public_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health);
}

