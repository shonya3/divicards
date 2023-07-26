use serde::{Deserialize, Serialize};

use crate::{
    consts::{CARDS, CARDS_N},
    league::TradeLeague,
};
use serde_big_array::BigArray;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DivinationCardPrice {
    pub name: String,
    #[serde(alias = "chaosValue")]
    pub price: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Prices(#[serde(with = "BigArray")] pub [DivinationCardPrice; CARDS_N]);
impl Prices {
    pub async fn fetch(league: &TradeLeague) -> Result<Prices, reqwest::Error> {
        Ok(Prices(DivinationCardPrice::fetch(league).await?))
    }
}
impl Default for Prices {
    fn default() -> Self {
        CARDS.into()
    }
}

impl From<[&'static str; CARDS_N]> for Prices {
    fn from(arr: [&'static str; CARDS_N]) -> Self {
        let prices: [DivinationCardPrice; CARDS_N] = arr
            .into_iter()
            .map(|name| DivinationCardPrice {
                name: name.to_string(),
                price: Default::default(),
            })
            .collect::<Vec<DivinationCardPrice>>()
            .try_into()
            .unwrap();
        Prices(prices)
    }
}

impl DivinationCardPrice {
    pub async fn fetch(league: &TradeLeague) -> Result<[Self; CARDS_N], reqwest::Error> {
        #[derive(Deserialize, Debug, Serialize)]
        struct PriceData {
            lines: Vec<DivinationCardPrice>,
        }

        let client = reqwest::Client::new();
        let url = format!(
            "https://poe.ninja/api/data/itemoverview?league={league}&type=DivinationCard&language=en",
        );
        let json = client.get(url).send().await?.text().await?;

        let price_data: PriceData = serde_json::from_str(&json).unwrap();
        let prices = price_data.lines;

        let prices_arr: [DivinationCardPrice; CARDS_N] = CARDS
            .into_iter()
            .map(|card| {
                let price = prices
                    .iter()
                    .find(|div_card_price| div_card_price.name == card)
                    .and_then(|v| v.price);
                DivinationCardPrice {
                    name: card.to_string(),
                    price,
                }
            })
            .collect::<Vec<Self>>()
            .try_into()
            .unwrap();

        Ok(prices_arr)
    }
}
