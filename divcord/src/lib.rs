//! Loading and parsing the [divcord spreadsheet](https://docs.google.com/spreadsheets/d/1Pf2KNuGguZLyf6eu_R0E503U0QNyfMZqaRETsN5g6kU/edit?pli=1#gid=0) [`Spreadsheet`]

pub mod cards;
pub mod consts;
pub mod dropsource;
pub mod parse;
pub mod spreadsheet;

pub use crate::{
    cards::{cards_by_source, cards_by_source_types, CardBySource},
    dropsource::Source,
    parse::{records, records_iter, records_with_collect_all_errors, ParseRecordError},
    spreadsheet::{record::Record, Spreadsheet},
};

pub use poe_data::PoeData;

#[cfg(feature = "fs_cache_fetcher")]
pub use {
    fetcher::DataFetcher, poe_data::fetchers::PoeDataFetcher,
    spreadsheet::fs_cache_fetcher::SpreadsheetFetcher,
};
