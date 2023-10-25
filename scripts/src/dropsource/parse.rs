use std::fmt::Display;

use crate::{
    poe_data::{act::parse_act_areas, PoeData},
    table::{
        rich::DropsFrom,
        table_record::{DivcordTableRecord, GreyNote},
    },
};

use super::Source;

#[derive(Debug)]
pub struct ParseSourceError(pub DropsFrom);
impl Display for ParseSourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Source variant not found for {:?}", self.0)
    }
}

pub fn parse_source(
    d: &DropsFrom,
    record: &DivcordTableRecord,
    poe_data: &PoeData,
) -> Result<Vec<Source>, ParseSourceError> {
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

    Err(ParseSourceError(d.to_owned()))
}
