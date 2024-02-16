#![cfg(feature = "fetch")]

use crate::{act::ActArea, cards::CardsData, error::Error, mapbosses::MapBoss, maps::Map, PoeData};
use loader::DataLoader;

pub struct MapBossesLoader;
impl DataLoader<Vec<MapBoss>, Error> for MapBossesLoader {
    fn filename() -> &'static str {
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
impl DataLoader<Vec<Map>, Error> for MapsLoader {
    fn filename() -> &'static str {
        "maps.json"
    }

    fn reload(&self) -> bool {
        false
    }

    async fn fetch(&self) -> Result<Vec<Map>, Error> {
        crate::maps::fetch::fetch().await
    }
}

pub struct ActsLoader;
impl DataLoader<Vec<ActArea>, Error> for ActsLoader {
    fn filename() -> &'static str {
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
impl DataLoader<CardsData, Error> for CardsLoader {
    fn filename() -> &'static str {
        "cards.json"
    }

    async fn fetch(&self) -> Result<CardsData, Error> {
        crate::cards::fetch::fetch().await
    }
}

pub struct PoeDataLoader;
impl DataLoader<PoeData, Error> for PoeDataLoader {
    fn filename() -> &'static str {
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
