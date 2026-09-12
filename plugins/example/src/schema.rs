use std::collections::HashMap;

use plugin_sdk::query::expression::{SortDirection, SortOption};
use plugin_sdk::query::schema::{QueryFieldSchema, QuerySchema, SearchFieldSchema, SortFieldSchema};

pub fn schema() -> QuerySchema {
    let mut fields = HashMap::new();

    fields.insert("search".to_string(), QueryFieldSchema::Search(SearchFieldSchema {
        required:   false,
        min_length: None,
        max_length: None,
        default:    None,
    }));

    fields.insert("sort".to_string(), QueryFieldSchema::Sort(SortFieldSchema {
        supported: vec![
            SortOption {
                sort: "title".to_string(),
                ascending: true,
                descending: true,
            },
        ],
        default_sort:      "title".to_string(),
        default_direction: SortDirection::Asc,
    }));

    QuerySchema { fields }
}
