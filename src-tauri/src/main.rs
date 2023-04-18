#![allow(unused)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod file_card_data;
mod prices;
mod starter_map;

#[cfg(debug_assertions)]
use tauri::Manager;

#[tokio::main]
async fn main() {
    lib::dev::init_tracing();
    tracing::event!(tracing::Level::DEBUG, "app startup");
    match prices::update_prices_data().await {
        Ok(_) => tracing::event!(tracing::Level::DEBUG, "prices updated"),
        Err(err) => tracing::event!(tracing::Level::ERROR, "could not update prices {:?}", err),
    };

    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::read_polish_csv,
            commands::all_cards_price,
            commands::merge_csv,
            commands::update_prices,
            commands::give_record,
            commands::get_hashmap,
            commands::weight_records_to_csv,
            commands::create_file_card_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
