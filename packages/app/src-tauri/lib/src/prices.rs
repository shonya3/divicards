use crate::{
    error::Error,
    event::{self, Event, ToastVariant},
    paths,
};
use divi::{league::TradeLeague, prices::Prices};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf};
use tauri::Window;
use tracing::{debug, instrument};

pub const DAY_AS_SECS: f64 = 86_400.0;

pub struct DaysOld(Option<f32>);

pub enum LeagueFileState {
    UpToDate(Prices),
    StillUsable(Prices, f32),
    TooOld,
    Invalid,
    NoFile,
}

impl AppCardPrices {
    pub fn read_file(&self, league: &TradeLeague) -> LeagueFileState {
        if !self.league_file_exists(league) {
            return LeagueFileState::NoFile;
        }

        let Ok(prices) =self.read_from_file(league) else {
            return LeagueFileState::Invalid;
        };

        if let Some(days_old) = self.file_days_old(league) {
            match days_old {
                n if n <= 1.0 => LeagueFileState::UpToDate(prices),
                n if n <= 7.0 => LeagueFileState::StillUsable(prices, n),
                _ => LeagueFileState::TooOld,
            }
        } else {
            return LeagueFileState::NoFile;
        }
    }

    pub fn read_league_file(&self, league: &TradeLeague) -> Result<Prices, Error> {
        let json = std::fs::read_to_string(self.league_path(league))?;
        let prices = serde_json::from_str::<Prices>(&json)?;
        Ok(prices)
    }

    pub async fn get_price(&mut self, league: &TradeLeague, window: &Window) -> Prices {
        if let Some(prices) = self.prices_by_league.get(league) {
            return prices.to_owned();
        }

        match self.read_file(league) {
            LeagueFileState::UpToDate(prices) => prices,
            _ => self
                .fetch_and_update(league, window)
                .await
                .unwrap_or_else(|_| {
                    Event::Toast {
                        variant: ToastVariant::Warning,
                        message: format!("Unable to load prices for league {league}. Skip price-dependant calculations."),
                    }
                    .emit(&window);
                    Prices::default()
                }),
        }
    }
}

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

    #[instrument(skip(self, window))]
    fn send_default_prices_with_toast_warning(
        &self,
        err: &Error,
        league: &TradeLeague,
        window: &Window,
    ) -> Prices {
        Event::Toast {
            variant: ToastVariant::Warning,
            message: format!("{err} Unable to load prices for league {league}. Skip price-dependant calculations."),
        }
        .emit(&window);
        Prices::default()
    }

    #[instrument(skip(self))]
    fn read_from_file_update_and_return(&mut self, league: &TradeLeague) -> Result<Prices, Error> {
        let json = std::fs::read_to_string(self.league_path(league))?;
        let prices = serde_json::from_str::<Prices>(&json)?;
        self.prices_by_league
            .insert(league.to_owned(), prices.clone());
        Ok(prices)
    }

    #[instrument(skip(self))]
    pub fn league_path(&self, league: &TradeLeague) -> PathBuf {
        self.dir.join(format!("{}-prices.json", { league }))
    }

    #[instrument(skip(self, window))]
    async fn fetch_and_update(
        &mut self,
        league: &TradeLeague,
        window: &Window,
    ) -> Result<Prices, Error> {
        let prices = Prices::fetch(league).await?;
        debug!("fetch_and_update: fetched. Serializing to json");
        let json = serde_json::to_string(&prices)?;

        debug!("fetch_and_update: Serialized. Next write to file");

        std::fs::write(self.league_path(league), &json).unwrap();

        debug!("fetch_and_update: wrote to file");
        self.prices_by_league
            .insert(league.to_owned(), prices.clone());

        Event::Toast {
            variant: ToastVariant::Neutral,
            message: format!("Prices for {league} league have been updated"),
        }
        .emit(&window);

        Ok(prices)
    }

    #[instrument(skip(self))]
    fn read_from_file(&self, league: &TradeLeague) -> Result<Prices, Error> {
        let json = std::fs::read_to_string(self.league_path(league))?;
        let prices = serde_json::from_str::<Prices>(&json)?;
        Ok(prices)
    }

    #[instrument(skip(self))]
    fn file_is_up_to_date(&self, league: &TradeLeague) -> bool {
        match self.file_days_old(league) {
            Some(days_old) => days_old <= 1.0,
            None => false,
        }
    }

    #[instrument(skip(self))]
    fn file_is_still_usable(&self, league: &TradeLeague) -> bool {
        match self.file_days_old(league) {
            Some(days_old) => days_old <= 7.0,
            None => false,
        }
    }

    #[instrument(skip(self))]
    fn file_days_old(&self, league: &TradeLeague) -> Option<f32> {
        let path = self.league_path(league);
        let exists = path.try_exists().unwrap();
        match exists {
            true => match fs::metadata(&path) {
                Ok(metadata) => match metadata.modified() {
                    Ok(time) => {
                        let days = (time.elapsed().unwrap().as_secs() as f64 / DAY_AS_SECS) as f32;
                        Some(days)
                    }
                    Err(err) => {
                        debug!("{err}");
                        None
                    }
                },
                Err(err) => {
                    debug!("{err}");
                    None
                }
            },
            false => None,
        }
    }

    #[instrument(skip(self))]
    fn league_file_exists(&self, league: &TradeLeague) -> bool {
        self.league_path(league).try_exists().unwrap()
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

// impl AppCardPrices {
//     #[instrument(skip(self, window))]
//     pub async fn get_or_update_or_default(
//         &mut self,
//         league: &TradeLeague,
//         window: &Window,
//     ) -> Prices {
//         match self.prices_by_league.get(league) {
//             Some(prices) => prices.to_owned(),
//             None => {
//                 debug!("No prices for league {league} in memory. Checking if file exists");
//                 match self.league_file_exists(league) {
//                     true => {
//                         debug!("File exists. Check if it is up-to-date");
//                         match self.file_is_up_to_date(league) {
//                             true => {
//                                 match self.read_from_file_update_and_return(league) {
//                                     Ok(prices) => {
//                                         debug!("File is up-to-date. Save to memory and return");
//                                         prices
//                                     }
//                                     Err(err) => {
//                                         debug!("Error reading file. Try to fetch prices");

//                                         // async_default_prices().await
//                                         Prices::default()

//                                         // match fs::remove_file(self.league_path(league)) {
//                                         //     Ok(_) => {
//                                         //         // Event::Toast {
//                                         //         //     variant: ToastVariant::Success,
//                                         //         //     message: format!(
//                                         //         //         "We deleted the prices file {}",
//                                         //         //         self.league_path(league).to_str().unwrap()
//                                         //         //     ),
//                                         //         // }
//                                         //         // .emit(&window);

//                                         //         match Prices::fetch(league).await {
//                                         //             Ok(prices) => prices,
//                                         //             Err(err) => {
//                                         //                 // dbg!(err);
//                                         //                 Prices::default()
//                                         //             }
//                                         //         }

//                                         //         // Prices::default()
//                                         //     }
//                                         //     Err(err) => {
//                                         //         debug!("Unable to fetch prices: {err}. Return default Prices with warning toast");
//                                         //         self.send_default_prices_with_toast_warning(
//                                         //             &Error::IoError(err),
//                                         //             league,
//                                         //             window,
//                                         //         );
//                                         //         Prices::default()
//                                         //     }
//                                         // }

//                                         // debug!("Unable to fetch prices: {err}. Return default Prices with warning toast");
//                                         // self.send_default_prices_with_toast_warning(
//                                         //     &err, league, window,
//                                         // );
//                                         // Prices::default()

//                                         // match self.fetch_and_update(league, window).await {
//                                         //     Ok(prices) => prices,
//                                         //     Err(err) => {
//                                         //         debug!("Unable to fetch prices: {err}. Return default Prices with warning toast");
//                                         //         self.send_default_prices_with_toast_warning(
//                                         //             &err, league, window,
//                                         //         )
//                                         //     }
//                                         // }
//                                     }
//                                 }
//                             }
//                             false => {
//                                 debug!("File is not up-to-date. Try to fetch new prices");
//                                 match self.fetch_and_update(league, window).await {
//                                     Ok(prices) => prices,
//                                     Err(err) => {
//                                         debug!("Unable to fetch prices: {err}. Check if file is still usable");
//                                         match self.file_is_still_usable(league) {
//                                             true => {
//                                                 match self.read_from_file_update_and_return(league)
//                                                 {
//                                                     Ok(prices) => {
//                                                         debug!("File is still usable. Save to memory and return");
//                                                         let days_old = format!(
//                                                             "{:.1}",
//                                                             self.file_days_old(league).unwrap()
//                                                         );
//                                                         let message = format!("Prices are not up-to-date, but still usable({days_old} days old). Unable to load new prices. So still usable is better than nothing.");
//                                                         Event::Toast {
//                                                             variant: ToastVariant::Warning,
//                                                             message,
//                                                         }
//                                                         .emit(window);
//                                                         prices
//                                                     }
//                                                     Err(err) => {
//                                                         debug!("Unable to fetch prices: {err}. Return default Prices with warning toast");
//                                                         self.send_default_prices_with_toast_warning(
//                                                             &err, league, window,
//                                                         )
//                                                     }
//                                                 }
//                                             }
//                                             false => {
//                                                 debug!("File is too old. Return default Prices with warning toast");
//                                                 self.send_default_prices_with_toast_warning(
//                                                     &err, league, window,
//                                                 )
//                                             }
//                                         }
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                     false => {
//                         debug!("File does not exist. Try to fetch new prices");
//                         match self.fetch_and_update(league, window).await {
//                             Ok(prices) => prices,
//                             Err(err) => {
//                                 debug!("Unable to fetch prices: {err}. Return default Prices with warning toast");
//                                 self.send_default_prices_with_toast_warning(&err, league, window)
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }
