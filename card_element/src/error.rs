use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    RegExpError(regex::Error),
    DiviError(divi::error::Error),
    IoError(std::io::Error),
    SerdeError(serde_json::Error),
    FetcherError(fetcher::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::RegExpError(err) => err.fmt(f),
            Error::DiviError(err) => err.fmt(f),
            Error::IoError(err) => err.fmt(f),
            Error::SerdeError(err) => err.fmt(f),
            Error::FetcherError(err) => err.fmt(f),
        }
    }
}

impl From<divi::error::Error> for Error {
    fn from(value: divi::error::Error) -> Self {
        Error::DiviError(value)
    }
}

impl From<regex::Error> for Error {
    fn from(value: regex::Error) -> Self {
        Error::RegExpError(value)
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

impl From<fetcher::Error> for Error {
    fn from(value: fetcher::Error) -> Self {
        Error::FetcherError(value)
    }
}
