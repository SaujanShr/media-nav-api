use plugin_sdk::library_item::{LibraryItemAttribute, LibraryItemDetail, LibraryItemMetadata, LibraryItemResources};
use plugin_sdk::media_item::{Media, MediaItem, MediaType};

pub fn enrich(id: &str) -> Option<LibraryItemDetail> {
    match id {
        "example-item-1" => Some(LibraryItemDetail {
            id: id.to_string(),
            version: Some("1.0".to_string()),
            metadata: LibraryItemMetadata {
                title:      "Example Item".to_string(),
                subtitle:   Some("Example Subtitle".to_string()),
                summary:    Some("Example Summary".to_string()),
                attributes: vec![
                    LibraryItemAttribute {
                        label: "Attribute 1".to_string(),
                        value: "Value 1".to_string(),
                    },
                    LibraryItemAttribute {
                        label: "Attribute 2".to_string(),
                        value: "Value 2".to_string(),
                    },
                    LibraryItemAttribute {
                        label: "Attribute 3".to_string(),
                        value: "Value 3".to_string(),
                    },
                ],
            },
            resources: LibraryItemResources {
                thumbnail: "https://example.com/thumb.png".to_string(),
                preview: vec![
                    MediaItem {
                        media_type: MediaType::Image,
                        title: Some("Preview 1".to_string()),
                        url: "https://example.com/thumb.png".to_string(),
                    },
                    MediaItem {
                        media_type: MediaType::Image,
                        title: Some("Preview 2".to_string()),
                        url: "https://example.com/thumb.png".to_string(),
                    }
                ],
                media: Some(Media::Item(
                    MediaItem {
                        media_type: MediaType::Video,
                        title: Some("Example Video".to_string()),
                        url: "https://example.com/video.mp4".to_string(),
                    }
                ))
            },
        }),
        _ => None,
    }
}
