#![cfg(feature = "fetch")]

use async_trait::async_trait;
use googlesheets::sheet::ValueRange;
use loader::DataLoader;

use crate::error::Error;

use super::{rich::RichSourcesColumn, DivcordTable};

pub struct DivcordTableLoader(reqwest::Client);
impl DivcordTableLoader {
    pub fn new() -> Self {
        Self(reqwest::Client::new())
    }

    pub async fn fetch_rich_sources_column(&self) -> Result<RichSourcesColumn, Error> {
        dotenv::dotenv().ok();
        let key = std::env::var("GOOGLE_API_KEY").expect("No google api key");
        let spreadsheet_id = "1Pf2KNuGguZLyf6eu_R0E503U0QNyfMZqaRETsN5g6kU";
        let sheet = "Cards_and_Hypotheses";
        let url = format!("https://sheets.googleapis.com/v4/spreadsheets/{spreadsheet_id}?&ranges={sheet}!F3:F&includeGridData=true&key={key}");
        Ok(self.0.get(url).send().await?.json().await?)
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

    pub async fn _fetch(&self) -> Result<DivcordTable, Error> {
        let sheet = self.fetch_table_sheet(&self.0).await?;
        let rich_sources_column = RichSourcesColumn::new(
            self.fetch_rich_sources_column().await?.sheets,
            sheet.values.len(),
        );

        Ok(DivcordTable {
            sheet,
            rich_sources_column,
        })
    }
}

#[async_trait]
impl DataLoader<DivcordTable, Error> for DivcordTableLoader {
    fn filename(&self) -> &'static str {
        "divcord_table.json"
    }

    async fn fetch(&self) -> Result<DivcordTable, Error> {
        self._fetch().await
    }
}
