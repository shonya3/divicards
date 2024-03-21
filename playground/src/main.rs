#![allow(unused)]
use divcord::{
    cards::{cards_by_source, cards_by_source_types, CardBySource, SourceAndCards},
    dropsource::{id::Identified, Source},
    parse::RichColumnVariant,
    spreadsheet::{load::SpreadsheetFetcher, record::Record, rich::RichColumn, Spreadsheet},
};
use error::Error;
use fetcher::Config;
use fetcher::DataFetcher;
use poe_data::{consts::WIKI_API_URL, league::LeagueReleaseInfo, PoeData};
use serde::{Deserialize, Serialize};
use serde_json::Value;
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
    let spreadsheet_fetcher = SpreadsheetFetcher(Config {
        stale: fetcher::Stale::After(Duration::from_secs(81400)),
        ..Default::default()
    });

    let (poe_data, spreadsheet) = tokio::join!(PoeData::load(), spreadsheet_fetcher.load());
    let poe_data = poe_data.unwrap();
    let spreadsheet = spreadsheet.unwrap();
    let records = divcord::records(&spreadsheet, &poe_data).unwrap();
}
