use serde::Serialize;

#[derive(Serialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum MediaType {
    Video,
    Audio,
    Image,
}

#[derive(Serialize, Clone)]
pub struct MediaItem {
    pub media_type: MediaType,
    pub url:        String,
}

