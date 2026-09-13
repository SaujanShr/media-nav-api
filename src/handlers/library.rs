use actix_web::{delete, get, post, HttpRequest, HttpResponse, Responder};
use actix_web::web::{Data, Json, Path, ServiceConfig, scope};
use serde::Deserialize;
use serde_json::json;

use crate::auth::user_id;
use crate::services::library_item::{self as library_item_service, LibraryItemError};
use crate::state::AppState;

#[cfg(test)]
#[path = "tests/library.rs"]
mod tests;

// ── Types ─────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct AddRequest {
    library_item_id: String,
}

// ── Private ───────────────────────────────────────────────────────────────────

fn error_response(err: LibraryItemError) -> HttpResponse {
    match err {
        LibraryItemError::PluginNotFound =>
            HttpResponse::NotFound().json(json!({ "error": "plugin not found" })),
        LibraryItemError::Forbidden =>
            HttpResponse::Forbidden().json(json!({ "error": "plugin does not belong to you" })),
        LibraryItemError::AlreadyAdded =>
            HttpResponse::Conflict().json(json!({ "error": "library item already added" })),
        LibraryItemError::NotFound =>
            HttpResponse::NotFound().json(json!({ "error": "library item not found" })),
        LibraryItemError::Internal =>
            HttpResponse::InternalServerError().json(json!({ "error": "internal server error" })),
    }
}

// ── Handlers ──────────────────────────────────────────────────────────────────

#[get("/{user_plugin_id}/items")]
async fn list(req: HttpRequest, state: Data<AppState>, path: Path<String>) -> impl Responder {
    let user_plugin_id = path.into_inner();
    let user_id = assert_ok!(user_id(&req));

    library_item_service::list(&state.db, &user_plugin_id, &user_id)
        .await
        .map(|items| HttpResponse::Ok().json(items))
        .unwrap_or_else(error_response)
}

#[post("/{user_plugin_id}/items")]
async fn add(req: HttpRequest, state: Data<AppState>, path: Path<String>, body: Json<AddRequest>) -> impl Responder {
    let user_plugin_id = path.into_inner();
    let user_id = assert_ok!(user_id(&req));

    library_item_service::add(&state.db, &user_plugin_id, &user_id, &body.library_item_id)
        .await
        .map(|item| HttpResponse::Created().json(item))
        .unwrap_or_else(error_response)
}

#[delete("/{user_plugin_id}/items/{library_item_id}")]
async fn remove(req: HttpRequest, state: Data<AppState>, path: Path<(String, String)>) -> impl Responder {
    let (user_plugin_id, library_item_id) = path.into_inner();
    let user_id = assert_ok!(user_id(&req));

    library_item_service::remove(&state.db, &user_plugin_id, &user_id, &library_item_id)
        .await
        .map(|_| HttpResponse::NoContent().finish())
        .unwrap_or_else(error_response)
}

// ── Public ────────────────────────────────────────────────────────────────────

pub fn protected_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/library")
            .service(list)
            .service(add)
            .service(remove),
    );
}
