use sqlx::FromRow;
use serde::Serialize;

#[derive(Clone, FromRow, Serialize)]
pub struct UserSettings {
    pub user_id: String,
    pub nsfw_enabled: bool,
}

