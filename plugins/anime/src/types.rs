use serde::Deserialize;

// ── Query Enums ───────────────────────────────────────────────────────────────

pub enum AnimeType {
    Tv, Movie, Ova, Special, Ona, Music, Cm, Pv, TvSpecial,
}

impl AnimeType {
    pub fn all() -> &'static [Self] {
        &[Self::Tv, Self::Movie, Self::Ova, Self::Special, Self::Ona,
          Self::Music, Self::Cm, Self::Pv, Self::TvSpecial]
    }
}

impl std::fmt::Display for AnimeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Tv        => "tv",
            Self::Movie     => "movie",
            Self::Ova       => "ova",
            Self::Special   => "special",
            Self::Ona       => "ona",
            Self::Music     => "music",
            Self::Cm        => "cm",
            Self::Pv        => "pv",
            Self::TvSpecial => "tv_special",
        })
    }
}

pub enum AnimeStatus {
    Airing, Complete, Upcoming,
}

impl AnimeStatus {
    pub fn all() -> &'static [Self] {
        &[Self::Airing, Self::Complete, Self::Upcoming]
    }
}

impl std::fmt::Display for AnimeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Airing   => "airing",
            Self::Complete => "complete",
            Self::Upcoming => "upcoming",
        })
    }
}

pub enum AnimeRating {
    G, Pg, Pg13, R17, R, Rx,
}

impl AnimeRating {
    pub fn all() -> &'static [Self] {
        &[Self::G, Self::Pg, Self::Pg13, Self::R17, Self::R, Self::Rx]
    }
}

impl std::fmt::Display for AnimeRating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::G    => "g",
            Self::Pg   => "pg",
            Self::Pg13 => "pg13",
            Self::R17  => "r17",
            Self::R    => "r",
            Self::Rx   => "rx",
        })
    }
}

pub enum AnimeOrderBy {
    MalId, Title, StartDate, EndDate, Episodes,
    Score, ScoredBy, Rank, Popularity, Members, Favorites,
}

impl AnimeOrderBy {
    pub fn all() -> &'static [Self] {
        &[Self::MalId, Self::Title, Self::StartDate, Self::EndDate, Self::Episodes,
          Self::Score, Self::ScoredBy, Self::Rank, Self::Popularity, Self::Members, Self::Favorites]
    }
}

impl std::fmt::Display for AnimeOrderBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::MalId      => "mal_id",
            Self::Title      => "title",
            Self::StartDate  => "start_date",
            Self::EndDate    => "end_date",
            Self::Episodes   => "episodes",
            Self::Score      => "score",
            Self::ScoredBy   => "scored_by",
            Self::Rank       => "rank",
            Self::Popularity => "popularity",
            Self::Members    => "members",
            Self::Favorites  => "favorites",
        })
    }
}

// ── Images ────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct JikanImageVariants {
    pub image_url:       String,
    pub small_image_url: String,
    pub large_image_url: String,
}

#[derive(Deserialize)]
pub struct JikanImages {
    pub jpg:  JikanImageVariants,
    pub webp: JikanImageVariants,
}

// ── Trailer ───────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct JikanTrailer {
    pub youtube_id: Option<String>,
    pub url:        Option<String>,
    pub embed_url:  Option<String>,
}

// ── Titles ────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct JikanTitle {
    #[serde(rename = "type")]
    pub title_type: String,
    pub title:      String,
}

// ── Aired ─────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct JikanAiredPropDate {
    pub day:   Option<u32>,
    pub month: Option<u32>,
    pub year:  Option<i32>,
}

#[derive(Deserialize)]
pub struct JikanAiredProp {
    pub from: JikanAiredPropDate,
    pub to:   JikanAiredPropDate,
}

#[derive(Deserialize)]
pub struct JikanAired {
    pub from: Option<String>,
    pub to:   Option<String>,
    pub prop: JikanAiredProp,
}

// ── Broadcast ─────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct JikanBroadcast {
    pub day:      Option<String>,
    pub time:     Option<String>,
    pub timezone: Option<String>,
}

// ── Entity ────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct JikanEntity {
    pub mal_id:      u64,
    #[serde(rename = "type")]
    pub entity_type: String,
    pub name:        String,
    pub url:         String,
}

// ── Anime ─────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct JikanAnime {
    pub mal_id:          u64,
    pub url:             String,
    pub images:          JikanImages,
    pub trailer:         JikanTrailer,
    pub approved:        bool,
    pub titles:          Vec<JikanTitle>,
    pub title:           String,
    pub title_english:   Option<String>,
    pub title_japanese:  Option<String>,
    pub title_synonyms:  Vec<String>,
    #[serde(rename = "type")]
    pub anime_type:      Option<String>,
    pub source:          Option<String>,
    pub episodes:        Option<u32>,
    pub status:          Option<String>,
    pub airing:          bool,
    pub aired:           JikanAired,
    pub duration:        Option<String>,
    pub rating:          Option<String>,
    pub score:           Option<f64>,
    pub scored_by:       Option<u64>,
    pub rank:            Option<u64>,
    pub popularity:      Option<u64>,
    pub members:         Option<u64>,
    pub favorites:       Option<u64>,
    pub synopsis:        Option<String>,
    pub background:      Option<String>,
    pub season:          Option<String>,
    pub year:            Option<i32>,
    pub broadcast:       JikanBroadcast,
    pub producers:       Vec<JikanEntity>,
    pub licensors:       Vec<JikanEntity>,
    pub studios:         Vec<JikanEntity>,
    pub genres:          Vec<JikanEntity>,
    pub explicit_genres: Vec<JikanEntity>,
    pub themes:          Vec<JikanEntity>,
    pub demographics:    Vec<JikanEntity>,
}

// ── Pictures ──────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct JikanPicture {
    pub jpg:  JikanImageVariants,
    pub webp: JikanImageVariants,
}

#[derive(Deserialize)]
pub struct JikanPicturesResponse {
    pub data: Vec<JikanPicture>,
}

// ── Responses ─────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct JikanPaginationItems {
    pub count:    u64,
    pub total:    u64,
    pub per_page: u64,
}

#[derive(Deserialize)]
pub struct JikanPagination {
    pub last_visible_page: u64,
    pub has_next_page:     bool,
    pub current_page:      u64,
    pub items:             JikanPaginationItems,
}

#[derive(Deserialize)]
pub struct JikanSearchResponse {
    pub data:       Vec<JikanAnime>,
    pub pagination: JikanPagination,
}

#[derive(Deserialize)]
pub struct JikanAnimeResponse {
    pub data: JikanAnime,
}

// ── Consumet ──────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct ConsumetEpisode {
    pub id:     String,
    pub title:  Option<String>,
    pub number: Option<u32>,
}

#[derive(Deserialize)]
pub struct ConsumetEpisodesResponse {
    pub episodes: Vec<ConsumetEpisode>,
}

#[derive(Deserialize)]
pub struct ConsumetSource {
    pub url:     String,
    #[serde(rename = "isM3U8")]
    pub is_m3u8: bool,
    pub quality: Option<String>,
}

#[derive(Deserialize)]
pub struct ConsumetSourcesResponse {
    pub sources:   Vec<ConsumetSource>,
    #[serde(rename = "embedURL")]
    pub embed_url: Option<String>,
}


