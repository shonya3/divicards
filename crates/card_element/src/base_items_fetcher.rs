//! This module defines a fetcher for base item data from the RePoE fork.
//!
//! It handles downloading, caching, and deserializing the `base_items.json` file,
//! which contains information about base items in Path of Exile. This data is
//! used to look up item classes for generic unique rewards like "Timeless Jewel".

use fs_cache_fetcher::{Config, DataFetcher, Stale};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};

const BASE_ITEMS_URL: &str = "https://repoe-fork.github.io/base_items.json";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BaseItem {
    pub name: String,
    pub item_class: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(transparent)]
pub struct BaseItems(pub HashMap<String, BaseItem>);
pub struct BaseItemsFetcher(Config);

impl Default for BaseItemsFetcher {
    fn default() -> Self {
        Self(Config {
            save: true,
            filename: "base_items.json",
            stale: Stale::After(std::time::Duration::from_secs(3600 * 24)), // Update once a day
        })
    }
}

impl DataFetcher for BaseItemsFetcher {
    type Item = BaseItems;
    type Error = Error;

    async fn fetch(&self) -> Result<Self::Item, Self::Error> {
        let items: HashMap<String, BaseItem> = reqwest::get(BASE_ITEMS_URL).await?.json().await?;
        Ok(BaseItems(items))
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
            Error::Http(e) => write!(f, "Failed to fetch base_items.json from RePoE: {e}"),
            Error::Json(e) => write!(f, "Failed to parse base_items.json from RePoE: {e}"),
            Error::Io(e) => write!(f, "Could not save base_items.json: {e}"),
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
