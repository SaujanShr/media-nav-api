mod auth;
mod config;
mod db;
mod handlers;
mod models;
mod plugins;
mod repositories;
mod services;
mod state;
mod validation;

use std::path::Path;
use dotenvy::dotenv;
use actix_web::{App, HttpServer, Error};
use actix_web::web::{Data, scope};
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web_httpauth::middleware::HttpAuthentication;
use sqlx::PgPool;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use auth::bearer_validator;
use config::Config;
use plugins::PluginRegistry;
use state::AppState;


async fn init_db(database_url: &str, max_connections: u32) -> PgPool {
    let pool = db::create_pool(database_url, max_connections)
        .await
        .expect("Failed to connect to PostgreSQL");

    db::run_migrations(&pool)
        .await
        .expect("Failed to run database migrations");

    pool
}

fn create_app(app_state: Data<AppState>) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse,
        Error = Error,
        InitError = (),
    >,
> {
    let auth = HttpAuthentication::bearer(bearer_validator);

    App::new()
        .app_data(app_state)
        // ── Public routes (no token required) ─────────────────────────
        .configure(handlers::health::public_routes)
        .configure(handlers::auth::public_routes)
        .configure(handlers::plugin::public_routes)
        .configure(handlers::media::public_routes)
        // ── Protected routes (token required) ─────────────────────────
        .service(
            scope("/api")
                .wrap(auth)
                .configure(handlers::auth::protected_routes)
                .configure(handlers::plugin::protected_routes)
                .configure(handlers::library::protected_routes)
                .configure(handlers::playlist::protected_routes)
                .configure(handlers::settings::protected_routes)
        )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "media_nav_api=info,actix_web=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env();
    let pool = init_db(&config.database_url, config.db_max_connections).await;
    let plugins = PluginRegistry::load_from_dir(Path::new(&config.plugins_dir));

    let host = config.host.clone();
    let port = config.port;
    let app_state = Data::new(AppState::new(pool, config, plugins));

    tracing::info!("Starting server on {}:{}", host, port);

    HttpServer::new(move || create_app(app_state.clone()))
        .bind((host.as_str(), port))?
        .run()
        .await
}
