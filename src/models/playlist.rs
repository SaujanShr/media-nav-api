use sqlx::FromRow;
use serde::Serialize;
use chrono::{DateTime, Utc};

#[derive(FromRow, Serialize)]
pub struct Playlist {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(FromRow, Serialize)]
pub struct PlaylistItem {
    pub id: String,
    pub playlist_id: String,
    pub user_library_item_id: String,
    pub index: f64,
}



