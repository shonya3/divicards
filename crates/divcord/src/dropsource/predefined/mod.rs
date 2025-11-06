//! Hardcoded drop sources that don't rely on [PoeData].

use super::id::Identified;
use area::Area;
use monster::UniqueMonster;
use other::{Chest, Strongbox, Vendor};
use std::str::FromStr;
use strum::IntoEnumIterator;

pub mod area;
pub mod monster;
mod other;

/// Hardcoded drop sources that don't rely on [PoeData]
#[derive(Debug, Clone, PartialEq, Eq, Hash, strum_macros::EnumIter, Default)]
pub enum PredefinedSource {
    UniqueMonster(UniqueMonster),
    Area(Area),

    Chest(Chest),
    Strongbox(Strongbox),
    Vendor(Vendor),

    KiracMissions,
    MaelstromOfChaosWithBarrelSextant,
    Delirium,
    DeliriumCurrencyRewards,
    #[default]
    Disabled,
}

impl PredefinedSource {
    pub fn types() -> Vec<String> {
        let mut vec: Vec<String> = vec![];

        PredefinedSource::iter().for_each(|variant| match variant {
            PredefinedSource::UniqueMonster(_) => vec.extend(UniqueMonster::_types()),
            PredefinedSource::Area(_) => vec.extend(Area::_types()),
            _ => vec.push(variant._type().to_string()),
        });

        vec
    }
}

/// `s.parse::<Source>` validates only predefined sources.
/// It does not account for [PoeData] and cannot be used independently.
/// Use it solely as the initial step in drop source parsing.
#[derive(Debug)]
pub struct UnknownPredefinedSource(pub String);

impl FromStr for PredefinedSource {
    type Err = UnknownPredefinedSource;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Kirac Missions" => Ok(PredefinedSource::KiracMissions),
            "Disabled" => Ok(PredefinedSource::Disabled),
            "Delirium Currency Rewards" | "Delirium Currency reward" => {
                Ok(PredefinedSource::DeliriumCurrencyRewards)
            }
            "Maelström of Chaos with Barrel Sextant" => {
                Ok(PredefinedSource::MaelstromOfChaosWithBarrelSextant)
            }
            _ => UniqueMonster::from_str(s)
                .map(Self::UniqueMonster)
                .or_else(|_| Area::from_str(s).map(Self::Area))
                .or_else(|_| Vendor::from_str(s).map(Self::Vendor))
                .or_else(|_| Strongbox::from_str(s).map(Self::Strongbox))
                .or_else(|_| Chest::from_str(s).map(Self::Chest))
                .map_err(|_| UnknownPredefinedSource(s.to_owned())),
        }
    }
}

impl PredefinedSource {
    pub fn _type(&self) -> &str {
        match self {
            PredefinedSource::UniqueMonster(monster) => monster._type(),
            PredefinedSource::Area(area) => area._type(),

            PredefinedSource::Chest(_) => "Chest",
            PredefinedSource::Strongbox(_) => "Strongbox",
            PredefinedSource::Vendor(_) => "Vendor",

            PredefinedSource::Delirium => "Delirium",
            PredefinedSource::DeliriumCurrencyRewards => "Delirium Currency Rewards",
            PredefinedSource::Disabled => "Disabled",
            PredefinedSource::MaelstromOfChaosWithBarrelSextant => {
                "Maelström of Chaos with Barrel Sextant"
            }
            PredefinedSource::KiracMissions => "Kirac Missions",
        }
    }
}

impl Identified for PredefinedSource {
    fn id(&self) -> &str {
        match self {
            PredefinedSource::UniqueMonster(m) => m.id(),
            PredefinedSource::Area(a) => a.id(),

            PredefinedSource::Chest(chest) => chest.id(),
            PredefinedSource::Strongbox(strongbox) => strongbox.id(),
            PredefinedSource::Vendor(vendor) => vendor.id(),

            PredefinedSource::Delirium => "Delirium",
            PredefinedSource::DeliriumCurrencyRewards => "Delirium Currency Rewards",
            PredefinedSource::Disabled => "Disabled",
            PredefinedSource::MaelstromOfChaosWithBarrelSextant => {
                "Maelström of Chaos with Barrel Sextant"
            }
            PredefinedSource::KiracMissions => "Kirac Missions",
        }
    }
}
