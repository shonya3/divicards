pub mod error;
pub mod images;
pub mod reward;

use divi::{league::TradeLeague, prices::NinjaCardData};
pub use error::Error;
use fetcher::DataFetcher;
use serde::{Deserialize, Serialize};

use self::reward::reward_to_html;

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
        CardElementDataFetcher::filename()
    }

    pub async fn load() -> Result<Vec<DivinationCardElementData>, Error> {
        CardElementDataFetcher.load().await
    }
}

pub async fn fetch() -> Result<Vec<DivinationCardElementData>, Error> {
    let vec: Vec<NinjaCardData> = NinjaCardData::fetch(&TradeLeague::default()).await?;
    let v: Vec<DivinationCardElementData> = vec
        .into_iter()
        .map(|data| {
            let mut fl = data.flavour_text;
            if fl.starts_with("<size:") {
                fl = fl[10..fl.len() - 1].to_string();
            }

            if fl.starts_with("<smaller>{") {
                fl = fl[10..fl.len() - 1].to_string();
            }

            DivinationCardElementData {
                name: data.name,
                art_filename: data.art_filename,
                flavour_text: fl,
                stack_size: data.stack_size,
                reward_html: reward_to_html(&data.explicit_modifiers[0].text),
            }
        })
        .collect();

    Ok(v)
}

pub struct CardElementDataFetcher;

impl DataFetcher<Vec<DivinationCardElementData>, Error> for CardElementDataFetcher {
    fn filename() -> &'static str {
        "cardElementData.json"
    }

    async fn fetch(&self) -> Result<Vec<DivinationCardElementData>, Error> {
        fetch().await
    }
}
