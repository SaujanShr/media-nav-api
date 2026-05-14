use plugin_sdk::query::schema::{QuerySchema, QueryFieldSchema, SearchFieldSchema, SortFieldSchema};
use plugin_sdk::query::expression::{SortDirection, SortOption};
use std::collections::HashMap;

pub fn schema() -> QuerySchema {
    let mut fields = HashMap::new();

    fields.insert("search", QueryFieldSchema::Search(SearchFieldSchema {
        required:   false,
        min_length: None,
        max_length: None,
        default:    None,
    }));

    fields.insert("sort", QueryFieldSchema::Sort(SortFieldSchema {
        supported: vec![
            SortOption {
                sort: "title".to_string(),
                ascending: true,
                descending: true
            },
        ],
        default_sort:      "title".to_string(),
        default_direction: SortDirection::Asc,
    }));

    QuerySchema { fields }
}

