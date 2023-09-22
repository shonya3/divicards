use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

use crate::{
    consts::{POEDB_MAPS_URL, WIKI_API_URL},
    error::Error,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapDataFromWiki {
    pub name: String,
    pub tier: u32,
}

pub async fn load_from_wiki() -> Result<Vec<MapDataFromWiki>, Error> {
    #[derive(Deserialize)]
    pub struct WikiResponse {
        pub cargoquery: Vec<Title>,
    }

    #[derive(Deserialize)]
    pub struct Title {
        pub title: MapDataFromWiki,
    }

    let url = format!("{WIKI_API_URL}?action=cargoquery&tables=maps,items,areas&fields=items.name,maps.tier&format=json&where=items.class_id='Map' AND maps.area_id LIKE '%MapWorlds%'&group_by=items.name&join_on=items._pageID=maps._pageID,maps.area_id=areas.id&smaxage=0&maxage=0");

    let response: WikiResponse = reqwest::get(url).await?.json().await?;
    Ok(response
        .cargoquery
        .into_iter()
        .map(|title| title.title)
        .collect::<Vec<MapDataFromWiki>>())
}

pub async fn load_poedb_non_unique_actual_maplist() -> Result<Vec<String>, Error> {
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

pub async fn collect_map_data() {}
