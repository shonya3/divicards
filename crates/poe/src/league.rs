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
    Mirage,
    #[serde(alias = "Hardcore Mirage")]
    HardcoreMirage,
    #[serde(alias = "SSF Mirage")]
    SSFMirage,
    #[serde(alias = "HC SSF Mirage")]
    SSFHCMirage,
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
            League::Mirage => write!(f, "Mirage"),
            League::HardcoreMirage => write!(f, "Hardcore Mirage"),
            League::SSFMirage => write!(f, "SSF Mirage"),
            League::SSFHCMirage => write!(f, "HC SSF Mirage"),
            League::Custom(league) => write!(f, "{league}"),
        }
    }
}

impl From<TradeLeague> for League {
    fn from(value: TradeLeague) -> Self {
        match value {
            TradeLeague::Standard => League::Standard,
            TradeLeague::Hardcore => League::Hardcore,
            TradeLeague::Mirage => League::Mirage,
            TradeLeague::HardcoreMirage => League::HardcoreMirage,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Hash, Eq, PartialEq, Default)]
pub enum TradeLeague {
    Standard,
    Hardcore,
    #[default]
    Mirage,
    #[serde(alias = "Hardcore Mirage", rename = "Hardcore Mirage")]
    HardcoreMirage,
}

impl Display for TradeLeague {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TradeLeague::Standard => write!(f, "Standard"),
            TradeLeague::Hardcore => write!(f, "Hardcore"),
            TradeLeague::Mirage => write!(f, "Mirage"),
            TradeLeague::HardcoreMirage => write!(f, "Hardcore Mirage"),
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
            League::Mirage => Ok(TradeLeague::Mirage),
            League::HardcoreMirage => Ok(TradeLeague::HardcoreMirage),
            League::SSFMirage => Err(msg),
            League::SSFHCMirage => Err(msg),
            League::Custom(_) => Err(msg),
        }
    }
}
