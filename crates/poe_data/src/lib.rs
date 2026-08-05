//! Data extraction and fetching for the divicards ecosystem.
//!
//! Extracts Path of Exile data from game files and external sources
//! (poewiki, poe.ninja, Google Sheets, RePoE), producing the model types
//! defined in [`divcord::poe_data`] (and [`card_element`]). Also hosts the
//! in-process `DataFetcher`s and the `dump` CLI.

pub mod act;
pub mod cards;
pub mod fetchers;
pub mod game_files;
pub mod mapbosses;
pub mod maps;

pub use fetchers::PoeDataFetcher;
pub use game_files::{GameFiles, OpenedGameData, SharedGameData, open_game_data};

use reqwest::Client;
use std::sync::LazyLock;

/// Base URL for all poewiki.net Cargo and MediaWiki API calls.
pub(crate) const WIKI_API_URL: &str = "https://www.poewiki.net/w/api.php";

/// Single shared HTTP client reused across all requests.
pub(crate) static HTTP_CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .expect("Failed to create reqwest client")
});
