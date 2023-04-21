use shared::{error::Error, types::record};

pub async fn div_prices() -> Result<String, Error> {
    let path = prices_path();
    let json = match std::fs::read_to_string(&path) {
        Ok(json) => json,
        Err(_) => {
            let json = record::fetch_div_prices().await?;
            std::fs::write(path, &json)?;
            json
        }
    };

    Ok(json)
}

pub async fn update_prices_data() -> Result<(), Error> {
    let path = prices_path();

    let json = record::fetch_div_prices().await?;
    std::fs::write(path, &json)?;
    Ok(())
}

fn get_appdata_dir() -> std::path::PathBuf {
    let mut path = tauri::api::path::config_dir().unwrap();
    path.push("divicards");

    if !path.exists() {
        std::fs::create_dir(&path).expect("Error on appdata dir creation");
    }

    path
}

fn prices_path() -> std::path::PathBuf {
    let mut path = get_appdata_dir();
    path.push("div-prices.json");
    path
}
