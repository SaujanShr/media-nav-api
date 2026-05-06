use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug)]
pub enum DateGranularity {
    /// Year only — month and day must be absent.
    Year,
    /// Year + month — day must be absent.
    Month,
    /// Full date — year, month, and day may all be set.
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
        (self.year, self.month.unwrap_or(1), self.day.unwrap_or(1))
    }
}

impl PartialOrd for PartialDate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PartialDate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_ord_tuple().cmp(&other.to_ord_tuple())
    }
}

impl std::fmt::Display for PartialDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.month, self.day) {
            (Some(m), Some(d)) => write!(f, "{:04}-{:02}-{:02}", self.year, m, d),
            (Some(m), None)    => write!(f, "{:04}-{:02}", self.year, m),
            _                  => write!(f, "{:04}", self.year),
        }
    }
}
