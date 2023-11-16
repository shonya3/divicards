use divcord::{
    dropsource::Source,
    table::{table_record::SourcefulDivcordTableRecord, DivcordTable},
};
use poe_data::PoeData;
use serde_json::json;

#[tokio::main]
async fn main() {
    let divcord_table = DivcordTable::load().await.unwrap();
    let poe_data = PoeData::load().await.unwrap();
    let records = divcord_table.sourceful_records(&poe_data).unwrap();
    let json = serde_json::to_string_pretty(&records).unwrap();
    for r in &records {}
    std::fs::write("records.json", &json).unwrap();

    let empty: Vec<SourcefulDivcordTableRecord> = records
        .into_iter()
        .filter(|r| r.sources.is_empty())
        .collect();
    let json = serde_json::to_string_pretty(&empty).unwrap();
    std::fs::write("empty.json", &json).unwrap();

    Source::write_typescript_file().unwrap();
}
