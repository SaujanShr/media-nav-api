use plugin_sdk::guest::{encode_query_param, fetch_json};
use plugin_sdk::library_item::LibraryItem;
use plugin_sdk::plugin::{FetchRequest, FetchResult, PluginCallError};
use plugin_sdk::query::expression::SortDirection;
use serde::Deserialize;

const PROVIDER_URL: &str = "http://localhost:4000";

#[derive(Deserialize)]
struct RawItem {
    id: String,
    title: String,
    thumbnail: String,
}

#[derive(Deserialize)]
struct ProviderResponse {
    items: Vec<RawItem>,
    total: u64,
}

pub fn fetch(req: FetchRequest) -> Result<FetchResult, PluginCallError> {
    let mut url = format!(
        "{}/items?page={}&limit={}",
        PROVIDER_URL,
        req.page,
        req.page_size
    );

    if let Some(search) = req.query.search_fields.get("search") {
        url.push_str(&format!("&search={}", encode_query_param(search)));
    }

    if let Some(sort) = req.query.sort_fields.get("sort") {
        let direction = match sort.direction {
            SortDirection::Asc  => "asc",
            SortDirection::Desc => "desc",
        };
        url.push_str(&format!("&sort={}&direction={}", encode_query_param(&sort.sort), direction));
    }

    let data: ProviderResponse = fetch_json(&url)?;

    let items = data.items
        .into_iter()
        .map(|raw| LibraryItem {
            id:            raw.id,
            title:         raw.title,
            thumbnail_url: raw.thumbnail,
        })
        .collect();

    Ok(FetchResult { items, total: data.total })
}
