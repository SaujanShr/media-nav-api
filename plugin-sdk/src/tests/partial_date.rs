use super::{Ordering, PartialDate};

#[test]
fn accessors_return_none_for_missing_components() {
    assert_eq!(PartialDate::Year(2024).year(), Some(2024));
    assert_eq!(PartialDate::Year(2024).month(), None);
    assert_eq!(PartialDate::Year(2024).day(), None);

    assert_eq!(PartialDate::Month(5).month(), Some(5));
    assert_eq!(PartialDate::Month(5).year(), None);

    assert_eq!(PartialDate::Day(15).day(), Some(15));

    assert_eq!(PartialDate::YearMonth(2024, 5).year(), Some(2024));
    assert_eq!(PartialDate::YearMonth(2024, 5).month(), Some(5));
    assert_eq!(PartialDate::YearMonth(2024, 5).day(), None);

    assert_eq!(PartialDate::MonthDay(5, 15).month(), Some(5));
    assert_eq!(PartialDate::MonthDay(5, 15).day(), Some(15));
    assert_eq!(PartialDate::MonthDay(5, 15).year(), None);

    assert_eq!(PartialDate::YearMonthDay(2024, 5, 15).year(), Some(2024));
    assert_eq!(PartialDate::YearMonthDay(2024, 5, 15).month(), Some(5));
    assert_eq!(PartialDate::YearMonthDay(2024, 5, 15).day(), Some(15));
}

#[test]
fn display_formats_each_variant() {
    assert_eq!(PartialDate::Year(2024).to_string(), "2024");
    assert_eq!(PartialDate::Month(5).to_string(), "05");
    assert_eq!(PartialDate::Day(9).to_string(), "09");
    assert_eq!(PartialDate::YearMonth(2024, 5).to_string(), "2024-05");
    assert_eq!(PartialDate::MonthDay(5, 9).to_string(), "05-09");
    assert_eq!(PartialDate::YearMonthDay(2024, 5, 9).to_string(), "2024-05-09");
}

#[test]
fn ordering_within_the_same_variant() {
    assert!(PartialDate::Year(2020) < PartialDate::Year(2021));
    assert!(PartialDate::YearMonthDay(2024, 1, 1) < PartialDate::YearMonthDay(2024, 1, 2));
    assert!(PartialDate::YearMonthDay(2024, 1, 31) < PartialDate::YearMonthDay(2024, 2, 1));
}

#[test]
fn distinct_variants_with_the_same_numeric_value_are_neither_equal_nor_tied() {
    let year_zero = PartialDate::Year(0);
    let month_one = PartialDate::Month(1);

    assert_ne!(year_zero, month_one);
    assert_ne!(year_zero.cmp(&month_one), Ordering::Equal);
}
