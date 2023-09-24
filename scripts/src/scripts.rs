use std::path::Path;

use googlesheets::sheet::ValueRange;
use reqwest::Client;
use serde_json::Value;

use crate::{error::Error, table::Table};

pub fn read_original_table_sheet<P: AsRef<Path>>(path: P) -> Result<ValueRange, Error> {
    let sheet: ValueRange = serde_json::from_str(&std::fs::read_to_string(path)?)?;
    Ok(sheet)
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

pub async fn update_all_jsons() {
    let sheet = download_table_sheet()
        .await
        .expect("Download table sheet error");
    write_hypothesis_tags("hypothesis-tags.json", &sheet).expect("Write hypothesis tags error");
    write_notes("notes.json", &sheet).expect("Write notes error");
    write_table_sheet("sheet.json", &sheet).expect("Write  sheet error");

    let table = Table::try_from(&sheet).expect("Could not parse the table");
    write_parsed_table("parsed-table.json", &table).expect("Write parsed table error");
    write_drops_from("drops-from.json", &table).expect("Write drops-from error");
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
