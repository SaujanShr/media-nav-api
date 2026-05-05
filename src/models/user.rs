use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Clone, FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}
