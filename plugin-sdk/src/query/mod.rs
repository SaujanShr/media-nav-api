pub mod schema;
pub mod expression;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::query::expression::{FilterExpression, SortExpression};
use crate::partial_date::PartialDate;

#[derive(Serialize, Deserialize, Default)]
pub struct Query {
    #[serde(default)]
    pub search_fields:  HashMap<String, String>,
    #[serde(default)]
    pub boolean_fields: HashMap<String, bool>,
    #[serde(default)]
    pub number_fields:  HashMap<String, (Option<f32>, Option<f32>)>,
    #[serde(default)]
    pub date_fields:    HashMap<String, (Option<PartialDate>, Option<PartialDate>)>,
    #[serde(default)]
    pub filter_fields:  HashMap<String, FilterExpression>,
    #[serde(default)]
    pub sort_fields:    HashMap<String, SortExpression>,
}
