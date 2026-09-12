use std::sync::OnceLock;

use actix_web::web;
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::create_token;
use crate::repositories::user as user_repo;
use crate::validation::{validate_username, validate_password};
use super::is_duplicate_key;

// ── Config ────────────────────────────────────────────────────────────────────

fn dummy_hash() -> &'static str {
    static HASH: OnceLock<String> = OnceLock::new();

    HASH.get_or_init(|| {
        hash("placeholder", DEFAULT_COST)
            .expect("failed to compute dummy bcrypt hash")
    })
}

// ── Types ─────────────────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct AuthResult {
    pub token: String,
    pub user_id: String,
    pub username: String,
}

pub enum AuthError {
    UsernameTaken,
    InvalidCredentials,
    ValidationError(String),
    Internal,
}

// ── Private ───────────────────────────────────────────────────────────────────

fn build_result(id: &str, username: &str, secret: &str) -> Result<AuthResult, AuthError> {
    let token = create_token(id, secret)
        .map_err(|_| AuthError::Internal)?;

    Ok(AuthResult {
        token,
        user_id: id.to_string(),
        username: username.to_string(),
    })
}

async fn verify_password(password: String, hash: String) -> bool {
    match web::block(move || verify(password, &hash)).await {
        Ok(Ok(matches)) => matches,
        Ok(Err(e)) => {
            tracing::error!("Bcrypt verify error: {}", e);
            false
        }
        Err(e) => {
            tracing::error!("Bcrypt verify task panicked: {}", e);
            false
        }
    }
}

// ── Public ────────────────────────────────────────────────────────────────────

pub async fn register(pool: &PgPool, username: &str, password: &str, secret: &str) -> Result<AuthResult, AuthError> {
    validate_username(username).map_err(AuthError::ValidationError)?;
    validate_password(password).map_err(AuthError::ValidationError)?;

    let id = Uuid::new_v4().to_string();
    let password_owned = password.to_string();

    let password_hash = web::block(move || hash(password_owned, DEFAULT_COST))
        .await
        .map_err(|e| {
            tracing::error!("Bcrypt hash task panicked: {}", e);
            AuthError::Internal
        })?
        .map_err(|e| {
            tracing::error!("Bcrypt hash error: {}", e);
            AuthError::Internal
        })?;

    user_repo::create(pool, &id, username, &password_hash)
        .await
        .map_err(|e| {
            if is_duplicate_key(&e) {
                AuthError::UsernameTaken
            } else {
                tracing::error!("Database error creating user: {}", e);
                AuthError::Internal
            }
        })?;

    user_repo::create_default_settings(pool, &id)
        .await
        .map_err(|e| {
            tracing::error!("Database error creating user settings: {}", e);
            AuthError::Internal
        })?;

    build_result(&id, username, secret)
}

pub async fn login(pool: &PgPool, username: &str, password: &str, secret: &str) -> Result<AuthResult, AuthError> {
    let user = user_repo::find_by_username(pool, username)
        .await
        .map_err(|e| {
            tracing::error!("Database error finding user during login: {}", e);
            AuthError::Internal
        })?;

    let hash_to_check = match &user {
        Some(u) => u.password_hash.clone(),
        None => dummy_hash().to_string(),
    };

    let password_matches = verify_password(password.to_string(), hash_to_check).await;

    match user {
        Some(user) if password_matches => build_result(&user.id, &user.username, secret),
        _ => Err(AuthError::InvalidCredentials),
    }
}

pub async fn delete(pool: &PgPool, user_id: &str, password: &str) -> Result<(), AuthError> {
    let user = user_repo::find_by_id(pool, user_id)
        .await
        .map_err(|e| {
            tracing::error!("Database error finding user during account deletion: {}", e);
            AuthError::Internal
        })?
        .ok_or(AuthError::InvalidCredentials)?;

    if !verify_password(password.to_string(), user.password_hash.clone()).await {
        return Err(AuthError::InvalidCredentials);
    }

    user_repo::delete(pool, user_id)
        .await
        .map_err(|e| {
            tracing::error!("Database error deleting user: {}", e);
            AuthError::Internal
        })?;

    Ok(())
}
