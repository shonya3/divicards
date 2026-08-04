#![doc = include_str!("../README.md")]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UniqueReward {
    pub name: String,
    pub item_class: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DivinationCardElementData {
    pub slug: String,
    pub name: String,
    pub art_filename: String,
    pub reward_html: String,
    pub flavour_text: String,
    pub stack_size: u32,
    pub min_level: u32,
    pub unique: Option<UniqueReward>,
}
