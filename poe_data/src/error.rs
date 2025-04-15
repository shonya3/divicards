#![cfg(feature = "fs_cache_fetcher")]
use crate::league::UnexpectedLeagueInfoShapeError;
use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    #[cfg(feature = "fs_cache_fetcher")]
    HttpError(reqwest::Error),
    #[cfg(feature = "fs_cache_fetcher")]
    GoogleError(googlesheets::error::Error),
    #[cfg(feature = "fs_cache_fetcher")]
    DiviError(divi::error::Error),
    IoError(std::io::Error),
    SerdeError(serde_json::Error),
    #[cfg(feature = "fs_cache_fetcher")]
    UnexpectedLeagueInfoShapeError(UnexpectedLeagueInfoShapeError),
    #[cfg(feature = "fs_cache_fetcher")]
    FetchMaps(crate::maps::FetchMapsError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "fs_cache_fetcher")]
            Error::HttpError(err) => err.fmt(f),
            Error::IoError(err) => err.fmt(f),
            Error::SerdeError(err) => err.fmt(f),
            #[cfg(feature = "fs_cache_fetcher")]
            Error::GoogleError(err) => err.fmt(f),
            #[cfg(feature = "fs_cache_fetcher")]
            Error::DiviError(err) => err.fmt(f),
            #[cfg(feature = "fs_cache_fetcher")]
            Error::UnexpectedLeagueInfoShapeError(err) => err.fmt(f),
            Error::FetchMaps(err) => write!(f, "{err:?}"),
        }
    }
}

#[cfg(feature = "fs_cache_fetcher")]
impl From<crate::maps::FetchMapsError> for Error {
    fn from(value: crate::maps::FetchMapsError) -> Self {
        Error::FetchMaps(value)
    }
}

#[cfg(feature = "fs_cache_fetcher")]
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

#[cfg(feature = "fs_cache_fetcher")]
impl From<googlesheets::error::Error> for Error {
    fn from(value: googlesheets::error::Error) -> Self {
        Error::GoogleError(value)
    }
}

#[cfg(feature = "fs_cache_fetcher")]
impl From<divi::error::Error> for Error {
    fn from(value: divi::error::Error) -> Self {
        Error::DiviError(value)
    }
}

#[cfg(feature = "fs_cache_fetcher")]
impl From<UnexpectedLeagueInfoShapeError> for Error {
    fn from(value: UnexpectedLeagueInfoShapeError) -> Self {
        Error::UnexpectedLeagueInfoShapeError(value)
    }
}
