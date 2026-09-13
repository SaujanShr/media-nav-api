use actix_web::body::to_bytes;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde_json::Value;

use super::error_response;
use crate::services::library_item::LibraryItemError;

async fn body_json(resp: HttpResponse) -> Value {
    let bytes = to_bytes(resp.into_body()).await.unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

#[actix_web::test]
async fn plugin_not_found_maps_to_404() {
    let resp = error_response(LibraryItemError::PluginNotFound);
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    assert_eq!(body_json(resp).await["error"], "plugin not found");
}

#[actix_web::test]
async fn forbidden_maps_to_403() {
    let resp = error_response(LibraryItemError::Forbidden);
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    assert_eq!(body_json(resp).await["error"], "plugin does not belong to you");
}

#[actix_web::test]
async fn already_added_maps_to_409() {
    let resp = error_response(LibraryItemError::AlreadyAdded);
    assert_eq!(resp.status(), StatusCode::CONFLICT);
    assert_eq!(body_json(resp).await["error"], "library item already added");
}

#[actix_web::test]
async fn not_found_maps_to_404() {
    let resp = error_response(LibraryItemError::NotFound);
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    assert_eq!(body_json(resp).await["error"], "library item not found");
}

#[actix_web::test]
async fn internal_maps_to_500() {
    let resp = error_response(LibraryItemError::Internal);
    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(body_json(resp).await["error"], "internal server error");
}
