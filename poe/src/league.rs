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
    Mercenaries,
    #[serde(alias = "Hardcore Mercenaries")]
    HardcoreMercenaries,
    #[serde(alias = "SSF Mercenaries")]
    SSFMercenaries,
    #[serde(alias = "HC SSF Mercenaries")]
    SSFHCMercenaries,
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
            League::Mercenaries => write!(f, "Mercenaries"),
            League::HardcoreMercenaries => write!(f, "Hardcore Mercenaries"),
            League::SSFMercenaries => write!(f, "SSF Mercenaries"),
            League::SSFHCMercenaries => write!(f, "HC SSF Mercenaries"),
            League::Custom(league) => write!(f, "{}", league),
        }
    }
}

impl From<TradeLeague> for League {
    fn from(value: TradeLeague) -> Self {
        match value {
            TradeLeague::Standard => League::Standard,
            TradeLeague::Hardcore => League::Hardcore,
            TradeLeague::Mercenaries => League::Mercenaries,
            TradeLeague::HardcoreMercenaries => League::HardcoreMercenaries,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Hash, Eq, PartialEq, Default)]
pub enum TradeLeague {
    Standard,
    Hardcore,
    #[default]
    Mercenaries,
    #[serde(alias = "Hardcore Mercenaries", rename = "Hardcore Mercenaries")]
    HardcoreMercenaries,
}

impl Display for TradeLeague {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TradeLeague::Standard => write!(f, "Standard"),
            TradeLeague::Hardcore => write!(f, "Hardcore"),
            TradeLeague::Mercenaries => write!(f, "Mercenaries"),
            TradeLeague::HardcoreMercenaries => write!(f, "Hardcore Mercenaries"),
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
            League::Mercenaries => Ok(TradeLeague::Mercenaries),
            League::HardcoreMercenaries => Ok(TradeLeague::HardcoreMercenaries),
            League::SSFMercenaries => Err(msg),
            League::SSFHCMercenaries => Err(msg),
            League::Custom(_) => Err(msg),
        }
    }
}
