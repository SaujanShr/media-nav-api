use plugin_sdk::media_item::{Media, MediaItem, MediaType};

use crate::client::{client, JIKAN_BASE_URL, CONSUMET_BASE_URL};
use crate::types::{ConsumetEpisodesResponse, ConsumetSourcesResponse, JikanAnimeResponse};

// ── Private ───────────────────────────────────────────────────────────────────

fn fetch_title(mal_id: &str) -> Option<String> {
    client()
        .get(format!("{JIKAN_BASE_URL}/anime/{mal_id}"))
        .send()
        .and_then(|r| r.json::<JikanAnimeResponse>())
        .ok()
        .map(|r| r.data.title_english.unwrap_or(r.data.title))
}

fn fetch_episodes(mal_id: &str) -> Option<ConsumetEpisodesResponse> {
    client()
        .get(format!("{CONSUMET_BASE_URL}/anime/{mal_id}/episodes"))
        .send()
        .and_then(|r|
            r.json::<ConsumetEpisodesResponse>()
        )
        .ok()
}

fn fetch_source_url(episode_id: &str) -> Option<String> {
    let r = client()
        .get(format!("{CONSUMET_BASE_URL}/episode/sources?episodeId={episode_id}"))
        .send()
        .and_then(|r| r.json::<ConsumetSourcesResponse>())
        .ok()?;

    // Prefer the embed URL; fall back to the first direct source URL.
    r.embed_url
        .or_else(|| r.sources.into_iter().next().map(|s| s.url))
}

// ── Public ────────────────────────────────────────────────────────────────────

pub fn media(mal_id: &str) -> Vec<Media> {
    let _ = fetch_title(mal_id);

    fetch_episodes(mal_id)
        .map(|r| r.episodes)
        .unwrap_or_default()
        .into_iter()
        .filter_map(|ep| {
            let embed_url = fetch_source_url(&ep.id)?;
            let title = ep.title
                .unwrap_or_else(|| format!("Episode {}", ep.number.unwrap_or(0)));

            Some(Media::Single(MediaItem {
                media_type: MediaType::Video,
                title,
                url: embed_url,
            }))
        })
        .collect()
}
