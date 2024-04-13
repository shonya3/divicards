use crate::{consts::CARDS, error::Error, league::TradeLeague};
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    pub async fn fetch(league: &TradeLeague) -> Result<Prices, Error> {
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
        Prices::from(
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NinjaCardData {
    pub id: usize,
    pub name: String,
    pub icon: String,
    pub stack_size: Option<usize>,
    pub art_filename: String,
    pub item_class: usize,
    pub sparkline: Sparkline,
    pub low_confidence_sparkline: Sparkline,
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

impl NinjaCardData {
    pub async fn fetch(league: &TradeLeague) -> Result<Vec<NinjaCardData>, Error> {
        #[derive(Deserialize, Debug, Serialize)]
        struct PriceData {
            lines: Vec<NinjaCardData>,
        }

        let client = reqwest::Client::new();
        let url = format!("https://poe.ninja/api/data/itemoverview?league={league}&type=DivinationCard&language=en");
        let json = client.get(url).send().await?.text().await?;
        let data = serde_json::from_str::<PriceData>(&json)?;
        if data.lines.len() == 0 {
            return Err(Error::NoPricesForLeagueOnNinja(league.to_owned()));
        }
        Ok(data.lines)
    }
}

#[test]
pub fn fetch_ninja() {
    let data: Vec<NinjaCardData> =
        serde_json::from_str(&std::fs::read_to_string("ninja-data.json").unwrap()).unwrap();
    for card in data.iter() {
        if card.stack_size.is_none() {
            dbg!(&card.name);
        }
    }
}
