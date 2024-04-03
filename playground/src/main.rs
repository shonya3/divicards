#![allow(unused)]
use divcord::{
    cards::{cards_by_source, cards_by_source_types, CardBySource, SourceAndCards},
    dropsource::{id::Identified, Source},
    parse::RichColumnVariant,
    records_iter,
    spreadsheet::{self, load::SpreadsheetFetcher, record::Record, rich::RichColumn, Spreadsheet},
    PoeData,
};
use error::Error;
use fetcher::DataFetcher;
use fetcher::{Config, Stale};
use poe_data::fetchers::MapsFetcher;
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
    let maps = MapsFetcher::default().load().await.unwrap();

    let spreadsheet = Spreadsheet::load().await.unwrap();
    let poe_data = PoeData::load().await.unwrap();
    let Ok(records) = divcord::records(&spreadsheet, &poe_data) else {
        eprintln!("divcord::records parse Err. Scanning all possible errors with records_iter...\n\n\n\n\n");
        for result in divcord::records_iter(&spreadsheet, &poe_data) {
            if let Err(err) = result {
                eprintln!("{err:#?}\n\n\n\n");
            }
        }

        std::process::exit(0);
    };
}
