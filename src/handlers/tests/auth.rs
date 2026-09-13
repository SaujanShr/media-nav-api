use actix_web::body::to_bytes;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde_json::Value;

use super::error_response;
use crate::services::auth::AuthError;

async fn body_json(resp: HttpResponse) -> Value {
    let bytes = to_bytes(resp.into_body()).await.unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

#[actix_web::test]
async fn username_taken_maps_to_409() {
    let resp = error_response(AuthError::UsernameTaken);
    assert_eq!(resp.status(), StatusCode::CONFLICT);
    assert_eq!(body_json(resp).await["error"], "username already taken");
}

#[actix_web::test]
async fn invalid_credentials_maps_to_401() {
    let resp = error_response(AuthError::InvalidCredentials);
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(body_json(resp).await["error"], "invalid credentials");
}

#[actix_web::test]
async fn validation_error_maps_to_400_with_its_message() {
    let resp = error_response(AuthError::ValidationError("username too short".to_string()));
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(body_json(resp).await["error"], "username too short");
}

#[actix_web::test]
async fn internal_maps_to_500() {
    let resp = error_response(AuthError::Internal);
    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(body_json(resp).await["error"], "internal server error");
}
