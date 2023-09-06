use divi::sample::{Column, DivinationCardsSample};
use googlesheets::Values;
use reqwest::Client;
use serde_json::{json, Value};

use crate::{
    error::Error,
    google::{AccessTokenStorage, Persist},
};

#[tauri::command]
pub async fn read_sheet() -> Result<Value, Error> {
    let client = Client::new();
    let response = client
        .get("https://sheets.googleapis.com/v4/spreadsheets/1RBkCNHCclRxGHZxKWi_UCWbDgdNnpnJ60g2rdL_msG0/values/Sheet1!A1:D5")
        .header(
            "Authorization",
            format!("Bearer {}", { AccessTokenStorage::new().get().unwrap() }),
        )
        .send()
        .await?;
    let value: Value = response.json().await?;

    Ok(value)
}

pub async fn write_sheet(
    spreadsheet_id: String,
    title: String,
    values: Values,
) -> Result<Value, Error> {
    let url = format!("https://sheets.googleapis.com/v4/spreadsheets/{spreadsheet_id}/values/{title}?valueInputOption=RAW
");

    let b = json!({
      "range": title,
      "majorDimension": "ROWS",
      "values": values
    });

    let body = serde_json::to_string(&b).unwrap();

    let response = Client::new()
        .put(url)
        .header(
            "Authorization",
            format!("Bearer {}", { AccessTokenStorage::new().get().unwrap() }),
        )
        .body(body)
        .send()
        .await?;

    let value: Value = response.json().await?;

    Ok(value)
}

#[tauri::command]
pub async fn write_sample(
    spreadsheet_id: String,
    title: String,
    sample: DivinationCardsSample,
) -> Result<Value, Error> {
    let val = write_sheet(
        spreadsheet_id,
        title,
        sample.into_values(&[Column::Name, Column::Amount, Column::Price, Column::Weight]),
    )
    .await?;
    Ok(val)
}

pub async fn create_sheet(spreadsheet_id: String, title: String) -> Result<Value, Error> {
    let value = googlesheets::create_sheet(
        spreadsheet_id,
        title,
        AccessTokenStorage::new().get().unwrap(),
    )
    .await?;
    Ok(value)
}

pub async fn sheet_with_sample(
    spreadsheet_id: String,
    title: String,
    sample: DivinationCardsSample,
) -> Result<Value, Error> {
    create_sheet(spreadsheet_id.clone(), title.clone()).await?;
    let value = write_sheet(
        spreadsheet_id,
        title,
        sample.into_values(&[Column::Name, Column::Amount, Column::Price]),
    )
    .await?;

    Ok(value)
}

#[tauri::command]
pub async fn create_sheet_with_sample(
    spreadsheet_id: String,
    title: String,
    sample: DivinationCardsSample,
) -> Result<Value, Error> {
    create_sheet(spreadsheet_id.clone(), title.clone()).await?;
    let value = write_sheet(
        spreadsheet_id,
        title,
        sample.into_values(&[Column::Name, Column::Amount, Column::Price]),
    )
    .await?;

    Ok(value)
}

#[tauri::command]
pub fn sample_into_values(sample: DivinationCardsSample) -> Values {
    sample.into_values(&[Column::Name, Column::Amount])
}
