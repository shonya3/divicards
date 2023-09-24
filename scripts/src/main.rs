pub mod card_element;
pub mod cards;
pub mod consts;
pub mod dropconsts;
pub mod dropsource;
pub mod error;
pub mod maps;
pub mod reward;
pub mod scripts;
pub mod table;
pub mod table_record;

#[allow(unused)]
use dropsource::{DropSource, Vendor};
#[allow(unused)]
use serde_json::{json, Value};
#[allow(unused)]
use std::collections::HashSet;
#[allow(unused)]
use std::path::Path;
#[allow(unused)]
use std::{collections::HashMap, fmt::Display, slice::Iter};
#[allow(unused)]
use table_record::{CardDropTableRecord, Confidence, GreyNote};

#[allow(unused)]
use error::Error;

#[allow(unused)]
use crate::scripts::read_original_table_sheet;
#[allow(unused)]
use crate::table::Table;

// #[tokio::main]
// async fn main() {}

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

pub fn main() {

    // let mut _map: HashMap<&CardDropTableRecord, Vec<HashSet<DropSource>>> = HashMap::new();

    // let mut set: HashSet<DropSource> = HashSet::new();
    // set.insert(DropSource::ChestObject);
    // set.insert(DropSource::ExpeditionLogbook);
    // set.insert(DropSource::Vendor(Some(Vendor::KiracShop)));
    // set.insert(DropSource::Vendor(Some(Vendor::KiracShop)));
    // dbg!(set);

    // let sheet = read_original_table_sheet("sheet.json").unwrap();
    // let records = parse_table(&sheet.values[2..]).unwrap();

    // for record in records {
    //     let drop_source = parse_drop_source(&record).unwrap();
    //     if drop_source.contains(&DropSource::ExpeditionLogbook) {
    //         dbg!(record);
    //     }
    // }
    // std::fs::write("map.json", &serde_json::to_string_pretty(&map).unwrap()).unwrap();
}
