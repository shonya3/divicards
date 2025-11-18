use crate::{
    error::Error,
    google::{AccessTokenState, AccessTokenStorage, Persist},
};
use chrono::Utc;
use divi::{sample::Sample, League};
use googlesheets::sheet::{Credential, Dimension, ReadBatchResponse, SheetUrl, ValueRange};
use serde_json::json;
use tracing::debug;
use tauri::State;
use reqwest::Client;

#[tauri::command]
#[tracing::instrument(skip(sample))]
pub async fn new_sheet_with_sample(
    spreadsheet_id: &str,
    title: &str,
    sample: Sample,
    league: League,
    preferences: Option<divi::sample::TablePreferences>,
    token_state: State<'_, AccessTokenState>,
) -> Result<SheetUrl, Error> {
    let token = match token_state.0.lock().await.clone() {
        Some(t) => t,
        None => AccessTokenStorage::new().get().unwrap(),
    };
    let sheet_gid: String = match googlesheets::add_sheet(spreadsheet_id, title, &token).await {
        Ok(add) => add.properties.sheet_id.to_string(),
        Err(_) => get_sheet_gid_by_title(spreadsheet_id, title, &token)
            .await?
            ,
    };

    let sample_values = ValueRange {
        dimension: Dimension::Rows,
        range: format!("{title}!A1"),
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

    let url = format!(
        "https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit#gid={sheet_gid}"
    );
    let sheet_url: SheetUrl = serde_json::from_value(json!(url))?;
    Ok(sheet_url)
}

#[derive(serde::Deserialize)]
struct SheetsListResponse {
    sheets: Vec<SheetEntry>,
}

#[derive(serde::Deserialize)]
struct SheetEntry {
    properties: SheetProps,
}

#[derive(serde::Deserialize)]
struct SheetProps {
    title: String,
    #[serde(rename = "sheetId")]
    sheet_id: u32,
}

async fn get_sheet_gid_by_title(
    spreadsheet_id: &str,
    title: &str,
    token: &str,
) -> Result<String, Error> {
    let url = format!(
        "https://sheets.googleapis.com/v4/spreadsheets/{spreadsheet_id}?fields=sheets(properties(title,sheetId))"
    );
    let resp = Client::new()
        .get(url)
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await?;
    if resp.status().as_u16() >= 400 {
        let err: googlesheets::error::GoogleErrorResponse = resp.json().await?;
        return Err(googlesheets::error::Error::GoogleError(err.error).into());
    }
    let data: SheetsListResponse = resp.json().await?;
    let id = data
        .sheets
        .into_iter()
        .find(|s| s.properties.title == title)
        .map(|s| s.properties.sheet_id.to_string())
        .ok_or_else(|| googlesheets::error::GoogleError {
            code: 404,
            message: "Sheet not found after creation".to_string(),
            status: "NOT_FOUND".to_string(),
        })
        .map_err(|e| googlesheets::error::Error::GoogleError(e))?;
    Ok(id)
}

#[tauri::command]
#[tracing::instrument]
pub async fn read_batch(
    spreadsheet_id: &str,
    ranges: Vec<&str>,
    token_state: State<'_, AccessTokenState>,
) -> Result<ReadBatchResponse, Error> {
    let token = match token_state.0.lock().await.clone() {
        Some(t) => t,
        None => AccessTokenStorage::new().get().unwrap(),
    };
    let value = googlesheets::read_batch(
        spreadsheet_id,
        &ranges,
        Credential::AccessToken(token),
    )
    .await?;

    Ok(value)
}

#[tauri::command]
#[tracing::instrument]
pub async fn read_sheet(
    spreadsheet_id: &str,
    range: &str,
    token_state: State<'_, AccessTokenState>,
) -> Result<ValueRange, Error> {
    let token = match token_state.0.lock().await.clone() {
        Some(t) => t,
        None => AccessTokenStorage::new().get().unwrap(),
    };
    let value_range = googlesheets::read(
        spreadsheet_id,
        range,
        Credential::AccessToken(token),
    )
    .await?;

    Ok(value_range)
}
