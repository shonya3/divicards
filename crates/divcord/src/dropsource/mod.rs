//! Big fat enum with all drop sources.

use id::Identified;
use poe_data::act::ActAreaId;
#[allow(unused_imports)]
use poe_data::PoeData;
use predefined::PredefinedSource;
use serde::{de, ser::SerializeStruct, Deserialize, Serialize};
use strum::IntoEnumIterator;

pub mod id;
pub mod predefined;

/// First group of variants of 4 (Act, Map, ActBoss, MapBoss)
/// is being resolved after s.parse::<PredefinedSource> fails.
/// Data goes from scrapped [PoeData]. And there are some color and font style rules
/// for differentiating acts (acts are written with italic font-style in spreadsheet).
///
/// [PredefinedSource] variant holds hardcoded dropsources
/// and gets parsed with simple [PredefinedSource::from_str] (s.parse::<PredefinedSource>)
#[derive(Debug, Clone, PartialEq, Eq, Hash, strum_macros::EnumIter)]
pub enum Source {
    // These 4 are being resolved after s.parse::<PredefinedSource> fails.
    Act(ActAreaId),
    Map(String),
    ActBoss(String),
    MapBoss(String),

    Predefined(PredefinedSource),
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
            Kind::Category => match _type.parse::<PredefinedSource>() {
                Ok(source) => Ok(Source::Predefined(source)),
                Err(_) => Err(de::Error::custom(format!(
                    "Could not deserialize Source. {_type}"
                ))),
            },
            Kind::Source => {
                let Some(id) = id else {
                    return Err(de::Error::custom("No id field"));
                };
                match id.parse::<PredefinedSource>() {
                    Ok(source) => Ok(Source::Predefined(source)),
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
            Source::Predefined(predefined_source) => predefined_source.id(),
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
            Source::Predefined(predefined_source) => predefined_source._type(),
        }
    }

    pub fn disabled() -> Source {
        Source::Predefined(PredefinedSource::Disabled)
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
            Source::Predefined(_) => vec.extend(PredefinedSource::types()),
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
