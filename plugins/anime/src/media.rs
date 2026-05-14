use plugin_sdk::media_item::{Media, MediaItem, MediaType};
use plugin_sdk::utils::filename_from_url;

use crate::client::{client, BASE_URL};
use crate::types::JikanAnimeResponse;

// ── Private ───────────────────────────────────────────────────────────────────

fn fetch_trailer_url(id: &str) -> Option<String> {
    let client = client();
    client
        .get(format!("{BASE_URL}/anime/{id}"))
        .send()
        .and_then(|r| r.json::<JikanAnimeResponse>())
        .ok()
        .and_then(|r| {
            r.data.trailer.embed_url
                .or(r.data.trailer.url)
        })
}

// ── Public ────────────────────────────────────────────────────────────────────

pub fn media(id: &str) -> Vec<Media> {
    let mut items: Vec<Media> = Vec::new();

    if let Some(url) = fetch_trailer_url(id) {
        items.push(Media::Single(MediaItem {
            media_type: MediaType::Video,
            title:      filename_from_url(&url),
            url,
        }));
    }

    items
}



