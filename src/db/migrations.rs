use sqlx::{PgPool, migrate::MigrateError, migrate};

pub async fn run_migrations(pool: &PgPool) -> Result<(), MigrateError> {
    migrate!("./migrations")
        .run(pool)
        .await
}
