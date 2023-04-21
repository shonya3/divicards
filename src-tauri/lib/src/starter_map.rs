use std::collections::HashMap;

use shared::{
    error::Error,
    types::record::{self, Record},
};

use crate::prices;

pub async fn starter_map() -> Result<HashMap<&'static str, Record>, Error> {
    let prices_json = prices::div_prices().await?;
    let names_prices = record::names_prices_from_json(&prices_json);
    let starter_map = record::create_starter_hashmap(names_prices);
    Ok(starter_map)
}
