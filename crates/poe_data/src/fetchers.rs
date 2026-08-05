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
//! poe.ninja + wiki + sheets) never runs twice in one process. All pieces
//! read game files through one shared handle (serial reads), so they run one
//! after another; the cards pipeline additionally does network calls after
//! its game-file reads.
//!
//! All fetchers read game files from the configured [`GameFiles`] source
//! (Steam install, GGG CDN or GGPK file), opened once per process via
//! [`open_game_data`] and shared across pieces.

use crate::{GameFiles, act, cards, log, mapbosses, maps, open_game_data};
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

impl From<tokio::task::JoinError> for Error {
    fn from(value: tokio::task::JoinError) -> Self {
        Self::Generation(anyhow::anyhow!("extraction task panicked: {value}"))
    }
}

/// Campaign act areas (`acts.json`).
pub struct ActsFetcher {
    config: Config,
    source: GameFiles,
}

impl Default for ActsFetcher {
    fn default() -> Self {
        Self {
            config: Config {
                save: true,
                filename: "acts.json",
                stale: Stale::Never,
            },
            source: GameFiles::default(),
        }
    }
}

impl ActsFetcher {
    pub fn with_source(mut self, source: GameFiles) -> Self {
        self.source = source;
        self
    }
}

impl DataFetcher for ActsFetcher {
    type Item = Vec<ActArea>;
    type Error = Error;

    async fn fetch(&self) -> Result<Vec<ActArea>, Error> {
        let opened = open_game_data(&self.source).await?;
        // Game-file reads use blocking IO (the CDN backend in particular), so
        // run the extraction on the blocking pool.
        tokio::task::spawn_blocking(move || {
            let opened = opened.lock().unwrap();
            eprintln!("{}", log::ColoredLabel::ActAreas);
            eprintln!("extracting act areas...");
            let (areas, _) = act::extract_areas(&opened.fs, &opened.schemas)?;
            eprintln!("  {} areas extracted", areas.len());
            Ok::<_, Error>(areas)
        })
        .await?
    }

    fn config(&self) -> &Config {
        &self.config
    }

    fn config_mut(&mut self) -> &mut Config {
        &mut self.config
    }
}

/// Atlas maps (`maps.json`).
pub struct MapsFetcher {
    config: Config,
    source: GameFiles,
}

impl Default for MapsFetcher {
    fn default() -> Self {
        Self {
            config: Config {
                save: true,
                filename: "maps.json",
                stale: Stale::Never,
            },
            source: GameFiles::default(),
        }
    }
}

impl MapsFetcher {
    pub fn with_source(mut self, source: GameFiles) -> Self {
        self.source = source;
        self
    }
}

impl DataFetcher for MapsFetcher {
    type Item = Vec<Map>;
    type Error = Error;

    async fn fetch(&self) -> Result<Vec<Map>, Error> {
        let opened = open_game_data(&self.source).await?;
        tokio::task::spawn_blocking(move || {
            let opened = opened.lock().unwrap();
            eprintln!("{}", log::ColoredLabel::Maps);
            eprintln!("extracting maps...");
            let maps = maps::extract(&opened.fs, &opened.schemas)?;
            eprintln!("  {} maps extracted", maps.len());
            Ok::<_, Error>(maps)
        })
        .await?
    }

    fn config(&self) -> &Config {
        &self.config
    }

    fn config_mut(&mut self) -> &mut Config {
        &mut self.config
    }
}

/// Map bosses (`mapBosses.json`).
pub struct MapBossesFetcher {
    config: Config,
    source: GameFiles,
}

impl Default for MapBossesFetcher {
    fn default() -> Self {
        Self {
            config: Config {
                save: true,
                filename: "mapBosses.json",
                stale: Stale::Never,
            },
            source: GameFiles::default(),
        }
    }
}

impl MapBossesFetcher {
    pub fn with_source(mut self, source: GameFiles) -> Self {
        self.source = source;
        self
    }
}

impl DataFetcher for MapBossesFetcher {
    type Item = Vec<MapBoss>;
    type Error = Error;

    async fn fetch(&self) -> Result<Vec<MapBoss>, Error> {
        let opened = open_game_data(&self.source).await?;
        tokio::task::spawn_blocking(move || {
            let opened = opened.lock().unwrap();
            eprintln!("{}", log::ColoredLabel::MapBosses);
            eprintln!("extracting map bosses...");
            let bosses = mapbosses::extract(&opened.fs, &opened.schemas)?;
            eprintln!("  {} bosses extracted", bosses.len());
            Ok::<_, Error>(bosses)
        })
        .await?
    }

    fn config(&self) -> &Config {
        &self.config
    }

    fn config_mut(&mut self) -> &mut Config {
        &mut self.config
    }
}

/// Divination cards with community weights, prices, and league info
/// (`cards.json`).
pub struct CardsFetcher {
    config: Config,
    source: GameFiles,
}

impl Default for CardsFetcher {
    fn default() -> Self {
        Self {
            config: Config {
                save: true,
                filename: "cards.json",
                stale: Stale::After(std::time::Duration::from_secs(1)),
            },
            source: GameFiles::default(),
        }
    }
}

impl CardsFetcher {
    pub fn with_source(mut self, source: GameFiles) -> Self {
        self.source = source;
        self
    }
}

impl DataFetcher for CardsFetcher {
    type Item = CardsData;
    type Error = Error;

    async fn fetch(&self) -> Result<CardsData, Error> {
        Ok(cards::extract_cards(&self.source, TradeLeague::Standard).await?)
    }

    fn config(&self) -> &Config {
        &self.config
    }

    fn config_mut(&mut self) -> &mut Config {
        &mut self.config
    }
}

/// Enriched card element data (`cardElementData.json`), built from the
/// [`CardsFetcher`] result so the divination pipeline runs at most once.
pub struct CardElementsFetcher {
    config: Config,
}

impl Default for CardElementsFetcher {
    fn default() -> Self {
        Self {
            config: Config {
                save: true,
                filename: "cardElementData.json",
                stale: Stale::After(std::time::Duration::from_secs(1)),
            },
        }
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
        &self.config
    }

    fn config_mut(&mut self) -> &mut Config {
        &mut self.config
    }
}

/// Full `PoeData` composition (`poeData.json`), assembled from the four piece
/// fetchers (each honoring its own staleness).
pub struct PoeDataFetcher {
    config: Config,
    source: GameFiles,
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
            source: GameFiles::default(),
            acts: Default::default(),
            cards: Default::default(),
            maps: Default::default(),
            mapbosses: Default::default(),
        }
    }
}

impl PoeDataFetcher {
    /// Sets the game files source for all four pieces.
    pub fn with_source(mut self, source: GameFiles) -> Self {
        self.source = source.clone();
        self.acts = self.acts.with_source(source.clone());
        self.cards = self.cards.with_source(source.clone());
        self.maps = self.maps.with_source(source.clone());
        self.mapbosses = self.mapbosses.with_source(source);
        self
    }

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
        let started = std::time::Instant::now();
        // All four pieces read game files through one shared `FS` handle, so
        // they run one after another. Reads are serialized by the mutex
        // around the handle (needed because the GGPK backend keeps a single
        // file cursor; steam/cdn reads would be safe concurrently, but share
        // the same handle). The cards pipeline only does network calls
        // (poe.ninja, wiki, spreadsheets) after its game-file reads, so
        // there is nothing to overlap — no `join!` here.
        let acts = self.acts.load().await?;
        eprintln!("{} {} areas -> acts.json", log::ColoredLabel::ActAreas, acts.len());
        let maps = self.maps.load().await?;
        eprintln!("{} {} maps -> maps.json", log::ColoredLabel::Maps, maps.len());
        let mapbosses = self.mapbosses.load().await?;
        eprintln!("{} {} bosses -> mapBosses.json", log::ColoredLabel::MapBosses, mapbosses.len());
        let cards = self.cards.load().await?;
        eprintln!("{} {} cards -> cards.json", log::ColoredLabel::Cards, cards.dict.len());

        eprintln!("done in {:.1}s", started.elapsed().as_secs_f64());
        Ok(PoeData {
            acts,
            cards,
            maps,
            mapbosses,
        })
    }

    fn config(&self) -> &Config {
        &self.config
    }

    fn config_mut(&mut self) -> &mut Config {
        &mut self.config
    }
}
