//! Parse drop sources.

use crate::dropsource::predefined::PredefinedSource;
use crate::dropsource::Source;
use crate::spreadsheet::record::ParseDumbError;
use crate::spreadsheet::rich::HexColor;
use crate::spreadsheet::{
    record::{Confidence, Dumb, GreyNote, Record},
    rich::DropsFrom,
    Spreadsheet,
};
use divi::IsCard;
use poe_data::act::ActArea;
use poe_data::PoeData;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Go-to function for records
pub fn records_with_collect_all_errors(
    spreadsheet: &Spreadsheet,
    poe_data: &PoeData,
) -> Result<Vec<Record>, Vec<ParseRecordError>> {
    let mut records: Vec<Record> = Vec::new();
    let mut errors: Vec<ParseRecordError> = Vec::new();
    records_iter(spreadsheet, poe_data).for_each(|result| match result {
        Ok(record_result) => {
            records.push(record_result.record);
            if !record_result.errors.is_empty() {
                errors.push(ParseRecordError::ParseDropSources(record_result.errors));
            }
        }
        Err(parse_dumb_err) => {
            errors.push(ParseRecordError::ParseDumb(parse_dumb_err));
        }
    });

    if !errors.is_empty() {
        return Err(errors);
    };

    Ok(records)
}

/// More low-level function for records.
pub fn records_iter<'a>(
    spreadsheet: &'a Spreadsheet,
    poe_data: &'a PoeData,
) -> impl Iterator<Item = Result<ParseRecordResult, ParseDumbError>> + 'a {
    spreadsheet
        .dumb_records()
        .map(|dumb| Ok(parse_record(dumb?, poe_data)))
}

/// Parse till first error. Maybe delete this one later.
pub fn records(
    spreadsheet: &Spreadsheet,
    poe_data: &PoeData,
) -> Result<Vec<Record>, ParseRecordError> {
    records_iter(spreadsheet, poe_data)
        .map(|result| match result {
            Ok(record_result) => match record_result.errors.is_empty() {
                true => Ok(record_result.record),
                false => {
                    let errors = ParseRecordError::ParseDropSources(record_result.errors);
                    Err(errors)
                }
            },
            Err(parse_dumb_err) => Err(ParseRecordError::ParseDumb(parse_dumb_err)),
        })
        .collect()
}

#[derive(Debug)]
pub enum ParseRecordError {
    ParseDumb(ParseDumbError),
    ParseDropSources(Vec<ParseSourceError>),
}

impl Display for ParseRecordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseRecordError::ParseDumb(parse_dumb_error2) => parse_dumb_error2.fmt(f),
            ParseRecordError::ParseDropSources(errors) => {
                let errors_string = errors
                    .iter()
                    .map(|err| err.to_string())
                    .collect::<Vec<_>>()
                    .join("\n");
                f.write_str(&errors_string)
            }
        }
    }
}

impl From<Vec<ParseSourceError>> for ParseRecordError {
    fn from(value: Vec<ParseSourceError>) -> Self {
        ParseRecordError::ParseDropSources(value)
    }
}

impl From<ParseDumbError> for ParseRecordError {
    fn from(value: ParseDumbError) -> Self {
        ParseRecordError::ParseDumb(value)
    }
}

#[derive(Debug)]
pub struct ParseRecordResult {
    pub record: Record,
    pub errors: Vec<ParseSourceError>,
}

/// [Dumb] -> [Record]
pub fn parse_record(dumb: Dumb, poe_data: &PoeData) -> ParseRecordResult {
    let (sources, mut errors) = parse_record_dropsources(&dumb, poe_data);
    let (verify_sources, errors_verify_drops_from) =
        parse_dropses_from(&dumb, poe_data, RichColumnVariant::Verify);

    errors.extend(
        errors_verify_drops_from
            .into_iter()
            .map(ParseSourceError::from),
    );

    ParseRecordResult {
        record: Record {
            sources,
            verify_sources,
            id: dumb.id,
            greynote: dumb.greynote,
            card: dumb.card,
            tag_hypothesis: dumb.tag_hypothesis,
            confidence: dumb.confidence,
            remaining_work: dumb.remaining_work,
            notes: dumb.notes,
        },
        errors,
    }
}

#[derive(Debug)]
pub struct ParseSourceError {
    pub card: String,
    pub record_id: usize,
    pub kind: ParseSourceErrorKind,
}

#[derive(Debug)]
pub enum ParseSourceErrorKind {
    UnknownDropSource(DropsFrom),
    ActsMustBeItalic(DropsFrom),
    SourceOrVerifyIsExpectedButEmpty,
    GreynoteDisabledButCardNotLegacy,
    LegacyCardShouldBeMarkedAsDisabled,
    ConfidenceNoneButHasSources,
}

impl From<ParseDropsFromError> for ParseSourceError {
    fn from(value: ParseDropsFromError) -> Self {
        ParseSourceError {
            card: value.card,
            record_id: value.record_id,
            kind: match value.kind {
                ParseDropsFromErrorKind::Unknown => {
                    ParseSourceErrorKind::UnknownDropSource(value.drops_from)
                }
                ParseDropsFromErrorKind::ActsMustBeItalic => {
                    ParseSourceErrorKind::ActsMustBeItalic(value.drops_from)
                }
            },
        }
    }
}

#[derive(Debug)]
pub struct UnknownDropsFrom {
    pub card: String,
    pub record_id: usize,
    pub drops_from: DropsFrom,
}

impl Display for ParseSourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ParseSourceError {
            card,
            record_id,
            kind,
        } = self;
        match kind {
            ParseSourceErrorKind::UnknownDropSource(drops_from) => write!(
                        f,
                        "{record_id}.{card}. Unknown variant of card source {}. {}",
                        drops_from.name,
                        record_url(*record_id, DivcordColumn::Sources)
                    ),
            ParseSourceErrorKind::ActsMustBeItalic(drops_from) => write!(
                        f,
                        "{record_id}.{card}. Spreadsheet styling error: If {} refers to acts, it's font-style must be italic. {}",
                        drops_from.name,
                        record_url(*record_id, DivcordColumn::Sources)
                    ),
            ParseSourceErrorKind::SourceOrVerifyIsExpectedButEmpty => write!(
                        f,
                        "{record_id}.{card}. Source or need-to-verify source is expected, but there is none. {}",
                        record_url(*record_id, DivcordColumn::Sources)
                    ),
            ParseSourceErrorKind::GreynoteDisabledButCardNotLegacy => write!(
                        f,
                        "{record_id}. Card {card} has greynote Disabled, but this is not a legacy card {}",
                        record_url(*record_id, DivcordColumn::GreyNote)
                    ),
            ParseSourceErrorKind::LegacyCardShouldBeMarkedAsDisabled => write!(
                        f,
                        "{record_id}. Card {card} is legacy, but not marked as disabled. {}",
                        record_url(*record_id, DivcordColumn::GreyNote)
                    ),
            ParseSourceErrorKind::ConfidenceNoneButHasSources => write!(
                        f,
                        "{record_id}.{card}. Confidence is None, but sources not empty {}",
                        record_url(*record_id, DivcordColumn::Sources)
                    ),
        }
    }
}

pub enum DivcordColumn {
    GreyNote,
    Card,
    TagHypothesis,
    Confidence,
    RemainingWork,
    Sources,
    Verify,
    Notes,
}

impl DivcordColumn {
    pub fn letter(&self) -> char {
        match self {
            DivcordColumn::GreyNote => 'A',
            DivcordColumn::Card => 'B',
            DivcordColumn::TagHypothesis => 'C',
            DivcordColumn::Confidence => 'D',
            DivcordColumn::RemainingWork => 'F',
            DivcordColumn::Sources => 'G',
            DivcordColumn::Verify => 'H',
            DivcordColumn::Notes => 'I',
        }
    }
}

pub fn record_url(id: usize, column: DivcordColumn) -> String {
    format!("https://docs.google.com/spreadsheets/d/1Pf2KNuGguZLyf6eu_R0E503U0QNyfMZqaRETsN5g6kU/edit?pli=1#gid=0&range={}{id}", column.letter())
}

pub fn parse_record_dropsources(
    dumb: &Dumb,
    poe_data: &PoeData,
) -> (Vec<Source>, Vec<ParseSourceError>) {
    // Legacy cards checks
    if dumb.greynote != GreyNote::Disabled && dumb.card.as_str().is_legacy_card() {
        let err = ParseSourceError {
            record_id: dumb.id,
            card: dumb.card.to_owned(),
            kind: ParseSourceErrorKind::LegacyCardShouldBeMarkedAsDisabled,
        };
        return (vec![Source::disabled()], vec![err]);
    }

    let mut errors: Vec<ParseSourceError> = Vec::new();
    if dumb.greynote == GreyNote::Disabled {
        if !dumb.card.as_str().is_legacy_card() {
            errors.push(ParseSourceError {
                record_id: dumb.id,
                card: dumb.card.to_owned(),
                kind: ParseSourceErrorKind::GreynoteDisabledButCardNotLegacy,
            });
        }
        return (vec![Source::disabled()], vec![]);
    }

    // Parse
    let (sources, drops_from_errors) =
        parse_dropses_from(dumb, poe_data, RichColumnVariant::Sources);
    errors.extend(drops_from_errors.into_iter().map(ParseSourceError::from));

    // Final checks
    if dumb.confidence == Confidence::None && !sources.is_empty() {
        errors.push(ParseSourceError {
            record_id: dumb.id,
            card: dumb.card.to_owned(),
            kind: ParseSourceErrorKind::ConfidenceNoneButHasSources,
        });
    }

    if dumb.confidence == Confidence::Done && sources.is_empty() && dumb.drops_to_verify.is_empty()
    {
        errors.push(ParseSourceError {
            record_id: dumb.id,
            card: dumb.card.to_owned(),
            kind: ParseSourceErrorKind::SourceOrVerifyIsExpectedButEmpty,
        });
    }

    (sources, errors)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RichColumnVariant {
    Sources,
    Verify,
}

impl RichColumnVariant {
    pub fn column_letter(&self) -> char {
        match self {
            RichColumnVariant::Sources => 'G',
            RichColumnVariant::Verify => 'H',
        }
    }
}

/// Parses all instances of record's drops_from and collects it into one Vec<Source>
pub fn parse_dropses_from(
    dumb: &Dumb,
    poe_data: &PoeData,
    column: RichColumnVariant,
) -> (Vec<Source>, Vec<ParseDropsFromError>) {
    let mut sources: Vec<Source> = vec![];
    let mut errors: Vec<ParseDropsFromError> = Vec::new();
    let drops_to_parse = match column {
        RichColumnVariant::Sources => &dumb.drops,
        RichColumnVariant::Verify => &dumb.drops_to_verify,
    };

    for d in drops_to_parse {
        match parse_one_drops_from(d, dumb, poe_data) {
            Ok(inner_sources) => sources.extend(inner_sources),
            Err(err) => errors.push(err),
        };
    }

    (sources, errors)
}

#[derive(Debug, PartialEq)]
pub enum ParseDropsFromErrorKind {
    Unknown,
    ActsMustBeItalic, // DropSourceLevelisLowerThanCardMinLevel {
                      //     level: u32,
                      //     card: String,
                      //     card_min_drop_level: u32,
                      // },
}

#[derive(Debug)]
pub struct ParseDropsFromError {
    pub card: String,
    pub record_id: usize,
    pub drops_from: DropsFrom,
    pub kind: ParseDropsFromErrorKind,
}

pub fn parse_one_drops_from(
    d: &DropsFrom,
    dumb: &Dumb,
    PoeData {
        acts,
        cards,
        maps,
        mapbosses,
    }: &PoeData,
) -> Result<Vec<Source>, ParseDropsFromError> {
    if d.styles.strikethrough {
        return Ok(vec![]);
    }

    if let Ok(predefined_source) = d.name.parse::<PredefinedSource>() {
        return Ok(vec![Source::Predefined(predefined_source)]);
    }

    let card_min_drop_level = cards.card(&dumb.card).min_level.unwrap_or_default();
    // Acts + bosses
    if d.styles.italic {
        if let Some(sources) =
            find_in_acts_or_act_bosses(d, acts, card_min_drop_level.try_into().unwrap_or_default())
        {
            return Ok(sources);
        }
    }

    // Maps + bosses
    if dumb.greynote == GreyNote::AreaSpecific
        || (d.styles.color == HexColor::White && !d.styles.italic)
    {
        if let Some(map) = maps
            .iter()
            .find(|m| d.name == m.name || d.name == m.name.replace(" Map", "").as_str())
        {
            return Ok(vec![Source::Map(map.name.to_owned())]);
        }

        if let Some(boss) = mapbosses.iter().find(|b| b.name == strip_comment(&d.name)) {
            return Ok(vec![Source::MapBoss(boss.name.to_owned())]);
        }
    }

    // Check for acts if they were not checked before.
    // If acts are found now, they are styled incorrectly in the spreadsheet.
    if find_in_acts_or_act_bosses(d, acts, card_min_drop_level.try_into().unwrap_or_default())
        .is_some()
    {
        return Err(ParseDropsFromError {
            card: dumb.card.to_owned(),
            record_id: dumb.id,
            drops_from: d.to_owned(),
            kind: ParseDropsFromErrorKind::ActsMustBeItalic,
        });
    }

    Err(ParseDropsFromError {
        card: dumb.card.to_owned(),
        record_id: dumb.id,
        drops_from: d.to_owned(),
        kind: ParseDropsFromErrorKind::Unknown,
    })
}

fn find_in_acts_or_act_bosses(
    d: &DropsFrom,
    acts: &[ActArea],
    card_min_drop_level: u8,
) -> Option<Vec<Source>> {
    let act_areas_ids = acts::parse_act_areas(d, acts, card_min_drop_level);
    if !act_areas_ids.is_empty() {
        return Some(act_areas_ids.into_iter().map(Source::Act).collect());
    }

    if acts.iter().any(|a| {
        a.bossfights.iter().any(|b| {
            if b.name != d.name {
                return false;
            }

            if (a.area_level + 2) < card_min_drop_level {
                println!("Monster level is lower than card drop requirement");
                return false;
            }

            true
        })
    }) {
        return Some(vec![Source::ActBoss(d.name.to_string())]);
    }

    None
}

fn strip_comment(input: &str) -> String {
    let mut result = String::new();
    let mut inside_brackets = false;

    for c in input.chars() {
        match c {
            '(' => inside_brackets = true,
            ')' => inside_brackets = false,
            _ => {
                if !inside_brackets {
                    result.push(c);
                }
            }
        }
    }

    result.trim().to_owned()
}

mod acts {
    use crate::spreadsheet::rich::DropsFrom;
    use poe_data::act::{ActArea, ActAreaId};
    use serde::{Deserialize, Serialize};

    /// Examples of acts areas in divcord spreadsheet:
    /// - "The Blood Aqueduct"
    /// - "The Solaris Temple Level 1/2 (A8)"
    /// - "The Ossuary (A5/A10)"
    /// - "The Riverways (A6)"
    #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
    enum ActAreaDivcordNotation {
        #[serde(untagged)]
        Name(String),
        #[serde(untagged)]
        NameWithAct((String, u8)),
    }

    pub fn parse_act_areas(
        drops_from: &DropsFrom,
        acts: &[ActArea],
        min_level: u8,
    ) -> Vec<ActAreaId> {
        let s = &drops_from.name;
        let names = match are_act_numbers_specified(s) {
            true if s == "The Belly of the Beast (A4/A9)" => vec![
                ActAreaDivcordNotation::NameWithAct((
                    "The Belly of the Beast Level 1".to_string(),
                    4,
                )),
                ActAreaDivcordNotation::NameWithAct((
                    "The Belly of the Beast Level 2".to_string(),
                    4,
                )),
                ActAreaDivcordNotation::NameWithAct(("The Belly of the Beast".to_string(), 9)),
            ],
            true => parse_with_act_numbers(s),
            false => vec![ActAreaDivcordNotation::Name(s.to_owned())],
        };

        let areas = names
            .iter()
            .flat_map(|name| find_ids(name, acts, min_level))
            .collect();

        areas
    }

    fn are_act_numbers_specified(s: &str) -> bool {
        match s {
            s if s.contains('(') && s.contains(')') => true,
            s if s.contains("1/2") => true,
            _ => false,
        }
    }

    fn parse_with_act_numbers(s: &str) -> Vec<ActAreaDivcordNotation> {
        if !s.contains('(') && !s.contains('/') {
            panic!("Expected act notation, got {s}");
        };

        let mut split = s.split('(');
        let name = split.next().expect("No name, {s}");
        let names = match name.contains("1/2") {
            true => {
                let name = name.replace("1/2", "");
                let name = name.trim();
                [1, 2].iter().map(|n| format!("{name} {n}")).collect()
            }
            false => vec![name.trim().to_string()],
        };

        if let Some(acts) = split.next() {
            if acts.contains('/') {
                let (left, right) = acts.split_once('/').unwrap();

                let left = left
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse::<u8>()
                    .unwrap();

                let right = right
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse::<u8>()
                    .unwrap();

                names
                    .into_iter()
                    .flat_map(|name| {
                        [
                            ActAreaDivcordNotation::NameWithAct((name.clone(), left)),
                            ActAreaDivcordNotation::NameWithAct((name, right)),
                        ]
                    })
                    .collect()
            } else {
                let f: Box<dyn Fn(String) -> ActAreaDivcordNotation> = match acts
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse::<u8>()
                {
                    Ok(act) => Box::new(move |name: String| {
                        ActAreaDivcordNotation::NameWithAct((name, act))
                    }),
                    Err(_) => Box::new(|name: String| {
                        println!("No act notation in brackets {acts}");
                        ActAreaDivcordNotation::Name(name)
                    }),
                };

                names.into_iter().map(f).collect()
            }
        } else {
            names
                .into_iter()
                .map(ActAreaDivcordNotation::Name)
                .collect()
        }
    }

    fn find_ids(name: &ActAreaDivcordNotation, acts: &[ActArea], min_level: u8) -> Vec<ActAreaId> {
        match name {
            ActAreaDivcordNotation::Name(name) => acts
                .iter()
                .filter(|a| &a.name == name && !a.is_town && (a.area_level + 2) >= min_level)
                .map(|a| a.id.to_owned())
                .collect(),
            ActAreaDivcordNotation::NameWithAct((name, act)) => {
                let mut v = vec![];
                if let Some(a) = acts
                    .iter()
                    .find(|a| &a.name == name && &a.act == act && (a.area_level + 2) >= min_level)
                {
                    v.push(a.id.to_owned())
                };

                v
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn act_area_notation() {
            assert_eq!(
                parse_with_act_numbers("The Solaris Temple Level 1/2 (A8)"),
                vec![
                    ActAreaDivcordNotation::NameWithAct((
                        "The Solaris Temple Level 1".to_owned(),
                        8
                    )),
                    ActAreaDivcordNotation::NameWithAct((
                        "The Solaris Temple Level 2".to_owned(),
                        8
                    ))
                ]
            );

            assert_eq!(
                parse_with_act_numbers("The Ossuary (A5/A10)"),
                vec![
                    ActAreaDivcordNotation::NameWithAct(("The Ossuary".to_owned(), 5)),
                    ActAreaDivcordNotation::NameWithAct(("The Ossuary".to_owned(), 10))
                ]
            );

            assert_eq!(
                parse_with_act_numbers("The Riverways (A6)"),
                vec![ActAreaDivcordNotation::NameWithAct((
                    "The Riverways".to_owned(),
                    6
                ))]
            );
        }
    }
}
