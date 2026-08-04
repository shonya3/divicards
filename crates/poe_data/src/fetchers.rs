//! Per-piece `DataFetcher`s that generate the data by running the extraction
//! pipeline in-process, caching each piece in the consumer's `data/` directory.
//!
//! Pieces mirror the `dump` CLI subcommands (`act`, `map`, `map-boss`,
//! `divination`):
//!
//! | Fetcher | Cache file | Staleness |
//! |---------|------------|-----------|
//! | [`ActsFetcher`] | `acts.json` | [`Stale::Never`] (game-file data) |
//! | [`MapsFetcher`] | `maps.json` | [`Stale::Never`] |
//! | [`MapBossesFetcher`] | `mapBosses.json` | [`Stale::Never`] |
//! | [`CardsFetcher`] | `cards.json` | [`Stale::After`] 1s (prices/weights are live) |
//! | [`CardElementsFetcher`] | `cardElementData.json` | [`Stale::After`] 1s |
//! | [`PoeDataFetcher`] | `poeData.json` | [`Stale::ReloadEveryTime`] (composition) |
//!
//! [`PoeDataFetcher`] composes the four pieces, and [`CardElementsFetcher`]
//! enriches from [`CardsFetcher`], so the divination pipeline (game files +
//! poe.ninja + wiki + sheets) never runs twice in one process.

use crate::{act, cards, mapbosses, maps, open_game_data};
use card_element::DivinationCardElementData;
use divcord::poe_data::{PoeData, act::ActArea, cards::CardsData, mapbosses::MapBoss, maps::Map};
use divi::TradeLeague;
use fs_cache_fetcher::{Config, DataFetcher, Stale};
use std::fmt::Display;

/// Error type for dump-backed fetchers.
#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Serde(serde_json::Error),
    Generation(anyhow::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => e.fmt(f),
            Error::Serde(e) => e.fmt(f),
            Error::Generation(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}

impl From<anyhow::Error> for Error {
    fn from(value: anyhow::Error) -> Self {
        Self::Generation(value)
    }
}

fn steam_path() -> std::path::PathBuf {
    crate::default_steam_path()
}

/// Campaign act areas (`acts.json`).
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
        let (fs, schemas) = open_game_data(&steam_path()).await?;
        eprintln!("extracting act areas...");
        let (areas, _) = act::extract_areas(&fs, &schemas)?;
        eprintln!("  {} areas extracted", areas.len());
        Ok(areas)
    }

    fn config(&self) -> &Config {
        &self.0
    }

    fn config_mut(&mut self) -> &mut Config {
        &mut self.0
    }
}

/// Atlas maps (`maps.json`).
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
        let (fs, schemas) = open_game_data(&steam_path()).await?;
        eprintln!("extracting maps...");
        let maps = maps::extract(&fs, &schemas).await?;
        eprintln!("  {} maps extracted", maps.len());
        Ok(maps)
    }

    fn config(&self) -> &Config {
        &self.0
    }

    fn config_mut(&mut self) -> &mut Config {
        &mut self.0
    }
}

/// Map bosses (`mapBosses.json`).
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
        let (fs, schemas) = open_game_data(&steam_path()).await?;
        eprintln!("extracting map bosses...");
        let bosses = mapbosses::extract(&fs, &schemas)?;
        eprintln!("  {} bosses extracted", bosses.len());
        Ok(bosses)
    }

    fn config(&self) -> &Config {
        &self.0
    }

    fn config_mut(&mut self) -> &mut Config {
        &mut self.0
    }
}

/// Divination cards with community weights, prices, and league info
/// (`cards.json`).
pub struct CardsFetcher(Config);

impl Default for CardsFetcher {
    fn default() -> Self {
        Self(Config {
            save: true,
            filename: "cards.json",
            stale: Stale::After(std::time::Duration::from_secs(1)),
        })
    }
}

impl DataFetcher for CardsFetcher {
    type Item = CardsData;
    type Error = Error;

    async fn fetch(&self) -> Result<CardsData, Error> {
        Ok(cards::extract_cards(&steam_path(), TradeLeague::Standard).await?)
    }

    fn config(&self) -> &Config {
        &self.0
    }

    fn config_mut(&mut self) -> &mut Config {
        &mut self.0
    }
}

/// Enriched card element data (`cardElementData.json`), built from the
/// [`CardsFetcher`] result so the divination pipeline runs at most once.
pub struct CardElementsFetcher(Config);

impl Default for CardElementsFetcher {
    fn default() -> Self {
        Self(Config {
            save: true,
            filename: "cardElementData.json",
            stale: Stale::After(std::time::Duration::from_secs(1)),
        })
    }
}

impl DataFetcher for CardElementsFetcher {
    type Item = Vec<DivinationCardElementData>;
    type Error = Error;

    async fn fetch(&self) -> Result<Vec<DivinationCardElementData>, Error> {
        let cards = CardsFetcher::default().load().await?;
        let cards: Vec<_> = cards.dict.values().cloned().collect();
        let (elements, _) = cards::card_element_data(&cards).await?;
        Ok(elements)
    }

    fn config(&self) -> &Config {
        &self.0
    }

    fn config_mut(&mut self) -> &mut Config {
        &mut self.0
    }
}

/// Full `PoeData` composition (`poeData.json`), assembled from the four piece
/// fetchers (each honoring its own staleness).
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

impl PoeDataFetcher {
    pub fn with_acts(mut self, acts: ActsFetcher) -> Self {
        self.acts = acts;
        self
    }

    pub fn with_cards(mut self, cards: CardsFetcher) -> Self {
        self.cards = cards;
        self
    }

    pub fn with_maps(mut self, maps: MapsFetcher) -> Self {
        self.maps = maps;
        self
    }

    pub fn with_mapbosses(mut self, mapbosses: MapBossesFetcher) -> Self {
        self.mapbosses = mapbosses;
        self
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
            self.mapbosses.load(),
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
