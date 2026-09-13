use super::NumberFieldSchema;

fn schema() -> NumberFieldSchema {
    NumberFieldSchema {
        min: None, max: None, float: true, range: true,
        required_from: false, required_to: false,
        default_from: None, default_to: None,
    }
}

#[test]
fn required_from_rejects_missing_value() {
    let schema = NumberFieldSchema { required_from: true, ..schema() };
    assert!(schema.validate(None).is_err());
    assert!(schema.validate(Some((None, Some(5.0)))).is_err());
}

#[test]
fn required_to_rejects_missing_value() {
    let schema = NumberFieldSchema { required_to: true, ..schema() };
    assert!(schema.validate(Some((Some(5.0), None))).is_err());
}

#[test]
fn rejects_from_greater_than_to() {
    assert!(schema().validate(Some((Some(10.0), Some(5.0)))).is_err());
}

#[test]
fn accepts_from_equal_to_to() {
    assert!(schema().validate(Some((Some(5.0), Some(5.0)))).is_ok());
}

#[test]
fn rejects_non_integer_when_float_disallowed() {
    let schema = NumberFieldSchema { float: false, ..schema() };
    assert!(schema.validate(Some((Some(1.5), None))).is_err());
}

#[test]
fn accepts_integer_when_float_disallowed() {
    let schema = NumberFieldSchema { float: false, ..schema() };
    assert!(schema.validate(Some((Some(2.0), None))).is_ok());
}

#[test]
fn rejects_value_outside_min_max() {
    let schema = NumberFieldSchema { min: Some(0.0), max: Some(10.0), ..schema() };
    assert!(schema.validate(Some((Some(-1.0), None))).is_err());
    assert!(schema.validate(Some((Some(11.0), None))).is_err());
    assert!(schema.validate(Some((Some(5.0), None))).is_ok());
}
