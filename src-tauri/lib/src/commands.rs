use divi::{Csv, CsvString, DivinationCardsSample, Prices};
use tauri::{Manager, State};

#[tauri::command]
pub fn sample(csv: String, prices: State<'_, Prices>) -> DivinationCardsSample {
    DivinationCardsSample::create(Csv::CsvString(CsvString(csv)), prices.inner().clone())
}

#[tauri::command]
pub async fn chaos(sample: Box<DivinationCardsSample>, min: Option<f32>) -> f32 {
    sample.as_ref().chaos(min)
}

#[tauri::command]
pub async fn merge(
    samples: Vec<DivinationCardsSample>,
    app_handle: tauri::AppHandle,
) -> DivinationCardsSample {
    DivinationCardsSample::merge(app_handle.state::<Prices>().inner().clone(), &samples)
}
