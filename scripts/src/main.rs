use std::time::Instant;

use divcord::{
    cards::find_cards_by_source_types,
    dropsource::Source,
    table::{table_record::SourcefulDivcordTableRecord, DivcordTable},
};
use poe_data::PoeData;

#[tokio::main]
async fn main() {
    let divcord_table = DivcordTable::load().await.unwrap();
    let poe_data = PoeData::load().await.unwrap();
    let records = divcord_table.sourceful_records(&poe_data).unwrap();
    let json = serde_json::to_string_pretty(&records).unwrap();
    std::fs::write("records.json", &json).unwrap();

    let empty: Vec<SourcefulDivcordTableRecord> = records
        .clone()
        .into_iter()
        .filter(|r| r.sources.is_empty())
        .collect();
    let json = serde_json::to_string_pretty(&empty).unwrap();
    std::fs::write("empty.json", &json).unwrap();

    // let c = cards_by_mapboss("Eater of Souls", &records, &poe_data);
    // dbg!(c);

    Source::write_typescript_file().unwrap();

    let now = Instant::now();

    // let cards = find_cards_by_source_types(&[String::from("Map")], &records, &poe_data);
    let cards = find_cards_by_source_types(&Source::types(), &records, &poe_data);

    dbg!(now.elapsed());

    std::fs::write(
        "temp/vec.json",
        &serde_json::to_string_pretty(&cards).unwrap(),
    )
    .unwrap();
}
