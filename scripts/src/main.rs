pub mod cards;
pub mod consts;
pub mod error;
pub mod reward;
pub mod scripts;

#[allow(unused)]
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    slice::Iter,
};

use divi::{league::TradeLeague, prices::NinjaCardData, sample::fix_name, IsCard};
use reward::reward_to_html;
use serde::{Deserialize, Serialize};

use error::Error;
use serde_json::Value;

#[tokio::main]
async fn main() {}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DivinationCardElementData {
    pub name: String,
    pub art_filename: String,
    pub reward_html: String,
    pub flavour_text: String,
    pub stack_size: Option<usize>,
}

impl DivinationCardElementData {
    pub async fn write_data() {
        let vec: Vec<NinjaCardData> = NinjaCardData::fetch(&TradeLeague::default()).await.unwrap();
        let v: Vec<DivinationCardElementData> = vec
            .into_iter()
            .map(|data| {
                let mut fl = data.flavour_text;
                if fl.starts_with("<size:") {
                    fl = fl[10..fl.len() - 1].to_string();
                }

                if fl.starts_with("<smaller>{") {
                    fl = fl[10..fl.len() - 1].to_string();
                }

                DivinationCardElementData {
                    name: data.name,
                    art_filename: data.art_filename,
                    flavour_text: fl,
                    stack_size: data.stack_size,
                    reward_html: reward_to_html(&data.explicit_modifiers[0].text),
                }
            })
            .collect();

        std::fs::write("data.json", serde_json::to_string(&v).unwrap()).unwrap();
    }
}

#[allow(unused)]
use crate::scripts::{parse_table, read_original_table_sheet};

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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum DropSource {
    ExpeditionLogbook,
    GlobalDrop,
    ChestObject,
    Map(String),
    MapBoss { boss: String, map: String },
    Disabled,
    Unknown,
    Delirium,
    Vendor(Option<Vendor>),
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
    pub tag_hypothesis: Option<String>,
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
    let tag_hypothesis = parse_string_cell(&row[2]);
    let confidence = parse_confidence(&row[3])?;
    let remaining_work = parse_remaining_work(&row[4])?;
    let drops_from = row.get(5).map(|val| parse_string_cell(val)).flatten();
    let wiki_disagreements = row.get(6).map(|val| parse_string_cell(val)).flatten();
    let sources_with_tag_but_not_on_wiki = row.get(7).map(|val| parse_string_cell(val)).flatten();
    let notes = row.get(8).map(|val| parse_string_cell(val)).flatten();

    Ok(CardDropRecord {
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

pub fn parse_drop_source(record: &CardDropRecord) -> Result<Vec<DropSource>, Error> {
    let mut sources: Vec<DropSource> = Vec::new();

    if let Some(tag_hypothesis) = &record.tag_hypothesis {
        if tag_hypothesis.contains("logbook") {
            sources.push(DropSource::ExpeditionLogbook);
        }
    }

    if let Some(greynote) = &record.greynote {
        if greynote == &GreyNote::Disabled {
            sources.push(DropSource::Disabled);
        }

        if greynote == &GreyNote::Vendor {
            if let Some(_drops_from) = &record.drops_from {}
            // return Ok(DropSource::Vendor());
        }
    }

    // match greynote {
    //     GreyNote::Disabled => return Ok(DropSource::Disabled),
    //     GreyNote::Delirium => return Ok(DropSource::Delirium),
    //     GreyNote::ChestObject => return Ok(DropSource::ChestObject),
    //     GreyNote::GlobalDrop => return Ok(DropSource::GlobalDrop),
    //     GreyNote::Vendor => return Ok(DropSource::Vendor),
    //     GreyNote::Strongbox => return Ok(DropSource::Strongbox),
    //     GreyNote::AreaSpecific => todo!(),
    //     GreyNote::MonsterSpecific => todo!(),
    //     GreyNote::Story => todo!(),
    // }

    // let sources = sources.into_iter().unique();

    Ok(sources)
}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Clone, Debug)]
pub enum Vendor {
    #[serde(alias = "Kirac shop")]
    KiracShop,
}

impl Vendor {
    pub fn iter() -> Iter<'static, Vendor> {
        static VENDORS: [Vendor; 1] = [Vendor::KiracShop];
        VENDORS.iter()
    }
}

impl Display for Vendor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Vendor::KiracShop => write!(f, "Kirac shop"),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::parse_greynote;

    use super::*;

    #[test]
    fn parses_table_without_errors() {
        let sheet = read_original_table_sheet("sheet.json").unwrap();
        for row in &sheet.values[2..] {
            parse_row(row).unwrap();
        }
    }

    #[test]
    fn test_parse_greynote() {
        let sheet = read_original_table_sheet("sheet.json").unwrap();
        let mut vec: Vec<Vec<Value>> = vec![];
        for val in &sheet.values {
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
        let sheet = read_original_table_sheet("sheet.json").unwrap();
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
        assert_eq!(Confidence::Done, parse_confidence(&json!("DONE")).unwrap());
        assert_eq!(Confidence::Low, parse_confidence(&json!("Low")).unwrap());
        assert_eq!(Confidence::Low, parse_confidence(&json!("low")).unwrap());
        assert_eq!(Confidence::None, parse_confidence(&json!("none")).unwrap());
        assert_eq!(Confidence::Ok, parse_confidence(&json!("OK")).unwrap());
        assert_eq!(Confidence::Ok, parse_confidence(&json!("ok")).unwrap());
    }

    #[test]
    fn test_parse_remaining_work() {
        let sheet = read_original_table_sheet("sheet.json").unwrap();
        let mut vec: Vec<Vec<Value>> = vec![];
        for val in &sheet.values[2..] {
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

pub fn temp_main() {
    let sheet = read_original_table_sheet("sheet.json").unwrap();
    let records = parse_table(&sheet.values[2..]).unwrap();

    let mut confidence_map: HashMap<Confidence, u16> = HashMap::new();
    for record in &records {
        let counter = confidence_map.entry(record.confidence.clone()).or_insert(0);
        *counter += 1;
    }

    dbg!(confidence_map);

    let mut map: HashMap<String, Vec<CardDropRecord>> = HashMap::new();
    for record in records {
        let vec = map.entry(record.name.as_str().to_owned()).or_insert(vec![]);
        vec.push(record);
    }

    dbg!(map.keys().len());
    std::fs::write("map.json", serde_json::to_string_pretty(&map).unwrap()).unwrap();

    let mut multiple_map: HashMap<String, Vec<CardDropRecord>> = HashMap::new();
    for (name, record) in map {
        if record.len() > 1 {
            multiple_map.insert(name.clone(), record.clone());
        }
    }

    dbg!(multiple_map.keys().len());
    std::fs::write(
        "multiple-map.json",
        serde_json::to_string_pretty(&multiple_map).unwrap(),
    )
    .unwrap();

    let mut _map: HashMap<&CardDropRecord, Vec<HashSet<DropSource>>> = HashMap::new();

    let mut set: HashSet<DropSource> = HashSet::new();
    set.insert(DropSource::ChestObject);
    set.insert(DropSource::ExpeditionLogbook);
    set.insert(DropSource::Vendor(Some(Vendor::KiracShop)));
    set.insert(DropSource::Vendor(Some(Vendor::KiracShop)));
    dbg!(set);

    let sheet = read_original_table_sheet("sheet.json").unwrap();
    let records = parse_table(&sheet.values[2..]).unwrap();

    for record in records {
        let drop_source = parse_drop_source(&record).unwrap();
        if drop_source.contains(&DropSource::ExpeditionLogbook) {
            dbg!(record);
        }
    }
    // std::fs::write("map.json", &serde_json::to_string_pretty(&map).unwrap()).unwrap();
}

pub fn write_sized_rewards() {
    let vec: Vec<NinjaCardData> =
        serde_json::from_str(&std::fs::read_to_string("ninja-data.json").unwrap()).unwrap();
    let mut with_size: Vec<String> = Vec::new();
    for card_data in vec {
        let reward = &card_data.explicit_modifiers[0].text;
        if reward.contains("<size:") {
            with_size.push(reward.clone());
        }
    }

    std::fs::write(
        "rewards-with-size.json",
        serde_json::to_string(&with_size).unwrap(),
    )
    .unwrap();
}
