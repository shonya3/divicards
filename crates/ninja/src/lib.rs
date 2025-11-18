pub mod card;
pub mod error;

pub use crate::{
    card::{fetch_card_data, CardData},
    error::Error,
};
pub use poe::TradeLeague;

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub async fn fetch_by_item_category(
    item_category: &str,
    league: &TradeLeague,
) -> Result<Vec<Value>, Error> {
    #[derive(Deserialize, Debug, Serialize)]
    struct ResponseShape {
        lines: Vec<Value>,
    }

    let url = format!(
        "https://poe.ninja/api/data/itemoverview?league={league}&type={item_category}&language=en"
    );
    let json = reqwest::get(url).await?.text().await?;
    let data = serde_json::from_str::<ResponseShape>(&json)?;
    if data.lines.is_empty() {
        return Err(Error::NoItemsBadRequest);
    }
    Ok(data.lines)
}

pub async fn fetch_currency_by_category(
    currency_category: &str,
    league: &TradeLeague,
) -> Result<Vec<Value>, Error> {
    #[derive(Deserialize, Debug, Serialize)]
    struct ResponseShape {
        lines: Vec<Value>,
    }

    let url = format!(
        "https://poe.ninja/api/data/currencyoverview?league={league}&type={currency_category}&language=en"
    );
    let json = reqwest::get(url).await?.text().await?;
    let data = serde_json::from_str::<ResponseShape>(&json)?;
    if data.lines.is_empty() {
        return Err(Error::NoItemsBadRequest);
    }
    Ok(data.lines)
}

pub async fn fetch_stash_currency_overview(
    currency_category: &str,
    league: &TradeLeague,
) -> Result<Vec<Value>, Error> {
    #[derive(Deserialize, Debug, Serialize)]
    struct ResponseShape {
        lines: Vec<Value>,
    }

    let url = format!(
        "https://poe.ninja/poe1/api/economy/stash/current/currency/overview?league={league}&type={currency_category}"
    );
    let json = reqwest::get(url).await?.text().await?;
    let data = serde_json::from_str::<ResponseShape>(&json)?;
    if data.lines.is_empty() {
        return Err(Error::NoItemsBadRequest);
    }
    Ok(data.lines)
}

pub async fn fetch_stash_item_overview(
    item_category: &str,
    league: &TradeLeague,
) -> Result<Vec<Value>, Error> {
    #[derive(Deserialize, Debug, Serialize)]
    struct ResponseShape {
        lines: Vec<Value>,
    }

    let url = format!(
        "https://poe.ninja/poe1/api/economy/stash/current/item/overview?league={league}&type={item_category}"
    );
    let json = reqwest::get(url).await?.text().await?;
    let data = serde_json::from_str::<ResponseShape>(&json)?;
    if data.lines.is_empty() {
        return Err(Error::NoItemsBadRequest);
    }
    Ok(data.lines)
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct DenseOverviewLine {
    name: String,
    #[serde(default)]
    variant: Option<String>,
    #[serde(alias = "chaos", alias = "chaosValue")]
    chaos_value: Option<f32>,
}

#[derive(Deserialize, Debug, Serialize)]
struct DenseOverviewCategory {
    #[serde(rename = "type")]
    r#type: String,
    #[serde(default)]
    lines: Vec<DenseOverviewLine>,
}

pub async fn fetch_stash_dense_overviews_flat(
    league: &TradeLeague,
) -> Result<Vec<Value>, Error> {
    let url = format!(
        "https://poe.ninja/poe1/api/economy/stash/current/dense/overviews?league={league}"
    );
    let json = reqwest::get(url).await?.text().await?;

    if let Ok(categories) = serde_json::from_str::<Vec<DenseOverviewCategory>>(&json) {
        let mut out: Vec<Value> = Vec::new();
        for cat in categories.into_iter() {
            for line in cat.lines.into_iter() {
                let v = serde_json::json!({
                    "name": line.name,
                    "variant": line.variant,
                    "chaos": line.chaos_value,
                    "chaosValue": line.chaos_value,
                });
                out.push(v);
            }
        }
        if out.is_empty() {
            return Err(Error::NoItemsBadRequest);
        }
        return Ok(out);
    }

    let v = serde_json::from_str::<Value>(&json)?;
    let Some(arr) = v.get("overviews").and_then(Value::as_array) else {
        return Err(Error::NoItemsBadRequest);
    };
    let mut out: Vec<Value> = Vec::new();
    for cat in arr.iter() {
        if let Some(lines) = cat.get("lines").and_then(Value::as_array) {
            for line in lines.iter() {
                let name = line.get("name").cloned().unwrap_or(Value::Null);
                let variant = line.get("variant").cloned().unwrap_or(Value::Null);
                let chaos = line
                    .get("chaos")
                    .cloned()
                    .or_else(|| line.get("chaosValue").cloned())
                    .unwrap_or(Value::Null);
                out.push(serde_json::json!({
                    "name": name,
                    "variant": variant,
                    "chaos": chaos,
                    "chaosValue": chaos,
                }));
            }
        }
    }
    if out.is_empty() {
        return Err(Error::NoItemsBadRequest);
    }
    Ok(out)
}

pub async fn fetch_stash_dense_overviews_raw(
    league: &TradeLeague,
) -> Result<Value, Error> {
    let url = format!(
        "https://poe.ninja/poe1/api/economy/stash/current/dense/overviews?league={league}"
    );
    let json = reqwest::get(url).await?.text().await?;
    let v = serde_json::from_str::<Value>(&json)?;
    Ok(v)
}
