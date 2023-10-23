use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

use crate::{
    consts::{POEDB_MAPS_URL, WIKI_API_URL},
    error::Error,
    loader::DataLoader,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Map {
    pub name: String,
    pub tier: u32,
    pub available: bool,
    pub unique: bool,
}

impl Map {
    pub fn level(&self) -> u32 {
        67 + self.tier
    }
}

pub struct MapLoader;
impl MapLoader {
    pub const fn new() -> Self {
        MapLoader
    }
}

#[async_trait::async_trait]
impl DataLoader<Vec<Map>> for MapLoader {
    fn filename(&self) -> &'static str {
        "maps.json"
    }

    async fn fetch(&self) -> Result<Vec<Map>, Error> {
        let available_maps = load_poedb_non_unique_available_maplist().await?;
        let wiki_maps = load_from_wiki().await?;

        Ok(wiki_maps
            .into_iter()
            .map(|MapDataFromWiki { name, tier }| {
                let unique = !name.ends_with(" Map");
                let available = unique || available_maps.contains(&name);
                Map {
                    name,
                    tier,
                    available,
                    unique,
                }
            })
            .collect())
    }
}

async fn load_poedb_non_unique_available_maplist() -> Result<Vec<String>, Error> {
    let markup = reqwest::get(POEDB_MAPS_URL).await?.text().await?;
    let html = Html::parse_fragment(&markup);
    let mut maps = html
        .select(&Selector::parse("#MapsList").unwrap())
        .next()
        .unwrap()
        .select(&Selector::parse("table").unwrap())
        .next()
        .unwrap()
        .select(&Selector::parse("tbody").unwrap())
        .next()
        .unwrap()
        .select(&Selector::parse("tr").unwrap())
        .map(|row| {
            row.select(&Selector::parse("td").unwrap())
                .skip(3)
                .next()
                .unwrap()
                .text()
                .collect::<String>()
        })
        .filter(|s| s.len() > 0)
        .collect::<Vec<String>>();
    maps.sort();

    Ok(maps)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapDataFromWiki {
    pub name: String,
    pub tier: u32,
}

async fn load_from_wiki() -> Result<Vec<MapDataFromWiki>, Error> {
    #[derive(Deserialize)]
    pub struct WikiResponse {
        pub cargoquery: Vec<Title>,
    }

    #[derive(Deserialize)]
    pub struct Title {
        pub title: MapRecord,
    }

    #[derive(Deserialize)]
    pub struct MapRecord {
        pub name: String,
        pub tier: String,
    }

    let url = format!("{WIKI_API_URL}?action=cargoquery&format=json&smaxage=0&maxage=0&limit=500&tables=maps,items,areas&join_on=items._pageID=maps._pageID,maps.area_id=areas.id&fields=maps.tier,items.name,maps.area_id,maps.area_level,areas.boss_monster_ids,maps.unique_area_id&group_by=items.name&where=items.class_id='Map' AND maps.area_id LIKE '%MapWorlds%'");

    let response: WikiResponse = reqwest::get(url).await?.json().await?;

    Ok(response
        .cargoquery
        .into_iter()
        .map(|title| MapDataFromWiki {
            name: title.title.name,
            tier: title.title.tier.parse().unwrap(),
        })
        .collect::<Vec<MapDataFromWiki>>())
}
