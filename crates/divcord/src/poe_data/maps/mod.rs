use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Map {
    pub name: String,
    pub tier: u32,
    pub unique: bool,
    pub icon: String,
    pub slug: String,

    /// List of card names, provided by in-game atlas.
    pub atlas_cards: Vec<String>,
}

impl Map {
    pub fn level(&self) -> u32 {
        67 + self.tier
    }
}
