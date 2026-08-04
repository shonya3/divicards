//! Item class resolution for unique rewards.
//!
//! Fetches uniques.json and base_items.json from RePoE fork, then resolves
//! a reward name to an item class via:
//! 1. Hardcoded special cases (items not in RePoE)
//! 2. Specific unique item lookup
//! 3. Base item lookup
//!
//! (Generic categories and caller-side fallback handle the rest)

use std::collections::HashMap;

pub struct ItemDb {
    uniques: HashMap<String, String>,
    base_items: HashMap<String, String>,
}

const SPECIAL: &[(&str, &str)] = &[
    ("The Original Scripture", "Relic"),
    ("Torrent's Reclamation", "Belt"),
    ("Attribute Transforming Jewel", "Jewel"),
];

impl ItemDb {
    pub async fn load() -> anyhow::Result<Self> {
        let (uniques, base_items) = tokio::join!(
            Self::fetch_map("https://repoe-fork.github.io/uniques.json"),
            Self::fetch_map("https://repoe-fork.github.io/base_items.json"),
        );
        Ok(Self {
            uniques: uniques?,
            base_items: base_items?,
        })
    }

    async fn fetch_map(url: &str) -> anyhow::Result<HashMap<String, String>> {
        let resp = reqwest::get(url).await?.error_for_status()?;
        let data: HashMap<String, serde_json::Value> = resp.json().await?;
        let mut map = HashMap::new();
        for entry in data.values() {
            let name = entry.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let cls = entry
                .get("item_class")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            if !name.is_empty() && !cls.is_empty() {
                map.insert(name.to_string(), cls.to_string());
            }
        }
        Ok(map)
    }

    pub fn resolve_item_class(&self, name: &str) -> Option<&str> {
        for &(k, v) in SPECIAL {
            if name == k {
                return Some(v);
            }
        }
        if let Some(cls) = self.uniques.get(name) {
            return Some(cls.as_str());
        }
        if let Some(cls) = self.base_items.get(name) {
            return Some(cls.as_str());
        }
        None
    }

    pub fn uniques_count(&self) -> usize {
        self.uniques.len()
    }
    pub fn base_items_count(&self) -> usize {
        self.base_items.len()
    }
}
