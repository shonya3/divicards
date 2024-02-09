pub mod area;
pub mod id;
pub mod monster;
pub mod parse;

use std::str::FromStr;

use serde::{de, ser::SerializeStruct, Deserialize, Serialize};
use strum::IntoEnumIterator;

use self::{area::Area, id::Identified, monster::UniqueMonster};

#[derive(Debug, Clone, PartialEq, Eq, Hash, strum_macros::EnumIter)]
pub enum Source {
    Act(String),
    Map(String),
    ActBoss(String),
    MapBoss(String),

    UniqueMonster(UniqueMonster),
    Area(Area),

    Chest(Chest),
    Strongbox(Strongbox),
    Vendor(Vendor),

    MaelstromOfChaosWithBarrelSextant,
    Delirium,
    DeliriumCurrencyRewards,
    Disabled,
    GlobalDrop {
        min_level: Option<u32>,
        max_level: Option<u32>,
    },
}

impl<'de> Deserialize<'de> for Source {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct JSSource {
            id: Option<String>,
            #[serde(rename = "type")]
            _type: String,
            kind: SourceKind,
        }

        let JSSource { id, _type, kind } = JSSource::deserialize(deserializer)?;
        match kind {
            SourceKind::EmptySource => match _type.parse::<Source>() {
                Ok(source) => Ok(source),
                Err(_) => Err(de::Error::custom(format!(
                    "Could not deserialize Source. {_type}"
                ))),
            },
            SourceKind::SourceWithMember => {
                let id = id.unwrap();
                match id.parse::<Source>() {
                    Ok(source) => Ok(source),
                    Err(_) => match _type.as_str() {
                        "Map" => Ok(Source::Map(id)),
                        "Map Boss" => Ok(Source::MapBoss(id)),
                        "Act" => Ok(Source::Act(id)),
                        "Act Boss" => Ok(Source::ActBoss(id)),
                        _ => Err(de::Error::custom(format!(
                            "Could not deserialize Source. {_type} {id}"
                        ))),
                    },
                }
            }
        }
    }
}

impl Identified for Source {
    fn id(&self) -> &str {
        match self {
            Source::Act(id) => id.as_str(),
            Source::Map(name) => name.as_str(),
            Source::ActBoss(name) => name.as_str(),
            Source::MapBoss(name) => name.as_str(),

            Source::UniqueMonster(m) => m.id(),
            Source::Area(a) => a.id(),

            Source::Chest(chest) => chest.id(),
            Source::Strongbox(strongbox) => strongbox.id(),
            Source::Vendor(vendor) => vendor.id(),

            Source::Delirium => "Delirium",
            Source::DeliriumCurrencyRewards => "Delirium Currency Rewards",
            Source::Disabled => "Disabled",
            Source::GlobalDrop { .. } => "Global Drop",
            Source::MaelstromOfChaosWithBarrelSextant => "Maelström of Chaos with Barrel Sextant",
        }
    }
}

impl Source {
    pub fn _type(&self) -> &str {
        match self {
            Source::Act { .. } => "Act",
            Source::Map { .. } => "Map",
            Source::ActBoss { .. } => "Act Boss",
            Source::MapBoss { .. } => "Map Boss",

            Source::UniqueMonster(monster) => monster._type(),
            Source::Area(area) => area._type(),

            Source::Chest(_) => "Chest",
            Source::Strongbox(_) => "Strongbox",
            Source::Vendor(_) => "Vendor",

            Source::Delirium => "Delirium",
            Source::DeliriumCurrencyRewards => "Delirium Currency Rewards",
            Source::Disabled => "Disabled",
            Source::GlobalDrop { .. } => "Global Drop",
            Source::MaelstromOfChaosWithBarrelSextant => "Maelström of Chaos with Barrel Sextant",
        }
    }

    pub fn _id(&self) -> &str {
        self.id()
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

    pub fn typescript_types() -> String {
        let mut buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"\t");
        let mut serializer = serde_json::Serializer::with_formatter(&mut buf, formatter);
        Source::types().serialize(&mut serializer).unwrap();
        let _types = String::from_utf8(buf).unwrap();

        let s = format!(
            r#"export type SourceWithMember = {{ type: SourceType; id: string; kind: SourceWithMemberKind; min_level?: number; max_level?: number }};
export type EmptySourceKind = 'empty-source';
export type SourceWithMemberKind = 'source-with-member';
export type Kind = EmptySourceKind | SourceWithMemberKind;
export type EmptySource = {{ type: SourceType; id: string; kind: EmptySourceKind; min_level?: number; max_level?: number}};
export type ISource = SourceWithMember | EmptySource;
export const sourceTypes = {_types} as const;

export type SourceType = (typeof sourceTypes)[number];
    "#,
        );

        s
    }
}

#[derive(Deserialize, Serialize)]
pub enum SourceKind {
    #[serde(rename = "empty-source")]
    EmptySource,
    #[serde(rename = "source-with-member")]
    SourceWithMember,
}

impl Serialize for Source {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut source = serializer.serialize_struct("Source", 5)?;
        let _type = self._type();
        let _id = self._id();

        source.serialize_field("type", _type)?;
        match _type == _id {
            true => {
                source.serialize_field("id", &self._id())?;
                source.serialize_field("kind", "empty-source")?;
            }
            false => {
                source.serialize_field("id", &self._id())?;
                source.serialize_field("kind", "source-with-member")?;
            }
        }

        if let Source::GlobalDrop {
            min_level,
            max_level,
        } = self
        {
            match max_level {
                Some(max_level) => source.serialize_field("max_level", max_level)?,
                None => source.skip_field("max_level")?,
            };

            match min_level {
                Some(min_level) => source.serialize_field("min_level", min_level)?,
                None => source.skip_field("min_level")?,
            };
        } else {
            source.skip_field("min_level")?;
            source.skip_field("max_level")?;
        }

        source.end()
    }
}

impl FromStr for Source {
    type Err = strum::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "Redeemer influenced maps" {
            return Ok(Source::Area(Area::RedeemerInfluencedMaps));
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

        match s {
            "Disabled" => return Ok(Source::Disabled),
            // "Unknown" => return Ok(Source::Unknown),
            "Redeemer influenced maps" => return Ok(Source::Area(Area::RedeemerInfluencedMaps)),
            "Delirium Currency Rewards" => return Ok(Source::DeliriumCurrencyRewards),
            "Maelström of Chaos with Barrel Sextant" => {
                return Ok(Source::MaelstromOfChaosWithBarrelSextant)
            }
            "Global Drop" => {
                return Ok(Source::GlobalDrop {
                    min_level: None,
                    max_level: None,
                })
            }
            _ => {}
        };

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
        write!(f, "{}", self.id())
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

impl Identified for Vendor {
    fn id(&self) -> &str {
        match self {
            Vendor::KiracShop => "Kirac shop",
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

impl Identified for Strongbox {
    fn id(&self) -> &str {
        match self {
            Strongbox::Jeweller => "Jeweller's Strongbox",
            Strongbox::Armourer => "Armourer's Strongbox",
            Strongbox::Cartographer => "Cartographer's Strongbox",
            Strongbox::Gemcutter => "Gemcutter's Strongbox",
            Strongbox::Arcanist => "Arcanist's Strongbox",
            Strongbox::Artisan => "Artisan's Strongbox",
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
        serialize = "Uber Labyrinth/Enriched Labyrinth (Izaro's Treasure, Labyrinth Trove, Curious Lockbox)",
        to_string = "Uber Labyrinth or Enriched Labyrinth (Izaro's Treasure, Labyrinth Trove, Curious Lockbox)"
    )]
    #[serde(
        rename = "Uber Labyrinth or Enriched Labyrinth (Izaro's Treasure, Labyrinth Trove, Curious Lockbox)",
        alias = "Uber Labyrinth/Enriched Labyrinth (Izaro's Treasure, Labyrinth Trove, Curious Lockbox)"
    )]
    UberlabChests,
    #[strum(
        serialize = "Merciless Labyrinth",
        to_string = "Merciless Labyrinth (Izaro's Treasure, Labyrinth Trove, Curious Lockbox, Hidden Coffer)"
    )]
    #[serde(
        rename = "Merciless Labyrinth (Izaro's Treasure, Labyrinth Trove, Curious Lockbox, Hidden Coffer)",
        alias = "Merciless Labyrinth"
    )]
    MercilessChests,
    #[serde(rename = "Hidden Coffer")]
    #[strum(to_string = "Hidden Coffer")]
    HiddenCoffer,
    #[strum(serialize = "Labyrinth Darkshrines")]
    #[serde(rename = "Labyrinth Darkshrines")]
    Darkshrine,
    #[strum(serialize = "Booty Chest (Mao Kun)")]
    #[serde(rename = "Booty Chest (Mao Kun)")]
    BootyChestMaoKun,
}

impl Identified for Chest {
    fn id(&self) -> &str {
        match self {
            Chest::AbyssalTrove => "Abyssal Trove",
            Chest::DelveChest => "Delve chest",
            Chest::DelveGemChests => "Delve Gem Chests",
            Chest::VoltaxicSulphite => "Voltaxic Sulphite",
            Chest::DelveInteractablesBehindFracturedWall => "Delve Interactables behind Fractured Wall",
            Chest::DelveCityLightJewelleryChest => "Light Jewellery chest (Primeval Ruins, Abyssal City, Vaal Outpost)",
            Chest::MavenCrucible => "The Maven's Crucible",
            Chest::HeistMapChest => "Map Reward Heist Chests",
            Chest::BreachClaspedHand => "Breach Clasped Hand",
            Chest::IzaroTreasure => "Izaro's Treasure",
            Chest::VaalVessel => "Vaal Vessel (Vaal Side Areas)",
            Chest::UberlabChests => "Uber Labyrinth or Enriched Labyrinth (Izaro's Treasure, Labyrinth Trove, Curious Lockbox)",
            Chest::Darkshrine => "Labyrinth Darkshrines",
            Chest::BootyChestMaoKun => "Booty Chest (Mao Kun)",
            Chest::MercilessChests => "Merciless Labyrinth (Izaro's Treasure, Labyrinth Trove, Curious Lockbox, Hidden Coffer)",
            Chest::HiddenCoffer => "Hidden Coffer",
        }
    }
}

pub fn poedb_page_url(boss: &str) {
    let name = boss.split("(").next().unwrap().trim();
    let name = name.replace(" ", "_");
    let name = name.replace(",", "%2C");
    format!("https://poedb.tw/us/{name}");
}
