use serde::{Serialize, Deserialize};

use crate::collection_type::CollectionType;
use crate::media_type::MediaType;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaItem {
    pub media_type: MediaType,
    pub title:      Option<String>,
    pub url:        String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaCollection {
    pub collection_type: CollectionType,
    pub title: Option<String>,
    pub items: Vec<Media>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Media {
    Item(MediaItem),
    Collection(MediaCollection),
}
