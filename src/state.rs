use jsonwebtoken::{DecodingKey, Validation};
use sqlx::PgPool;

use crate::auth;
use crate::config::Config;
use crate::plugins::PluginRegistry;

pub struct AppState {
    pub db: PgPool,
    pub jwt_secret: String,
    pub jwt_decoding_key: DecodingKey,
    pub jwt_validation: Validation,
    pub plugins: PluginRegistry,
}

impl AppState {
    pub fn new(db: PgPool, config: Config, plugins: PluginRegistry) -> Self {
        let jwt_secret = config.jwt_secret;
        let jwt_decoding_key = auth::decoding_key(&jwt_secret);
        let jwt_validation = auth::validation();

        AppState {
            db,
            jwt_secret,
            jwt_decoding_key,
            jwt_validation,
            plugins,
        }
    }
}
