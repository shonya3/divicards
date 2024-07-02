use crate::drop_level;
use crate::{reward::reward_to_html, DivinationCardElementData, Error};
use fetcher::{Config, DataFetcher, Stale};
use poe::TradeLeague;
use poe_data::fetchers::CardsFetcher;

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

impl DataFetcher for Fetcher {
    type Item = Vec<DivinationCardElementData>;
    type Error = Error;

    async fn fetch(&self) -> Result<Vec<DivinationCardElementData>, Error> {
        let league = TradeLeague::default();
        let cards_fetcher = CardsFetcher::default();
        let (ninja_data, cards) =
            tokio::join!(ninja::fetch_card_data(&league), cards_fetcher.load());
        let ninja_data = ninja_data?;
        let cards = cards?;

        let v: Vec<DivinationCardElementData> = ninja_data
            .into_iter()
            .map(|data| {
                let mut fl = data.flavour_text;
                if fl.starts_with("<size:") {
                    fl = fl[10..fl.len() - 1].to_string();
                }

                if fl.starts_with("<smaller>{") {
                    fl = fl[10..fl.len() - 1].to_string();
                }

                let card = cards.0.get(&data.name);
                DivinationCardElementData {
                    name: data.name,
                    art_filename: data.art_filename,
                    flavour_text: fl,
                    stack_size: data.stack_size,
                    reward_html: reward_to_html(&data.explicit_modifiers[0].text),
                    drop_level: drop_level::extract_drop_level(card),
                }
            })
            .collect();

        Ok(v)
    }

    fn config(&self) -> &Config {
        &self.0
    }
    fn config_mut(&mut self) -> &mut Config {
        &mut self.0
    }
}
