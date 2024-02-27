//! Loading and parsing of [divcord spreadsheet](https://docs.google.com/spreadsheets/d/1Pf2KNuGguZLyf6eu_R0E503U0QNyfMZqaRETsN5g6kU/edit?pli=1#gid=0) [`Spreadsheet`]

pub mod cards;
pub mod consts;
pub mod dropsource;
pub mod error;
pub mod parse;
pub mod spreadsheet;

pub use dropsource::Source;
pub use parse::{records, records_iter};
pub use spreadsheet::{record::Record, Spreadsheet};
