mod auth;
mod config;
mod db;
mod handlers;
mod models;
mod repositories;
mod services;
mod state;

use actix_web::{web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use auth::bearer_validator;
use config::Config;
use state::AppState;


async fn init_db(database_url: &str) -> sqlx::PgPool {
    let pool = db::create_pool(database_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    db::run_migrations(&pool)
        .await
        .expect("Failed to run database migrations");

    pool
}

fn create_app(app_state: web::Data<AppState>) -> actix_web::App<
    impl actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    let auth = HttpAuthentication::bearer(bearer_validator);

    App::new()
        .app_data(app_state)
        // ── Public routes (no token required) ─────────────────────────
        .configure(handlers::health::public_routes)
        .configure(handlers::auth::public_routes)
        // ── Protected routes (token required) ─────────────────────────
        .service(
            web::scope("/api")
                .wrap(auth)
                .configure(handlers::auth::protected_routes),
        )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let config = Config::from_env();
    let pool = init_db(&config.database_url).await;
    let app_state = web::Data::new(AppState::new(pool, config));

    HttpServer::new(move || create_app(app_state.clone()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}