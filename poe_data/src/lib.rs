pub mod act;
pub mod cards;
pub mod consts;

#[cfg(feature = "fetch")]
pub mod error;
#[cfg(feature = "fetch")]
pub mod fetchers;
pub mod league;
pub mod mapbosses;
pub mod maps;

use serde::{Deserialize, Serialize};

use self::{act::ActArea, cards::CardsData, mapbosses::MapBoss, maps::Map};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PoeData {
    pub acts: Vec<ActArea>,
    pub cards: CardsData,
    pub maps: Vec<Map>,
    pub mapbosses: Vec<MapBoss>,
}

impl PoeData {
    #[cfg(feature = "fetch")]
    pub async fn load() -> Result<Self, crate::error::Error> {
        use fetcher::DataFetcher;
        crate::fetchers::PoeDataFetcher::default().load().await
    }

    #[cfg(feature = "fetch")]
    pub fn filename() -> &'static str {
        use fetcher::WithConfig;
        crate::fetchers::PoeDataFetcher::default().config().filename
    }

    pub fn mapboss(&self, name: &str) -> Option<&MapBoss> {
        self.mapbosses
            .iter()
            .find(|map_boss| map_boss.name.to_lowercase() == name.to_lowercase())
    }

    pub fn act_area(&self, id: &str) -> Option<&ActArea> {
        self.act_area_id(&id).or_else(|| self.act_area_name(&id))
    }

    pub fn act_area_id(&self, id: &str) -> Option<&ActArea> {
        self.acts.iter().find(|act_area| act_area.id == id)
    }

    pub fn act_area_name(&self, name: &str) -> Option<&ActArea> {
        self.acts.iter().find(|act_area| act_area.name == name)
    }

    pub fn bosses_of_map(&self, map: &str) -> Vec<&MapBoss> {
        self.mapbosses
            .iter()
            .filter(|map_boss| {
                map_boss
                    .maps
                    .iter()
                    .any(|m| m.to_lowercase() == map.to_lowercase())
            })
            .collect()
    }
}
