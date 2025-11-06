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
    // Handle special, abstract unique categories and specific items that aren't in the data files.
    // This is a fallback layer for when the primary data sources are incomplete.
    match name {
        // https://divicards-site.pages.dev/card/damnation
        "The Original Scripture" => {
            return Some(UniqueReward {
                name: name.to_string(),
                item_class: "Relic".to_string(),
            })
        }

        // http:///divicards-site.pages.dev/card/deadly-joy
        "Torrent's Reclamation" => {
            return Some(UniqueReward {
                name: name.to_string(),
                item_class: "Belt".to_string(),
            })
        }

        // https://divicards-site.pages.dev/card/the-unexpected-prize
        "Attribute Transforming Jewel" => {
            return Some(UniqueReward {
                name: name.to_string(),
                item_class: "Jewel".to_string(),
            })
        }

        "Vaal Aspect"
        | "League-Specific Item"
        | "Maven Item"
        | "Farrul Item"
        | "Chayula Item"
        | "Atziri Item"
        | "Nemesis Item"
        | "Breach Item"
        | "Synthesis Item"
        | "Bestiary Item"
        | "Delve Item"
        | "Harbinger Piece"
        | "Item"
        | "Fishing Item"
        | "Synthesis Map"
        | "Replica Item"
        | "Weapon"
        | "Lioneye Item"
        | "Doedre Item"
        | "Shavronne Item"
        | "Agnerod Staff"
        | "Rigwald Item"
        | "Jewellery"
        | "Metamorph Item"
        | "Beyond Item" => {
            return Some(UniqueReward {
                name: name.to_string(),
                item_class: name.to_string(),
            })
        }

        _ => {}
    }

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

        // A map of specific unique items (from uniques.json)
        let mut uniques_map = HashMap::new();
        uniques_map.insert(
            "Mageblood".to_string(),
            UniqueInfo {
                name: "Mageblood".to_string(),
                item_class: "Belt".to_string(),
            },
        );

        // A map of base items (from base_items.json)
        let timeless_jewel_base = BaseItem {
            name: "Timeless Jewel".to_string(),
            item_class: "Jewel".to_string(),
        };
        let mut base_items_map = HashMap::new();
        base_items_map.insert("Timeless Jewel".to_string(), &timeless_jewel_base);

        // A comprehensive set of all possible item classes from both data sources
        let mut item_class_set = HashSet::new();
        item_class_set.insert("Jewel".to_string());
        item_class_set.insert("Belt".to_string());
        item_class_set.insert("Relic".to_string());

        // --- Test Cases ---
        struct TestCase<'a> {
            name: &'a str,
            input: &'a str,
            expected_name: &'a str,
            expected_class: &'a str,
        }

        let test_cases = vec![
            TestCase {
                name: "Case 0: Special 'League-Specific Item'",
                input: "League-Specific Item",
                expected_name: "League-Specific Item",
                expected_class: "League-Specific Item",
            },
            TestCase {
                name: "Case 1: Specific unique item ('Mageblood')",
                input: "Mageblood",
                expected_name: "Mageblood",
                expected_class: "Belt",
            },
            TestCase {
                name: "Case 2: Unique base type ('Timeless Jewel')",
                input: "Timeless Jewel",
                expected_name: "Timeless Jewel",
                expected_class: "Jewel",
            },
            TestCase {
                name: "Case 3: Generic item class ('Relic')",
                input: "Relic",
                expected_name: "Relic",
                expected_class: "Relic",
            },
            TestCase {
                name: "Case 4: Special case unique ('The Original Scripture')",
                input: "The Original Scripture",
                expected_name: "The Original Scripture",
                expected_class: "Relic",
            },
        ];

        for case in test_cases {
            let result =
                find_unique_reward(case.input, &uniques_map, &base_items_map, &item_class_set)
                    .unwrap_or_else(|| panic!("Test case '{}' failed: got None", case.name));

            assert_eq!(
                result.name, case.expected_name,
                "Failed name for case: {}",
                case.name
            );
            assert_eq!(
                result.item_class, case.expected_class,
                "Failed item_class for case: {}",
                case.name
            );
        }

        // --- Test No Match ---
        let no_match_result = find_unique_reward(
            "Random Item",
            &uniques_map,
            &base_items_map,
            &item_class_set,
        );
        assert!(
            no_match_result.is_none(),
            "Expected None for non-existent item"
        );
    }
}
