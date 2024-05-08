use crate::consts::CARDS;
use ninja::CardData as NinjaCardData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DivinationCardPrice {
    pub name: String,
    #[serde(alias = "chaosValue")]
    pub price: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct Prices(pub Vec<DivinationCardPrice>);
impl Prices {
    pub async fn fetch(league: &poe::TradeLeague) -> Result<Prices, ninja::Error> {
        let ninja_card_data = ninja::fetch_card_data(league).await?;
        let mut prices = Prices::default();
        prices.0.iter_mut().for_each(|price| {
            if let Some(NinjaCardData {
                sparkline,
                chaos_value,
                ..
            }) = ninja_card_data
                .iter()
                .find(|ninja_data| ninja_data.name == price.name)
            {
                if sparkline.data.is_empty() {
                    price.price = *chaos_value;
                }
            }
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
                })
                .collect::<Vec<DivinationCardPrice>>(),
        )
    }
}
