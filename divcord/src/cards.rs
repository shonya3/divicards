use crate::{dropsource::predefined::PredefinedSource, Record, Source};
use poe_data::{act::Bossfight, mapbosses::MapBoss, maps::Map, PoeData};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub enum VerificationStatus {
    #[serde(rename = "done")]
    Done,
    #[serde(rename = "verify")]
    Verify,
}

pub fn cards_by_source<'a>(
    source: &'a Source,
    records: &'a [Record],
    poe_data: &'a PoeData,
) -> Vec<CardBySource> {
    let direct_cards = get_direct_cards_from_source(source, records)
        .collect::<HashSet<Direct>>()
        .into_iter()
        .map(CardBySource::Direct);
    let transitive_cards = get_transitive_cards_from_source(source, records, poe_data)
        .collect::<HashSet<Transitive>>()
        .into_iter()
        .map(CardBySource::Transitive);
    direct_cards.chain(transitive_cards).collect()
}

pub fn cards_by_source_types(
    source_types: &[String],
    records: &[Record],
    poe_data: &PoeData,
) -> Vec<SourceAndCards> {
    let mut hash_map: HashMap<Source, HashSet<CardBySource>> = HashMap::new();

    // 1. Collect all direct drops
    let mut process_source = |record: &Record, source: &Source, status: VerificationStatus| {
        if !source_types.iter().any(|s| source._type() == *s) {
            return;
        }

        let entry = hash_map.entry(source.clone()).or_default();
        entry.insert(CardBySource::Direct(Direct {
            // source: source.clone(),
            card: record.card.clone(),
            status,
        }));
    };
    records.iter().for_each(|r| {
        r.sources
            .iter()
            .for_each(|s| process_source(r, s, VerificationStatus::Done));
        r.verify_sources
            .iter()
            .for_each(|v| process_source(r, v, VerificationStatus::Verify));
    });

    // 2. Add transitive drops for existing sources
    hash_map.iter_mut().for_each(|(source, cards)| {
        get_transitive_cards_from_source(source, records, poe_data)
            .map(CardBySource::Transitive)
            .for_each(|c| {
                cards.insert(c);
            });
    });

    // 3. Add sources that only have transitive drops
    if source_types.contains(&"Map".to_owned()) {
        poe_data.maps.iter().for_each(|m| {
            let map = Source::Map(m.name.clone());

            if hash_map.contains_key(&map) {
                return;
            }

            let entry = hash_map.entry(map.clone()).or_default();

            get_transitive_cards_from_source(&map, records, poe_data)
                .map(CardBySource::Transitive)
                .for_each(|c| {
                    entry.insert(c);
                });
        });
    };

    if source_types.contains(&"Act".to_owned()) {
        poe_data.acts.iter().for_each(|a| {
            if a.is_town {
                return;
            }

            let act = Source::Act(a.id.clone());

            if hash_map.contains_key(&act) {
                return;
            }

            let entry = hash_map.entry(act.clone()).or_default();

            get_transitive_cards_from_source(&act, records, poe_data)
                .map(CardBySource::Transitive)
                .for_each(|c| {
                    entry.insert(c);
                });
        });
    }

    hash_map
        .into_iter()
        .map(|(source, cards)| SourceAndCards {
            source,
            cards: Vec::from_iter(cards),
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Direct {
    // #[serde(skip_serializing)]
    // pub source: Source,
    pub card: String,
    pub status: VerificationStatus,
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Transitive {
    // #[serde(skip_serializing)]
    // pub source: Source,
    pub card: String,
    pub status: VerificationStatus,
    pub transitive: Source,
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CardBySource {
    Direct(Direct),
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

    pub fn status(&self) -> VerificationStatus {
        match self {
            CardBySource::Direct(d) => d.status,
            CardBySource::Transitive(c) => c.status,
        }
    }

    pub fn card(&self) -> &String {
        match self {
            CardBySource::Direct(d) => &d.card,
            CardBySource::Transitive(c) => &c.card,
        }
    }

    // pub fn source(&self) -> &Source {
    //     match self {
    //         CardBySource::Direct(d) => &d.source,
    //         CardBySource::Transitive(c) => &c.source,
    //     }
    // }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SourceAndCards {
    pub source: Source,
    pub cards: Vec<CardBySource>,
}

impl From<MapBoss> for Source {
    fn from(value: MapBoss) -> Self {
        // For cases like Chimera, which is predefined source, but also can be found as mapboss.
        // It should be Predefined.
        if let Ok(source) = value.name.parse::<PredefinedSource>() {
            return Source::Predefined(source);
        }
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
                    // source: direct_source.to_owned(),
                    card: by_transit.card,
                    status: by_transit.status,
                    transitive: transit.clone(),
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
            .map(move |_s| Direct {
                // source: s.clone(),
                card: card.clone(),
                status: VerificationStatus::Done,
            })
            .chain(
                record
                    .verify_sources
                    .iter()
                    .filter(move |&s| s == direct_source)
                    .map(move |_s| Direct {
                        // source: s.clone(),
                        card: card.clone(),
                        status: VerificationStatus::Verify,
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
