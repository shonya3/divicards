//! Load a list of maps from wiki

use crate::consts::WIKI_API_URL;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapDataFromWiki {
    pub name: String,
    pub tier: u32,
}

pub async fn fetch_wiki_maplist() -> Result<Vec<MapDataFromWiki>, reqwest::Error> {
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

    let params = [
        ("action", "cargoquery"),
        ("format", "json"),
        ("smaxage", "0"),
        ("maxage", "0"),
        ("limit", "500"),
        ("tables", "maps,items,areas"),
        ("join_on", "items._pageID=maps._pageID,maps.area_id=areas.id"),
        ("fields", "maps.tier,items.name,maps.area_id,maps.area_level,areas.boss_monster_ids,maps.unique_area_id"),
        ("group_by", "items.name"),
        ("where", "items.class_id='Map' AND maps.area_id LIKE '%MapWorlds%'"),
    ];

    let response: WikiResponse = reqwest::Client::new()
        .get(WIKI_API_URL)
        .query(&params)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(response
        .cargoquery
        .into_iter()
        .map(|title| MapDataFromWiki {
            name: title.title.name,
            tier: title.title.tier.parse().unwrap(),
        })
        .collect::<Vec<MapDataFromWiki>>())
}
