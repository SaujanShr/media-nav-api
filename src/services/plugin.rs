use sqlx::PgPool;
use uuid::Uuid;

use crate::models::plugin::{Plugin, UserPlugin};
use crate::repositories::plugin as plugin_repo;

// ── Types ─────────────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum PluginError {
    AlreadyInstalled,
    NotFound,
    Internal,
}

// ── Public API ────────────────────────────────────────────────────────────────

pub async fn list_all(pool: &PgPool) -> Result<Vec<Plugin>, PluginError> {
    plugin_repo::list_all(pool)
        .await
        .map_err(|_| PluginError::Internal)
}

pub async fn list(pool: &PgPool, user_id: &str) -> Result<Vec<UserPlugin>, PluginError> {
    plugin_repo::list(pool, user_id)
        .await
        .map_err(|_| PluginError::Internal)
}

pub async fn install(pool: &PgPool, user_id: &str, plugin_id: &str) -> Result<UserPlugin, PluginError> {
    plugin_repo::find_plugin_by_id(pool, plugin_id)
        .await
        .map_err(|_| PluginError::Internal)?
        .ok_or(PluginError::NotFound)?;

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


