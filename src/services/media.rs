use plugin_sdk::library_item::LibraryItemDetail;
use plugin_sdk::media_item::Media;
use plugin_sdk::plugin::{FetchRequest, FetchResult};
use plugin_sdk::query::Query;

use crate::plugins::PluginRegistry;
use crate::services::plugin::PluginError;

// ── Public ────────────────────────────────────────────────────────────────────

pub fn fetch(
    registry: &PluginRegistry,
    plugin_id: &str,
    page: u32,
    page_size: u32,
    query: Query,
) -> Result<FetchResult, PluginError> {
    let plugin = registry
        .get(plugin_id)
        .ok_or(PluginError::NotFound)?;

    Ok((plugin.fetch)(FetchRequest { page, page_size, query }))
}

pub fn enrich(
    registry: &PluginRegistry,
    plugin_id: &str,
    item_id: &str,
) -> Result<Option<LibraryItemDetail>, PluginError> {
    let plugin = registry
        .get(plugin_id)
        .ok_or(PluginError::NotFound)?;

    Ok((plugin.enrich)(item_id))
}

pub fn media(
    registry: &PluginRegistry,
    plugin_id: &str,
    library_item_id: &str,
) -> Result<Vec<Media>, PluginError> {
    let plugin = registry
        .get(plugin_id)
        .ok_or(PluginError::NotFound)?;

    Ok((plugin.media)(library_item_id))
}

