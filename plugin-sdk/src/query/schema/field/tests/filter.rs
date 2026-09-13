use super::{FilterExpression, FilterFieldSchema, HashSet};

fn schema() -> FilterFieldSchema {
    FilterFieldSchema {
        supported: HashSet::from(["a".to_string(), "b".to_string()]),
        multiple: true, include: true, exclude: true,
        default_include: None, default_exclude: None,
    }
}

#[test]
fn missing_value_is_valid() {
    assert!(schema().validate(None).is_ok());
}

#[test]
fn accepts_supported_selections() {
    let expr = FilterExpression {
        include: HashSet::from(["a".to_string()]),
        exclude: HashSet::from(["b".to_string()]),
    };
    assert!(schema().validate(Some(&expr)).is_ok());
}

#[test]
fn rejects_unsupported_include() {
    let expr = FilterExpression {
        include: HashSet::from(["c".to_string()]),
        exclude: HashSet::new(),
    };
    assert!(schema().validate(Some(&expr)).is_err());
}

#[test]
fn rejects_unsupported_exclude() {
    let expr = FilterExpression {
        include: HashSet::new(),
        exclude: HashSet::from(["c".to_string()]),
    };
    assert!(schema().validate(Some(&expr)).is_err());
}
