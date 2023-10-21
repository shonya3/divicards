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

use std::collections::HashMap;

use dropsource::Source;
use error::Error;

use crate::{poe_data::PoeData, table::DivcordTable};

#[tokio::main]
async fn main() {
    let divcord_table = DivcordTable::load().await.unwrap();
    let poe_data = PoeData::load().await.unwrap();
    dbg!(sources_by_card(&divcord_table, &poe_data).unwrap());
}

pub fn sources_by_card(
    divcord_table: &DivcordTable,
    poe_data: &PoeData,
) -> Result<HashMap<String, Vec<Source>>, Error> {
    let mut map: HashMap<String, Vec<Source>> = HashMap::new();
    for record in divcord_table.records() {
        let record = record?;
        for d in &record.drops_from {
            let sources = crate::dropsource::parse_source(d, &record, poe_data).unwrap();
            for source in sources {
                map.entry(record.name.clone()).or_default().push(source);
            }
        }
    }

    Ok(map)
}
