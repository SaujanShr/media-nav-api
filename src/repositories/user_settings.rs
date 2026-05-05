use sqlx::{PgPool, Error, query_as};

use crate::models::user_settings::UserSettings;

pub async fn get(pool: &PgPool, user_id: &str) -> Result<UserSettings, Error> {
    query_as::<_, UserSettings>("
        SELECT   user_id, nsfw_enabled
        FROM     user_settings
        WHERE    user_id = $1
        ")
        .bind(user_id)
        .fetch_one(pool)
        .await
}

pub async fn set_nsfw_enabled(pool: &PgPool, user_id: &str, nsfw_enabled: bool) -> Result<UserSettings, Error> {
    query_as::<_, UserSettings>("
        INSERT INTO user_settings (user_id, nsfw_enabled)
        VALUES ($1, $2)
        ON CONFLICT (user_id) DO UPDATE SET nsfw_enabled = EXCLUDED.nsfw_enabled
        RETURNING user_id, nsfw_enabled
        ")
        .bind(user_id)
        .bind(nsfw_enabled)
        .fetch_one(pool)
        .await
}

