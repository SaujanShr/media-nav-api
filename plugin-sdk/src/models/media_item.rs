use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaType {
    Image,
    Video
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CollectionType {
    Comic
}

#[derive(Serialize)]
pub struct MediaItem {
    pub media_type: MediaType,
    pub title:      Option<String>,
    pub url:        String,
}

#[derive(Serialize)]
pub struct MediaCollection {
    pub collection_type: CollectionType,
    pub title: Option<String>,
    pub items: Vec<Media>,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Media {
    Item(MediaItem),
    Collection(MediaCollection),
}
