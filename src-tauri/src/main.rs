#![allow(unused)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use divi::{League, Prices};
use lib::{commands, paths, prices};
#[cfg(debug_assertions)]
use tauri::Manager;

#[tokio::main]
async fn main() {
    lib::dev::init_tracing();
    tracing::event!(tracing::Level::DEBUG, "app startup");
    match prices::update(&paths::prices(), League::Crucible).await {
        Ok(prices) => {
            tracing::event!(tracing::Level::DEBUG, "prices updated");
        }
        Err(err) => {
            tracing::event!(tracing::Level::ERROR, "could not update prices {:?}", err);
        }
    };

    let prices = prices::prices(League::Crucible).await;

    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }
            app.manage(prices);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::sample,
            commands::chaos,
            commands::merge,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
