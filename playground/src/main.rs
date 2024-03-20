#![allow(unused)]

use std::{
    clone,
    collections::HashMap,
    ops::{Sub, SubAssign},
    path::{Display, PathBuf},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use fetcher::DataFetcher;

use divcord::{
    cardsnew::{cards_by_source, cards_by_source_types, CardBySource},
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

    /* bench
    // let now = Instant::now();
    // println!("{c:#?}");
    // println!("{}", c.len());
    // let core_map = Source::Map(String::from("Core Map"));
    // for _ in 1..10 {
    // cards_by_source(&core_map, &records, &poe_data);
    // }
    // println!("{}", now.elapsed().as_micros());
     */

    // let core_map = Source::Map(String::from("Core Map"));
    // let the_harvest = Source::Act("1_4_6_3".to_owned());
    // let mao_kun = Source::Map("Mao Kun".to_owned());

    // let cards: Vec<CardBySource> = cards_by_source(&mao_kun, &records, &poe_data)
    //     .into_iter()
    //     .filter(|s| s.column == RichColumnVariant::Verify)
    //     .collect();
    // println!("{cards:#?}");
    // println!("{}", cards.len());

    let source_types = Source::types();

    let now = Instant::now();

    // for _ in 0..1 {
    cards_by_source_types(&source_types, &records, &poe_data);

    // }

    println!("{}", now.elapsed().as_millis());

    // let map = cards_by_source_types(&source_types, &records, &poe_data)
    //     .into_iter()
    //     .map(|(key, vec)| (key.id().to_owned(), vec))
    //     .collect::<HashMap<String, Vec<CardBySource>>>();

    // let json = serde_json::to_string(&map).unwrap();

    // std::fs::write("mapjson.json", &json).unwrap();

    // let map: HashMap<Source, CardBySource> = serde_json::from_str(&json).unwrap();
    //
    // println!("{}", map.keys().len());

    // let cards = map.get(&Source::Map("Wasteland Map".to_owned()));
    // println!("{cards:#?}");
}

// #[tokio::main]
// async fn main() {
//     let spreadsheet = Spreadsheet::load().await.unwrap();
//     let poe_data = PoeData::load().await.unwrap();
//     let records = divcord::records(&spreadsheet, &poe_data).unwrap();
//     let json = serde_json::to_string_pretty(&records).unwrap();
//     std::fs::write("records.json", &json).unwrap();

//     let records = check_deserialize().unwrap();

//     let empty: Vec<Record> = records
//         .clone()
//         .into_iter()
//         .filter(|r| r.sources.is_empty())
//         .collect();
//     let json = serde_json::to_string_pretty(&empty).unwrap();
//     std::fs::write("empty.json", &json).unwrap();

//     // Source::write_typescript_file().unwrap();

//     for (index, record) in divcord::records_iter(&spreadsheet, &poe_data).enumerate() {
//         // if index == 63 {
//         //     println!("{record:#?}");
//         // }

//         // if let Ok(record) = record {
//         //     if record.id == 66 {
//         //         println!("here");
//         //         println!("{record:#?}");
//         //     }
//         // }

//         let record = record.unwrap();

//         for verify in record.verify_sources {
//             // println!("{verify:?}");
//         }
//     }
// }

fn h_column() -> RichColumn {
    serde_json::from_str(&std::fs::read_to_string("H.json").unwrap()).unwrap()
}

pub async fn inspect_h_column(spreadsheet: Spreadsheet) {
    fetch_h().await;
    let mut erroneus_cell_indexes = vec![];
    for (index, cell) in h_column().cells().enumerate() {
        let vec = cell.drops_from();
        if let Err(err) = vec {
            let index = index + 3;
            eprintln!("{} {err}", index);
            println!("Cell :{cell:#?}");
            println!("==========================================");
            erroneus_cell_indexes.push(index);
        }
    }
    println!("Erroneus cells: {erroneus_cell_indexes:?}");

    for record in spreadsheet.dumb_records() {
        let record = record.unwrap();

        if record.id == 66 {
            println!("{record:#?}");
        }
    }
}

pub async fn fetch_h() -> RichColumn {
    dotenv::dotenv().ok();
    let key = std::env::var("GOOGLE_API_KEY").expect("No google api key");
    let spreadsheet_id = "1Pf2KNuGguZLyf6eu_R0E503U0QNyfMZqaRETsN5g6kU";
    let sheet = "Cards_and_Hypotheses";
    let url = format!("https://sheets.googleapis.com/v4/spreadsheets/{spreadsheet_id}?&ranges={sheet}!H3:H&includeGridData=true&key={key}");
    let h = reqwest::Client::new()
        .get(url)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    std::fs::write("H.json", &serde_json::to_string(&h).unwrap()).unwrap();
    h
}

pub async fn fetch_f_h() -> RichColumn {
    dotenv::dotenv().ok();
    let key = std::env::var("GOOGLE_API_KEY").expect("No google api key");
    let spreadsheet_id = "1Pf2KNuGguZLyf6eu_R0E503U0QNyfMZqaRETsN5g6kU";
    let sheet = "Cards_and_Hypotheses";
    let url = format!("https://sheets.googleapis.com/v4/spreadsheets/{spreadsheet_id}?&ranges={sheet}!F3:F&ranges={sheet}!H3:H&includeGridData=true&key={key}");
    let h = reqwest::Client::new()
        .get(url)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    std::fs::write("H.json", &serde_json::to_string(&h).unwrap()).unwrap();
    h
}

// #[tokio::main]
// async fn main() {
//     let spreadsheet = DivcordTable::load().await.unwrap();
//     let poe_data = PoeData::load().await.unwrap();
//     let records = spreadsheet.sourceful_records(&poe_data).unwrap();
//     let json = serde_json::to_string_pretty(&records).unwrap();
//     std::fs::write("records.json", &json).unwrap();

//     let records = check_deserialize().unwrap();

//     let empty: Vec<SourcefulRecord> = records
//         .clone()
//         .into_iter()
//         .filter(|r| r.sources.is_empty())
//         .collect();
//     let json = serde_json::to_string_pretty(&empty).unwrap();
//     std::fs::write("empty.json", &json).unwrap();

//     // Source::write_typescript_file().unwrap();
// }
