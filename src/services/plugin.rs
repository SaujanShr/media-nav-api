use sqlx::PgPool;
use uuid::Uuid;

use plugin_sdk::library_item::LibraryItemDetail;
use plugin_sdk::plugin::{FetchRequest, FetchResult, Plugin};
use plugin_sdk::query::Query;

use crate::models::plugin::UserPlugin;
use crate::plugins::{PluginRegistry};
use crate::repositories::plugin as plugin_repo;

// ── Types ─────────────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum PluginError {
    AlreadyInstalled,
    NotFound,
    Internal,
}

// ── Public ────────────────────────────────────────────────────────────────────

pub fn list_all(registry: &PluginRegistry) -> Vec<&Plugin> {
    registry.plugins().collect()
}

pub fn enrich(
    registry: &PluginRegistry,
    plugin_id: &str,
    item_id: &str,
) -> Result<Option<LibraryItemDetail>, PluginError> {
    let plugin = registry
        .get(plugin_id)
        .ok_or(PluginError::NotFound)?;

    let result = (plugin.enrich)(
        item_id
    );

    Ok(result)
}

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

    let result = (plugin.fetch)(FetchRequest { page, page_size, query });

    Ok(result)
}

pub async fn list(pool: &PgPool, user_id: &str) -> Result<Vec<UserPlugin>, PluginError> {
    plugin_repo::list(pool, user_id)
        .await
        .map_err(|_| PluginError::Internal)
}

pub async fn install(pool: &PgPool, user_id: &str, plugin_id: &str) -> Result<UserPlugin, PluginError> {
    let id = Uuid::new_v4().to_string();

    plugin_repo::install(pool, &id, user_id, plugin_id)
        .await
        .map_err(|e| {
            if let sqlx::Error::Database(ref db_err) = e {
                if db_err.code().as_deref() == Some("23505") {
                    return PluginError::AlreadyInstalled;
                }
            }
            PluginError::Internal
        })
}

pub async fn uninstall(pool: &PgPool, user_id: &str, plugin_id: &str) -> Result<(), PluginError> {
    let deleted = plugin_repo::uninstall(pool, user_id, plugin_id)
        .await
        .map_err(|_| PluginError::Internal)?;

    if deleted { Ok(()) } else { Err(PluginError::NotFound) }
}

pub async fn set_enabled(
    pool: &PgPool,
    user_id: &str,
    plugin_id: &str,
    enabled: bool,
) -> Result<UserPlugin, PluginError> {
    plugin_repo::set_enabled(pool, user_id, plugin_id, enabled)
        .await
        .map_err(|_| PluginError::Internal)?
        .ok_or(PluginError::NotFound)
}
