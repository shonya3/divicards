# divcord

A library for loading and parsing the [divcord spreadsheet](https://docs.google.com/spreadsheets/d/1Pf2KNuGguZLyf6eu_R0E503U0QNyfMZqaRETsN5g6kU/edit?pli=1#gid=0)

Check example dir.

## Example

```toml
[dependencies]
divcord = { git = "https://github.com/shonya3/divicards.git", features = ["fetch"]}
serde_json = "1"
tokio = { version = "1", features = ["full"] }
```

In project root, create .env file.
GOOGLE_API_KEY=

```rust
use divcord::{PoeData, Spreadsheet};

#[tokio::main]
async fn main() {
    let (poe_data, spreadsheet) = tokio::join!(PoeData::load(), Spreadsheet::load());
    let poe_data = poe_data.unwrap();
    let spreadsheet = spreadsheet.unwrap();

    let records = divcord::records(&spreadsheet, &poe_data).unwrap();
    let json = serde_json::to_string_pretty(&records).unwrap();
    std::fs::write("records.json", &json).unwrap();
}
```
