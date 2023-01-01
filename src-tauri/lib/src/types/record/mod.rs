pub mod csv;
mod initial_data;

use std::collections::HashMap;

// pub use initial_data::create_starter_hashmap;
// pub use initial_data::download_json;
// pub use initial_data::dummy_tup_names_prices;
// pub use initial_data::is_legacy_card;
// pub use initial_data::names_prices_from_json;
// pub use initial_data::LEGACY_CARDS;
// pub use initial_data::NAMES;
// pub use initial_data::TUP_NAMES_PRICES;
pub use initial_data::*;

use serde::{Deserialize, Serialize};

use crate::error::Error;

use super::weighted_record::WeightedRecord;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    pub stack_size: i32,
    pub name: String,
    pub calculated: Option<f32>,
    pub total: Option<f32>,
}

impl Default for Record {
    fn default() -> Self {
        Self {
            stack_size: 0,
            name: String::default(),
            calculated: Some(0.),
            total: Some(0.),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NinjaRecord {
    pub name: String,
    #[serde(rename(serialize = "calculated", deserialize = "chaosValue"))]
    pub calculated: f32,
}

pub fn total_price_chaos(records: &Vec<Record>) -> f32 {
    records
        .iter()
        .map(|r| -> f32 { r.calculated.unwrap_or_default() * r.stack_size as f32 })
        .sum::<f32>()
}

pub fn total_price_divine(records: &Vec<Record>, divine_price: f32) -> f32 {
    total_price_chaos(records) / divine_price
}

pub fn map_to_vec(map: HashMap<&str, Record>) -> Vec<Record> {
    let mut v: Vec<Record> = vec![];
    for (_, record) in map {
        v.push(record);
    }
    v
}

pub fn vec_to_map(
    records: Vec<Record>,
    mut map: HashMap<&'static str, Record>,
) -> Result<HashMap<&'static str, Record>, Error> {
    // let mode = std::env::var("RUST_ENV");

    // let mut map = create_starter_hashmap();
    for r in records {
        let name = r.name.as_str();
        let record_from_map: &mut Record = match map.get_mut(name) {
            Some(record) => record,
            None => {
                // find_most_similar(name, &NAMES);

                tracing::warn!("{}", Error::NotDivinationCard(name.to_string()));
                continue;

                // tracing::error!("Could not get record from map. Record name: {}", name);
                // panic!("Could not get Record from map")
                // return Err(Error::BadRecord(name.to_string()));
            }
        };
        // map.get_mut(r.name.as_str())
        //     .expect("Could not get Record from map");

        record_from_map.stack_size = r.stack_size;
        record_from_map.total = Some(
            record_from_map.stack_size as f32 * record_from_map.calculated.unwrap_or_default(),
        );
    }

    Ok(map)
}

pub fn find_most_similar(name: &str, names: &[&str]) -> (String, f64) {
    let mut similarity_map = HashMap::<String, f64>::new();
    for initial_name in names {
        let similarity = strsim::normalized_damerau_levenshtein(name, initial_name);
        similarity_map.insert(initial_name.to_string(), similarity);
    }

    let most_similar = similarity_map
        .iter()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .unwrap();

    (most_similar.0.to_owned(), most_similar.1.to_owned())

    // println!("most similar for {name} is {:?}", most_similar);
}

pub fn fix_record_names(
    records: &mut Vec<Record>,
    names: &[&str],
    map: &HashMap<&'static str, Record>,
) -> (Vec<Record>, HashMap<String, String>) {
    let mut fixed_names: HashMap<String, String> = HashMap::new();
    let records = records
        .into_iter()
        .map(|record| {
            if let None = map.get(&*record.name) {
                let (similar_name, similar_score) = find_most_similar(&record.name, names);
                match similar_score >= 0.75 {
                    true => {
                        fixed_names.insert(record.name.clone(), similar_name.clone());
                        record.name = similar_name
                    }
                    false => {
                        let record_name = format!("The {}", record.name);
                        let (similar_name, similar_score) = find_most_similar(&record_name, names);
                        match similar_score >= 0.75 {
                            true => {
                                fixed_names.insert(record.name.clone(), similar_name.clone());
                                record.name = similar_name
                            }
                            false => {
                                tracing::warn!(
                                    "Too low similarity score. Name: {}, score: {similar_score}",
                                    record.name
                                )
                            }
                        }
                    }
                }
            }

            record.clone()
        })
        .collect::<Vec<Record>>();

    println!("Fixed names: {:?}", fixed_names);
    (records, fixed_names)
}

pub fn find_not_divination_cards(
    records: &Vec<Record>,
    map: &HashMap<&'static str, Record>,
) -> Vec<String> {
    let mut not_cards: Vec<String> = vec![];

    for r in records {
        let name = r.name.as_str();
        if let None = map.get(name) {
            not_cards.push(name.into());
        }
    }

    not_cards
}

pub fn polish_records(
    records: Vec<Record>,
    map: HashMap<&'static str, Record>,
) -> Result<Vec<Record>, Error> {
    let records_map = vec_to_map(records, map)?;
    let records = map_to_vec(records_map);
    Ok(records)
}

// #[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
// #[serde(rename_all = "camelCase")]
// pub struct WeightedRecord {
//     pub stack_size: i32,
//     pub name: String,
//     pub calculated: Option<f32>,
//     pub total: Option<f32>,
//     pub real_weight: f32,
// }

// impl Default for WeightedRecord {
//     fn default() -> Self {
//         Self {
//             real_weight: 0.,
//             stack_size: 0,
//             name: String::default(),
//             calculated: Some(0.),
//             total: Some(0.),
//         }
//     }
// }

fn calc_record_weight(record: &Record, all_stack_size: i32) -> f32 {
    record.stack_size as f32 / all_stack_size as f32
}

fn calc_record_real_weight(
    record: &Record,
    real_stacked_summary_weight: f32,
    condense_factor: f32,
    all_stack_size: i32,
) -> f32 {
    (real_stacked_summary_weight * calc_record_weight(record, all_stack_size))
        .powf(1.0 / condense_factor)
}

pub fn weight_records(records: Vec<Record>) -> Vec<WeightedRecord> {
    let all_stack_size: i32 = records.iter().map(|r| r.stack_size).sum();
    let real_stacked_rain_of_chaos_weight: f32 = 2452.65513;
    let condense_factor: f32 = 2.0 / 3.0;
    let rain_of_chaos = records.iter().find(|r| r.name == "Rain of Chaos").unwrap();
    let weight = rain_of_chaos.stack_size as f32 / all_stack_size as f32;
    let real_stacked_summary_weight = real_stacked_rain_of_chaos_weight / weight;
    records
        .into_iter()
        .map(|record| {
            let real_weight = calc_record_real_weight(
                &record,
                real_stacked_summary_weight,
                condense_factor,
                all_stack_size,
            );
            WeightedRecord {
                stack_size: record.stack_size,
                name: record.name,
                calculated: record.calculated,
                total: record.total,
                real_weight,
            }
        })
        .collect::<Vec<WeightedRecord>>()
}
