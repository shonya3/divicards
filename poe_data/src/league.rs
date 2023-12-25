use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{consts::WIKI_API_URL, error::Error};

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
    pub async fn fetch() -> Result<Vec<LeagueReleaseInfo>, Error> {
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
