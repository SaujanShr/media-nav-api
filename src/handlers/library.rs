use actix_web::{delete, get, post, HttpRequest, HttpResponse, Responder};
use actix_web::web::{Data, Json, Path, ServiceConfig, scope};
use serde::Deserialize;
use serde_json::json;

use crate::auth::user_id;
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

/// `GET /api/library/{user_plugin_id}/items`
#[get("/{user_plugin_id}/items")]
async fn list(req: HttpRequest, state: Data<AppState>, path: Path<String>) -> impl Responder {
    let user_plugin_id = path.into_inner();
    let user_id = assert_ok!(user_id(&req));

    match library_item_service::list(&state.db, &user_plugin_id, &user_id).await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(e) => error_response(e),
    }
}

/// `POST /api/library/{user_plugin_id}/items`
///
/// Body: `{ "library_item_id": "abc" }`
#[post("/{user_plugin_id}/items")]
async fn add(req: HttpRequest, state: Data<AppState>, path: Path<String>, body: Json<AddRequest>) -> impl Responder {
    let user_plugin_id = path.into_inner();
    let user_id = assert_ok!(user_id(&req));

    match library_item_service::add(&state.db, &user_plugin_id, &user_id, &body.library_item_id).await {
        Ok(item) => HttpResponse::Created().json(item),
        Err(e) => error_response(e),
    }
}

/// `DELETE /api/library/{user_plugin_id}/items/{library_item_id}`
#[delete("/{user_plugin_id}/items/{library_item_id}")]
async fn remove(req: HttpRequest, state: Data<AppState>, path: Path<(String, String)>) -> impl Responder {
    let (user_plugin_id, library_item_id) = path.into_inner();
    let user_id = assert_ok!(user_id(&req));

    match library_item_service::remove(&state.db, &user_plugin_id, &user_id, &library_item_id).await {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(e) => error_response(e),
    }
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
