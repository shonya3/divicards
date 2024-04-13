use chrono::Utc;
use divi::{sample::DivinationCardsSample, League};
use googlesheets::sheet::{Credential, Dimension, ReadBatchResponse, SheetUrl, ValueRange};
use serde_json::json;
use tracing::debug;

use crate::{
    error::Error,
    google::{AccessTokenStorage, Persist},
};

#[tauri::command]
#[tracing::instrument(skip(sample))]
pub async fn new_sheet_with_sample(
    spreadsheet_id: &str,
    title: &str,
    sample: DivinationCardsSample,
    league: League,
    preferences: Option<divi::sample::TablePreferences>,
) -> Result<SheetUrl, Error> {
    let token = AccessTokenStorage::new().get().unwrap();
    let add_sheet_response = googlesheets::add_sheet(spreadsheet_id, title, &token).await?;

    let sample_values = ValueRange {
        dimension: Dimension::Rows,
        range: title.to_string(),
        values: sample.into_serde_values(preferences),
    };

    let aside_values = ValueRange {
        dimension: Dimension::Rows,
        range: format!("{title}!H5"),
        values: vec![vec![json!(format!(
            "{} {league} League",
            Utc::now().date_naive().format("%-d %b, %C%y")
        ))]],
    };

    let batch_response =
        googlesheets::batch_update(spreadsheet_id, vec![sample_values, aside_values], &token)
            .await?;

    debug!("{batch_response}");

    Ok(SheetUrl::create(
        spreadsheet_id,
        add_sheet_response.properties.sheet_id,
    ))
}

#[tauri::command]
#[tracing::instrument]
pub async fn read_batch(
    spreadsheet_id: &str,
    ranges: Vec<&str>,
) -> Result<ReadBatchResponse, Error> {
    let value = googlesheets::read_batch(
        spreadsheet_id,
        &ranges,
        Credential::AccessToken(AccessTokenStorage::new().get().unwrap()),
    )
    .await?;

    Ok(value)
}

#[tauri::command]
#[tracing::instrument]
pub async fn read_sheet(spreadsheet_id: &str, range: &str) -> Result<ValueRange, Error> {
    let value_range = googlesheets::read(
        spreadsheet_id,
        range,
        Credential::AccessToken(AccessTokenStorage::new().get().unwrap()),
    )
    .await?;

    Ok(value_range)
}
