use crate::{
    error::Error,
    event::{Event, ToastVariant},
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
    pub async fn get_price(&mut self, league: &TradeLeague, window: &Window) -> Prices {
        if let Some(prices) = self.prices_by_league.get(league) {
            return prices.to_owned();
        }

        match self.read_file(league) {
            LeagueFileState::UpToDate(prices) => prices,
            LeagueFileState::StillUsable(prices, days_old) => self
                .fetch_and_update(league, window)
                .await
                .unwrap_or_else(|_| {
                       let message = format!("Prices are not up-to-date, but still usable({days_old:.1} days old). Unable to load new prices.");
                        Event::Toast {
                            variant: ToastVariant::Warning,
                            message,
                        }
                        .emit(window);
                        prices
                }),
            _ => self
                .fetch_and_update(league, window)
                .await
                .unwrap_or_else(|err| {
                    self.send_default_prices_with_toast_warning(&err, league, window)
                }),
        }
    }

    pub fn read_file(&self, league: &TradeLeague) -> LeagueFileState {
        if !self.league_file_exists(league) {
            return LeagueFileState::NoFile;
        }

        let Ok(prices) = self.read_from_file(league) else {
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
