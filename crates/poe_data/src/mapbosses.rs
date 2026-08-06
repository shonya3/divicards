//! Map boss extraction from game files.
//!
//! # Data sources
//!
//! | Field | Source |
//! |-------|--------|
//! | `name` | `MonsterVarieties.datc64` → `Name` column, joined via `WorldAreas.Bosses_MonsterVarietiesKeys` FK |
//! | `maps` | `WorldAreas.datc64` → `Name` (with `" Map"` appended for non-unique map areas), filtered to atlas maps (`AtlasNode.datc64`) or unique maps with an item (`UniqueMaps.datc64`), matching the `maps` list |

use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result};
use arrow_array::{Array, BooleanArray, ListArray, StringArray, UInt64Array};
use poe_data_tools::{
    dat::{schema::SchemaCollection, table::parse_table},
    file_parsers::{FileParser, dat::DatParser},
    fs::FileSystem,
};
use divcord::poe_data::mapbosses::MapBoss;

fn str_at(df: &arrow_array::RecordBatch, col: &str, row: usize) -> String {
    if row >= df.num_rows() { return String::new(); }
    let arr = df.column(df.schema().index_of(col).unwrap())
        .as_any().downcast_ref::<StringArray>().unwrap();
    if arr.is_null(row) { String::new() } else { arr.value(row).to_string() }
}

pub fn extract(
    fs: &impl FileSystem,
    schemas: &SchemaCollection,
) -> Result<Vec<MapBoss>> {
    let schema = |name: &str, valid_fors: &[u32]| {
        schemas.tables.iter()
            .filter(|t| valid_fors.contains(&t.valid_for))
            .find(|t| t.name.eq_ignore_ascii_case(name))
    };

    // ── MonsterVarieties: row index → name ─────────────────────
    let mv_bytes = fs.read("Data/MonsterVarieties.datc64")
        .context("Failed to read MonsterVarieties")?;
    let mv_table = DatParser.parse(&mv_bytes)?;
    let mv_schema = schema("MonsterVarieties", &[1])
        .context("No schema for MonsterVarieties")?;
    let mv_df = parse_table(&mv_table, mv_schema)
        .context("Failed to parse MonsterVarieties")?;

    let monster_names: Vec<String> = (0..mv_df.num_rows())
        .map(|i| str_at(&mv_df, "Name", i))
        .collect();

    // ── WorldAreas: Bosses_MonsterVarietiesKeys ────────────────
    let wa_bytes = fs.read("Data/WorldAreas.datc64")
        .context("Failed to read WorldAreas")?;
    let wa_table = DatParser.parse(&wa_bytes)?;
    let wa_schema = schema("WorldAreas", &[1])
        .context("No schema for WorldAreas")?;
    let wa_df = parse_table(&wa_table, wa_schema)
        .context("Failed to parse WorldAreas")?;

    let wa_names = wa_df.column(wa_df.schema().index_of("Name").unwrap());
    let wa_names = wa_names.as_any().downcast_ref::<StringArray>().unwrap();

    let is_map = wa_df.column(wa_df.schema().index_of("IsMapArea").unwrap());
    let is_map = is_map.as_any().downcast_ref::<BooleanArray>().unwrap();

    let is_unique = wa_df.column(wa_df.schema().index_of("IsUniqueMapArea").unwrap());
    let is_unique = is_unique.as_any().downcast_ref::<BooleanArray>().unwrap();

    let bosses_col = wa_df.column(wa_df.schema().index_of("Bosses_MonsterVarietiesKeys").unwrap());
    let bosses_col = bosses_col.as_any().downcast_ref::<ListArray>().unwrap();

    // WorldAreas rows that are real maps: atlas nodes (normal, or unique with
    // an item entry) — the same membership rule as `maps::extract`.
    let mut um_areas: HashSet<usize> = HashSet::new();
    {
        let um_bytes = fs.read("Data/UniqueMaps.datc64")
            .context("Failed to read UniqueMaps")?;
        let um_table = DatParser.parse(&um_bytes)?;
        let um_schema = schema("UniqueMaps", &[3])
            .context("No schema for UniqueMaps")?;
        let um_df = parse_table(&um_table, um_schema)
            .context("Failed to parse UniqueMaps")?;

        let um_wa = um_df
            .column(um_df.schema().index_of("WorldAreasKey").unwrap())
            .as_any()
            .downcast_ref::<UInt64Array>()
            .unwrap();
        let um_replica = um_df.column(4).as_any().downcast_ref::<BooleanArray>().unwrap();
        let um_disabled = um_df
            .column(um_df.schema().index_of("IsDropDisabled").unwrap())
            .as_any()
            .downcast_ref::<BooleanArray>()
            .unwrap();
        for i in 0..um_df.num_rows() {
            if um_replica.value(i) && !um_disabled.value(i) {
                continue;
            }
            um_areas.insert(um_wa.value(i) as usize);
        }
    }
    let mut real_map_areas: HashSet<usize> = HashSet::new();
    {
        let an_bytes = fs.read("Data/AtlasNode.datc64")
            .context("Failed to read AtlasNode")?;
        let an_table = DatParser.parse(&an_bytes)?;
        let an_schema = schema("AtlasNode", &[3])
            .context("No schema for AtlasNode")?;
        let an_df = parse_table(&an_table, an_schema)
            .context("Failed to parse AtlasNode")?;

        let an_area = an_df
            .column(an_df.schema().index_of("Area1").unwrap())
            .as_any()
            .downcast_ref::<UInt64Array>()
            .unwrap();
        let an_is_norm = an_df
            .column(an_df.schema().index_of("IsNormalMap").unwrap())
            .as_any()
            .downcast_ref::<BooleanArray>()
            .unwrap();
        let an_is_uniq = an_df
            .column(an_df.schema().index_of("IsUniqueMap").unwrap())
            .as_any()
            .downcast_ref::<BooleanArray>()
            .unwrap();
        for i in 0..an_df.num_rows() {
            let area = an_area.value(i) as usize;
            if an_is_norm.value(i) || (an_is_uniq.value(i) && um_areas.contains(&area)) {
                real_map_areas.insert(area);
            }
        }
    }

    // Build map_name → [boss_name]
    let mut map_bosses: HashMap<String, Vec<String>> = HashMap::new();

    for i in 0..wa_df.num_rows() {
        if !real_map_areas.contains(&i) { continue; }
        if bosses_col.is_null(i) { continue; }

        let name = wa_names.value(i).to_string();
        if name.is_empty() || name == "NULL" { continue; }
        let map_name = if is_map.value(i) && !is_unique.value(i) {
            format!("{name} Map")
        } else {
            name
        };

        let list = bosses_col.value(i);
        let keys = list.as_any().downcast_ref::<UInt64Array>().unwrap();
        for j in 0..keys.len() {
            let row = keys.value(j) as usize;
            if row < monster_names.len() {
                let bname = &monster_names[row];
                if bname.is_empty() || bname == "NULL" || bname.contains("<<>>") { continue; }
                let entry = map_bosses.entry(bname.clone()).or_default();
                if !entry.contains(&map_name) {
                    entry.push(map_name.clone());
                }
            }
        }
    }

    // Convert to Vec<MapBoss>
    let result: Vec<MapBoss> = map_bosses.into_iter()
        .map(|(name, maps)| MapBoss { name, maps })
        .collect();

    Ok(result)
}