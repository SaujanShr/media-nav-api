use sqlx::FromRow;
use serde::Serialize;
use chrono::{DateTime, Utc};

#[derive(FromRow, Serialize)]
pub struct Plugin {
    pub id: String,
    pub version: String,
    pub nsfw: bool,
}

#[derive(FromRow, Serialize)]
pub struct UserPlugin {
    pub id: String,
    pub user_id: String,
    pub plugin_id: String,
    pub last_accessed: DateTime<Utc>,
    pub enabled: bool,
}
