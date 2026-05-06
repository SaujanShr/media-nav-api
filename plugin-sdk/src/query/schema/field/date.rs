use chrono::NaiveDate;

pub struct DateFieldSchema {
    pub min:            Option<NaiveDate>,
    pub max:            Option<NaiveDate>,
    pub range:          bool,
    pub required_from:  bool,
    pub required_to:    bool,
    pub default_from:   Option<NaiveDate>,
    pub default_to:     Option<NaiveDate>,
}

impl DateFieldSchema {
    pub fn validate(&self, value: Option<(Option<NaiveDate>, Option<NaiveDate>)>) -> Result<(), String> {
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
            if let Some(min) = self.min {
                if date < min {
                    return Err(format!("Minimum date is {}", min.format("%d-%m-%Y")));
                }
            }
            if let Some(max) = self.max {
                if date > max {
                    return Err(format!("Maximum date is {}", max.format("%d-%m-%Y")));
                }
            }
        }

        Ok(())
    }
}
