#![allow(unused)]
use divcord::{
    cards::{cards_by_source, cards_by_source_types, CardBySource, SourceAndCards},
    dropsource::{id::Identified, Source},
    parse::RichColumnVariant,
    records_iter,
    spreadsheet::{self, load::SpreadsheetFetcher, record::Record, rich::RichColumn, Spreadsheet},
    PoeData,
};
use divi::Prices;
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
    let maps_fetcher = MapsFetcher::default();
    let maps = maps_fetcher.load().await.unwrap();

    // // // card_element::images::download_card_images().await.unwrap();
    // let spreadsheet = SpreadsheetFetcher::default_with_mut_config(|config| {
    //     config.stale(Stale::Never);
    // })
    // .load()
    // .await
    // .unwrap();
    // let poe_data = PoeData::load().await.unwrap();
    // let Ok(records) = divcord::records(&spreadsheet, &poe_data) else {
    //     eprintln!("divcord::records parse Err. Scanning all possible errors with records_iter...");
    //     for result in divcord::records_iter(&spreadsheet, &poe_data) {
    //         if let Err(err) = result {
    //             eprintln!("{err:?}");
    //         }
    //     }

    //     std::process::exit(0);
    // };

    // std::fs::write("records.json", &serde_json::to_string(&records).unwrap()).unwrap();

    // let p = Prices::fetch(&poe::TradeLeague::Settlers).await.unwrap();
    // println!("{p:#?}");
}
