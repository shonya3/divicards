use std::fmt::Display;

use serde::Serialize;

#[derive(Debug)]
pub enum Error {
    HttpError(reqwest::Error),
    SerdeError(serde_json::Error),
    DiviError(divi::error::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::HttpError(err) => err.fmt(f),
            Error::SerdeError(err) => err.fmt(f),
            Error::DiviError(err) => err.fmt(f),
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

impl From<divi::error::Error> for Error {
    fn from(value: divi::error::Error) -> Self {
        Error::DiviError(value)
    }
}
