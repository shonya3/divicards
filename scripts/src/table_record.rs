use divi::{sample::fix_name, IsCard};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CardDropTableRecord {
    pub greynote: Option<GreyNote>,
    pub name: String,
    pub tag_hypothesis: Option<String>,
    pub confidence: Confidence,
    pub remaining_work: Option<RemainingWork>,
    pub drops_from: Option<String>,
    pub wiki_disagreements: Option<String>,
    pub sources_with_tag_but_not_on_wiki: Option<String>,
    pub notes: Option<String>,
}

impl CardDropTableRecord {
    pub fn parse(row: &[Value]) -> Result<CardDropTableRecord, Error> {
        let greynote = GreyNote::parse(&row[0])?;
        let name = parse_name(&row[1])?;
        let tag_hypothesis = parse_string_cell(&row[2]);
        let confidence = Confidence::parse(&row[3])?;
        let remaining_work = RemainingWork::parse(&row[4])?;
        let drops_from = row.get(5).map(|val| parse_string_cell(val)).flatten();
        let wiki_disagreements = row.get(6).map(|val| parse_string_cell(val)).flatten();
        let sources_with_tag_but_not_on_wiki =
            row.get(7).map(|val| parse_string_cell(val)).flatten();
        let notes = row.get(8).map(|val| parse_string_cell(val)).flatten();

        Ok(CardDropTableRecord {
            greynote,
            name,
            tag_hypothesis,
            confidence,
            remaining_work,
            drops_from,
            wiki_disagreements,
            sources_with_tag_but_not_on_wiki,
            notes,
        })
    }
}

pub fn parse_name(val: &Value) -> Result<String, Error> {
    let Some(second_column_contents) = val.as_str() else {
        return Err(Error::ValueNotStr(val.to_owned()));
    };

    match second_column_contents.is_card() {
        true => Ok(second_column_contents.to_string()),
        false => match fix_name(second_column_contents) {
            Some(s) => Ok(s),
            None => Err(Error::ParseCardNameError(
                second_column_contents.to_string(),
            )),
        },
    }
}

pub fn parse_string_cell(val: &Value) -> Option<String> {
    match val.as_str() {
        Some(s) if s.is_empty() || s == "n/a" => None,
        Some(s) => Some(s.to_string()),
        None => None,
    }
}

#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, strum_macros::EnumString, strum_macros::Display,
)]
#[serde(rename_all = "camelCase")]
pub enum GreyNote {
    #[strum(serialize = "Monster-specific")]
    #[serde(alias = "Monster-specific")]
    MonsterSpecific,
    #[strum(serialize = "Area-specific")]
    #[serde(alias = "Area-specific")]
    AreaSpecific,
    #[strum(serialize = "disabled", serialize = "Drop disabled")]
    #[serde(alias = "disabled", alias = "Drop disabled")]
    Disabled,
    #[strum(serialize = "story")]
    #[serde(alias = "story")]
    Story,
    #[strum(serialize = "Delirium_reward")]
    #[serde(alias = "Delirium_reward")]
    Delirium,
    #[strum(to_string = "Chest_object", serialize = "Chest_obkect")]
    #[serde(alias = "Chest_object", alias = "Chest_obkect")]
    ChestObject,
    #[strum(serialize = "strongbox")]
    #[serde(alias = "strongbox")]
    Strongbox,
    #[strum(serialize = "Global Drop")]
    #[serde(alias = "Global Drop")]
    GlobalDrop,
    #[strum(serialize = "Vendor")]
    #[serde(alias = "Vendor")]
    Vendor,
}

impl GreyNote {
    pub fn parse(val: &Value) -> Result<Option<Self>, Error> {
        match val.as_str() {
            Some(s) if s.is_empty() || s == "n/a" => Ok(None),
            Some(s) => Ok(Some(s.parse()?)),
            None => Ok(None),
        }
    }
}

#[derive(
    Serialize,
    Deserialize,
    Clone,
    Debug,
    PartialEq,
    Hash,
    Eq,
    strum_macros::EnumString,
    strum_macros::Display,
)]
#[serde(rename_all = "camelCase")]
pub enum Confidence {
    #[strum(to_string = "None", serialize = "none")]
    #[serde(rename = "None", alias = "none")]
    None,
    #[strum(to_string = "Low", serialize = "low")]
    #[serde(rename = "Low", alias = "low")]
    Low,
    #[strum(to_string = "OK", serialize = "ok")]
    #[serde(rename = "OK", alias = "ok")]
    Ok,
    #[strum(to_string = "Done", serialize = "DONE")]
    #[serde(rename = "Done", alias = "DONE")]
    Done,
}

impl Confidence {
    pub fn parse(val: &Value) -> Result<Self, Error> {
        let conf: Confidence = serde_json::from_value(val.to_owned())?;
        Ok(conf)
    }
}

#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, strum_macros::EnumString, strum_macros::Display,
)]
#[serde(rename_all = "camelCase")]
pub enum RemainingWork {
    #[strum(serialize = "confirm")]
    #[serde(alias = "confirm")]
    Confirm,
    #[strum(serialize = "unclear hypothesis")]
    #[serde(alias = "unclear hypothesis")]
    UnclearHypothesis,
    #[strum(serialize = "no hypothesis")]
    #[serde(alias = "no hypothesis")]
    NoHypothesis,
    #[strum(serialize = "story only")]
    #[serde(alias = "story only")]
    StoryOnly,
    #[strum(serialize = "legacy tag")]
    #[serde(alias = "legacy tag")]
    LegacyTag,
    #[strum(serialize = "open ended")]
    #[serde(alias = "open ended")]
    OpenEnded,
}

impl RemainingWork {
    pub fn parse(val: &Value) -> Result<Option<Self>, Error> {
        match val.as_str() {
            Some(s) if s.is_empty() || s == "n/a" => Ok(None),
            Some(s) => Ok(Some(s.parse()?)),
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::scripts::read_original_table_sheet;

    use super::*;

    #[test]
    fn parses_table_without_errors() {
        let sheet = read_original_table_sheet("jsons/sheet.json").unwrap();
        for row in &sheet.values[2..] {
            CardDropTableRecord::parse(row).unwrap();
        }
    }

    #[test]
    fn test_parse_greynote() {
        let sheet = read_original_table_sheet("jsons/sheet.json").unwrap();
        let mut vec: Vec<Vec<Value>> = vec![];
        for val in &sheet.values {
            if let Err(_) = GreyNote::parse(&val[0]) {
                vec.push(val.to_owned());
                dbg!(val);
            }
        }
        assert_eq!(vec.len(), 0);

        assert_eq!(
            Some(GreyNote::AreaSpecific),
            GreyNote::parse(&json!("Area-specific")).unwrap()
        );
        assert_eq!(
            Some(GreyNote::MonsterSpecific),
            GreyNote::parse(&json!("Monster-specific")).unwrap()
        );
        assert_eq!(
            Some(GreyNote::Disabled),
            GreyNote::parse(&json!("disabled")).unwrap()
        );
        assert_eq!(
            Some(GreyNote::Disabled),
            GreyNote::parse(&json!("disabled")).unwrap()
        );
        assert_eq!(
            Some(GreyNote::Disabled),
            GreyNote::parse(&json!("Drop disabled")).unwrap()
        );
        assert_eq!(
            Some(GreyNote::Story),
            GreyNote::parse(&json!("story")).unwrap()
        );
        assert_eq!(
            Some(GreyNote::Delirium),
            GreyNote::parse(&json!("Delirium_reward")).unwrap()
        );
        assert_eq!(
            Some(GreyNote::ChestObject),
            GreyNote::parse(&json!("Chest_object")).unwrap()
        );
        assert_eq!(
            Some(GreyNote::ChestObject),
            GreyNote::parse(&json!("Chest_obkect")).unwrap()
        );
        assert_eq!(
            Some(GreyNote::Strongbox),
            GreyNote::parse(&json!("strongbox")).unwrap()
        );
        assert_eq!(
            Some(GreyNote::GlobalDrop),
            GreyNote::parse(&json!("Global Drop")).unwrap()
        );
        assert_eq!(
            Some(GreyNote::Vendor),
            GreyNote::parse(&json!("Vendor")).unwrap()
        );
        assert_eq!(None, GreyNote::parse(&json!("")).unwrap());
        assert_eq!(None, GreyNote::parse(&json!("n/a")).unwrap());
    }

    #[test]
    fn test_parse_name() {
        let sheet = read_original_table_sheet("jsons/sheet.json").unwrap();
        let mut vec: Vec<Vec<Value>> = vec![];
        for val in &sheet.values[2..] {
            if let Err(_) = super::parse_name(&val[1]) {
                vec.push(val.to_owned());
            }
        }

        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn test_parse_confidence() {
        assert_eq!(Confidence::Done, Confidence::parse(&json!("DONE")).unwrap());
        assert_eq!(Confidence::Low, Confidence::parse(&json!("Low")).unwrap());
        assert_eq!(Confidence::Low, Confidence::parse(&json!("low")).unwrap());
        assert_eq!(Confidence::None, Confidence::parse(&json!("none")).unwrap());
        assert_eq!(Confidence::Ok, Confidence::parse(&json!("OK")).unwrap());
        assert_eq!(Confidence::Ok, Confidence::parse(&json!("ok")).unwrap());
    }

    #[test]
    fn test_parse_remaining_work() {
        let sheet = read_original_table_sheet("jsons/sheet.json").unwrap();
        let mut vec: Vec<Vec<Value>> = vec![];
        for val in &sheet.values[2..] {
            if val.len() < 5 {
                continue;
            }
            if let Err(_) = RemainingWork::parse(&val[4]) {
                vec.push(val.to_owned());
            }
        }
        assert_eq!(vec.len(), 0);

        assert_eq!(
            Some(RemainingWork::Confirm),
            RemainingWork::parse(&json!("confirm")).unwrap()
        );
        assert_eq!(
            Some(RemainingWork::UnclearHypothesis),
            RemainingWork::parse(&json!("unclear hypothesis")).unwrap()
        );
        assert_eq!(
            Some(RemainingWork::NoHypothesis),
            RemainingWork::parse(&json!("no hypothesis")).unwrap()
        );
        assert_eq!(
            Some(RemainingWork::StoryOnly),
            RemainingWork::parse(&json!("story only")).unwrap()
        );
        assert_eq!(None, RemainingWork::parse(&json!("n/a")).unwrap());
        assert_eq!(
            Some(RemainingWork::LegacyTag),
            RemainingWork::parse(&json!("legacy tag")).unwrap()
        );

        assert_eq!(
            Some(RemainingWork::OpenEnded),
            RemainingWork::parse(&json!("open ended")).unwrap()
        );

        assert_eq!(None, RemainingWork::parse(&json!("")).unwrap());
    }
}
