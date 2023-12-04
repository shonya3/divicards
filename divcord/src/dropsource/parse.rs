use divi::IsCard;
use std::fmt::Display;

use crate::table::{
    rich::DropsFrom,
    table_record::{Confidence, DivcordTableRecord, GreyNote},
};

use poe_data::{
    act::{ActArea, ActAreaName},
    cards::Card,
    PoeData,
};

use super::{monster::UniqueMonster, Source, Vendor};

#[derive(Debug)]
pub enum ParseSourceError {
    NoSourceParsingDropsFrom {
        record_id: usize,
        drops_from: DropsFrom,
    },
    SourceIsExptectedButEmpty {
        record_id: usize,
    },

    GreynoteDisabledCardShouldBeLegacy {
        record_id: usize,
        card: String,
    },

    LegacyCardShouldBeMarkedAsDisabled {
        record_id: usize,
        card: String,
    },
}

impl Display for ParseSourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseSourceError::NoSourceParsingDropsFrom {
                record_id,
                drops_from,
            } => {
                write!(
                    f,
                    "Parsing drops_from, no source found. Record id: {record_id}. {}",
                    drops_from.name
                )
            }
            ParseSourceError::SourceIsExptectedButEmpty { record_id } => {
                write!(
                    f,
                    "Source is expected, but there is nothing. Record id: {record_id}"
                )
            }
            ParseSourceError::GreynoteDisabledCardShouldBeLegacy { record_id, card } => {
                write!(
                    f,
                    "Record {record_id}. Card {card} has greynote Disabled, but this is not a legacy card"
                )
            }
            ParseSourceError::LegacyCardShouldBeMarkedAsDisabled { record_id, card } => write!(
                f,
                "Record {record_id}. Card {card} is legacy, but not marked as disabled"
            ),
        }
    }
}

pub fn parse_record_dropsources(
    record: &DivcordTableRecord,
    poe_data: &PoeData,
) -> Result<Vec<Source>, ParseSourceError> {
    let mut sources: Vec<Source> = vec![];

    // 1. Legacy cards rules
    if record.card.as_str().is_legacy_card() && record.greynote != Some(GreyNote::Disabled) {
        return Err(ParseSourceError::LegacyCardShouldBeMarkedAsDisabled {
            record_id: record.id,
            card: record.card.to_owned(),
        });
    }

    if record.greynote == Some(GreyNote::Disabled) {
        if !record.card.as_str().is_legacy_card() {
            return Err(ParseSourceError::GreynoteDisabledCardShouldBeLegacy {
                record_id: record.id,
                card: record.card.to_owned(),
            });
        }
        sources.push(Source::Disabled);
        return Ok(sources);
    }

    // 2. Parse sources from "Wiki Map/Monster Agreements" column(the main part)
    sources.append(&mut parse_dropses_from(record, poe_data)?);

    // 3. Read from tags(3rd column)
    if record.tag_hypothesis == Some(String::from("invasion_boss")) {
        sources.push(Source::UniqueMonster(UniqueMonster::AllInvasionBosses))
    }

    if record.tag_hypothesis == Some(String::from("vaalsidearea_boss")) {
        sources.push(Source::UniqueMonster(UniqueMonster::AllVaalSideAreaBosses))
    }

    // 4. Read greynotes(first column)
    if record.greynote == Some(GreyNote::GlobalDrop) {
        let Card {
            min_level,
            max_level,
            ..
        } = poe_data.cards.card(&record.card).to_owned();
        sources.push(Source::GlobalDrop {
            min_level,
            max_level,
        });
    };

    if record.greynote == Some(GreyNote::Delirium) {
        if record.notes
            == Some("Appears to drop from any source of Delirium Currency rewards".to_string())
        {
            sources.push(Source::DeliriumCurrencyRewards);
        }
    }

    if record.greynote == Some(GreyNote::Vendor) {
        if record.notes == Some(String::from("Kirac shop")) {
            sources.push(Source::Vendor(Vendor::KiracShop));
        }
    }

    // 5. Read notes(last column)
    if record.notes == Some(String::from("Redeemer influenced maps")) {
        sources.push(Source::RedeemerInfluencedMaps)
    }

    // 6. Final rules
    if record.confidence == Confidence::None && sources.len() > 0 {
        // println!("{} {} {sources:?}", record.id, record.card);
    }

    if record.greynote.is_some() && sources.is_empty() && record.confidence == Confidence::Done {
        return Err(ParseSourceError::SourceIsExptectedButEmpty {
            record_id: record.id,
        });
    }

    Ok(sources)
}

/// Parses all instances of record's drops_from and collects it into one Vec<Source>
pub fn parse_dropses_from(
    record: &DivcordTableRecord,
    poe_data: &PoeData,
) -> Result<Vec<Source>, ParseSourceError> {
    let mut sources: Vec<Source> = vec![];
    for d in &record.drops_from {
        let Ok(mut inner_sources) = parse_one_drops_from(d, &record, poe_data) else {
            return Err(ParseSourceError::NoSourceParsingDropsFrom {
                record_id: record.id,
                drops_from: d.to_owned(),
            });
        };
        sources.append(&mut inner_sources);
    }

    Ok(sources)
}

pub fn parse_one_drops_from(
    d: &DropsFrom,
    record: &DivcordTableRecord,
    poe_data: &PoeData,
) -> Result<Vec<Source>, ParseSourceError> {
    if d.styles.strikethrough == true {
        return Ok(vec![]);
    }

    let PoeData {
        acts,
        cards,
        maps,
        mapbosses,
    } = poe_data;

    let card = cards.card(&record.card);
    let card_drop_level_requirement = card.min_level.unwrap_or_default();
    let card_name = &record.card;
    let row = record.id;
    if let Ok(source) = d.name.parse::<Source>() {
        match source.to_string().as_str() {
            "The Alluring Abyss" => {
                if card_drop_level_requirement > 80 {
                    println!(
                        "Row: {row} Card: {card_name} Warning. Min level of card({card_drop_level_requirement}) is higher than Map level({}). Map: {}",
                        80, "The Alluring Abyss"
                    );
                }
            }
            "The Apex of Sacrifice" => {
                if card_drop_level_requirement > 70 {
                    println!(
                        "Row: {row} Card: {card_name} Warning. Min level of card({card_drop_level_requirement}) is higher than Map level({}). Map: {}",
                        70, "The Apex of Sacrifice"
                    );
                }
            }
            _ => {}
        };
        return Ok(vec![source]);
    }

    // Acts areas or act area bosses
    if d.styles.italic == true
        && (d.styles.color.as_str() == "#FFFFFF" || record.greynote == Some(GreyNote::Story))
    {
        let ids = parse_act_areas(d, &acts, card_drop_level_requirement.try_into().unwrap());
        if ids.is_empty() {
            if acts.iter().any(|a| {
                a.bossfights.iter().any(|b| {
                    b.name == d.name
                        && a.area_level >= card_drop_level_requirement.try_into().unwrap()
                })
            }) {
                return Ok(vec![Source::ActBoss {
                    name: d.name.to_string(),
                }]);
            } else {
                println!(
                    "From acts parsing. Could not resolve the source of the name: {}",
                    &d.name
                );
            }
        }

        // return Some(Source::Acts { ids });
        return Ok(ids.into_iter().map(|id| Source::Act { id }).collect());
    }

    // Maps or MapBosses
    if (d.styles.italic == false && d.styles.color.as_str() == "#FFFFFF")
        || record.greynote == Some(GreyNote::AreaSpecific)
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
            return Ok(vec![Source::Map {
                name: map.name.to_owned(),
            }]);
        }

        let s = s.split("(").next().unwrap().trim().to_string();
        if let Some(_) = mapbosses.iter().find(|b| b.name == s) {
            return Ok(vec![Source::MapBoss { name: s }]);
        }
    }

    Err(ParseSourceError::NoSourceParsingDropsFrom {
        record_id: record.id,
        drops_from: d.to_owned(),
    })
}

pub fn parse_act_areas(drops_from: &DropsFrom, acts: &[ActArea], min_level: u8) -> Vec<String> {
    if !drops_from.styles.italic {
        panic!("Act areas should be italic");
    }

    let s = &drops_from.name;
    let names = match is_act_notation(s) {
        true if s == "The Belly of the Beast (A4/A9)" => vec![
            ActAreaName::NameWithAct(("The Belly of the Beast Level 1".to_string(), 4)),
            ActAreaName::NameWithAct(("The Belly of the Beast Level 2".to_string(), 4)),
            ActAreaName::NameWithAct(("The Belly of the Beast".to_string(), 9)),
        ],
        true => parse_act_notation(s),
        false => vec![ActAreaName::Name(s.to_owned())],
    };

    names
        .iter()
        .flat_map(|name| find_ids(&name, acts, min_level))
        .collect()
}

pub fn is_act_notation(s: &str) -> bool {
    match s {
        s if s.contains("(") && s.contains(")") => true,
        s if s.contains("1/2") => true,
        _ => false,
    }
}

pub fn parse_act_notation(s: &str) -> Vec<ActAreaName> {
    if !s.contains("(") && !s.contains("/") {
        panic!("Expected act notation, got {s}");
    };

    let mut split = s.split("(");

    let name = split.next().expect("No name, {s}");
    let mut names: Vec<String> = Vec::new();

    if name.contains("1/2") {
        let name = name.replace("1/2", "");
        let name = name.trim();
        for n in [1, 2] {
            let name = format!("{name} {n}");
            names.push(name);
        }
    } else {
        names.push(name.to_string());
    }

    let names = match name.contains("1/2") {
        true => {
            let name = name.replace("1/2", "");
            let name = name.trim();
            [1, 2].iter().map(|n| format!("{name} {n}")).collect()
        }
        false => vec![name.trim().to_string()],
    };

    if let Some(acts) = split.next() {
        if acts.contains("/") {
            let (left, right) = acts.split_once("/").unwrap();

            let left = left
                .chars()
                .into_iter()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .parse::<u8>()
                .unwrap();

            let right = right
                .chars()
                .into_iter()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .parse::<u8>()
                .unwrap();

            names
                .into_iter()
                .flat_map(|name| {
                    [
                        ActAreaName::NameWithAct((name.clone(), left)),
                        ActAreaName::NameWithAct((name, right)),
                    ]
                })
                .collect()
        } else {
            let left = acts
                .chars()
                .into_iter()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .parse::<u8>()
                .unwrap();

            names
                .into_iter()
                .map(|name| ActAreaName::NameWithAct((name, left)))
                .collect()
        }
    } else {
        names
            .into_iter()
            .map(|name| ActAreaName::Name(name))
            .collect()
    }
}

pub fn find_ids(name: &ActAreaName, acts: &[ActArea], min_level: u8) -> Vec<String> {
    match name {
        ActAreaName::Name(name) => acts
            .iter()
            .filter(|a| &a.name == name && a.is_town == false && a.area_level >= min_level)
            .map(|a| a.id.to_owned())
            .collect(),
        ActAreaName::NameWithAct((name, act)) => {
            let mut v = vec![];
            if let Some(a) = acts
                .iter()
                .find(|a| &a.name == name && &a.act == act && a.area_level >= min_level)
            {
                v.push(a.id.to_owned())
            };

            v
        }
    }
}
