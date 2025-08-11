use crate::{parse::SourcesKind, Record, Source};
use poe_data::{act::Bossfight, mapbosses::MapBoss, maps::Map, PoeData};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn cards_by_source<'a>(
    source: &'a Source,
    records: &'a [Record],
    poe_data: &'a PoeData,
) -> Vec<CardBySource> {
    let direct_cards = get_direct_cards_from_source(source, records).map(CardBySource::Direct);
    let transitive_cards =
        get_transitive_cards_from_source(source, records, poe_data).map(CardBySource::Transitive);
    direct_cards.chain(transitive_cards).collect()
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

            let cards = get_transitive_cards_from_source(&source, records, poe_data)
                .map(CardBySource::Transitive)
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
            let cards = get_transitive_cards_from_source(&source, records, poe_data)
                .map(CardBySource::Transitive)
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
pub struct Transitive {
    #[serde(skip_serializing)]
    pub source: Source,
    pub card: String,
    pub column: SourcesKind,
    pub transitive: Source,
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CardBySource {
    Direct(Direct),
    #[serde(rename = "transitive")]
    Transitive(Transitive),
}

impl CardBySource {
    pub fn is_transitive(&self) -> bool {
        match self {
            CardBySource::Direct(_) => false,
            CardBySource::Transitive(_) => true,
        }
    }

    pub fn is_direct(&self) -> bool {
        match self {
            CardBySource::Direct(_) => true,
            CardBySource::Transitive(_) => false,
        }
    }

    pub fn column(&self) -> &SourcesKind {
        match self {
            CardBySource::Direct(d) => &d.column,
            CardBySource::Transitive(c) => &c.column,
        }
    }

    pub fn card(&self) -> &String {
        match self {
            CardBySource::Direct(d) => &d.card,
            CardBySource::Transitive(c) => &c.card,
        }
    }

    pub fn source(&self) -> &Source {
        match self {
            CardBySource::Direct(d) => &d.source,
            CardBySource::Transitive(c) => &c.source,
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

pub fn transitive_sources(source: &Source, poe_data: &PoeData) -> Vec<Source> {
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

pub fn get_transitive_cards_from_source<'a>(
    direct_source: &'a Source,
    records: &'a [Record],
    poe_data: &'a PoeData,
) -> impl Iterator<Item = Transitive> + 'a {
    transitive_sources(direct_source, poe_data)
        .into_iter()
        .flat_map(move |transit| {
            get_direct_cards_from_source(&transit, records)
                .collect::<Vec<_>>()
                .into_iter()
                .map(move |by_transit| Transitive {
                    source: direct_source.to_owned(),
                    card: by_transit.card,
                    column: by_transit.column,
                    transitive: by_transit.source,
                })
        })
}

pub fn get_direct_cards_from_source<'a>(
    direct_source: &'a Source,
    records: &'a [Record],
) -> impl Iterator<Item = Direct> + 'a {
    records.iter().flat_map(move |record| {
        let card = &record.card;
        record
            .sources
            .iter()
            .filter(move |&s| s == direct_source)
            .map(move |s| Direct {
                source: s.clone(),
                card: card.clone(),
                column: SourcesKind::Source,
            })
            .chain(
                record
                    .verify_sources
                    .iter()
                    .filter(move |&s| s == direct_source)
                    .map(move |s| Direct {
                        source: s.clone(),
                        card: card.clone(),
                        column: SourcesKind::Verify,
                    }),
            )
    })
}

impl From<Transitive> for CardBySource {
    fn from(value: Transitive) -> Self {
        Self::Transitive(value)
    }
}

impl From<Direct> for CardBySource {
    fn from(value: Direct) -> Self {
        Self::Direct(value)
    }
}
