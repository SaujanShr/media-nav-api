use super::{BooleanFieldSchema, NumberFieldSchema, Query, QueryFieldSchema, SearchFieldSchema, SortFieldSchema};
use crate::query::expression::{SortDirection, SortOption};

#[test]
fn boolean_field_always_validates() {
    let field = QueryFieldSchema::Boolean(BooleanFieldSchema { default: false });
    assert!(field.validate_for(&Query::default(), "nsfw").is_ok());
}

#[test]
fn search_field_validates_against_the_matching_query_entry() {
    let field = QueryFieldSchema::Search(SearchFieldSchema {
        required: true, min_length: None, max_length: None, default: None,
    });

    assert!(field.validate_for(&Query::default(), "search").is_err());

    let mut query = Query::default();
    query.search_fields.insert("search".to_string(), "hello".to_string());
    assert!(field.validate_for(&query, "search").is_ok());
}

#[test]
fn apply_default_inserts_boolean_default() {
    let field = QueryFieldSchema::Boolean(BooleanFieldSchema { default: true });
    let mut query = Query::default();

    field.apply_default("nsfw", &mut query);

    assert_eq!(query.boolean_fields.get("nsfw"), Some(&true));
}

#[test]
fn apply_default_inserts_sort_default() {
    let field = QueryFieldSchema::Sort(SortFieldSchema {
        supported: vec![SortOption { sort: "title".to_string(), ascending: true, descending: true }],
        default_sort: "title".to_string(),
        default_direction: SortDirection::Asc,
    });
    let mut query = Query::default();

    field.apply_default("sort", &mut query);

    let sort = query.sort_fields.get("sort").unwrap();
    assert_eq!(sort.sort, "title");
}

#[test]
fn apply_default_skips_number_field_with_no_defaults() {
    let field = QueryFieldSchema::Number(NumberFieldSchema {
        min: None, max: None, float: true, range: true,
        required_from: false, required_to: false,
        default_from: None, default_to: None,
    });
    let mut query = Query::default();

    field.apply_default("year", &mut query);

    assert!(query.number_fields.get("year").is_none());
}
