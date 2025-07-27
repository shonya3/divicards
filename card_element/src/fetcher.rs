use crate::drop_level;
use crate::{
    reward::reward_to_html, unique::extract_unique_name_from_mod, unique::UniqueReward,
    uniques_fetcher::UniquesFetcher, DivinationCardElementData, Error,
};
use fs_cache_fetcher::{Config, DataFetcher, Stale};
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
        let uniques_fetcher = UniquesFetcher::default();
        let (ninja_data, cards, uniques_data) = tokio::join!(
            ninja::fetch_card_data(&league),
            cards_fetcher.load(),
            uniques_fetcher.load()
        );
        let ninja_data = ninja_data?;
        let cards = cards?;
        let uniques_data = uniques_data?;

        // Build a lookup map for faster access.
        let uniques_map = crate::unique::build_uniques_map(&uniques_data);

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

                // Extract unique item name from any of the explicit modifiers.
                let unique_name = data
                    .explicit_modifiers
                    .iter()
                    .find_map(|modifier| extract_unique_name_from_mod(&modifier.text));

                // Look up the item class and create the UniqueReward struct.
                let unique = unique_name.and_then(|name| {
                    uniques_map.get(&name).map(|info| UniqueReward {
                        name: info.name.clone(),
                        item_class: info.item_class.clone(),
                    })
                });

                let card = cards.0.get(&data.name);
                DivinationCardElementData {
                    slug: slug::slugify(&data.name),
                    name: data.name,
                    art_filename: data.art_filename,
                    flavour_text: fl,
                    stack_size: data.stack_size,
                    reward_html: reward_to_html(&data.explicit_modifiers[0].text),
                    drop_level: drop_level::extract_drop_level(card),
                    unique,
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
