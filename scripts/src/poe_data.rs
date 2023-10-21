use serde::{Deserialize, Serialize};

use crate::{
    act::{ActArea, ActsLoader},
    cards::{CardsData, CardsLoader},
    error::Error,
    loader::DataLoader,
    mapbosses::{BossLoader, MapBoss},
    maps::{Map, MapLoader},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PoeData {
    pub acts: Vec<ActArea>,
    pub cards: CardsData,
    pub maps: Vec<Map>,
    pub mapbosses: Vec<MapBoss>,
}

impl PoeData {
    pub async fn load() -> Result<Self, Error> {
        Ok(Self {
            acts: ActsLoader::new(reqwest::Client::new()).load().await?,
            cards: CardsLoader::new(reqwest::Client::new()).load().await?,
            maps: MapLoader::new().load().await?,
            mapbosses: BossLoader::new().load().await?,
        })
    }
}
