//! This module defines a fetcher for unique item data from the RePoE fork.
//!
//! It handles downloading, caching, and deserializing the `uniques.json` file,
//! which contains information about unique items in Path of Exile, such as their
//! name and item class. This data is crucial for enriching divination card
//! rewards with item-specific details.

use crate::unique::UniqueInfo;
use fs_cache_fetcher::{Config, DataFetcher, Stale};
use std::{collections::HashMap, fmt::Display};

const UNIQUES_URL: &str = "https://repoe-fork.github.io/uniques.json";

pub struct UniquesFetcher(Config);

impl Default for UniquesFetcher {
    fn default() -> Self {
        Self(Config {
            save: true,
            filename: "uniques.json",
            stale: Stale::After(std::time::Duration::from_secs(3600 * 24)), // Update once a day
        })
    }
}

impl DataFetcher for UniquesFetcher {
    type Item = HashMap<String, UniqueInfo>;
    type Error = Error;

    async fn fetch(&self) -> Result<Self::Item, Self::Error> {
        let uniques: HashMap<String, UniqueInfo> = reqwest::get(UNIQUES_URL).await?.json().await?;
        Ok(uniques)
    }

    fn config(&self) -> &Config {
        &self.0
    }

    fn config_mut(&mut self) -> &mut Config {
        &mut self.0
    }
}

#[derive(Debug)]
pub enum Error {
    Http(reqwest::Error),
    Json(serde_json::Error),
    Io(std::io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Http(e) => write!(f, "Failed to fetch uniques from RePoE: {e}"),
            Error::Json(e) => write!(f, "Failed to parse uniques info from RePoE: {e}"),
            Error::Io(e) => write!(f, "Could not save uniques json: {e}"),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Http(err)
    }
}
impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Json(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Io(value)
    }
}
