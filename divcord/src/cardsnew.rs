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

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Card name with verification status and possible transitive source. Being used in context of drop source
pub struct CardBySource {
    pub source: Source,
    pub card: String,
    pub transitive_source: Option<Source>,
    pub column: RichColumnVariant,
}

impl CardBySource {
    pub const fn new(
        source: Source,
        card: String,
        transitive_source: Option<Source>,
        column: RichColumnVariant,
    ) -> Self {
        Self {
            source,
            card,
            transitive_source,
            column,
        }
    }

    pub const fn new_without_transitive(
        source: Source,
        card: String,
        column: RichColumnVariant,
    ) -> Self {
        Self::new(source, card, None, column)
    }
}

pub fn transitive_sources(source: &Source, poe_data: &PoeData) -> Vec<Source> {
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

pub fn cards_by_source_from_transitive_sources(
    direct_source: &Source,
    records: &[Record],
    poe_data: &PoeData,
) -> Vec<CardBySource> {
    transitive_sources(&direct_source, &poe_data)
        .iter()
        .flat_map(|transitive| {
            cards_by_source_directly(&transitive, &records)
                .into_iter()
                .map(|by_transitive| CardBySource {
                    source: direct_source.to_owned(),
                    card: by_transitive.card,
                    transitive_source: Some(by_transitive.source),
                    column: by_transitive.column,
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
                .map(|source| CardBySource {
                    source: source.to_owned(),
                    card: record.card.to_owned(),
                    transitive_source: None,
                    column: RichColumnVariant::Sources,
                })
                .chain(
                    // 2. by verify sources
                    record
                        .verify_sources
                        .iter()
                        .filter(|verify| *verify == direct_source)
                        .map(|source| CardBySource {
                            source: source.to_owned(),
                            card: record.card.to_owned(),
                            transitive_source: None,
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
    direct_cards.extend(cards_by_source_from_transitive_sources(
        &source, &records, &poe_data,
    ));
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
) -> HashMap<Source, Vec<CardBySource>> {
    let mut hash_map: HashMap<Source, Vec<CardBySource>> = HashMap::new();
    let mut visited_transitive_sources: HashSet<Source> = HashSet::new();

    // 1. filter sources by source types and push to entry
    records.iter().for_each(|record| {
        //for each source, get its entry and push to it
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
                entry.push(CardBySource::new_without_transitive(
                    source.clone(),
                    record.card.clone(),
                    RichColumnVariant::Sources,
                ));

                for transitive in transitive_sources(&source, &poe_data) {
                    if !visited_transitive_sources.contains(&transitive) {
                        for by_transitive in cards_by_source_directly(&transitive, &records) {
                            entry.push(CardBySource {
                                source: source.to_owned(),
                                card: by_transitive.card.to_owned(),
                                transitive_source: Some(by_transitive.source.to_owned()),
                                column: by_transitive.column.to_owned(),
                            })
                        }

                        visited_transitive_sources.insert(transitive);
                    }
                }
            })
    });

    // If map area directly drops no cards, but some of it's bosses can
    if source_types.contains(&"Map".to_owned()) {
        poe_data.maps.clone().into_iter().for_each(|map| {
            let source = Source::from(map);
            if !visited_transitive_sources.contains(&source) {
                hash_map.entry(source.clone()).or_default().extend(
                    cards_by_source_from_transitive_sources(&source, &records, &poe_data),
                );
            }
        })
    };

    // If act area directly drops no cards, but some of it's bosses can
    if source_types.contains(&"Act".to_owned()) {
        poe_data.acts.clone().into_iter().for_each(|act_area| {
            let source = Source::from(act_area);
            if !visited_transitive_sources.contains(&source) {
                hash_map.entry(source.clone()).or_default().extend(
                    cards_by_source_from_transitive_sources(&source, &records, &poe_data),
                )
            }
        })
    };

    hash_map
}
