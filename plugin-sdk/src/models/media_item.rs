use serde::Serialize;

#[derive(Serialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum MediaType {
    Text,
    Video,
    Audio,
    Image,
    Gallery
}

#[derive(Serialize, Clone)]
pub struct MediaItem {
    pub media_type: MediaType,
    pub title:      String,
    pub url:        String,
}

#[derive(Serialize, Clone)]
pub struct MediaCollection {
    pub title: String,
    pub items: Vec<Media>,
}

#[derive(Serialize, Clone)]
#[serde(untagged)]
pub enum Media {
    Single(MediaItem),
    Collection(MediaCollection),
}
