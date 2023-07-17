use divi::{CardNameAmount, CsvString, DivinationCardsSample, Prices, SampleData, TradeLeague};
use tauri::{command, AppHandle, Manager};

use crate::prices;

#[command]
pub async fn sample(
    csv: String,
    league: TradeLeague,
) -> Result<DivinationCardsSample, divi::error::MissingHeaders> {
    DivinationCardsSample::create(
        SampleData::CsvString(CsvString(csv)),
        prices::prices(&league).await,
    )
}

#[command]
pub async fn sample_cards(
    cards: Vec<CardNameAmount>,
    league: TradeLeague,
) -> Result<DivinationCardsSample, divi::error::MissingHeaders> {
    DivinationCardsSample::create(
        SampleData::CardNameAmountList(cards),
        prices::prices(&league).await,
    )
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
) -> Result<DivinationCardsSample, divi::error::MissingHeaders> {
    DivinationCardsSample::create(
        SampleData::CsvString(sample.polished),
        prices::prices(&league).await,
    )
}
