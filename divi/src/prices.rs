use crate::{consts::CARDS, error::Error};
use ninja::{card::Sparkline, CardData};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DivinationCardPrice {
    pub name: String,
    #[serde(alias = "chaosValue")]
    pub price: Option<f32>,
    pub sparkline: Sparkline,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct Prices(pub Vec<DivinationCardPrice>);
impl Prices {
    pub async fn fetch(league: &poe::TradeLeague) -> Result<Prices, Error> {
        let ninja_card_data = ninja::fetch_card_data(&league).await.unwrap();
        let mut prices = Prices::default();
        prices.0.iter_mut().for_each(|price| {
            ninja_card_data
                .iter()
                .find(|ninja_data| ninja_data.name == price.name)
                .map(
                    |CardData {
                         sparkline,
                         chaos_value,
                         ..
                     }| {
                        if sparkline.data.len() > 0 {
                            price.price = *chaos_value
                        }
                    },
                );
        });

        Ok(prices)
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
