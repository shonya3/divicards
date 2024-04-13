use crate::Error;
use poe::league::TradeLeague;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub async fn fetch_card_data(league: &TradeLeague) -> Result<Vec<CardData>, Error> {
    #[derive(Deserialize, Debug, Serialize)]
    struct ResponseShape {
        lines: Vec<CardData>,
    }

    let url = format!(
        "https://poe.ninja/api/data/itemoverview?league={league}&type=DivinationCard&language=en"
    );
    let json = reqwest::get(url).await?.text().await?;
    let data = serde_json::from_str::<ResponseShape>(&json)?;
    if data.lines.len() == 0 {
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

impl CardData {
    pub async fn fetch(league: &TradeLeague) -> Result<Vec<CardData>, Error> {
        fetch_card_data(&league).await
    }
}
