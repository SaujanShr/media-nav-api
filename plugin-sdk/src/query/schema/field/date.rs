use crate::query::partial_date::{DateGranularity, PartialDate};

pub struct DateFieldSchema {
    pub granularity:   DateGranularity,
    pub min:           Option<PartialDate>,
    pub max:           Option<PartialDate>,
    pub range:         bool,
    pub required_from: bool,
    pub required_to:   bool,
    pub default_from:  Option<PartialDate>,
    pub default_to:    Option<PartialDate>,
}

impl DateFieldSchema {
    pub fn validate(&self, value: Option<(Option<PartialDate>, Option<PartialDate>)>) -> Result<(), String> {
        let (from, to) = value.unwrap_or((None, None));

        if self.required_from && from.is_none() {
            return Err("From date is required".into());
        }
        if self.required_to && to.is_none() {
            return Err("To date is required".into());
        }
        if let (Some(f), Some(t)) = (from, to) {
            if f > t {
                return Err("From date must be earlier than or equal to To date".into());
            }
        }
        for date in [from, to].into_iter().flatten() {
            self.validate_granularity(date)?;
            self.validate_bounds(date)?;
        }

        Ok(())
    }

    fn validate_granularity(&self, date: PartialDate) -> Result<(), String> {
        match self.granularity {
            DateGranularity::Year => {
                if date.month.is_some() || date.day.is_some() {
                    return Err("Only the year may be set for this field".into());
                }
            }
            DateGranularity::Month => {
                if date.day.is_some() {
                    return Err("Only year and month may be set for this field".into());
                }
            }
            DateGranularity::Day => {}
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
