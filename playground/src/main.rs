#![allow(unused)]
use card_element::DivinationCardElementData;
use divcord::{
    cards::{
        cards_by_source, cards_by_source_types, get_direct_cards_from_source,
        get_transitive_cards_from_source, CardBySource, SourceAndCards,
    },
    dropsource::{id::Identified, predefined::PredefinedSource, Source},
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
    let poe_data = PoeData::load().await.unwrap();

    let spreadsheet = SpreadsheetFetcher::default_with_mut_config(|c| {
        c.stale = Stale::After(Duration::from_secs(84400))
    })
    .load()
    .await
    .unwrap();
    let records = divcord::records_with_collect_all_errors(&spreadsheet, &poe_data).unwrap();

    // let now = Instant::now();

    let rogues = Source::Predefined("All Rogue Exiles".parse::<PredefinedSource>().unwrap());
    // let source_types = ["All Rogue Exiles".to_string()];
    // let sources_and_cards =
    //     divcord::cards::cards_by_source_types(&source_types, &records, &poe_data);

    // println!("{}", now.elapsed().as_millis());
    // jsonsave("sourcesAndCards.json", &sources_and_cards);

    let cards = cards_by_source(&rogues, &records, &poe_data);
    println!("{cards:#?}");
}

pub fn jsonsave<S: Serialize>(path: &str, data: S) {
    let json = serde_json::to_string_pretty(&data).unwrap();
    std::fs::write(path, json).unwrap();
}
