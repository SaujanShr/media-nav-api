use plugin_sdk::library_item::LibraryItem;
use plugin_sdk::plugin::{FetchRequest, FetchResult};

pub fn fetch(_req: FetchRequest) -> FetchResult {
    FetchResult {
        items: vec![
            LibraryItem {
                id:            "example-item-1".to_string(),
                title:         "Example Item 1".to_string(),
                thumbnail_url: "https://example.com/thumb.png".to_string(),
            },
        ],
        total: 1,
    }
}
