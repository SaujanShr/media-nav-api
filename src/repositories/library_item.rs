use sqlx::{PgPool, Error, query, query_as};

use crate::models::library_item::UserLibraryItem;

pub async fn list(pool: &PgPool, user_plugin_id: &str) -> Result<Vec<UserLibraryItem>, Error> {
    query_as::<_, UserLibraryItem>("
        SELECT   id, user_plugin_id, library_item_id, last_accessed
        FROM     user_library_items
        WHERE    user_plugin_id = $1
        ")
        .bind(user_plugin_id)
        .fetch_all(pool)
        .await
}

pub async fn add(
    pool: &PgPool,
    id: &str,
    user_plugin_id: &str,
    library_item_id: &str,
) -> Result<UserLibraryItem, Error> {
    query_as::<_, UserLibraryItem>("
        INSERT INTO user_library_items (id, user_plugin_id, library_item_id)
        VALUES ($1, $2, $3)
        RETURNING id, user_plugin_id, library_item_id, last_accessed
        ")
        .bind(id)
        .bind(user_plugin_id)
        .bind(library_item_id)
        .fetch_one(pool)
        .await
}

pub async fn remove(pool: &PgPool, user_plugin_id: &str, library_item_id: &str) -> Result<bool, Error> {
    let result = query("
        DELETE FROM user_library_items
        WHERE  user_plugin_id = $1
        AND    library_item_id = $2
        ")
        .bind(user_plugin_id)
        .bind(library_item_id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}
