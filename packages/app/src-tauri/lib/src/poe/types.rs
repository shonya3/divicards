use divi::{
    consts::{CARDS, LEGACY_CARDS},
    sample::{CardNameAmount, SampleData},
    IsCard,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum StashType {
    PremiumStash,
    CurrencyStash,
    MapStash,
    QuadStash,
    FragmentStash,
    EssenceStash,
    Folder,
    NormalStash,
    DivinationCardStash,
    #[serde(other)]
    Other,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub base_type: String,
    pub stack_size: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TabNoItems {}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TabWithItems {
    pub items: Vec<Item>,
    #[serde(rename = "type")]
    pub kind: Option<StashType>,
}

impl IsCard for Item {
    fn is_card(&self) -> bool {
        CARDS.contains(&self.base_type.as_str())
    }

    fn is_legacy_card(&self) -> bool {
        LEGACY_CARDS.contains(&self.base_type.as_str())
    }
}

impl From<TabWithItems> for SampleData {
    fn from(tab: TabWithItems) -> Self {
        let cards: Vec<CardNameAmount> = tab
            .items
            .into_iter()
            .filter(|item| item.is_card())
            .map(|item| CardNameAmount {
                name: item.base_type,
                amount: item.stack_size.unwrap_or_default(),
            })
            .collect();

        SampleData::CardNameAmountList(cards)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::TabWithItems;

    #[test]
    fn ser_stash() {
        let json = read_to_string("stash.json").unwrap();
        let stash: TabWithItems = serde_json::from_str(&json).unwrap();
        dbg!(stash);
    }
}
