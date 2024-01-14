use crate::error::Error;
use std::path::PathBuf;

pub fn appdata_dir() -> Result<PathBuf, Error> {
    let Some(path) = tauri::api::path::config_dir() else {
        return Err(Error::ConfigDirNotExists);
    };

    if !path.exists() {
        std::fs::create_dir(&path)?;
    }

    Ok(path)
}
