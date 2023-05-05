use std::collections::HashMap;

use serde::Serialize;
use shared::{
    error::Error,
    types::record::{self, Record},
};

use crate::{file_card_data::FileCardData, prices, starter_map};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordsCsv {
    pub csv: String,
    pub records: Vec<Record>,
    pub not_cards: Vec<String>,
    pub fixed_names: HashMap<String, String>,
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
pub async fn create_file_card_data(
    csv_string: &str,
    minimum_card_price: Option<f32>,
) -> Result<FileCardData, Error> {
    FileCardData::create(csv_string, minimum_card_price).await
}
