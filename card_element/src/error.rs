use crate::uniques_fetcher::Error as FetchUniquesError;
use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    RegExpError(regex::Error),
    IoError(std::io::Error),
    SerdeError(serde_json::Error),
    NinjaError(ninja::Error),
    PoeDataError(poe_data::error::Error),
    FetchUniques(FetchUniquesError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::RegExpError(e) => e.fmt(f),
            Error::IoError(e) => e.fmt(f),
            Error::SerdeError(e) => e.fmt(f),
            Error::NinjaError(e) => e.fmt(f),
            Error::PoeDataError(e) => e.fmt(f),
            Error::FetchUniques(e) => e.fmt(f),
        }
    }
}

impl From<regex::Error> for Error {
    fn from(value: regex::Error) -> Self {
        Self::RegExpError(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeError(value)
    }
}

impl From<ninja::Error> for Error {
    fn from(value: ninja::Error) -> Self {
        Self::NinjaError(value)
    }
}

impl From<poe_data::error::Error> for Error {
    fn from(value: poe_data::error::Error) -> Self {
        Self::PoeDataError(value)
    }
}

impl From<FetchUniquesError> for Error {
    fn from(value: FetchUniquesError) -> Self {
        Self::FetchUniques(value)
    }
}
