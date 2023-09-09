// export type SheetsError = {
// 	error: { code: number; message: string; status: string };
// };

use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct GoogleError {
    pub code: u32,
    pub message: String,
    pub status: String,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct GoogleErrorResponse {
    pub error: GoogleError,
}

impl Display for GoogleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Debug)]
pub enum Error {
    ReqwestError(reqwest::Error),
    GoogleError(GoogleError),
    SerdeError(serde_json::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ReqwestError(err) => err.fmt(f),
            Error::GoogleError(err) => err.fmt(f),
            Error::SerdeError(err) => err.fmt(f),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::ReqwestError(value)
    }
}

impl From<GoogleError> for Error {
    fn from(value: GoogleError) -> Self {
        Error::GoogleError(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::SerdeError(value)
    }
}
