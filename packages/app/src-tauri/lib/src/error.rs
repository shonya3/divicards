use std::fmt::Display;

use crate::poe::error::AuthError;
use serde::{ser::SerializeStruct, Serialize};

#[derive(Debug)]
pub enum Error {
    HttpError(reqwest::Error),
    SerdeError(serde_json::Error),
    DiviError(divi::error::Error),
    AuthError(AuthError),
}

impl Error {
    pub fn kind(&self) -> &'static str {
        match self {
            Error::HttpError(_) => "httpError",
            Error::SerdeError(_) => "serdeError",
            Error::DiviError(_) => "diviError",
            Error::AuthError(_) => "authError",
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AuthError(err) => err.fmt(f),
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
        match self {
            Error::AuthError(err) => err.serialize(serializer),
            _ => {
                let mut err = serializer.serialize_struct("Error", 2)?;
                err.serialize_field("message", self.to_string().as_str())?;
                err.serialize_field("kind", self.kind())?;
                err.serialize_field("appErrorFromTauri", &true)?;
                err.end()
            }
        }
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