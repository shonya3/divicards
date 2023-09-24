pub mod card_element;
pub mod cards;
pub mod consts;
pub mod dropconsts;
pub mod dropsource;
pub mod error;
pub mod maps;
pub mod reward;
pub mod scripts;
pub mod table_record;

#[allow(unused)]
use serde_json::{json, Value};
#[allow(unused)]
use std::{collections::HashMap, fmt::Display, slice::Iter};

#[allow(unused)]
use error::Error;

#[tokio::main]
async fn main() {}

#[allow(unused)]
use crate::scripts::{parse_table, read_original_table_sheet};

// pub fn parse_drop_source(record: &CardDropTableRecord) -> Result<Vec<DropSource>, Error> {
//     let mut sources: Vec<DropSource> = Vec::new();

//     if let Some(tag_hypothesis) = &record.tag_hypothesis {
//         if tag_hypothesis.contains("logbook") {
//             sources.push(DropSource::ExpeditionLogbook);
//         }
//     }

//     if let Some(greynote) = &record.greynote {
//         if greynote == &GreyNote::Disabled {
//             sources.push(DropSource::Disabled);
//         }

//         if greynote == &GreyNote::Vendor {
//             if let Some(_drops_from) = &record.drops_from {}
//             // return Ok(DropSource::Vendor());
//         }
//     }

//     // match greynote {
//     //     GreyNote::Disabled => return Ok(DropSource::Disabled),
//     //     GreyNote::Delirium => return Ok(DropSource::Delirium),
//     //     GreyNote::ChestObject => return Ok(DropSource::ChestObject),
//     //     GreyNote::GlobalDrop => return Ok(DropSource::GlobalDrop),
//     //     GreyNote::Vendor => return Ok(DropSource::Vendor),
//     //     GreyNote::Strongbox => return Ok(DropSource::Strongbox),
//     //     GreyNote::AreaSpecific => todo!(),
//     //     GreyNote::MonsterSpecific => todo!(),
//     //     GreyNote::Story => todo!(),
//     // }

//     // let sources = sources.into_iter().unique();

//     Ok(sources)
// }

// pub fn temp_main() {
//     let sheet = read_original_table_sheet("sheet.json").unwrap();
//     let records = parse_table(&sheet.values[2..]).unwrap();

//     let mut confidence_map: HashMap<Confidence, u16> = HashMap::new();
//     for record in &records {
//         let counter = confidence_map.entry(record.confidence.clone()).or_insert(0);
//         *counter += 1;
//     }

//     dbg!(confidence_map);

//     let mut map: HashMap<String, Vec<CardDropTableRecord>> = HashMap::new();
//     for record in records {
//         let vec = map.entry(record.name.as_str().to_owned()).or_insert(vec![]);
//         vec.push(record);
//     }

//     dbg!(map.keys().len());
//     std::fs::write("map.json", serde_json::to_string_pretty(&map).unwrap()).unwrap();

//     let mut multiple_map: HashMap<String, Vec<CardDropTableRecord>> = HashMap::new();
//     for (name, record) in map {
//         if record.len() > 1 {
//             multiple_map.insert(name.clone(), record.clone());
//         }
//     }

//     dbg!(multiple_map.keys().len());
//     std::fs::write(
//         "multiple-map.json",
//         serde_json::to_string_pretty(&multiple_map).unwrap(),
//     )
//     .unwrap();

//     let mut _map: HashMap<&CardDropTableRecord, Vec<HashSet<DropSource>>> = HashMap::new();

//     let mut set: HashSet<DropSource> = HashSet::new();
//     set.insert(DropSource::ChestObject);
//     set.insert(DropSource::ExpeditionLogbook);
//     set.insert(DropSource::Vendor(Some(Vendor::KiracShop)));
//     set.insert(DropSource::Vendor(Some(Vendor::KiracShop)));
//     dbg!(set);

//     let sheet = read_original_table_sheet("sheet.json").unwrap();
//     let records = parse_table(&sheet.values[2..]).unwrap();

//     for record in records {
//         let drop_source = parse_drop_source(&record).unwrap();
//         if drop_source.contains(&DropSource::ExpeditionLogbook) {
//             dbg!(record);
//         }
//     }
//     // std::fs::write("map.json", &serde_json::to_string_pretty(&map).unwrap()).unwrap();
// }

// pub fn write_sized_rewards() {
//     let vec: Vec<NinjaCardData> =
//         serde_json::from_str(&std::fs::read_to_string("ninja-data.json").unwrap()).unwrap();
//     let mut with_size: Vec<String> = Vec::new();
//     for card_data in vec {
//         let reward = &card_data.explicit_modifiers[0].text;
//         if reward.contains("<size:") {
//             with_size.push(reward.clone());
//         }
//     }

//     std::fs::write(
//         "rewards-with-size.json",
//         serde_json::to_string(&with_size).unwrap(),
//     )
//     .unwrap();
// }
