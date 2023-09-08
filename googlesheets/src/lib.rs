pub mod sheet;

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub use sheet::{add_sheet, add_sheet_with_values, write_values_into_sheet};

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
