use std::fmt::{Debug, Display};

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::error::{Error, GoogleErrorResponse};

#[tracing::instrument(skip(values, token))]
pub async fn add_sheet_with_values<T: Serialize + Debug>(
    spreadsheet_id: &str,
    title: &str,
    values: Vec<Vec<T>>,
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

pub async fn write_values_into_sheet<T>(
    spreadsheet_id: &str,
    title: &str,
    token: &str,
    values: Vec<Vec<T>>,
) -> Result<WriteValuesResponse, Error>
where
    T: Serialize + Debug,
{
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
    // let spredsheet_id = "1sDXpbG2bkqrOYScnvjMXTTg718dEc0LMDVHzllbAgmM";
    let body = serde_json::to_string(&serde_json::json!({
       "requests":[
          {
             "addSheet":{
                "properties":{
                   "title": title
                }
             }

          }
       ]
    }))
    .unwrap();
    let url = format!("https://sheets.googleapis.com/v4/spreadsheets/{spreadsheet_id}:batchUpdate");
    let response = Client::new()
        .post(url)
        .header("Authorization", format!("Bearer {}", { token }))
        .body(body)
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

    // #[tokio::test]
    // async fn test() {
    //     let token = "";
    //     let spreadsheet_id = "";
    //     let title = "test";
    //     let values: Values<&str> = Values(vec![vec!["name", "amount"], vec!["The Doctor", "2"]]);

    //     let _add_sheet_response = add_sheet(spreadsheet_id, title, token).await.unwrap();
    //     // let id = add_sheet_response.properties.sheet_id;

    //     let r = write_values_into_sheet(spreadsheet_id, title, token, values)
    //         .await
    //         .unwrap();
    //     dbg!(r);
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

// #[command]
// pub async fn create_spreadsheet() -> Result<Value, Error> {
//     let token = AccessTokenStorage::new().get().unwrap();
//     Ok(api_create_spreadsheet(token).await?)
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct A1Range(String);

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub enum Dimension {
//     Rows,
//     Columns,
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct RangeRequestBody {
//     pub range: A1Range,
//     pub major_dimension: Dimension,
//     pub values: Values,
// }
