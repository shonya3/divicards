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
    Chest(Chest),
    Delirium,
    Strongbox(Strongbox),
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
        } else if let Ok(strongbox) = Strongbox::from_str(s) {
            return Ok(Source::Strongbox(strongbox));
        } else if let Ok(chest) = Chest::from_str(s) {
            return Ok(Source::Chest(chest));
        }

        Err(strum::ParseError::VariantNotFound)
    }
}

impl std::fmt::Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Source::ExpeditionLogbook => write!(f, "ExpeditionLogbook"),
            Source::Chest(chest) => chest.fmt(f),
            Source::Delirium => write!(f, "Deilirum"),
            Source::Strongbox(strongbox) => strongbox.fmt(f),
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
#[serde(tag = "strongbox")]
pub enum Strongbox {
    #[strum(serialize = "Jeweller's Strongbox")]
    #[serde(rename = "Jeweller's Strongbox")]
    Jeweller,
    #[strum(serialize = "Armourer's Strongbox")]
    #[serde(rename = "Armourer's Strongbox")]
    Armourer,
    #[strum(serialize = "Cartographer's Strongbox")]
    #[serde(rename = "Cartographer's Strongbox")]
    Cartographer,
    #[strum(serialize = "Gemcutter's Strongbox")]
    #[serde(rename = "Gemcutter's Strongbox")]
    Gemcutter,
    #[strum(serialize = "Arcanist's Strongbox")]
    #[serde(rename = "Arcanist's Strongbox")]
    Arcanist,
    #[strum(serialize = "Artisan's Strongbox")]
    #[serde(rename = "Artisan's Strongbox")]
    Artisan,
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
pub enum Chest {
    #[strum(serialize = "Abyssal Trove")]
    #[serde(rename = "Abyssal Trove")]
    AbyssalTrove,
    #[strum(serialize = "Delve chest")]
    #[serde(rename = "Delve chest")]
    DelveChest,
    #[strum(serialize = "Delve Gem Chests")]
    #[serde(rename = "Delve Gem Chests")]
    DelveGemChests,
    #[strum(serialize = "Voltaxic Sulphite")]
    #[serde(rename = "Voltaxic Sulphite")]
    VoltaxicSulphite,
    #[strum(serialize = "Delve Interactables behind Fractured Wall")]
    #[serde(rename = "Delve Interactables behind Fractured Wall")]
    DelveInteractablesBehindFracturedWall,
    #[strum(serialize = "Light Jewellery chest (Primeval Ruins, Abyssal City, Vaal Outpost)")]
    #[serde(rename = "Light Jewellery chest (Primeval Ruins, Abyssal City, Vaal Outpost)")]
    DelveCityLightJewelleryChest,
    #[strum(serialize = "The Maven's Crucible")]
    #[serde(rename = "The Maven's Crucible")]
    MavenCrucible,
    #[strum(serialize = "Map Reward Heist Chests")]
    #[serde(rename = "Map Reward Heist Chests")]
    HeistMapChest,
    #[strum(serialize = "Breach Clasped Hand")]
    #[serde(rename = "Breach Clasped Hand")]
    BreachClaspedHand,
    #[strum(serialize = "Izaro's Treasure")]
    #[serde(rename = "Izaro's Treasure")]
    IzaroTreasure,
    #[strum(serialize = "Vaal Vessel (Vaal Side Areas)")]
    #[serde(rename = "Vaal Vessel (Vaal Side Areas)")]
    VaalVessel,
    #[strum(
        serialize = "Uber Labyrinth/Enriched Labyrinth (Izaro's Treasure, Labyrinth Trove, Curious Lockbox)"
    )]
    #[serde(
        rename = "Uber Labyrinth/Enriched Labyrinth (Izaro's Treasure, Labyrinth Trove, Curious Lockbox)"
    )]
    UberlabChests,
    #[strum(serialize = "Labyrinth Darkshrines")]
    #[serde(rename = "Labyrinth Darkshrines")]
    Darkshrine,
    #[strum(serialize = "Booty Chest (Mao Kun)")]
    #[serde(rename = "Booty Chest (Mao Kun)")]
    BootyChestMaoKun,
}
