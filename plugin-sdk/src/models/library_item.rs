use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct LibraryItemMetadata {
    pub title: String,
}

#[derive(Serialize, Clone)]
pub struct LibraryItemResources {
    pub thumbnail_url: String,
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
