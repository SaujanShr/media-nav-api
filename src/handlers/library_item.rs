use actix_web::{delete, get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;

use crate::auth::Claims;
use crate::services::library_item::{self as library_item_service, LibraryItemError};
use crate::state::AppState;

// ── Types ─────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct AddRequest {
    library_item_id: String,
}

// ── Private ───────────────────────────────────────────────────────────────────

fn error_response(err: LibraryItemError) -> HttpResponse {
    match err {
        LibraryItemError::PluginNotFound =>
            HttpResponse::NotFound().json(serde_json::json!({ "error": "plugin not found" })),
        LibraryItemError::Forbidden =>
            HttpResponse::Forbidden().json(serde_json::json!({ "error": "plugin does not belong to you" })),
        LibraryItemError::AlreadyAdded =>
            HttpResponse::Conflict().json(serde_json::json!({ "error": "library item already added" })),
        LibraryItemError::NotFound =>
            HttpResponse::NotFound().json(serde_json::json!({ "error": "library item not found" })),
        LibraryItemError::Internal =>
            HttpResponse::InternalServerError().json(serde_json::json!({ "error": "internal server error" })),
    }
}

fn user_id(req: &HttpRequest) -> String {
    req.extensions().get::<Claims>().unwrap().sub.clone()
}

/// `GET /api/plugins/{user_plugin_id}/items` – list library items for a plugin.
#[get("/plugins/{user_plugin_id}/items")]
async fn list(req: HttpRequest, state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let user_plugin_id = path.into_inner();
    match library_item_service::list(&state.db, &user_plugin_id, &user_id(&req)).await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(e) => error_response(e),
    }
}

/// `POST /api/plugins/{user_plugin_id}/items` – add a library item to a plugin.
///
/// Body: `{ "library_item_id": "abc" }`
#[post("/plugins/{user_plugin_id}/items")]
async fn add(req: HttpRequest, state: web::Data<AppState>, path: web::Path<String>, body: web::Json<AddRequest>) -> impl Responder {
    let user_plugin_id = path.into_inner();
    match library_item_service::add(&state.db, &user_plugin_id, &user_id(&req), &body.library_item_id).await {
        Ok(item) => HttpResponse::Created().json(item),
        Err(e) => error_response(e),
    }
}

/// `DELETE /api/plugins/{user_plugin_id}/items/{library_item_id}` – remove a library item.
#[delete("/plugins/{user_plugin_id}/items/{library_item_id}")]
async fn remove(req: HttpRequest, state: web::Data<AppState>, path: web::Path<(String, String)>) -> impl Responder {
    let (user_plugin_id, library_item_id) = path.into_inner();
    match library_item_service::remove(&state.db, &user_plugin_id, &user_id(&req), &library_item_id).await {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(e) => error_response(e),
    }
}

// ── Route config ──────────────────────────────────────────────────────────────

/// Registers protected library item routes. Mount inside the app's protected scope.
pub fn protected_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(list)
        .service(add)
        .service(remove);
}

