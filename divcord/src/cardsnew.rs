use crate::{parse::RichColumnVariant, Record, Source};
use poe_data::{act::Bossfight, mapbosses::MapBoss, maps::Map, PoeData};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SourceAndCards {
    pub source: Source,
    pub cards: Vec<CardBySource>,
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CardBySource {
    Direct {
        source: Source,
        card: String,
        column: RichColumnVariant,
    },
    #[serde(rename = "child")]
    FromChildSource {
        source: Source,
        card: String,
        column: RichColumnVariant,
        child: Source,
    },
}

impl CardBySource {
    pub fn source(&self) -> &Source {
        match self {
            CardBySource::Direct { source, .. } => &source,
            CardBySource::FromChildSource { source, .. } => &source,
        }
    }

    pub fn card(&self) -> &String {
        match self {
            CardBySource::Direct { card, .. } => &card,
            CardBySource::FromChildSource { card, .. } => &card,
        }
    }

    pub fn column(&self) -> &RichColumnVariant {
        match self {
            CardBySource::Direct { column, .. } => &column,
            CardBySource::FromChildSource { column, .. } => &column,
        }
    }

    pub fn is_direct(&self) -> bool {
        if let Self::Direct { .. } = self {
            return true;
        } else {
            return false;
        }
    }

    pub fn is_child(&self) -> bool {
        if let Self::FromChildSource { .. } = self {
            return true;
        } else {
            return false;
        }
    }
}

pub fn child_sources(source: &Source, poe_data: &PoeData) -> Vec<Source> {
    match source {
        Source::Act(act) => poe_data
            .act_area(act)
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
) -> Vec<CardBySource> {
    child_sources(&direct_source, &poe_data)
        .iter()
        .flat_map(|child| {
            cards_by_source_directly(&child, &records)
                .into_iter()
                .map(|child| CardBySource::FromChildSource {
                    source: direct_source.to_owned(),
                    card: child.card().to_owned(),
                    child: child.source().to_owned(),
                    column: child.column().to_owned(),
                })
        })
        .collect()
}

pub fn cards_by_source_directly(direct_source: &Source, records: &[Record]) -> Vec<CardBySource> {
    records
        .iter()
        .flat_map(|record| {
            // 1. by sources
            record
                .sources
                .iter()
                .filter(|source| *source == direct_source)
                .map(|source| CardBySource::Direct {
                    source: source.to_owned(),
                    card: record.card.to_owned(),
                    column: RichColumnVariant::Sources,
                })
                .chain(
                    // 2. by verify sources
                    record
                        .verify_sources
                        .iter()
                        .filter(|verify| *verify == direct_source)
                        .map(|source| CardBySource::Direct {
                            source: source.to_owned(),
                            card: record.card.to_owned(),
                            column: RichColumnVariant::Verify,
                        }),
                )
                .collect::<Vec<CardBySource>>()
        })
        .collect()
}

pub fn cards_by_source(
    source: &Source,
    records: &[Record],
    poe_data: &PoeData,
) -> Vec<CardBySource> {
    let mut direct_cards = cards_by_source_directly(&source, &records);
    direct_cards.extend(cards_from_child_sources(&source, &records, &poe_data));
    direct_cards
        .into_iter()
        .collect::<HashSet<CardBySource>>()
        .into_iter()
        .collect::<Vec<CardBySource>>()
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
                entry.push(CardBySource::Direct {
                    source: source.clone(),
                    card: record.card.clone(),
                    column: RichColumnVariant::Sources,
                });
            })
    });

    if source_types.contains(&"Map".to_owned()) {
        poe_data.maps.clone().into_iter().for_each(|map| {
            let source = Source::from(map);

            let cards = cards_from_child_sources(&source, &records, &poe_data);
            if cards.is_empty() {
                return;
            }

            hash_map.entry(source.clone()).or_default().extend(cards);
        })
    };

    if source_types.contains(&"Act".to_owned()) {
        poe_data.acts.clone().into_iter().for_each(|act_area| {
            if act_area.is_town {
                return;
            }

            let source = Source::from(act_area);
            let cards = cards_from_child_sources(&source, &records, &poe_data);
            if cards.is_empty() {
                return;
            }

            hash_map.entry(source.clone()).or_default().extend(cards)
        })
    };

    hash_map
        .into_iter()
        .map(|(source, cards)| SourceAndCards { source, cards })
        .collect()
}
