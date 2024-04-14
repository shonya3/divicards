use crate::{consts::CARDS, error::Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DivinationCardPrice {
    pub name: String,
    #[serde(alias = "chaosValue")]
    pub price: Option<f32>,
    pub sparkline: Sparkline,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Sparkline {
    pub data: Vec<Option<f32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct Prices(pub Vec<DivinationCardPrice>);
impl Prices {
    pub async fn fetch(league: &poe::TradeLeague) -> Result<Prices, Error> {
        #[derive(Deserialize, Debug, Serialize)]
        struct PriceData {
            lines: Vec<DivinationCardPrice>,
        }

        let url = format!("https://poe.ninja/api/data/itemoverview?league={league}&type=DivinationCard&language=en");
        let json = reqwest::get(url).await?.text().await?;
        let data = serde_json::from_str::<PriceData>(&json)?;
        if data.lines.len() == 0 {
            return Err(Error::NoPricesForLeagueOnNinja(league.to_owned()));
        }
        Ok(Prices::from(data.lines))
    }
}

impl Default for Prices {
    fn default() -> Self {
        Prices(
            CARDS
                .into_iter()
                .map(|name| DivinationCardPrice {
                    name: name.to_string(),
                    price: Default::default(),
                    sparkline: Default::default(),
                })
                .collect::<Vec<DivinationCardPrice>>(),
        )
    }
}

impl From<Vec<DivinationCardPrice>> for Prices {
    fn from(value: Vec<DivinationCardPrice>) -> Self {
        let mut prices = Prices::default();
        for card in prices.0.iter_mut() {
            if let Some(found) = value.iter().find(|c| card.name == c.name) {
                if found.sparkline.data.len() > 0 {
                    card.price = found.price;
                }
            }
        }
        prices
    }
}
