//! Structs for pre-parsed and parsed spreadsheet's row

use super::rich::DropsFrom;
use crate::{dropsource::Source, error::Error};
use divi::{cards::fix_name, IsCard};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// "Sourceful" record, with figured out dropsources and need-to-verify-sources.
#[derive(Debug, Serialize, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    pub id: usize,
    #[serde(default)]
    pub greynote: GreyNote,
    pub card: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_hypothesis: Option<String>,
    pub confidence: Confidence,
    #[serde(default)]
    pub remaining_work: RemainingWork,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<Source>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wiki_disagreements: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources_with_tag_but_not_on_wiki: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    pub verify_sources: Vec<Source>,
}

/// Represents spreadsheet's row after the initial preparation.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Dumb {
    pub id: usize,
    #[serde(default)]
    pub greynote: GreyNote,
    pub card: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_hypothesis: Option<String>,
    pub confidence: Confidence,
    #[serde(default)]
    pub remaining_work: RemainingWork,
    #[serde(skip_serializing)]
    pub sources_drops_from: Vec<DropsFrom>,
    #[serde(skip_serializing)]
    pub verify_drops_from: Vec<DropsFrom>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wiki_disagreements: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources_with_tag_but_not_on_wiki: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl Dumb {
    pub fn create(
        row_index: usize,
        spreadsheet_row: &[Value],
        sources_drops_from: Vec<DropsFrom>,
        verify_drops_from: Vec<DropsFrom>,
    ) -> Result<Self, Error> {
        let greynote = GreyNote::parse(&spreadsheet_row[0])?;
        let card = parse_card_name(&spreadsheet_row[1])?;
        let tag_hypothesis = parse_string_cell(&spreadsheet_row[2]);
        let confidence = Confidence::parse(&spreadsheet_row[3])?;
        let remaining_work = RemainingWork::parse(&spreadsheet_row[4])?;
        let wiki_disagreements = spreadsheet_row
            .get(6)
            .map(|val| parse_string_cell(val))
            .flatten();
        let sources_with_tag_but_not_on_wiki = spreadsheet_row
            .get(7)
            .map(|val| parse_string_cell(val))
            .flatten();
        let notes = spreadsheet_row
            .get(8)
            .map(|val| parse_string_cell(val))
            .flatten();

        Ok(Self {
            greynote,
            card,
            tag_hypothesis,
            confidence,
            remaining_work,
            sources_drops_from,
            verify_drops_from,
            wiki_disagreements,
            sources_with_tag_but_not_on_wiki,
            notes,
            id: row_index + 3,
        })
    }
}

pub fn parse_card_name(val: &Value) -> Result<String, Error> {
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
    #[strum(to_string = "Empty")]
    #[serde(rename = "Empty")]
    Empty,
    #[strum(to_string = "Monster-specific")]
    #[serde(rename = "Monster-specific")]
    MonsterSpecific,
    #[strum(to_string = "Area-specific")]
    #[serde(rename = "Area-specific")]
    AreaSpecific,
    #[strum(to_string = "disabled", serialize = "Drop disabled")]
    #[serde(rename = "disabled", alias = "Drop disabled")]
    Disabled,
    #[strum(to_string = "story")]
    #[serde(rename = "story")]
    Story,
    #[strum(to_string = "Delirium_reward")]
    #[serde(rename = "Delirium_reward")]
    Delirium,
    #[strum(to_string = "Chest_object", serialize = "Chest_obkect")]
    #[serde(rename = "Chest_object", alias = "Chest_obkect")]
    ChestObject,
    #[strum(to_string = "strongbox")]
    #[serde(rename = "strongbox")]
    Strongbox,
    #[strum(to_string = "Global Drop")]
    #[serde(rename = "Global Drop")]
    GlobalDrop,
    #[strum(to_string = "Vendor")]
    #[serde(rename = "Vendor")]
    Vendor,
}

impl Default for GreyNote {
    fn default() -> Self {
        GreyNote::Empty
    }
}

impl GreyNote {
    pub fn parse(val: &Value) -> Result<Self, Error> {
        match val.as_str() {
            Some(s) if s.is_empty() || s == "n/a" => Ok(Self::Empty),
            Some(s) => Ok(s.parse()?),
            None => Ok(Self::Empty),
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
    #[serde(alias = "None", alias = "none")]
    None,
    #[strum(to_string = "Low", serialize = "low")]
    #[serde(alias = "Low", alias = "low")]
    Low,
    #[strum(to_string = "OK", serialize = "ok")]
    #[serde(alias = "OK", alias = "ok")]
    Ok,
    #[strum(to_string = "Done", serialize = "DONE")]
    #[serde(alias = "Done", alias = "DONE")]
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
    #[strum(serialize = "n/a")]
    #[serde(rename = "n/a")]
    NotApplicable,
    #[strum(serialize = "confirm")]
    #[serde(alias = "confirm")]
    Confirm,
    #[strum(serialize = "unclear hypothesis")]
    #[serde(rename = "unclear hypothesis")]
    UnclearHypothesis,
    #[strum(serialize = "no hypothesis")]
    #[serde(rename = "no hypothesis")]
    NoHypothesis,
    #[strum(serialize = "story only")]
    #[serde(rename = "story only")]
    StoryOnly,
    #[strum(serialize = "legacy tag")]
    #[serde(rename = "legacy tag")]
    LegacyTag,
    #[strum(serialize = "open ended")]
    #[serde(rename = "open ended")]
    OpenEnded,
}

impl Default for RemainingWork {
    fn default() -> Self {
        RemainingWork::NotApplicable
    }
}

impl RemainingWork {
    pub fn parse(val: &Value) -> Result<Self, Error> {
        match val.as_str() {
            Some(s) if s.is_empty() || s == "n/a" => Ok(Self::NotApplicable),
            Some(s) => Ok(s.parse()?),
            None => Ok(Self::NotApplicable),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_parse_greynote() {
        assert_eq!(
            GreyNote::AreaSpecific,
            GreyNote::parse(&json!("Area-specific")).unwrap()
        );
        assert_eq!(
            GreyNote::MonsterSpecific,
            GreyNote::parse(&json!("Monster-specific")).unwrap()
        );
        assert_eq!(
            GreyNote::Disabled,
            GreyNote::parse(&json!("disabled")).unwrap()
        );
        assert_eq!(
            GreyNote::Disabled,
            GreyNote::parse(&json!("disabled")).unwrap()
        );
        assert_eq!(
            GreyNote::Disabled,
            GreyNote::parse(&json!("Drop disabled")).unwrap()
        );
        assert_eq!(GreyNote::Story, GreyNote::parse(&json!("story")).unwrap());
        assert_eq!(
            GreyNote::Delirium,
            GreyNote::parse(&json!("Delirium_reward")).unwrap()
        );
        assert_eq!(
            GreyNote::ChestObject,
            GreyNote::parse(&json!("Chest_object")).unwrap()
        );
        assert_eq!(
            GreyNote::ChestObject,
            GreyNote::parse(&json!("Chest_obkect")).unwrap()
        );
        assert_eq!(
            GreyNote::Strongbox,
            GreyNote::parse(&json!("strongbox")).unwrap()
        );
        assert_eq!(
            GreyNote::GlobalDrop,
            GreyNote::parse(&json!("Global Drop")).unwrap()
        );
        assert_eq!(GreyNote::Vendor, GreyNote::parse(&json!("Vendor")).unwrap());
        assert_eq!(GreyNote::Empty, GreyNote::parse(&json!("")).unwrap());
        assert_eq!(GreyNote::Empty, GreyNote::parse(&json!("n/a")).unwrap());
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
        assert_eq!(
            RemainingWork::Confirm,
            RemainingWork::parse(&json!("confirm")).unwrap()
        );
        assert_eq!(
            RemainingWork::UnclearHypothesis,
            RemainingWork::parse(&json!("unclear hypothesis")).unwrap()
        );
        assert_eq!(
            RemainingWork::NoHypothesis,
            RemainingWork::parse(&json!("no hypothesis")).unwrap()
        );
        assert_eq!(
            RemainingWork::StoryOnly,
            RemainingWork::parse(&json!("story only")).unwrap()
        );
        assert_eq!(
            RemainingWork::NotApplicable,
            RemainingWork::parse(&json!("n/a")).unwrap()
        );
        assert_eq!(
            RemainingWork::LegacyTag,
            RemainingWork::parse(&json!("legacy tag")).unwrap()
        );

        assert_eq!(
            RemainingWork::OpenEnded,
            RemainingWork::parse(&json!("open ended")).unwrap()
        );

        assert_eq!(
            RemainingWork::NotApplicable,
            RemainingWork::parse(&json!("")).unwrap()
        );
    }
}
