mod comments;
pub mod error;

use divi::{sample::fix_name, IsCard};
use serde::{Deserialize, Serialize};

use error::Error;
use googlesheets::sheet::ValueRange;
use reqwest::Client;
use serde_json::Value;

pub fn deserialize_table(table: ValueRange) {
    for row in &table.values {
        let len = row.len();

        if len == 0 {
            continue;
        }

        let Some(name) = row.get(1) else {
            continue;
        };
        let name = name.to_string();
        if !name.as_str().is_card() {
            continue;
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

pub async fn download_areas_sheet() -> Result<ValueRange, Error> {
    dotenv::dotenv().ok();
    let api_key = std::env::var("GOOGLE_API_KEY").expect("No google api key");

    let url = format!("https://sheets.googleapis.com/v4/spreadsheets/1Pf2KNuGguZLyf6eu_R0E503U0QNyfMZqaRETsN5g6kU/values/Cards_and_Hypotheses?key={api_key}");
    let value_range: ValueRange = Client::new().get(url).send().await?.json().await?;
    Ok(value_range)
}

pub async fn update_areas_sheet() -> Result<(), Error> {
    let sheet = download_areas_sheet().await?;
    std::fs::write("areas.json", serde_json::to_string_pretty(&sheet)?)?;
    Ok(())
}

pub fn read_areas_file() -> ValueRange {
    let value_range: ValueRange =
        serde_json::from_str(&std::fs::read_to_string("areas.json").unwrap()).unwrap();
    value_range
}

#[tokio::main]
async fn main() {
    // let mut vec: Vec<Vec<Value>> = vec![];
    // let mut i = 0;
    // for val in &read_areas_file().values[2..] {
    //     if val.len() == 5 {
    //         dbg!(val);
    //         continue;
    //     }

    //     let second_column_contents = val[1].as_str().unwrap();
    //     let name = match second_column_contents.is_card() {
    //         true => second_column_contents.to_string(),
    //         false => match fix_name(second_column_contents) {
    //             Some(s) => s,
    //             None => panic!("Could not parse name {second_column_contents}"),
    //         },
    //     };
    // }

    // let g = serde_json::to_string(&GreyNote::Delirium).unwrap();

    // write("short.json", serde_json::to_string(&vec).unwrap()).unwrap();
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
    let conf: Confidence = serde_json::from_str(&val.to_string())?;
    Ok(conf)
}

// pub fn parse_greynote(val: &Value) -> Result<GreyNote, Error> {
//     let conf: GreyNote = serde_json::from_str(&val.to_string())?;
//     Ok(conf)
// }

pub fn parse_greynote(val: &Value) -> Result<Option<GreyNote>, Error> {
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

pub fn parse_remaining_work(val: &Value) -> Result<Option<RemainingWork>, Error> {
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

pub fn parse_notes(row: &[Value]) -> Result<String, Error> {
    if row.len() < 9 {
        return Err(Error::RowIsTooShort("Notes".to_string(), 9));
    };

    Ok(row[8].to_string())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::parse_greynote;

    use super::*;

    #[test]
    fn test_parse_notes() {
        let mut vec: Vec<String> = vec![];
        for row in &read_areas_file().values {
            if let Ok(notes) = parse_notes(&row) {
                vec.push(notes);
            }
        }

        std::fs::write("notes.json", serde_json::to_string_pretty(&vec).unwrap()).unwrap();
    }

    #[test]
    fn test_parse_remaining_work() {
        let mut vec: Vec<Vec<Value>> = vec![];
        for val in &read_areas_file().values[2..] {
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

    #[test]
    fn test_parse_name() {
        let mut vec: Vec<Vec<Value>> = vec![];
        for val in &read_areas_file().values[2..] {
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
    fn test_parse_greynote() {
        let mut vec: Vec<Vec<Value>> = vec![];
        for val in &read_areas_file().values {
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
}

// #[test]
// fn test_parse_table() {
//     let vr = read_areas_file();
//     for row in &vr.values {
//         parse_row(&row);
//     }
// }

#[test]
fn write_hypothesis_tags() {
    let mut tags: Vec<&str> = vec![];
    let vr = read_areas_file();
    for row in &vr.values[2..] {
        if row.len() < 3 {
            continue;
        }

        let Some(s) = row[2].as_str() else {
            continue;
        };

        if s.is_empty() {
            continue;
        }

        tags.push(s);

        println!("{}", s);
    }

    let s = serde_json::to_string(&tags).unwrap();
    std::fs::write("tags.json", s).unwrap();
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum StringCell {
    String(String),
    Empty,
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

#[test]
fn test_parse_table() {
    let vr = read_areas_file();
    let table = parse_table(&vr.values[2..]).unwrap();
    let json = serde_json::to_string_pretty(&table).unwrap();
    std::fs::write("parsed-table.json", &json).unwrap();

    let drops_from: Vec<Option<String>> = table
        .iter()
        .map(|record| record.drops_from.to_owned())
        .collect();

    std::fs::write(
        "drops-from.json",
        serde_json::to_string(&drops_from).unwrap(),
    )
    .unwrap();

    // let a = parse_table(&vr.values[2..]).unwrap();
}

pub fn parse_table(values: &[Vec<Value>]) -> Result<Vec<CardDropRecord>, Error> {
    let mut records: Vec<CardDropRecord> = Vec::new();
    for row in values {
        match parse_row(row) {
            Ok(record) => records.push(record),
            Err(err) => {
                println!("{err}");
            }
        }
    }

    Ok(records)
}

pub fn parse_string_cell(val: &Value) -> Option<String> {
    let Some(s) = val.as_str() else { return None };
    if s.is_empty() || s == "n/a" {
        return None;
    } else {
        return Some(s.to_string());
    }
}
