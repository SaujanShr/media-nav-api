use super::{SortDirection, SortExpression, SortFieldSchema, SortOption};

fn schema() -> SortFieldSchema {
    SortFieldSchema {
        supported: vec![SortOption { sort: "title".to_string(), ascending: true, descending: false }],
        default_sort: "title".to_string(),
        default_direction: SortDirection::Asc,
    }
}

#[test]
fn missing_value_is_valid() {
    assert!(schema().validate(None).is_ok());
}

#[test]
fn rejects_unsupported_field() {
    let expr = SortExpression { sort: "year".to_string(), direction: SortDirection::Asc };
    assert!(schema().validate(Some(&expr)).is_err());
}

#[test]
fn accepts_allowed_direction() {
    let expr = SortExpression { sort: "title".to_string(), direction: SortDirection::Asc };
    assert!(schema().validate(Some(&expr)).is_ok());
}

#[test]
fn rejects_disallowed_direction() {
    let expr = SortExpression { sort: "title".to_string(), direction: SortDirection::Desc };
    assert!(schema().validate(Some(&expr)).is_err());
}
