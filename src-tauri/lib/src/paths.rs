use std::path::PathBuf;

pub fn prices() -> PathBuf {
    let mut path = appdata();
    path.push("prices.json");
    path
}
pub fn appdata() -> PathBuf {
    let mut path = tauri::api::path::config_dir().unwrap();
    path.push("divicards");

    if !path.exists() {
        std::fs::create_dir(&path).expect("Error on appdata dir creation");
    }

    path
}
