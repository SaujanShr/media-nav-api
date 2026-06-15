use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    #[allow(dead_code)]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "text", rename_all = "lowercase")]
pub enum Theme {
    Default,
    Light,
    Dark,
}

impl Default for Theme {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(FromRow, Serialize)]
pub struct UserSettings {
    pub user_id: String,
    pub nsfw_enabled: bool,
    pub theme: Theme,
}
