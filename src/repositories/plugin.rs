use sqlx::{PgPool, Error, query, query_as};

use crate::models::plugin::{Plugin, UserPlugin};

pub async fn list_all(pool: &PgPool) -> Result<Vec<Plugin>, Error> {
    query_as::<_, Plugin>("
        SELECT   id, version, nsfw
        FROM     plugins
        ORDER BY id
        ")
        .fetch_all(pool)
        .await
}

pub async fn find_plugin_by_id(pool: &PgPool, id: &str) -> Result<Option<Plugin>, Error> {
    query_as::<_, Plugin>("
        SELECT   id, version, nsfw
        FROM     plugins
        WHERE    id = $1
        ")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn find_by_id(pool: &PgPool, id: &str) -> Result<Option<UserPlugin>, Error> {
    query_as::<_, UserPlugin>("
        SELECT   id, user_id, plugin_id, last_accessed
        FROM     user_plugins
        WHERE    id = $1
        ")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn list(pool: &PgPool, user_id: &str) -> Result<Vec<UserPlugin>, Error> {
    query_as::<_, UserPlugin>("
        SELECT   id, user_id, plugin_id, last_accessed
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
) -> Result<UserPlugin, Error> {
    query_as::<_, UserPlugin>("
        INSERT INTO user_plugins (id, user_id, plugin_id)
        VALUES ($1, $2, $3)
        RETURNING id, user_id, plugin_id, last_accessed
        ")
        .bind(id)
        .bind(user_id)
        .bind(plugin_id)
        .fetch_one(pool)
        .await
}

pub async fn uninstall(pool: &PgPool, user_id: &str, plugin_id: &str) -> Result<bool, Error> {
    let result = query("
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


