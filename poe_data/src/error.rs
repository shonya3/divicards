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
    ParseCardNameError(String),
    ValueNotStr(serde_json::Value),
    RowIsTooShort(String, u8),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "fetch")]
            Error::HttpError(err) => err.fmt(f),
            Error::IoError(err) => err.fmt(f),
            Error::SerdeError(err) => err.fmt(f),
            Error::ParseCardNameError(name) => write!(f, "{name} is not a card"),
            Error::ValueNotStr(val) => write!(f, "{val} is not an str"),
            Error::RowIsTooShort(column, n_columns) => write!(f, "Could not parse {column}. Row is too short. Expected at least {n_columns} columns to extract {column}"),
            #[cfg(feature = "fetch")]
            Error::GoogleError(err) => err.fmt(f),
            #[cfg(feature = "fetch")]
            Error::DiviError(err) => err.fmt(f),
        }
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
