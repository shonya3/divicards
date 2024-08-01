#![cfg(feature = "fetch")]
use crate::league::UnexpectedLeagueInfoShapeError;
use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    #[cfg(feature = "fetch")]
    HttpError(reqwest::Error),
    #[cfg(feature = "fetch")]
    GoogleError(googlesheets::error::Error),
    #[cfg(feature = "fetch")]
    DiviError(divi::error::Error),
    IoError(std::io::Error),
    SerdeError(serde_json::Error),
    #[cfg(feature = "fetch")]
    UnexpectedLeagueInfoShapeError(UnexpectedLeagueInfoShapeError),
    #[cfg(feature = "fetch")]
    FetchMaps(crate::maps::FetchMapsError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "fetch")]
            Error::HttpError(err) => err.fmt(f),
            Error::IoError(err) => err.fmt(f),
            Error::SerdeError(err) => err.fmt(f),
            #[cfg(feature = "fetch")]
            Error::GoogleError(err) => err.fmt(f),
            #[cfg(feature = "fetch")]
            Error::DiviError(err) => err.fmt(f),
            #[cfg(feature = "fetch")]
            Error::UnexpectedLeagueInfoShapeError(err) => err.fmt(f),
            Error::FetchMaps(err) => write!(f, "{err:?}"),
        }
    }
}

#[cfg(feature = "fetch")]
impl From<crate::maps::FetchMapsError> for Error {
    fn from(value: crate::maps::FetchMapsError) -> Self {
        Error::FetchMaps(value)
    }
}

#[cfg(feature = "fetch")]
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

#[cfg(feature = "fetch")]
impl From<googlesheets::error::Error> for Error {
    fn from(value: googlesheets::error::Error) -> Self {
        Error::GoogleError(value)
    }
}

#[cfg(feature = "fetch")]
impl From<divi::error::Error> for Error {
    fn from(value: divi::error::Error) -> Self {
        Error::DiviError(value)
    }
}

#[cfg(feature = "fetch")]
impl From<UnexpectedLeagueInfoShapeError> for Error {
    fn from(value: UnexpectedLeagueInfoShapeError) -> Self {
        Error::UnexpectedLeagueInfoShapeError(value)
    }
}
