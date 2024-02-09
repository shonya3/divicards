use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub enum League {
    Standard,
    Hardcore,
    #[serde(alias = "Solo Self-Found", alias = "SSF Standard")]
    SSFStandard,
    #[serde(alias = "SSF Hardcore", alias = "Hardcore SSF")]
    SSFHardcore,
    Affliction,
    #[serde(alias = "Hardcore Affliction")]
    HardcoreAffliction,
    #[serde(alias = "SSF Affliction")]
    SSFAffliction,
    #[serde(alias = "HC SSF Affliction")]
    SSFHCAffliction,
    #[serde(untagged)]
    Custom(String),
}

impl League {
    pub fn is_trade(&self) -> bool {
        match TradeLeague::try_from(self.to_owned()) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

impl Default for League {
    fn default() -> Self {
        League::Affliction
    }
}

impl Display for League {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            League::Standard => write!(f, "Standard"),
            League::Hardcore => write!(f, "Hardcore"),
            League::SSFStandard => write!(f, "Solo Self-Found"),
            League::SSFHardcore => write!(f, "Hardcore SSF"),
            League::Affliction => write!(f, "Affliction"),
            League::HardcoreAffliction => write!(f, "Hardcore Affliction"),
            League::SSFAffliction => write!(f, "SSF Affliction"),
            League::SSFHCAffliction => write!(f, "HC SSF Affliction"),
            League::Custom(league) => write!(f, "{}", league),
        }
    }
}

impl From<TradeLeague> for League {
    fn from(value: TradeLeague) -> Self {
        match value {
            TradeLeague::Standard => League::Standard,
            TradeLeague::Hardcore => League::Hardcore,
            TradeLeague::Affliction => League::Affliction,
            TradeLeague::HardcoreAffliction => League::HardcoreAffliction,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Hash, Eq, PartialEq)]
pub enum TradeLeague {
    Standard,
    Hardcore,
    Affliction,
    #[serde(alias = "Hardcore Affliction", rename = "Hardcore Affliction")]
    HardcoreAffliction,
}

impl Display for TradeLeague {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TradeLeague::Standard => write!(f, "Standard"),
            TradeLeague::Hardcore => write!(f, "Hardcore"),
            TradeLeague::Affliction => write!(f, "Affliction"),
            TradeLeague::HardcoreAffliction => write!(f, "Hardcore Affliction"),
        }
    }
}
impl Default for TradeLeague {
    fn default() -> Self {
        TradeLeague::Affliction
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
            League::Affliction => Ok(TradeLeague::Affliction),
            League::HardcoreAffliction => Ok(TradeLeague::HardcoreAffliction),
            League::SSFAffliction => Err(msg),
            League::SSFHCAffliction => Err(msg),
            League::Custom(_) => Err(msg),
        }
    }
}
