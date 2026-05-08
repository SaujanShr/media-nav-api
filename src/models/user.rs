use sqlx::FromRow;
use serde::Serialize;
use chrono::{DateTime, Utc};

#[derive(Clone, FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    #[allow(dead_code)]
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, FromRow, Serialize)]
pub struct UserSettings {
    pub user_id: String,
    pub nsfw_enabled: bool,
}

