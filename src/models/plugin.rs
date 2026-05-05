use sqlx::FromRow;
use serde::Serialize;
use chrono::{DateTime, Utc};

use crate::plugins::{PluginMetadata, PluginResources};

/// Returned by `GET /plugins/all` — built from the live plugin registry.
#[derive(Serialize)]
pub struct PluginDto<'a> {
    pub id: &'a str,
    pub version: &'a str,
    pub metadata: &'a PluginMetadata,
    pub resources: &'a PluginResources,
}

#[derive(FromRow, Serialize)]
pub struct UserPlugin {
    pub id: String,
    pub user_id: String,
    pub plugin_id: String,
    pub last_accessed: DateTime<Utc>,
    pub enabled: bool,
}
