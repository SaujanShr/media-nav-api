use actix_web::body::to_bytes;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde_json::Value;

use plugin_sdk::plugin::PluginCallError;

use super::error_response;
use crate::services::plugin::PluginError;

async fn body_json(resp: HttpResponse) -> Value {
    let bytes = to_bytes(resp.into_body()).await.unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

#[actix_web::test]
async fn already_installed_maps_to_409_here() {
    let resp = error_response(PluginError::AlreadyInstalled);
    assert_eq!(resp.status(), StatusCode::CONFLICT);
    assert_eq!(body_json(resp).await["error"], "plugin already installed");
}

#[actix_web::test]
async fn not_found_maps_to_404() {
    let resp = error_response(PluginError::NotFound);
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    assert_eq!(body_json(resp).await["error"], "plugin not found");
}

#[actix_web::test]
async fn validation_error_maps_to_400_with_its_message() {
    let resp = error_response(PluginError::ValidationError("plugin_id is required".to_string()));
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(body_json(resp).await["error"], "plugin_id is required");
}

#[actix_web::test]
async fn upstream_error_passes_through_the_status_and_message() {
    let resp = error_response(PluginError::UpstreamError(PluginCallError::new(503, "provider down")));
    assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(body_json(resp).await["error"], "provider down");
}

#[actix_web::test]
async fn upstream_error_falls_back_to_502_for_an_invalid_status_code() {
    let resp = error_response(PluginError::UpstreamError(PluginCallError::new(0, "malformed")));
    assert_eq!(resp.status(), StatusCode::BAD_GATEWAY);
}

#[actix_web::test]
async fn internal_maps_to_500() {
    let resp = error_response(PluginError::Internal);
    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(body_json(resp).await["error"], "internal server error");
}
