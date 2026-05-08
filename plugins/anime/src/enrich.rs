use plugin_sdk::library_item::{LibraryItemAttribute, LibraryItemDetail, LibraryItemMetadata, LibraryItemResources};
use plugin_sdk::media_item::{MediaItem, MediaType};
use plugin_sdk::utils::filename_from_url;
use plugin_sdk::push_attr;
use reqwest::blocking::Client;

use crate::client::{client, BASE_URL};
use crate::types::{JikanAnime, JikanAnimeResponse, JikanPicturesResponse};

// ── Private ───────────────────────────────────────────────────────────────────

fn fetch_anime(client: &Client, id: &str) -> Option<JikanAnime> {
    client
        .get(format!("{BASE_URL}/anime/{id}"))
        .send()
        .and_then(|r| r.json::<JikanAnimeResponse>())
        .ok()
        .map(|r| r.data)
}

fn fetch_pictures(client: &Client, id: &str) -> Option<JikanPicturesResponse> {
    client
        .get(format!("{BASE_URL}/anime/{id}/pictures"))
        .send()
        .and_then(|r| r.json())
        .ok()
}

fn build_preview_items(
    anime: &JikanAnime,
    pictures: Option<JikanPicturesResponse>
) -> Vec<MediaItem> {
    let mut items: Vec<MediaItem> = Vec::new();

    if let Some(url) = anime.trailer.embed_url.as_deref().or(anime.trailer.url.as_deref()) {
        items.push(MediaItem {
            title:      filename_from_url(url),
            media_type: MediaType::Video,
            url:        url.to_string()
        });
    }

    if let Some(pics) = pictures {
        for pic in pics.data {
            items.push(MediaItem {
                title:      filename_from_url(&pic.jpg.large_image_url),
                media_type: MediaType::Image,
                url:        pic.jpg.large_image_url
            });
        }
    }

    items
}

fn build_attributes(anime: &JikanAnime) -> Vec<LibraryItemAttribute> {
    let mut attrs: Vec<LibraryItemAttribute> = Vec::new();

    push_attr!(attrs, "Type",       anime.anime_type.as_deref());
    push_attr!(attrs, "Episodes",   anime.episodes);
    push_attr!(attrs, "Status",     anime.status.as_deref());
    push_attr!(attrs, "Rating",     anime.rating.as_deref());
    push_attr!(attrs, "Score",      anime.score);
    push_attr!(attrs, "Popularity", anime.popularity);

    match (anime.season.as_deref(), anime.year) {
        (Some(s), Some(y)) => attrs.push(
            LibraryItemAttribute { label: "Season".into(), value: format!("{s} {y}") }
        ),
        (Some(s), None) => attrs.push(
            LibraryItemAttribute { label: "Season".into(), value: s.to_string() }
        ),
        (None, Some(y)) => attrs.push(
            LibraryItemAttribute { label: "Season".into(), value: y.to_string() }
        ),
        _ => {}
    }

    attrs
}

// ── Public ────────────────────────────────────────────────────────────────────

pub fn enrich(id: &str) -> Option<LibraryItemDetail> {
    let client = client();

    let anime = fetch_anime(&client, id)?;
    let pictures = fetch_pictures(&client, id);

    let title = anime.title_english.clone().unwrap_or_else(|| anime.title.clone());
    let summary = anime.synopsis.clone();
    let attributes = build_attributes(&anime);
    let preview_items = build_preview_items(&anime, pictures);
    let thumbnail_url = anime.images.jpg.large_image_url.clone();

    Some(LibraryItemDetail {
        id: anime.mal_id.to_string(),
        metadata: LibraryItemMetadata {
            title,
            subtitle: None,
            summary,
            attributes,
        },
        resources: LibraryItemResources {
            thumbnail: MediaItem {
                title:      filename_from_url(&thumbnail_url),
                media_type: MediaType::Image,
                url:        thumbnail_url
            },
            preview_items,
        },
    })
}
