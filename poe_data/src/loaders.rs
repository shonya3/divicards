#![cfg(feature = "fetch")]

use crate::{act::ActArea, cards::CardsData, error::Error, mapbosses::MapBoss, maps::Map, PoeData};
use async_trait::async_trait;
use loader::DataLoader;

pub struct MapBossesLoader;
#[async_trait]
impl DataLoader<Vec<MapBoss>, Error> for MapBossesLoader {
    fn filename(&self) -> &'static str {
        "mapBosses.json"
    }

    async fn fetch(&self) -> Result<Vec<MapBoss>, Error> {
        crate::mapbosses::fetch().await
    }

    fn reload(&self) -> bool {
        false
    }
}

pub struct MapsLoader;
#[async_trait]
impl DataLoader<Vec<Map>, Error> for MapsLoader {
    fn filename(&self) -> &'static str {
        "maps.json"
    }

    fn reload() -> bool {
        return false;
    }

    async fn fetch(&self) -> Result<Vec<Map>, Error> {
        crate::maps::fetch::fetch().await
    }
}

pub struct ActsLoader;
#[async_trait]
impl DataLoader<Vec<ActArea>, Error> for ActsLoader {
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
impl DataLoader<CardsData, Error> for CardsLoader {
    fn filename(&self) -> &'static str {
        "cards.json"
    }

    async fn fetch(&self) -> Result<CardsData, Error> {
        crate::cards::fetch().await
    }
}

pub struct PoeDataLoader;
#[async_trait]
impl DataLoader<PoeData, Error> for PoeDataLoader {
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
