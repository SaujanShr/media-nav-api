use serde::Serialize;
use crate::media::MediaItem;

#[derive(Serialize, Clone)]
pub struct LibraryItemAttribute {
    pub label: String,
    pub value: String,
}

#[derive(Serialize, Clone)]
pub struct LibraryItemMetadata {
    pub title: String,
    pub subtitle: Option<String>,
    pub summary: Option<String>,
    pub attributes: Vec<LibraryItemAttribute>,
}

#[derive(Serialize, Clone)]
pub struct LibraryItemResources {
    pub thumbnail: MediaItem,
    pub preview_items: Vec<MediaItem>,
}

#[derive(Serialize, Clone)]
pub struct LibraryItem {
    pub id:            String,
    pub title:         String,
    pub thumbnail_url: String,
}

#[derive(Serialize, Clone)]
pub struct LibraryItemDetail {
    pub id:        String,
    pub metadata:  LibraryItemMetadata,
    pub resources: LibraryItemResources,
}
