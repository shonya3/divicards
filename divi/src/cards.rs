use crate::{
    card_record::CardRecord,
    consts::CARDS,
    prices::Prices,
    sample::{Column, Order},
    IsCard,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    slice::{Iter, IterMut},
};

impl IntoIterator for Cards {
    type Item = CardRecord;
    type IntoIter = std::vec::IntoIter<CardRecord>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// Holds an array of card records with length equal to the number of all divination cards(For example, 440 in 3.23 patch)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Cards(pub Vec<CardRecord>);

impl Cards {
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&CardRecord> {
        self.0.iter().find(|c| c.name == name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut CardRecord> {
        self.0.iter_mut().find(|c| c.name == name)
    }

    #[must_use]
    pub fn get_record(&self, name: &str) -> GetRecord {
        match check_card_name(name) {
            CheckCardName::Valid => GetRecord::Valid(self.get_card(name)),
            CheckCardName::TypoFixed(fixed) => {
                GetRecord::TypoFixed(self.get_card(&fixed.fixed), fixed)
            }
            CheckCardName::NotACard => GetRecord::NotACard,
        }
    }

    pub fn get_record_mut(&mut self, name: &str) -> GetRecordMut {
        match check_card_name(name) {
            CheckCardName::Valid => GetRecordMut::Valid(self.get_card_mut(name)),
            CheckCardName::TypoFixed(fixed) => {
                GetRecordMut::TypoFixed(self.get_card_mut(&fixed.fixed), fixed)
            }
            CheckCardName::NotACard => GetRecordMut::NotACard,
        }
    }

    /// Use only with trusted card name(item of CARDS const). Otherwise, use get
    ///  # Panics
    /// If name is not a member of CARDS
    #[must_use]
    pub fn get_card(&self, name: &str) -> &CardRecord {
        self.get(name).unwrap()
    }

    /// Use only with trusted card name(item of CARDS const). Otherwise, use `get_mut`
    /// # Panics
    /// If name is not a member of CARDS
    pub fn get_card_mut(&mut self, name: &str) -> &mut CardRecord {
        self.get_mut(name).unwrap()
    }

    pub fn iter(&self) -> Iter<'_, CardRecord> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, CardRecord> {
        self.0.iter_mut()
    }

    pub fn order_by(&mut self, ordered_by: Column, order: Order) {
        let vec = &mut self.0;
        match ordered_by {
            Column::Name => match order {
                Order::Asc => vec.sort_by(|a, b| a.name.cmp(&b.name)),
                Order::Desc => vec.sort_by(|a, b| b.name.cmp(&a.name)),
                Order::Unordered => {}
            },
            Column::Amount => match order {
                Order::Asc => vec.sort_by(|a, b| a.amount.cmp(&b.amount)),
                Order::Desc => vec.sort_by(|a, b| b.amount.cmp(&a.amount)),
                Order::Unordered => {}
            },
            Column::Weight => match order {
                Order::Asc => vec.sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap()),
                Order::Desc => vec.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap()),
                Order::Unordered => {}
            },
            Column::Price => match order {
                Order::Asc => vec.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap()),
                Order::Desc => vec.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap()),
                Order::Unordered => {}
            },
            Column::Sum => match order {
                Order::Asc => vec.sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap()),
                Order::Desc => vec.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap()),
                Order::Unordered => {}
            },
        }
    }
}

impl Default for Cards {
    fn default() -> Self {
        Cards(
            CARDS
                .into_iter()
                .map(|name| CardRecord::new(name.to_owned(), 0, None))
                .collect::<Vec<CardRecord>>(),
        )
    }
}

impl From<Prices> for Cards {
    fn from(prices: Prices) -> Self {
        Cards(
            prices
                .0
                .into_iter()
                .map(|p| CardRecord::new(p.name, 0, p.price))
                .collect::<Vec<CardRecord>>(),
        )
    }
}

#[must_use]
pub fn check_card_name(card: &str) -> CheckCardName {
    if card.is_card() {
        return CheckCardName::Valid;
    };

    match fix_name(card) {
        Some(fixed) => CheckCardName::TypoFixed(FixedCardName {
            old: card.to_owned(),
            fixed,
        }),
        None => CheckCardName::NotACard,
    }
}

pub enum CheckCardName {
    Valid,
    TypoFixed(FixedCardName),
    NotACard,
}

pub enum GetRecord<'a> {
    Valid(&'a CardRecord),
    TypoFixed(&'a CardRecord, FixedCardName),
    NotACard,
}

pub enum GetRecordMut<'a> {
    Valid(&'a mut CardRecord),
    TypoFixed(&'a mut CardRecord, FixedCardName),
    NotACard,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FixedCardName {
    pub old: String,
    pub fixed: String,
}

impl FixedCardName {
    #[must_use]
    pub fn new(old: &str, fixed: &str) -> FixedCardName {
        FixedCardName {
            old: String::from(old),
            fixed: String::from(fixed),
        }
    }
}

fn fix_name(name: &str) -> Option<String> {
    if name.is_card() {
        return None;
    }

    let (most_similar, score) = most_similar_card(name);

    match score >= 0.75 {
        true => Some(String::from(most_similar)),
        false => {
            // Try to prefix name with "The" - a lot of cards start with "The"
            let the = format!("The {name}");
            let (most_similar, score) = most_similar_card(&the);
            match score >= 0.75 {
                true => Some(String::from(most_similar)),
                false => None,
            }
        }
    }
}

fn most_similar_card(name: &str) -> (&str, f64) {
    let mut similarity_map = HashMap::<&str, f64>::new();
    for card in CARDS {
        let similarity = strsim::normalized_damerau_levenshtein(name, card);
        similarity_map.insert(card, similarity);
    }

    let most_similar = similarity_map
        .iter()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .unwrap();

    (most_similar.0, most_similar.1.to_owned())
}
