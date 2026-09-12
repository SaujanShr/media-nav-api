use sqlx::{PgPool, postgres::PgPoolOptions, Error};

// ── Public ────────────────────────────────────────────────────────────────────

pub async fn create_pool(database_url: &str, max_connections: u32) -> Result<PgPool, Error> {
    PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(database_url)
        .await
}
