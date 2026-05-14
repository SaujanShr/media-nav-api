use crate::query::partial_date::PartialDate;

/// Numeric range field. Bounds are `[from, to]`; either may be `None`.
pub struct NumberFieldSchema {
    pub min:            Option<f32>,
    pub max:            Option<f32>,
    pub float:          bool,
    pub range:          bool,
    pub required_from:  bool,
    pub required_to:    bool,
    pub default_from:   Option<f32>,
    pub default_to:     Option<f32>,
}

impl NumberFieldSchema {
    pub fn validate(&self, value: Option<(Option<f32>, Option<f32>)>) -> Result<(), String> {
        let (from, to) = value.unwrap_or((None, None));

        self.validate_required(from, to)?;
        self.validate_order(from, to)?;
        for n in [from, to].into_iter().flatten() {
            self.validate_number(n)?;
        }
        
        Ok(())
    }

    fn validate_required(&self, from: Option<f32>, to: Option<f32>) -> Result<(), String> {
        if self.required_from && from.is_none() {
            return Err("From value is required".into());
        }
        if self.required_to && to.is_none() {
            return Err("To value is required".into());
        }
        Ok(())
    }

    fn validate_order(&self, from: Option<f32>, to: Option<f32>) -> Result<(), String> {
        if let (Some(f), Some(t)) = (from, to) {
            if f > t {
                return Err("From value must be less than or equal to To value".into());
            }
        }
        Ok(())
    }

    fn validate_number(&self, n: f32) -> Result<(), String> {
        if !self.float && n % 1.0 != 0.0 {
            return Err("Only integers allowed".into());
        }
        if let Some(min) = self.min {
            if n < min {
                return Err(format!("Minimum value is {min}"));
            }
        }
        if let Some(max) = self.max {
            if n > max {
                return Err(format!("Maximum value is {max}"));
            }
        }
        Ok(())
    }
}
