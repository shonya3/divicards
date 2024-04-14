use crate::TradeLeague;
use csv::Error as CsvError;
use ninja::Error as NinjaError;
use reqwest::Error as ReqwestError;
use serde::Serialize;
use serde_json::Error as SerdeError;
use std::{fmt::Display, num::ParseIntError};

#[derive(Debug)]
pub enum Error {
    ReqwestError(ReqwestError),
    SerdeError(SerdeError),
    MissingHeaders,
    NoPricesForLeagueOnNinja(TradeLeague),
    ParseIntError(ParseIntError),
    CsvError(CsvError),
    NinjaError(NinjaError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ReqwestError(err) => err.fmt(f),
            Error::SerdeError(err) => err.fmt(f),
            Error::MissingHeaders => write!(f, "File should contain headers: name, amount."),
            Error::NoPricesForLeagueOnNinja(league) => {
                write!(f, "Prices for {} league do not exist on poe.ninja.", league)
            }
            Error::ParseIntError(err) => err.fmt(f),
            Error::CsvError(err) => err.fmt(f),
            Error::NinjaError(err) => err.fmt(f),
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

impl From<ReqwestError> for Error {
    fn from(value: ReqwestError) -> Self {
        Error::ReqwestError(value)
    }
}

impl From<SerdeError> for Error {
    fn from(value: SerdeError) -> Self {
        Error::SerdeError(value)
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error::ParseIntError(value)
    }
}

impl From<CsvError> for Error {
    fn from(value: CsvError) -> Self {
        Error::CsvError(value)
    }
}

impl From<NinjaError> for Error {
    fn from(value: NinjaError) -> Self {
        Error::NinjaError(value)
    }
}
