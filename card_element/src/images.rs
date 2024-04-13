use std::{env, fs::File};

use divi::{prices::NinjaCardData, TradeLeague};
use tokio::task::spawn_blocking;

pub const POE_CDN_CARDS: &'static str = "https://web.poecdn.com/image/divination-card/";

pub async fn download_card_images() -> Result<(), divi::error::Error> {
    let data = NinjaCardData::fetch(&TradeLeague::Standard).await?;

    let cards_images_dir = env::current_dir()
        .unwrap()
        .join("public")
        .join("images")
        .join("cards");

    if !cards_images_dir.exists() {
        std::fs::create_dir_all(&cards_images_dir).unwrap();
    }

    spawn_blocking(move || {
        for card in data {
            let url = format!("{POE_CDN_CARDS}{}.png", card.art_filename);
            let filename = format!("{}.png", card.art_filename);
            let path = cards_images_dir.join(filename);
            let mut file = File::create(path).unwrap();
            let _ = reqwest::blocking::get(url)
                .unwrap()
                .copy_to(&mut file)
                .unwrap();
        }
    });

    Ok(())
}

pub fn download_images(urls: Vec<String>) {
    let act_images_dir = env::current_dir()
        .unwrap()
        .join("public")
        .join("images")
        .join("acts");

    if !act_images_dir.exists() {
        std::fs::create_dir_all(&act_images_dir).unwrap();
    }

    tokio::task::spawn_blocking(move || {
        for url in urls {
            let (_, filename) = url.rsplit_once("/").unwrap();
            let path = act_images_dir.join(filename);
            let mut file = File::create(path).unwrap();
            let _ = reqwest::blocking::get(url)
                .unwrap()
                .copy_to(&mut file)
                .unwrap();
        }
    });
}
