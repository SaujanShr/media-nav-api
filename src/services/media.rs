use actix_web::web;

use plugin_sdk::library_item::LibraryItemDetail;
use plugin_sdk::plugin::{FetchRequest, FetchResult};
use plugin_sdk::query::schema::QuerySchema;
use plugin_sdk::query::Query;

use crate::plugins::{wasm, PluginRegistry};
use crate::services::plugin::PluginError;

// ── Private ───────────────────────────────────────────────────────────────────

fn as_internal<T>(result: Result<T, wasm::CallError>) -> Result<T, PluginError> {
    result.map_err(|e| {
        tracing::error!("{}", e);
        PluginError::Internal
    })
}

// ── Public ────────────────────────────────────────────────────────────────────

pub fn schema(registry: &PluginRegistry, plugin_id: &str) -> Result<QuerySchema, PluginError> {
    Ok(registry.get(plugin_id).ok_or(PluginError::NotFound)?.schema.clone())
}

pub async fn fetch(
    registry: &PluginRegistry,
    plugin_id: &str,
    page: u32,
    page_size: u32,
    query: Query,
) -> Result<FetchResult, PluginError> {
    let plugin = registry.get(plugin_id).ok_or(PluginError::NotFound)?;

    plugin.schema.validate(&query).map_err(|errors| {
        let message = errors
            .into_iter()
            .map(|e| format!("{}: {}", e.field, e.message))
            .collect::<Vec<_>>()
            .join("; ");
        
        PluginError::ValidationError(message)
    })?;

    let pool = plugin.pool.clone();
    let request = FetchRequest { page, page_size, query };

    let call_result = web::block(move || wasm::fetch(&pool, &request))
        .await
        .map_err(|e| {
            tracing::error!("Plugin fetch task panicked: {}", e);
            PluginError::Internal
        })?;

    as_internal(call_result)?.map_err(PluginError::UpstreamError)
}

pub async fn enrich(
    registry: &PluginRegistry,
    plugin_id: &str,
    item_id: &str,
) -> Result<Option<LibraryItemDetail>, PluginError> {
    let pool = registry
        .get(plugin_id)
        .ok_or(PluginError::NotFound)?
        .pool
        .clone();

    let item_id = item_id.to_string();

    let call_result = web::block(move || wasm::enrich(&pool, &item_id))
        .await
        .map_err(|e| {
            tracing::error!("Plugin enrich task panicked: {}", e);
            PluginError::Internal
        })?;

    as_internal(call_result)?.map_err(PluginError::UpstreamError)
}
