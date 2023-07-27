use tokio::sync::Mutex;

use divi::{
    league::TradeLeague,
    prices::Prices,
    sample::{CardNameAmount, DivinationCardsSample, SampleData},
};
use tauri::{command, AppHandle, Manager, State};

use crate::{
    js_result::JSResult,
    prices::{self, AppCardPrices},
};

#[command]
pub async fn sample(
    csv: String,
    league: TradeLeague,
    state: State<'_, Mutex<AppCardPrices>>,
) -> Result<JSResult<DivinationCardsSample>, ()> {
    let mut guard = state.lock().await;
    let prices = guard.get_or_update(&league).await;
    Ok(JSResult::from(DivinationCardsSample::create(
        SampleData::CsvString(csv),
        Some(prices),
    )))
}

#[command]
pub async fn sample_cards(
    cards: Vec<CardNameAmount>,
    league: TradeLeague,
    state: State<'_, Mutex<AppCardPrices>>,
) -> Result<JSResult<DivinationCardsSample>, ()> {
    let mut guard = state.lock().await;
    let prices = guard.get_or_update(&league).await;
    Ok(JSResult::from(DivinationCardsSample::create(
        SampleData::CardNameAmountList(cards),
        Some(prices),
    )))
}

#[command]
pub async fn merge(
    samples: Vec<DivinationCardsSample>,
    app_handle: AppHandle,
) -> DivinationCardsSample {
    DivinationCardsSample::merge(Some(app_handle.state::<Prices>().inner().clone()), &samples)
}

#[command]
pub async fn league(
    sample: Box<DivinationCardsSample>,
    league: TradeLeague,
    state: State<'_, Mutex<AppCardPrices>>,
) -> Result<JSResult<DivinationCardsSample>, ()> {
    let mut guard = state.lock().await;
    let prices = guard.get_or_update(&league).await;
    Ok(JSResult::from(DivinationCardsSample::create(
        SampleData::CsvString(sample.csv),
        Some(prices),
    )))
}
