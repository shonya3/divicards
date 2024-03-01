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
}
