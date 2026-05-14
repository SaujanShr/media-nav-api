use plugin_sdk::media_item::{Media, MediaItem, MediaType};

pub fn media(_id: &str) -> Vec<Media> {
    vec![
        Media::Single(MediaItem {
            media_type: MediaType::Video,
            title:      "Example Video".to_string(),
            url:        "https://example.com/video.mp4".to_string(),
        }),
    ]
}



