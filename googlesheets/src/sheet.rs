use std::fmt::{Debug, Display};

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tracing::debug;

use crate::error::{Error, GoogleErrorResponse};

pub async fn read_batch(
    spreadsheet_id: &str,
    ranges: &[&str],
    token: &str,
) -> Result<Value, Error> {
    let formatted_ranges = ranges
        .iter()
        .map(|range| format!("ranges={range}"))
        .collect::<Vec<String>>()
        .join("&");

    let url =
        format!("https://sheets.googleapis.com/v4/spreadsheets/{spreadsheet_id}/values:batchGet?{formatted_ranges}");
    debug!(url);
    let response = Client::new()
        .get(url)
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await?;

    if response.status().as_u16() >= 400 {
        let err_response: GoogleErrorResponse = response.json().await?;
        Err(err_response.error.into())
    } else {
        let value: Value = response.json().await?;
        Ok(value)
    }
}

pub async fn read(spreadsheet_id: &str, range: &str, token: &str) -> Result<ValueRange, Error> {
    let url =
        format!("https://sheets.googleapis.com/v4/spreadsheets/{spreadsheet_id}/values/{range}");

    dbg!(&url);
    let response = Client::new()
        .get(url)
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await?;

    if response.status().as_u16() >= 400 {
        let err_response: GoogleErrorResponse = response.json().await?;
        Err(err_response.error.into())
    } else {
        let value_range: ValueRange = response.json().await?;
        Ok(value_range)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Dimension {
    #[serde(rename = "ROWS")]
    Rows,
    #[serde(rename = "COLUMNS")]
    Columns,
}

impl Default for Dimension {
    fn default() -> Self {
        Dimension::Rows
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueRange {
    #[serde(rename = "majorDimension")]
    pub dimension: Dimension,
    pub range: String,
    pub values: Vec<Vec<Value>>,
}

impl ValueRange {
    pub const fn new(dimension: Dimension, range: String, values: Vec<Vec<Value>>) -> ValueRange {
        ValueRange {
            dimension,
            range,
            values,
        }
    }
}

#[tracing::instrument(skip(data, token))]
pub async fn batch_update(
    spreadsheet_id: &str,
    data: Vec<ValueRange>,
    token: &str,
) -> Result<Value, Error> {
    let response = Client::new()
        .post(format!(
            "https://sheets.googleapis.com/v4/spreadsheets/{spreadsheet_id}/values:batchUpdate
"
        ))
        .header("Authorization", format!("Bearer {token}"))
        .body(
            json!({
                "valueInputOption": "RAW",
                "data": data
            })
            .to_string(),
        )
        .send()
        .await?;

    if response.status().as_u16() >= 400 {
        let err_response: GoogleErrorResponse = response.json().await?;
        Err(err_response.error.into())
    } else {
        let write_values: Value = response.json().await?;
        Ok(write_values)
    }
}

#[tracing::instrument(skip(values))]
pub async fn add_sheet_with_values<T: Serialize + Debug>(
    spreadsheet_id: &str,
    title: &str,
    values: Vec<Vec<Value>>,
    token: &str,
) -> Result<SheetUrl, Error> {
    let add_sheet_data = add_sheet(spreadsheet_id, title, &token).await?;
    let _ = write_values_into_sheet(spreadsheet_id, title, &token, values).await?;

    Ok(SheetUrl::create(
        spreadsheet_id,
        add_sheet_data.properties.sheet_id,
    ))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WriteValuesResponse {
    spreadsheet_id: String,
    updated_cells: Option<u32>,
    updated_columns: Option<u32>,
    updated_range: Option<String>,
    updated_rows: Option<u32>,
}

pub async fn write_values_into_sheet(
    spreadsheet_id: &str,
    title: &str,
    token: &str,
    values: Vec<Vec<Value>>,
) -> Result<WriteValuesResponse, Error> {
    let url = format!("https://sheets.googleapis.com/v4/spreadsheets/{spreadsheet_id}/values/{title}?valueInputOption=RAW
");

    let body = serde_json::to_string(&json!({
      "range": title,
      "majorDimension": "ROWS",
      "values": values
    }))?;

    let response = Client::new()
        .put(url)
        .header("Authorization", format!("Bearer {token}"))
        .body(body)
        .send()
        .await?;

    if response.status().as_u16() >= 400 {
        let err_response: GoogleErrorResponse = response.json().await?;
        Err(err_response.error.into())
    } else {
        let write_values: WriteValuesResponse = response.json().await?;
        Ok(write_values)
    }
}

pub async fn add_sheet(spreadsheet_id: &str, title: &str, token: &str) -> Result<AddSheet, Error> {
    let response = Client::new()
        .post(format!(
            "https://sheets.googleapis.com/v4/spreadsheets/{spreadsheet_id}:batchUpdate"
        ))
        .header("Authorization", format!("Bearer {}", { token }))
        .body(
            json!({
               "requests":[
                  {
                     "addSheet":{
                        "properties":{
                           "title": title
                        }
                     }
                  }
               ]
            })
            .to_string(),
        )
        .send()
        .await?;

    if response.status().as_u16() >= 400 {
        let err_response: GoogleErrorResponse = response.json().await?;
        Err(err_response.error.into())
    } else {
        let response: BatchResponse = response.json().await?;
        let add_sheet = response.replies[0].add_sheet.clone();

        Ok(add_sheet)
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    //     #[tokio::test]
    //     async fn batch_update_test() {
    //         let token = "TOKEN";
    //         let spreadsheet_id = "1sDXpbG2bkqrOYScnvjMXTTg718dEc0LMDVHzllbAgmM";

    //         let url = format!(
    //             "https://sheets.googleapis.com/v4/spreadsheets/{spreadsheet_id}/values:batchUpdate
    // "
    //         );

    //         let json = json!({
    //           "valueInputOption": "USER_ENTERED",
    //           "data": [
    //             {
    //               "range": "science!A1",
    //               "majorDimension": "ROWS",
    //               "values": [
    //                 [
    //                   "name",
    //                   "amount"
    //                 ],
    //                 ["Rain of Chaos", 1],
    //                 ["The Doctor", 1]
    //               ]
    //             },
    //             {
    //                 "range": "science!H5",
    //                 "majorDimension": "ROWS",
    //                 "values": [
    //                     ["league", "Ancestor"],
    //                     ["date", "11 Sep, 2023"],
    //                     ["total cards", 40000],
    //                     ["total price", 200000],
    //                 ]
    //             }
    //           ]
    //         }
    //         );
    //         let body = serde_json::to_string(&json).unwrap();

    //         let response = Client::new()
    //             .post(url)
    //             .header("Authorization", format!("Bearer {token}"))
    //             .body(body)
    //             .send()
    //             .await
    //             .unwrap();

    //         if response.status().as_u16() >= 400 {
    //             let err_response: GoogleErrorResponse = response.json().await.unwrap();
    //             dbg!(err_response.error);
    //         } else {
    //             let write_values: Value = response.json().await.unwrap();
    //             dbg!(write_values);
    //         }
    // }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BatchResponse {
    pub spreadsheet_id: String,
    pub replies: Vec<BatchReply>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BatchReply {
    pub add_sheet: AddSheet,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AddSheet {
    pub properties: AddSheetProperties,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AddSheetProperties {
    pub sheet_id: SheetId,
    pub title: String,
    pub index: u32,
    pub sheet_type: String,
    pub grid_properties: GridProperties,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Hash, PartialEq, PartialOrd)]
pub struct SheetId(u32);
impl Display for SheetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Hash, PartialEq, PartialOrd)]
pub struct SheetUrl(String);
impl SheetUrl {
    pub fn create(spreadsheet_id: &str, sheet_id: SheetId) -> Self {
        SheetUrl(format!(
            "https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit#gid={sheet_id}"
        ))
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GridProperties {
    pub column_count: u32,
    pub row_count: u32,
}

// #[tauri::command]
// pub async fn read_sheet() -> Result<Value, Error> {
//     let client = Client::new();
//     let response = client
//         .get("https://sheets.googleapis.com/v4/spreadsheets/1RBkCNHCclRxGHZxKWi_UCWbDgdNnpnJ60g2rdL_msG0/values/Sheet1!A1:D5")
//         .header(
//             "Authorization",
//             format!("Bearer {}", { AccessTokenStorage::new().get().unwrap() }),
//         )
//         .send()
//         .await?;
//     let value: Value = response.json().await?;

//     Ok(value)
// }

// pub async fn api_create_spreadsheet(access_token: String) -> Result<Value, Error> {
//     let client = Client::new();
//     let url = "https://sheets.googleapis.com/v4/spreadsheets";
//     let response = client
//         .post(url)
//         .header(
//             "Authorization",
//             format!("Bearer {}", { access_token.clone() }),
//         )
//         .body("{}")
//         .send()
//         .await?;
//     // dbg!(response);
//     let value: Value = response.json().await?;
//     Ok(value)
// }

#[allow(dead_code)]
const SPREADSHEET_ID: &'static str = "1sDXpbG2bkqrOYScnvjMXTTg718dEc0LMDVHzllbAgmM";
