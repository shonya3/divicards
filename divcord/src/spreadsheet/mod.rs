#[cfg(feature = "fetch")]
pub mod load;
pub mod record;
pub mod rich;

use self::{record::Dumb, rich::RichColumn};
use crate::{dropsource::Source, error::Error};
use googlesheets::sheet::ValueRange;
use poe_data::PoeData;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Spreadsheet {
    pub sheet: ValueRange,
    pub rich_sources_column: RichColumn,
    pub rich_verify_column: RichColumn,
}

impl Spreadsheet {
    pub const fn new(
        sheet: ValueRange,
        rich_sources_column: RichColumn,
        rich_verify_column: RichColumn,
    ) -> Self {
        Self {
            sheet,
            rich_sources_column,
            rich_verify_column,
        }
    }

    #[cfg(feature = "fetch")]
    pub async fn load() -> Result<Self, Error> {
        use loader::DataLoader;

        load::SpreadsheetLoader::new().load().await
    }

    pub fn records_by_card(&self) -> Result<HashMap<String, Vec<Dumb>>, Error> {
        let mut map: HashMap<String, Vec<Dumb>> = HashMap::new();
        for record in self.dumb_records() {
            let record = record?;
            map.entry(record.card.clone())
                .and_modify(|vec| vec.push(record.clone()))
                .or_insert(vec![record]);
        }

        Ok(map)
    }

    pub fn dumb_records(&self) -> impl Iterator<Item = Result<Dumb, Error>> + '_ {
        self.sheet
            .values
            .iter()
            .zip(self.rich_sources_column.cells())
            .zip(self.rich_verify_column.cells())
            .enumerate()
            .map(
                |(row_index, ((spreadsheet_row, sources_cell), verify_cell))| {
                    Dumb::create(
                        row_index,
                        spreadsheet_row,
                        sources_cell.drops_from()?,
                        verify_cell.drops_from()?,
                    )
                },
            )
    }
}

pub fn sources_by_card(
    spreadsheet: &Spreadsheet,
    poe_data: &PoeData,
) -> Result<HashMap<String, Vec<Source>>, Error> {
    let mut map: HashMap<String, Vec<Source>> = HashMap::new();
    for record in spreadsheet.dumb_records() {
        let record = record?;
        for _d in &record.sources_drops_from {
            let sources = crate::dropsource::parse::parse_dropses_from(
                &record,
                &poe_data,
                crate::dropsource::parse::RichColumnVariant::Sources,
            )
            .unwrap();
            for source in sources {
                map.entry(record.card.clone()).or_default().push(source);
            }
        }
    }

    Ok(map)
}
