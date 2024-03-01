use crate::{reward::reward_to_html, DivinationCardElementData, Error};
use divi::{league::TradeLeague, prices::NinjaCardData};
use fetcher::{Config, DataFetcher, Stale, WithConfig};

pub struct Fetcher(Config);

impl Default for Fetcher {
    fn default() -> Self {
        Self(Config {
            save: true,
            filename: "cardElementData.json",
            stale: Stale::ReloadEveryTime,
        })
    }
}

impl WithConfig for Fetcher {
    fn config(&self) -> &Config {
        &self.0
    }
}

impl DataFetcher<Vec<DivinationCardElementData>, Error> for Fetcher {
    async fn fetch(&self) -> Result<Vec<DivinationCardElementData>, Error> {
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
}
