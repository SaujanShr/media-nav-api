#[derive(sqlx::FromRow, serde::Serialize)]
pub struct UserLibraryItem {
    pub id: String,
    pub user_plugin_id: String,
    pub library_item_id: String,
}
