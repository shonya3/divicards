use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, Default)]
pub enum League {
    Standard,
    Hardcore,
    #[serde(alias = "Solo Self-Found", alias = "SSF Standard")]
    SSFStandard,
    #[serde(alias = "SSF Hardcore", alias = "Hardcore SSF")]
    SSFHardcore,
    #[default]
    Allflame,
    #[serde(alias = "Hardcore Allflame")]
    HardcoreAllflame,
    #[serde(alias = "SSF Allflame")]
    SSFAllflame,
    #[serde(alias = "HC SSF Allflame")]
    SSFHCAllflame,
    #[serde(untagged)]
    Custom(String),
}

impl League {
    pub fn is_trade(&self) -> bool {
        TradeLeague::try_from(self.to_owned()).is_ok()
    }
}

impl Display for League {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            League::Standard => write!(f, "Standard"),
            League::Hardcore => write!(f, "Hardcore"),
            League::SSFStandard => write!(f, "Solo Self-Found"),
            League::SSFHardcore => write!(f, "Hardcore SSF"),
            League::Allflame => write!(f, "Allflame"),
            League::HardcoreAllflame => write!(f, "Hardcore Allflame"),
            League::SSFAllflame => write!(f, "SSF Allflame"),
            League::SSFHCAllflame => write!(f, "HC SSF Allflame"),
            League::Custom(league) => write!(f, "{league}"),
        }
    }
}

impl From<TradeLeague> for League {
    fn from(value: TradeLeague) -> Self {
        match value {
            TradeLeague::Standard => League::Standard,
            TradeLeague::Hardcore => League::Hardcore,
            TradeLeague::Allflame => League::Allflame,
            TradeLeague::HardcoreAllflame => League::HardcoreAllflame,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, Eq, PartialEq, Default)]
pub enum TradeLeague {
    Standard,
    Hardcore,
    #[default]
    Allflame,
    #[serde(alias = "Hardcore Allflame", rename = "Hardcore Allflame")]
    HardcoreAllflame,
}

impl Display for TradeLeague {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TradeLeague::Standard => write!(f, "Standard"),
            TradeLeague::Hardcore => write!(f, "Hardcore"),
            TradeLeague::Allflame => write!(f, "Allflame"),
            TradeLeague::HardcoreAllflame => write!(f, "Hardcore Allflame"),
        }
    }
}

impl TryFrom<League> for TradeLeague {
    type Error = &'static str;

    fn try_from(value: League) -> Result<Self, Self::Error> {
        let msg = "This league is not a trade league";

        match value {
            League::Standard => Ok(TradeLeague::Standard),
            League::Hardcore => Ok(TradeLeague::Hardcore),
            League::SSFStandard => Err(msg),
            League::SSFHardcore => Err(msg),
            League::Allflame => Ok(TradeLeague::Allflame),
            League::HardcoreAllflame => Ok(TradeLeague::HardcoreAllflame),
            League::SSFAllflame => Err(msg),
            League::SSFHCAllflame => Err(msg),
            League::Custom(_) => Err(msg),
        }
    }
}
