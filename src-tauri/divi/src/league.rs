use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum League {
    Crucible,
    Standard,
    #[serde(alias = "Crucible-HC")]
    HardcoreCrucible,
    Hardcore,
    #[serde(alias = "SSF Standard")]
    SSFStandard,
    #[serde(alias = "SSF Hardcore")]
    SSFHardcore,
    #[serde(alias = "SSF Crucible")]
    SSFCrucible,
    #[serde(alias = "HC SSF Crucible")]
    SSFHCCrucible,
}

impl Display for League {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            League::Crucible => write!(f, "Crucible"),
            League::Standard => write!(f, "Standard"),
            League::HardcoreCrucible => write!(f, "Hardcore Crucible"),
            League::Hardcore => write!(f, "Hardcore"),
            League::SSFStandard => write!(f, "Solo Self-Found"),
            League::SSFHardcore => write!(f, "Hardcore SSF"),
            League::SSFCrucible => write!(f, "SSF Crucible"),
            League::SSFHCCrucible => write!(f, "HC SSF Crucible"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TradeLeague {
    Crucible,
    Standard,
    #[serde(alias = "Crucible-HC")]
    HardcoreCrucible,
    Hardcore,
}

impl Display for TradeLeague {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TradeLeague::Crucible => write!(f, "Crucible"),
            TradeLeague::Standard => write!(f, "Standard"),
            TradeLeague::HardcoreCrucible => write!(f, "Hardcore Crucible"),
            TradeLeague::Hardcore => write!(f, "Hardcore"),
        }
    }
}
