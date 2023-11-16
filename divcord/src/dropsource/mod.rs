pub mod area;
pub mod monster;
pub mod parse;

use std::str::FromStr;

use serde::{ser::SerializeStruct, Deserialize, Serialize};
use strum::IntoEnumIterator;

use self::{area::Area, monster::UniqueMonster};

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash, strum_macros::EnumIter, Default)]
#[serde(tag = "type")]
pub enum Source {
    #[default]
    #[serde(rename = "Expedition Logbook")]
    ExpeditionLogbook,
    Chest(Chest),
    Delirium,
    Strongbox(Strongbox),
    Vendor(Vendor),
    Unknown,
    #[serde(rename = "Delirium Currency Rewards")]
    DeliriumCurrencyRewards,
    #[serde(rename = "Redeemer influenced maps")]
    RedeemerInfluencedMaps,
    Disabled,
    #[serde(rename = "Global Drop")]
    GlobalDrop {
        min_level: Option<u32>,
        max_level: Option<u32>,
    },
    Act {
        id: String,
    },
    Map {
        name: String,
    },
    MapBoss {
        name: String,
    },
    ActBoss {
        name: String,
    },
    #[serde(untagged)]
    UniqueMonster(UniqueMonster),
    #[serde(untagged)]
    Area(Area),
}

impl Source {
    pub fn _type(&self) -> &str {
        match self {
            Source::ExpeditionLogbook => "Expedition Logbook",
            Source::Chest(_) => "Chest",
            Source::Delirium => "Delirium",
            Source::Strongbox(_) => "Strongbox",
            Source::Vendor(_) => "Vendor",
            Source::Unknown => "Unknown",
            Source::DeliriumCurrencyRewards => "Delirium Currency Rewards",
            Source::RedeemerInfluencedMaps => "Redeemer influenced maps",
            Source::Disabled => "Disabled",
            Source::GlobalDrop { .. } => "Global Drop",
            Source::Act { .. } => "Act",
            Source::Map { .. } => "Map",
            Source::MapBoss { .. } => "Map Boss",
            Source::ActBoss { .. } => "Act Boss",
            Source::UniqueMonster(monster) => monster._type(),
            Source::Area(area) => area._type(),
        }
    }

    pub fn _id(&self) -> String {
        self.to_string()
    }

    pub fn types() -> Vec<String> {
        let mut vec: Vec<String> = vec![];
        Source::iter().for_each(|source| match source {
            Source::UniqueMonster(_) => vec.extend(UniqueMonster::_types()),
            Source::Area(_) => vec.extend(Area::_types()),
            _ => vec.push(source._type().to_string()),
        });
        vec
    }

    pub fn write_typescript_file() -> std::io::Result<()> {
        let mut _types: String = Source::types()
            .into_iter()
            .map(|t| {
                let q = match t.contains("'") {
                    true => "\"",
                    false => "'",
                };
                format!("\n\t| {q}{t}{q}")
            })
            .collect();

        let s = format!(
            r#"export type SourceWithMember = {{ type: SourceType; id: string; kind: 'source-with-member' }};
export type EmptySource = {{ type: SourceType; kind: 'empty-source' }};
export type ISource = SourceWithMember | EmptySource;

export type SourceType = {_types};
    "#,
        );

        std::fs::write("ISource.interface.ts", &s)?;

        Ok(())
    }
}

impl Serialize for Source {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut source = serializer.serialize_struct("Source", 3)?;
        let _type = self._type();
        let _id = self._id();

        source.serialize_field("type", _type)?;
        match _type == _id {
            true => {
                source.skip_field("id")?;
                source.serialize_field("kind", "empty-source")?;
            }
            false => {
                source.serialize_field("id", &self._id())?;
                source.serialize_field("kind", "source-with-member")?;
            }
        }

        source.end()
    }
}

impl FromStr for Source {
    type Err = strum::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "Redeemer influenced maps" {
            return Ok(Source::RedeemerInfluencedMaps);
        }
        if s == "Delirium Currency Rewards" {
            return Ok(Source::DeliriumCurrencyRewards);
        }
        if s == "Global Drop" {
            return Ok(Source::GlobalDrop {
                min_level: None,
                max_level: None,
            });
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
            Source::GlobalDrop { .. } => write!(f, "Global Drop"),
            Source::UniqueMonster(uniquemonster) => uniquemonster.fmt(f),
            Source::Area(area) => area.fmt(f),
            Source::Vendor(vendor) => vendor.fmt(f),
            Source::ActBoss { name } => write!(f, "{name}"),
            Source::Map { name } => write!(f, "{name}"),
            Source::MapBoss { name } => write!(f, "{name}"),
            Source::Act { id } => write!(f, "{id}"),
            Source::DeliriumCurrencyRewards => write!(f, "Delirium Currency Rewards"),
            Source::RedeemerInfluencedMaps => write!(f, "Redeemer influenced maps"),
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
    strum_macros::EnumIter,
    Default,
)]
#[serde(tag = "name")]
pub enum Vendor {
    #[default]
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
    strum_macros::EnumIter,
    Default,
)]
#[serde(tag = "name")]
pub enum Strongbox {
    #[default]
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
    strum_macros::EnumIter,
    Default,
)]
#[serde(tag = "name")]
pub enum Chest {
    #[default]
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

pub fn poedb_page_url(boss: &str) {
    let name = boss.split("(").next().unwrap().trim();
    let name = name.replace(" ", "_");
    let name = name.replace(",", "%2C");
    format!("https://poedb.tw/us/{name}");
}
