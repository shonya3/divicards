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

    // 2. Transitive sources(for maps and acts)
    if source_types.contains(&"Map".to_owned()) {
        poe_data.maps.iter().for_each(|m| {
            let map = Source::Map(m.name.clone());

            let set: HashSet<CardBySource> =
                get_transitive_cards_from_source(&map, records, poe_data)
                    .map(CardBySource::Transitive)
                    .collect();

            if !set.is_empty() {
                hash_map.entry(map).or_default().extend(set);
            }
        });
    };

    if source_types.contains(&"Act".to_owned()) {
        poe_data.acts.iter().for_each(|a| {
            if a.is_town {
                return;
            }

            let act = Source::Act(a.id.clone());

            let set: HashSet<CardBySource> =
                get_transitive_cards_from_source(&act, records, poe_data)
                    .map(CardBySource::Transitive)
                    .collect();

            if !set.is_empty() {
                hash_map.entry(act).or_default().extend(set);
            }
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

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Direct {
    // #[serde(skip_serializing)]
    // pub source: Source,
    pub card: String,
    pub status: VerificationStatus,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Transitive {
    // #[serde(skip_serializing)]
    // pub source: Source,
    pub card: String,
    pub status: VerificationStatus,
    pub transitive: Source,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
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

#[cfg(test)]
mod tests {
    use super::VerificationStatus;
    use crate::{
        dropsource::predefined::PredefinedSource, spreadsheet::record::Confidence, Record, Source,
    };

    #[test]
    fn test_get_direct_cards_from_source() {
        let source = Source::Predefined(PredefinedSource::Delirium);
        let records = vec![
            Record {
                id: 1,
                greynote: Default::default(),
                card: "Card1".to_string(),
                tag_hypothesis: None,
                confidence: Confidence::None,
                remaining_work: Default::default(),
                sources: vec![source.clone()],
                notes: None,
                verify_sources: vec![],
            },
            Record {
                id: 2,
                greynote: Default::default(),
                card: "Card2".to_string(),
                tag_hypothesis: None,
                confidence: Confidence::None,
                remaining_work: Default::default(),
                sources: vec![],
                notes: None,
                verify_sources: vec![source.clone()],
            },
            Record {
                id: 3,
                greynote: Default::default(),
                card: "Card3".to_string(),
                tag_hypothesis: None,
                confidence: Confidence::None,
                remaining_work: Default::default(),
                sources: vec![],
                notes: None,
                verify_sources: vec![],
            },
        ];

        let result: Vec<_> = super::get_direct_cards_from_source(&source, &records).collect();

        assert_eq!(result.len(), 2);
        assert!(result
            .iter()
            .any(|d| d.card == "Card1" && d.status == VerificationStatus::Done));
        assert!(result
            .iter()
            .any(|d| d.card == "Card2" && d.status == VerificationStatus::Verify));
    }

    #[test]
    fn test_get_transitive_cards_from_source() {
        use poe_data::{cards::CardsData, mapbosses::MapBoss, PoeData};
        use std::collections::HashMap;

        let map_source = Source::Map("Some Map".to_string());
        let boss_source = Source::MapBoss("Some Map Boss".to_string());

        let records = vec![Record {
            id: 1,
            greynote: Default::default(),
            card: "Some Card".to_string(),
            tag_hypothesis: None,
            confidence: Confidence::None,
            remaining_work: Default::default(),
            sources: vec![boss_source.clone()],
            notes: None,
            verify_sources: vec![],
        }];

        let poe_data = PoeData {
            acts: vec![],
            cards: CardsData(HashMap::new()),
            maps: vec![],
            mapbosses: vec![MapBoss {
                name: "Some Map Boss".to_string(),
                maps: vec!["Some Map".to_string()],
            }],
        };

        let result: Vec<_> =
            super::get_transitive_cards_from_source(&map_source, &records, &poe_data).collect();

        assert_eq!(result.len(), 1);
        let transitive = &result[0];
        assert_eq!(transitive.card, "Some Card");
        assert_eq!(transitive.status, VerificationStatus::Done);
        assert_eq!(transitive.transitive, boss_source);
    }

    #[test]
    fn test_cards_by_source() {
        use poe_data::{cards::CardsData, mapbosses::MapBoss, PoeData};
        use std::collections::HashMap;

        let map_source = Source::Map("Some Map".to_string());
        let boss_source = Source::MapBoss("Some Map Boss".to_string());

        let records = vec![
            Record {
                id: 1,
                greynote: Default::default(),
                card: "Direct Card".to_string(),
                tag_hypothesis: None,
                confidence: Confidence::None,
                remaining_work: Default::default(),
                sources: vec![map_source.clone()],
                notes: None,
                verify_sources: vec![],
            },
            Record {
                id: 2,
                greynote: Default::default(),
                card: "Transitive Card".to_string(),
                tag_hypothesis: None,
                confidence: Confidence::None,
                remaining_work: Default::default(),
                sources: vec![boss_source.clone()],
                notes: None,
                verify_sources: vec![],
            },
        ];

        let poe_data = PoeData {
            acts: vec![],
            cards: CardsData(HashMap::new()),
            maps: vec![],
            mapbosses: vec![MapBoss {
                name: "Some Map Boss".to_string(),
                maps: vec!["Some Map".to_string()],
            }],
        };

        let result = super::cards_by_source(&map_source, &records, &poe_data);

        assert_eq!(result.len(), 2);
        assert!(result
            .iter()
            .any(|c| c.is_direct() && c.card() == "Direct Card"));
        assert!(result
            .iter()
            .any(|c| c.is_transitive() && c.card() == "Transitive Card"));
    }

    #[test]
    fn test_cards_by_source_types() {
        use poe_data::{cards::CardsData, mapbosses::MapBoss, maps::Map, PoeData};
        use std::collections::HashMap;

        let map_source = Source::Map("Some Map".to_string());
        let boss_source = Source::MapBoss("Some Map Boss".to_string());
        let other_source = Source::Predefined(PredefinedSource::Delirium);

        let records = vec![
            Record {
                id: 1,
                greynote: Default::default(),
                card: "Direct Card".to_string(),
                tag_hypothesis: None,
                confidence: Confidence::None,
                remaining_work: Default::default(),
                sources: vec![map_source.clone()],
                notes: None,
                verify_sources: vec![],
            },
            Record {
                id: 2,
                greynote: Default::default(),
                card: "Transitive Card".to_string(),
                tag_hypothesis: None,
                confidence: Confidence::None,
                remaining_work: Default::default(),
                sources: vec![boss_source.clone()],
                notes: None,
                verify_sources: vec![],
            },
            Record {
                id: 3,
                greynote: Default::default(),
                card: "Other Card".to_string(),
                tag_hypothesis: None,
                confidence: Confidence::None,
                remaining_work: Default::default(),
                sources: vec![other_source],
                notes: None,
                verify_sources: vec![],
            },
        ];

        let poe_data = PoeData {
            acts: vec![],
            cards: CardsData(HashMap::new()),
            maps: vec![Map {
                name: "Some Map".to_string(),
                tier: 1,
                available: true,
                unique: false,
                icon: "".to_string(),
                slug: "".to_string(),
                atlas_cards: vec![],
            }],
            mapbosses: vec![MapBoss {
                name: "Some Map Boss".to_string(),
                maps: vec!["Some Map".to_string()],
            }],
        };

        let result = super::cards_by_source_types(&["Map".to_string()], &records, &poe_data);

        assert_eq!(result.len(), 1);
        let source_and_cards = &result[0];
        assert_eq!(source_and_cards.source, map_source);
        assert_eq!(source_and_cards.cards.len(), 2);
        assert!(source_and_cards
            .cards
            .iter()
            .any(|c| c.is_direct() && c.card() == "Direct Card"));
        assert!(source_and_cards
            .cards
            .iter()
            .any(|c| c.is_transitive() && c.card() == "Transitive Card"));
    }
}
