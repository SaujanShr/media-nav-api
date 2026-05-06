use std::collections::HashMap;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::query::expression::{FilterExpression, SortExpression};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Query {
    #[serde(default)]
    pub search_fields:  HashMap<String, String>,
    #[serde(default)]
    pub boolean_fields: HashMap<String, bool>,
    #[serde(default)]
    pub number_fields:  HashMap<String, (Option<f32>, Option<f32>)>,
    #[serde(default)]
    pub date_fields:    HashMap<String, (Option<NaiveDate>, Option<NaiveDate>)>,
    #[serde(default)]
    pub filter_fields:  HashMap<String, FilterExpression>,
    #[serde(default)]
    pub sort_fields:    HashMap<String, SortExpression>,
}
