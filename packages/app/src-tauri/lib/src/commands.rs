use divi::{sample::DivinationCardsSample, CardNameAmount, Prices, SampleData, TradeLeague};
use tauri::{command, AppHandle, Manager};

use crate::{js_result::JSResult, prices};

#[command]
pub async fn sample(csv: String, league: TradeLeague) -> JSResult<DivinationCardsSample> {
    JSResult::from(DivinationCardsSample::create(
        SampleData::CsvString(csv),
        prices::prices(&league).await,
    ))
}

#[command]
pub async fn sample_cards(
    cards: Vec<CardNameAmount>,
    league: TradeLeague,
) -> JSResult<DivinationCardsSample> {
    JSResult::from(DivinationCardsSample::create(
        SampleData::CardNameAmountList(cards),
        prices::prices(&league).await,
    ))
}

#[command]
pub async fn chaos(sample: Box<DivinationCardsSample>, min: Option<f32>) -> f32 {
    sample.as_ref().chaos(min)
}

#[command]
pub async fn merge(
    samples: Vec<DivinationCardsSample>,
    app_handle: AppHandle,
) -> DivinationCardsSample {
    DivinationCardsSample::merge(app_handle.state::<Prices>().inner().clone(), &samples)
}

#[command]
pub async fn league(
    sample: Box<DivinationCardsSample>,
    league: TradeLeague,
) -> JSResult<DivinationCardsSample> {
    JSResult::from(DivinationCardsSample::create(
        SampleData::CsvString(sample.csv),
        prices::prices(&league).await,
    ))
}
