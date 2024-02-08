#![allow(unused)]

use std::{clone, path::Display};

use divcord::{
    dropsource::Source,
    table::{rich::RichColumn, table_record::SourcefulDivcordTableRecord, DivcordTable},
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

    // Source::write_typescript_file().unwrap();

    for (index, record) in divcord_table
        .sourceful_records_iter(poe_data.clone())
        .enumerate()
    {
        // if index == 63 {
        //     println!("{record:#?}");
        // }

        // if let Ok(record) = record {
        //     if record.id == 66 {
        //         println!("here");
        //         println!("{record:#?}");
        //     }
        // }

        let record = record.unwrap();

        for verify in record.verify_sources {
            // println!("{verify:?}");
        }
    }
}

fn h_column() -> RichColumn {
    serde_json::from_str(&std::fs::read_to_string("H.json").unwrap()).unwrap()
}

pub async fn inspect_h_column(divcord_table: DivcordTable) {
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

    for record in divcord_table.records() {
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

//     // Source::write_typescript_file().unwrap();
// }
