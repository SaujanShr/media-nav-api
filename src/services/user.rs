use sqlx::PgPool;

use crate::models::user::UserSettings;
use crate::repositories::user as user_repo;

// ── Types ─────────────────────────────────────────────────────────────────────

pub enum UserSettingsError {
    NotFound,
    Internal,
}

// ── Public ────────────────────────────────────────────────────────────────────

pub async fn get(pool: &PgPool, user_id: &str) -> Result<UserSettings, UserSettingsError> {
    user_repo::get_settings(pool, user_id)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => UserSettingsError::NotFound,
            _ => UserSettingsError::Internal,
        })
}

pub async fn set_nsfw_enabled(pool: &PgPool, user_id: &str, nsfw_enabled: bool) -> Result<UserSettings, UserSettingsError> {
    user_repo::set_nsfw_enabled(pool, user_id, nsfw_enabled)
        .await
        .map_err(|_| UserSettingsError::Internal)
}
