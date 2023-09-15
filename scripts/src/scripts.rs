use googlesheets::sheet::ValueRange;
use reqwest::Client;
use serde_json::Value;

use crate::{error::Error, parse_row, read_original_table_sheet, CardDropRecord};

pub async fn download_table_sheet() -> Result<ValueRange, Error> {
    dotenv::dotenv().ok();
    let api_key = std::env::var("GOOGLE_API_KEY").expect("No google api key");

    let url = format!("https://sheets.googleapis.com/v4/spreadsheets/1Pf2KNuGguZLyf6eu_R0E503U0QNyfMZqaRETsN5g6kU/values/Cards_and_Hypotheses?key={api_key}");
    let value_range: ValueRange = Client::new().get(url).send().await?.json().await?;
    Ok(value_range)
}

pub async fn write_table_sheet_json() -> Result<(), Error> {
    let sheet = download_table_sheet().await?;
    std::fs::write("table-source.json", serde_json::to_string_pretty(&sheet)?)?;
    Ok(())
}

pub fn write_notes() {
    fn parse_notes(row: &[Value]) -> Result<String, Error> {
        if row.len() < 9 {
            return Err(Error::RowIsTooShort("Notes".to_string(), 9));
        };

        Ok(row[8].to_string())
    }

    let mut vec: Vec<String> = vec![];
    for row in &read_original_table_sheet().values {
        if let Ok(notes) = parse_notes(&row) {
            vec.push(notes);
        }
    }

    std::fs::write("notes.json", serde_json::to_string_pretty(&vec).unwrap()).unwrap();
}

pub fn write_hypothesis_tags() {
    let mut tags: Vec<&str> = vec![];
    let vr = read_original_table_sheet();
    for row in &vr.values[2..] {
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

        println!("{}", s);
    }

    let s = serde_json::to_string(&tags).unwrap();
    std::fs::write("tags.json", s).unwrap();
}

pub fn test_parse_table() {
    pub fn parse_table(values: &[Vec<Value>]) -> Result<Vec<CardDropRecord>, Error> {
        let mut records: Vec<CardDropRecord> = Vec::new();
        for row in values {
            match parse_row(row) {
                Ok(record) => records.push(record),
                Err(err) => {
                    println!("{err}");
                }
            }
        }

        Ok(records)
    }

    let vr = read_original_table_sheet();
    let table = parse_table(&vr.values[2..]).unwrap();
    let json = serde_json::to_string_pretty(&table).unwrap();
    std::fs::write("parsed-table.json", &json).unwrap();

    let drops_from: Vec<Option<String>> = table
        .iter()
        .map(|record| record.drops_from.to_owned())
        .collect();

    std::fs::write(
        "drops-from.json",
        serde_json::to_string(&drops_from).unwrap(),
    )
    .unwrap();
}
