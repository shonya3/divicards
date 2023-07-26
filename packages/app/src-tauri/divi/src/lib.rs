#![allow(unused)]

use consts::{CARDS, LEGACY_CARDS};
use serde::{Deserialize, Serialize};
pub mod card_record;
pub mod cards;
pub mod consts;
pub mod error;
pub mod league;
pub mod prices;
pub mod sample;

#[derive(Debug)]
pub enum IsACard {
    FixCardName(String, String),
    NotACard(String),
    Card,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FixedCardName {
    pub old: String,
    pub fixed: String,
}

impl FixedCardName {
    pub fn new(old: &str, fixed: &str) -> FixedCardName {
        FixedCardName {
            old: String::from(old),
            fixed: String::from(fixed),
        }
    }
}

impl DivinationCard for &str {
    fn is_card(&self) -> bool {
        CARDS.contains(self)
    }

    fn is_legacy_card(&self) -> bool {
        LEGACY_CARDS.contains(self)
    }
}

pub trait DivinationCard {
    fn is_card(&self) -> bool;
    fn is_legacy_card(&self) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[tokio::test]
    // async fn name_amount() {
    //     let json = std::fs::read_to_string("cardNameAmountList.json").unwrap();
    //     let vec: Vec<CardNameAmount> = serde_json::from_str(&json).unwrap();
    //     let cards_total_amount: i32 = vec.iter().map(|card| card.amount).sum();
    //     assert_eq!(cards_total_amount, 181);
    //     let sample = DivinationCardsSample::create(
    //         SampleData::CardNameAmountList(vec),
    //         Prices::fetch(&TradeLeague::HardcoreCrucible).await.unwrap(),
    //     )
    //     .unwrap();

    //     let sample_total_amount: i32 = sample.cards.iter().map(|card| card.amount).sum();
    //     dbg!(sample_total_amount);
    // }

    use serde_json::Value;

    use super::*;
}
