use crate::paths;
use divi::{error::Error, League, Prices};
use std::path::PathBuf;

pub async fn prices(league: League) -> Prices {
    let path = paths::prices();
    match std::fs::read_to_string(&path) {
        Ok(json) => serde_json::from_str(&json).unwrap(),
        Err(_) => {
            println!("Error reading file. Fetchin new one");
            update(&path, league).await.unwrap()
        }
    }
}

// TODO: add error types
pub async fn update(path: &PathBuf, league: League) -> Result<Prices, Error> {
    let prices = Prices::fetch(league).await?;
    let json = serde_json::to_string(&prices).unwrap();
    std::fs::write(path, &json).unwrap();
    Ok(prices)
}
