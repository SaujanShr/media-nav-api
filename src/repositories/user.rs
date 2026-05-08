use sqlx::{PgPool, Error, query, query_as};

use crate::models::user::{User, UserSettings};

pub async fn find_by_username(pool: &PgPool, username: &str) -> Result<Option<User>, Error> {
    query_as::<_, User>("
        SELECT   id, username, password_hash, created_at
        FROM     users
        WHERE    username = $1
        ")
        .bind(username)
        .fetch_optional(pool)
        .await
}

pub async fn create(
    pool: &PgPool,
    id: &str,
    username: &str,
    password_hash: &str,
) -> Result<(), Error> {
    query("
        INSERT INTO users (id, username, password_hash)
        VALUES ($1, $2, $3)
        ")
        .bind(id)
        .bind(username)
        .bind(password_hash)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_settings(pool: &PgPool, user_id: &str) -> Result<UserSettings, Error> {
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

