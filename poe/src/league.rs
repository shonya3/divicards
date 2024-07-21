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
    Settlers,
    #[serde(alias = "Hardcore Settlers")]
    HardcoreSettlers,
    #[serde(alias = "SSF Settlers")]
    SSFSettlers,
    #[serde(alias = "HC SSF Settlers")]
    SSFHCSettlers,
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
            League::Settlers => write!(f, "Settlers"),
            League::HardcoreSettlers => write!(f, "Hardcore Settlers"),
            League::SSFSettlers => write!(f, "SSF Settlers"),
            League::SSFHCSettlers => write!(f, "HC SSF Settlers"),
            League::Custom(league) => write!(f, "{}", league),
        }
    }
}

impl From<TradeLeague> for League {
    fn from(value: TradeLeague) -> Self {
        match value {
            TradeLeague::Standard => League::Standard,
            TradeLeague::Hardcore => League::Hardcore,
            TradeLeague::Settlers => League::Settlers,
            TradeLeague::HardcoreSettlers => League::HardcoreSettlers,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Hash, Eq, PartialEq, Default)]
pub enum TradeLeague {
    Standard,
    Hardcore,
    #[default]
    Settlers,
    #[serde(alias = "Hardcore Settlers", rename = "Hardcore Settlers")]
    HardcoreSettlers,
}

impl Display for TradeLeague {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TradeLeague::Standard => write!(f, "Standard"),
            TradeLeague::Hardcore => write!(f, "Hardcore"),
            TradeLeague::Settlers => write!(f, "Settlers"),
            TradeLeague::HardcoreSettlers => write!(f, "Hardcore Settlers"),
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
            League::Settlers => Ok(TradeLeague::Settlers),
            League::HardcoreSettlers => Ok(TradeLeague::HardcoreSettlers),
            League::SSFSettlers => Err(msg),
            League::SSFHCSettlers => Err(msg),
            League::Custom(_) => Err(msg),
        }
    }
}
