use sqlx::{PgPool, migrate::MigrateError, migrate};

// ── Public ────────────────────────────────────────────────────────────────────

pub async fn run_migrations(pool: &PgPool) -> Result<(), MigrateError> {
    migrate!("./migrations")
        .run(pool)
        .await
}
