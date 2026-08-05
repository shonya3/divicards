//! Where the Path of Exile game files come from, and the process-wide cache
//! of opened game data.
//!
//! # Concurrency
//!
//! The extraction pieces all read game files through one shared handle, so
//! they run one after another:
//!
//! - `FS` is blocking-only and not `Sync` (the GGPK backend keeps a single
//!   file cursor in a `RefCell`), so it can't be shared across threads or
//!   stored in a `static` directly. `SharedGameData` (`Arc<Mutex<..>>`) makes
//!   it shareable: the `Arc` gives shared ownership, the `Mutex` provides
//!   `Sync` and serializes reads. Steam/CDN reads would in principle be safe
//!   concurrently, but they go through the same handle and are serialized too.
//! - Blocking IO never runs on the async executor: reads happen in
//!   `tokio::task::spawn_blocking` tasks (the CDN backend uses
//!   `reqwest::blocking`, which panics inside an async context), with the
//!   mutex guard dropped before any `await`.
//! - Game data is opened at most once per source per process (see
//!   [`open_game_data`]); the download-heavy CDN open happens only on the
//!   first access, subsequent fetchers just share the cached handle.
//!
//! # Usage
//!
//! ```no_run
//! use anyhow::Result;
//! use poe_data::{GameFiles, open_game_data};
//! use poe_data_tools::fs::FileSystem as _;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! // Defaults to the Steam install; "cdn:1" (latest PoE 1 patch) and
//! // "ggpk:/path/to/Content.ggpk" are also valid.
//! let source: GameFiles = "steam".parse().map_err(anyhow::Error::msg)?;
//! let opened = open_game_data(&source).await?;
//!
//! // Reads are blocking, so they run on the blocking pool, holding the
//! // mutex guard only for the read itself.
//! let path = "Metadata/Maps/Atlas.datc64".to_owned();
//! let bytes = tokio::task::spawn_blocking(move || {
//!     let opened = opened.lock().unwrap();
//!     opened.fs.read(&path)
//! })
//! .await??;
//! println!("atlas is {} bytes", bytes.len());
//! # Ok(())
//! # }
//! ```

use anyhow::{Context, Result};
use poe_data_tools::{
    dat::schema::fetch_schema,
    fs::{FS, cdn::cdn_base_url},
};
use std::{
    collections::HashMap,
    fmt::Display,
    path::PathBuf,
    sync::{Arc, LazyLock, Mutex},
};

/// Default Path of Exile installation (Steam): the standard Steam library
/// location per platform. Only a fallback — an explicit path always wins.
pub(crate) fn default_steam_path() -> PathBuf {
    let home = dirs::home_dir().expect("home directory not found");
    #[cfg(target_os = "windows")]
    let steam = PathBuf::from(r"C:\Program Files (x86)\Steam");
    #[cfg(target_os = "macos")]
    let steam = home.join("Library/Application Support/Steam");
    #[cfg(target_os = "linux")]
    let steam = home.join(".local/share/Steam");
    steam.join("steamapps/common/Path of Exile")
}

/// Where the Path of Exile game files are read from.
///
/// Parsed from a `kind:value` string via [`std::str::FromStr`] (and printed back with
/// [`std::fmt::Display`]):
///
/// | Kind | Syntax | Meaning |
/// |------|--------|---------|
/// | `steam` | `steam[:path]` | Locally installed game (Steam or standalone folder); without a path, the default Steam install is used |
/// | `cdn` | `cdn[:patch]` | GGG CDN; `cdn:1` is the latest PoE 1 patch, `cdn:2` the latest PoE 2 patch (the only bare version numbers — pinned patches need the full `major.minor.patch`, e.g. `cdn:3.29.0`) |
/// | `ggpk` | `ggpk:path` | A local `Content.ggpk` bundle file (path is required) |
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameFiles {
    /// Locally installed game (Steam or standalone folder).
    Steam(PathBuf),
    /// GGG CDN: `"1"` for the latest PoE 1 patch, or a specific patch like `"3.29.0"`.
    Cdn(String),
    /// Standalone GGPK bundle file.
    Ggpk(PathBuf),
}

impl Default for GameFiles {
    fn default() -> Self {
        Self::Steam(default_steam_path())
    }
}

impl Display for GameFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Steam(path) => write!(f, "steam:{}", path.display()),
            Self::Cdn(version) => write!(f, "cdn:{version}"),
            Self::Ggpk(path) => write!(f, "ggpk:{}", path.display()),
        }
    }
}

impl std::str::FromStr for GameFiles {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (kind, value) = s
            .split_once(':')
            .map(|(k, v)| (k, Some(v)))
            .unwrap_or((s, None));
        match kind {
            "steam" => Ok(Self::Steam(
                value.map(PathBuf::from).unwrap_or_else(default_steam_path),
            )),
            "cdn" => Ok(Self::Cdn(value.unwrap_or("1").to_string())),
            "ggpk" => {
                Ok(Self::Ggpk(value.map(PathBuf::from).ok_or(
                    "ggpk requires a path, e.g. `ggpk:/path/to/Content.ggpk`",
                )?))
            }
            other => Err(format!(
                "unknown game files source `{other}` (expected `steam`, `cdn` or `ggpk`)"
            )),
        }
    }
}

impl GameFiles {
    /// Open the game files. For [`GameFiles::Cdn`] the patch files are
    /// downloaded on first access and cached in the OS cache directory.
    pub fn open(&self) -> Result<FS> {
        let cache_dir = dirs::cache_dir().unwrap().join("poe_data_tools");
        match self {
            Self::Steam(path) => FS::from_steam(path.clone()).map_err(anyhow::Error::msg),
            Self::Cdn(version) => {
                let base_url =
                    cdn_base_url(&cache_dir, version).context("Failed to resolve CDN URL")?;
                FS::from_cdn(&base_url, &cache_dir).map_err(anyhow::Error::msg)
            }
            Self::Ggpk(path) => FS::from_ggpk(path).map_err(anyhow::Error::msg),
        }
    }
}

/// Opened game files and dat schemas, shared by all fetchers in the process.
///
/// The game files themselves can't be shared across threads (`FS` is not
/// `Sync`), so callers lock the mutex while reading, and drop the guard
/// before any long-running network work.
pub struct OpenedGameData {
    pub fs: FS,
    pub schemas: poe_data_tools::dat::schema::SchemaCollection,
}

pub type SharedGameData = Arc<Mutex<OpenedGameData>>;

static OPENED_GAME_DATA: LazyLock<Mutex<HashMap<String, SharedGameData>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Opens the game files and fetches the dat schemas (cached in the OS cache dir).
///
/// The opened game data is cached per [`GameFiles`] source, so concurrent
/// fetchers share a single open instead of opening the game files each time.
pub async fn open_game_data(source: &GameFiles) -> Result<SharedGameData> {
    let key = source.to_string();
    if let Some(opened) = OPENED_GAME_DATA.lock().unwrap().get(&key) {
        return Ok(opened.clone());
    }

    let cache_dir = dirs::cache_dir().unwrap().join("poe_data_tools");
    let source = source.clone();
    eprintln!("opening game files...");
    eprintln!("fetching schema...");
    // Both opening and schema fetch do blocking work (reqwest::blocking in the
    // CDN backend panics inside an async context), so run them on the blocking pool.
    let (fs, schemas) = tokio::task::spawn_blocking(move || {
        let fs = source.open().context("Failed to open game files")?;
        let schemas = fetch_schema(&cache_dir).context("Failed to fetch schema")?;
        Ok::<_, anyhow::Error>((fs, schemas))
    })
    .await??;

    let opened = std::sync::Arc::new(std::sync::Mutex::new(OpenedGameData { fs, schemas }));
    OPENED_GAME_DATA.lock().unwrap().insert(key, opened.clone());
    Ok(opened)
}
