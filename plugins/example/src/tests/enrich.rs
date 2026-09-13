use super::{
    book_chapter_media, book_volume_media, chapter_media, media_from_kind, node_item, season_media,
    volume_media, RawAlbum, RawBookSeries, RawBookVolume, RawChapter, RawComic, RawGallery, RawKind,
    RawLeaf, RawMovieSeries, RawNode, RawSeason, RawTvSeries, RawVolume,
};
use plugin_sdk::collection_type::CollectionType;
use plugin_sdk::media_item::{Media, MediaCollection, MediaItem};
use plugin_sdk::media_type::MediaType;

#[test]
fn node_item_uses_the_provided_title() {
    let node = RawNode { title: Some("Episode 1".to_string()), url: "http://x/ep1.mp4".to_string() };
    let media = node_item(MediaType::Video, &node);

    assert_eq!(media, Media::Item(MediaItem {
        media_type: MediaType::Video,
        title:      Some("Episode 1".to_string()),
        url:        "http://x/ep1.mp4".to_string(),
    }));
}

#[test]
fn node_item_falls_back_to_the_url_filename_when_title_is_missing() {
    let node = RawNode { title: None, url: "http://x/a/b/track.mp3".to_string() };
    let media = node_item(MediaType::Audio, &node);

    assert_eq!(media, Media::Item(MediaItem {
        media_type: MediaType::Audio,
        title:      Some("track.mp3".to_string()),
        url:        "http://x/a/b/track.mp3".to_string(),
    }));
}

#[test]
fn chapter_media_numbers_pages_positionally() {
    let chapter = RawChapter {
        title: Some("Chapter 1".to_string()),
        pages: vec!["http://x/p1.jpg".to_string(), "http://x/p2.jpg".to_string()],
    };
    let media = chapter_media(&chapter);

    assert_eq!(media, Media::Collection(MediaCollection {
        collection_type: CollectionType::Chapter,
        title: Some("Chapter 1".to_string()),
        items: vec![
            Media::Item(MediaItem { media_type: MediaType::Image, title: Some("Page 1".to_string()), url: "http://x/p1.jpg".to_string() }),
            Media::Item(MediaItem { media_type: MediaType::Image, title: Some("Page 2".to_string()), url: "http://x/p2.jpg".to_string() }),
        ],
    }));
}

#[test]
fn volume_media_wraps_its_chapters() {
    let volume = RawVolume {
        title: Some("Volume 1".to_string()),
        chapters: vec![RawChapter { title: None, pages: vec!["http://x/p1.jpg".to_string()] }],
    };
    let media = volume_media(&volume);

    let Media::Collection(collection) = media else { panic!("expected a collection") };
    assert_eq!(collection.collection_type, CollectionType::Volume);
    assert_eq!(collection.title, Some("Volume 1".to_string()));
    assert_eq!(collection.items.len(), 1);
}

#[test]
fn season_media_treats_episodes_as_video() {
    let season = RawSeason {
        title: Some("Season 1".to_string()),
        episodes: vec![RawNode { title: Some("Pilot".to_string()), url: "http://x/pilot.mp4".to_string() }],
    };
    let media = season_media(&season);

    assert_eq!(media, Media::Collection(MediaCollection {
        collection_type: CollectionType::Season,
        title: Some("Season 1".to_string()),
        items: vec![Media::Item(MediaItem {
            media_type: MediaType::Video,
            title:      Some("Pilot".to_string()),
            url:        "http://x/pilot.mp4".to_string(),
        })],
    }));
}

#[test]
fn book_chapter_media_wraps_a_single_text_item() {
    let chapter = RawNode { title: Some("Chapter 1".to_string()), url: "http://x/ch1.txt".to_string() };
    let media = book_chapter_media(&chapter);

    assert_eq!(media, Media::Collection(MediaCollection {
        collection_type: CollectionType::Chapter,
        title: Some("Chapter 1".to_string()),
        items: vec![Media::Item(MediaItem {
            media_type: MediaType::Text,
            title:      Some("Chapter 1".to_string()),
            url:        "http://x/ch1.txt".to_string(),
        })],
    }));
}

#[test]
fn book_volume_media_wraps_its_chapters() {
    let volume = RawBookVolume {
        title: Some("Volume 1".to_string()),
        chapters: vec![RawNode { title: Some("Chapter 1".to_string()), url: "http://x/ch1.txt".to_string() }],
    };
    let media = book_volume_media(&volume);

    let Media::Collection(collection) = media else { panic!("expected a collection") };
    assert_eq!(collection.collection_type, CollectionType::Volume);
    assert_eq!(collection.items.len(), 1);
}

#[test]
fn media_from_kind_builds_a_leaf_item_using_the_details_title() {
    let media = media_from_kind("My Photo", &RawKind::Image(RawLeaf { url: "http://x/photo.jpg".to_string() }));

    assert_eq!(media, Media::Item(MediaItem {
        media_type: MediaType::Image,
        title:      Some("My Photo".to_string()),
        url:        "http://x/photo.jpg".to_string(),
    }));
}

#[test]
fn media_from_kind_builds_a_comic_from_nested_volumes_and_chapters() {
    let kind = RawKind::Comic(RawComic {
        volumes: vec![RawVolume {
            title: Some("Volume 1".to_string()),
            chapters: vec![RawChapter { title: Some("Chapter 1".to_string()), pages: vec!["http://x/p1.jpg".to_string()] }],
        }],
    });
    let media = media_from_kind("My Comic", &kind);

    let Media::Collection(comic) = media else { panic!("expected a collection") };
    assert_eq!(comic.collection_type, CollectionType::Comic);
    assert_eq!(comic.items.len(), 1);

    let Media::Collection(volume) = &comic.items[0] else { panic!("expected a collection") };
    assert_eq!(volume.collection_type, CollectionType::Volume);
    assert_eq!(volume.items.len(), 1);
}

#[test]
fn media_from_kind_builds_an_album_from_tracks() {
    let kind = RawKind::Album(RawAlbum {
        tracks: vec![RawNode { title: Some("Track 1".to_string()), url: "http://x/t1.mp3".to_string() }],
    });
    let media = media_from_kind("My Album", &kind);

    let Media::Collection(album) = media else { panic!("expected a collection") };
    assert_eq!(album.collection_type, CollectionType::Album);
    assert_eq!(album.items, vec![Media::Item(MediaItem {
        media_type: MediaType::Audio,
        title:      Some("Track 1".to_string()),
        url:        "http://x/t1.mp3".to_string(),
    })]);
}

#[test]
fn media_from_kind_builds_a_tv_series_from_seasons_and_episodes() {
    let kind = RawKind::TvSeries(RawTvSeries {
        seasons: vec![RawSeason {
            title: Some("Season 1".to_string()),
            episodes: vec![RawNode { title: Some("Pilot".to_string()), url: "http://x/pilot.mp4".to_string() }],
        }],
    });
    let media = media_from_kind("My Show", &kind);

    let Media::Collection(tv) = media else { panic!("expected a collection") };
    assert_eq!(tv.collection_type, CollectionType::TvSeries);
    assert_eq!(tv.items.len(), 1);
}

#[test]
fn media_from_kind_builds_a_movie_series_from_movies() {
    let kind = RawKind::MovieSeries(RawMovieSeries {
        movies: vec![RawNode { title: Some("Part 1".to_string()), url: "http://x/p1.mp4".to_string() }],
    });
    let media = media_from_kind("My Series", &kind);

    let Media::Collection(series) = media else { panic!("expected a collection") };
    assert_eq!(series.collection_type, CollectionType::MovieSeries);
    assert_eq!(series.items, vec![Media::Item(MediaItem {
        media_type: MediaType::Video,
        title:      Some("Part 1".to_string()),
        url:        "http://x/p1.mp4".to_string(),
    })]);
}

#[test]
fn media_from_kind_builds_a_gallery_from_images() {
    let kind = RawKind::Gallery(RawGallery {
        images: vec![RawNode { title: Some("Front".to_string()), url: "http://x/front.jpg".to_string() }],
    });
    let media = media_from_kind("My Gallery", &kind);

    let Media::Collection(gallery) = media else { panic!("expected a collection") };
    assert_eq!(gallery.collection_type, CollectionType::Gallery);
    assert_eq!(gallery.items, vec![Media::Item(MediaItem {
        media_type: MediaType::Image,
        title:      Some("Front".to_string()),
        url:        "http://x/front.jpg".to_string(),
    })]);
}

#[test]
fn media_from_kind_builds_a_book_series_from_volumes_and_chapters() {
    let kind = RawKind::BookSeries(RawBookSeries {
        volumes: vec![RawBookVolume {
            title: Some("Volume 1".to_string()),
            chapters: vec![RawNode { title: Some("Chapter 1".to_string()), url: "http://x/ch1.txt".to_string() }],
        }],
    });
    let media = media_from_kind("My Book", &kind);

    let Media::Collection(book) = media else { panic!("expected a collection") };
    assert_eq!(book.collection_type, CollectionType::BookSeries);
    assert_eq!(book.items.len(), 1);

    let Media::Collection(volume) = &book.items[0] else { panic!("expected a collection") };
    assert_eq!(volume.collection_type, CollectionType::Volume);
    assert_eq!(volume.items.len(), 1);
}
