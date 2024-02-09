use crate::{
    card_record::DivinationCardRecord,
    consts::{CARDS, CARDS_N},
    prices::Prices,
    sample::{Column, Order},
};
use serde::{Deserialize, Serialize};
use std::{
    slice::{Iter, IterMut},
    vec::IntoIter,
};

/// Holds an array of card records with length equal to the number of all divination cards(For example, 440 in 3.23 patch)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Cards(pub Vec<DivinationCardRecord>);

impl Cards {
    pub fn get(&self, name: &str) -> Option<&DivinationCardRecord> {
        self.0.iter().find(|c| c.name == name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut DivinationCardRecord> {
        self.0.iter_mut().find(|c| c.name == name)
    }

    /// Use only with trusted card name(item of CARDS const). Otherwise, use get
    ///  # Panics
    /// If name is not a member of CARDS
    pub fn get_card(&self, name: &str) -> &DivinationCardRecord {
        self.get(name).unwrap()
    }

    /// Use only with trusted card name(item of CARDS const). Otherwise, use get_mut
    /// # Panics
    /// If name is not a member of CARDS
    pub fn get_card_mut(&mut self, name: &str) -> &mut DivinationCardRecord {
        self.get_mut(name).unwrap()
    }

    pub fn iter(&self) -> Iter<'_, DivinationCardRecord> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, DivinationCardRecord> {
        self.0.iter_mut()
    }

    pub fn into_iter(self) -> IntoIter<DivinationCardRecord> {
        self.0.into_iter()
    }

    pub fn into_not_nullish(self) -> Vec<DivinationCardRecord> {
        self.into_iter().filter(|card| card.amount > 0).collect()
    }

    pub fn sort_by_amount(&mut self) {
        self.0.sort_by(|a, b| a.amount.cmp(&b.amount));
    }

    pub fn order_by(&mut self, ordered_by: Column, order: Order) {
        let vec = &mut self.0;
        match ordered_by {
            Column::Name => match order {
                Order::Asc => vec.sort_by(|a, b| a.name.cmp(&b.name)).to_owned(),
                Order::Desc => vec.sort_by(|a, b| b.name.cmp(&a.name)).to_owned(),
                Order::Unordered => {}
            },
            Column::Amount => match order {
                Order::Asc => vec.sort_by(|a, b| a.amount.cmp(&b.amount)).to_owned(),
                Order::Desc => vec.sort_by(|a, b| b.amount.cmp(&a.amount)).to_owned(),
                Order::Unordered => {}
            },
            Column::Weight => match order {
                Order::Asc => vec
                    .sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap())
                    .to_owned(),
                Order::Desc => vec
                    .sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap())
                    .to_owned(),
                Order::Unordered => {}
            },
            Column::Price => match order {
                Order::Asc => vec
                    .sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap())
                    .to_owned(),
                Order::Desc => vec
                    .sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap())
                    .to_owned(),
                Order::Unordered => {}
            },
            Column::Sum => match order {
                Order::Asc => vec
                    .sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap())
                    .to_owned(),
                Order::Desc => vec
                    .sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap())
                    .to_owned(),
                Order::Unordered => {}
            },
        }
    }
}

impl Default for Cards {
    fn default() -> Self {
        Cards::from(CARDS)
    }
}

impl From<Prices> for Cards {
    fn from(prices: Prices) -> Self {
        Cards(
            prices
                .0
                .into_iter()
                .map(|p| DivinationCardRecord::new(p.name, 0, p.price))
                .collect::<Vec<DivinationCardRecord>>()
                .try_into()
                .unwrap(),
        )
    }
}

impl From<[&'static str; CARDS_N]> for Cards {
    fn from(arr: [&'static str; CARDS_N]) -> Self {
        Self(
            arr.into_iter()
                .map(|name| DivinationCardRecord::new(String::from(name), 0, None))
                .collect::<Vec<DivinationCardRecord>>()
                .try_into()
                .unwrap(),
        )
    }
}
