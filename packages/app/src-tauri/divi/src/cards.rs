use std::{
    ops::{Deref, DerefMut},
    slice::{Iter, IterMut},
};

use crate::{
    card_record::DivinationCardRecord,
    consts::{CARDS, CARDS_N},
    prices::{DivinationCardPrice, Prices},
};
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cards(#[serde(with = "BigArray")] pub [DivinationCardRecord; CARDS_N]);

impl Cards {
    pub fn get(&self, name: &str) -> Option<&DivinationCardRecord> {
        self.0.iter().find(|c| c.name == name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut DivinationCardRecord> {
        self.0.iter_mut().find(|c| c.name == name)
    }

    pub fn iter(&self) -> Iter<'_, DivinationCardRecord> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, DivinationCardRecord> {
        self.0.iter_mut()
    }
}

impl Default for Cards {
    fn default() -> Self {
        CARDS.into()
    }
}

impl From<Prices> for Cards {
    fn from(prices: Prices) -> Self {
        Cards(
            prices
                .0
                .into_iter()
                .map(|DivinationCardPrice { name, price }| DivinationCardRecord {
                    name,
                    price,
                    ..Default::default()
                })
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
                .map(|s| DivinationCardRecord {
                    name: s.to_string(),
                    ..Default::default()
                })
                .collect::<Vec<DivinationCardRecord>>()
                .try_into()
                .unwrap(),
        )
    }
}

impl Deref for Cards {
    type Target = [DivinationCardRecord; CARDS_N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Cards {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

mod tests {}
