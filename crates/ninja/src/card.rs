use crate::Error;
use poe::league::TradeLeague;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeCardPrice {
    pub name: String,
    pub chaos_value: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExchangeLine {
    id: String,
    primary_value: f64,
    max_volume_currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExchangeItem {
    id: String,
    name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExchangeResponse {
    lines: Vec<ExchangeLine>,
    items: Vec<ExchangeItem>,
}

/// Fetch divination card prices from poe.ninja's exchange economy endpoint.
///
/// Only lists cards traded on the currency exchange — coverage varies by league
/// (narrow on Standard, wider on challenge leagues).
/// Cards not found on the exchange are not included in the result.
/// Cards priced in divine are converted to chaos using the divine:chaos rate
/// fetched from the Currency exchange endpoint.
pub async fn fetch_exchange_prices(league: TradeLeague) -> Result<Vec<ExchangeCardPrice>, Error> {
    let league_str = league.to_string();
    let client = Client::new();

    let cards_url = format!(
        "https://poe.ninja/poe1/api/economy/exchange/current/overview?league={league_str}&type=DivinationCard"
    );
    let currency_url = format!(
        "https://poe.ninja/poe1/api/economy/exchange/current/overview?league={league_str}&type=Currency"
    );

    let cards_data: ExchangeResponse = client
        .get(&cards_url)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    let currency_lines: Vec<Value> = client
        .get(&currency_url)
        .send()
        .await?
        .error_for_status()?
        .json::<Value>()
        .await?["lines"]
        .as_array()
        .cloned()
        .unwrap_or_default();

    // Divine→chaos rate: divine's primaryValue when maxVolumeCurrency is "chaos"
    let divine_to_chaos = currency_lines
        .iter()
        .find(|line| line["id"] == "divine")
        .and_then(|divine| {
            if divine["maxVolumeCurrency"] == "chaos" {
                divine["primaryValue"].as_f64()
            } else {
                None
            }
        })
        .unwrap_or(0.0);

    let id_to_name: std::collections::HashMap<&str, &str> = cards_data
        .items
        .iter()
        .map(|item| (item.id.as_str(), item.name.as_str()))
        .collect();

    let prices: Vec<ExchangeCardPrice> = cards_data
        .lines
        .iter()
        .map(|line| {
            let name = id_to_name
                .get(line.id.as_str())
                .map(|&n| n.to_string())
                .unwrap_or_default();
            let chaos_value = match line.max_volume_currency.as_str() {
                "chaos" => Some(line.primary_value as f32),
                "divine" if divine_to_chaos > 0.0 => {
                    Some((line.primary_value * divine_to_chaos) as f32)
                }
                _ => None,
            };
            ExchangeCardPrice { name, chaos_value }
        })
        .collect();

    Ok(prices)
}

pub async fn fetch_card_data(league: TradeLeague) -> Result<Vec<CardData>, Error> {
    #[derive(Deserialize, Debug, Serialize)]
    struct ResponseShape {
        lines: Vec<CardData>,
    }

    let league_str = league.to_string();
    let params = [
        ("league", league_str.as_str()),
        ("type", "DivinationCard"),
        ("language", "en"),
    ];
    let data = Client::new()
        .get("https://poe.ninja/api/data/itemoverview")
        .query(&params)
        .send()
        .await?
        .json::<ResponseShape>()
        .await?;
    if data.lines.is_empty() {
        return Err(Error::NoItemsBadRequest);
    }
    Ok(data.lines)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardData {
    pub id: usize,
    pub name: String,
    pub icon: String,
    pub stack_size: Option<usize>,
    pub art_filename: String,
    pub item_class: usize,
    pub spark_line: Sparkline,
    pub low_confidence_spark_line: Sparkline,
    pub implicit_modifiers: Vec<Value>,
    pub explicit_modifiers: Vec<ExpilicitModifier>,
    pub flavour_text: String,
    pub chaos_value: Option<f32>,
    pub exalted_value: Option<f32>,
    pub divine_value: Option<f32>,
    pub count: usize,
    pub details_id: String,
    pub trade_info: Vec<Value>,
    pub listing_count: usize,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Sparkline {
    pub data: Vec<Option<f32>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpilicitModifier {
    pub optional: bool,
    pub text: String,
}

impl CardData {
    pub async fn fetch(league: TradeLeague) -> Result<Vec<CardData>, Error> {
        fetch_card_data(league).await
    }
}
