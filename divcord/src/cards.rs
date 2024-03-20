use std::collections::{HashMap, HashSet};

use crate::{
    dropsource::id::Identified, parse::RichColumnVariant, spreadsheet::record::Record, Source,
};
use poe_data::{mapbosses::MapBoss, PoeData};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct SourceAndCards {
    source: Source,
    cards: Vec<CardFromSource>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum CardFromSource {
    Direct {
        card: String,
        column: RichColumnVariant,
    },
    Transitive {
        card: String,
        transitive: Source,
        column: RichColumnVariant,
    },
}

pub enum Directness {
    Direct,
    Transitive,
}

impl CardFromSource {
    pub fn direct(card: String, column: RichColumnVariant) -> Self {
        Self::Direct { card, column }
    }

    pub fn transitive(card: String, column: RichColumnVariant, transitive: Source) -> Self {
        Self::Transitive {
            card,
            transitive,
            column,
        }
    }
}

pub fn bosses_in_map(map: &str, poe_data: &PoeData) -> Vec<MapBoss> {
    let Some(map) = poe_data.maps.iter().find(|m| m.name == map) else {
        return vec![];
    };

    poe_data
        .mapbosses
        .iter()
        .filter(|boss| boss.maps.contains(&map.name))
        .map(|b| b.to_owned())
        .collect::<Vec<_>>()
}

pub fn cards_by_mapboss(boss: &str, records: &[Record], poe_data: &PoeData) -> Vec<String> {
    let mut cards: Vec<String> = vec![];
    let Some(_) = poe_data.mapbosses.iter().find(|b| b.name == boss) else {
        return cards;
    };

    for record in records {
        if record.sources.iter().any(|s| s.id() == boss) {
            cards.push(record.card.to_owned())
        }
    }

    cards
}

pub fn cards_by_actboss(boss: &str, records: &[Record]) -> Vec<String> {
    let mut cards: Vec<String> = vec![];

    for record in records {
        if record.sources.iter().any(|s| s.id() == boss) {
            cards.push(record.card.to_owned())
        }
    }

    cards
}

pub fn find_cards_by_source_types(
    source_types: &[String],
    records: &[Record],
    poe_data: &PoeData,
) -> Vec<SourceAndCards> {
    let mut map: HashMap<&Source, Vec<CardFromSource>> = HashMap::new();
    let mut set: HashSet<&str> = HashSet::new();

    let source_types_as_str: Vec<&str> = source_types.iter().map(|t| t.as_str()).collect();

    for record in records {
        let filtered_sources = record
            .sources
            .iter()
            .filter(|s| source_types_as_str.contains(&s._type()))
            .collect::<Vec<_>>();

        for source in filtered_sources {
            let entry = map.entry(source).or_default();
            entry.push(CardFromSource::Direct {
                card: record.card.to_owned(),
                column: RichColumnVariant::Sources,
            });

            match source {
                Source::Map(name) => {
                    if !set.contains(name.as_str()) {
                        set.insert(name);
                        for boss in bosses_in_map(name, poe_data) {
                            for card in cards_by_mapboss(&boss.name, records, poe_data) {
                                entry.push(CardFromSource::Transitive {
                                    column: RichColumnVariant::Sources,
                                    card,
                                    transitive: Source::MapBoss(boss.name.clone()),
                                });
                            }
                        }
                    }
                }
                Source::Act(id) => {
                    let act_area = poe_data.acts.iter().find(|a| a.id == *id).unwrap();
                    if !set.contains(id.as_str()) {
                        set.insert(id);
                        for fight in &act_area.bossfights {
                            for card in cards_by_actboss(&fight.name, records) {
                                entry.push(CardFromSource::Transitive {
                                    card,
                                    transitive: Source::ActBoss(fight.name.to_owned()),
                                    column: RichColumnVariant::Sources,
                                })
                            }
                        }
                    }
                }

                _ => {}
            }
        }
    }

    map.into_iter()
        .map(|(source, cards)| SourceAndCards {
            source: source.to_owned(),
            cards,
        })
        .collect()
}
