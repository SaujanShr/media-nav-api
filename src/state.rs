use sqlx::PgPool;
use crate::config::Config;

pub struct AppState {
    pub db: PgPool,
    pub jwt_secret: String,
}

impl AppState {
    pub fn new(db: PgPool, config: Config) -> Self {
        AppState { db, jwt_secret: config.jwt_secret }
    }
}
