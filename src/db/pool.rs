use sqlx::{PgPool, postgres::PgPoolOptions, Error};

// ── Config ────────────────────────────────────────────────────────────────────

const MAX_CONNECTIONS: u32 = 5;

// ── Public ────────────────────────────────────────────────────────────────────

pub async fn create_pool(database_url: &str) -> Result<PgPool, Error> {
    PgPoolOptions::new()
        .max_connections(MAX_CONNECTIONS)
        .connect(database_url)
        .await
}
