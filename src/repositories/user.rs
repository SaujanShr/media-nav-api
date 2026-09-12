use sqlx::{PgPool, Error, query, query_as};

use crate::models::user::{User, UserSettings, Theme};

// ── Public ────────────────────────────────────────────────────────────────────

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

pub async fn find_by_id(pool: &PgPool, user_id: &str) -> Result<Option<User>, Error> {
    query_as::<_, User>("
        SELECT   id, username, password_hash, created_at
        FROM     users
        WHERE    id = $1
        ")
        .bind(user_id)
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
        SELECT   user_id, nsfw_enabled, theme
        FROM     user_settings
        WHERE    user_id = $1
        ")
        .bind(user_id)
        .fetch_one(pool)
        .await
}

pub async fn create_default_settings(pool: &PgPool, user_id: &str) -> Result<(), Error> {
    query("
        INSERT INTO user_settings (user_id, nsfw_enabled, theme)
        VALUES ($1, FALSE, 'default')
        ")
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update_settings(
    pool: &PgPool,
    user_id: &str,
    nsfw_enabled: Option<bool>,
    theme: Option<&Theme>,
) -> Result<UserSettings, Error> {
    query_as::<_, UserSettings>("
        UPDATE user_settings
        SET    nsfw_enabled = COALESCE($2, nsfw_enabled),
               theme        = COALESCE($3, theme)
        WHERE  user_id = $1
        RETURNING user_id, nsfw_enabled, theme
        ")
        .bind(user_id)
        .bind(nsfw_enabled)
        .bind(theme)
        .fetch_one(pool)
        .await
}

pub async fn delete(pool: &PgPool, user_id: &str) -> Result<bool, Error> {
    let result = query("
        DELETE FROM users
        WHERE  id = $1
        ")
        .bind(user_id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}
