pub mod commands;

use reqwest::Client;
use serde_json::Value;
use tauri::command;

use crate::{
    error::Error,
    google::{AccessTokenStorage, Persist},
};

pub async fn api_create_spreadsheet(access_token: String) -> Result<Value, Error> {
    let client = Client::new();
    let url = "https://sheets.googleapis.com/v4/spreadsheets";
    let response = client
        .post(url)
        .header(
            "Authorization",
            format!("Bearer {}", { access_token.clone() }),
        )
        .body("{}")
        .send()
        .await?;
    // dbg!(response);
    let value: Value = response.json().await?;
    Ok(value)
}

#[command]
pub async fn create_spreadsheet() -> Result<Value, Error> {
    let token = AccessTokenStorage::new().get().unwrap();
    Ok(api_create_spreadsheet(token).await?)
}

// struct Error {}

// {
// "error": {
//  "code": 400,
//   "message": "Invalid requests[0].addSheet: A sheet with the name \"Sheet\" already exists. Please enter another name.",
//    "status": "INVALID_ARGUMENT"
//  }
//}
