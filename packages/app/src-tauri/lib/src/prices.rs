#![allow(unused)]

use crate::{error::Error, paths};
use divi::{league::TradeLeague, prices::Prices};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};
use tracing::{debug, instrument};

pub const DAY_AS_SECS: u64 = 86_400;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppCardPrices {
    pub dir: PathBuf,
    pub prices_by_league: HashMap<TradeLeague, Prices>,
}
impl AppCardPrices {
    pub const fn new(
        dir: PathBuf,
        prices_by_league: HashMap<TradeLeague, Prices>,
    ) -> AppCardPrices {
        AppCardPrices {
            dir,
            prices_by_league,
        }
    }

    #[instrument(skip(self))]
    pub async fn get_or_update(&mut self, league: &TradeLeague) -> Prices {
        match self.prices_by_league.get(league) {
            Some(prices) => prices.to_owned(),
            None => match self.up_to_date(league) {
                true => {
                    let prices = self.read_from_file(league).unwrap();
                    self.prices_by_league
                        .insert(league.to_owned(), prices.clone());
                    prices
                }
                false => {
                    dbg!("get_or_update. false branch of up_to_date");
                    match self.fetch_and_update(league).await {
                        Ok(prices) => prices,
                        Err(err) => {
                            dbg!(err);
                            Prices::default()
                        }
                    }
                }
            },
        }
    }

    pub fn league_path(&self, league: &TradeLeague) -> PathBuf {
        self.dir.join(format!("{}-prices.json", { league }))
    }

    #[instrument(skip(self))]
    async fn fetch_and_update(&mut self, league: &TradeLeague) -> Result<Prices, Error> {
        let prices = Prices::fetch(league).await?;
        debug!("fetch_and_update: fetched. Serializing to json");
        let json = serde_json::to_string(&prices)?;

        debug!("fetch_and_update: Serialized. Next write to file");

        std::fs::write(self.league_path(league), &json).unwrap();

        debug!("fetch_and_update: wrote to file");
        self.prices_by_league
            .insert(league.to_owned(), prices.clone());

        Ok(prices)
    }

    fn read_from_file(&self, league: &TradeLeague) -> Option<Prices> {
        match std::fs::read_to_string(self.league_path(league)) {
            Ok(json) => serde_json::from_str(&json).unwrap(),
            Err(_) => {
                dbg!("No file");
                None
            }
        }
    }

    fn up_to_date(&self, league: &TradeLeague) -> bool {
        let path = self.league_path(league);
        let exists = path.try_exists().unwrap();
        dbg!("up_to_date:before match", exists);
        match exists {
            true => match fs::metadata(&path) {
                Ok(metadata) => match metadata.modified() {
                    Ok(time) => time.elapsed().unwrap().as_secs() < DAY_AS_SECS,
                    Err(_) => false,
                },
                Err(_) => false,
            },
            false => false,
        }
    }
}

impl Default for AppCardPrices {
    fn default() -> Self {
        Self {
            dir: paths::appdata(),
            prices_by_league: Default::default(),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::prices::AppCardPrices;

//     #[tokio::test]
//     async fn appcards() {
//         let mut prices = AppCardPrices::default();
//         let prices = prices
//             .get_or_update(&divi::league::TradeLeague::Crucible)
//             .await;
//         dbg!(prices);
//     }
// }
