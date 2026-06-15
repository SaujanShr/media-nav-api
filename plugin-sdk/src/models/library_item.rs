use serde::{Serialize, Deserialize};
use crate::media_item::{Media, MediaItem};

#[derive(Serialize, Deserialize)]
pub struct LibraryItemAttribute {
    pub label: String,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct LibraryItemMetadata {
    pub title:      String,
    pub subtitle:   Option<String>,
    pub summary:    Option<String>,
    pub attributes: Vec<LibraryItemAttribute>,
}

#[derive(Serialize, Deserialize)]
pub struct LibraryItemResources {
    pub thumbnail:  String,
    pub preview:    Vec<MediaItem>,
    pub media:      Option<Media>
}

#[derive(Serialize, Deserialize)]
pub struct LibraryItem {
    pub id:            String,
    pub title:         String,
    pub thumbnail_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct LibraryItemDetail {
    pub id:        String,
    pub version:   Option<String>,
    pub metadata:  LibraryItemMetadata,
    pub resources: LibraryItemResources,
}
