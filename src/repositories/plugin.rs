use sqlx::PgPool;

use crate::models::plugin::UserPlugin;

pub async fn find_by_id(pool: &PgPool, id: &str) -> Result<Option<UserPlugin>, sqlx::Error> {
    sqlx::query_as::<_, UserPlugin>("
        SELECT   id, user_id, plugin_id, version_id
        FROM     user_plugins
        WHERE    id = $1
        ")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn list(pool: &PgPool, user_id: &str) -> Result<Vec<UserPlugin>, sqlx::Error> {
    sqlx::query_as::<_, UserPlugin>("
        SELECT   id, user_id, plugin_id, version_id
        FROM     user_plugins
        WHERE    user_id = $1
        ")
        .bind(user_id)
        .fetch_all(pool)
        .await
}

pub async fn install(
    pool: &PgPool,
    id: &str,
    user_id: &str,
    plugin_id: &str,
    version_id: &str,
) -> Result<UserPlugin, sqlx::Error> {
    sqlx::query_as::<_, UserPlugin>("
        INSERT INTO user_plugins (id, user_id, plugin_id, version_id)
        VALUES ($1, $2, $3, $4)
        RETURNING id, user_id, plugin_id, version_id
        ")
        .bind(id)
        .bind(user_id)
        .bind(plugin_id)
        .bind(version_id)
        .fetch_one(pool)
        .await
}

pub async fn uninstall(pool: &PgPool, user_id: &str, plugin_id: &str) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("
        DELETE FROM user_plugins
        WHERE  user_id = $1
        AND    plugin_id = $2
        ")
        .bind(user_id)
        .bind(plugin_id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}


