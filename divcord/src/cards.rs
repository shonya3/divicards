use std::collections::{HashMap, HashSet};

use poe_data::{mapbosses::MapBoss, PoeData};
use serde::Serialize;

use crate::{dropsource::Source, table::table_record::SourcefulDivcordTableRecord};

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

#[derive(Debug, Serialize, Clone)]
pub struct SourceAndCards {
    source: Source,
    cards: Vec<CFromSource>,
}

pub fn cards_by_mapboss(
    boss: &str,
    records: &[SourcefulDivcordTableRecord],
    poe_data: &PoeData,
) -> Vec<String> {
    let mut cards: Vec<String> = vec![];
    let Some(_) = poe_data.mapbosses.iter().find(|b| b.name == boss) else {
        return cards;
    };

    for record in records {
        if record.sources.iter().any(|s| s._id() == boss) {
            cards.push(record.card.to_owned())
        }
    }

    cards
}

pub fn cards_by_actboss(boss: &str, records: &[SourcefulDivcordTableRecord]) -> Vec<String> {
    let mut cards: Vec<String> = vec![];

    for record in records {
        if record.sources.iter().any(|s| s._id() == boss) {
            cards.push(record.card.to_owned())
        }
    }

    cards
}

#[derive(Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum CFromSource {
    Card { card: String },
    WithBoss { card: String, boss: String },
}

pub fn find_cards_by_source_types(
    source_types: &[String],
    records: &[SourcefulDivcordTableRecord],
    poe_data: &PoeData,
) -> Vec<SourceAndCards> {
    let mut map: HashMap<&Source, Vec<CFromSource>> = HashMap::new();
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
            entry.push(CFromSource::Card {
                card: record.card.to_owned(),
            });

            match source {
                Source::Map { name } => {
                    if !set.contains(name.as_str()) {
                        set.insert(name);
                        for boss in bosses_in_map(name, poe_data) {
                            for card in cards_by_mapboss(&boss.name, records, poe_data) {
                                entry.push(CFromSource::WithBoss {
                                    card,
                                    boss: boss.name.to_owned(),
                                });
                            }
                        }
                    }
                }
                Source::Act { id } => {
                    let act_area = poe_data.acts.iter().find(|a| a.id == *id).unwrap();
                    if !set.contains(id.as_str()) {
                        set.insert(id);
                        for fight in &act_area.bossfights {
                            for card in cards_by_actboss(&fight.name, records) {
                                entry.push(CFromSource::WithBoss {
                                    card,
                                    boss: fight.name.clone(),
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

// pub fn find_cards_by_source_types(
//     source_types: &[String],
//     records: &[SourcefulDivcordTableRecord],
//     poe_data: &PoeData,
// ) -> Vec<SourceAndCards> {
//     // let mut map: HashMap<String, (&Source, Vec<CFromSource>)> = HashMap::new();
//     let mut map: HashMap<String, Vec<CFromSource>> = HashMap::new();
//     let mut sourcemap: HashMap<String, &Source> = HashMap::new();
//     let mut set: HashSet<&str> = HashSet::new();
//     // let mut i = 0;

//     // let source_types_as_str: Vec<&str> = source_types.iter().map(|t| t.as_str()).collect();

//     for record in records {
//         // let filtered_sources = record
//         //     .sources
//         //     .iter()
//         //     .filter(|s| source_types_as_str.contains(&s._type()))
//         //     .collect::<Vec<_>>();

//         for source in &record.sources {
//             // sourcemap.insert(source._id(), source);
//             // let entry = map.entry(source._id()).or_default();
//             // entry.push(CFromSource::Card {
//             //     card: record.card.to_owned(),
//             // });

//             // let entry = map.entry(source._id()).or_insert((source, vec![]));
//             // entry.1.push(CFromSource::Card {
//             //     card: record.card.to_owned(),
//             // });

//             // if let Source::Map { name } = source {
//             //     if !set.contains(name.as_str()) {
//             //         set.insert(name);
//             //         for boss in bosses_in_map(name, poe_data) {
//             //             for card in cards_by_mapboss(&boss.name, records, poe_data) {
//             //                 entry.push(CFromSource::WithBoss {
//             //                     card,
//             //                     boss: boss.name.to_owned(),
//             //                 });
//             //             }
//             //         }
//             //     }
//             // };

//             // if let Source::Act { id } = source {
//             //     let act_area = poe_data.acts.iter().find(|a| a.id == *id).unwrap();
//             //     if !set.contains(id.as_str()) {
//             //         set.insert(id);
//             //         for fight in &act_area.bossfights {
//             //             for card in cards_by_actboss(&fight.name, records) {
//             //                 entry.push(CFromSource::WithBoss {
//             //                     card,
//             //                     boss: fight.name.clone(),
//             //                 })
//             //             }
//             //         }
//             //     }
//             // };
//         }
//     }

//     // dbg!(i);

//     map.into_iter()
//         .map(|(id, cards)| {
//             let s = sourcemap.get(&id).unwrap();
//             let s = <Source>::clone(s);
//             SourceAndCards { source: s, cards }
//         })
//         .collect()

//     // map.into_iter()
//     //     .map(|(_, (source, cards))| {
//     //         let s = sourcemap.get(&source._id()).unwrap();
//     //         let s = <Source>::clone(s);
//     //         SourceAndCards { source: s, cards }
//     //     })
//     //     .collect()
// }
