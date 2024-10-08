use crate::{
    consts::{CARDS, LEGACY_CARDS},
    IsCard,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CardRecord {
    #[serde(alias = "Name")]
    pub name: String,
    #[serde(alias = "Quantity")]
    pub amount: u32,
    #[serde(alias = "calculated")]
    pub price: Option<f32>,
    pub sum: Option<f32>,
    pub weight: Option<f32>,
}

impl CardRecord {
    #[must_use]
    pub fn new(name: String, amount: u32, price: Option<f32>) -> CardRecord {
        CardRecord {
            name,
            price,
            amount,
            sum: Some(price.unwrap_or_default() * amount as f32),
            weight: None,
        }
    }

    pub fn set_amount(&mut self, amount: u32) {
        self.amount = amount;
        self.sum = Some(self.amount as f32 * self.price.unwrap_or_default());
    }

    pub fn add_amount(&mut self, amount: u32) {
        self.set_amount(self.amount + amount);
    }
}

impl IsCard for CardRecord {
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
        let record = CardRecord::new("Rain of Chaos".to_string(), 1, None);
        assert!(record.is_card());
    }

    #[test]
    fn is_legacy_card() {
        let record = CardRecord::new(LEGACY_CARDS[0].to_owned(), 1, None);
        assert!(record.is_legacy_card());
    }
}
