use bcrypt::{hash, verify, DEFAULT_COST};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::create_token;
use crate::models::user::User;
use crate::repositories::user as user_repo;
use crate::validation::{validate_username, validate_password};

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

fn build_result(user: &User, secret: &str) -> Result<AuthResult, AuthError> {
    let token = create_token(&user.id, secret)
        .map_err(|_| AuthError::Internal)?;

    let result = AuthResult {
        token,
        user_id: user.id.clone(),
        username: user.username.clone()
    };

    Ok(result)
}

// ── Public ────────────────────────────────────────────────────────────────────

pub async fn register(pool: &PgPool, username: &str, password: &str, secret: &str) -> Result<AuthResult, AuthError> {
    validate_username(username).map_err(AuthError::ValidationError)?;
    validate_password(password).map_err(AuthError::ValidationError)?;

    let existing = user_repo::find_by_username(pool, username)
        .await
        .map_err(|e| {
            tracing::error!("Database error checking username existence: {}", e);
            AuthError::Internal
        })?;

    if existing.is_some() {
        return Err(AuthError::UsernameTaken);
    }

    let id = Uuid::new_v4().to_string();
    let password_hash = hash(password, DEFAULT_COST)
        .map_err(|e| {
            tracing::error!("Bcrypt hash error: {}", e);
            AuthError::Internal
        })?;

    user_repo::create(pool, &id, username, &password_hash)
        .await
        .map_err(|e| {
            tracing::error!("Database error creating user: {}", e);
            AuthError::Internal
        })?;

    // Create default settings for the new user
    user_repo::create_default_settings(pool, &id)
        .await
        .map_err(|e| {
            tracing::error!("Database error creating user settings: {}", e);
            AuthError::Internal
        })?;

    let user = user_repo::find_by_username(pool, username)
        .await
        .map_err(|e| {
            tracing::error!("Database error finding user after creation: {}", e);
            AuthError::Internal
        })?
        .ok_or(AuthError::Internal)?;

    build_result(&user, secret)
}

pub async fn login(pool: &PgPool, username: &str, password: &str, secret: &str) -> Result<AuthResult, AuthError> {
    let user = user_repo::find_by_username(pool, username)
        .await
        .map_err(|e| {
            tracing::error!("Database error finding user during login: {}", e);
            AuthError::Internal
        })?
        .ok_or(AuthError::InvalidCredentials)?;

    if !verify(password, &user.password_hash).unwrap_or(false) {
        return Err(AuthError::InvalidCredentials);
    }

    build_result(&user, secret)
}

pub async fn delete(pool: &PgPool, user_id: &str) -> Result<(), AuthError> {
    user_repo::delete(pool, user_id)
        .await
        .map_err(|e| {
            tracing::error!("Database error deleting user: {}", e);
            AuthError::Internal
        })?;

    Ok(())
}
