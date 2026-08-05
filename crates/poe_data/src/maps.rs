//! Map extraction from game files.
//!
//! # Data sources
//!
//! | Field | Source |
//! |-------|--------|
//! | `name` | Normal: `AtlasNode.Area1` FK → `WorldAreas.Name`; unique: `UniqueMaps.WordsKey` FK → `Words.Text` |
//! | `tier` | `AtlasNode.Tier` (unique maps on the atlas; replicas: `0`) |
//! | `unique` | `AtlasNode.IsUniqueMap` / replica rows in `UniqueMaps` |
//! | `icon` | Normal: `AtlasNode.Node_DDSFile`; unique: `UniqueMaps.ItemVisualIdentityKey` FK → `ItemVisualIdentity.DDSFile` → CDN URL |
//! | `slug` | Derived from `name` via [`slug::slugify`] |
//! | `atlas_cards` | `AtlasNode.DivCards` FK → `BaseItemTypes.Name` |
//!
//! Unique maps are resolved via `UniqueMaps.dat`, which is keyed by
//! `WorldAreasKey` (the area row id used by `AtlasNode.Area1`). Replicas
//! (e.g. `Replica Cortex`) share the `WorldAreasKey` of their base map, so
//! they are appended separately with a `" Map"` suffix.

use anyhow::{Context, Result};
use arrow_array::{Array, BooleanArray, Int32Array, ListArray, StringArray, UInt64Array};
use poe_data_tools::{
    dat::{schema::SchemaCollection, table::parse_table},
    file_parsers::{FileParser, dat::DatParser},
    fs::FileSystem,
};
use divcord::poe_data::maps::Map;

const CDN_BASE: &str = "https://web.poecdn.com/image";

fn cdn_url(path: &str) -> String {
    let cleaned = path.strip_suffix(".dds").unwrap_or(path);
    format!("{CDN_BASE}/{cleaned}.png")
}

fn str_at(df: &arrow_array::RecordBatch, col: &str, row: usize) -> String {
    if row >= df.num_rows() {
        return String::new();
    }
    let arr = df
        .column(df.schema().index_of(col).unwrap())
        .as_any()
        .downcast_ref::<StringArray>()
        .unwrap();
    if arr.is_null(row) {
        String::new()
    } else {
        arr.value(row).to_string()
    }
}

fn bool_at(df: &arrow_array::RecordBatch, col: &str, row: usize) -> bool {
    if row >= df.num_rows() {
        return false;
    }
    let arr = df
        .column(df.schema().index_of(col).unwrap())
        .as_any()
        .downcast_ref::<BooleanArray>()
        .unwrap();
    if arr.is_null(row) {
        false
    } else {
        arr.value(row)
    }
}

/// Unique-map item info, resolved via `UniqueMaps.dat`:
/// `WorldAreasKey` → item name, `ItemVisualIdentityKey` → icon.
///
/// Replica detection: `UniqueMaps` column 4 (unnamed bool) is true for both
/// drop-disabled maps and replicas; `IsDropDisabled` disambiguates — a row is
/// a replica iff `column4 && !IsDropDisabled`.
struct UniqueMapInfo {
    name: String,
    icon: String,
}

pub fn extract(fs: &impl FileSystem, schemas: &SchemaCollection) -> Result<Vec<Map>> {
    let schema = |name: &str, valid_fors: &[u32]| {
        schemas
            .tables
            .iter()
            .filter(|t| valid_fors.contains(&t.valid_for))
            .find(|t| t.name.eq_ignore_ascii_case(name))
    };

    // ── BaseItemTypes (for card name resolution) ────────────────
    let bit_bytes = fs
        .read("Data/BaseItemTypes.datc64")
        .context("Failed to read BaseItemTypes")?;
    let bit_table = DatParser.parse(&bit_bytes)?;
    let bit_schema = schema("BaseItemTypes", &[1]).context("No schema for BaseItemTypes")?;
    let bit_df = parse_table(&bit_table, bit_schema).context("Failed to parse BaseItemTypes")?;

    let base_item_names: Vec<String> = (0..bit_df.num_rows())
        .map(|i| str_at(&bit_df, "Name", i))
        .collect();

    // ── WorldAreas (for area→name resolution) ──────────────────
    let wa_bytes = fs
        .read("Data/WorldAreas.datc64")
        .context("Failed to read WorldAreas")?;
    let wa_table = DatParser.parse(&wa_bytes)?;
    let wa_schema = schema("WorldAreas", &[1]).context("No schema for WorldAreas")?;
    let wa_df = parse_table(&wa_table, wa_schema).context("Failed to parse WorldAreas")?;

    let area_names: Vec<String> = (0..wa_df.num_rows())
        .map(|i| {
            let name = str_at(&wa_df, "Name", i);
            if bool_at(&wa_df, "IsMapArea", i) && !bool_at(&wa_df, "IsUniqueMapArea", i) {
                format!("{name} Map")
            } else {
                name
            }
        })
        .collect();

    // ── AtlasNode (for icon + atlas_cards) ─────────────────────
    let an_bytes = fs
        .read("Data/AtlasNode.datc64")
        .context("Failed to read AtlasNode")?;
    let an_table = DatParser.parse(&an_bytes)?;
    let an_schema = schema("AtlasNode", &[3]).context("No schema for AtlasNode")?;
    let an_df = parse_table(&an_table, an_schema).context("Failed to parse AtlasNode")?;

    // Build map list from AtlasNode + WorldAreas
    let an_area = an_df
        .column(an_df.schema().index_of("Area1").unwrap())
        .as_any()
        .downcast_ref::<UInt64Array>()
        .unwrap();
    let an_tier = an_df
        .column(an_df.schema().index_of("Tier").unwrap())
        .as_any()
        .downcast_ref::<Int32Array>()
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
    let an_dds = an_df
        .column(an_df.schema().index_of("Node_DDSFile").unwrap())
        .as_any()
        .downcast_ref::<StringArray>()
        .unwrap();

    let div_cards_col = an_df
        .column(an_df.schema().index_of("DivCards").unwrap())
        .as_any()
        .downcast_ref::<ListArray>()
        .unwrap();

    // ── UniqueMaps + Words + ItemVisualIdentity (name/icon) ─────
    let um_bytes = fs
        .read("Data/UniqueMaps.datc64")
        .context("Failed to read UniqueMaps")?;
    let um_table = DatParser.parse(&um_bytes)?;
    let um_schema = schema("UniqueMaps", &[3]).context("No schema for UniqueMaps")?;
    let um_df = parse_table(&um_table, um_schema).context("Failed to parse UniqueMaps")?;

    let words_bytes = fs
        .read("Data/Words.datc64")
        .context("Failed to read Words")?;
    let words_table = DatParser.parse(&words_bytes)?;
    let words_schema = schema("Words", &[3]).context("No schema for Words")?;
    let words_df = parse_table(&words_table, words_schema).context("Failed to parse Words")?;

    let ivi_bytes = fs
        .read("Data/ItemVisualIdentity.datc64")
        .context("Failed to read ItemVisualIdentity")?;
    let ivi_table = DatParser.parse(&ivi_bytes)?;
    let ivi_schema =
        schema("ItemVisualIdentity", &[1]).context("No schema for ItemVisualIdentity")?;
    let ivi_df =
        parse_table(&ivi_table, ivi_schema).context("Failed to parse ItemVisualIdentity")?;

    // WorldAreas row → (item name, icon) for unique maps;
    // replicas (e.g. "Replica Cortex") are collected separately.
    let mut uniques: std::collections::HashMap<usize, UniqueMapInfo> = Default::default();
    let mut replicas: Vec<UniqueMapInfo> = Vec::new();
    {
        let um_wa = um_df
            .column(um_df.schema().index_of("WorldAreasKey").unwrap())
            .as_any()
            .downcast_ref::<UInt64Array>()
            .unwrap();
        let um_name = um_df
            .column(um_df.schema().index_of("Name").unwrap())
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap();
        let um_words = um_df
            .column(um_df.schema().index_of("WordsKey").unwrap())
            .as_any()
            .downcast_ref::<UInt64Array>()
            .unwrap();
        let words_text = words_df
            .column(words_df.schema().index_of("Text").unwrap())
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap();
        let um_ivi = um_df
            .column(um_df.schema().index_of("ItemVisualIdentityKey").unwrap())
            .as_any()
            .downcast_ref::<UInt64Array>()
            .unwrap();
        let ivi_dds = ivi_df
            .column(ivi_df.schema().index_of("DDSFile").unwrap())
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap();
        let um_unknown = um_df
            .column(4)
            .as_any()
            .downcast_ref::<BooleanArray>()
            .unwrap();
        let um_disabled = um_df
            .column(um_df.schema().index_of("IsDropDisabled").unwrap())
            .as_any()
            .downcast_ref::<BooleanArray>()
            .unwrap();

        for i in 0..um_df.num_rows() {
            let name = if !um_words.is_null(i) {
                words_text.value(um_words.value(i) as usize).to_string()
            } else {
                um_name.value(i).to_string()
            };
            if name.is_empty() || name == "NULL" {
                continue;
            }
            let icon = {
                let row = um_ivi.value(i) as usize;
                if row < ivi_df.num_rows() && !ivi_dds.is_null(row) {
                    cdn_url(ivi_dds.value(row))
                } else {
                    String::new()
                }
            };
            // Replicas share the WorldAreasKey with their base map but are
            // separate items ("Replica Cortex Map" on trade).
            if um_unknown.value(i) && !um_disabled.value(i) {
                replicas.push(UniqueMapInfo {
                    name: format!("{name} Map"),
                    icon,
                });
            } else {
                uniques
                    .entry(um_wa.value(i) as usize)
                    .or_insert(UniqueMapInfo { name, icon });
            }
        }
    }

    // ── Build maps from AtlasNode rows ────────────────────────────
    let mut maps: Vec<Map> = Vec::new();
    for i in 0..an_df.num_rows() {
        let is_norm = an_is_norm.value(i);
        let is_uniq = an_is_uniq.value(i);
        if !is_norm && !is_uniq {
            continue;
        }

        let area_row = an_area.value(i) as usize;
        if area_row >= area_names.len() {
            continue;
        }
        let name = &area_names[area_row];
        if name.is_empty() || name == "NULL" {
            continue;
        }

        let tier = an_tier.value(i) as u32;
        let (name, icon) = if is_uniq {
            match uniques.get(&area_row) {
                Some(u) => (u.name.clone(), u.icon.clone()),
                None => continue,
            }
        } else {
            let d = if an_dds.is_null(i) {
                ""
            } else {
                an_dds.value(i)
            };
            (
                name.clone(),
                if !d.is_empty() {
                    cdn_url(d)
                } else {
                    String::new()
                },
            )
        };

        let atlas_cards = if !div_cards_col.is_null(i) {
            let card_list = div_cards_col.value(i);
            let card_keys = card_list.as_any().downcast_ref::<UInt64Array>().unwrap();
            (0..card_keys.len())
                .filter_map(|j| {
                    let row = card_keys.value(j) as usize;
                    if row < base_item_names.len() {
                        let n = &base_item_names[row];
                        if !n.is_empty() { Some(n.clone()) } else { None }
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            Vec::new()
        };

        maps.push(Map {
            name: name.clone(),
            tier,
            unique: is_uniq,
            icon,
            slug: slug::slugify(&name),
            atlas_cards,
        });
    }

    // Append replica unique maps (not on the atlas).
    for r in replicas {
        maps.push(Map {
            name: r.name.clone(),
            tier: 0,
            unique: true,
            icon: r.icon,
            slug: slug::slugify(&r.name),
            atlas_cards: Vec::new(),
        });
    }

    Ok(maps)
}
