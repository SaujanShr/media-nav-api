pub mod auth;
pub mod library_item;
pub mod media;
pub mod playlist;
pub mod plugin;
pub mod user;

use sqlx::Error::Database;

#[cfg(test)]
#[path = "tests/services.rs"]
mod tests;

fn is_duplicate_key(e: &sqlx::Error) -> bool {
    matches!(e, Database(db) if db.code().as_deref() == Some("23505"))
}
