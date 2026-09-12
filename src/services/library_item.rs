use sqlx::PgPool;
use uuid::Uuid;

use crate::models::library_item::UserLibraryItem;
use crate::repositories::{library_item as library_item_repo, plugin as plugin_repo};
use super::is_duplicate_key;

// ── Types ─────────────────────────────────────────────────────────────────────

pub enum LibraryItemError {
    PluginNotFound,
    Forbidden,
    AlreadyAdded,
    NotFound,
    Internal,
}

// ── Private ───────────────────────────────────────────────────────────────────

async fn resolve_plugin(pool: &PgPool, user_plugin_id: &str, user_id: &str) -> Result<(), LibraryItemError> {
    let plugin = plugin_repo::find_by_id(pool, user_plugin_id)
        .await
        .map_err(|_| LibraryItemError::Internal)?
        .ok_or(LibraryItemError::PluginNotFound)?;

    if plugin.user_id != user_id {
        return Err(LibraryItemError::Forbidden);
    }

    Ok(())
}

// ── Public ────────────────────────────────────────────────────────────────────

pub async fn list(pool: &PgPool, user_plugin_id: &str, user_id: &str) -> Result<Vec<UserLibraryItem>, LibraryItemError> {
    resolve_plugin(pool, user_plugin_id, user_id).await?;

    library_item_repo::list(pool, user_plugin_id)
        .await
        .map_err(|_| LibraryItemError::Internal)
}

pub async fn add(pool: &PgPool, user_plugin_id: &str, user_id: &str, library_item_id: &str) -> Result<UserLibraryItem, LibraryItemError> {
    resolve_plugin(pool, user_plugin_id, user_id).await?;

    let id = Uuid::new_v4().to_string();
    
    library_item_repo::add(pool, &id, user_plugin_id, library_item_id)
        .await
        .map_err(|e|
            if is_duplicate_key(&e) { LibraryItemError::AlreadyAdded }
            else { LibraryItemError::Internal }
        )
}

pub async fn remove(pool: &PgPool, user_plugin_id: &str, user_id: &str, library_item_id: &str) -> Result<(), LibraryItemError> {
    resolve_plugin(pool, user_plugin_id, user_id).await?;

    let deleted = library_item_repo::remove(pool, user_plugin_id, library_item_id)
        .await
        .map_err(|_| LibraryItemError::Internal)?;

    if !deleted {
        return Err(LibraryItemError::NotFound);
    }

    Ok(())
}
