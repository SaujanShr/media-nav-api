use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaType {
    Image,
    Video,
    Audio
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollectionType {
    Comic,
    Playlist
}

#[derive(Serialize, Deserialize)]
pub struct MediaItem {
    pub media_type: MediaType,
    pub title:      Option<String>,
    pub url:        String,
}

#[derive(Serialize, Deserialize)]
pub struct MediaCollection {
    pub collection_type: CollectionType,
    pub title: Option<String>,
    pub items: Vec<Media>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Media {
    Item(MediaItem),
    Collection(MediaCollection),
}
