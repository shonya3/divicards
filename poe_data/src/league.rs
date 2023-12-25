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

impl PartialEq for ReleaseVersion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[test]
fn same_league() {
    let version = ReleaseVersion("2.0.3".to_string());
    let other = ReleaseVersion("2.0.5".to_string());
    assert_eq!(version.major(), Some(2u16));
    assert_eq!(version.minor(), Some(0u16));
    assert_eq!(version.patch(), Some(3u16));
    assert_eq!(other.patch(), Some(5u16));
    assert!(version.same_league(&other))
}

impl ReleaseVersion {
    pub fn same_league(&self, other: &Self) -> bool {
        let major = self.major();
        let minor = self.minor();

        major.map_or(false, |_| major == other.major() && minor == other.minor())
    }

    pub fn major(&self) -> Option<u16> {
        self.0
            .split(".")
            .next()
            .map(|v| v.parse::<u16>().ok())
            .flatten()
    }

    pub fn minor(&self) -> Option<u16> {
        self.0
            .split(".")
            .nth(1)
            .map(|v| v.parse::<u16>().ok())
            .flatten()
    }

    pub fn patch(&self) -> Option<u16> {
        self.0
            .split(".")
            .nth(2)
            .map(|v| v.parse::<u16>().ok())
            .flatten()
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

impl LeagueReleaseInfo {
    #[cfg(feature = "fetch")]
    pub async fn fetch() -> Result<Vec<LeagueReleaseInfo>, crate::error::Error> {
        use crate::consts::WIKI_API_URL;
        use serde_json::Value;

        let mut league_relese_info_vec: Vec<LeagueReleaseInfo> = vec![];
        let url = format!("{WIKI_API_URL}?action=cargoquery&format=json&tables=events&fields=events.name,release_date,release_version");
        let json: Value = reqwest::get(url).await?.json().await?;
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
                .map(|v| {
                    v.as_str()
                        .map(|n| n.contains("league").then_some(n))
                        .flatten()
                })
                .flatten();

            if valid_league_name.is_some() {
                let info: LeagueReleaseInfo = serde_json::from_value(info_value.to_owned())?;
                league_relese_info_vec.push(info);
            }
        }

        Ok(league_relese_info_vec)
    }
}
