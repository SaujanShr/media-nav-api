use std::collections::{HashMap, HashSet};

use plugin_sdk::query::expression::{SortDirection, SortOption};
use plugin_sdk::query::partial_date::{DateGranularity, PartialDate};
use plugin_sdk::query::query::Query;
use plugin_sdk::query::schema::{
    BooleanFieldSchema, DateFieldSchema, FilterFieldSchema, NumberFieldSchema,
    QueryFieldSchema, QuerySchema, SearchFieldSchema, SortFieldSchema,
};

use crate::types::{AnimeOrderBy, AnimeRating, AnimeStatus, AnimeType};

// ── Config ────────────────────────────────────────────────────────────────────

const KEY_Q:               &str = "q";
const KEY_TYPE:            &str = "type";
const KEY_SCORE:           &str = "score";
const KEY_STATUS:          &str = "status";
const KEY_RATING:          &str = "rating";
const KEY_SFW:             &str = "sfw";
const KEY_ORDER_BY:        &str = "order_by";
const KEY_START_YEAR:      &str = "start_year";

// ── Types ─────────────────────────────────────────────────────────────────────

pub struct AnimeRequest {
    pub search:           Option<String>,
    pub anime_type:       Option<String>,
    pub score_min:        Option<f32>,
    pub score_max:        Option<f32>,
    pub status:           Option<String>,
    pub rating:           Option<String>,
    pub sfw:              bool,
    pub order_by:         String,
    pub sort_dir:         &'static str,
    pub start_year:       Option<PartialDate>,
}

// ── Public ────────────────────────────────────────────────────────────────────

pub fn parse(query: &Query) -> AnimeRequest {
    let search = query.search_fields.get(KEY_Q).cloned();

    let anime_type = query.filter_fields
        .get(KEY_TYPE)
        .and_then(|f| f.include.iter().next().cloned());

    let score = query.number_fields.get(KEY_SCORE).copied().unwrap_or((None, None));

    let status = query.filter_fields
        .get(KEY_STATUS)
        .and_then(|f| f.include.iter().next().cloned());

    let rating = query.filter_fields
        .get(KEY_RATING)
        .and_then(|f| f.include.iter().next().cloned());

    let sfw = query.boolean_fields.get(KEY_SFW).copied().unwrap_or(true);

    let (order_by, sort_dir) = query.sort_fields
        .get(KEY_ORDER_BY)
        .map(|s| (s.sort.clone(), match s.direction {
            SortDirection::Asc  => "asc",
            SortDirection::Desc => "desc",
        }))
        .unwrap_or_else(|| ("popularity".to_string(), "asc"));

    let start_year = query.date_fields
        .get(KEY_START_YEAR)
        .and_then(|d| d.0);

    AnimeRequest {
        search, anime_type, score_min: score.0, score_max: score.1,
        status, rating, sfw, order_by, sort_dir,
        start_year,
    }
}

pub fn schema() -> QuerySchema {
    let mut fields = HashMap::new();

    fields.insert(KEY_Q, QueryFieldSchema::Search(SearchFieldSchema {
        required: false,
        min_length: None,
        max_length: Some(100),
        default: None,
    }));

    fields.insert(KEY_TYPE, QueryFieldSchema::Filter(FilterFieldSchema {
        supported: AnimeType::all().iter().map(|v| v.to_string()).collect::<HashSet<_>>(),
        multiple: false,
        include: true,
        exclude: false,
        default_include: None,
        default_exclude: None,
    }));

    fields.insert(KEY_SCORE, QueryFieldSchema::Number(NumberFieldSchema {
        min: Some(0.0),
        max: Some(10.0),
        float: true,
        range: true,
        required_from: false,
        required_to: false,
        default_from: None,
        default_to: None,
    }));

    fields.insert(KEY_STATUS, QueryFieldSchema::Filter(FilterFieldSchema {
        supported: AnimeStatus::all().iter().map(|v| v.to_string()).collect::<HashSet<_>>(),
        multiple: false,
        include: true,
        exclude: false,
        default_include: None,
        default_exclude: None,
    }));

    fields.insert(KEY_RATING, QueryFieldSchema::Filter(FilterFieldSchema {
        supported: AnimeRating::all().iter().map(|v| v.to_string()).collect::<HashSet<_>>(),
        multiple: false,
        include: true,
        exclude: false,
        default_include: None,
        default_exclude: None,
    }));

    fields.insert(KEY_SFW, QueryFieldSchema::Boolean(BooleanFieldSchema {
        default: true,
    }));

    fields.insert(KEY_START_YEAR, QueryFieldSchema::Date(DateFieldSchema {
        granularity:   DateGranularity::Year,
        min:           None,
        max:           None,
        range:         false,
        required_from: false,
        required_to:   false,
        default_from:  None,
        default_to:    None,
    }));

    fields.insert(KEY_ORDER_BY, QueryFieldSchema::Sort(SortFieldSchema {
        supported: AnimeOrderBy::all().iter().map(|v| SortOption {
            sort: v.to_string(), ascending: true, descending: true,
        }).collect::<Vec<_>>(),
        required: true,
        default_sort: AnimeOrderBy::Score.to_string(),
        default_direction: SortDirection::Desc,
    }));

    QuerySchema { fields }
}
