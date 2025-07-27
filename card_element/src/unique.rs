//! This module provides utilities for parsing and identifying unique item rewards.
//!
//! It contains the data structures (`UniqueReward`, `UniqueInfo`) for representing
//! unique items, functions for extracting unique item names from raw text, and the
//! core logic (`find_unique_reward`) for resolving a reward name against multiple
//! data sources (specific uniques, base items, and item classes).
use crate::base_items_fetcher::BaseItem;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

// Regex to find the unique item name inside the <uniqueitem>{...} tag.
static UNIQUE_ITEM_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"<uniqueitem>\{([^}]+)\}").unwrap());

/// The final, structured information about a unique item reward.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UniqueReward {
    pub name: String,
    pub item_class: String,
}

/// Represents the raw unique item data from your JSON resource.
/// The JSON is expected to be a map where keys are arbitrary and values
/// have this shape.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UniqueInfo {
    pub name: String,
    pub item_class: String,
}

/// Extracts the unique item name from a poe.ninja explicit modifier string.
pub fn extract_unique_name_from_mod(mod_text: &str) -> Option<String> {
    UNIQUE_ITEM_RE.captures(mod_text).and_then(|captures| {
        captures
            .get(1)
            .map(|unique_name| unique_name.as_str().trim().to_string())
    })
}

/// Builds a map from a unique item's name to its full info for quick lookups.
pub fn build_uniques_map(
    uniques_data: &HashMap<String, UniqueInfo>,
) -> HashMap<String, UniqueInfo> {
    uniques_data
        .values()
        .map(|info| (info.name.clone(), info.clone()))
        .collect()
}

/// Builds a set of all unique item classes for quick lookups.
pub fn build_item_class_set(uniques_data: &HashMap<String, UniqueInfo>) -> HashSet<String> {
    uniques_data
        .values()
        .map(|info| info.item_class.clone())
        .collect()
}

/// Finds a unique reward by checking specific uniques, then base items, then item classes.
pub fn find_unique_reward<'a>(
    name: &str,
    uniques_map: &'a HashMap<String, UniqueInfo>,
    base_items_map: &'a HashMap<String, &'a BaseItem>,
    item_class_set: &'a HashSet<String>,
) -> Option<UniqueReward> {
    if let Some(info) = uniques_map.get(name) {
        // Case 1: A specific unique item.
        Some(UniqueReward {
            name: info.name.clone(),
            item_class: info.item_class.clone(),
        })
    } else if let Some(base_item) = base_items_map.get(name) {
        // Case 2: A unique base type, like "Timeless Jewel".
        Some(UniqueReward {
            name: name.to_string(),
            item_class: base_item.item_class.clone(),
        })
    } else if item_class_set.contains(name) {
        // Case 3: A generic unique item class, like "Jewel" or "Ring".
        Some(UniqueReward {
            name: name.to_string(),
            item_class: name.to_string(),
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_unique_name_from_mod_with_unique() {
        let mod_text = "<uniqueitem>{The Apothecary}";
        assert_eq!(
            extract_unique_name_from_mod(mod_text),
            Some("The Apothecary".to_string())
        );
    }

    #[test]
    fn test_extract_unique_name_from_mod_with_unique_and_whitespace() {
        let mod_text = "<uniqueitem>{  Mageblood  }";
        assert_eq!(
            extract_unique_name_from_mod(mod_text),
            Some("Mageblood".to_string())
        );
    }

    #[test]
    fn test_extract_unique_name_from_mod_without_unique() {
        let mod_text = "<currencyitem>{Mirror of Kalandra}";
        assert_eq!(extract_unique_name_from_mod(mod_text), None);
    }

    #[test]
    fn test_extract_unique_name_from_mod_with_empty_string() {
        let mod_text = "";
        assert_eq!(extract_unique_name_from_mod(mod_text), None);
    }

    #[test]
    fn test_extract_unique_name_from_mod_with_other_text() {
        let mod_text = "Some other text <uniqueitem>{Headhunter} and more text";
        assert_eq!(
            extract_unique_name_from_mod(mod_text),
            Some("Headhunter".to_string())
        );
    }

    #[test]
    fn test_find_unique_reward() {
        // --- Mock Data Setup ---
        let mut uniques_map = HashMap::new();
        uniques_map.insert(
            "Mageblood".to_string(),
            UniqueInfo {
                name: "Mageblood".to_string(),
                item_class: "Belt".to_string(),
            },
        );
        uniques_map.insert(
            "Unnatural Instinct".to_string(),
            UniqueInfo {
                name: "Unnatural Instinct".to_string(),
                item_class: "Jewel".to_string(),
            },
        );

        let timeless_jewel = BaseItem {
            name: "Timeless Jewel".to_string(),
            item_class: "Jewel".to_string(),
        };
        let mut base_items_map = HashMap::new();
        base_items_map.insert("Timeless Jewel".to_string(), &timeless_jewel);

        let mut item_class_set = HashSet::new();
        item_class_set.insert("Jewel".to_string());
        item_class_set.insert("Belt".to_string());

        // --- Test Cases ---

        // Case 1: Specific unique item
        let result1 =
            find_unique_reward("Mageblood", &uniques_map, &base_items_map, &item_class_set);
        assert_eq!(result1.unwrap().item_class, "Belt");

        // Case 2: Unique base type
        let result2 = find_unique_reward(
            "Timeless Jewel",
            &uniques_map,
            &base_items_map,
            &item_class_set,
        );
        assert_eq!(result2.unwrap().item_class, "Jewel");

        // Case 3: Generic item class
        let result3 = find_unique_reward("Jewel", &uniques_map, &base_items_map, &item_class_set);
        assert_eq!(result3.unwrap().item_class, "Jewel");

        // Case 4: No match
        let result4 = find_unique_reward(
            "Random Item",
            &uniques_map,
            &base_items_map,
            &item_class_set,
        );
        assert!(result4.is_none());
    }
}
