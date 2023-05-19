use crate::paths;
use divi::{League, Prices};
use std::{fs, path::Path};

pub const DAY_AS_SECS: u64 = 86_400;

pub async fn prices(league: &League) -> Prices {
    let path = paths::prices(&league);

    match Path::new(&path).exists() {
        true => match fs::metadata(&path) {
            Ok(metadata) => match metadata.modified() {
                Ok(time) => match time.elapsed().unwrap().as_secs() > DAY_AS_SECS {
                    true => update(&league).await.unwrap(),
                    false => match std::fs::read_to_string(&path) {
                        Ok(json) => serde_json::from_str(&json).unwrap(),
                        Err(err) => {
                            dbg!(err);
                            update(&league).await.unwrap()
                        }
                    },
                },
                Err(err) => {
                    dbg!(err);
                    update(&league).await.unwrap()
                }
            },
            Err(err) => {
                dbg!(err);
                update(&league).await.unwrap()
            }
        },
        false => update(&league).await.unwrap(),
    }
}

//TODO: add error types
pub async fn update(league: &League) -> Result<Prices, reqwest::Error> {
    let prices = Prices::fetch(league).await?;
    let json = serde_json::to_string(&prices).unwrap();
    std::fs::write(paths::prices(league), &json).unwrap();
    Ok(prices)
}
