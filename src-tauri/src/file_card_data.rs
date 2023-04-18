use std::collections::HashMap;

use lib::{
    error::Error,
    types::{
        record,
        weighted_record::{self, WeightedRecord},
    },
};
use serde::Serialize;
use tracing::instrument;

use crate::starter_map;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileCardData {
    pub all_cards_price: f32,
    pub records: Vec<WeightedRecord>,
    pub not_cards: Vec<String>,
    pub fixed_names: HashMap<String, String>,
    pub csv_polished: String,
    pub minimum_card_price: Option<f32>,
}

impl FileCardData {
    pub async fn create(csv_string: &str, minimum_card_price: Option<f32>) -> Result<Self, Error> {
        let map = starter_map::starter_map().await?;

        let mut records = record::csv::string::read(csv_string, None)?;

        let (records, fixed_names) = record::fix_record_names(&mut records, &record::CARDS, &map);

        let not_cards = record::find_not_divination_cards(&records, &map);

        let records = record::polish_records(records, map.clone())?;
        let records = record::weight_records(records);
        let csv_polished = lib::types::weighted_record::write(&records)?;
        let all_cards_price =
            record::csv::string::all_cards_price(&csv_polished, minimum_card_price)?;

        Ok(FileCardData {
            minimum_card_price,
            all_cards_price,
            records,
            not_cards,
            fixed_names,
            csv_polished,
        })
    }
}
