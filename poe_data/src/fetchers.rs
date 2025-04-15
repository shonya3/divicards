#![cfg(feature = "fs_cache_fetcher")]
use crate::{act::ActArea, cards::CardsData, error::Error, mapbosses::MapBoss, maps::Map, PoeData};
use fs_cache_fetcher::{Config, DataFetcher, Stale};
use std::time::Duration;

pub struct PoeDataFetcher {
    config: Config,
    acts: ActsFetcher,
    cards: CardsFetcher,
    maps: MapsFetcher,
    mapbosses: MapBossesFetcher,
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
impl DataFetcher for PoeDataFetcher {
    type Item = PoeData;
    type Error = Error;
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
    fn config(&self) -> &Config {
        &self.config
    }
    fn config_mut(&mut self) -> &mut Config {
        &mut self.config
    }
}

// 1. Map Bosses
pub struct MapBossesFetcher(Config);
impl Default for MapBossesFetcher {
    fn default() -> Self {
        Self(Config {
            save: true,
            filename: "mapBosses.json",
            stale: Stale::Never,
        })
    }
}
impl DataFetcher for MapBossesFetcher {
    type Item = Vec<MapBoss>;
    type Error = Error;
    async fn fetch(&self) -> Result<Vec<MapBoss>, Error> {
        crate::mapbosses::fetch().await
    }
    fn config(&self) -> &Config {
        &self.0
    }
    fn config_mut(&mut self) -> &mut Config {
        &mut self.0
    }
}

// 2. Maps
pub struct MapsFetcher(Config);
impl Default for MapsFetcher {
    fn default() -> Self {
        Self(Config {
            save: true,
            filename: "maps.json",
            stale: Stale::Never,
        })
    }
}
impl DataFetcher for MapsFetcher {
    type Item = Vec<Map>;
    type Error = Error;
    async fn fetch(&self) -> Result<Vec<Map>, Error> {
        Ok(crate::maps::fetch_maps().await?)
    }
    fn config(&self) -> &Config {
        &self.0
    }
    fn config_mut(&mut self) -> &mut Config {
        &mut self.0
    }
}

// 3. Acts
pub struct ActsFetcher(Config);
impl Default for ActsFetcher {
    fn default() -> Self {
        Self(Config {
            save: true,
            filename: "acts.json",
            stale: Stale::Never,
        })
    }
}
impl DataFetcher for ActsFetcher {
    type Item = Vec<ActArea>;
    type Error = Error;
    async fn fetch(&self) -> Result<Vec<ActArea>, Error> {
        crate::act::fetch().await
    }
    fn config(&self) -> &Config {
        &self.0
    }
    fn config_mut(&mut self) -> &mut Config {
        &mut self.0
    }
}

// 4. Cards
pub struct CardsFetcher(Config);
impl Default for CardsFetcher {
    fn default() -> Self {
        Self(Config {
            save: true,
            filename: "cards.json",
            stale: Stale::After(Duration::from_secs(86_400)),
        })
    }
}
impl DataFetcher for CardsFetcher {
    type Item = CardsData;
    type Error = Error;
    async fn fetch(&self) -> Result<CardsData, Error> {
        crate::cards::fetch::fetch().await
    }
    fn config(&self) -> &Config {
        &self.0
    }
    fn config_mut(&mut self) -> &mut Config {
        &mut self.0
    }
}
