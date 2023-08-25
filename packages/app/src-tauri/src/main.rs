#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::collections::HashMap;
use tokio::sync::Mutex;

use lib::{commands, paths, poe, prices::AppCardPrices};
use tauri::Manager;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    lib::dev::init_tracing();
    tracing::event!(tracing::Level::DEBUG, "app startup");

    let app_prices = Mutex::new(AppCardPrices::new(paths::appdata(), HashMap::new()));

    tauri::Builder::default()
        .setup(|app| {
            app.manage(app_prices);
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
            poe::auth::poe_auth,
            poe::auth::poe_logout,
            poe::stash::stashes,
            poe::stash::sample_from_tab
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
