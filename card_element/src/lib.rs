pub mod error;
pub mod fetcher;
pub mod images;
pub mod reward;

use ::fetcher::DataFetcher;
pub use error::Error;
use fetcher::Fetcher;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DivinationCardElementData {
    pub name: String,
    pub art_filename: String,
    pub reward_html: String,
    pub flavour_text: String,
    pub stack_size: Option<usize>,
}

impl DivinationCardElementData {
    pub fn filename() -> &'static str {
        Fetcher::default().config().filename
    }

    pub async fn load() -> Result<Vec<DivinationCardElementData>, Error> {
        Fetcher::default().load().await
    }
}
