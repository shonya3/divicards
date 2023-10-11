use std::{fs, path::PathBuf};

use googlesheets::sheet::ValueRange;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{error::Error, rich::RichSourcesColumn, table_record::CardDropTableRecord};

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
}

pub struct DivcordTableRepository(reqwest::Client);
impl DivcordTableRepository {
    pub fn new() -> Self {
        Self(reqwest::Client::new())
    }

    fn file_path(&self, jsons_dir: &PathBuf) -> PathBuf {
        jsons_dir.join("divcord_table.json")
    }

    fn jsons_dir(&self) -> PathBuf {
        let dir = std::env::current_dir().unwrap().join("jsons");
        if !dir.exists() {
            std::fs::create_dir(&dir).unwrap();
        }

        dir
    }

    fn exists_and_up_to_date(&self) -> bool {
        match self.file_days_old() {
            Some(n) if n <= 1.0 => true,
            _ => false,
        }
    }

    fn file_days_old(&self) -> Option<f32> {
        pub const DAY_AS_SECS: f64 = 86_400.0;
        let path = self.file_path(&self.jsons_dir());
        let exists = path.try_exists().unwrap();
        match exists {
            true => match fs::metadata(&path) {
                Ok(metadata) => match metadata.modified() {
                    Ok(time) => {
                        let days = (time.elapsed().unwrap().as_secs() as f64 / DAY_AS_SECS) as f32;
                        Some(days)
                    }
                    Err(_) => None,
                },
                Err(_) => None,
            },
            false => None,
        }
    }

    fn read_from_file(&self, file_path: &PathBuf) -> Result<DivcordTable, Error> {
        let s = std::fs::read_to_string(&file_path)?;
        Ok(serde_json::from_str(&s)?)
    }

    pub async fn fetch(&self) -> Result<DivcordTable, Error> {
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
    pub async fn load(&self) -> Result<DivcordTable, Error> {
        match self.exists_and_up_to_date() {
            true => self.read_from_file(&self.file_path(&self.jsons_dir())),
            false => {
                let t = self.fetch().await?;
                self.save(&t)?;
                Ok(t)
            }
        }
    }

    pub async fn update(&self) -> Result<(), Error> {
        let t = self.fetch().await?;
        self.save(&t)?;
        Ok(())
    }

    pub fn save(&self, table: &DivcordTable) -> Result<(), Error> {
        let json = serde_json::to_string(table)?;
        fs::write(self.file_path(&self.jsons_dir()), &json)?;

        Ok(())
    }

    pub async fn fetch_rich_sources_column(
        &self,
        client: &reqwest::Client,
    ) -> Result<RichSourcesColumn, Error> {
        dotenv::dotenv().ok();
        let key = std::env::var("GOOGLE_API_KEY").expect("No google api key");
        let spreadsheet_id = "1Pf2KNuGguZLyf6eu_R0E503U0QNyfMZqaRETsN5g6kU";
        let sheet = "Cards_and_Hypotheses";
        // let url = format!("https://sheets.googleapis.com/v4/spreadsheets/{spreadsheet_id}?&ranges={sheet}!B3:B&ranges={sheet}!F3:F&includeGridData=true&key={key}");
        let url = format!("https://sheets.googleapis.com/v4/spreadsheets/{spreadsheet_id}?&ranges={sheet}!F3:F&includeGridData=true&key={key}");
        // let url = format!("https://sheets.googleapis.com/v4/spreadsheets/{spreadsheet_id}?&ranges={sheet}!A:A&ranges={sheet}!F:F&includeGridData=true&key={key}");
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
