use std::path::Path;

use actix_web::dev::Payload;
use actix_web::test::TestRequest;
use actix_web::web::Data;
use actix_web::{FromRequest, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use sqlx::postgres::PgPoolOptions;

use super::{bearer_validator, claims, user_id};
use crate::auth::{create_token, Claims};
use crate::config::Config;
use crate::plugins::PluginRegistry;
use crate::state::AppState;

const SECRET: &str = "test-secret-at-least-32-characters-long";

fn app_state() -> AppState {
    let db = PgPoolOptions::new()
        .connect_lazy("postgres://user:password@localhost/media_nav_db")
        .unwrap();
    let plugins = PluginRegistry::load_from_dir(Path::new("."));
    let config = Config {
        database_url: String::new(),
        jwt_secret: SECRET.to_string(),
        plugins_dir: String::new(),
        host: String::new(),
        port: 0,
        db_max_connections: 1,
    };

    AppState::new(db, config, plugins)
}

async fn bearer_auth(token: &str) -> BearerAuth {
    let req = TestRequest::default()
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_http_request();

    BearerAuth::from_request(&req, &mut Payload::None).await.unwrap()
}

#[test]
fn claims_returns_the_claims_when_present() {
    let req = TestRequest::default().to_http_request();
    let inserted = Claims { sub: "user-1".to_string(), iat: 0, exp: 0 };
    req.extensions_mut().insert(inserted.clone());

    let found = claims(&req).unwrap();
    assert_eq!(found.sub, inserted.sub);
}

#[test]
fn claims_errors_when_missing() {
    let req = TestRequest::default().to_http_request();
    assert!(claims(&req).is_err());
}

#[test]
fn user_id_returns_the_subject_when_claims_are_present() {
    let req = TestRequest::default().to_http_request();
    req.extensions_mut().insert(Claims { sub: "user-1".to_string(), iat: 0, exp: 0 });

    assert_eq!(user_id(&req).unwrap(), "user-1");
}

#[test]
fn user_id_errors_when_missing() {
    let req = TestRequest::default().to_http_request();
    assert!(user_id(&req).is_err());
}

#[actix_web::test]
async fn bearer_validator_accepts_a_valid_token_and_stores_the_claims() {
    let token = create_token("user-1", SECRET).unwrap();
    let creds = bearer_auth(&token).await;
    let req = TestRequest::default().app_data(Data::new(app_state())).to_srv_request();

    let req = bearer_validator(req, creds).await.unwrap();
    let inserted = req.extensions().get::<Claims>().unwrap().clone();
    assert_eq!(inserted.sub, "user-1");
}

#[actix_web::test]
async fn bearer_validator_rejects_a_token_signed_with_a_different_secret() {
    let token = create_token("user-1", "a-completely-different-secret-value").unwrap();
    let creds = bearer_auth(&token).await;
    let req = TestRequest::default().app_data(Data::new(app_state())).to_srv_request();

    assert!(bearer_validator(req, creds).await.is_err());
}

#[actix_web::test]
async fn bearer_validator_rejects_a_malformed_token() {
    let creds = bearer_auth("not-a-jwt").await;
    let req = TestRequest::default().app_data(Data::new(app_state())).to_srv_request();

    assert!(bearer_validator(req, creds).await.is_err());
}

#[actix_web::test]
async fn bearer_validator_rejects_when_app_state_is_missing() {
    let token = create_token("user-1", SECRET).unwrap();
    let creds = bearer_auth(&token).await;
    let req = TestRequest::default().to_srv_request();

    assert!(bearer_validator(req, creds).await.is_err());
}
