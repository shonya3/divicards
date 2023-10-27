pub mod rich;
pub mod table_record;

use std::{collections::HashMap, io::BufReader};

use async_trait::async_trait;
use googlesheets::sheet::ValueRange;
use serde::{Deserialize, Serialize};

use self::{
    rich::RichSourcesColumn,
    table_record::{DivcordTableRecord, SourcefulDivcordTableRecord},
};
use crate::{dropsource::Source, error::Error, loader::DataLoader};
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

    pub async fn load() -> Result<Self, Error> {
        DivcordTableLoader::new().load().await
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

    pub fn records(&self) -> impl Iterator<Item = Result<DivcordTableRecord, Error>> + '_ {
        self.sheet
            .values
            .iter()
            .zip(self.rich_sources_column.cells())
            .enumerate()
            .map(|(row_index, (divcord_table_row, cell))| {
                DivcordTableRecord::create(row_index, divcord_table_row, cell.drops_from())
            })
    }
}

pub struct DivcordTableLoader(reqwest::Client);
impl DivcordTableLoader {
    pub fn read_file(&self) -> DivcordTable {
        let path = std::env::current_dir()
            .unwrap()
            .join("data")
            .join("divcord_table.json");
        let file = std::fs::File::open(&path).unwrap();
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap()
    }
}
impl DivcordTableLoader {
    pub fn new() -> Self {
        Self(reqwest::Client::new())
    }

    pub async fn fetch_rich_sources_column(
        &self,
        client: &reqwest::Client,
    ) -> Result<RichSourcesColumn, Error> {
        dotenv::dotenv().ok();
        let key = std::env::var("GOOGLE_API_KEY").expect("No google api key");
        let spreadsheet_id = "1Pf2KNuGguZLyf6eu_R0E503U0QNyfMZqaRETsN5g6kU";
        let sheet = "Cards_and_Hypotheses";
        let url = format!("https://sheets.googleapis.com/v4/spreadsheets/{spreadsheet_id}?&ranges={sheet}!F3:F&includeGridData=true&key={key}");
        Ok(client.get(url).send().await?.json().await?)
    }

    pub async fn fetch_table_sheet(&self, client: &reqwest::Client) -> Result<ValueRange, Error> {
        dotenv::dotenv().ok();
        let key = std::env::var("GOOGLE_API_KEY").expect("No google api key");
        let spreadsheet_id = "1Pf2KNuGguZLyf6eu_R0E503U0QNyfMZqaRETsN5g6kU";
        let sheet = "Cards_and_Hypotheses";
        let range = format!("{sheet}!A3:Z");

        let url = format!(
            "https://sheets.googleapis.com/v4/spreadsheets/{spreadsheet_id}/values/{range}?key={key}"
        );
        let value_range: ValueRange = client.get(url).send().await?.json().await?;
        Ok(value_range)
    }
}
#[async_trait]
impl DataLoader<DivcordTable> for DivcordTableLoader {
    fn filename(&self) -> &'static str {
        "divcord_table.json"
    }

    async fn fetch(&self) -> Result<DivcordTable, Error> {
        let sheet = self.fetch_table_sheet(&self.0).await?;
        let len = sheet.values.len();

        Ok(DivcordTable {
            sheet: self.fetch_table_sheet(&self.0).await?,
            rich_sources_column: RichSourcesColumn::new(
                self.fetch_rich_sources_column(&self.0).await?.sheets,
                len,
            ),
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
        for d in &record.drops_from {
            let sources = crate::dropsource::parse_source(d, &record, poe_data).unwrap();
            for source in sources {
                map.entry(record.card.clone()).or_default().push(source);
            }
        }
    }

    Ok(map)
}
