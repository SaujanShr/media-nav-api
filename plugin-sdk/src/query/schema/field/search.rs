pub struct SearchFieldSchema {
    pub required:   bool,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub default:    Option<String>,
}

impl SearchFieldSchema {
    pub fn validate(&self, value: Option<&str>) -> Result<(), String> {
        match value.filter(|s| !s.trim().is_empty()) {
            None => {
                if self.required {
                    Err("This field is required".into())
                } else {
                    Ok(())
                }
            }
            Some(s) => {
                if let Some(min) = self.min_length {
                    if s.len() < min {
                        return Err(format!("Minimum length is {min}"));
                    }
                }
                if let Some(max) = self.max_length {
                    if s.len() > max {
                        return Err(format!("Maximum length is {max}"));
                    }
                }
                
                Ok(())
            }
        }
    }
}
