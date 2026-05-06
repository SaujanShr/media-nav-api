use serde::Serialize;

/// Lightweight item returned by a plugin's `fetch` — used in list views.
#[derive(Serialize, Clone)]
pub struct LibraryItem {
    pub id:            String,
    pub title:         String,
    pub thumbnail_url: String,
}

/// Richer metadata for a single item, returned by a plugin's `enrich`.
#[derive(Serialize, Clone)]
pub struct LibraryItemMetadata {
    pub title: String,
}

/// Full resources for a single item, returned by a plugin's `enrich`.
#[derive(Serialize, Clone)]
pub struct LibraryItemResources {
    pub thumbnail_url: String,
}

/// Full contextual data for a single item, returned by a plugin's `enrich`.
#[derive(Serialize, Clone)]
pub struct LibraryItemDetail {
    pub id:        String,
    pub metadata:  LibraryItemMetadata,
    pub resources: LibraryItemResources,
}
