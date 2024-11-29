#![cfg(feature = "fs_cache_fetcher")]

use super::Spreadsheet;
pub use fetcher::{Config, DataFetcher, Stale};

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
    type Error = FetcherError;
    async fn fetch(&self) -> Result<Spreadsheet, FetcherError> {
        dotenv::dotenv().ok();
        let key = std::env::var("GOOGLE_API_KEY").expect("No google api key");
        Ok(super::fetch_spreadsheet(&key).await?)
    }
    fn config(&self) -> &Config {
        &self.0
    }
    fn config_mut(&mut self) -> &mut Config {
        &mut self.0
    }
}

#[derive(Debug)]
pub enum FetcherError {
    Serde(serde_json::Error),
    Io(std::io::Error),
    Reqwest(reqwest::Error),
}

impl From<serde_json::Error> for FetcherError {
    fn from(value: serde_json::Error) -> Self {
        FetcherError::Serde(value)
    }
}

impl From<std::io::Error> for FetcherError {
    fn from(value: std::io::Error) -> Self {
        FetcherError::Io(value)
    }
}

impl From<reqwest::Error> for FetcherError {
    fn from(value: reqwest::Error) -> Self {
        FetcherError::Reqwest(value)
    }
}
