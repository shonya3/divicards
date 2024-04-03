use crate::{parse::ParseSourceError, spreadsheet::rich::ParseCellError};
use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    HttpError(reqwest::Error),
    IoError(std::io::Error),
    SerdeError(serde_json::Error),
    ParseCardNameError(String),
    ValueNotStr(serde_json::Value),
    GoogleError(googlesheets::error::Error),
    DiviError(divi::error::Error),
    StrumParseError(strum::ParseError),
    ParseSourceError(ParseSourceError),
    ParseCellError(crate::spreadsheet::rich::ParseCellError),
    #[cfg(feature = "fetch")]
    FetcherError(fetcher::FetcherError),
    #[cfg(feature = "fetch")]
    PoeDataError(poe_data::error::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::HttpError(err) => err.fmt(f),
            Error::IoError(err) => err.fmt(f),
            Error::SerdeError(err) => err.fmt(f),
            Error::ParseCardNameError(name) => write!(f, "{name} is not a card"),
            Error::ValueNotStr(val) => write!(f, "{val} is not an str"),
            Error::GoogleError(err) => err.fmt(f),
            Error::DiviError(err) => err.fmt(f),
            Error::StrumParseError(err) => err.fmt(f),
            Error::ParseSourceError(err) => err.fmt(f),
            Error::ParseCellError(err) => err.fmt(f),
            #[cfg(feature = "fetch")]
            Error::FetcherError(err) => err.fmt(f),
            #[cfg(feature = "fetch")]
            Error::PoeDataError(err) => err.fmt(f),
        }
    }
}

#[cfg(feature = "fetch")]
impl From<fetcher::FetcherError> for Error {
    fn from(value: fetcher::FetcherError) -> Self {
        Error::FetcherError(value)
    }
}

#[cfg(feature = "fetch")]
impl From<poe_data::error::Error> for Error {
    fn from(value: poe_data::error::Error) -> Self {
        Error::PoeDataError(value)
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::HttpError(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IoError(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::SerdeError(value)
    }
}

impl From<googlesheets::error::Error> for Error {
    fn from(value: googlesheets::error::Error) -> Self {
        Error::GoogleError(value)
    }
}

impl From<divi::error::Error> for Error {
    fn from(value: divi::error::Error) -> Self {
        Error::DiviError(value)
    }
}

impl From<strum::ParseError> for Error {
    fn from(value: strum::ParseError) -> Self {
        Error::StrumParseError(value)
    }
}

impl From<ParseSourceError> for Error {
    fn from(value: ParseSourceError) -> Self {
        Error::ParseSourceError(value)
    }
}

impl From<ParseCellError> for Error {
    fn from(value: ParseCellError) -> Self {
        Error::ParseCellError(value)
    }
}
