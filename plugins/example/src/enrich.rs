use plugin_sdk::library_item::{LibraryItemDetail, LibraryItemMetadata, LibraryItemResources};
use plugin_sdk::media::{MediaItem, MediaType};

pub fn enrich(id: &str) -> Option<LibraryItemDetail> {
    match id {
        "example-item-1" => Some(LibraryItemDetail {
            id: id.to_string(),
            metadata: LibraryItemMetadata {
                title:      "Example Item".to_string(),
                subtitle:   None,
                summary:    None,
                attributes: vec![],
            },
            resources: LibraryItemResources {
                thumbnail: MediaItem {
                    media_type: MediaType::Image,
                    url:        "https://example.com/thumb.png".to_string(),
                },
                preview_items: vec![],
            },
        }),
        _ => None,
    }
}
