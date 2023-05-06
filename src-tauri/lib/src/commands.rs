use crate::prices::prices;
use divi::{Csv, CsvString, DivinationCardsSample, League};

#[tauri::command]
pub async fn sample(csv: String) -> DivinationCardsSample {
    DivinationCardsSample::create(
        Csv::CsvString(CsvString(csv)),
        prices(League::Crucible).await,
    )
}

#[tauri::command]
pub async fn chaos(csv: String, min: Option<f32>) -> f32 {
    DivinationCardsSample::create(
        Csv::CsvString(CsvString(csv)),
        prices(League::Crucible).await,
    )
    .chaos(min)
}

#[tauri::command]
pub async fn merge(samples: Vec<DivinationCardsSample>) -> DivinationCardsSample {
    let prices = prices(League::Crucible).await;
    DivinationCardsSample::merge(prices, &samples)
}
