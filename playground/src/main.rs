#![allow(unused)]

use std::{
    clone,
    collections::{HashMap, HashSet},
    ops::{Sub, SubAssign},
    path::{Display, PathBuf},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use fetcher::DataFetcher;

use divcord::{
    cards::{cards_by_source, cards_by_source_types, CardBySource, SourceAndCards},
    dropsource::{id::Identified, Source},
    parse::RichColumnVariant,
    spreadsheet::{load::SpreadsheetFetcher, record::Record, rich::RichColumn, Spreadsheet},
};
use error::Error;
use fetcher::Config;
use poe_data::{consts::WIKI_API_URL, league::LeagueReleaseInfo, PoeData};
use serde::{Deserialize, Serialize};
use serde_json::Value;

mod error;

fn check_deserialize() -> Result<Vec<Record>, Error> {
    let json = std::fs::read_to_string("records.json")?;
    let records = serde_json::from_str(&json)?;
    Ok(records)
}

#[tokio::main]
async fn main() {
    let spreadsheet_fetcher = SpreadsheetFetcher(Config {
        save: true,
        filename: "spreadsheet.json",
        stale: fetcher::Stale::Never,
    });

    let (poe_data, spreadsheet) = tokio::join!(PoeData::load(), spreadsheet_fetcher.load());
    let poe_data = poe_data.unwrap();
    let spreadsheet = spreadsheet.unwrap();

    let records = divcord::records(&spreadsheet, &poe_data).unwrap();

    let source_types = Source::types();

    let now = Instant::now();

    let mut i = 1;
    // for _ in 0..10 {
    let cards = cards_by_source_types(&["Act Boss".to_owned()], &records, &poe_data)
        .into_iter()
        .filter_map(|SourceAndCards { cards, .. }| {
            let cards = cards
                .into_iter()
                // .filter(|card| card.is_child())
                .collect::<Vec<_>>();
            if cards.is_empty() {
                None
            } else {
                Some(cards)
            }
        })
        .collect::<Vec<_>>();

    println!("{cards:#?}");

    std::fs::write("cards.json", &serde_json::to_string(&cards).unwrap()).unwrap();
}
