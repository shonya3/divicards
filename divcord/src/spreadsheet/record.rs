//! Structs for pre-parsed and parsed spreadsheet's row

use std::fmt::Display;

use super::rich::{Cell, DropsFrom, ParseCellError};
use crate::dropsource::Source;
use divi::cards::CheckCardName;
use serde::{Deserialize, Serialize};
use serde_json::{Error as SerdeJsonError, Value};

/// "Sourceful" spreadsheet row, with figured out dropsources and need-to-verify-sources.
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

/// Represents spreadsheet's row after the initial preparation (but before the main parsing).
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Dumb {
    pub id: usize,

    /// A
    #[serde(default)]
    pub greynote: GreyNote,

    /// B
    pub card: String,

    /// C
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_hypothesis: Option<String>,

    /// D
    pub confidence: Confidence,

    /// F
    #[serde(default)]
    pub remaining_work: RemainingWork,

    /// G
    pub drops: Vec<DropsFrom>,

    /// H
    pub drops_to_verify: Vec<DropsFrom>,

    /// I
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug)]
pub enum ParseDumbErrKind {
    Greynote(SerdeJsonError),
    CardName(ParseCardNameError),
    Confidence(SerdeJsonError),
    RemainingWork(SerdeJsonError),
    StyledCell(ParseCellError),
}

impl Display for ParseDumbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ParseDumbError {
            record_id,
            card,
            kind,
        } = self;

        match kind {
            ParseDumbErrKind::Greynote(error) => {
                write!(f, "{record_id} {card}. Parse greynote error. {error}")
            }
            ParseDumbErrKind::CardName(parse_card_name_error) => write!(
                f,
                "{record_id}. Parse card name error. {parse_card_name_error}"
            ),
            ParseDumbErrKind::Confidence(error) => {
                write!(f, "{record_id} {card}. Parse confidence error. {error}")
            }
            ParseDumbErrKind::RemainingWork(error) => {
                write!(f, "{record_id} {card}. Parse remaining work error. {error}")
            }
            ParseDumbErrKind::StyledCell(parse_cell_error) => write!(
                f,
                "{record_id} {card} Could not parse styled cell into chunks. {parse_cell_error}"
            ),
        }
    }
}

#[derive(Debug)]
pub struct ParseDumbError {
    pub record_id: usize,
    pub card: String,
    pub kind: ParseDumbErrKind,
}

impl ParseDumbError {
    pub fn new(record_id: usize, card: String, kind: ParseDumbErrKind) -> ParseDumbError {
        ParseDumbError {
            record_id,
            card,
            kind,
        }
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
    ) -> Result<Self, ParseDumbError> {
        let record_id = Dumb::record_id(row_index);
        // B 1 Card name
        let card = parse_card_name(&spreadsheet_row[1]).map_err(|parse_card_name_error| {
            let card_name = match parse_card_name_error {
                ParseCardNameError::CellValueIsNotStr(_) => {
                    "Invalid card name cell value".to_owned()
                }
                ParseCardNameError::CardNameNotExists(ref name) => name.clone(),
            };

            ParseDumbError::new(
                record_id,
                card_name,
                ParseDumbErrKind::CardName(parse_card_name_error),
            )
        })?;

        // A 0 Greynote
        let greynote: GreyNote =
            serde_json::from_value(spreadsheet_row[0].clone()).map_err(|parse_greynote_error| {
                ParseDumbError::new(
                    record_id,
                    card.clone(),
                    ParseDumbErrKind::Greynote(parse_greynote_error),
                )
            })?;

        // C 2 Tag hypothesis
        let tag_hypothesis = parse_string_cell(&spreadsheet_row[2]);

        // D 3 3.25 Confidence
        let confidence: Confidence = serde_json::from_value(spreadsheet_row[3].clone()).map_err(
            |parse_confidence_error| {
                ParseDumbError::new(
                    record_id,
                    card.clone(),
                    ParseDumbErrKind::Confidence(parse_confidence_error),
                )
            },
        )?;

        // E 4 Old Confidence SKIP

        // F 5 Remaining work
        let remaining_work: RemainingWork = serde_json::from_value(spreadsheet_row[5].clone())
            .map_err(|parse_remaining_error| {
                ParseDumbError::new(
                    record_id,
                    card.clone(),
                    ParseDumbErrKind::RemainingWork(parse_remaining_error),
                )
            })?;

        // G 6 - New confirmations - drops
        let drops = confirmations_new_325_cell
            .drops_from()
            .map_err(|parse_styled_cell_error| {
                ParseDumbError::new(
                    record_id,
                    card.clone(),
                    ParseDumbErrKind::StyledCell(parse_styled_cell_error),
                )
            })?;

        // H 7 - To Confirm or Verify
        let drops_to_verify =
            to_confirm_or_verify_cell
                .drops_from()
                .map_err(|parse_styled_cell_error| {
                    ParseDumbError::new(
                        record_id,
                        card.clone(),
                        ParseDumbErrKind::StyledCell(parse_styled_cell_error),
                    )
                })?;

        // I 8 - Notes
        let notes = spreadsheet_row.get(8).and_then(parse_string_cell);
        // J 9 - Old Sources - SKIP
        // K 10 - Old Wiki Disagreements - SKIP
        // L 11 - Old Need to verify - SKIP

        Ok(Dumb {
            id: record_id,
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

#[derive(Debug)]
pub enum ParseCardNameError {
    CellValueIsNotStr(Value),
    CardNameNotExists(String),
}

impl Display for ParseCardNameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseCardNameError::CellValueIsNotStr(value) => {
                write!(f, "Card name cell value is not a string. {value:#?}")
            }
            ParseCardNameError::CardNameNotExists(card) => write!(f, "Card {card} does not exist."),
        }
    }
}

pub fn parse_card_name(val: &Value) -> Result<String, ParseCardNameError> {
    let Some(second_column_contents) = val.as_str() else {
        return Err(ParseCardNameError::CellValueIsNotStr(val.to_owned()));
    };

    match divi::cards::check_card_name(second_column_contents) {
        CheckCardName::Valid => Ok(second_column_contents.to_owned()),
        CheckCardName::TypoFixed(fixed) => Ok(fixed.fixed),
        CheckCardName::NotACard => Err(ParseCardNameError::CardNameNotExists(
            second_column_contents.to_owned(),
        )),
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
    #[strum(to_string = "Empty", serialize = "", serialize = "n/a")]
    #[serde(rename = "Empty", alias = "", alias = "n/a")]
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
