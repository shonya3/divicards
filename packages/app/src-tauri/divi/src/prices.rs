use serde::{Deserialize, Serialize};

use crate::{
    consts::{CARDS, CARDS_N},
    league::TradeLeague,
};
use serde_big_array::BigArray;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DivinationCardPrice {
    pub name: String,
    #[serde(alias = "chaosValue")]
    pub price: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct Prices(#[serde(with = "BigArray")] pub [DivinationCardPrice; CARDS_N]);
impl Prices {
    pub async fn fetch(league: &TradeLeague) -> Result<Prices, reqwest::Error> {
        #[derive(Deserialize, Debug, Serialize)]
        struct PriceData {
            lines: Vec<DivinationCardPrice>,
        }

        let client = reqwest::Client::new();
        let url = format!("https://poe.ninja/api/data/itemoverview?league={league}&type=DivinationCard&language=en");
        let json = client.get(url).send().await?.text().await?;
        let data = serde_json::from_str::<PriceData>(&json).unwrap();
        Ok(Prices::from(data.lines))
    }
}

impl Default for Prices {
    fn default() -> Self {
        Prices::from(CARDS)
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

impl From<Vec<DivinationCardPrice>> for Prices {
    fn from(value: Vec<DivinationCardPrice>) -> Self {
        let mut prices = Prices::default();
        for card in prices.0.iter_mut() {
            if let Some(found) = value.iter().find(|c| card.name == c.name) {
                card.price = found.price
            }
        }
        prices
    }
}
