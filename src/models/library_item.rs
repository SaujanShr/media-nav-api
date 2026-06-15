use sqlx::FromRow;
use serde::Serialize;
use chrono::{DateTime, Utc};

#[derive(FromRow, Serialize)]
pub struct UserLibraryItem {
    pub id: String,
    pub user_plugin_id: String,
    pub library_item_id: String,
    pub version: String,
    pub last_accessed: DateTime<Utc>,
}
