#![allow(unused)]
use card_element::DivinationCardElementData;
use divcord::{
    dropsource::{id::Identified, Source},
    records_iter,
    spreadsheet::{
        self,
        fs_cache_fetcher::SpreadsheetFetcher,
        record::Record,
        rich::{DropsFrom, FontStyles, HexColor, RichColumn},
        Spreadsheet,
    },
    PoeData, PoeDataFetcher,
};
use divi::Prices;
use error::Error;
use fs_cache_fetcher::DataFetcher;
use fs_cache_fetcher::{Config, Stale};
use poe_data::fetchers::MapsFetcher;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    clone,
    collections::{HashMap, HashSet},
    ops::{Sub, SubAssign},
    path::{Display, PathBuf},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

mod error;

#[tokio::main]
async fn main() {}
