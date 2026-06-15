pub mod auth;
pub mod library_item;
pub mod media;
pub mod playlist;
pub mod plugin;
pub mod user;

use sqlx::Error::Database;

// ── Public ────────────────────────────────────────────────────────────────────

pub(crate) fn is_duplicate_key(e: &sqlx::Error) -> bool {
    matches!(e, Database(db) if db.code().as_deref() == Some("23505"))
}
