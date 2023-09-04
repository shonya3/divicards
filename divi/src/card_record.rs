use serde::{Deserialize, Serialize};

use crate::{
    consts::{CARDS, CONDENSE_FACTOR, LEGACY_CARDS},
    IsCard,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DivinationCardRecord {
    pub name: String,
    #[serde(alias = "stackSize")]
    pub amount: u32,
    #[serde(alias = "calculated")]
    pub price: Option<f32>,
    pub sum: Option<f32>,
    pub weight: Option<f32>,
}

impl DivinationCardRecord {
    pub fn new(name: String, amount: u32, price: Option<f32>) -> DivinationCardRecord {
        DivinationCardRecord {
            name,
            price,
            amount,
            sum: Some(price.unwrap_or_default() * amount as f32),
            weight: None,
        }
    }

    pub fn set_amount(&mut self, amount: u32) {
        self.amount = amount;
        self.sum = Some(self.amount as f32 * self.price.unwrap_or_default())
    }

    pub fn add_amount(&mut self, amount: u32) {
        self.set_amount(self.amount + amount);
    }

    pub fn set_weight(&mut self, weight_multiplier: f32) {
        self.weight = Some((weight_multiplier * self.amount as f32).powf(1.0 / CONDENSE_FACTOR));
    }
}

impl IsCard for DivinationCardRecord {
    fn is_card(&self) -> bool {
        CARDS.contains(&self.name.as_str())
    }

    fn is_legacy_card(&self) -> bool {
        LEGACY_CARDS.contains(&self.name.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_card() {
        let record = DivinationCardRecord::new("Rain of Chaos".to_string(), 1, None);
        assert_eq!(record.is_card(), true);
    }

    #[test]
    fn is_legacy_card() {
        let record = DivinationCardRecord::new("Friendship".to_string(), 1, None);
        assert_eq!(record.is_legacy_card(), true);
    }
}
