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
    let mut settings = user_repo::get_settings(pool, user_id)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => UserSettingsError::NotFound,
            _ => UserSettingsError::Internal,
        })?;

    if let Some(nsfw_enabled) = request.nsfw_enabled {
        settings.nsfw_enabled = nsfw_enabled;
    }

    if let Some(theme) = request.theme {
        settings.theme = theme;
    }

    user_repo::update_settings(pool, user_id, settings.nsfw_enabled, &settings.theme)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => UserSettingsError::NotFound,
            _ => UserSettingsError::Internal,
        })
}
