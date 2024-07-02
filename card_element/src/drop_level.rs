use poe_data::cards::Card;
use serde::{Deserialize, Serialize};

pub fn extract_drop_level(card: Option<&Card>) -> DropLevel {
    let Some(card) = card else {
        return DropLevel::default();
    };

    let level = Level {
        min: card.min_level,
        max: card.max_level,
    };

    DropLevel {
        label: create_label(&level),
        level,
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DropLevel {
    pub level: Level,
    pub label: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Level {
    pub min: Option<u32>,
    pub max: Option<u32>,
}

fn create_label(Level { min, max }: &Level) -> String {
    let min = min.unwrap_or_default();
    let Some(max) = max else {
        return format!("{min}+");
    };

    format!("{min}-{max}")
}
