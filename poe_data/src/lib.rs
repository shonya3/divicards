pub mod act;
pub mod cards;
pub mod consts;

#[cfg(feature = "fetch")]
pub mod error;
pub mod league;
#[cfg(feature = "fetch")]
// pub mod loader;
// #[cfg(feature = "fetch")]
pub mod loaders;
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
        use loader::DataLoader;
        crate::loaders::PoeDataLoader.load().await
    }
}
