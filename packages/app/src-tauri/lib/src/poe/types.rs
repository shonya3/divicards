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

/// Any item from stash tab
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub base_type: String,
    pub stack_size: Option<u32>,
}

/// Tab from /stashes poe api route, contains only metadata and not items
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TabNoItems {}

/// Tab from /stash poe api route, contains items field
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct TabWithItems {
    pub items: Option<Vec<Item>>,
    #[serde(rename = "type")]
    pub kind: Option<StashType>,
}

impl IsCard for Item {
    fn is_card(&self) -> bool {
        let name = self.base_type.as_str();
        // Fire of Unknown Origin casing bug https://www.pathofexile.com/forum/view-thread/3411333
        name == "Fire Of Unknown Origin" || CARDS.contains(&name)
    }

    fn is_legacy_card(&self) -> bool {
        LEGACY_CARDS.contains(&self.base_type.as_str())
    }
}

impl From<TabWithItems> for SampleData {
    fn from(tab: TabWithItems) -> Self {
        let cards: Vec<CardNameAmount> = tab
            .items
            .unwrap_or_default()
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
