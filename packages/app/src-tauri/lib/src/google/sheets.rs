use divi::sample::DivinationCardsSample;
use googlesheets::sheet::SheetUrl;

use crate::{
    error::Error,
    google::{AccessTokenStorage, Persist},
};

#[tauri::command]
#[tracing::instrument(skip(sample))]
pub async fn add_sheet_with_sample(
    spreadsheet_id: &str,
    title: &str,
    sample: DivinationCardsSample,
    preferences: Option<divi::sample::TablePreferences>,
) -> Result<SheetUrl, Error> {
    let url = googlesheets::add_sheet_with_values(
        spreadsheet_id,
        title,
        sample.into_values(preferences),
        &AccessTokenStorage::new().get().unwrap(),
    )
    .await?;
    Ok(url)
}
