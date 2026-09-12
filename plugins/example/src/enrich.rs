use plugin_sdk::collection_type::CollectionType;
use plugin_sdk::guest::{fetch_json_optional, filename_from_url};
use plugin_sdk::library_item::{LibraryItemDetail, LibraryItemMetadata, LibraryItemResources};
use plugin_sdk::media_item::{Media, MediaCollection, MediaItem};
use plugin_sdk::media_type::MediaType;
use plugin_sdk::plugin::PluginCallError;
use plugin_sdk::push_attr;
use serde::Deserialize;

const PROVIDER_URL: &str = "http://localhost:4000";

// ── Raw shapes ────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct RawLeaf {
    url: String,
}

#[derive(Deserialize)]
struct RawNode {
    title: Option<String>,
    url:   String,
}

#[derive(Deserialize)]
struct RawChapter {
    title: Option<String>,
    pages: Vec<String>,
}

#[derive(Deserialize)]
struct RawVolume {
    title:    Option<String>,
    chapters: Vec<RawChapter>,
}

#[derive(Deserialize)]
struct RawComic {
    volumes: Vec<RawVolume>,
}

#[derive(Deserialize)]
struct RawAlbum {
    tracks: Vec<RawNode>,
}

#[derive(Deserialize)]
struct RawSeason {
    title:    Option<String>,
    episodes: Vec<RawNode>,
}

#[derive(Deserialize)]
struct RawTvSeries {
    seasons: Vec<RawSeason>,
}

#[derive(Deserialize)]
struct RawMovieSeries {
    movies: Vec<RawNode>,
}

#[derive(Deserialize)]
struct RawGallery {
    images: Vec<RawNode>,
}

#[derive(Deserialize)]
struct RawBookVolume {
    title:    Option<String>,
    chapters: Vec<RawNode>,
}

#[derive(Deserialize)]
struct RawBookSeries {
    volumes: Vec<RawBookVolume>,
}

#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum RawKind {
    Image(RawLeaf),
    Video(RawLeaf),
    Audio(RawLeaf),
    Text(RawLeaf),
    Comic(RawComic),
    Album(RawAlbum),
    TvSeries(RawTvSeries),
    MovieSeries(RawMovieSeries),
    Gallery(RawGallery),
    BookSeries(RawBookSeries),
}

#[derive(Deserialize)]
struct RawDetail {
    id:          String,
    version:     Option<String>,
    title:       String,
    subtitle:    Option<String>,
    description: Option<String>,
    genre:       Option<String>,
    year:        Option<i32>,
    rating:      Option<f32>,
    thumbnail:   String,
    #[serde(flatten)]
    kind: RawKind,
}

// ── Transformation ────────────────────────────────────────────────────────────

fn node_item(media_type: MediaType, node: &RawNode) -> Media {
    Media::Item(MediaItem {
        media_type,
        title: node.title.clone().or_else(|| Some(filename_from_url(&node.url))),
        url:   node.url.clone(),
    })
}

fn chapter_media(chapter: &RawChapter) -> Media {
    Media::Collection(MediaCollection {
        collection_type: CollectionType::Chapter,
        title: chapter.title.clone(),
        items: chapter.pages.iter().enumerate().map(|(i, url)| Media::Item(MediaItem {
            media_type: MediaType::Image,
            title:      Some(format!("Page {}", i + 1)),
            url:        url.clone(),
        })).collect(),
    })
}

fn volume_media(volume: &RawVolume) -> Media {
    Media::Collection(MediaCollection {
        collection_type: CollectionType::Volume,
        title: volume.title.clone(),
        items: volume.chapters.iter().map(chapter_media).collect(),
    })
}

fn season_media(season: &RawSeason) -> Media {
    Media::Collection(MediaCollection {
        collection_type: CollectionType::Season,
        title: season.title.clone(),
        items: season.episodes.iter().map(|ep| node_item(MediaType::Video, ep)).collect(),
    })
}

fn book_chapter_media(chapter: &RawNode) -> Media {
    Media::Collection(MediaCollection {
        collection_type: CollectionType::Chapter,
        title: chapter.title.clone(),
        items: vec![node_item(MediaType::Text, chapter)],
    })
}

fn book_volume_media(volume: &RawBookVolume) -> Media {
    Media::Collection(MediaCollection {
        collection_type: CollectionType::Volume,
        title: volume.title.clone(),
        items: volume.chapters.iter().map(book_chapter_media).collect(),
    })
}

fn media_from_kind(title: &str, kind: &RawKind) -> Media {
    let leaf = |media_type, url: &String| node_item(media_type, &RawNode {
        title: Some(title.to_string()),
        url:   url.clone(),
    });

    match kind {
        RawKind::Image(leaf_url) => leaf(MediaType::Image, &leaf_url.url),
        RawKind::Video(leaf_url) => leaf(MediaType::Video, &leaf_url.url),
        RawKind::Audio(leaf_url) => leaf(MediaType::Audio, &leaf_url.url),
        RawKind::Text(leaf_url)  => leaf(MediaType::Text,  &leaf_url.url),

        RawKind::Comic(comic) => Media::Collection(MediaCollection {
            collection_type: CollectionType::Comic,
            title: None,
            items: comic.volumes.iter().map(volume_media).collect(),
        }),
        RawKind::Album(album) => Media::Collection(MediaCollection {
            collection_type: CollectionType::Album,
            title: None,
            items: album.tracks.iter().map(|t| node_item(MediaType::Audio, t)).collect(),
        }),
        RawKind::TvSeries(tv) => Media::Collection(MediaCollection {
            collection_type: CollectionType::TvSeries,
            title: None,
            items: tv.seasons.iter().map(season_media).collect(),
        }),
        RawKind::MovieSeries(series) => Media::Collection(MediaCollection {
            collection_type: CollectionType::MovieSeries,
            title: None,
            items: series.movies.iter().map(|m| node_item(MediaType::Video, m)).collect(),
        }),
        RawKind::Gallery(gallery) => Media::Collection(MediaCollection {
            collection_type: CollectionType::Gallery,
            title: None,
            items: gallery.images.iter().map(|i| node_item(MediaType::Image, i)).collect(),
        }),
        RawKind::BookSeries(series) => Media::Collection(MediaCollection {
            collection_type: CollectionType::BookSeries,
            title: None,
            items: series.volumes.iter().map(book_volume_media).collect(),
        }),
    }
}

// ── Public ────────────────────────────────────────────────────────────────────

pub fn enrich(id: &str) -> Result<Option<LibraryItemDetail>, PluginCallError> {
    let url = format!("{}/items/{}", PROVIDER_URL, id);

    let Some(raw) = fetch_json_optional::<RawDetail>(&url)? else {
        return Ok(None);
    };

    let media = media_from_kind(&raw.title, &raw.kind);

    let mut attributes = Vec::new();
    push_attr!(attributes, "Genre", raw.genre);
    push_attr!(attributes, "Year", raw.year);
    push_attr!(attributes, "Rating", raw.rating);

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
            preview:   Vec::new(),
            media:     Some(media),
        },
    }))
}
