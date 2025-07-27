#![allow(unused)]
use card_element::DivinationCardElementData;
use divcord::{
    cards::{cards_by_source, cards_by_source_types, CardBySource, SourceAndCards},
    dropsource::{id::Identified, Source},
    parse::SourcesKind,
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
use poe::TradeLeague;
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
async fn main() {
    let data = DivinationCardElementData::load().await.unwrap();

    jsonsave("cardElement.json", data);
}

pub fn jsonsave<S: Serialize>(path: &str, data: S) {
    let json = serde_json::to_string(&data).unwrap();
    std::fs::write(path, json).unwrap();
}
