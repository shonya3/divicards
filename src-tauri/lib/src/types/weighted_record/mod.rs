use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WeightedRecord {
    pub stack_size: i32,
    pub name: String,
    pub calculated: Option<f32>,
    pub total: Option<f32>,
    pub real_weight: f32,
}

impl Default for WeightedRecord {
    fn default() -> Self {
        Self {
            real_weight: 0.,
            stack_size: 0,
            name: String::default(),
            calculated: Some(0.),
            total: Some(0.),
        }
    }
}

pub fn write(records: &Vec<WeightedRecord>) -> Result<String, Error> {
    let mut writer = csv::Writer::from_writer(vec![]);
    for record in records {
        writer.serialize(record)?;
    }
    let content_string = String::from_utf8(writer.into_inner().expect("Error with csv serialize"))?;
    Ok(content_string)
}
