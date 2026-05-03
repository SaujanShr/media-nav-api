use bcrypt::{hash, verify, DEFAULT_COST};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::create_token;
use crate::models::user::User;
use crate::repositories::user as user_repo;

// ── Types ─────────────────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct AuthResult {
    pub token: String,
    pub user_id: String,
    pub username: String,
}

#[derive(Debug)]
pub enum AuthError {
    UsernameTaken,
    InvalidCredentials,
    Internal,
}

// ── Private ───────────────────────────────────────────────────────────────────

fn build_result(user: &User, secret: &str) -> Result<AuthResult, AuthError> {
    let token =
        create_token(&user.id, &user.username, secret)
        .map_err(|_| AuthError::Internal)?;

    Ok(AuthResult { token, user_id: user.id.clone(), username: user.username.clone() })
}

// ── Public API ────────────────────────────────────────────────────────────────

pub async fn register(pool: &PgPool, username: &str, password: &str, secret: &str) -> Result<AuthResult, AuthError> {
    let existing = user_repo::find_by_username(pool, username)
        .await
        .map_err(|_| AuthError::Internal)?;

    if existing.is_some() {
        return Err(AuthError::UsernameTaken);
    }

    let id = Uuid::new_v4().to_string();
    let password_hash = hash(password, DEFAULT_COST).map_err(|_| AuthError::Internal)?;

    user_repo::create(pool, &id, username, &password_hash)
        .await
        .map_err(|_| AuthError::Internal)?;

    let user = User { id, username: username.to_owned(), password_hash };
    build_result(&user, secret)
}

pub async fn login(pool: &PgPool, username: &str, password: &str, secret: &str) -> Result<AuthResult, AuthError> {
    let user = user_repo::find_by_username(pool, username)
        .await
        .map_err(|_| AuthError::Internal)?
        .ok_or(AuthError::InvalidCredentials)?;

    if !verify(password, &user.password_hash).unwrap_or(false) {
        return Err(AuthError::InvalidCredentials);
    }

    build_result(&user, secret)
}
