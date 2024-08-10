#![allow(unused)]
use divcord::{
    cards::{cards_by_source, cards_by_source_types, CardBySource, SourceAndCards},
    dropsource::{id::Identified, Source},
    parse::RichColumnVariant,
    records_iter,
    spreadsheet::{
        self,
        load::SpreadsheetFetcher,
        record::Record,
        rich::{DropsFrom, FontStyles, HexColor, RichColumn},
        Spreadsheet,
    },
    PoeData, PoeDataFetcher,
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
    // // card_element::images::download_card_images().await.unwrap();
    let spreadsheet = Spreadsheet::load().await.unwrap();
    let poe_data = PoeData::load().await.unwrap();
    let Ok(records) = divcord::records(&spreadsheet, &poe_data) else {
        eprintln!("divcord::records parse Err. Scanning all possible errors with records_iter...");
        for result in divcord::records_iter(&spreadsheet, &poe_data) {
            if let Err(err) = result {
                eprintln!("{err:?}");
            }
        }

        std::process::exit(0);
    };

    std::fs::write("records.json", serde_json::to_string(&records).unwrap()).unwrap();

    let sources_hashmap: HashMap<String, Source> = records
        .clone()
        .into_iter()
        .flat_map(|record| record.sources.into_iter().chain(record.verify_sources))
        .collect::<HashSet<Source>>()
        .into_iter()
        .map(|source| (source.slug(), source))
        .collect();

    let sources_vec = records
        .into_iter()
        .flat_map(|record| record.sources.into_iter().chain(record.verify_sources))
        .collect::<HashSet<Source>>();

    std::fs::write(
        "sources_obj.json",
        serde_json::to_string(&sources_hashmap).unwrap(),
    )
    .unwrap();
    std::fs::write(
        "sources_arr.json",
        serde_json::to_string(&sources_vec).unwrap(),
    )
    .unwrap();

    // let mut records = vec![];
    // for result in divcord::records_iter(&spreadsheet, &poe_data) {
    //     // if let Err(err) = result {
    //     //     eprintln!("{err:?}");
    //     // }

    //     match result {
    //         Ok(record) => records.push(record),
    //         Err(err) => eprintln!("{err:?}"),
    //     }
    // }
    // std::fs::write("records.json", serde_json::to_string(&records).unwrap()).unwrap();
}
