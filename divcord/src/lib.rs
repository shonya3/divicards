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
pub use crate::spreadsheet::load::SpreadsheetFetcher;
#[cfg(feature = "fetch")]
pub use fetcher::DataFetcher;
#[cfg(feature = "fetch")]
pub use poe_data::fetchers::PoeDataFetcher;
