#![cfg(feature = "fetch")]
use crate::{act::ActArea, cards::CardsData, error::Error, mapbosses::MapBoss, maps::Map, PoeData};
use fetcher::{Config, Stale, WithConfig};
use std::time::Duration;

pub struct PoeDataFetcher {
    config: Config,
    acts: ActsFetcher,
    cards: CardsFetcher,
    maps: MapsFetcher,
    mapbosses: MapBossesFetcher,
}

impl WithConfig for PoeDataFetcher {
    fn config(&self) -> &Config {
        &self.config
    }
}

impl Default for PoeDataFetcher {
    fn default() -> Self {
        Self {
            config: Config {
                save: true,
                filename: "poeData.json",
                stale: Stale::ReloadEveryTime,
            },
            acts: Default::default(),
            cards: Default::default(),
            maps: Default::default(),
            mapbosses: Default::default(),
        }
    }
}

impl fetcher::DataFetcher<PoeData, Error> for PoeDataFetcher {
    async fn fetch(&self) -> Result<PoeData, Error> {
        let (acts, cards, maps, mapbosses) = tokio::join!(
            self.acts.load(),
            self.cards.load(),
            self.maps.load(),
            self.mapbosses.load()
        );

        Ok(PoeData {
            acts: acts?,
            cards: cards?,
            maps: maps?,
            mapbosses: mapbosses?,
        })
    }
}

// 1. Map Bosses
pub struct MapBossesFetcher(Config);
impl WithConfig for MapBossesFetcher {
    fn config(&self) -> &Config {
        &self.0
    }
}
impl Default for MapBossesFetcher {
    fn default() -> Self {
        Self(Config {
            save: true,
            filename: "mapBosses.json",
            stale: Stale::Never,
        })
    }
}
impl fetcher::DataFetcher<Vec<MapBoss>, Error> for MapBossesFetcher {
    async fn fetch(&self) -> Result<Vec<MapBoss>, Error> {
        crate::mapbosses::fetch().await
    }
}

// 2. Maps
pub struct MapsFetcher(Config);
impl WithConfig for MapsFetcher {
    fn config(&self) -> &Config {
        &self.0
    }
}
impl Default for MapsFetcher {
    fn default() -> Self {
        Self(Config {
            save: true,
            filename: "maps.json",
            stale: Stale::Never,
        })
    }
}
impl fetcher::DataFetcher<Vec<Map>, Error> for MapsFetcher {
    async fn fetch(&self) -> Result<Vec<Map>, Error> {
        crate::maps::fetch::fetch().await
    }
}

// 3. Acts
pub struct ActsFetcher(Config);
impl WithConfig for ActsFetcher {
    fn config(&self) -> &Config {
        &self.0
    }
}
impl Default for ActsFetcher {
    fn default() -> Self {
        Self(Config {
            save: true,
            filename: "acts.json",
            stale: Stale::Never,
        })
    }
}
impl fetcher::DataFetcher<Vec<ActArea>, Error> for ActsFetcher {
    async fn fetch(&self) -> Result<Vec<ActArea>, Error> {
        crate::act::fetch().await
    }
}

// 4. Cards
pub struct CardsFetcher(Config);
impl WithConfig for CardsFetcher {
    fn config(&self) -> &Config {
        &self.0
    }
}
impl Default for CardsFetcher {
    fn default() -> Self {
        Self(Config {
            save: true,
            filename: "cards.json",
            stale: Stale::After(Duration::from_secs(86_400)),
        })
    }
}
impl fetcher::DataFetcher<CardsData, Error> for CardsFetcher {
    async fn fetch(&self) -> Result<CardsData, Error> {
        crate::cards::fetch::fetch().await
    }
}
