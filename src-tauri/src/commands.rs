use crate::file_card_data::FileCardData;
use crate::prices;
use crate::starter_map;
use std::collections::HashMap;

use lib::{
    error::Error,
    types::{
        record::{self, Record},
        weighted_record::WeightedRecord,
    },
};
use serde::Serialize;
use tracing::instrument;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordsCsv {
    pub csv: String,
    pub records: Vec<Record>,
    pub not_cards: Vec<String>,
    pub fixed_names: HashMap<String, String>,
}

#[tauri::command]
pub async fn read_polish_csv(
    csv_string: &str,
    minimum_card_price: Option<f32>,
) -> Result<RecordsCsv, Error> {
    let map = starter_map::starter_map().await?;

    let mut records = record::csv::string::read(csv_string, minimum_card_price)?;
    let (records, fixed_names) = record::fix_record_names(&mut records, &record::CARDS, &map);

    let not_cards = record::find_not_divination_cards(&records, &map);
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
pub fn all_cards_price(csv_string: &str, minimum_card_price: Option<f32>) -> Result<f32, Error> {
    record::csv::string::all_cards_price(csv_string, minimum_card_price)
}

#[tauri::command]
pub async fn merge_csv(csv_file_strings: Vec<&str>) -> Result<String, Error> {
    let starter_map = starter_map::starter_map().await?;
    record::csv::string::merge(starter_map, &csv_file_strings[..])
}

#[tauri::command]
pub async fn update_prices() -> Result<(), Error> {
    Ok(prices::update_prices_data().await?)
}

#[tauri::command]
pub fn give_record() -> Result<Record, Error> {
    Ok(Record {
        stack_size: 0,
        name: String::from("test"),
        calculated: None,
        total: None,
    })
}

#[tauri::command]
pub async fn get_hashmap() -> Result<HashMap<&'static str, Record>, Error> {
    starter_map::starter_map().await
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WeightedRecordCsv {
    pub csv: String,
    pub records: Vec<WeightedRecord>,
}

#[tauri::command]
pub async fn weight_records_to_csv(records: Vec<Record>) -> Result<WeightedRecordCsv, Error> {
    let weighted_records = record::weight_records(records);
    let csv = lib::types::weighted_record::write(&weighted_records)?;
    Ok(WeightedRecordCsv {
        records: weighted_records,
        csv,
    })
}

#[tauri::command]
pub async fn create_file_card_data(
    csv_string: &str,
    minimum_card_price: Option<f32>,
) -> Result<FileCardData, Error> {
    FileCardData::create(csv_string, minimum_card_price).await
}
