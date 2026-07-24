use crate::consts::CARDS;
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
    ///
    /// ## Errors
    /// Returns `ninja::Error` when cannot fetch from ninja
    pub async fn fetch(league: poe::TradeLeague) -> Result<Prices, ninja::Error> {
        let exchange_prices = ninja::fetch_exchange_prices(league).await?;
        let mut prices = Prices::default();
        prices.0.iter_mut().for_each(|price| {
            if let Some(exchange) = exchange_prices.iter().find(|e| e.name == price.name) {
                price.price = exchange.chaos_value;
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
                    price: None,
                })
                .collect::<Vec<DivinationCardPrice>>(),
        )
    }
}
