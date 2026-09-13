use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, fmt};

#[cfg(test)]
#[path = "tests/partial_date.rs"]
mod tests;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PartialDate {
    Year(i16),
    Month(u8),
    Day(u8),
    YearMonth(i16, u8),
    MonthDay(u8, u8),
    YearMonthDay(i16, u8, u8),
}

impl PartialDate {
    pub fn year(&self) -> Option<i16> {
        match self {
            PartialDate::Year(y) => Some(*y),
            PartialDate::Month(_) => None,
            PartialDate::Day(_) => None,
            PartialDate::YearMonth(y, _) => Some(*y),
            PartialDate::MonthDay(_, _) => None,
            PartialDate::YearMonthDay(y, _, _) => Some(*y),
        }
    }

    pub fn month(&self) -> Option<u8> {
        match self {
            PartialDate::Year(_) => None,
            PartialDate::Month(m) => Some(*m),
            PartialDate::Day(_) => None,
            PartialDate::YearMonth(_, m) => Some(*m),
            PartialDate::MonthDay(m, _) => Some(*m),
            PartialDate::YearMonthDay(_, m, _) => Some(*m),
        }
    }

    pub fn day(&self) -> Option<u8> {
        match self {
            PartialDate::Year(_) => None,
            PartialDate::Month(_) => None,
            PartialDate::Day(d) => Some(*d),
            PartialDate::YearMonth(_, _) => None,
            PartialDate::MonthDay(_, d) => Some(*d),
            PartialDate::YearMonthDay(_, _, d) => Some(*d),
        }
    }

    fn to_ord_tuple(self) -> (Option<i16>, Option<u8>, Option<u8>) {
        match self {
            PartialDate::Year(y) => (Some(y), None, None),
            PartialDate::Month(m) => (None, Some(m), None),
            PartialDate::Day(d) => (None, None, Some(d)),
            PartialDate::YearMonth(y, m) => (Some(y), Some(m), None),
            PartialDate::MonthDay(m, d) => (None, Some(m), Some(d)),
            PartialDate::YearMonthDay(y, m, d) => (Some(y), Some(m), Some(d)),
        }
    }
}

impl PartialOrd for PartialDate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PartialDate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_ord_tuple().cmp(&other.to_ord_tuple())
    }
}

impl fmt::Display for PartialDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PartialDate::Year(y) => write!(f, "{:04}", y),
            PartialDate::Month(m) => write!(f, "{:02}", m),
            PartialDate::Day(d) => write!(f, "{:02}", d),
            PartialDate::YearMonth(y, m) => write!(f, "{:04}-{:02}", y, m),
            PartialDate::MonthDay(m, d) => write!(f, "{:02}-{:02}", m, d),
            PartialDate::YearMonthDay(y, m, d) => write!(f, "{:04}-{:02}-{:02}", y, m, d),
        }
    }
}
