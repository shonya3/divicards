use divi::{sample::fix_name, IsCard};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    dropsource::{parse::ParseSourceError, parse_source, Source},
    error::Error,
};
use poe_data::PoeData;

use super::rich::DropsFrom;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DivcordTableRecord {
    pub id: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub greynote: Option<GreyNote>,
    pub card: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_hypothesis: Option<String>,
    pub confidence: Confidence,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_work: Option<RemainingWork>,
    #[serde(skip_serializing)]
    pub drops_from: Vec<DropsFrom>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wiki_disagreements: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources_with_tag_but_not_on_wiki: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl DivcordTableRecord {
    pub fn create(
        row_index: usize,
        divcord_table_row: &[Value],
        drops_from: Vec<DropsFrom>,
    ) -> Result<Self, Error> {
        let greynote = GreyNote::parse(&divcord_table_row[0])?;
        let card = parse_card_name(&divcord_table_row[1])?;
        let tag_hypothesis = parse_string_cell(&divcord_table_row[2]);
        let confidence = Confidence::parse(&divcord_table_row[3])?;
        let remaining_work = RemainingWork::parse(&divcord_table_row[4])?;
        let wiki_disagreements = divcord_table_row
            .get(6)
            .map(|val| parse_string_cell(val))
            .flatten();
        let sources_with_tag_but_not_on_wiki = divcord_table_row
            .get(7)
            .map(|val| parse_string_cell(val))
            .flatten();
        let notes = divcord_table_row
            .get(8)
            .map(|val| parse_string_cell(val))
            .flatten();

        Ok(DivcordTableRecord {
            greynote,
            card,
            tag_hypothesis,
            confidence,
            remaining_work,
            drops_from,
            wiki_disagreements,
            sources_with_tag_but_not_on_wiki,
            notes,
            id: row_index + 3,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SourcefulDivcordTableRecord {
    pub id: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub greynote: Option<GreyNote>,
    pub card: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_hypothesis: Option<String>,
    pub confidence: Confidence,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_work: Option<RemainingWork>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<Source>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wiki_disagreements: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources_with_tag_but_not_on_wiki: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl SourcefulDivcordTableRecord {
    pub fn from_record(
        record: DivcordTableRecord,
        poe_data: &PoeData,
    ) -> Result<Self, ParseSourceError> {
        let mut sources: Vec<Source> = Vec::new();
        for d in &record.drops_from {
            let Ok(inner_sources) = parse_source(d, &record, poe_data) else {
                return Err(ParseSourceError(d.to_owned()));
            };
            for s in inner_sources {
                sources.push(s)
            }
        }

        Ok(SourcefulDivcordTableRecord {
            sources,
            id: record.id,
            greynote: record.greynote,
            card: record.card,
            tag_hypothesis: record.tag_hypothesis,
            confidence: record.confidence,
            remaining_work: record.remaining_work,
            wiki_disagreements: record.wiki_disagreements,
            sources_with_tag_but_not_on_wiki: record.sources_with_tag_but_not_on_wiki,
            notes: record.notes,
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

    use super::*;

    #[test]
    fn test_parse_greynote() {
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
