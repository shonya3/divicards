#![cfg(feature = "fetch")]

use crate::{
    act::ActArea, cards::CardsData, error::Error, loader::DataLoader, mapbosses::MapBoss,
    maps::Map, PoeData,
};
use async_trait::async_trait;

pub struct MapBossesLoader;
#[async_trait]
impl DataLoader<Vec<MapBoss>> for MapBossesLoader {
    fn filename(&self) -> &'static str {
        "mapBosses.json"
    }

    async fn fetch(&self) -> Result<Vec<MapBoss>, Error> {
        crate::mapbosses::fetch().await
    }
}

pub struct MapsLoader;
#[async_trait]
impl DataLoader<Vec<Map>> for MapsLoader {
    fn filename(&self) -> &'static str {
        "maps.json"
    }

    async fn fetch(&self) -> Result<Vec<Map>, Error> {
        crate::maps::fetch().await
    }
}

pub struct ActsLoader;
#[async_trait]
impl DataLoader<Vec<ActArea>> for ActsLoader {
    fn filename(&self) -> &'static str {
        "acts.json"
    }

    fn reload(&self) -> bool {
        false
    }

    async fn fetch(&self) -> Result<Vec<ActArea>, Error> {
        crate::act::fetch().await
    }
}

pub struct CardsLoader;
#[async_trait]
impl DataLoader<CardsData> for CardsLoader {
    fn filename(&self) -> &'static str {
        "cards.json"
    }

    async fn fetch(&self) -> Result<CardsData, Error> {
        crate::cards::fetch().await
    }
}

pub struct PoeDataLoader;
#[async_trait]
impl DataLoader<PoeData> for PoeDataLoader {
    fn filename(&self) -> &'static str {
        "poeData.json"
    }

    async fn fetch(&self) -> Result<PoeData, Error> {
        Ok(PoeData {
            acts: ActsLoader.load().await?,
            cards: CardsLoader.load().await?,
            maps: MapsLoader.load().await?,
            mapbosses: MapBossesLoader.load().await?,
        })
    }
}
