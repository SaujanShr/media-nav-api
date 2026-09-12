use sqlx::PgPool;

use crate::handlers::settings::UpdateSettingsRequest;
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

pub async fn update(pool: &PgPool, user_id: &str, request: UpdateSettingsRequest) -> Result<UserSettings, UserSettingsError> {
    user_repo::update_settings(pool, user_id, request.nsfw_enabled, request.theme.as_ref())
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => UserSettingsError::NotFound,
            _ => UserSettingsError::Internal,
        })
}
