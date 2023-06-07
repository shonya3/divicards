#![allow(unused)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use divi::{Prices, TradeLeague};
use lib::{
    commands,
    discord::{self, DiscordProvider},
    google, paths, poe, prices,
};
use tauri::Manager;

#[tokio::main]
async fn main() {
    lib::dev::init_tracing();
    tracing::event!(tracing::Level::DEBUG, "app startup");
    match prices::update(&TradeLeague::Crucible).await {
        Ok(prices) => {
            tracing::event!(tracing::Level::DEBUG, "prices updated");
        }
        Err(err) => {
            tracing::event!(tracing::Level::ERROR, "could not update prices {:?}", err);
        }
    };

    let prices = prices::prices(&TradeLeague::Crucible).await;

    tauri::Builder::default()
        .setup(|app| {
            app.manage(prices);
            let v = app.config().package.version.clone().unwrap();
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::sample,
            commands::chaos,
            commands::merge,
            commands::league,
            commands::sample_cards,
            discord::discord_auth,
            discord::discord_authenticated,
            discord::discord_identity,
            discord::discord_logout,
            google::google_auth,
            google::google_identity,
            poe::poe_auth,
            poe::poe_logout,
            poe::stashes,
            poe::stash
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
