use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Values(pub Vec<Vec<Value>>);
impl Values {
    pub const fn new(v: Vec<Vec<Value>>) -> Self {
        Values(v)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct A1Range(String);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Dimension {
    Rows,
    Columns,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RangeRequestBody {
    pub range: A1Range,
    pub major_dimension: Dimension,
    pub values: Values,
}

pub async fn create_sheet(
    spreadsheet_id: String,
    title: String,
    access_token: String,
) -> Result<Value, reqwest::Error> {
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
        .header("Authorization", format!("Bearer {}", { access_token }))
        .body(body)
        .send()
        .await?;
    let value: Value = response.json().await?;

    Ok(value)
}
