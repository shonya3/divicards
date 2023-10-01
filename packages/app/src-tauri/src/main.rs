#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::collections::HashMap;
use tokio::sync::Mutex;

use lib::{commands, google, paths, poe, prices::AppCardPrices, version::AppVersion};
use tauri::Manager;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    lib::dev::init_tracing();
    tracing::event!(tracing::Level::DEBUG, "app startup");

    let app_prices = Mutex::new(AppCardPrices::new(paths::appdata(), HashMap::new()));

    tauri::Builder::default()
        .setup(|app| {
            let app_version = AppVersion(app.config().package.version.clone().unwrap());
            app.manage(app_prices);
            app.manage(app_version);
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::sample,
            commands::merge,
            commands::open_url,
            commands::sample_into_csv,
            poe::auth::poe_auth,
            poe::auth::poe_logout,
            poe::stash::stashes,
            poe::stash::sample_from_tab,
            google::auth::google_auth,
            google::auth::google_logout,
            google::auth::google_identity,
            google::sheets::new_sheet_with_sample,
            google::sheets::read_sheet,
            google::sheets::read_batch
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
