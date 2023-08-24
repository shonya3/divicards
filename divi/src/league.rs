use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub enum League {
    Standard,
    Hardcore,
    #[serde(alias = "SSF Standard")]
    SSFStandard,
    #[serde(alias = "SSF Hardcore")]
    SSFHardcore,
    Ancestor,
    #[serde(alias = "Hardcore Ancestor")]
    HardcoreAncestor,
    #[serde(alias = "SSF Ancestor")]
    SSFAncestor,
    #[serde(alias = "HC SSF Ancestor")]
    SSFHCAncestor,
}

impl League {
    pub fn is_trade(&self) -> bool {
        match self {
            League::Standard => true,
            League::Hardcore => true,
            League::SSFStandard => false,
            League::SSFHardcore => false,
            League::Ancestor => true,
            League::HardcoreAncestor => true,
            League::SSFAncestor => false,
            League::SSFHCAncestor => false,
        }
    }
}

impl Default for League {
    fn default() -> Self {
        League::Ancestor
    }
}

impl Display for League {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            League::Standard => write!(f, "Standard"),
            League::Hardcore => write!(f, "Hardcore"),
            League::SSFStandard => write!(f, "Solo Self-Found"),
            League::SSFHardcore => write!(f, "Hardcore SSF"),
            League::Ancestor => write!(f, "Ancestor"),
            League::HardcoreAncestor => write!(f, "Hardcore Ancestor"),
            League::SSFAncestor => write!(f, "SSF Ancestor"),
            League::SSFHCAncestor => write!(f, "HC SSF Ancestor"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Hash, Eq, PartialEq)]
pub enum TradeLeague {
    Standard,
    Hardcore,
    Ancestor,
    #[serde(alias = "Hardcore Ancestor", rename = "Hardcore Ancestor")]
    HardcoreAncestor,
}

impl Display for TradeLeague {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TradeLeague::Standard => write!(f, "Standard"),
            TradeLeague::Hardcore => write!(f, "Hardcore"),
            TradeLeague::Ancestor => write!(f, "Ancestor"),
            TradeLeague::HardcoreAncestor => write!(f, "Hardcore Ancestor"),
        }
    }
}
impl Default for TradeLeague {
    fn default() -> Self {
        TradeLeague::Ancestor
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
            League::Ancestor => Ok(TradeLeague::Ancestor),
            League::HardcoreAncestor => Ok(TradeLeague::HardcoreAncestor),
            League::SSFAncestor => Err(msg),
            League::SSFHCAncestor => Err(msg),
        }
    }
}
