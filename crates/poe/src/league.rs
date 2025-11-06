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
    Keepers,
    #[serde(alias = "Hardcore Keepers")]
    HardcoreKeepers,
    #[serde(alias = "SSF Keepers")]
    SSFKeepers,
    #[serde(alias = "HC SSF Keepers")]
    SSFHCKeepers,
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
            League::Keepers => write!(f, "Keepers"),
            League::HardcoreKeepers => write!(f, "Hardcore Keepers"),
            League::SSFKeepers => write!(f, "SSF Keepers"),
            League::SSFHCKeepers => write!(f, "HC SSF Keepers"),
            League::Custom(league) => write!(f, "{league}"),
        }
    }
}

impl From<TradeLeague> for League {
    fn from(value: TradeLeague) -> Self {
        match value {
            TradeLeague::Standard => League::Standard,
            TradeLeague::Hardcore => League::Hardcore,
            TradeLeague::Keepers => League::Keepers,
            TradeLeague::HardcoreKeepers => League::HardcoreKeepers,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Hash, Eq, PartialEq, Default)]
pub enum TradeLeague {
    Standard,
    Hardcore,
    #[default]
    Keepers,
    #[serde(alias = "Hardcore Keepers", rename = "Hardcore Keepers")]
    HardcoreKeepers,
}

impl Display for TradeLeague {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TradeLeague::Standard => write!(f, "Standard"),
            TradeLeague::Hardcore => write!(f, "Hardcore"),
            TradeLeague::Keepers => write!(f, "Keepers"),
            TradeLeague::HardcoreKeepers => write!(f, "Hardcore Keepers"),
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
            League::Keepers => Ok(TradeLeague::Keepers),
            League::HardcoreKeepers => Ok(TradeLeague::HardcoreKeepers),
            League::SSFKeepers => Err(msg),
            League::SSFHCKeepers => Err(msg),
            League::Custom(_) => Err(msg),
        }
    }
}
