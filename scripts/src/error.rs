use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    HttpError(reqwest::Error),
    IoError(std::io::Error),
    SerdeError(serde_json::Error),
    ParseNameError(String),
    ValueNotStr(serde_json::Value),
    RowIsTooShort(String, u8),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::HttpError(err) => err.fmt(f),
            Error::IoError(err) => err.fmt(f),
            Error::SerdeError(err) => err.fmt(f),
            Error::ParseNameError(name) => write!(f, "{name} is not a card"),
            Error::ValueNotStr(val) => write!(f, "{val} is not an str"),
            Error::RowIsTooShort(column, n_columns) => write!(f, "Could not parse {column}. Row is too short. Expected at least {n_columns} columns to extract {column}"),
        }
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
