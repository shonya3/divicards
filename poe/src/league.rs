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
    Necropolis,
    #[serde(alias = "Hardcore Necropolis")]
    HardcoreNecropolis,
    #[serde(alias = "SSF Necropolis")]
    SSFNecropolis,
    #[serde(alias = "HC SSF Necropolis")]
    SSFHCNecropolis,
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
            League::Necropolis => write!(f, "Necropolis"),
            League::HardcoreNecropolis => write!(f, "Hardcore Necropolis"),
            League::SSFNecropolis => write!(f, "SSF Necropolis"),
            League::SSFHCNecropolis => write!(f, "HC SSF Necropolis"),
            League::Custom(league) => write!(f, "{}", league),
        }
    }
}

impl From<TradeLeague> for League {
    fn from(value: TradeLeague) -> Self {
        match value {
            TradeLeague::Standard => League::Standard,
            TradeLeague::Hardcore => League::Hardcore,
            TradeLeague::Necropolis => League::Necropolis,
            TradeLeague::HardcoreNecropolis => League::HardcoreNecropolis,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Hash, Eq, PartialEq, Default)]
pub enum TradeLeague {
    Standard,
    Hardcore,
    #[default]
    Necropolis,
    #[serde(alias = "Hardcore Necropolis", rename = "Hardcore Necropolis")]
    HardcoreNecropolis,
}

impl Display for TradeLeague {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TradeLeague::Standard => write!(f, "Standard"),
            TradeLeague::Hardcore => write!(f, "Hardcore"),
            TradeLeague::Necropolis => write!(f, "Necropolis"),
            TradeLeague::HardcoreNecropolis => write!(f, "Hardcore Necropolis"),
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
            League::Necropolis => Ok(TradeLeague::Necropolis),
            League::HardcoreNecropolis => Ok(TradeLeague::HardcoreNecropolis),
            League::SSFNecropolis => Err(msg),
            League::SSFHCNecropolis => Err(msg),
            League::Custom(_) => Err(msg),
        }
    }
}
