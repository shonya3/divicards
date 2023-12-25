#![allow(unused)]

use std::{clone, path::Display};

use divcord::{
    dropsource::Source,
    table::{table_record::SourcefulDivcordTableRecord, DivcordTable},
};
use error::Error;
use poe_data::{consts::WIKI_API_URL, league::LeagueReleaseInfo, PoeData};
use serde::{Deserialize, Serialize};
use serde_json::Value;

mod error;

fn check_deserialize() -> Result<Vec<SourcefulDivcordTableRecord>, Error> {
    let json = std::fs::read_to_string("records.json")?;
    let records = serde_json::from_str(&json)?;
    Ok(records)
}

#[tokio::main]
async fn main() {
    let divcord_table = DivcordTable::load().await.unwrap();
    let poe_data = PoeData::load().await.unwrap();
    let records = divcord_table.sourceful_records(&poe_data).unwrap();
    let json = serde_json::to_string_pretty(&records).unwrap();
    std::fs::write("records.json", &json).unwrap();

    let records = check_deserialize().unwrap();

    let empty: Vec<SourcefulDivcordTableRecord> = records
        .clone()
        .into_iter()
        .filter(|r| r.sources.is_empty())
        .collect();
    let json = serde_json::to_string_pretty(&empty).unwrap();
    std::fs::write("empty.json", &json).unwrap();

    Source::write_typescript_file().unwrap();
}

// #[tokio::main]
// async fn main() {
//     // get_leagues_info().await.unwrap();

//     let info = LeagueReleaseInfo::fetch().await.unwrap();
//     dbg!(info);
// }

// #[tokio::main]
// async fn main() {
//     let divcord_table = DivcordTable::load().await.unwrap();
//     let poe_data = PoeData::load().await.unwrap();
//     let records = divcord_table.sourceful_records(&poe_data).unwrap();
//     let json = serde_json::to_string_pretty(&records).unwrap();
//     std::fs::write("records.json", &json).unwrap();

//     let records = check_deserialize().unwrap();

//     let empty: Vec<SourcefulDivcordTableRecord> = records
//         .clone()
//         .into_iter()
//         .filter(|r| r.sources.is_empty())
//         .collect();
//     let json = serde_json::to_string_pretty(&empty).unwrap();
//     std::fs::write("empty.json", &json).unwrap();

//     Source::write_typescript_file().unwrap();

//     let cards = poe_data::cards::fetch().await.unwrap();

//     let url = format!("{WIKI_API_URL}?action=cargoquery&format=json&tables=events&fields=events.name,release_date,release_version");
//     let s = reqwest::get(url).await.unwrap().text().await.unwrap();
//     println!("{s}");
//     std::fs::write("leagues.json", &s).unwrap();

//     let active = false;
//     let a = active.then_some("name");
//     dbg!(a);
// }
