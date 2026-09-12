use sqlx::{PgPool, Error, query, query_as, query_scalar};

use crate::models::playlist::{Playlist, PlaylistItem};

// ── Public ────────────────────────────────────────────────────────────────────

pub async fn list(pool: &PgPool, user_id: &str) -> Result<Vec<Playlist>, Error> {
    query_as::<_, Playlist>("
        SELECT   id, user_id, name, created_at
        FROM     user_playlists
        WHERE    user_id = $1
        ORDER BY created_at
        ")
        .bind(user_id)
        .fetch_all(pool)
        .await
}

pub async fn find_by_id(pool: &PgPool, id: &str) -> Result<Option<Playlist>, Error> {
    query_as::<_, Playlist>("
        SELECT id, user_id, name, created_at
        FROM   user_playlists
        WHERE  id = $1
        ")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn create(pool: &PgPool, id: &str, user_id: &str, name: &str) -> Result<Playlist, Error> {
    query_as::<_, Playlist>("
        INSERT INTO user_playlists (id, user_id, name)
        VALUES ($1, $2, $3)
        RETURNING id, user_id, name, created_at
        ")
        .bind(id)
        .bind(user_id)
        .bind(name)
        .fetch_one(pool)
        .await
}

pub async fn delete(pool: &PgPool, id: &str) -> Result<bool, Error> {
    let result = query("
        DELETE FROM user_playlists
        WHERE  id = $1
        ")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn rename(pool: &PgPool, id: &str, name: &str) -> Result<Option<Playlist>, Error> {
    query_as::<_, Playlist>("
        UPDATE user_playlists
        SET    name = $2
        WHERE  id = $1
        RETURNING id, user_id, name, created_at
        ")
        .bind(id)
        .bind(name)
        .fetch_optional(pool)
        .await
}

pub async fn list_items(pool: &PgPool, playlist_id: &str) -> Result<Vec<PlaylistItem>, Error> {
    query_as::<_, PlaylistItem>("
        SELECT   id, playlist_id, user_library_item_id, index
        FROM     user_playlist_items
        WHERE    playlist_id = $1
        ORDER BY index
        ")
        .bind(playlist_id)
        .fetch_all(pool)
        .await
}

pub async fn item_exists(pool: &PgPool, playlist_id: &str, item_id: &str) -> Result<bool, Error> {
    query_scalar::<_, bool>("
        SELECT EXISTS(
            SELECT 1 FROM user_playlist_items
            WHERE  id = $1 AND playlist_id = $2
        )
        ")
        .bind(item_id)
        .bind(playlist_id)
        .fetch_one(pool)
        .await
}

pub async fn count_items(pool: &PgPool, playlist_id: &str) -> Result<i64, Error> {
    query_scalar("
        SELECT COUNT(*) FROM user_playlist_items
        WHERE  playlist_id = $1
        ")
        .bind(playlist_id)
        .fetch_one(pool)
        .await
}

pub async fn neighbor_indices(
    pool: &PgPool,
    playlist_id: &str,
    item_id: &str,
    offset: i64,
) -> Result<Vec<f64>, Error> {
    query_scalar("
        SELECT   index FROM user_playlist_items
        WHERE    playlist_id = $1 AND id != $2
        ORDER BY index
        OFFSET   $3
        LIMIT    2
        ")
        .bind(playlist_id)
        .bind(item_id)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn find_library_item_user(pool: &PgPool, user_library_item_id: &str) -> Result<Option<String>, Error> {
    query_scalar::<_, String>("
        SELECT up.user_id
        FROM   user_library_items uli
        JOIN   user_plugins up ON up.id = uli.user_plugin_id
        WHERE  uli.id = $1
        ")
        .bind(user_library_item_id)
        .fetch_optional(pool)
        .await
}

pub async fn add_item(
    pool: &PgPool,
    id: &str,
    playlist_id: &str,
    user_library_item_id: &str,
) -> Result<PlaylistItem, Error> {
    let mut tx = pool.begin().await?;

    query("SELECT 1 FROM user_playlists WHERE id = $1 FOR UPDATE")
        .bind(playlist_id)
        .execute(&mut *tx)
        .await?;

    let max_index: Option<f64> = query_scalar(
        "SELECT MAX(index) FROM user_playlist_items WHERE playlist_id = $1"
    )
        .bind(playlist_id)
        .fetch_one(&mut *tx)
        .await?;

    let new_index = max_index.unwrap_or(-1.0) + 1.0;

    let item = query_as::<_, PlaylistItem>("
        INSERT INTO user_playlist_items (id, playlist_id, user_library_item_id, index)
        VALUES ($1, $2, $3, $4)
        RETURNING id, playlist_id, user_library_item_id, index
        ")
        .bind(id)
        .bind(playlist_id)
        .bind(user_library_item_id)
        .bind(new_index)
        .fetch_one(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(item)
}

pub async fn remove_item(
    pool: &PgPool,
    playlist_id: &str,
    user_library_item_id: &str,
) -> Result<bool, Error> {
    let result = query("
        DELETE FROM user_playlist_items
        WHERE  playlist_id = $1
        AND    user_library_item_id = $2
        ")
        .bind(playlist_id)
        .bind(user_library_item_id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn update_item_index(
    pool: &PgPool,
    item_id: &str,
    index: f64,
) -> Result<Option<PlaylistItem>, Error> {
    query_as::<_, PlaylistItem>("
        UPDATE user_playlist_items
        SET    index = $2
        WHERE  id = $1
        RETURNING id, playlist_id, user_library_item_id, index
        ")
        .bind(item_id)
        .bind(index)
        .fetch_optional(pool)
        .await
}

pub async fn set_item_indices(pool: &PgPool, updates: Vec<(String, f64)>) -> Result<(), Error> {
    let Some((first_item_id, _)) = updates.first() else {
        return Ok(());
    };

    let mut tx = pool.begin().await?;

    let playlist_id: String = query_scalar("SELECT playlist_id FROM user_playlist_items WHERE id = $1")
        .bind(first_item_id)
        .fetch_one(&mut *tx)
        .await?;

    query("SELECT 1 FROM user_playlists WHERE id = $1 FOR UPDATE")
        .bind(&playlist_id)
        .execute(&mut *tx)
        .await?;

    let (ids, indices): (Vec<String>, Vec<f64>) = updates.into_iter().unzip();

    query("
        UPDATE user_playlist_items AS t
        SET    index = u.index
        FROM   UNNEST($1::text[], $2::float8[]) AS u(id, index)
        WHERE  t.id = u.id
        ")
        .bind(ids)
        .bind(indices)
        .execute(&mut *tx)
        .await?;

    tx.commit().await
}
