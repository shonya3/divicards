use crate::{
    card_record::CardRecord,
    consts::CARDS,
    prices::Prices,
    sample::{Column, Order},
    IsCard,
};
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    collections::HashMap,
    slice::{Iter, IterMut},
};

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
                Order::Asc => {
                    vec.sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap_or(Ordering::Less));
                }
                Order::Desc => {
                    vec.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap_or(Ordering::Less));
                }
                Order::Unordered => {}
            },
            Column::Price => match order {
                Order::Asc => {
                    vec.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap_or(Ordering::Less));
                }
                Order::Desc => {
                    vec.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap_or(Ordering::Less));
                }
                Order::Unordered => {}
            },
            Column::Sum => {
                match order {
                    Order::Asc => vec
                        .sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap_or(Ordering::Less)),
                    Order::Desc => vec
                        .sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap_or(Ordering::Less)),
                    Order::Unordered => {}
                }
            }
        }
    }
}

impl IntoIterator for Cards {
    type Item = CardRecord;
    type IntoIter = std::vec::IntoIter<CardRecord>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Cards {
    type Item = &'a CardRecord;
    type IntoIter = std::slice::Iter<'a, CardRecord>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Cards {
    type Item = &'a mut CardRecord;
    type IntoIter = std::slice::IterMut<'a, CardRecord>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
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
    }

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

    if score >= 0.75 {
        Some(String::from(most_similar))
    } else {
        // Try to prefix name with "The" - a lot of cards start with "The"
        let the = format!("The {name}");
        let (most_similar, score) = most_similar_card(&the);
        match score >= 0.75 {
            true => Some(String::from(most_similar)),
            false => None,
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
