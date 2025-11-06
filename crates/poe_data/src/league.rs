use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeagueReleaseInfo {
    #[serde(alias = "name")]
    pub name: String,
    #[serde(alias = "release date")]
    pub date: String,
    #[serde(alias = "release version")]
    pub version: ReleaseVersion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ReleaseVersion(String);

impl Display for ReleaseVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq for ReleaseVersion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[test]
fn is_equal() {
    let version = ReleaseVersion("2.0.3".to_string());
    let other = ReleaseVersion("2.0.5".to_string());
    assert_eq!(version.major(), Some(2u16));
    assert_eq!(version.minor(), Some(0u16));
    assert_eq!(version.patch(), Some(3u16));
    assert_eq!(other.patch(), Some(5u16));
    assert!(version.is_equal(&other))
}

impl ReleaseVersion {
    pub const fn new(version: String) -> Self {
        Self(version)
    }

    pub fn is_equal(&self, other: &Self) -> bool {
        let major = self.major();
        let minor = self.minor();

        major.is_some_and(|_| major == other.major() && minor == other.minor())
    }

    pub fn major(&self) -> Option<u16> {
        self.0.split('.').next().and_then(|v| v.parse::<u16>().ok())
    }

    pub fn minor(&self) -> Option<u16> {
        self.0.split('.').nth(1).and_then(|v| v.parse::<u16>().ok())
    }

    pub fn patch(&self) -> Option<u16> {
        self.0.split('.').nth(2).and_then(|v| v.parse::<u16>().ok())
    }
}

#[derive(Debug)]
pub enum UnexpectedLeagueInfoShapeError {
    NoCargoqueryArray,
    NoTitleObject,
}

impl Display for UnexpectedLeagueInfoShapeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnexpectedLeagueInfoShapeError::NoCargoqueryArray => {
                f.write_str("Unexpected outer object shape: no cargoquery array")
            }
            UnexpectedLeagueInfoShapeError::NoTitleObject => {
                f.write_str("Unexpected member of array shape: no title object")
            }
        }
    }
}

#[cfg(feature = "fs_cache_fetcher")]
pub mod fetch {
    use super::{LeagueReleaseInfo, UnexpectedLeagueInfoShapeError};
    use crate::consts::WIKI_API_URL;
    use crate::HTTP_CLIENT;
    use serde_json::Value;
    use std::fmt::Display;

    #[derive(Debug)]
    pub enum Error {
        Http(reqwest::Error),
        Json(serde_json::Error),
        Shape(UnexpectedLeagueInfoShapeError),
    }

    impl Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Error::Http(e) => write!(f, "Failed to fetch league info from wiki: {e}"),
                Error::Json(e) => write!(f, "Failed to parse league info JSON from wiki: {e}"),
                Error::Shape(e) => write!(f, "Unexpected shape for league info from wiki: {e}"),
            }
        }
    }

    impl From<reqwest::Error> for Error {
        fn from(err: reqwest::Error) -> Self {
            Error::Http(err)
        }
    }
    impl From<serde_json::Error> for Error {
        fn from(err: serde_json::Error) -> Self {
            Error::Json(err)
        }
    }
    impl From<UnexpectedLeagueInfoShapeError> for Error {
        fn from(err: UnexpectedLeagueInfoShapeError) -> Self {
            Error::Shape(err)
        }
    }

    pub async fn fetch() -> Result<Vec<LeagueReleaseInfo>, Error> {
        let mut league_relese_info_vec: Vec<LeagueReleaseInfo> = vec![];
        let params = [
            ("action", "cargoquery"),
            ("format", "json"),
            ("tables", "events"),
            ("fields", "events.name,release_date,release_version"),
            ("where", "events.type=\"Challenge league\""),
        ];
        let json: Value = HTTP_CLIENT
            .get(WIKI_API_URL)
            .query(&params)
            .send()
            .await?
            .json()
            .await?;
        let Some(vec) = json["cargoquery"].as_array() else {
            return Err(UnexpectedLeagueInfoShapeError::NoCargoqueryArray.into());
        };

        for value in vec.iter() {
            let info_value = &value["title"];
            let Some(info) = info_value.as_object() else {
                return Err(UnexpectedLeagueInfoShapeError::NoTitleObject.into());
            };

            let valid_league_name = info
                .get("name")
                .and_then(|v| v.as_str().and_then(|n| n.contains("league").then_some(n)));

            if valid_league_name.is_some() {
                let info: LeagueReleaseInfo = serde_json::from_value(info_value.to_owned())?;
                league_relese_info_vec.push(info);
            }
        }

        Ok(league_relese_info_vec)
    }
}
