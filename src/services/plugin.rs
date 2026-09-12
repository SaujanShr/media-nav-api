use sqlx::PgPool;
use uuid::Uuid;

use plugin_sdk::plugin::{PluginCallError, PluginInfo};

use crate::models::plugin::UserPlugin;
use crate::plugins::PluginRegistry;
use crate::repositories::plugin as plugin_repo;
use super::is_duplicate_key;

// ── Types ─────────────────────────────────────────────────────────────────────

pub enum PluginError {
    AlreadyInstalled,
    NotFound,
    ValidationError(String),
    UpstreamError(PluginCallError),
    Internal,
}

// ── Public ────────────────────────────────────────────────────────────────────

pub fn list_all(registry: &PluginRegistry) -> Vec<&PluginInfo> {
    registry.plugins().map(|p| &p.info).collect()
}

pub async fn list(pool: &PgPool, user_id: &str) -> Result<Vec<UserPlugin>, PluginError> {
    plugin_repo::list(pool, user_id)
        .await
        .map_err(|_| PluginError::Internal)
}

pub async fn install(
    pool: &PgPool,
    registry: &PluginRegistry,
    user_id: &str,
    plugin_id: &str,
) -> Result<UserPlugin, PluginError> {
    let plugin = registry.get(plugin_id).ok_or(PluginError::NotFound)?;
    let id = Uuid::new_v4().to_string();

    plugin_repo::install(pool, &id, user_id, plugin_id, &plugin.info.version)
        .await
        .map_err(|e|
            if is_duplicate_key(&e) { PluginError::AlreadyInstalled }
            else { PluginError::Internal }
        )
}

pub async fn uninstall(pool: &PgPool, user_id: &str, plugin_id: &str) -> Result<(), PluginError> {
    let deleted = plugin_repo::uninstall(pool, user_id, plugin_id)
        .await
        .map_err(|_| PluginError::Internal)?;
    
    if !deleted {
        return Err(PluginError::NotFound);
    }

    Ok(())
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
