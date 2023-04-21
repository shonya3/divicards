use serde::Serialize;

#[derive(Debug)]
pub enum Error {
    ConfigurationError,
    ConfigError(&'static str),
    CSVError(csv::Error),
    ParseError(&'static str, std::num::ParseFloatError),
    IOError(&'static str, std::io::Error),
    NoPriceError(String),
    NotDivinationCard(String),
    FromUtf8Error(std::string::FromUtf8Error),
    ReqwestError(reqwest::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Error::ConfigurationError => write!(f, "Configuration error"),
            Error::ConfigError(err_str) => write!(f, "No argument: {}. Example usage: ./divcards 5.5 200 data.csv\nWhere 5.5 - minimal card price, 200 - divine price in chaos, data.csv - path to data file", err_str),
            Error::ParseError(err_str, _) => {
                write!(f, "Parse error: {}. ", err_str)
            }
            Error::CSVError(ref err) => write!(f, "{}", err),
            Error::IOError(err_str, ref err) => write!(f, "{} : {}", err_str, err),
            Error::NoPriceError(name) => write!(f, "Divination card has no price. Card name: {}", name),
            Error::NotDivinationCard(name) => write!(f, "This item is probably not a divination card: {}", name),
            Error::FromUtf8Error(err) => write!(f, "{}", err),
            Error::ReqwestError(err) => write!(f, "{}", err),
        }
    }
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

// impl From<std::num::ParseIntError> for Error {
//     fn from(err: std::num::ParseIntError) -> Self {
//         Error::Parse(err)
//     }
// }

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Error::FromUtf8Error(err)
    }
}

impl From<csv::Error> for Error {
    fn from(err: csv::Error) -> Self {
        Error::CSVError(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::ReqwestError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IOError("", err)
    }
}
