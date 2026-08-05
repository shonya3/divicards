//! Extracts campaign act area data from Path of Exile 1 game files.
//!
//! # Data sources
//!
//! | Field | Source |
//! |-------|--------|
//! | `id` | `WorldAreas.datc64` → `Id` column |
//! | `name` | `WorldAreas.datc64` → `Name` column |
//! | `act` | `WorldAreas.datc64` → `Act` column |
//! | `area_level` | `WorldAreas.datc64` → `AreaLevel` column |
//! | `image_url` | Derived from `id`: strip game prefix, drop trailing sub-level number, format `/images/acts/{key}.webp` |
//! | `poedb_image_url` | `https://cdn.poedb.tw/image/Art/2DArt/UIImages/InGame/Acts/{act}/{key}.webp` |
//! | `has_waypoint` | `WorldAreas.datc64` → `HasWaypoint` column |
//! | `is_town` | `WorldAreas.datc64` → `IsTown` column |
//! | `has_labyrinth_trial` | `LabyrinthTrials.datc64` → FK into `WorldAreas` row index |
//! | `bossfights` | `WorldAreas.datc64` → `Bosses_MonsterVarietiesKeys` FK into `MonsterVarieties.datc64` → `Name`; multi-phase fixup by area id |
//!
//! ## Filtering
//!
//! WorldAreas has 1915 entries (hideouts, league mechanics, maps, test areas,
//! campaign zones). Campaign areas are identified by id matching `{game}_{act}_{rest}`
//! where `game ∈ {1, 2}` and `act ∈ 1..=10`, excluding blank/`NULL` names.

use std::collections::HashSet;
use std::path::Path;

use anyhow::{Context, Result};
use arrow_array::{Array, ListArray, StringArray, UInt64Array};
use divcord::poe_data::act::{ActArea, ActAreaId, Bossfight};
use poe_data_tools::{
    dat::{schema::SchemaCollection, schema::fetch_schema, table::parse_table},
    file_parsers::{FileParser, dat::DatParser},
    fs::FileSystem,
};
use crate::GameFiles;

fn is_campaign_area(id: &str, act: u32) -> bool {
    let parts: Vec<&str> = id.splitn(3, '_').collect();
    if parts.len() < 3 {
        return false;
    }
    let game = parts[0];
    if game != "1" && game != "2" {
        return false;
    }
    let parsed_act: u32 = match parts[1].parse() {
        Ok(n) => n,
        Err(_) => return false,
    };
    parsed_act == act && (1..=10).contains(&act)
}

fn derive_image_key(id: &str, _name: &str) -> String {
    // Manual overrides for poedb's non-algorithmic image keys
    match id {
        "1_4_3_1" | "1_4_3_2" => return "4_3_2".to_string(),
        "1_4_5_2" => return "4_5_3".to_string(),
        "1_4_6_2" => return "4_6_1".to_string(),
        _ => {}
    }
    let without_game = id.split_once('_').map(|x| x.1).unwrap_or(id);
    let parts: Vec<&str> = without_game.split('_').filter(|s| !s.is_empty()).collect();
    // In acts 4 and 9, the numeric suffix is part of the area's unique key, not a sub-level
    if parts.len() > 2 {
        let last = parts.last().unwrap().trim_end_matches('_');
        let act = parts
            .first()
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(0);
        if act != 4 && act != 9 && last.parse::<u32>().ok().is_some_and(|n| n > 0) {
            return parts[..parts.len() - 1].join("_");
        }
    }
    without_game.to_string()
}

pub fn extract_areas(
    fs: &impl FileSystem,
    schemas: &SchemaCollection,
) -> Result<(Vec<ActArea>, Vec<String>)> {
    let schema = |name: &str| {
        schemas
            .tables
            .iter()
            .filter(|t| t.valid_for == 1 || t.valid_for == 3)
            .find(|t| t.name.eq_ignore_ascii_case(name))
    };

    // --- Read MonsterVarieties for boss name lookup ---
    let mv_bytes = fs.read("Data/MonsterVarieties.datc64")?;
    let mv_table = DatParser.parse(&mv_bytes)?;
    let mv_schema = schema("MonsterVarieties").context("No schema for MonsterVarieties")?;
    let mv_df = parse_table(&mv_table, mv_schema)?;

    let mv_names = mv_df
        .column(mv_df.schema().index_of("Name").unwrap())
        .as_any()
        .downcast_ref::<StringArray>()
        .unwrap();

    let monster_names: Vec<String> = (0..mv_df.num_rows())
        .map(|i| mv_names.value(i).to_string())
        .collect();

    // --- Read LabyrinthTrials for trial check ---
    let trial_bytes = fs.read("Data/LabyrinthTrials.datc64")?;
    let trial_table = DatParser.parse(&trial_bytes)?;
    let trial_schema = schema("LabyrinthTrials").context("No schema for LabyrinthTrials")?;
    let trial_df = parse_table(&trial_table, trial_schema)?;

    let trial_world_areas = trial_df
        .column(trial_df.schema().index_of("WorldAreas").unwrap())
        .as_any()
        .downcast_ref::<UInt64Array>()
        .unwrap();

    let areas_with_trials: HashSet<u64> = (0..trial_df.num_rows())
        .filter_map(|i| {
            if trial_world_areas.is_null(i) {
                None
            } else {
                Some(trial_world_areas.value(i))
            }
        })
        .collect();

    // --- Read WorldAreas ---
    let wa_bytes = fs.read("Data/WorldAreas.datc64")?;
    let wa_table = DatParser.parse(&wa_bytes)?;
    let wa_schema = schema("WorldAreas").context("No schema for WorldAreas")?;
    let wa_df = parse_table(&wa_table, wa_schema)?;

    let ids = wa_df
        .column(wa_df.schema().index_of("Id").unwrap())
        .as_any()
        .downcast_ref::<StringArray>()
        .unwrap();
    let names = wa_df
        .column(wa_df.schema().index_of("Name").unwrap())
        .as_any()
        .downcast_ref::<StringArray>()
        .unwrap();
    let acts = wa_df
        .column(wa_df.schema().index_of("Act").unwrap())
        .as_any()
        .downcast_ref::<arrow_array::Int32Array>()
        .unwrap();
    let area_levels = wa_df
        .column(wa_df.schema().index_of("AreaLevel").unwrap())
        .as_any()
        .downcast_ref::<arrow_array::Int32Array>()
        .unwrap();
    let is_towns = wa_df
        .column(wa_df.schema().index_of("IsTown").unwrap())
        .as_any()
        .downcast_ref::<arrow_array::BooleanArray>()
        .unwrap();
    let has_waypoints = wa_df
        .column(wa_df.schema().index_of("HasWaypoint").unwrap())
        .as_any()
        .downcast_ref::<arrow_array::BooleanArray>()
        .unwrap();

    // Bosses_MonsterVarietiesKeys is a foreignrow[]
    let bosses_col = wa_df
        .column(
            wa_df
                .schema()
                .index_of("Bosses_MonsterVarietiesKeys")
                .unwrap(),
        )
        .as_any()
        .downcast_ref::<ListArray>()
        .unwrap();

    // --- Build list of areas ---
    let mut areas: Vec<ActArea> = Vec::new();

    for i in 0..wa_df.num_rows() {
        let id = ids.value(i);
        let name = names.value(i);
        if id.is_empty() || name.is_empty() || name == "NULL" {
            continue;
        }

        let act = acts.value(i) as u32;
        if !is_campaign_area(id, act) {
            continue;
        }

        let area_level = area_levels.value(i);
        let is_town = is_towns.value(i);
        let has_waypoint = has_waypoints.value(i);

        // Boss names
        let mut bossfights = Vec::new();
        if !bosses_col.is_null(i) {
            let boss_keys = bosses_col.value(i);
            let boss_keys = boss_keys.as_any().downcast_ref::<UInt64Array>().unwrap();
            for j in 0..boss_keys.len() {
                let key = boss_keys.value(j) as usize;
                let name = &monster_names[key];
                if !name.is_empty() {
                    bossfights.push(Bossfight { name: name.clone() });
                }
            }
        }

        // Multi-phase boss fixup: add boss names that exist in MonsterVarieties
        // but aren't linked in WorldAreas.Bosses_MonsterVarietiesKeys
        let id_str = id.to_string();
        if id_str == "1_3_18_2" && !bossfights.contains(&Bossfight { name: "Dominus, Ascendant".to_string() }) {
            bossfights.push(Bossfight { name: "Dominus, Ascendant".to_string() });
        }
        if id_str == "1_5_5"
            && !bossfights.contains(&Bossfight { name: "Innocence, God-Emperor of Eternity".to_string() })
        {
            bossfights.push(Bossfight { name: "Innocence, God-Emperor of Eternity".to_string() });
        }

        let has_labyrinth_trial = areas_with_trials.contains(&(i as u64));

        let key = derive_image_key(id, name);
        areas.push(ActArea {
            id: ActAreaId::new(id.to_string()),
            name: name.to_string(),
            act: act as u8,
            area_level: area_level as u8,
            image_url: format!("/images/acts/{key}.webp"),
            poedb_image_url: format!(
                "https://cdn.poedb.tw/image/Art/2DArt/UIImages/InGame/Acts/{act}/{key}.webp"
            ),
            has_waypoint,
            is_town,
            has_labyrinth_trial,
            bossfights,
        });
    }

    // --- Sort by act, then area level, then name ---
    areas.sort_by(|a, b| {
        a.act
            .cmp(&b.act)
            .then(a.area_level.cmp(&b.area_level))
            .then(a.name.cmp(&b.name))
    });

    Ok((areas, monster_names))
}

pub fn run(source: &GameFiles, output: &Path) -> Result<()> {
    let cache_dir = dirs::cache_dir().unwrap().join("poe_data_tools");
    let fs = source.open().context("Failed to open game files")?;
    let schemas = fetch_schema(&cache_dir).context("Failed to fetch schema")?;

    let (areas, _all_monster_names) = extract_areas(&fs, &schemas)?;

    std::fs::create_dir_all(output)?;

    let json_path = output.join("acts.json");
    let json_data = serde_json::to_string_pretty(&areas)?;
    std::fs::write(&json_path, json_data)?;

    println!("Done — {} areas", areas.len());
    println!("  JSON: {}", json_path.display());

    Ok(())
}
