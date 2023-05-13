use std::path::PathBuf;

use divi::League;

pub fn prices(league: &League) -> PathBuf {
    appdata().join(format!("{}-prices.json", { league }))
}

pub fn appdata() -> PathBuf {
    let mut path = tauri::api::path::config_dir().unwrap();
    path.push("divicards");

    if !path.exists() {
        std::fs::create_dir(&path).expect("Error on appdata dir creation");
    }

    path
}
