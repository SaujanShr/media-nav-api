mod auth;
mod config;
mod db;
mod handlers;
mod models;
mod plugins;
mod repositories;
mod services;
mod state;

use std::path::Path;
use actix_web::{App, HttpServer, Error};
use actix_web::web::{Data, scope};
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web_httpauth::middleware::HttpAuthentication;
use sqlx::PgPool;
use auth::bearer_validator;
use config::Config;
use plugins::PluginRegistry;
use state::AppState;


async fn init_db(database_url: &str) -> PgPool {
    let pool = db::create_pool(database_url)
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
        // ── Protected routes (token required) ─────────────────────────
        .service(
            scope("/api")
                .wrap(auth)
                .configure(handlers::auth::protected_routes)
                .configure(handlers::plugin::protected_routes)
                .configure(handlers::library_item::protected_routes)
                .configure(handlers::playlist::protected_routes)
                .configure(handlers::settings::protected_routes)
        )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let config = Config::from_env();
    let pool = init_db(&config.database_url).await;
    let plugins = PluginRegistry::load_from_dir(Path::new(&config.plugins_dir));
    let app_state = Data::new(AppState::new(pool, config, plugins));

    HttpServer::new(move || create_app(app_state.clone()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}