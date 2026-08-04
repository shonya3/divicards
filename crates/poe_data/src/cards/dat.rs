//! Game file extraction for divination cards.
//!
//! Parses `.datc64` tables to extract raw card metadata:
//!
//! | File | Table | Columns used |
//! |---|---|---|
//! | `BaseItemTypes.datc64` | `BaseItemTypes` | `Id`, `Name`, `DropLevel`, `InheritsFrom`, `FlavourTextKey` |
//! | `DivinationCardArt.datc64` | `DivinationCardArt` | `BaseItemTypesKey`, `VirtualFile` |
//! | `FlavourText.datc64` | `FlavourText` | `Text` |
//! | `CurrencyItems.datc64` | `CurrencyItems` | `BaseItemTypesKey`, `StackSize` |

use divcord::poe_data::cards::Card;
use anyhow::{Context, Result};
use arrow_array::{Array, StringArray, UInt64Array};
use poe_data_tools::{
    dat::{schema::SchemaCollection, table::parse_table},
    file_parsers::{FileParser, dat::DatParser},
    fs::FileSystem,
};
use slug;
use std::collections::HashMap;

pub fn extract(fs: &impl FileSystem, schemas: &SchemaCollection) -> Result<Vec<Card>> {
    let schema = |name: &str| {
        schemas
            .tables
            .iter()
            .filter(|t| t.valid_for == 1 || t.valid_for == 3)
            .find(|t| t.name.eq_ignore_ascii_case(name))
    };

    // DivinationCardArt
    let art_bytes = fs
        .read("Data/DivinationCardArt.datc64")
        .context("Failed to read DivinationCardArt")?;
    let art_table = DatParser.parse(&art_bytes)?;
    let art_schema = schema("DivinationCardArt").context("No schema for DivinationCardArt")?;
    let art_df =
        parse_table(&art_table, art_schema).context("Failed to parse DivinationCardArt")?;
    let art_base_item_keys = art_df
        .column(art_df.schema().index_of("BaseItemTypesKey").unwrap())
        .as_any()
        .downcast_ref::<UInt64Array>()
        .unwrap();
    // 3.29 renamed `VirtualFile` → `ArtFile`; fall back so both schemas work
    let art_virtual_files = art_df
        .column(
            art_df
                .schema()
                .index_of("VirtualFile")
                .unwrap_or_else(|_| art_df.schema().index_of("ArtFile").unwrap()),
        )
        .as_any()
        .downcast_ref::<StringArray>()
        .unwrap();
    let mut art_map: HashMap<usize, String> = HashMap::new();
    for j in 0..art_df.num_rows() {
        let key = art_base_item_keys.value(j) as usize;
        let vf = art_virtual_files.value(j).to_string();
        if !vf.is_empty() {
            art_map.insert(key, vf);
        }
    }

    // FlavourText
    let ft_bytes = fs
        .read("Data/FlavourText.datc64")
        .context("Failed to read FlavourText")?;
    let ft_table = DatParser.parse(&ft_bytes)?;
    let ft_schema = schema("FlavourText").context("No schema for FlavourText")?;
    let ft_df = parse_table(&ft_table, ft_schema).context("Failed to parse FlavourText")?;
    let ft_texts = ft_df
        .column(ft_df.schema().index_of("Text").unwrap())
        .as_any()
        .downcast_ref::<StringArray>()
        .unwrap();
    let flavour_texts: Vec<String> = (0..ft_df.num_rows())
        .map(|i| ft_texts.value(i).to_string())
        .collect();

    // CurrencyItems
    let ci_bytes = fs
        .read("Data/CurrencyItems.datc64")
        .context("Failed to read CurrencyItems")?;
    let ci_table = DatParser.parse(&ci_bytes)?;
    let ci_schema = schema("CurrencyItems").context("No schema for CurrencyItems")?;
    let ci_df = parse_table(&ci_table, ci_schema).context("Failed to parse CurrencyItems")?;
    let ci_base_item_keys = ci_df
        .column(ci_df.schema().index_of("BaseItemTypesKey").unwrap())
        .as_any()
        .downcast_ref::<UInt64Array>()
        .unwrap();
    let ci_stack_sizes = ci_df
        .column(ci_df.schema().index_of("StackSize").unwrap())
        .as_any()
        .downcast_ref::<arrow_array::Int32Array>()
        .unwrap();
    let mut stack_size_map: HashMap<usize, u32> = HashMap::new();
    for j in 0..ci_df.num_rows() {
        let key = ci_base_item_keys.value(j) as usize;
        stack_size_map.insert(key, ci_stack_sizes.value(j).max(0) as u32);
    }

    // BaseItemTypes
    let bit_bytes = fs
        .read("Data/BaseItemTypes.datc64")
        .context("Failed to read BaseItemTypes")?;
    let bit_table = DatParser.parse(&bit_bytes)?;
    let bit_schema = schema("BaseItemTypes").context("No schema for BaseItemTypes")?;
    let bit_df = parse_table(&bit_table, bit_schema).context("Failed to parse BaseItemTypes")?;
    let names = bit_df
        .column(bit_df.schema().index_of("Name").unwrap())
        .as_any()
        .downcast_ref::<StringArray>()
        .unwrap();
    let ids = bit_df
        .column(bit_df.schema().index_of("Id").unwrap())
        .as_any()
        .downcast_ref::<StringArray>()
        .unwrap();
    let drop_levels = bit_df
        .column(bit_df.schema().index_of("DropLevel").unwrap())
        .as_any()
        .downcast_ref::<arrow_array::Int32Array>()
        .unwrap();
    let inherits = bit_df
        .column(bit_df.schema().index_of("InheritsFrom").unwrap())
        .as_any()
        .downcast_ref::<StringArray>()
        .unwrap();
    let flavour_keys = bit_df
        .column(
            bit_df
                .schema()
                .index_of("FlavourTextKey")
                .unwrap_or_else(|_| bit_df.schema().index_of("FlavourText").unwrap()),
        )
        .as_any()
        .downcast_ref::<UInt64Array>()
        .unwrap();

    let mut cards = Vec::new();
    for i in 0..bit_df.num_rows() {
        if !inherits.value(i).contains("AbstractDivinationCard") {
            continue;
        }
        let key = flavour_keys.value(i) as usize;
        let flavour = if key > 0 && key < flavour_texts.len() {
            super::markup::clean_flavour_text(&flavour_texts[key])
        } else {
            String::new()
        };
        let art_filename = art_map
            .get(&i)
            .map(|vf| {
                vf.rsplit_once('/')
                    .map(|(_, name)| name.to_string())
                    .unwrap_or(vf.to_string())
            })
            .unwrap_or_default();
        let stack_size = stack_size_map.get(&i).copied().unwrap_or(1);
        let name = names.value(i).to_string();
        cards.push(Card {
            slug: slug::slugify(&name),
            id: ids.value(i).to_string(),
            name,
            min_level: drop_levels.value(i) as u32,
            flavour_text: flavour,
            art_filename,
            stack_size,
            weights: HashMap::new(),
            price: None,
            league: None,
            atlas_maps: Vec::new(),
            disabled: false,
        });
    }
    Ok(cards)
}
