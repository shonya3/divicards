use crate::poe_data::league::ReleaseVersion;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CardsData {
    pub dict: HashMap<String, Card>,
    /// Total number of cards collected during latest league by community.
    pub latest_weights_collected: LeagueWeightsCollected,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LeagueWeightsCollected {
    pub version: ReleaseVersion,
    pub total_cards: u32,
}

impl CardsData {
    pub fn card(&self, s: &str) -> &Card {
        let Some(card) = self.dict.get(s) else {
            panic!("Card not exists {s}");
        };
        card
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub slug: String,
    pub name: String,
    #[serde(default)]
    pub min_level: u32,
    pub id: String,
    pub flavour_text: String,
    pub art_filename: String,
    pub stack_size: u32,
    #[serde(default)]
    pub weights: HashMap<String, f32>,
    #[serde(default)]
    pub price: Option<f32>,
    #[serde(default)]
    pub league: Option<crate::poe_data::league::LeagueReleaseInfo>,
    pub disabled: bool,

    /// List of map names, provided by in-game atlas.
    pub atlas_maps: Vec<String>,
}
