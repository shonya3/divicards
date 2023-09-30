pub mod area;
pub mod dropconsts;
pub mod monster;

use std::str::FromStr;

use serde::{Deserialize, Serialize};

use self::{area::Area, monster::UniqueMonster};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(tag = "source")]
pub enum Source {
    ExpeditionLogbook,
    Chest,
    Delirium,
    Strongbox,
    Unknown,
    Disabled,
    #[serde(rename = "Global Drop")]
    GlobalDrop,
    #[serde(rename = "uniqueMonster")]
    UniqueMonster(UniqueMonster),
    #[serde(rename = "area")]
    Area(Area),
    Vendor(Vendor),
}

impl FromStr for Source {
    type Err = strum::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "Global Drop" {
            return Ok(Source::GlobalDrop);
        }
        if let Ok(uniquemonster) = UniqueMonster::from_str(s) {
            return Ok(Source::UniqueMonster(uniquemonster));
        } else if let Ok(area) = Area::from_str(s) {
            return Ok(Source::Area(area));
        } else if let Ok(vendor) = Vendor::from_str(s) {
            return Ok(Source::Vendor(vendor));
        }

        Err(strum::ParseError::VariantNotFound)
    }
}

impl std::fmt::Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Source::ExpeditionLogbook => write!(f, "ExpeditionLogbook"),
            Source::Chest => write!(f, "Chest"),
            Source::Delirium => write!(f, "Deilirum"),
            Source::Strongbox => write!(f, "Strongbox"),
            Source::Unknown => write!(f, "Unknown"),
            Source::Disabled => write!(f, "Disabled"),
            Source::GlobalDrop => write!(f, "Global Drop"),
            Source::UniqueMonster(uniquemonster) => uniquemonster.fmt(f),
            Source::Area(area) => area.fmt(f),
            Source::Vendor(vendor) => vendor.fmt(f),
        }
    }
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Hash,
    strum_macros::EnumString,
    strum_macros::Display,
)]
#[serde(tag = "vendor")]
pub enum Vendor {
    #[strum(serialize = "Kirac shop")]
    #[serde(rename = "Kirac shop")]
    KiracShop,
}
