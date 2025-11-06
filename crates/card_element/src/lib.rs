pub mod base_items_fetcher;
pub mod drop_level;
pub mod error;
pub mod fetcher;
pub mod images;
pub mod reward;
pub mod unique;
pub mod uniques_fetcher;
use crate::unique::UniqueReward;

use drop_level::DropLevel;
pub use error::Error;
use fetcher::Fetcher;
use fs_cache_fetcher::DataFetcher;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DivinationCardElementData {
    pub slug: String,
    pub name: String,
    pub art_filename: String,
    pub reward_html: String,
    pub flavour_text: String,
    pub stack_size: Option<usize>,
    pub drop_level: DropLevel,
    pub unique: Option<UniqueReward>,
}

impl DivinationCardElementData {
    pub fn filename() -> &'static str {
        Fetcher::default().config().filename
    }

    pub async fn load() -> Result<Vec<DivinationCardElementData>, Error> {
        Fetcher::default().load().await
    }
}
