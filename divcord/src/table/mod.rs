#[cfg(feature = "fetch")]
pub mod load;
pub mod rich;
pub mod table_record;

use std::collections::HashMap;

use googlesheets::sheet::ValueRange;
use serde::{Deserialize, Serialize};

use self::{
    rich::RichSourcesColumn,
    table_record::{DivcordTableRecord, SourcefulDivcordTableRecord},
};
use crate::{dropsource::Source, error::Error};
use poe_data::PoeData;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DivcordTable {
    pub sheet: ValueRange,
    pub rich_sources_column: RichSourcesColumn,
}

impl DivcordTable {
    pub const fn new(sheet: ValueRange, rich_sources_column: RichSourcesColumn) -> Self {
        Self {
            sheet,
            rich_sources_column,
        }
    }

    #[cfg(feature = "fetch")]
    pub async fn load() -> Result<Self, Error> {
        use loader::DataLoader;

        load::DivcordTableLoader::new().load().await
    }

    pub fn records_by_card(&self) -> Result<HashMap<String, Vec<DivcordTableRecord>>, Error> {
        let mut map: HashMap<String, Vec<DivcordTableRecord>> = HashMap::new();
        for record in self.records() {
            let record = record?;
            map.entry(record.card.clone())
                .and_modify(|vec| vec.push(record.clone()))
                .or_insert(vec![record]);
        }

        Ok(map)
    }

    pub fn sourceful_records(
        &self,
        poe_data: &PoeData,
    ) -> Result<Vec<SourcefulDivcordTableRecord>, Error> {
        self.records()
            .map(|r| Ok(SourcefulDivcordTableRecord::from_record(r?, poe_data)?))
            .collect()
    }

    pub fn sourceful_records_iter(
        &self,
        poe_data: PoeData,
    ) -> impl Iterator<Item = Result<SourcefulDivcordTableRecord, Error>> + '_ {
        self.records().map(move |record| {
            Ok(SourcefulDivcordTableRecord::from_record(
                record?, &poe_data,
            )?)
        })
    }

    pub fn records(&self) -> impl Iterator<Item = Result<DivcordTableRecord, Error>> + '_ {
        self.sheet
            .values
            .iter()
            .zip(self.rich_sources_column.cells())
            .enumerate()
            .map(|(row_index, (divcord_table_row, cell))| {
                DivcordTableRecord::create(row_index, divcord_table_row, cell.drops_from()?)
            })
    }
}

pub fn sources_by_card(
    divcord_table: &DivcordTable,
    poe_data: &PoeData,
) -> Result<HashMap<String, Vec<Source>>, Error> {
    let mut map: HashMap<String, Vec<Source>> = HashMap::new();
    for record in divcord_table.records() {
        let record = record?;
        for _d in &record.drops_from {
            let sources = crate::dropsource::parse::parse_dropses_from(&record, &poe_data).unwrap();
            for source in sources {
                map.entry(record.card.clone()).or_default().push(source);
            }
        }
    }

    Ok(map)
}
