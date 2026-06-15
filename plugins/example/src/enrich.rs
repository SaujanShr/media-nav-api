use plugin_sdk::library_item::LibraryItemDetail;

const PROVIDER_URL: &str = "http://localhost:4000";

pub fn enrich(id: &str) -> Option<LibraryItemDetail> {
    let url = format!("{}/items/{}", PROVIDER_URL, id);

    match ureq::get(&url).call() {
        Ok(mut response) => {
            match response.body_mut().read_json::<LibraryItemDetail>() {
                Ok(detail) => Some(detail),
                Err(e) => {
                    eprintln!("Failed to parse item detail for {}: {}", id, e);
                    None
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to enrich item {}: {}", id, e);
            None
        }
    }
}
