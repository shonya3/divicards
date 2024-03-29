#![allow(unused)]
use divcord::{
    cards::{cards_by_source, cards_by_source_types, CardBySource, SourceAndCards},
    dropsource::{id::Identified, Source},
    parse::RichColumnVariant,
    spreadsheet::{load::SpreadsheetFetcher, record::Record, rich::RichColumn, Spreadsheet},
};
use error::Error;
use fetcher::DataFetcher;
use fetcher::{Config, Stale};
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
    let mut spreadsheet_fetcher = SpreadsheetFetcher::default();
    spreadsheet_fetcher.0.stale = Stale::After(Duration::from_secs(81400));

    let (poe_data, spreadsheet) = tokio::join!(PoeData::load(), spreadsheet_fetcher.load());
    let poe_data = poe_data.unwrap();
    let spreadsheet = spreadsheet.unwrap();
    let records = divcord::records(&spreadsheet, &poe_data).unwrap();

    let dried_lake = Source::Act("The Dried Lake".to_owned());
    let cards = divcord::cards::cards_by_source_types(&["Act".to_owned()], &records, &poe_data)
        .into_iter()
        .flat_map(|card| {
            let cards = card
                .cards
                .into_iter()
                .filter(|card| card.is_child())
                .collect::<Vec<_>>();

            match cards.is_empty() {
                true => None,
                false => Some(cards),
            }
        })
        .collect::<Vec<_>>();
    println!("{cards:#?}");

    println!("{}", std::env::var("CARGO_MANIFEST_DIR").unwrap());
}
