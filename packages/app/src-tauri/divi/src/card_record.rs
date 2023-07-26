use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    consts::{CARDS, CONDENSE_FACTOR, LEGACY_CARDS},
    IsCard,
};

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DivinationCardRecord {
    pub name: String,
    #[serde(alias = "calculated")]
    pub price: Option<f32>,
    #[serde(alias = "stackSize")]
    pub amount: i32,
    pub sum: Option<f32>,
    pub weight: Option<f32>,
}

impl DivinationCardRecord {
    pub fn new(name: &str, price: Option<f32>, amount: Option<i32>) -> DivinationCardRecord {
        DivinationCardRecord {
            name: name.to_string(),
            price,
            amount: amount.unwrap_or_default(),
            sum: Some(price.unwrap_or_default() * amount.unwrap_or_default() as f32),
            weight: None,
        }
    }

    pub fn sum(&self) -> Option<f32> {
        Some(self.price.unwrap_or_default() * self.amount as f32)
    }

    pub fn set_amount(&mut self, amount: i32) -> &mut Self {
        self.amount = amount;
        self.sum = self.sum();
        self
    }

    pub fn local_weight(&self, sample_size: i32) -> f32 {
        self.amount as f32 / sample_size as f32
    }

    pub fn weight(&mut self, weight_sample: f32, sample_size: i32) -> &mut Self {
        self.weight =
            Some((weight_sample * self.local_weight(sample_size)).powf(1.0 / CONDENSE_FACTOR));
        self
    }

    fn most_similar_card(name: &str) -> (String, f64) {
        let mut similarity_map = HashMap::<String, f64>::new();
        for card in CARDS {
            let similarity = strsim::normalized_damerau_levenshtein(&name, card);
            similarity_map.insert(card.to_string(), similarity);
        }

        let most_similar = similarity_map
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap();

        (most_similar.0.to_owned(), most_similar.1.to_owned())
    }

    pub fn fix_name(&mut self) -> Option<FixedCardName> {
        match self.is_card() {
            true => None,
            false => self.fix_name_unchecked(),
        }
    }

    pub fn fix_name_unchecked(&mut self) -> Option<FixedCardName> {
        let (similar, score) = Self::most_similar_card(&self.name);
        match score >= 0.75 {
            true => {
                let fixed_name = FixedCardName::new(&self.name, &similar);
                self.name = similar;
                Some(fixed_name)
            }
            false => {
                let the_name = format!("The {}", &self.name);
                let (similar, score) = Self::most_similar_card(&the_name);
                match score >= 0.75 {
                    true => {
                        let fixed_name = FixedCardName::new(&self.name, &similar);
                        self.name = similar;
                        Some(fixed_name)
                    }
                    false => None,
                }
            }
        }
    }
}

impl Default for DivinationCardRecord {
    fn default() -> Self {
        Self {
            name: String::from("Rain Of Chaos"),
            price: None,
            amount: 0,
            weight: None,
            sum: None,
        }
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
        let record = DivinationCardRecord::new("Rain of Chaos", None, None);
        assert_eq!(record.is_card(), true);
    }

    #[test]
    fn is_legacy_card() {
        let record = DivinationCardRecord::new("Friendship", None, None);
        assert_eq!(record.is_legacy_card(), true);
    }
}
