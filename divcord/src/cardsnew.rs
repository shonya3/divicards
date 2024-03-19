use std::collections::HashSet;

use poe_data::{act::Bossfight, mapbosses::MapBoss, PoeData};

use crate::{dropsource::id::Identified, parse::RichColumnVariant, Record, Source};

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

#[derive(Debug, PartialEq, Eq, Hash)]
/// Card name with verification status and possible transitive source. Being used in context of drop source
pub struct CardBySource {
    pub source: Source,
    pub card: String,
    pub transitive_source: Option<Source>,
    pub column: RichColumnVariant,
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

pub fn cards_by_source_directly(direct_source: &Source, records: &[Record]) -> Vec<CardBySource> {
    records
        .iter()
        .flat_map(|record| {
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
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn cards_by_source(
    source: &Source,
    records: &[Record],
    poe_data: &PoeData,
) -> HashSet<CardBySource> {
    cards_by_source_directly(&source, &records)
        .into_iter()
        // add cards by transitive sources(bosses for acts and maps, maybe something else)
        .chain(
            transitive_sources(&source, &poe_data)
                .iter()
                .flat_map(|transitive| {
                    cards_by_source_directly(transitive, records)
                        .into_iter()
                        .map(|by_transitive_source| CardBySource {
                            source: source.to_owned(),
                            transitive_source: Some(by_transitive_source.source),
                            card: by_transitive_source.card,
                            column: by_transitive_source.column,
                        })
                })
                .collect::<Vec<CardBySource>>(),
        )
        .collect()
}
