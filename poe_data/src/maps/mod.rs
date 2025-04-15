#[cfg(feature = "fs_cache_fetcher")]
pub mod fetch;
#[cfg(feature = "fs_cache_fetcher")]
pub mod icon;
#[cfg(feature = "fs_cache_fetcher")]
pub mod wiki;

#[cfg(feature = "fs_cache_fetcher")]
pub use fetch::{fetch_maps, FetchMapsError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Map {
    pub name: String,
    pub tier: u32,
    pub available: bool,
    pub unique: bool,
    pub icon: String,
    pub slug: String,
}

impl Map {
    pub fn level(&self) -> u32 {
        67 + self.tier
    }
}
