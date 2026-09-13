use actix_web::body::to_bytes;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde_json::Value;

use super::error_response;
use crate::services::user::UserSettingsError;

async fn body_json(resp: HttpResponse) -> Value {
    let bytes = to_bytes(resp.into_body()).await.unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

#[actix_web::test]
async fn not_found_maps_to_404() {
    let resp = error_response(UserSettingsError::NotFound);
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    assert_eq!(body_json(resp).await["error"], "settings not found");
}

#[actix_web::test]
async fn internal_maps_to_500() {
    let resp = error_response(UserSettingsError::Internal);
    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(body_json(resp).await["error"], "internal server error");
}
