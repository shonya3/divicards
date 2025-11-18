use crate::{
    error::Error,
    event::{Event, ToastVariant},
};
use divi::{prices::Prices, Error as DiviError, TradeLeague};
use ninja::fetch_by_item_category;
use serde_json::Value;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf};
use tauri::Window;
use tracing::{debug, instrument, info};

pub const MINUTE_AS_SECS: f64 = 60.0;
const UP_TO_DATE_THRESHOLD_MINUTES: f32 = 20.0;
const STILL_USABLE_THRESHOLD_MINUTES: f32 = 20.0;

pub enum LeagueFileState {
    UpToDate(Prices),
    StillUsable(Prices, f32),
    TooOld,
    Invalid,
    NoFile,
}

impl AppCardPrices {
    #[instrument(skip(self, window))]
    pub async fn get_price(&mut self, league: &TradeLeague, window: &Window) -> Prices {
        if let Some(prices) = self.prices_by_league.get(league) {
            return prices.to_owned();
        }

        match self.read_file(league) {
            LeagueFileState::UpToDate(prices) => prices,
            LeagueFileState::StillUsable(prices, minutes_old) => self
                .fetch_and_update(league)
                .await
                .unwrap_or_else(|_| {
                       let message = format!("Prices are not up-to-date, but still usable ({minutes_old:.0} minutes old). Unable to load new prices.");
                        Event::Toast {
                            variant: ToastVariant::Warning,
                            message,
                        }
                        .emit(window);
                        prices
                }),
            _ => self
                .fetch_and_update(league)
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

        if let Some(minutes_old) = self.file_minutes_old(league) {
            match minutes_old {
                n if n <= UP_TO_DATE_THRESHOLD_MINUTES => LeagueFileState::UpToDate(prices),
                n if n <= STILL_USABLE_THRESHOLD_MINUTES => LeagueFileState::StillUsable(prices, n),
                _ => LeagueFileState::TooOld,
            }
        } else {
            LeagueFileState::NoFile
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapPrice {
    pub name: String,
    pub tier: u8,
    pub chaos_value: Option<f32>,
}

#[tauri::command]
#[instrument]
pub async fn map_prices(league: TradeLeague) -> Result<Vec<MapPrice>, Error> {
    let lines = fetch_by_item_category("Map", &league).await.map_err(DiviError::NinjaError)?;
    let mut out: Vec<MapPrice> = Vec::with_capacity(lines.len());
    for v in lines.into_iter() {
        let name = v.get("name").and_then(Value::as_str).unwrap_or("").to_string();
        let tier = v
            .get("mapTier")
            .and_then(Value::as_u64)
            .map(|n| n as u8)
            .unwrap_or(0);
        let chaos_value = v.get("chaosValue").and_then(Value::as_f64).map(|n| n as f32);
        if !name.is_empty() {
            out.push(MapPrice { name, tier, chaos_value });
        }
    }
    info!(league = %league, count = out.len(), "map_prices fetched");
    Ok(out)
}
impl AppCardPrices {
    pub fn new(dir: PathBuf) -> Result<Self, Error> {
        if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }
        Ok(AppCardPrices {
            dir,
            prices_by_league: HashMap::new(),
        })
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
        .emit(window);
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

    #[instrument(skip(self))]
    async fn fetch_and_update(&mut self, league: &TradeLeague) -> Result<Prices, Error> {
        let prices = Prices::fetch(league).await.map_err(DiviError::NinjaError)?;
        debug!("fetch_and_update: fetched. Serializing to json");
        let json = serde_json::to_string(&prices)?;

        debug!("fetch_and_update: Serialized. Next write to file");

        std::fs::write(self.league_path(league), json)?;

        debug!("fetch_and_update: wrote to file");
        self.prices_by_league
            .insert(league.to_owned(), prices.clone());

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
        match self.file_minutes_old(league) {
            Some(minutes_old) => minutes_old <= UP_TO_DATE_THRESHOLD_MINUTES,
            None => false,
        }
    }

    #[instrument(skip(self))]
    fn file_is_still_usable(&self, league: &TradeLeague) -> bool {
        match self.file_minutes_old(league) {
            Some(minutes_old) => minutes_old <= STILL_USABLE_THRESHOLD_MINUTES,
            None => false,
        }
    }

    #[instrument(skip(self))]
    fn file_minutes_old(&self, league: &TradeLeague) -> Option<f32> {
        let path = self.league_path(league);
        match fs::metadata(&path) {
            Ok(metadata) => match metadata.modified() {
                Ok(modified_time) => match modified_time.elapsed() {
                    Ok(duration) => Some((duration.as_secs_f64() / MINUTE_AS_SECS) as f32),
                    Err(_e) => {
                        // SystemTimeError: modified time is later than current time.
                        debug!(
                            "File {:?} modification time is in the future. Treating as needing update.",
                            path
                        );
                        None
                    }
                },
                Err(e) => {
                    debug!("Failed to read modification time for {:?}: {}", path, e);
                    None
                }
            },
            Err(e) => {
                debug!("Failed to read metadata for {:?}: {}", path, e);
                None
            }
        }
    }

    #[instrument(skip(self))]
    fn league_file_exists(&self, league: &TradeLeague) -> bool {
        self.league_path(league).try_exists().unwrap_or(false)
    }
}
