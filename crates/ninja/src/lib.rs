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
