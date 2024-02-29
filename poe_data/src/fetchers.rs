#![cfg(feature = "fetch")]

use crate::{act::ActArea, cards::CardsData, error::Error, mapbosses::MapBoss, maps::Map, PoeData};
use fetcher::DataFetcher;

pub struct MapBossesFetcher;
impl DataFetcher<Vec<MapBoss>, Error> for MapBossesFetcher {
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

pub struct MapsFetcher;
impl DataFetcher<Vec<Map>, Error> for MapsFetcher {
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

pub struct ActsFetcher;
impl DataFetcher<Vec<ActArea>, Error> for ActsFetcher {
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

pub struct CardsFetcher;
impl DataFetcher<CardsData, Error> for CardsFetcher {
    fn filename() -> &'static str {
        "cards.json"
    }

    async fn fetch(&self) -> Result<CardsData, Error> {
        crate::cards::fetch::fetch().await
    }
}

pub struct PoeDataFetcher;
impl DataFetcher<PoeData, Error> for PoeDataFetcher {
    fn filename() -> &'static str {
        "poeData.json"
    }

    async fn fetch(&self) -> Result<PoeData, Error> {
        let (acts, cards, maps, mapbosses) = tokio::join!(
            ActsFetcher.load(),
            CardsFetcher.load(),
            MapsFetcher.load(),
            MapBossesFetcher.load()
        );

        Ok(PoeData {
            acts: acts?,
            cards: cards?,
            maps: maps?,
            mapbosses: mapbosses?,
        })
    }
}
