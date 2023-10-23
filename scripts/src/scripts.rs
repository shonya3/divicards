use std::{collections::HashMap, path::Path};

use googlesheets::sheet::ValueRange;
use reqwest::Client;
use serde_json::Value;

use crate::{
    error::Error,
    table::table_record::{CardDropTableRecord, Confidence},
    table::Table,
};

pub fn read_original_table_sheet<P: AsRef<Path>>(path: P) -> Result<ValueRange, Error> {
    let sheet: ValueRange = serde_json::from_str(&std::fs::read_to_string(path)?)?;
    Ok(sheet)
}

pub fn read_table<P: AsRef<Path>>(path: P) -> Result<Table, Error> {
    let sheet = read_original_table_sheet(path)?;
    Ok(Table::try_from(&sheet)?)
}

pub async fn download_table_sheet() -> Result<ValueRange, Error> {
    dotenv::dotenv().ok();
    let api_key = std::env::var("GOOGLE_API_KEY").expect("No google api key");

    let url = format!("https://sheets.googleapis.com/v4/spreadsheets/1Pf2KNuGguZLyf6eu_R0E503U0QNyfMZqaRETsN5g6kU/values/Cards_and_Hypotheses?key={api_key}");
    let value_range: ValueRange = Client::new().get(url).send().await?.json().await?;
    Ok(value_range)
}

pub fn write_table_sheet<P: AsRef<Path>>(path: P, sheet: &ValueRange) -> Result<(), Error> {
    std::fs::write(path, serde_json::to_string_pretty(&sheet)?)?;
    Ok(())
}

pub fn write_notes<P: AsRef<Path>>(path: P, sheet: &ValueRange) -> Result<(), Error> {
    fn parse_notes(row: &[Value]) -> Result<String, Error> {
        if row.len() < 9 {
            return Err(Error::RowIsTooShort("Notes".to_string(), 9));
        };

        Ok(row[8].to_string())
    }

    let mut vec: Vec<String> = vec![];
    for row in &sheet.values {
        if let Ok(notes) = parse_notes(&row) {
            vec.push(notes);
        }
    }

    std::fs::write(path, serde_json::to_string_pretty(&vec)?)?;
    Ok(())
}

pub fn write_hypothesis_tags<P: AsRef<Path>>(path: P, sheet: &ValueRange) -> Result<(), Error> {
    let mut tags: Vec<&str> = vec![];
    for row in &sheet.values[2..] {
        if row.len() < 3 {
            continue;
        }

        let Some(s) = row[2].as_str() else {
            continue;
        };

        if s.is_empty() {
            continue;
        }

        tags.push(s);
    }

    let s = serde_json::to_string_pretty(&tags)?;
    std::fs::write(path, s)?;

    Ok(())
}

pub fn write_parsed_table<P: AsRef<Path>>(path: P, table: &Table) -> Result<(), Error> {
    let json = serde_json::to_string_pretty(&table)?;
    std::fs::write(path, &json)?;

    Ok(())
}

pub fn write_drops_from<P: AsRef<Path>>(path: P, table: &Table) -> Result<(), Error> {
    let drops_from: Vec<Option<String>> = table
        .0
        .iter()
        .map(|record| record.drops_from.to_owned())
        .collect();

    std::fs::write(path, serde_json::to_string_pretty(&drops_from)?)?;
    Ok(())
}

pub fn write_confidence_map<P: AsRef<Path>>(path: P, table: &Table) -> Result<(), Error> {
    let mut confidence_map: HashMap<Confidence, u16> = HashMap::new();
    for record in &table.0 {
        let counter = confidence_map.entry(record.confidence.clone()).or_insert(0);
        *counter += 1;
    }

    std::fs::write(path, serde_json::to_string(&confidence_map)?)?;
    Ok(())
}

pub fn write_hypothesis_maps(table: Table) -> Result<(), Error> {
    let mut map: HashMap<String, Vec<CardDropTableRecord>> = HashMap::new();
    for record in table.0 {
        let vec = map.entry(record.name.as_str().to_owned()).or_insert(vec![]);
        vec.push(record);
    }

    dbg!(map.keys().len());
    std::fs::write(
        "jsons/hypothesis-map.json",
        serde_json::to_string_pretty(&map)?,
    )?;

    let mut multiple_map: HashMap<String, Vec<CardDropTableRecord>> = HashMap::new();
    for (name, record) in map {
        if record.len() > 1 {
            multiple_map.insert(name.clone(), record.clone());
        }
    }

    dbg!(multiple_map.keys().len());
    std::fs::write(
        "jsons/multiple-hypothesis-map.json",
        serde_json::to_string_pretty(&multiple_map)?,
    )?;

    Ok(())
}

// pub fn write_sources(sheet: &ValueRange) -> Result<(), Error> {
//     let table = Table::try_from(sheet)?;
//     let drops = table.all_drops_from();
//     let mut unparsed: Vec<String> = Vec::new();
//     let mut sources: Vec<Source> = Vec::new();

//     for s in drops {
//         match s.parse::<Source>() {
//             Ok(source) => sources.push(source),
//             Err(_) => {
//                 unparsed.push(s);
//             }
//         }
//     }

//     std::fs::write(
//         "jsons/sources.json",
//         serde_json::to_string(&sources).unwrap(),
//     )?;

//     std::fs::write(
//         "jsons/unparsed_sources.json",
//         serde_json::to_string(&unparsed).unwrap(),
//     )?;

//     Ok(())
// }

pub async fn update_all_jsons() {
    let sheet = download_table_sheet()
        .await
        .expect("Download table sheet error");
    write_hypothesis_tags("jsons/hypothesis-tags.json", &sheet)
        .expect("Write hypothesis tags error");
    write_notes("jsons/notes.json", &sheet).expect("Write notes error");
    write_table_sheet("jsons/sheet.json", &sheet).expect("Write  sheet error");

    let table = Table::try_from(&sheet).expect("Could not parse the table");
    write_parsed_table("jsons/parsed-table.json", &table).expect("Write parsed table error");
    write_drops_from("jsons/drops-from.json", &table).expect("Write drops-from error");
    write_confidence_map("jsons/confidence-map.json", &table).expect("Wrtie confidence map error");
    write_hypothesis_maps(table.clone()).expect("write_hypothesis_maps eror");
    // write_sources(&sheet).unwrap();
    // write_drops_debug_jsons(table);
}

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

// pub fn write_drops_debug_jsons(table: Table) {
//     // update_all_jsons().await;
//     let mut map: HashMap<String, HashSet<DropSource>> = HashMap::new();

//     for card in CARDS {
//         map.insert(card.to_string(), HashSet::new());
//     }

//     let mut unrecognized: Vec<CardDropTableRecord> = Vec::new();
//     for table_record in table.0 {
//         let set = table_record.resolve_dropsources();
//         if table_record.drops_from.is_none() {
//             if set.len() == 0 {
//                 unrecognized.push(table_record.clone());
//             }
//         };

//         if set.len() > 0 {
//             map.entry(table_record.name)
//                 .and_modify(|s1| s1.extend(set.clone()))
//                 .or_insert(set.clone());
//         }
//     }

//     let mut unrecognized_names: Vec<String> = Vec::new();
//     for (name, set) in &map {
//         if set.len() == 0 {
//             unrecognized_names.push(name.to_string());
//         }
//     }

//     dbg!(&unrecognized_names.len());

//     std::fs::write(
//         "jsons/cards_without_dropsorces.json",
//         &serde_json::to_string(&unrecognized_names).unwrap(),
//     )
//     .unwrap();

//     std::fs::write(
//         "jsons/unrecognized.json",
//         serde_json::to_string(&unrecognized).unwrap(),
//     )
//     .unwrap();

//     std::fs::write("jsons/drops.json", serde_json::to_string(&map).unwrap()).unwrap();
//     dbg!(unrecognized.len());
// }
