#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::collections::HashMap;

use lib::{
    error::Error,
    types::{
        record::{self, find_not_divination_cards, Record},
        weighted_record::WeightedRecord,
    },
};

use serde::Serialize;
#[cfg(debug_assertions)]
use tauri::Manager;

#[tokio::main]
async fn main() {
    lib::dev::init_tracing();
    tracing::event!(tracing::Level::DEBUG, "app startup");
    match update_prices_data().await {
        Ok(_) => tracing::event!(tracing::Level::DEBUG, "prices updated"),
        Err(err) => tracing::event!(tracing::Level::ERROR, "could not update prices {:?}", err),
    };

    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            read_polish_csv,
            total_chaos,
            merge_csv,
            update_prices,
            give_record,
            get_hashmap,
            weight_records_to_csv
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RecordsCsv {
    csv: String,
    records: Vec<Record>,
    not_cards: Vec<String>,
    fixed_names: HashMap<String, String>,
}

#[tauri::command]
async fn read_polish_csv(
    csv_string: &str,
    minimum_card_price: Option<f32>,
) -> Result<RecordsCsv, Error> {
    let map = starter_map().await?;

    let mut records = record::csv::string::read(csv_string, minimum_card_price)?;
    let (records, fixed_names) = record::fix_record_names(&mut records, &record::CARDS, &map);

    let not_cards = find_not_divination_cards(&records, &map);
    println!("not cards: {:?}", not_cards);

    let records = record::polish_records(records, map.clone())?;
    let csv_output = record::csv::string::write(&records)?;

    Ok(RecordsCsv {
        csv: csv_output,
        records,
        not_cards,
        fixed_names,
    })
}

#[tauri::command]
fn total_chaos(csv_string: &str, minimum_card_price: Option<f32>) -> Result<f32, Error> {
    record::csv::string::total_chaos(csv_string, minimum_card_price)
}

#[tauri::command]
async fn merge_csv(csv_file_strings: Vec<&str>) -> Result<String, Error> {
    let starter_map = starter_map().await?;
    record::csv::string::merge(starter_map, &csv_file_strings[..])
}

#[tauri::command]
async fn update_prices() -> Result<(), Error> {
    Ok(update_prices_data().await?)
}

// Ok(Record::default())
#[tauri::command]
fn give_record() -> Result<Record, Error> {
    Ok(Record {
        stack_size: 0,
        name: String::from("test"),
        calculated: None,
        total: None,
    })
}

#[tauri::command]
async fn get_hashmap() -> Result<HashMap<&'static str, Record>, Error> {
    starter_map().await
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct WeightedRecordCsv {
    csv: String,
    records: Vec<WeightedRecord>,
}

#[tauri::command]
async fn weight_records_to_csv(records: Vec<Record>) -> Result<WeightedRecordCsv, Error> {
    let weighted_records = record::weight_records(records);
    let csv = lib::types::weighted_record::write(&weighted_records)?;
    Ok(WeightedRecordCsv {
        records: weighted_records,
        csv,
    })
}

async fn starter_map() -> Result<HashMap<&'static str, Record>, Error> {
    let prices_json = div_prices().await?;
    let names_prices = record::names_prices_from_json(&prices_json);
    let starter_map = record::create_starter_hashmap(names_prices);
    Ok(starter_map)
}

async fn div_prices() -> Result<String, Error> {
    let path = prices_path();
    let json = match std::fs::read_to_string(&path) {
        Ok(json) => json,
        Err(_) => {
            let json = record::fetch_div_prices().await?;
            std::fs::write(path, &json)?;
            json
        }
    };

    Ok(json)
}

async fn update_prices_data() -> Result<(), Error> {
    let path = prices_path();

    let json = record::fetch_div_prices().await?;
    std::fs::write(path, &json)?;
    Ok(())
}

fn get_appdata_dir() -> std::path::PathBuf {
    let mut path = tauri::api::path::config_dir().unwrap();
    path.push("divicards");

    if !path.exists() {
        std::fs::create_dir(&path).expect("Error on appdata dir creation");
    }

    path
}

fn prices_path() -> std::path::PathBuf {
    let mut path = get_appdata_dir();
    path.push("div-prices.json");
    path
}

// const S: &'static str = "stackSize,name,calculated,total
// 49,The Opulent,1.0,49.0
// 0,The Price of Devotion,5772.65,0.0
// 0,Perfection,3.0,0.0
// 65,Three Faces in the Dark,1.0,65.0";
