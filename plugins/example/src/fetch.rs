use plugin_sdk::library_item::LibraryItem;
use plugin_sdk::plugin::{FetchRequest, FetchResult};
use serde::Deserialize;

const PROVIDER_URL: &str = "http://localhost:4000";

#[derive(Deserialize)]
struct ProviderResponse {
    items: Vec<LibraryItem>,
    total: u64,
}

pub fn fetch(req: FetchRequest) -> FetchResult {
    let url = format!(
        "{}/items?page={}&limit={}",
        PROVIDER_URL,
        req.page,
        req.page_size
    );

    match ureq::get(&url).call() {
        Ok(mut response) => {
            match response.body_mut().read_json::<ProviderResponse>() {
                Ok(data) => FetchResult {
                    items: data.items,
                    total: data.total,
                },
                Err(e) => {
                    eprintln!("Failed to parse provider response: {}", e);
                    FetchResult {
                        items: vec![],
                        total: 0,
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to fetch from provider: {}", e);
            FetchResult {
                items: vec![],
                total: 0,
            }
        }
    }
}
