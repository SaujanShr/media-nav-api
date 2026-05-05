use sqlx::{PgPool, Error, query, query_as};

use crate::models::user::User;

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
