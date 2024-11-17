//! Loading and parsing the [divcord spreadsheet](https://docs.google.com/spreadsheets/d/1Pf2KNuGguZLyf6eu_R0E503U0QNyfMZqaRETsN5g6kU/edit?pli=1#gid=0) [`Spreadsheet`]

pub mod cards;
pub mod consts;
pub mod dropsource;
pub mod error;
pub mod parse;
pub mod spreadsheet;

pub use crate::{
    cards::{cards_by_source, cards_by_source_types, CardBySource},
    dropsource::Source,
    error::Error,
    parse::{records, records_iter},
    spreadsheet::{record::Record, Spreadsheet},
};

pub use poe_data::PoeData;

#[cfg(feature = "fetch")]
pub use {
    fetcher::DataFetcher, poe_data::fetchers::PoeDataFetcher, spreadsheet::load::SpreadsheetFetcher,
};

// // Final rules
// if dumb.confidence == Confidence::None && !sources.is_empty() {
//     println!("{} {} {sources:?}", dumb.id, dumb.card);
// }

// if dumb.greynote != GreyNote::Empty
//     && dumb.confidence == Confidence::Done
//     && sources.is_empty()
//     && dumb.drops_to_verify.is_empty()
//     && dumb.id != 501
// {
//     return Err(ParseSourceError::SourceIsExptectedButEmpty {
//         record_id: dumb.id,
//         card: dumb.card.to_owned(),
//     });
// }
