#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tokio::sync::Mutex;

use lib::{commands, google, poe, prices::AppCardPrices, version::AppVersion};
use tauri::Manager;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    lib::dev::init_tracing();
    tracing::event!(tracing::Level::DEBUG, "app startup");

    tauri::Builder::default()
        .setup(|app| {
            println!("{}", app.path().app_local_data_dir().unwrap().display());
            let app_dir = app
                .path()
                .app_local_data_dir()
                .map_err(|_| lib::error::Error::ConfigDirNotExists)
                .unwrap();
            let app_prices = Mutex::new(AppCardPrices::new(app_dir).unwrap());
            let app_version = AppVersion(app.config().version.clone().unwrap());
            let google_token_state = google::AccessTokenState(Mutex::new(None));
            app.manage(app_prices);
            app.manage(app_version);
            app.manage(google_token_state);
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            #[cfg(desktop)]
            app.handle()
                .plugin(tauri_plugin_updater::Builder::new().build())?;
            app.handle().plugin(tauri_plugin_process::init())?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::version,
            commands::sample,
            commands::merge,
            commands::open_url,
            commands::sample_into_csv,
            lib::prices::map_prices,
            lib::prices::currency_prices,
            lib::prices::fragment_prices,
            lib::prices::essence_prices,
            lib::prices::gem_prices,
            lib::prices::oil_prices,
            lib::prices::incubator_prices,
            lib::prices::fossil_prices,
            lib::prices::resonator_prices,
            lib::prices::delirium_orb_prices,
            lib::prices::vial_prices,
            lib::prices::divination_card_prices,
            lib::prices::ninja_dense_overviews_raw,
            lib::prices::set_gem_prices_cache_ttl_minutes,
            poe::auth::poe_auth,
            poe::auth::poe_logout,
            poe::stash::stashes,
            poe::stash::sample_from_tab,
            poe::stash::tab_with_items,
            google::auth::google_auth,
            google::auth::google_logout,
            google::auth::google_identity,
            google::sheets::new_sheet_with_sample,
            google::sheets::read_sheet,
            google::sheets::read_batch,
            poe::stash::sample_from_tab_with_items,
            poe::stash::tab,
            poe::stash::extract_cards
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
