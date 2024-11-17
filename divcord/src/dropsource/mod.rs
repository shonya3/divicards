pub mod area;
pub mod id;
pub mod monster;
mod other;

use self::id::Identified;
pub use self::{area::Area, monster::UniqueMonster};
pub use other::{Chest, Strongbox, Vendor};
use poe_data::act::ActAreaId;
#[allow(unused_imports)]
use poe_data::PoeData;
use serde::{de, ser::SerializeStruct, Deserialize, Serialize};
use std::str::FromStr;
use strum::IntoEnumIterator;

#[derive(Debug, Clone, PartialEq, Eq, Hash, strum_macros::EnumIter)]
pub enum Source {
    Act(ActAreaId),
    Map(String),
    ActBoss(String),
    MapBoss(String),

    UniqueMonster(UniqueMonster),
    Area(Area),

    Chest(Chest),
    Strongbox(Strongbox),
    Vendor(Vendor),

    KiracMissions,
    MaelstromOfChaosWithBarrelSextant,
    Delirium,
    DeliriumCurrencyRewards,
    Disabled,
}

/// `s.parse::<Source>` validates only predefined sources.
/// It does not account for [PoeData] and cannot be used independently.
/// Use it solely as the initial step in drop source parsing.
#[derive(Debug)]
pub struct UnknownPredefinedSource(pub String);

impl FromStr for Source {
    type Err = UnknownPredefinedSource;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Kirac Missions" => Ok(Source::KiracMissions),
            "Disabled" => Ok(Source::Disabled),
            "Delirium Currency Rewards" | "Delirium Currency reward" => {
                Ok(Source::DeliriumCurrencyRewards)
            }
            "Maelström of Chaos with Barrel Sextant" => {
                Ok(Source::MaelstromOfChaosWithBarrelSextant)
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
            kind: Kind,
        }

        let JSSource { id, _type, kind } = JSSource::deserialize(deserializer)?;
        match kind {
            Kind::Category => match _type.parse::<Source>() {
                Ok(source) => Ok(source),
                Err(_) => Err(de::Error::custom(format!(
                    "Could not deserialize Source. {_type}"
                ))),
            },
            Kind::Source => {
                let Some(id) = id else {
                    return Err(de::Error::custom("No id field"));
                };
                match id.parse::<Source>() {
                    Ok(source) => Ok(source),
                    Err(_) => match _type.as_str() {
                        "Map" => Ok(Source::Map(id)),
                        "Map Boss" => Ok(Source::MapBoss(id)),
                        "Act" => Ok(Source::Act(ActAreaId::new(id))),
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

impl std::fmt::Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id())
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
            Source::MaelstromOfChaosWithBarrelSextant => "Maelström of Chaos with Barrel Sextant",
            Source::KiracMissions => "Kirac Missions",
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
            Source::MaelstromOfChaosWithBarrelSextant => "Maelström of Chaos with Barrel Sextant",
            Source::KiracMissions => "Kirac Missions",
        }
    }

    pub fn slug(&self) -> String {
        slug::slugify(self.id())
    }

    pub fn type_slug(&self) -> String {
        slug::slugify(self._type())
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
            r#"export type Source = {{ type: SourceType; typeSlug: string; idSlug: string; id: string; kind: Kind }};
export type Kind = 'category' | 'source';
export const SOURCE_TYPE_VARIANTS = {_types} as const;

export type SourceType = (typeof SOURCE_TYPE_VARIANTS)[number];
    "#,
        );

        s
    }
}

#[derive(Deserialize, Serialize)]
pub enum Kind {
    #[serde(rename = "category")]
    Category,
    #[serde(rename = "source")]
    Source,
}

impl Serialize for Source {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut source = serializer.serialize_struct("Source", 7)?;
        let _type = self._type();
        let id = self.id();

        source.serialize_field("type", _type)?;
        source.serialize_field("id", &id)?;
        source.serialize_field("idSlug", &slug::slugify(id))?;
        source.serialize_field("typeSlug", &slug::slugify(_type))?;

        if _type == id {
            source.serialize_field("kind", "category")?;
        } else {
            source.serialize_field("kind", "source")?;
        }

        source.end()
    }
}
