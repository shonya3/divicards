use crate::TradeLeague;
use serde::Serialize;
use std::{fmt::Display, num::ParseIntError};

#[derive(Debug)]
pub enum Error {
    HttpError(reqwest::Error),
    SerdeError(serde_json::Error),
    MissingHeaders,
    NoPricesForLeagueOnNinja(TradeLeague),
    ParseIntError(ParseIntError),
    CsvError(csv::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::HttpError(err) => err.fmt(f),
            Error::SerdeError(err) => err.fmt(f),
            Error::MissingHeaders => write!(f, "File should contain headers: name, amount."),
            Error::NoPricesForLeagueOnNinja(league) => {
                write!(f, "Prices for {} league do not exist on poe.ninja.", league)
            }
            Error::ParseIntError(err) => err.fmt(f),
            Error::CsvError(err) => err.fmt(f),
        }
    }
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::HttpError(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::SerdeError(value)
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error::ParseIntError(value)
    }
}

impl From<csv::Error> for Error {
    fn from(value: csv::Error) -> Self {
        Error::CsvError(value)
    }
}
