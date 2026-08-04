//! Data extraction and fetching for the divicards ecosystem.
//!
//! Extracts Path of Exile data from game files and external sources
//! (poewiki, poe.ninja, Google Sheets, RePoE), producing the model types
//! defined in [`divcord::poe_data`] (and [`card_element`]). Also hosts the
//! in-process `DataFetcher`s and the `dump` CLI.

pub mod act;
pub mod cards;
pub mod fetchers;
pub mod mapbosses;
pub mod maps;

use anyhow::{Context, Result};
pub use fetchers::PoeDataFetcher;
use poe_data_tools::{dat::schema::fetch_schema, fs::FS};
use reqwest::Client;
use std::path::{Path, PathBuf};
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

/// Default Path of Exile installation (Steam).
pub fn default_steam_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/home/shonya".into());
    PathBuf::from(format!(
        "{home}/.local/share/Steam/steamapps/common/Path of Exile"
    ))
}

/// Opens the game files and fetches the dat schemas (cached in the OS cache dir).
pub async fn open_game_data(
    steam: &Path,
) -> Result<(FS, poe_data_tools::dat::schema::SchemaCollection)> {
    let cache_dir = dirs::cache_dir().unwrap().join("poe_data_tools");
    eprintln!("opening game files...");
    let fs = FS::from_steam(steam.to_path_buf()).context("Failed to open game files")?;
    eprintln!("fetching schema...");
    let schemas = tokio::task::spawn_blocking(move || {
        fetch_schema(&cache_dir).context("Failed to fetch schema")
    })
    .await??;
    Ok((fs, schemas))
}
