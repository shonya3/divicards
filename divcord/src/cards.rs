use crate::{parse::SourcesKind, Record, Source};
use poe_data::{act::Bossfight, mapbosses::MapBoss, maps::Map, PoeData};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn cards_by_source(
    source: &Source,
    records: &[Record],
    poe_data: &PoeData,
) -> Vec<CardBySource> {
    let mut direct_cards = cards_by_source_directly(source, records)
        .into_iter()
        .map(CardBySource::Direct)
        .collect::<Vec<_>>();
    let from_children = cards_from_child_sources(source, records, poe_data)
        .into_iter()
        .map(CardBySource::FromChild);
    direct_cards.extend(from_children);
    direct_cards
}

pub fn cards_by_source_types(
    source_types: &[String],
    records: &[Record],
    poe_data: &PoeData,
) -> Vec<SourceAndCards> {
    let mut hash_map: HashMap<Source, Vec<CardBySource>> = HashMap::new();

    records.iter().for_each(|record| {
        record
            .sources
            .iter()
            .filter(|source| source_types.iter().any(|s| source._type() == *s))
            .chain(
                record
                    .verify_sources
                    .iter()
                    .filter(|verify| source_types.iter().any(|s| verify._type() == *s)),
            )
            .for_each(|source| {
                let entry = hash_map.entry(source.clone()).or_default();
                entry.push(CardBySource::Direct(Direct {
                    source: source.clone(),
                    card: record.card.clone(),
                    column: SourcesKind::Source,
                }));
            })
    });

    if source_types.contains(&"Map".to_owned()) {
        poe_data.maps.clone().into_iter().for_each(|map| {
            let source = Source::from(map);

            let cards = cards_from_child_sources(&source, records, poe_data)
                .into_iter()
                .map(CardBySource::FromChild)
                .collect::<Vec<_>>();
            if !cards.is_empty() {
                hash_map.entry(source.clone()).or_default().extend(cards);
            }
        })
    };

    if source_types.contains(&"Act".to_owned()) {
        poe_data.acts.clone().into_iter().for_each(|act_area| {
            if act_area.is_town {
                return;
            }

            let source = Source::from(act_area);
            let cards = cards_from_child_sources(&source, records, poe_data)
                .into_iter()
                .map(CardBySource::FromChild)
                .collect::<Vec<_>>();
            if !cards.is_empty() {
                hash_map.entry(source.clone()).or_default().extend(cards)
            }
        })
    };

    hash_map
        .into_iter()
        .map(|(source, cards)| SourceAndCards { source, cards })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Direct {
    #[serde(skip_serializing)]
    pub source: Source,
    pub card: String,
    pub column: SourcesKind,
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FromChild {
    #[serde(skip_serializing)]
    pub source: Source,
    pub card: String,
    pub column: SourcesKind,
    pub child: Source,
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CardBySource {
    Direct(Direct),
    #[serde(rename = "child")]
    FromChild(FromChild),
}

impl CardBySource {
    pub fn is_child(&self) -> bool {
        match self {
            CardBySource::Direct(_) => false,
            CardBySource::FromChild(_) => true,
        }
    }

    pub fn is_direct(&self) -> bool {
        match self {
            CardBySource::Direct(_) => true,
            CardBySource::FromChild(_) => false,
        }
    }

    pub fn column(&self) -> &SourcesKind {
        match self {
            CardBySource::Direct(d) => &d.column,
            CardBySource::FromChild(c) => &c.column,
        }
    }

    pub fn card(&self) -> &String {
        match self {
            CardBySource::Direct(d) => &d.card,
            CardBySource::FromChild(c) => &c.card,
        }
    }

    pub fn source(&self) -> &Source {
        match self {
            CardBySource::Direct(d) => &d.source,
            CardBySource::FromChild(c) => &c.source,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SourceAndCards {
    pub source: Source,
    pub cards: Vec<CardBySource>,
}

impl From<MapBoss> for Source {
    fn from(value: MapBoss) -> Self {
        Source::MapBoss(value.name)
    }
}

impl From<Bossfight> for Source {
    fn from(value: Bossfight) -> Self {
        Source::ActBoss(value.name)
    }
}

impl From<poe_data::maps::Map> for Source {
    fn from(value: Map) -> Self {
        Source::Map(value.name)
    }
}

impl From<poe_data::act::ActArea> for Source {
    fn from(value: poe_data::act::ActArea) -> Self {
        Source::Act(value.id)
    }
}

pub fn child_sources(source: &Source, poe_data: &PoeData) -> Vec<Source> {
    match source {
        Source::Act(act) => poe_data
            .act_area_id(act)
            .map(|act_area| {
                act_area
                    .bossfights
                    .iter()
                    .map(|b| Source::from(b.to_owned()))
                    .collect()
            })
            .unwrap_or_default(),
        Source::Map(map) => poe_data
            .bosses_of_map(map)
            .into_iter()
            .map(|b| Source::from(b.to_owned()))
            .collect(),
        _ => vec![],
    }
}

pub fn cards_from_child_sources(
    direct_source: &Source,
    records: &[Record],
    poe_data: &PoeData,
) -> Vec<FromChild> {
    child_sources(direct_source, poe_data)
        .iter()
        .flat_map(|child| {
            cards_by_source_directly(child, records)
                .into_iter()
                .map(|by_child| FromChild {
                    source: direct_source.to_owned(),
                    card: by_child.card,
                    column: by_child.column,
                    child: by_child.source,
                })
        })
        .collect()
}

pub fn cards_by_source_directly(direct_source: &Source, records: &[Record]) -> Vec<Direct> {
    records
        .iter()
        .flat_map(|record| {
            // 1. by sources
            record
                .sources
                .iter()
                .filter(|source| *source == direct_source)
                .map(|source| Direct {
                    source: source.to_owned(),
                    card: record.card.to_owned(),
                    column: SourcesKind::Source,
                })
                .chain(
                    // 2. by verify sources
                    record
                        .verify_sources
                        .iter()
                        .filter(|verify| *verify == direct_source)
                        .map(|source| Direct {
                            source: source.to_owned(),
                            card: record.card.to_owned(),
                            column: SourcesKind::Verify,
                        }),
                )
                .collect::<Vec<Direct>>()
        })
        .collect()
}

impl From<FromChild> for CardBySource {
    fn from(value: FromChild) -> Self {
        Self::FromChild(value)
    }
}

impl From<Direct> for CardBySource {
    fn from(value: Direct) -> Self {
        Self::Direct(value)
    }
}
