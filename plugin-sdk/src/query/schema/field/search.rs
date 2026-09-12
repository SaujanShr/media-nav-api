use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SearchFieldSchema {
    pub required:   bool,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub default:    Option<String>,
}

impl SearchFieldSchema {

    // ── Public ────────────────────────────────────────────────────────────────

    pub fn validate(&self, value: Option<&str>) -> Result<(), String> {
        let value = value.filter(|s| !s.trim().is_empty());

        self.validate_required(value)?;
        if let Some(s) = value {
            self.validate_length(s)?;
        }

        Ok(())
    }

    // ── Private ───────────────────────────────────────────────────────────────

    fn validate_required(&self, value: Option<&str>) -> Result<(), String> {
        if self.required && value.is_none() {
            return Err("This field is required".into());
        }
        Ok(())
    }

    fn  validate_length(&self, value: &str) -> Result<(), String> {
        if let Some(min) = self.min_length {
            if value.len() < min {
                return Err(format!("Minimum length is {min}"));
            }
        }
        if let Some(max) = self.max_length {
            if value.len() > max {
                return Err(format!("Maximum length is {max}"));
            }
        }
        Ok(())
    }
}
