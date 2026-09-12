use extism_pdk::{http, HttpRequest};
use serde::de::DeserializeOwned;
use serde::Deserialize;

use crate::plugin::PluginCallError;

// ── Types ─────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct ProviderError {
    error: String,
}

// ── Private ───────────────────────────────────────────────────────────────────

fn describe_error(status: u16, body: &[u8]) -> PluginCallError {
    extism_pdk::error!("Provider returned status {status}: {}", String::from_utf8_lossy(body));

    let message = serde_json::from_slice::<ProviderError>(body)
        .map(|e| e.error)
        .unwrap_or_else(|_| format!("Provider returned status {status}"));

    PluginCallError::new(status, message)
}

// ── Public ────────────────────────────────────────────────────────────────────

pub fn fetch_json<T: DeserializeOwned>(url: &str) -> Result<T, PluginCallError> {
    let response = http::request::<()>(&HttpRequest::new(url), None)
        .map_err(|e| PluginCallError::new(502, format!("Failed to reach provider: {e}")))?;

    match response.status_code() {
        200 => response.json::<T>()
            .map_err(|e| PluginCallError::new(502, format!("Provider returned a malformed response: {e}"))),
        status => Err(describe_error(status, &response.body())),
    }
}

pub fn fetch_json_optional<T: DeserializeOwned>(url: &str) -> Result<Option<T>, PluginCallError> {
    let response = http::request::<()>(&HttpRequest::new(url), None)
        .map_err(|e| PluginCallError::new(502, format!("Failed to reach provider: {e}")))?;

    match response.status_code() {
        200 => response.json::<T>()
            .map(Some)
            .map_err(|e| PluginCallError::new(502, format!("Provider returned a malformed response: {e}"))),
        404 => Ok(None),
        status => Err(describe_error(status, &response.body())),
    }
}
