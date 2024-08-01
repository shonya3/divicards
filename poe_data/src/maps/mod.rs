#[cfg(feature = "fetch")]
pub mod fetch;
#[cfg(feature = "fetch")]
pub mod icon;
#[cfg(feature = "fetch")]
pub mod wiki;

#[cfg(feature = "fetch")]
pub use fetch::{fetch_maps, FetchMapsError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Map {
    pub name: String,
    pub tier: u32,
    pub available: bool,
    pub unique: bool,
    pub icon: String,
}

impl Map {
    pub fn level(&self) -> u32 {
        67 + self.tier
    }
}
