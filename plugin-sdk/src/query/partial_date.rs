use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, fmt};

#[derive(Clone, Copy, Debug)]
pub enum DateGranularity {
    Year,
    Month,
    Day,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct PartialDate {
    pub year:  i32,
    pub month: Option<u32>,
    pub day:   Option<u32>,
}

impl PartialDate {
    fn to_ord_tuple(self) -> (i32, u32, u32) {
        (
            self.year,
            self.month.unwrap_or(1),
            self.day.unwrap_or(1)
        )
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
        match (self.month, self.day) {
            (Some(m), Some(d)) =>
                write!(f, "{:04}-{:02}-{:02}", self.year, m, d),
            (Some(m), None) =>
                write!(f, "{:04}-{:02}", self.year, m),
            _ =>
                write!(f, "{:04}", self.year),
        }
    }
}
