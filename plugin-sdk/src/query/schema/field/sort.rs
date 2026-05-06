use crate::query::expression::{SortDirection, SortExpression, SortOption};

pub struct SortFieldSchema {
    pub supported:         Vec<SortOption>,
    pub required:          bool,
    pub default_sort:      String,
    pub default_direction: SortDirection,
}

impl SortFieldSchema {
    pub fn validate(&self, value: Option<&SortExpression>) -> Result<(), String> {
        match value {
            None => {
                if self.required {
                    Err("This field is required".into())
                } else {
                    Ok(())
                }
            }
            Some(expr) => self.validate_sort(expr),
        }
    }

    fn validate_sort(&self, expr: &SortExpression) -> Result<(), String> {
        let option = self.supported.iter()
            .find(|o| o.sort == expr.sort)
            .ok_or_else(|| format!("Sort '{}' is not supported", expr.sort))?;

        match expr.direction {
            SortDirection::Asc if !option.ascending =>
                Err(format!("Ascending sort is not allowed for '{}'", expr.sort)),
            SortDirection::Desc if !option.descending =>
                Err(format!("Descending sort is not allowed for '{}'", expr.sort)),
            _ => Ok(()),
        }
    }
}
