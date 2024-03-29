use tokio::sync::Mutex;

use divi::{
    league::TradeLeague,
    sample::{DivinationCardsSample, SampleData, TablePreferences},
};
use tauri::{command, State, Window};
use tracing::instrument;

use crate::{error::Error, prices::AppCardPrices, version::AppVersion};

#[command]
#[instrument(skip(data, state, window))]
pub async fn sample(
    data: SampleData,
    league: Option<TradeLeague>,
    state: State<'_, Mutex<AppCardPrices>>,
    window: Window,
) -> Result<DivinationCardsSample, Error> {
    let prices = match league {
        Some(league) => {
            let mut guard = state.lock().await;
            Some(guard.get_price(&league, &window).await)
        }
        None => None,
    };

    let sample = DivinationCardsSample::create(data, prices)?;
    Ok(sample)
}

#[command]
pub async fn merge(
    samples: Vec<DivinationCardsSample>,
    state: State<'_, Mutex<AppCardPrices>>,
    window: Window,
) -> Result<DivinationCardsSample, Error> {
    let mut guard = state.lock().await;
    let prices = guard.get_price(&TradeLeague::default(), &window).await;
    Ok(DivinationCardsSample::merge(Some(prices), &samples))
}

#[command]
pub async fn open_url(url: String) {
    open::that(url).unwrap();
}

#[command]
pub async fn sample_into_csv(
    sample: DivinationCardsSample,
    preferences: TablePreferences,
) -> String {
    sample.into_csv(Some(preferences))
}

#[command]
pub async fn version(version: State<'_, AppVersion>) -> Result<AppVersion, Error> {
    Ok(version.inner().clone())
}
