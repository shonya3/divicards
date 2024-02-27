# divcord

A library for loading and parsing the [divcord spreadsheet](https://docs.google.com/spreadsheets/d/1Pf2KNuGguZLyf6eu_R0E503U0QNyfMZqaRETsN5g6kU/edit?pli=1#gid=0)

## Example

Add divcord and poe_data to Cargo.toml. You'll also need "fetch" features and tokio, if you need to load data first.

```toml
divcord = {path = "../divcord", features = ["fetch"]}
poe_data = {path = "../poe_data", features = ["fetch"]}
tokio = { version = "1", features = ["full"] }
serde_json = "1.0"
```

And then get started in your `main.rs`:

```rust
use divcord::{Error, Source, Spreadsheet};
use poe_data::PoeData;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let spreadsheet = Spreadsheet::load().await.unwrap();
    let poe_data = PoeData::load().await.unwrap();

    // parse and write to json, that's all
    let records = divcord::records(&spreadsheet, &poe_data)?;
    fs::write("records.json", serde_json::to_string_pretty(&records)?)?;

    // iterate the records and do something
    for record in divcord::records_iter(&spreadsheet, &poe_data) {
        let record = record?;
        let boxes_and_chests_string = record
            .sources
            .iter()
            .filter_map(|source| match source {
                Source::Strongbox(..) | Source::Chest(..) => Some(source.to_string()),
                Source::GlobalDrop { .. } => {
                    println!("GlobalDrop #{} {} {source:?}", record.id, record.card);
                    None
                }
                _ => None,
            })
            .collect::<Vec<String>>()
            .join("; ");

        if !boxes_and_chests_string.is_empty() {
            println!("#{} {} {}", record.id, record.card, boxes_and_chests_string)
        }
    }

    Ok(())
}
```
