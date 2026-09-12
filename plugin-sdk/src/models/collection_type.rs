use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollectionType {
    // Comic -> Volume -> Chapter -> Image
    Comic,
    Volume,
    Chapter,

    // Album -> Audio
    Album,

    // TvSeries -> Season -> Video
    TvSeries,
    Season,

    // MovieSeries -> Video
    MovieSeries,

    // Gallery -> Image
    Gallery,

    // BookSeries -> Volume -> Chapter -> Text
    BookSeries,
}
