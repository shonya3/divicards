use std::{collections::HashMap, env, fs::File};

use divi::{
    league::TradeLeague,
    prices::{NinjaCardData, Prices},
    sample::{DivinationCardsSample, SampleData},
};
use googlesheets::sheet::Credential;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::task::spawn_blocking;

use crate::{
    consts::{WEIGHT_RANGES, WIKI_API_URL},
    error::Error,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WikiCard {
    pub name: String,
    #[serde(alias = "drop level")]
    pub min_level: Option<u32>,
    #[serde(alias = "drop areas")]
    pub drop_areas: Option<Vec<String>>,
    #[serde(alias = "drop monsters")]
    pub drop_monsters: Option<Vec<String>>,
    #[serde(alias = "drop level maximum")]
    pub max_level: Option<u32>,
}

pub async fn load_wiki() -> Result<HashMap<String, WikiCard>, Error> {
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct WikiCardsResponse {
        pub cargoquery: Vec<WikiCardWrapper>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct WikiCardRaw {
        name: String,
        #[serde(alias = "drop level")]
        min_level: Option<String>,
        #[serde(alias = "drop areas")]
        pub drop_areas: Option<String>,
        #[serde(alias = "drop monsters")]
        pub drop_monsters: Option<String>,
        #[serde(alias = "drop level maximum")]
        pub max_level: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct WikiCardWrapper {
        title: WikiCardRaw,
    }

    let url = format!("{WIKI_API_URL}?action=cargoquery&format=json&smaxage=1&maxage=1&tables=items&limit=500&fields=items.name,items.drop_level,items.drop_level_maximum,items.drop_areas,items.drop_monsters&where=items.class_id='DivinationCard'");

    let response = Client::new().get(url).send().await?;
    let wiki_maps: WikiCardsResponse = response.json().await?;

    let map: HashMap<String, WikiCard> = wiki_maps
        .cargoquery
        .into_iter()
        .map(|wrapper| {
            let raw = wrapper.title;
            let card = WikiCard {
                name: raw.name.clone(),
                min_level: raw.min_level.map(|s| s.parse().unwrap()),
                drop_areas: raw
                    .drop_areas
                    .map(|s| s.split(",").into_iter().map(|s| s.to_string()).collect()),
                drop_monsters: raw
                    .drop_monsters
                    .map(|s| s.split(",").into_iter().map(|s| s.to_string()).collect()),
                max_level: raw.max_level.map(|s| s.parse().unwrap()),
            };
            (raw.name.clone(), card)
        })
        .collect();

    Ok(map)
}

pub async fn load_total_sample(
    api_key: String,
    prices: Option<Prices>,
) -> Result<DivinationCardsSample, Error> {
    let batch_read = googlesheets::read_batch(
        "1NDTZqLcwrKjR3CflLU7B7IGJtzynNoWTe7AVQ-gUA-c",
        WEIGHT_RANGES,
        Credential::ApiKey(api_key),
    )
    .await?;
    let data = SampleData::try_from(batch_read)?;
    let sample = DivinationCardsSample::create(data, prices)?;
    Ok(sample)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardData {
    pub name: String,
    pub min_level: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_level: Option<u32>,
    pub weight: Option<f32>,
    pub price: Option<f32>,
}

// #[tokio::test]
// async fn write_card_data() {
//     dotenv::dotenv().ok();
//     let api_key = std::env::var("GOOGLE_API_KEY").expect("No google api key");
//     let prices = Prices::fetch(&divi::league::TradeLeague::Ancestor)
//         .await
//         .unwrap();
//     let sample = load_total_sample(api_key, Some(prices)).await.unwrap();
//     let wiki_map = load_wiki().await.unwrap();

//     let mut map: HashMap<String, CardData> = HashMap::new();
//     for card in sample.cards.iter() {
//         let name = &card.name;
//         let wiki = wiki_map.get(name).map(|w| w.to_owned());
//         map.insert(
//             name.to_string(),
//             CardData {
//                 name: name.to_string(),
//                 min_level: wiki.clone().map(|w| w.min_level).flatten(),
//                 max_level: wiki.map(|w| w.max_level).flatten(),
//                 weight: card.weight,
//                 price: card.price,
//             },
//         );
//     }

//     std::fs::write(
//         "card-data.json",
//         serde_json::to_string_pretty(&map).unwrap(),
//     )
//     .unwrap();
// }

pub const POE_CDN_CARDS: &'static str = "https://web.poecdn.com/image/divination-card/";

pub async fn download_card_images() -> Result<(), Error> {
    let data = NinjaCardData::fetch(&TradeLeague::Ancestor).await?;

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
