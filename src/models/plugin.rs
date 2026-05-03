#[derive(sqlx::FromRow, serde::Serialize)]
pub struct UserPlugin {
    pub id: String,
    pub user_id: String,
    pub plugin_id: String,
    pub version_id: String,
}
