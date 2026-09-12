pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub plugins_dir: String,
    pub host: String,
    pub port: u16,
    pub db_max_connections: u32,
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
            host: std::env::var("HOST")
                .expect("HOST must be set in the environment or .env file"),
            port: std::env::var("PORT")
                .expect("PORT must be set in the environment or .env file")
                .parse()
                .expect("PORT must be a valid number"),
            db_max_connections: std::env::var("DB_MAX_CONNECTIONS")
                .expect("DB_MAX_CONNECTIONS must be set in the environment or .env file")
                .parse()
                .expect("DB_MAX_CONNECTIONS must be a valid number"),
        }
    }
}
