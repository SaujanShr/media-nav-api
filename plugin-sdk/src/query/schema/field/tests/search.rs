use super::SearchFieldSchema;

fn schema() -> SearchFieldSchema {
    SearchFieldSchema { required: false, min_length: None, max_length: None, default: None }
}

#[test]
fn required_field_rejects_missing_value() {
    let schema = SearchFieldSchema { required: true, ..schema() };
    assert!(schema.validate(None).is_err());
}

#[test]
fn required_field_rejects_blank_value() {
    let schema = SearchFieldSchema { required: true, ..schema() };
    assert!(schema.validate(Some("   ")).is_err());
}

#[test]
fn optional_field_accepts_missing_value() {
    assert!(schema().validate(None).is_ok());
}

#[test]
fn rejects_value_shorter_than_min_length() {
    let schema = SearchFieldSchema { min_length: Some(3), ..schema() };
    assert!(schema.validate(Some("ab")).is_err());
}

#[test]
fn rejects_value_longer_than_max_length() {
    let schema = SearchFieldSchema { max_length: Some(3), ..schema() };
    assert!(schema.validate(Some("abcd")).is_err());
}

#[test]
fn accepts_value_within_length_bounds() {
    let schema = SearchFieldSchema { min_length: Some(2), max_length: Some(5), ..schema() };
    assert!(schema.validate(Some("abc")).is_ok());
}
