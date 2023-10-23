pub mod act;
pub mod card_element;
pub mod cards;
pub mod consts;
pub mod dropsource;
pub mod error;
pub mod loader;
pub mod mapbosses;
pub mod maps;
pub mod poe_data;
pub mod reward;
pub mod rich;
pub mod scripts;
pub mod table;
pub mod table_record;

use crate::{poe_data::PoeData, table::DivcordTable};

#[tokio::main]
async fn main() {
    let divcord_table = DivcordTable::load().await.unwrap();
    let poe_data = PoeData::load().await.unwrap();
    let records = divcord_table.parsed_records(&poe_data).unwrap();
    let json = serde_json::to_string_pretty(&records).unwrap();
    std::fs::write("records.json", &json).unwrap();
}
