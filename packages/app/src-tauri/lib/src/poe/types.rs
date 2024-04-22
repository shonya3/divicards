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
#[serde(transparent)]
pub struct Item(serde_json::Value);

impl Item {
    pub fn base_type(&self) -> Option<&str> {
        self.0["baseType"].as_str()
    }
    pub fn stack_size(&self) -> Option<u32> {
        self.0["stackSize"].as_u64().map(|v| v as u32)
    }
}

/// Tab from /stashes poe api route, contains only metadata and not items
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TabNoItems {}

/// Tab from /stash poe api route, contains items field
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct TabWithItems {
    #[serde(default)]
    pub items: Vec<Item>,
    #[serde(rename = "type")]
    pub kind: Option<StashType>,
}

impl IsCard for Item {
    fn is_card(&self) -> bool {
        let Some(name) = self.base_type() else {
            return false;
        };
        // Fire of Unknown Origin casing bug https://www.pathofexile.com/forum/view-thread/3411333
        name == "Fire Of Unknown Origin" || CARDS.contains(&name)
    }

    fn is_legacy_card(&self) -> bool {
        let Some(name) = self.base_type() else {
            return false;
        };
        LEGACY_CARDS.contains(&name)
    }
}

impl From<TabWithItems> for SampleData {
    fn from(tab: TabWithItems) -> Self {
        let cards: Vec<CardNameAmount> = tab
            .items
            .into_iter()
            .filter(|item| item.is_card())
            .map(|item| CardNameAmount {
                name: item.base_type().unwrap_or_default().to_owned(),
                amount: item.stack_size().unwrap_or_default(),
            })
            .collect();

        SampleData::CardNameAmountList(cards)
    }
}
