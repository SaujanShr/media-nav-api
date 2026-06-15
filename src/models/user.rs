use sqlx::FromRow;
use serde::Serialize;
use chrono::{DateTime, Utc};

#[derive(FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    #[allow(dead_code)]
    pub created_at: DateTime<Utc>,
}

#[derive(FromRow, Serialize)]
pub struct UserSettings {
    pub user_id: String,
    pub nsfw_enabled: bool,
}
