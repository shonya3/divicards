#![allow(unused)]
use card_element::DivinationCardElementData;
use divcord::{
    cards::{
        cards_by_source, cards_by_source_types, get_direct_cards_from_source,
        get_transitive_cards_from_source, CardBySource, SourceAndCards, Transitive,
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
use serde::{de::DeserializeOwned, Deserialize, Serialize};
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
    let records = match divcord::records_with_collect_all_errors(&spreadsheet, &poe_data) {
        Ok(records) => records,
        Err(err) => {
            println!("Errors parsing divcord");
            for e in err {
                println!("{e:?}");
            }
            return;
        }
    };

    // let rogues = Source::Predefined("All Rogue Exiles".parse::<PredefinedSource>().unwrap());
    let source_types = Source::types();

    let now = Instant::now();
    let cards = divcord::cards::cards_by_source_types(&source_types, &records, &poe_data);
    dbg!(now.elapsed().as_micros());

    jsonsave("cards.json", &cards);
}

pub fn jsonsave<S: Serialize>(path: &str, data: S) {
    let json = serde_json::to_string_pretty(&data).unwrap();
    std::fs::write(path, json).unwrap();
}

pub fn jsonread<D: DeserializeOwned>(path: &str) -> D {
    let s = std::fs::read_to_string(path).unwrap();
    serde_json::from_str::<D>(&s).unwrap()
}
