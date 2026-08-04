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

#[derive(Debug, Clone, Serialize, Deserialize, Eq, Default)]
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

impl ReleaseVersion {
    pub const fn new(version: String) -> Self {
        Self(version)
    }

    pub fn as_str(&self) -> &str {
        &self.0
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
