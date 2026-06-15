use serde::Serialize;
use crate::media_item::{Media, MediaItem};

#[derive(Serialize)]
pub struct LibraryItemAttribute {
    pub label: String,
    pub value: String,
}

#[derive(Serialize)]
pub struct LibraryItemMetadata {
    pub title:      String,
    pub subtitle:   Option<String>,
    pub summary:    Option<String>,
    pub attributes: Vec<LibraryItemAttribute>,
}

#[derive(Serialize)]
pub struct LibraryItemResources {
    pub thumbnail:  String,
    pub preview:    Vec<MediaItem>,
    pub media:      Option<Media>
}

#[derive(Serialize)]
pub struct LibraryItem {
    pub id:            String,
    pub title:         String,
    pub thumbnail_url: String,
}

#[derive(Serialize)]
pub struct LibraryItemDetail {
    pub id:        String,
    pub version:   String,
    pub metadata:  LibraryItemMetadata,
    pub resources: LibraryItemResources,
}
