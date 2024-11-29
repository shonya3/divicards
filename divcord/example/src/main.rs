//!
//! Activate "fetch" feature
//! ```Cargo.toml
//! divcord = {path = "../divcord", features = ["fetch"]}
//! ```

use divcord::{PoeData, Source, Spreadsheet};
use std::fs;

#[tokio::main]
async fn main() {
    // Keep in mind, PoeData needs Playwright for data scrapping and requires special feature(check Cargo.toml).
    // Spreadsheet is just 3 http requests, and can be loaded without special conditions and features.
    let (poe_data, spreadsheet) = tokio::join!(PoeData::load(), Spreadsheet::load());
    let poe_data = poe_data.unwrap();
    let spreadsheet = spreadsheet.unwrap();

    // parse and write to json
    let records = divcord::records_with_collect_all_errors(&spreadsheet, &poe_data).unwrap();
    fs::write(
        "records.json",
        serde_json::to_string_pretty(&records).unwrap(),
    )
    .unwrap();

    // More low-level function. Iterate the records and do something
    for result in divcord::records_iter(&spreadsheet, &poe_data) {
        let record_result = result.unwrap();
        let record = record_result.record;
        let errors = record_result.errors;
        if !errors.is_empty() {
            println!("{errors:#?}");
        }

        let boxes_and_chests_string = record
            .sources
            .iter()
            .filter_map(|source| match source {
                Source::Strongbox(..) | Source::Chest(..) => Some(source.to_string()),
                _ => None,
            })
            .collect::<Vec<String>>()
            .join("; ");

        if !boxes_and_chests_string.is_empty() {
            println!("#{} {} {}", record.id, record.card, boxes_and_chests_string)
        }
    }
}
