#[derive(Clone, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
}
