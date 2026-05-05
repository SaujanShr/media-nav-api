use sqlx::PgPool;
use crate::config::Config;
use crate::plugins::PluginRegistry;

pub struct AppState {
    pub db: PgPool,
    pub jwt_secret: String,
    pub plugins: PluginRegistry,
}

impl AppState {
    pub fn new(db: PgPool, config: Config, plugins: PluginRegistry) -> Self {
        AppState { db, jwt_secret: config.jwt_secret, plugins }
    }
}
