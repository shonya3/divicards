use crate::paths;
use divi::{error::Error, League, Prices};

pub async fn prices(league: &League) -> Prices {
    let path = paths::prices(&league);
    match std::fs::read_to_string(&path) {
        Ok(json) => serde_json::from_str(&json).unwrap(),
        Err(_) => {
            println!("Error reading file. Fetchin new one");
            update(&league).await.unwrap()
        }
    }
}

//TODO: add error types
pub async fn update(league: &League) -> Result<Prices, Error> {
    let prices = Prices::fetch(league).await?;
    let json = serde_json::to_string(&prices).unwrap();
    std::fs::write(paths::prices(league), &json).unwrap();
    Ok(prices)
}
