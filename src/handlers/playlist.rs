use actix_web::{delete, get, patch, post, HttpRequest, HttpResponse, Responder};
use actix_web::web::{Data, Json, Path, ServiceConfig, scope};
use serde::Deserialize;
use serde_json::json;

use crate::auth::user_id;
use crate::services::playlist::{self as playlist_service, PlaylistError};
use crate::state::AppState;

#[cfg(test)]
#[path = "tests/playlist.rs"]
mod tests;

// ── Types ─────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct CreateRequest {
    name: String,
}

#[derive(Deserialize)]
struct RenameRequest {
    name: String,
}

#[derive(Deserialize)]
struct AddItemRequest {
    user_library_item_id: String,
}

#[derive(Deserialize)]
struct MoveItemRequest {
    /// 0-based index the item should occupy in the final list.
    /// Items currently at or after this index are pushed back.
    index: usize,
}

// ── Private ───────────────────────────────────────────────────────────────────

fn error_response(err: PlaylistError) -> HttpResponse {
    match err {
        PlaylistError::NotFound =>
            HttpResponse::NotFound().json(json!({ "error": "not found" })),
        PlaylistError::Forbidden =>
            HttpResponse::Forbidden().json(json!({ "error": "access denied" })),
        PlaylistError::AlreadyAdded =>
            HttpResponse::Conflict().json(json!({ "error": "item already in playlist" })),
        PlaylistError::ValidationError(msg) =>
            HttpResponse::BadRequest().json(json!({ "error": msg })),
        PlaylistError::Internal =>
            HttpResponse::InternalServerError().json(json!({ "error": "internal server error" })),
    }
}

// ── Handlers ──────────────────────────────────────────────────────────────────

#[get("")]
async fn list(req: HttpRequest, state: Data<AppState>) -> impl Responder {
    let user_id = assert_ok!(user_id(&req));

    playlist_service::list(&state.db, &user_id)
        .await
        .map(|playlists| HttpResponse::Ok().json(playlists))
        .unwrap_or_else(error_response)
}

#[post("")]
async fn create(req: HttpRequest, state: Data<AppState>, body: Json<CreateRequest>) -> impl Responder {
    let user_id = assert_ok!(user_id(&req));

    playlist_service::create(&state.db, &user_id, &body.name)
        .await
        .map(|playlist| HttpResponse::Created().json(playlist))
        .unwrap_or_else(error_response)
}

#[delete("/{playlist_id}")]
async fn delete(req: HttpRequest, state: Data<AppState>, path: Path<String>) -> impl Responder {
    let playlist_id = path.into_inner();
    let user_id = assert_ok!(user_id(&req));

    playlist_service::delete(&state.db, &user_id, &playlist_id)
        .await
        .map(|_| HttpResponse::NoContent().finish())
        .unwrap_or_else(error_response)
}

#[patch("/{playlist_id}")]
async fn rename(req: HttpRequest, state: Data<AppState>, path: Path<String>, body: Json<RenameRequest>) -> impl Responder {
    let playlist_id = path.into_inner();
    let user_id = assert_ok!(user_id(&req));

    playlist_service::rename(&state.db, &user_id, &playlist_id, &body.name)
        .await
        .map(|playlist| HttpResponse::Ok().json(playlist))
        .unwrap_or_else(error_response)
}

#[get("/{playlist_id}/items")]
async fn list_items(req: HttpRequest, state: Data<AppState>, path: Path<String>) -> impl Responder {
    let playlist_id = path.into_inner();
    let user_id = assert_ok!(user_id(&req));

    playlist_service::list_items(&state.db, &user_id, &playlist_id)
        .await
        .map(|items| HttpResponse::Ok().json(items))
        .unwrap_or_else(error_response)
}

#[post("/{playlist_id}/items")]
async fn add_item(
    req: HttpRequest,
    state: Data<AppState>,
    path: Path<String>,
    body: Json<AddItemRequest>,
) -> impl Responder {
    let playlist_id = path.into_inner();
    let user_id = assert_ok!(user_id(&req));

    playlist_service::add_item(&state.db, &user_id, &playlist_id, &body.user_library_item_id)
        .await
        .map(|item| HttpResponse::Created().json(item))
        .unwrap_or_else(error_response)
}

#[delete("/{playlist_id}/items/{user_library_item_id}")]
async fn remove_item(
    req: HttpRequest,
    state: Data<AppState>,
    path: Path<(String, String)>,
) -> impl Responder {
    let (playlist_id, user_library_item_id) = path.into_inner();
    let user_id = assert_ok!(user_id(&req));

    playlist_service::remove_item(&state.db, &user_id, &playlist_id, &user_library_item_id)
        .await
        .map(|_| HttpResponse::NoContent().finish())
        .unwrap_or_else(error_response)
}

#[patch("/{playlist_id}/items/{item_id}")]
async fn move_item(
    req: HttpRequest,
    state: Data<AppState>,
    path: Path<(String, String)>,
    body: Json<MoveItemRequest>,
) -> impl Responder {
    let (playlist_id, item_id) = path.into_inner();
    let user_id = assert_ok!(user_id(&req));

    playlist_service::move_item(&state.db, &user_id, &playlist_id, &item_id, body.index)
        .await
        .map(|item| HttpResponse::Ok().json(item))
        .unwrap_or_else(error_response)
}

// ── Public ────────────────────────────────────────────────────────────────────

pub fn protected_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/playlists")
            .service(list)
            .service(create)
            .service(delete)
            .service(rename)
            .service(list_items)
            .service(add_item)
            .service(remove_item)
            .service(move_item),
    );
}
