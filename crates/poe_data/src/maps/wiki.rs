//! Load a list of maps from wiki

use std::{collections::HashSet, fmt::Display};

use crate::consts::WIKI_API_URL;
use serde::{Deserialize, Serialize};

pub async fn fetch_wiki_maplist() -> Result<Vec<MapDataFromWiki>, FetchWikiMapsError> {
    #[derive(Deserialize, Serialize)]
    struct WikiResponse {
        cargoquery: Vec<Title>,
    }

    #[derive(Deserialize, Serialize)]
    struct Title {
        title: MapRecord,
    }

    #[derive(Deserialize, Serialize)]
    struct MapRecord {
        #[serde(alias = "area id")]
        id: String,
        name: String,
        tier: String,
    }

    let where_s: String = [
        "areas.area_level != 0 AND areas.is_legacy_map_area=false AND areas.is_hideout_area=false AND ",
        "areas.is_town_area=false AND areas.is_labyrinth_area=false AND areas.is_labyrinth_airlock_area=false AND ",
        "areas.is_labyrinth_boss_area=false AND areas.is_vaal_area=false AND ",
        "(areas.is_map_area OR areas.is_unique_map_area OR areas.act != 11 AND ",
        "(areas.id LIKE '1_%' OR areas.id LIKE '2_%') OR areas.id LIKE '%Labyrinth%')"
    ].into_iter().collect();

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
        ("where", where_s.as_str()),
    ];

    let response: WikiResponse = reqwest::Client::new()
        .get(WIKI_API_URL)
        .query(&params)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    let wiki_maps: Vec<&str> = response
        .cargoquery
        .iter()
        .map(|Title { title }| title.name.as_str())
        .collect();

    if let Err(missing_maps) = ensure_expected_maps(&wiki_maps) {
        return Err(FetchWikiMapsError::MissingExpectedMaps(missing_maps));
    };

    Ok(response
        .cargoquery
        .into_iter()
        .map(|Title { title }| MapDataFromWiki {
            id: title.id,
            name: title.name,
            tier: title.tier.parse().unwrap(),
        })
        .collect::<Vec<MapDataFromWiki>>())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapDataFromWiki {
    pub id: String,
    pub name: String,
    pub tier: u32,
}

#[derive(Debug)]
pub enum FetchWikiMapsError {
    Reqwest(reqwest::Error),
    MissingExpectedMaps(Vec<String>),
}

impl Display for FetchWikiMapsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FetchWikiMapsError::Reqwest(err) => err.fmt(f),
            FetchWikiMapsError::MissingExpectedMaps(expected_maps) => {
                write!(f, "Missing expected maps: {}", expected_maps.join(","))
            }
        }
    }
}

impl From<reqwest::Error> for FetchWikiMapsError {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}

fn ensure_expected_maps(wiki_maps: &[&str]) -> Result<(), Vec<String>> {
    // t17 and Shaper Guardians
    const EXPECTED_MAPS: [&str; 10] = [
        "Abomination Map",
        "Citadel Map",
        "Fortress Map",
        "Sanctuary Map",
        "Ziggurat Map",
        "Forge of the Phoenix Map",
        "Lair of the Hydra Map",
        "Maze of the Minotaur Map",
        "Pit of the Chimera Map",
        "Cortex",
    ];

    let wiki_maps_set: HashSet<&str> = wiki_maps.iter().copied().collect();

    let missing_maps: Vec<String> = EXPECTED_MAPS
        .iter()
        .filter(|&&expected_map| !wiki_maps_set.contains(expected_map))
        .map(|&s| s.to_string())
        .collect();

    if missing_maps.is_empty() {
        Ok(())
    } else {
        Err(missing_maps)
    }
}
