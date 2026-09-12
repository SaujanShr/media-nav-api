use plugin_sdk::guest::{fetch_json_optional, filename_from_url};
use plugin_sdk::library_item::{LibraryItemDetail, LibraryItemMetadata, LibraryItemResources};
use plugin_sdk::media_item::{CollectionType, Media, MediaCollection, MediaItem, MediaType};
use plugin_sdk::plugin::PluginCallError;
use plugin_sdk::push_attr;
use serde::Deserialize;

const PROVIDER_URL: &str = "http://localhost:4000";

#[derive(Deserialize)]
struct RawDetail {
    id:           String,
    version:      Option<String>,
    title:        String,
    subtitle:     Option<String>,
    description:  Option<String>,
    genre:        Option<String>,
    year:         Option<i32>,
    rating:       Option<f32>,
    type_label:   Option<String>,
    thumbnail:    String,
    preview_urls: Vec<String>,
    video_urls:   Vec<String>,
    audio_urls:   Vec<String>,
}

fn media_item_from_url(media_type: MediaType, url: &str) -> MediaItem {
    MediaItem {
        media_type,
        title: Some(filename_from_url(url)),
        url:   url.to_string(),
    }
}

fn media_from_raw(raw: &RawDetail) -> Option<Media> {
    if let Some(url) = raw.audio_urls.first() {
        return Some(Media::Item(media_item_from_url(MediaType::Audio, url)));
    }

    match raw.video_urls.as_slice() {
        [] => None,
        [url] => Some(Media::Item(media_item_from_url(MediaType::Video, url))),
        urls => Some(Media::Collection(MediaCollection {
            collection_type: CollectionType::Playlist,
            title: None,
            items: urls.iter().map(|u| Media::Item(media_item_from_url(MediaType::Video, u))).collect(),
        })),
    }
}

pub fn enrich(id: &str) -> Result<Option<LibraryItemDetail>, PluginCallError> {
    let url = format!("{}/items/{}", PROVIDER_URL, id);

    let Some(raw) = fetch_json_optional::<RawDetail>(&url)? else {
        return Ok(None);
    };

    let media = media_from_raw(&raw);
    let preview = raw.preview_urls
        .iter()
        .map(|u| media_item_from_url(MediaType::Image, u))
        .collect();

    let mut attributes = Vec::new();
    push_attr!(attributes, "Genre", raw.genre);
    push_attr!(attributes, "Year", raw.year);
    push_attr!(attributes, "Rating", raw.rating);
    push_attr!(attributes, "Type", raw.type_label);

    Ok(Some(LibraryItemDetail {
        id:      raw.id,
        version: raw.version,
        metadata: LibraryItemMetadata {
            title:    raw.title,
            subtitle: raw.subtitle,
            summary:  raw.description,
            attributes,
        },
        resources: LibraryItemResources {
            thumbnail: raw.thumbnail,
            preview,
            media,
        },
    }))
}
