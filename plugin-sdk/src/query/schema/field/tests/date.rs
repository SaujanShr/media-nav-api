use super::{DateFieldSchema, PartialDate};

fn schema() -> DateFieldSchema {
    DateFieldSchema {
        variant: PartialDate::Year(0), min: None, max: None, range: true,
        required_from: false, required_to: false,
        default_from: None, default_to: None,
    }
}

#[test]
fn required_from_rejects_missing_value() {
    let schema = DateFieldSchema { required_from: true, ..schema() };
    assert!(schema.validate(None).is_err());
}

#[test]
fn required_to_rejects_missing_value() {
    let schema = DateFieldSchema { required_to: true, ..schema() };
    assert!(schema.validate(Some((Some(PartialDate::Year(2020)), None))).is_err());
}

#[test]
fn rejects_from_after_to() {
    let from = Some(PartialDate::Year(2021));
    let to = Some(PartialDate::Year(2020));
    assert!(schema().validate(Some((from, to))).is_err());
}

#[test]
fn rejects_value_of_a_different_variant_than_declared() {
    let schema = DateFieldSchema { variant: PartialDate::Year(0), ..schema() };
    let mismatched = Some(PartialDate::YearMonth(2020, 1));
    assert!(schema.validate(Some((mismatched, None))).is_err());
}

#[test]
fn accepts_value_matching_the_declared_variant() {
    let schema = DateFieldSchema { variant: PartialDate::Year(0), ..schema() };
    let matching = Some(PartialDate::Year(2020));
    assert!(schema.validate(Some((matching, None))).is_ok());
}

#[test]
fn rejects_value_outside_min_max_bounds() {
    let schema = DateFieldSchema {
        variant: PartialDate::Year(0),
        min: Some(PartialDate::Year(2000)),
        max: Some(PartialDate::Year(2020)),
        ..schema()
    };
    assert!(schema.validate(Some((Some(PartialDate::Year(1999)), None))).is_err());
    assert!(schema.validate(Some((Some(PartialDate::Year(2021)), None))).is_err());
    assert!(schema.validate(Some((Some(PartialDate::Year(2010)), None))).is_ok());
}
