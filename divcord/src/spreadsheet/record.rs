//! Structs for pre-parsed and parsed spreadsheet's row

use super::rich::{Cell, DropsFrom, ParseCellError};
use crate::{dropsource::Source, error::Error};
use divi::cards::CheckCardName;
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
    pub sources: Vec<Source>,
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
    // A
    pub greynote: GreyNote,
    // B
    pub card: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    // C
    pub tag_hypothesis: Option<String>,
    pub confidence: Confidence, // D
    #[serde(default)]
    // F
    pub remaining_work: RemainingWork,
    // G
    pub drops: Vec<DropsFrom>,
    // H
    pub drops_to_verify: Vec<DropsFrom>,
    // I
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug)]
pub struct ParseDumbError {
    pub record_id: usize,
    pub parse_cell_error: ParseCellError,
}

impl std::fmt::Display for ParseDumbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}. {}", &self.record_id, &self.parse_cell_error)
    }
}

impl Dumb {
    pub fn record_id(row_index: usize) -> usize {
        row_index + 3
    }

    pub fn create(
        row_index: usize,
        spreadsheet_row: &[Value],
        confirmations_new_325_cell: &Cell,
        to_confirm_or_verify_cell: &Cell,
    ) -> Result<Self, Error> {
        // A 0 Greynote
        // let greynote = GreyNote::parse(&spreadsheet_row[0]);
        let Ok(greynote) = GreyNote::parse(&spreadsheet_row[0]) else {
            panic!("Could not parse greynote: {}", spreadsheet_row[0]);
        };

        // B 1 Card name
        let card = parse_card_name(&spreadsheet_row[1]).expect("Card error");

        // C 2 Tag hypothesis
        let tag_hypothesis = parse_string_cell(&spreadsheet_row[2]);

        // D 3 3.25 Confidence
        let confidence = Confidence::parse(&spreadsheet_row[3]).unwrap();

        // E 4 Old Confidence SKIP

        // F 5 Remaining work
        let remaining_work = RemainingWork::parse(&spreadsheet_row[5])?;

        // G 6 - New confirmations - drops
        let drops = confirmations_new_325_cell
            .drops_from()
            .map_err(|err| ParseDumbError {
                record_id: Dumb::record_id(row_index),
                parse_cell_error: err,
            })?;

        // UPDATE
        // H 7 - To Confirm or Verify
        let drops_to_verify =
            to_confirm_or_verify_cell
                .drops_from()
                .map_err(|err| ParseDumbError {
                    record_id: Dumb::record_id(row_index),
                    parse_cell_error: err,
                })?;

        // I 8 - Notes
        let notes = spreadsheet_row.get(8).and_then(parse_string_cell);
        // J 9 - Old Sources - SKIP
        // K 10 - Old Wiki Disagreements - SKIP
        // L 11 - Old Need to verify - SKIP

        // H 7 - Sources           - sources_drops_from

        // K 10 - Notes

        Ok(Self {
            id: Dumb::record_id(row_index),
            greynote,
            card,
            tag_hypothesis,
            confidence,
            remaining_work,
            drops,
            drops_to_verify,
            notes,
        })
    }
}

pub fn parse_card_name(val: &Value) -> Result<String, Error> {
    let Some(second_column_contents) = val.as_str() else {
        return Err(Error::ValueNotStr(val.to_owned()));
    };

    match divi::cards::check_card_name(second_column_contents) {
        CheckCardName::Valid => Ok(second_column_contents.to_owned()),
        CheckCardName::TypoFixed(fixed) => Ok(fixed.fixed),
        CheckCardName::NotACard => {
            Err(Error::ParseCardNameError(second_column_contents.to_owned()))
        }
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
    Serialize,
    Deserialize,
    Clone,
    Default,
    Debug,
    PartialEq,
    strum_macros::EnumString,
    strum_macros::Display,
)]
#[serde(rename_all = "camelCase")]
pub enum GreyNote {
    #[default]
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
    #[strum(to_string = "atlas")]
    #[serde(rename = "atlas")]
    Atlas,
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
    Serialize,
    Deserialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    strum_macros::EnumString,
    strum_macros::Display,
)]
#[serde(rename_all = "camelCase")]
pub enum RemainingWork {
    #[default]
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
    #[strum(serialize = "reverify")]
    #[serde(rename = "reverify")]
    Reverify,
    #[strum(serialize = "story")]
    #[serde(rename = "story")]
    Story,
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
