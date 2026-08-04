//! Atlas map extraction from game files.
//!
//! Reads `AtlasNode.datc64` to find which divination cards drop on which atlas
//! areas, then resolves area names from `WorldAreas.datc64` and card IDs from
//! `BaseItemTypes.datc64`.
//!
//! # Data flow
//!
//! 1. Parse `BaseItemTypes.datc64` to build a row-index → card-ID lookup.
//! 2. Parse `WorldAreas.datc64` to build a row-index → area-name lookup.
//!    Standard map areas (`IsMapArea && !IsUniqueMapArea`) get `" Map"` appended
//!    (e.g. `"Frozen Cabins"` → `"Frozen Cabins Map"`). Unique maps
//!    (e.g. `"Pillars of Arun"`) are left as-is.
//! 3. Parse `AtlasNode.datc64`. Each row has an `Area1` FK (→ `WorldAreas`) and
//!    a `DivCards` array FK (→ `BaseItemTypes`). Iterate every atlas node,
//!    link each referenced card ID to the resolved area name.
//!
//! # Naming convention
//!
//! Standard map areas get the `" Map"` suffix (e.g. `"Frozen Cabins"` →
//! `"Frozen Cabins Map"`). Unique maps (e.g. `"Pillars of Arun"`) are left
//! as-is. `The Immortal` → `Hall of Grandmasters` is absent from game data —
//! no `AtlasNode` row links that card via `DivCards`.

use anyhow::{Context, Result};
use arrow_array::{Array, BooleanArray, ListArray, StringArray, UInt64Array};
use poe_data_tools::{
    dat::{schema::SchemaCollection, table::parse_table},
    file_parsers::{FileParser, dat::DatParser},
    fs::FileSystem,
};
use std::collections::HashMap;

pub fn extract(
    fs: &impl FileSystem,
    schemas: &SchemaCollection,
) -> Result<HashMap<String, Vec<String>>> {
    let schema = |name: &str| {
        schemas
            .tables
            .iter()
            .filter(|t| t.valid_for == 1 || t.valid_for == 3)
            .find(|t| t.name.eq_ignore_ascii_case(name))
    };

    // --- BaseItemTypes to resolve DivCards row index → Id string ---
    let bit_bytes = fs
        .read("Data/BaseItemTypes.datc64")
        .context("Failed to read BaseItemTypes")?;
    let bit_table = DatParser.parse(&bit_bytes)?;
    let bit_schema = schema("BaseItemTypes").context("No schema for BaseItemTypes")?;
    let bit_df = parse_table(&bit_table, bit_schema).context("Failed to parse BaseItemTypes")?;
    let bit_ids = bit_df
        .column(bit_df.schema().index_of("Id").unwrap())
        .as_any()
        .downcast_ref::<StringArray>()
        .unwrap();
    let base_item_ids: Vec<String> = (0..bit_df.num_rows())
        .map(|i| bit_ids.value(i).to_string())
        .collect();

    // --- WorldAreas to resolve Area1 row index → area name ---
    let wa_bytes = fs
        .read("Data/WorldAreas.datc64")
        .context("Failed to read WorldAreas")?;
    let wa_table = DatParser.parse(&wa_bytes)?;
    let wa_schema = schema("WorldAreas").context("No schema for WorldAreas")?;
    let wa_df = parse_table(&wa_table, wa_schema).context("Failed to parse WorldAreas")?;

    let wa_names = wa_df
        .column(wa_df.schema().index_of("Name").unwrap())
        .as_any()
        .downcast_ref::<StringArray>()
        .unwrap();

    let is_map_area = wa_df
        .column(wa_df.schema().index_of("IsMapArea").unwrap())
        .as_any()
        .downcast_ref::<BooleanArray>()
        .unwrap();
    let is_unique_map = wa_df
        .column(wa_df.schema().index_of("IsUniqueMapArea").unwrap())
        .as_any()
        .downcast_ref::<BooleanArray>()
        .unwrap();

    let area_names: Vec<String> = (0..wa_df.num_rows())
        .map(|i| {
            let name = wa_names.value(i).to_string();
            // Append " Map" for standard map areas (not unique maps like Hall of Grandmasters)
            if is_map_area.value(i) && !is_unique_map.value(i) {
                format!("{} Map", name)
            } else {
                name
            }
        })
        .collect();

    // --- AtlasNode for card→area links ---
    let an_bytes = fs
        .read("Data/AtlasNode.datc64")
        .context("Failed to read AtlasNode")?;
    let an_table = DatParser.parse(&an_bytes)?;
    let an_schema = schema("AtlasNode").context("No schema for AtlasNode")?;
    let an_df = parse_table(&an_table, an_schema).context("Failed to parse AtlasNode")?;

    let div_cards_col = an_df
        .column(an_df.schema().index_of("DivCards").unwrap())
        .as_any()
        .downcast_ref::<ListArray>()
        .unwrap();
    let area1_col = an_df
        .column(an_df.schema().index_of("Area1").unwrap())
        .as_any()
        .downcast_ref::<UInt64Array>()
        .unwrap();

    let mut card_areas: HashMap<String, Vec<String>> = HashMap::new();

    for i in 0..an_df.num_rows() {
        if div_cards_col.is_null(i) || area1_col.is_null(i) {
            continue;
        }
        let area_row = area1_col.value(i) as usize;
        if area_row >= area_names.len() {
            continue;
        }
        let area_name = &area_names[area_row];
        if area_name.is_empty() || area_name == "NULL" {
            continue;
        }

        let card_list = div_cards_col.value(i);
        let card_keys = card_list.as_any().downcast_ref::<UInt64Array>().unwrap();
        for j in 0..card_keys.len() {
            let card_row = card_keys.value(j) as usize;
            if card_row >= base_item_ids.len() {
                continue;
            }
            card_areas
                .entry(base_item_ids[card_row].clone())
                .or_default()
                .push(area_name.clone());
        }
    }

    Ok(card_areas)
}
