use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Values(pub Vec<Vec<Value>>);
impl Values {
    pub const fn new(v: Vec<Vec<Value>>) -> Self {
        Values(v)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct A1Range(String);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Dimension {
    Rows,
    Columns,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RangeRequestBody {
    pub range: A1Range,
    pub major_dimension: Dimension,
    pub values: Values,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Column {
    Name,
    Amount,
    Weight,
    Price,
}

impl Display for Column {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Column::Name => write!(f, "name"),
            Column::Amount => write!(f, "amount"),
            Column::Weight => write!(f, "weight"),
            Column::Price => write!(f, "price"),
        }
    }
}

pub fn create_sheet(spreadsheet_id: String, sheet_name: String, values: Values) {}
