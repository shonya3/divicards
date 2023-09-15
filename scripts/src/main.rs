mod comments;
pub mod error;
pub mod scripts;

use std::collections::HashMap;

use divi::{sample::fix_name, IsCard};
use serde::{Deserialize, Serialize};

use error::Error;
use googlesheets::sheet::ValueRange;
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum GreyNote {
    #[serde(alias = "Monster-specific")]
    MonsterSpecific,
    #[serde(alias = "Area-specific")]
    AreaSpecific,
    #[serde(alias = "disabled", alias = "Drop disabled")]
    Disabled,
    #[serde(alias = "story")]
    Story,
    #[serde(alias = "Delirium_reward")]
    Delirium,
    #[serde(alias = "Chest_object", alias = "Chest_obkect")]
    ChestObject,
    #[serde(alias = "strongbox")]
    Strongbox,
    #[serde(alias = "Global Drop")]
    GlobalDrop,
    #[serde(alias = "Vendor")]
    Vendor,
}

impl GreyNote {
    pub fn parse(val: &Value) -> Result<Option<Self>, Error> {
        let Some(s) = val.as_str() else {
            return Ok(None);
        };
        if s.is_empty() || s == "n/a" {
            return Ok(None);
        } else {
            let greynote = serde_json::from_str(&val.to_string())?;
            Ok(greynote)
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Confidence {
    #[serde(alias = "none")]
    None,
    #[serde(alias = "Low", alias = "low")]
    Low,
    #[serde(alias = "OK", alias = "ok")]
    Ok,
    #[serde(alias = "DONE", alias = "Done")]
    Done,
}

impl Confidence {
    pub fn parse(val: &Value) -> Result<Self, Error> {
        let conf: Confidence = serde_json::from_str(&val.to_string())?;
        Ok(conf)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum RemainingWork {
    #[serde(alias = "confirm")]
    Confirm,
    #[serde(alias = "unclear hypothesis")]
    UnclearHypothesis,
    #[serde(alias = "no hypothesis")]
    NoHypothesis,
    #[serde(alias = "story only")]
    StoryOnly,
    #[serde(alias = "legacy tag")]
    LegacyTag,
    #[serde(alias = "open ended")]
    OpenEnded,
}

impl RemainingWork {
    pub fn parse(val: &Value) -> Result<Option<Self>, Error> {
        let Some(s) = val.as_str() else {
            return Ok(None);
        };
        if s.is_empty() || s == "n/a" {
            return Ok(None);
        } else {
            let remaining_work = serde_json::from_str(&val.to_string())?;
            Ok(remaining_work)
        }
    }
}

pub fn read_original_table_sheet() -> ValueRange {
    let value_range: ValueRange =
        serde_json::from_str(&std::fs::read_to_string("areas.json").unwrap()).unwrap();
    value_range
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum DropSource {
    GlobalDrop,
    ChestObject,
    Map(String),
    MapBoss { boss: String, map: String },
    Disabled,
    Unknown,
    Delirium,
    Vendor,
    Strongbox,
}

pub fn parse_greynote(val: &Value) -> Result<Option<GreyNote>, Error> {
    GreyNote::parse(val)
}

pub fn parse_name(val: &Value) -> Result<String, Error> {
    let Some(second_column_contents) = val.as_str() else {
        return Err(Error::ValueNotStr(val.to_owned()));
    };

    match second_column_contents.is_card() {
        true => Ok(second_column_contents.to_string()),
        false => match fix_name(second_column_contents) {
            Some(s) => Ok(s),
            None => Err(Error::ParseNameError(second_column_contents.to_string())),
        },
    }
}

pub fn parse_confidence(val: &Value) -> Result<Confidence, Error> {
    Confidence::parse(val)
}

pub fn parse_remaining_work(val: &Value) -> Result<Option<RemainingWork>, Error> {
    RemainingWork::parse(val)
}

pub fn parse_string_cell(val: &Value) -> Option<String> {
    let Some(s) = val.as_str() else { return None };
    if s.is_empty() || s == "n/a" {
        return None;
    } else {
        return Some(s.to_string());
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CardDropRecord {
    pub greynote: Option<GreyNote>,
    pub name: String,
    pub hypothesis: Option<String>,
    pub confidence: Confidence,
    pub remaining_work: Option<RemainingWork>,
    pub drops_from: Option<String>,
    pub wiki_disagreements: Option<String>,
    pub sources_with_tag_but_not_on_wiki: Option<String>,
    pub notes: Option<String>,
}

pub fn parse_row(row: &[Value]) -> Result<CardDropRecord, Error> {
    let greynote = parse_greynote(&row[0])?;
    let name = parse_name(&row[1])?;
    let hypothesis = parse_string_cell(&row[2]);
    let confidence = parse_confidence(&row[3])?;
    let remaining_work = parse_remaining_work(&row[4])?;
    let drops_from = row.get(5).map(|val| parse_string_cell(val)).flatten();
    let wiki_disagreements = row.get(6).map(|val| parse_string_cell(val)).flatten();
    let sources_with_tag_but_not_on_wiki = row.get(7).map(|val| parse_string_cell(val)).flatten();
    let notes = row.get(8).map(|val| parse_string_cell(val)).flatten();

    Ok(CardDropRecord {
        greynote,
        name,
        hypothesis,
        confidence,
        remaining_work,
        drops_from,
        wiki_disagreements,
        sources_with_tag_but_not_on_wiki,
        notes,
    })
}

fn main() {
    let vr = read_original_table_sheet();
    let mut map: HashMap<String, Vec<CardDropRecord>> = HashMap::new();
    for row in &vr.values[2..] {
        let record = parse_row(row).unwrap();
        let r = map.entry(record.name.as_str().to_owned()).or_insert(vec![]);
        r.push(record);
    }

    dbg!(map.keys().len());
}

// pub fn parse_map_source(row: &[Value]) -> Result<DropSource, Error> {
//     if row.len() < 6 {
//         return Err(Error::RowIsTooShort("Drop Source".to_string(), 6));
//     };

//     let greynote = parse_greynote(&row[0])?;

//     // match greynote {
//     //     GreyNote::Disabled => return Ok(DropSource::Disabled),
//     //     GreyNote::Delirium => return Ok(DropSource::Delirium),
//     //     GreyNote::ChestObject => return Ok(DropSource::ChestObject),
//     //     GreyNote::GlobalDrop => return Ok(DropSource::GlobalDrop),
//     //     GreyNote::Vendor => return Ok(DropSource::Vendor),
//     //     GreyNote::Strongbox => return Ok(DropSource::Strongbox),
//     //     GreyNote::AreaSpecific => todo!(),
//     //     GreyNote::MonsterSpecific => todo!(),
//     //     GreyNote::Story => todo!(),
//     // }
// }

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::parse_greynote;

    use super::*;

    #[test]
    fn parses_table_without_errors() {
        let vr = read_original_table_sheet();
        for row in &vr.values[2..] {
            parse_row(row).unwrap();
        }
    }

    #[test]
    fn test_parse_greynote() {
        let mut vec: Vec<Vec<Value>> = vec![];
        for val in &read_original_table_sheet().values {
            if let Err(_) = parse_greynote(&val[0]) {
                vec.push(val.to_owned());
                dbg!(val);
            }
        }
        assert_eq!(vec.len(), 0);

        assert_eq!(
            Some(GreyNote::AreaSpecific),
            parse_greynote(&json!("Area-specific")).unwrap()
        );
        assert_eq!(
            Some(GreyNote::MonsterSpecific),
            parse_greynote(&json!("Monster-specific")).unwrap()
        );
        assert_eq!(
            Some(GreyNote::Disabled),
            parse_greynote(&json!("disabled")).unwrap()
        );
        assert_eq!(
            Some(GreyNote::Disabled),
            parse_greynote(&json!("disabled")).unwrap()
        );
        assert_eq!(
            Some(GreyNote::Disabled),
            parse_greynote(&json!("Drop disabled")).unwrap()
        );
        assert_eq!(
            Some(GreyNote::Story),
            parse_greynote(&json!("story")).unwrap()
        );
        assert_eq!(
            Some(GreyNote::Delirium),
            parse_greynote(&json!("Delirium_reward")).unwrap()
        );
        assert_eq!(
            Some(GreyNote::ChestObject),
            parse_greynote(&json!("Chest_object")).unwrap()
        );
        assert_eq!(
            Some(GreyNote::ChestObject),
            parse_greynote(&json!("Chest_obkect")).unwrap()
        );
        assert_eq!(
            Some(GreyNote::Strongbox),
            parse_greynote(&json!("strongbox")).unwrap()
        );
        assert_eq!(
            Some(GreyNote::GlobalDrop),
            parse_greynote(&json!("Global Drop")).unwrap()
        );
        assert_eq!(
            Some(GreyNote::Vendor),
            parse_greynote(&json!("Vendor")).unwrap()
        );
        assert_eq!(None, parse_greynote(&json!("")).unwrap());
        assert_eq!(None, parse_greynote(&json!("n/a")).unwrap());
    }

    #[test]
    fn test_parse_name() {
        let mut vec: Vec<Vec<Value>> = vec![];
        for val in &read_original_table_sheet().values[2..] {
            if let Err(_) = super::parse_name(&val[1]) {
                vec.push(val.to_owned());
            }
        }

        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn test_parse_confidence() {
        assert_eq!(Confidence::Done, parse_confidence(&json!("DONE")).unwrap());
        assert_eq!(Confidence::Low, parse_confidence(&json!("Low")).unwrap());
        assert_eq!(Confidence::Low, parse_confidence(&json!("low")).unwrap());
        assert_eq!(Confidence::None, parse_confidence(&json!("none")).unwrap());
        assert_eq!(Confidence::Ok, parse_confidence(&json!("OK")).unwrap());
        assert_eq!(Confidence::Ok, parse_confidence(&json!("ok")).unwrap());
    }

    #[test]
    fn test_parse_remaining_work() {
        let mut vec: Vec<Vec<Value>> = vec![];
        for val in &read_original_table_sheet().values[2..] {
            if val.len() < 5 {
                continue;
            }
            if let Err(_) = parse_remaining_work(&val[4]) {
                vec.push(val.to_owned());
            }
        }
        assert_eq!(vec.len(), 0);

        assert_eq!(
            Some(RemainingWork::Confirm),
            parse_remaining_work(&json!("confirm")).unwrap()
        );
        assert_eq!(
            Some(RemainingWork::UnclearHypothesis),
            parse_remaining_work(&json!("unclear hypothesis")).unwrap()
        );
        assert_eq!(
            Some(RemainingWork::NoHypothesis),
            parse_remaining_work(&json!("no hypothesis")).unwrap()
        );
        assert_eq!(
            Some(RemainingWork::StoryOnly),
            parse_remaining_work(&json!("story only")).unwrap()
        );
        assert_eq!(None, parse_remaining_work(&json!("n/a")).unwrap());
        assert_eq!(
            Some(RemainingWork::LegacyTag),
            parse_remaining_work(&json!("legacy tag")).unwrap()
        );

        assert_eq!(
            Some(RemainingWork::OpenEnded),
            parse_remaining_work(&json!("open ended")).unwrap()
        );

        assert_eq!(None, parse_remaining_work(&json!("")).unwrap());
    }
}
