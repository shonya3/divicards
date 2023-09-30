use std::fs;

use googlesheets::sheet::ValueRange;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{error::Error, table_record::CardDropTableRecord};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Table(pub Vec<CardDropTableRecord>);
impl Table {
    pub fn parse(rows: &[Vec<Value>]) -> Result<Table, Error> {
        let mut records: Vec<CardDropTableRecord> = Vec::new();
        for row in rows {
            let record = CardDropTableRecord::parse(row)?;
            records.push(record);
        }

        Ok(Table(records))
    }

    pub fn all_drops_from(&self) -> Vec<String> {
        self.0.iter().flat_map(|r| r.vec_drops_from()).collect()
    }

    pub fn read_file(path: Option<&str>) -> Result<Table, Error> {
        let p = path.unwrap_or("jsons/sheet.json");
        let s = fs::read_to_string(p)?;
        let sheet: ValueRange = serde_json::from_str(&s)?;
        Table::try_from(&sheet)
    }
}

impl TryFrom<&ValueRange> for Table {
    type Error = Error;

    fn try_from(sheet: &ValueRange) -> Result<Self, Self::Error> {
        Table::parse(&sheet.values[2..])
    }
}
