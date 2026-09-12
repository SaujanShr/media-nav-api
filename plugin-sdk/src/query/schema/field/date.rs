use serde::{Deserialize, Serialize};

use crate::partial_date::PartialDate;

#[derive(Serialize, Deserialize)]
pub struct DateFieldSchema {
    pub variant:       PartialDate,
    pub min:           Option<PartialDate>,
    pub max:           Option<PartialDate>,
    pub range:         bool,
    pub required_from: bool,
    pub required_to:   bool,
    pub default_from:  Option<PartialDate>,
    pub default_to:    Option<PartialDate>,
}

impl DateFieldSchema {
    
    // ── Public ────────────────────────────────────────────────────────────────

    pub fn validate(&self, value: Option<(Option<PartialDate>, Option<PartialDate>)>) -> Result<(), String> {
        let (from, to) = value.unwrap_or((None, None));

        self.validate_required(from, to)?;
        self.validate_order(from, to)?;
        for date in [from, to].into_iter().flatten() {
            self.validate_variant(date)?;
            self.validate_bounds(date)?;
        }

        Ok(())
    }

    // ── Private ───────────────────────────────────────────────────────────────

    fn validate_required(&self, from: Option<PartialDate>, to: Option<PartialDate>) -> Result<(), String> {
        if self.required_from && from.is_none() {
            return Err("From date is required".into());
        }
        if self.required_to && to.is_none() {
            return Err("To date is required".into());
        }
        Ok(())
    }

    fn validate_order(&self, from: Option<PartialDate>, to: Option<PartialDate>) -> Result<(), String> {
        if let (Some(f), Some(t)) = (from, to) {
            if f > t {
                return Err("From date must be earlier than or equal to To date".into());
            }
        }
        Ok(())
    }

    fn validate_variant(&self, date: PartialDate) -> Result<(), String> {
        use std::mem::discriminant;

        if discriminant(&self.variant) != discriminant(&date) {
            return Err(format!("Date must match variant for this field"));
        }

        Ok(())
    }

    fn validate_bounds(&self, date: PartialDate) -> Result<(), String> {
        if let Some(min) = self.min {
            if date < min {
                return Err(format!("Minimum date is {min}"));
            }
        }
        if let Some(max) = self.max {
            if date > max {
                return Err(format!("Maximum date is {max}"));
            }
        }
        Ok(())
    }
}
