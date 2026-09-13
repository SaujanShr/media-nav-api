use super::{HashMap, Query, QueryFieldSchema, QuerySchema};
use crate::query::schema::{BooleanFieldSchema, SearchFieldSchema};

#[test]
fn validate_ok_when_no_fields_are_declared() {
    let schema = QuerySchema { fields: HashMap::new() };
    assert!(schema.validate(&Query::default()).is_ok());
}

#[test]
fn validate_collects_errors_from_every_failing_field() {
    let mut fields = HashMap::new();
    fields.insert("search".to_string(), QueryFieldSchema::Search(SearchFieldSchema {
        required: true, min_length: None, max_length: None, default: None,
    }));
    fields.insert("nsfw".to_string(), QueryFieldSchema::Boolean(BooleanFieldSchema { default: false }));

    let schema = QuerySchema { fields };
    let errors = schema.validate(&Query::default()).unwrap_err();

    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].field, "search");
}

#[test]
fn default_applies_every_fields_default() {
    let mut fields = HashMap::new();
    fields.insert("nsfw".to_string(), QueryFieldSchema::Boolean(BooleanFieldSchema { default: true }));

    let schema = QuerySchema { fields };
    let query = schema.default();

    assert_eq!(query.boolean_fields.get("nsfw"), Some(&true));
}
