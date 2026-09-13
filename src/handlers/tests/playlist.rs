use actix_web::body::to_bytes;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde_json::Value;

use super::error_response;
use crate::services::playlist::PlaylistError;

async fn body_json(resp: HttpResponse) -> Value {
    let bytes = to_bytes(resp.into_body()).await.unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

#[actix_web::test]
async fn not_found_maps_to_404() {
    let resp = error_response(PlaylistError::NotFound);
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    assert_eq!(body_json(resp).await["error"], "not found");
}

#[actix_web::test]
async fn forbidden_maps_to_403() {
    let resp = error_response(PlaylistError::Forbidden);
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    assert_eq!(body_json(resp).await["error"], "access denied");
}

#[actix_web::test]
async fn already_added_maps_to_409() {
    let resp = error_response(PlaylistError::AlreadyAdded);
    assert_eq!(resp.status(), StatusCode::CONFLICT);
    assert_eq!(body_json(resp).await["error"], "item already in playlist");
}

#[actix_web::test]
async fn validation_error_maps_to_400_with_its_message() {
    let resp = error_response(PlaylistError::ValidationError("name cannot be empty".to_string()));
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(body_json(resp).await["error"], "name cannot be empty");
}

#[actix_web::test]
async fn internal_maps_to_500() {
    let resp = error_response(PlaylistError::Internal);
    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(body_json(resp).await["error"], "internal server error");
}
