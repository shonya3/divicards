use tokio::sync::Mutex;

use divi::{
    league::TradeLeague,
    sample::{DivinationCardsSample, SampleData},
};
use tauri::{command, State};

use crate::{js_result::JSResult, prices::AppCardPrices};

#[command]
pub async fn sample(
    data: SampleData,
    league: Option<TradeLeague>,
    state: State<'_, Mutex<AppCardPrices>>,
) -> Result<JSResult<DivinationCardsSample>, ()> {
    let prices = match league {
        Some(league) => {
            let mut guard = state.lock().await;
            Some(guard.get_or_update(&league).await)
        }
        None => None,
    };

    Ok(JSResult::from(DivinationCardsSample::create(data, prices)))
}

#[command]
pub async fn merge(
    samples: Vec<DivinationCardsSample>,
    state: State<'_, Mutex<AppCardPrices>>,
) -> Result<DivinationCardsSample, ()> {
    let mut guard = state.lock().await;
    let prices = guard.get_or_update(&TradeLeague::default()).await;
    Ok(DivinationCardsSample::merge(Some(prices), &samples))
}

#[command]
pub async fn open_url(url: String) {
    open::that(url).unwrap();
}
