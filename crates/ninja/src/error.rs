use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;

#[derive(Debug)]
pub enum Error {
    ReqwestError(ReqwestError),
    SerdeError(SerdeError),
    NoItemsBadRequest,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ReqwestError(err) => err.fmt(f),
            Error::SerdeError(err) => err.fmt(f),
            Error::NoItemsBadRequest => f.write_str("No items, probably bad request."),
        }
    }
}

impl From<SerdeError> for Error {
    fn from(value: SerdeError) -> Self {
        Self::SerdeError(value)
    }
}
impl From<ReqwestError> for Error {
    fn from(value: ReqwestError) -> Self {
        Self::ReqwestError(value)
    }
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}
