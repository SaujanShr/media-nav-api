pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub plugins_dir: String,
}

impl Config {
    pub fn from_env() -> Self {
        Config {
            database_url: std::env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set in the environment or .env file"),
            jwt_secret: std::env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set in the environment or .env file"),
            plugins_dir: std::env::var("PLUGINS_DIR")
                .expect("PLUGINS_DIR must be set in the environment or .env file"),
        }
    }
}
