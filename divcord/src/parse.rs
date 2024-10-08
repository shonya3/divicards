use crate::dropsource::{Area, Source, UniqueMonster, Vendor};
use crate::spreadsheet::rich::HexColor;
use crate::{
    error::Error,
    spreadsheet::{
        record::{Confidence, Dumb, GreyNote, Record},
        rich::DropsFrom,
        Spreadsheet,
    },
};
use divi::IsCard;
use poe_data::PoeData;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub fn records_iter<'a>(
    spreadsheet: &'a Spreadsheet,
    poe_data: &'a PoeData,
) -> impl Iterator<Item = Result<Record, Error>> + 'a {
    spreadsheet
        .dumb_records()
        .map(|dumb| Ok(Record::from_dumb(dumb?, poe_data)?))
}

pub fn records(spreadsheet: &Spreadsheet, poe_data: &PoeData) -> Result<Vec<Record>, Error> {
    records_iter(spreadsheet, poe_data).collect()
}

impl Record {
    pub fn from_dumb(dumb: Dumb, poe_data: &PoeData) -> Result<Self, ParseSourceError> {
        Ok(Record {
            sources: parse_record_dropsources(&dumb, poe_data)?,
            verify_sources: parse_dropses_from(&dumb, poe_data, RichColumnVariant::Verify)?,
            id: dumb.id,
            greynote: dumb.greynote,
            card: dumb.card,
            tag_hypothesis: dumb.tag_hypothesis,
            confidence: dumb.confidence,
            remaining_work: dumb.remaining_work,
            notes: dumb.notes,
        })
    }
}

#[derive(Debug)]
pub enum ParseSourceError {
    UnknownVariant {
        card: String,
        record_id: usize,
        drops_from: DropsFrom,
    },
    SourceIsExptectedButEmpty {
        card: String,
        record_id: usize,
    },

    GreynoteDisabledButCardNotLegacy {
        card: String,
        record_id: usize,
    },

    LegacyCardShouldBeMarkedAsDisabled {
        card: String,
        record_id: usize,
    },
}

impl Display for ParseSourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseSourceError::UnknownVariant {
                record_id,
                drops_from,
                card,
            } => {
                write!(
                    f,
                    "{record_id}.{card}. Unknown variant of card source {}. {}",
                    drops_from.name,
                    record_url(*record_id, DivcordColumn::Source)
                )
            }
            ParseSourceError::SourceIsExptectedButEmpty { record_id, card } => {
                write!(
                    f,
                    "{record_id}.{card}.  Source is expected, but there is nothing. {}",
                    record_url(*record_id, DivcordColumn::Source)
                )
            }
            ParseSourceError::GreynoteDisabledButCardNotLegacy { record_id, card } => {
                write!(
                    f,
                    "{record_id}. Card {card} has greynote Disabled, but this is not a legacy card {}", record_url(*record_id, DivcordColumn::GreyNote)
                )
            }
            ParseSourceError::LegacyCardShouldBeMarkedAsDisabled { record_id, card } => write!(
                f,
                "{record_id}. Card {card} is legacy, but not marked as disabled. {}",
                record_url(*record_id, DivcordColumn::GreyNote)
            ),
        }
    }
}

pub enum DivcordColumn {
    GreyNote,
    Card,
    TagHypothesis,
    Confidence,
    ConfidenceNew325,
    RemainingWork,
    New325Confirmations,
    Source,
    WikiDisagreements,
    SourcesWithTagButNotOnWiki,
    Notes,
}

impl DivcordColumn {
    pub fn letter(&self) -> &str {
        match self {
            DivcordColumn::GreyNote => "A",
            DivcordColumn::Card => "B",
            DivcordColumn::TagHypothesis => "C",
            DivcordColumn::ConfidenceNew325 => "D",
            DivcordColumn::Confidence => "E",
            DivcordColumn::RemainingWork => "F",
            DivcordColumn::New325Confirmations => "G",
            DivcordColumn::Source => "H",
            DivcordColumn::WikiDisagreements => "I",
            DivcordColumn::SourcesWithTagButNotOnWiki => "J",
            DivcordColumn::Notes => "K",
        }
    }
}

pub fn record_url(id: usize, column: DivcordColumn) -> String {
    format!("https://docs.google.com/spreadsheets/d/1Pf2KNuGguZLyf6eu_R0E503U0QNyfMZqaRETsN5g6kU/edit?pli=1#gid=0&range={}{id}", column.letter())
}

/// Add new sources:
/// - Kirac Missions
/// - All (Scourge) beyond demons, including from Settlers (Crimson Iron/Orichalcum)
/// - Vaal side area bosses (including Vaal Temple)
///

pub fn parse_record_dropsources(
    dumb: &Dumb,
    poe_data: &PoeData,
) -> Result<Vec<Source>, ParseSourceError> {
    let mut sources: Vec<Source> = vec![];

    // 1. Legacy cards rules
    if dumb.card.as_str().is_legacy_card() && dumb.greynote != GreyNote::Disabled {
        return Err(ParseSourceError::LegacyCardShouldBeMarkedAsDisabled {
            record_id: dumb.id,
            card: dumb.card.to_owned(),
        });
    }

    if dumb.greynote == GreyNote::Disabled {
        if !dumb.card.as_str().is_legacy_card() {
            return Err(ParseSourceError::GreynoteDisabledButCardNotLegacy {
                record_id: dumb.id,
                card: dumb.card.to_owned(),
            });
        }
        sources.push(Source::Disabled);
        return Ok(sources);
    }

    // 2. Parse sources from "Wiki Map/Monster Agreements" column(the main part)
    sources.append(&mut parse_dropses_from(
        dumb,
        poe_data,
        RichColumnVariant::Sources,
    )?);

    // 3. Read from tags(3rd column)
    if dumb.tag_hypothesis.as_deref() == Some("invasion_boss") {
        sources.push(Source::UniqueMonster(UniqueMonster::AllInvasionBosses))
    }

    if dumb.tag_hypothesis.as_deref() == Some("vaalsidearea_boss") {
        sources.push(Source::UniqueMonster(UniqueMonster::AllVaalSideAreaBosses))
    }

    if dumb.tag_hypothesis.as_deref() == Some("expedition_common_remnant_logbook") {
        sources.push(Source::Area(Area::ExpeditionLogbook))
    }

    // 4. Read greynotes(first column)

    if dumb.greynote == GreyNote::Delirium
        && dumb.notes.as_deref()
            == Some("Appears to drop from any source of Delirium Currency rewards")
    {
        sources.push(Source::DeliriumCurrencyRewards);
    }

    if dumb.greynote == GreyNote::Vendor && dumb.notes.as_deref() == Some("Kirac shop") {
        sources.push(Source::Vendor(Vendor::KiracShop));
    }

    // 5. Read notes(last column)
    if dumb.notes.as_deref() == Some("Redeemer influenced maps") {
        sources.push(Source::Area(Area::RedeemerInfluencedMaps))
    }

    // 6. Final rules
    if dumb.confidence == Confidence::None && !sources.is_empty() {
        // println!("{} {} {sources:?}", record.id, record.card);
    }

    // if dumb.greynote != GreyNote::Empty && sources.is_empty() && dumb.confidence == Confidence::Done
    // {
    //     return Err(ParseSourceError::SourceIsExptectedButEmpty {
    //         record_id: dumb.id,
    //         card: dumb.card.to_owned(),
    //     });
    // }

    Ok(sources)
}

// PRE 3.25. Copy and create new one
// pub fn parse_record_dropsources(
//     dumb: &Dumb,
//     poe_data: &PoeData,
// ) -> Result<Vec<Source>, ParseSourceError> {
//     let mut sources: Vec<Source> = vec![];

//     // 1. Legacy cards rules
//     if dumb.card.as_str().is_legacy_card() && dumb.greynote != GreyNote::Disabled {
//         return Err(ParseSourceError::LegacyCardShouldBeMarkedAsDisabled {
//             record_id: dumb.id,
//             card: dumb.card.to_owned(),
//         });
//     }

//     if dumb.greynote == GreyNote::Disabled {
//         if !dumb.card.as_str().is_legacy_card() {
//             return Err(ParseSourceError::GreynoteDisabledButCardNotLegacy {
//                 record_id: dumb.id,
//                 card: dumb.card.to_owned(),
//             });
//         }
//         sources.push(Source::Disabled);
//         return Ok(sources);
//     }

//     // 2. Parse sources from "Wiki Map/Monster Agreements" column(the main part)
//     sources.append(&mut parse_dropses_from(
//         dumb,
//         poe_data,
//         RichColumnVariant::Sources,
//     )?);

//     // 3. Read from tags(3rd column)
//     if dumb.tag_hypothesis.as_deref() == Some("invasion_boss") {
//         sources.push(Source::UniqueMonster(UniqueMonster::AllInvasionBosses))
//     }

//     if dumb.tag_hypothesis.as_deref() == Some("vaalsidearea_boss") {
//         sources.push(Source::UniqueMonster(UniqueMonster::AllVaalSideAreaBosses))
//     }

//     if dumb.tag_hypothesis.as_deref() == Some("expedition_common_remnant_logbook") {
//         sources.push(Source::Area(Area::ExpeditionLogbook))
//     }

//     // 4. Read greynotes(first column)
//     if dumb.greynote == GreyNote::GlobalDrop {
//         let Card {
//             min_level,
//             max_level,
//             ..
//         } = poe_data.cards.card(&dumb.card).to_owned();
//         sources.push(Source::GlobalDrop {
//             min_level,
//             max_level,
//         });
//     };

//     if dumb.greynote == GreyNote::Delirium
//         && dumb.notes.as_deref()
//             == Some("Appears to drop from any source of Delirium Currency rewards")
//     {
//         sources.push(Source::DeliriumCurrencyRewards);
//     }

//     if dumb.greynote == GreyNote::Vendor && dumb.notes.as_deref() == Some("Kirac shop") {
//         sources.push(Source::Vendor(Vendor::KiracShop));
//     }

//     // 5. Read notes(last column)
//     if dumb.notes.as_deref() == Some("Redeemer influenced maps") {
//         sources.push(Source::Area(Area::RedeemerInfluencedMaps))
//     }

//     // 6. Final rules
//     if dumb.confidence == Confidence::None && !sources.is_empty() {
//         // println!("{} {} {sources:?}", record.id, record.card);
//     }

//     if dumb.greynote != GreyNote::Empty && sources.is_empty() && dumb.confidence == Confidence::Done
//     {
//         return Err(ParseSourceError::SourceIsExptectedButEmpty {
//             record_id: dumb.id,
//             card: dumb.card.to_owned(),
//         });
//     }

//     Ok(sources)
// }

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RichColumnVariant {
    Sources,
    Verify,
}

impl RichColumnVariant {
    pub fn column_letter(&self) -> char {
        match self {
            RichColumnVariant::Sources => 'G',
            RichColumnVariant::Verify => 'H',
        }
    }
}

/// Parses all instances of record's drops_from and collects it into one Vec<Source>
pub fn parse_dropses_from(
    dumb: &Dumb,
    poe_data: &PoeData,
    column: RichColumnVariant,
) -> Result<Vec<Source>, ParseSourceError> {
    let mut sources: Vec<Source> = vec![];

    match column {
        RichColumnVariant::Sources => {
            for d in &dumb.drops {
                let Ok(mut inner_sources) = parse_one_drops_from(d, dumb, poe_data) else {
                    return Err(ParseSourceError::UnknownVariant {
                        card: dumb.card.to_owned(),
                        record_id: dumb.id,
                        drops_from: d.to_owned(),
                    });
                };
                sources.append(&mut inner_sources);
            }
        }
        RichColumnVariant::Verify => {
            for d in &dumb.drops_to_verify {
                let Ok(mut inner_sources) = parse_one_drops_from(d, dumb, poe_data) else {
                    return Err(ParseSourceError::UnknownVariant {
                        card: dumb.card.to_owned(),
                        record_id: dumb.id,
                        drops_from: d.to_owned(),
                    });
                };
                sources.append(&mut inner_sources);
            }
        }
    }

    Ok(sources)
}

pub fn parse_one_drops_from(
    d: &DropsFrom,
    dumb: &Dumb,
    poe_data: &PoeData,
) -> Result<Vec<Source>, ParseSourceError> {
    if d.styles.strikethrough {
        return Ok(vec![]);
    }

    let PoeData {
        acts,
        cards,
        maps,
        mapbosses,
    } = poe_data;

    let card = cards.card(&dumb.card);
    let card_drop_level_requirement = card.min_level.unwrap_or_default();
    let card_name = &dumb.card;
    let row = dumb.id;
    if let Ok(source) = d.name.parse::<Source>() {
        match source.to_string().as_str() {
            "The Alluring Abyss" => {
                if card_drop_level_requirement > 80 {
                    println!(
                        "Row: {row} Card: {card_name} Warning. Min level of card({card_drop_level_requirement}) is higher than Map level({}). Map: The Alluring Abyss",
                        80
                    );
                }
            }
            "The Apex of Sacrifice" => {
                if card_drop_level_requirement > 70 {
                    println!(
                        "Row: {row} Card: {card_name} Warning. Min level of card({card_drop_level_requirement}) is higher than Map level({}). Map: The Apex of Sacrifice",
                        70
                    );
                }
            }
            _ => {}
        };
        return Ok(vec![source]);
    }

    // Acts areas or act area bosses
    if d.styles.italic && (d.styles.color == HexColor::White || dumb.greynote == GreyNote::Story) {
        let ids = acts::parse_act_areas(d, acts, card_drop_level_requirement.try_into().unwrap());
        if ids.is_empty() {
            if acts.iter().any(|a| {
                a.bossfights.iter().any(|b| {
                    let names_match = b.name == d.name;
                    if names_match {
                        let monster_level = a.area_level as u32 + 2u32;
                        let level_matches = monster_level >= card_drop_level_requirement;
                        if !level_matches {
                            println!("Level of monster level is lower than card drop requirement");
                        };

                        level_matches
                    } else {
                        false
                    }
                    // b.name == d.name && a.area_level + 2 >= card_drop_level_requirement as u8
                })
            }) {
                return Ok(vec![Source::ActBoss(d.name.to_string())]);
            } else {
                println!(
                    "From acts parsing. Could not resolve the source of the name: {} {d:#?} dumb_id: {}.",
                    &d.name, dumb.id
                );
            }
        }

        // return Some(Source::Acts { ids });
        return Ok(ids.into_iter().map(Source::Act).collect());
    }

    // Maps or MapBosses
    if (!d.styles.italic && d.styles.color == HexColor::White)
        || dumb.greynote == GreyNote::AreaSpecific
    {
        let s = &d.name;

        if let Some(map) = maps.iter().find(|m| {
            let shortname = m.name.replace(" Map", "");
            s == &shortname || s == &m.name
        }) {
            // let maplevel = map.level();
            // if maplevel < card_drop_level_requirement as u32 {
            // let mapname = &map.name;
            // println!(
            //     "{row} {card_name}. {mapname}(lv{maplevel}), need lv{card_drop_level_requirement}"
            // );
            // }
            return Ok(vec![Source::Map(map.name.to_owned())]);
        }

        let s = strip_comment(s);
        if mapbosses.iter().any(|b| b.name == s) {
            return Ok(vec![Source::MapBoss(s)]);
        }
    }

    Err(ParseSourceError::UnknownVariant {
        card: dumb.card.to_owned(),
        record_id: dumb.id,
        drops_from: d.to_owned(),
    })
}

fn strip_comment(input: &str) -> String {
    let mut result = String::new();
    let mut inside_brackets = false;

    for c in input.chars() {
        match c {
            '(' => inside_brackets = true,
            ')' => inside_brackets = false,
            _ => {
                if !inside_brackets {
                    result.push(c);
                }
            }
        }
    }

    result.trim().to_owned()
}

mod acts {
    use crate::spreadsheet::rich::DropsFrom;
    use poe_data::act::{ActArea, ActAreaId};
    use serde::{Deserialize, Serialize};

    /// Examples of acts areas in divcord spreadsheet:
    /// - "The Blood Aqueduct"
    /// - "The Solaris Temple Level 1/2 (A8)"
    /// - "The Ossuary (A5/A10)"
    /// - "The Riverways (A6)"
    #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
    enum ActAreaDivcordNotation {
        #[serde(untagged)]
        Name(String),
        #[serde(untagged)]
        NameWithAct((String, u8)),
    }

    pub fn parse_act_areas(
        drops_from: &DropsFrom,
        acts: &[ActArea],
        min_level: u8,
    ) -> Vec<ActAreaId> {
        if !drops_from.styles.italic {
            panic!("Act areas should be italic");
        }

        let s = &drops_from.name;
        let names = match are_act_numbers_specified(s) {
            true if s == "The Belly of the Beast (A4/A9)" => vec![
                ActAreaDivcordNotation::NameWithAct((
                    "The Belly of the Beast Level 1".to_string(),
                    4,
                )),
                ActAreaDivcordNotation::NameWithAct((
                    "The Belly of the Beast Level 2".to_string(),
                    4,
                )),
                ActAreaDivcordNotation::NameWithAct(("The Belly of the Beast".to_string(), 9)),
            ],
            true => parse_with_act_numbers(s),
            false => vec![ActAreaDivcordNotation::Name(s.to_owned())],
        };

        let areas = names
            .iter()
            .flat_map(|name| find_ids(name, acts, min_level))
            .collect();

        areas
    }

    fn are_act_numbers_specified(s: &str) -> bool {
        match s {
            s if s.contains('(') && s.contains(')') => true,
            s if s.contains("1/2") => true,
            _ => false,
        }
    }

    fn parse_with_act_numbers(s: &str) -> Vec<ActAreaDivcordNotation> {
        if !s.contains('(') && !s.contains('/') {
            panic!("Expected act notation, got {s}");
        };

        let mut split = s.split('(');
        let name = split.next().expect("No name, {s}");
        let names = match name.contains("1/2") {
            true => {
                let name = name.replace("1/2", "");
                let name = name.trim();
                [1, 2].iter().map(|n| format!("{name} {n}")).collect()
            }
            false => vec![name.trim().to_string()],
        };

        if let Some(acts) = split.next() {
            if acts.contains('/') {
                let (left, right) = acts.split_once('/').unwrap();

                let left = left
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse::<u8>()
                    .unwrap();

                let right = right
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse::<u8>()
                    .unwrap();

                names
                    .into_iter()
                    .flat_map(|name| {
                        [
                            ActAreaDivcordNotation::NameWithAct((name.clone(), left)),
                            ActAreaDivcordNotation::NameWithAct((name, right)),
                        ]
                    })
                    .collect()
            } else {
                let f: Box<dyn Fn(String) -> ActAreaDivcordNotation> = match acts
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse::<u8>()
                {
                    Ok(act) => Box::new(move |name: String| {
                        ActAreaDivcordNotation::NameWithAct((name, act))
                    }),
                    Err(_) => Box::new(|name: String| {
                        println!("No act notation in brackets {acts}");
                        ActAreaDivcordNotation::Name(name)
                    }),
                };

                names.into_iter().map(f).collect()
            }
        } else {
            names
                .into_iter()
                .map(ActAreaDivcordNotation::Name)
                .collect()
        }
    }

    fn find_ids(name: &ActAreaDivcordNotation, acts: &[ActArea], min_level: u8) -> Vec<ActAreaId> {
        match name {
            ActAreaDivcordNotation::Name(name) => acts
                .iter()
                .filter(|a| &a.name == name && !a.is_town && (a.area_level + 2) >= min_level)
                .map(|a| a.id.to_owned())
                .collect(),
            ActAreaDivcordNotation::NameWithAct((name, act)) => {
                let mut v = vec![];
                if let Some(a) = acts
                    .iter()
                    .find(|a| &a.name == name && &a.act == act && (a.area_level + 2) >= min_level)
                {
                    v.push(a.id.to_owned())
                };

                v
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn act_area_notation() {
            assert_eq!(
                parse_with_act_numbers("The Solaris Temple Level 1/2 (A8)"),
                vec![
                    ActAreaDivcordNotation::NameWithAct((
                        "The Solaris Temple Level 1".to_owned(),
                        8
                    )),
                    ActAreaDivcordNotation::NameWithAct((
                        "The Solaris Temple Level 2".to_owned(),
                        8
                    ))
                ]
            );

            assert_eq!(
                parse_with_act_numbers("The Ossuary (A5/A10)"),
                vec![
                    ActAreaDivcordNotation::NameWithAct(("The Ossuary".to_owned(), 5)),
                    ActAreaDivcordNotation::NameWithAct(("The Ossuary".to_owned(), 10))
                ]
            );

            assert_eq!(
                parse_with_act_numbers("The Riverways (A6)"),
                vec![ActAreaDivcordNotation::NameWithAct((
                    "The Riverways".to_owned(),
                    6
                ))]
            );
        }
    }
}
