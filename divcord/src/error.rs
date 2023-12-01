use std::fmt::Display;

use crate::{dropsource::parse::ParseSourceError, table::rich::ParseCellError};

#[derive(Debug)]
pub enum Error {
    HttpError(reqwest::Error),
    IoError(std::io::Error),
    SerdeError(serde_json::Error),
    ParseCardNameError(String),
    ValueNotStr(serde_json::Value),
    RowIsTooShort(String, u8),
    GoogleError(googlesheets::error::Error),
    DiviError(divi::error::Error),
    StrumParseError(strum::ParseError),
    ParseSourceError(ParseSourceError),
    #[cfg(feature = "fetch")]
    LoaderError(loader::Error),
    ParseCellError(crate::table::rich::ParseCellError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::HttpError(err) => err.fmt(f),
            Error::IoError(err) => err.fmt(f),
            Error::SerdeError(err) => err.fmt(f),
            Error::ParseCardNameError(name) => write!(f, "{name} is not a card"),
            Error::ValueNotStr(val) => write!(f, "{val} is not an str"),
            Error::RowIsTooShort(column, n_columns) => write!(f, "Could not parse {column}. Row is too short. Expected at least {n_columns} columns to extract {column}"),
            Error::GoogleError(err) => err.fmt(f),
            Error::DiviError(err) => err.fmt(f),
            Error::StrumParseError(err) => err.fmt(f),
            Error::ParseSourceError(err) => err.fmt(f),
            #[cfg(feature = "fetch")]
            Error::LoaderError(err) => err.fmt(f),
            Error::ParseCellError(err) => err.fmt(f),
        }
    }
}

#[cfg(feature = "fetch")]
impl From<loader::Error> for Error {
    fn from(value: loader::Error) -> Self {
        Error::LoaderError(value)
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
