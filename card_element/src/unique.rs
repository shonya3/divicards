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

/// Builds a set of all unique item classes.
pub fn build_item_class_set(uniques_data: &HashMap<String, UniqueInfo>) -> HashSet<String> {
    uniques_data
        .values()
        .map(|info| info.item_class.clone())
        .collect()
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
}
