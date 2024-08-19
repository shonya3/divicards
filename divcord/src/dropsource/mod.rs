pub mod area;
pub mod id;
pub mod monster;
mod other;

use self::id::Identified;
pub use self::{area::Area, monster::UniqueMonster};
pub use other::{Chest, Strongbox, Vendor};
use poe_data::act::ActAreaId;
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
            min_level: Option<u32>,
            max_level: Option<u32>,
        }

        let JSSource {
            id,
            _type,
            kind,
            max_level,
            min_level,
        } = JSSource::deserialize(deserializer)?;
        match kind {
            SourceKind::EmptySource => match _type.parse::<Source>() {
                Ok(source) => match source {
                    Source::GlobalDrop { .. } => Ok(Source::GlobalDrop {
                        min_level,
                        max_level,
                    }),
                    _ => Ok(source),
                },
                Err(_) => Err(de::Error::custom(format!(
                    "Could not deserialize Source. {_type}"
                ))),
            },
            SourceKind::SourceWithMember => {
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

impl FromStr for Source {
    type Err = strum::ParseError;

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
            "Global Drop" => Ok(Source::GlobalDrop {
                min_level: None,
                max_level: None,
            }),
            _ => UniqueMonster::from_str(s)
                .map(Self::UniqueMonster)
                .or_else(|_| Area::from_str(s).map(Self::Area))
                .or_else(|_| Vendor::from_str(s).map(Self::Vendor))
                .or_else(|_| Strongbox::from_str(s).map(Self::Strongbox))
                .or_else(|_| Chest::from_str(s).map(Self::Chest))
                .map_err(|_| strum::ParseError::VariantNotFound),
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
            Source::GlobalDrop { .. } => "Global Drop",
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
            Source::GlobalDrop { .. } => "Global Drop",
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
            r#"export type SourceWithMember = {{ type: SourceType; id: string; kind: SourceWithMemberKind; min_level?: number; max_level?: number; idSlug: string; typeSlug: string }};
export type EmptySourceKind = 'empty-source';
export type SourceWithMemberKind = 'source-with-member';
export type Kind = EmptySourceKind | SourceWithMemberKind;
export type EmptySource = {{ type: SourceType; id: string; kind: EmptySourceKind; min_level?: number; max_level?: number; idSlug: string; typeSlug: string }};
export type Source = SourceWithMember | EmptySource;
export const SOURCE_TYPE_VARIANTS = {_types} as const;

export type SourceType = (typeof SOURCE_TYPE_VARIANTS)[number];
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
        let mut source = serializer.serialize_struct("Source", 7)?;
        let _type = self._type();
        let id = self.id();

        source.serialize_field("type", _type)?;
        source.serialize_field("id", &id)?;
        source.serialize_field("idSlug", &slug::slugify(id))?;
        source.serialize_field("typeSlug", &slug::slugify(_type))?;

        if _type == id {
            source.serialize_field("kind", "empty-source")?;
        } else {
            source.serialize_field("kind", "source-with-member")?;
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

pub fn poedb_page_url(boss: &str) {
    let name = boss.split('(').next().unwrap().trim();
    let name = name.replace(' ', "_");
    let name = name.replace(',', "%2C");
    format!("https://poedb.tw/us/{name}");
}

#[cfg(test)]
mod tests {

    use crate::Source;

    #[test]
    pub fn deserialize_global_drop() {
        let json = r#"{"type":"Global Drop","id":"Global Drop","kind":"empty-source","max_level":68,"min_level":68}"#;
        let source = serde_json::from_str::<Source>(json).unwrap();

        let Source::GlobalDrop {
            min_level,
            max_level,
        } = source
        else {
            panic!("Source is not type of Global Drop")
        };

        assert_eq!(min_level, Some(68));
        assert_eq!(max_level, Some(68));
    }
}
