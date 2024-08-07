#![cfg(feature = "fetch")]

pub use fetcher::{Config, DataFetcher, Stale};
use googlesheets::sheet::ValueRange;

use crate::{error::Error, parse::RichColumnVariant};

use super::{rich::RichColumn, Spreadsheet};

pub struct SpreadsheetFetcher(pub Config);

impl Default for SpreadsheetFetcher {
    fn default() -> Self {
        Self(Config {
            save: true,
            filename: "spreadsheet.json",
            stale: Stale::ReloadEveryTime,
        })
    }
}
impl DataFetcher for SpreadsheetFetcher {
    type Item = Spreadsheet;
    type Error = Error;
    async fn fetch(&self) -> Result<Spreadsheet, Error> {
        self._fetch().await
    }
    fn config(&self) -> &Config {
        &self.0
    }
    fn config_mut(&mut self) -> &mut Config {
        &mut self.0
    }
}

pub async fn fetch_rich_column(letter: char) -> Result<RichColumn, reqwest::Error> {
    dotenv::dotenv().ok();
    let key = std::env::var("GOOGLE_API_KEY").expect("No google api key");
    let spreadsheet_id = "1Pf2KNuGguZLyf6eu_R0E503U0QNyfMZqaRETsN5g6kU";
    let sheet = "Cards_and_Hypotheses";
    let column = letter;
    let url = format!("https://sheets.googleapis.com/v4/spreadsheets/{spreadsheet_id}?&ranges={sheet}!{column}3:{column}&includeGridData=true&key={key}");
    reqwest::get(url).await?.json().await
}

impl SpreadsheetFetcher {
    pub async fn fetch_rich_column(&self, variant: RichColumnVariant) -> Result<RichColumn, Error> {
        Ok(fetch_rich_column(variant.column_letter()).await?)
    }

    pub async fn fetch_table_sheet(&self) -> Result<ValueRange, Error> {
        dotenv::dotenv().ok();
        let key = std::env::var("GOOGLE_API_KEY").expect("No google api key");
        let spreadsheet_id = "1Pf2KNuGguZLyf6eu_R0E503U0QNyfMZqaRETsN5g6kU";
        let sheet = "Cards_and_Hypotheses";
        let range = format!("{sheet}!A3:Z");

        let url = format!(
            "https://sheets.googleapis.com/v4/spreadsheets/{spreadsheet_id}/values/{range}?key={key}"
        );
        let value_range: ValueRange = reqwest::get(url).await?.json().await?;
        Ok(value_range)
    }

    pub async fn _fetch(&self) -> Result<Spreadsheet, Error> {
        let sheet = self.fetch_table_sheet().await?;
        let number_of_rows = sheet.values.len();
        let rich_confirmations_new_325 = RichColumn::new(
            self.fetch_rich_column(RichColumnVariant::Sources)
                .await?
                .sheets,
            number_of_rows,
        );
        let rich_to_confirm_or_reverify = RichColumn::new(
            self.fetch_rich_column(RichColumnVariant::Verify)
                .await?
                .sheets,
            number_of_rows,
        );

        Ok(Spreadsheet {
            sheet,
            rich_confirmations_new_325,
            rich_to_confirm_or_reverify,
        })
    }
}
