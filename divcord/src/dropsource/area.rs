use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(tag = "type")]
pub enum Area {
    #[serde(rename = "Trial of Stinging Doubt")]
    TrialOfStingingDoubt,
    #[serde(rename = "The Temple of Atzoatl")]
    TempleOfAtzoatl,
    #[serde(rename = "All Vaal side areas (need specific information)")]
    AllVaalSideAreas,
    #[serde(rename = "Vaal Side Areas")]
    VaalSideAreas,
    #[serde(rename = "Atziri Area")]
    AtziriArea(AtziriArea),
    #[serde(rename = "Area-Specific")]
    AreaSpecific(AreaSpecific),
}

impl Area {
    pub fn _type(&self) -> &str {
        match self {
            Area::TrialOfStingingDoubt => "Trial of Stinging Doubt",
            Area::TempleOfAtzoatl => "The Temple of Atzoatl",
            Area::AllVaalSideAreas => "All Vaal side areas (need specific information)",
            Area::VaalSideAreas => "Vaal Side Areas",
            Area::AtziriArea(_) => "Atziri Area",
            Area::AreaSpecific(_) => "Area-Specific",
        }
    }
}

impl FromStr for Area {
    type Err = strum::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Trial of Stinging Doubt" => return Ok(Area::TrialOfStingingDoubt),
            "The Temple of Atzoatl" => return Ok(Area::TempleOfAtzoatl),
            "All Vaal side areas (need specific information)" => return Ok(Area::AllVaalSideAreas),
            "Vaal Side Areas" => return Ok(Area::VaalSideAreas),
            _ => {
                if let Ok(areaspecific) = s.parse::<AreaSpecific>() {
                    return Ok(Area::AreaSpecific(areaspecific));
                } else if let Ok(atziri_area) = s.parse::<AtziriArea>() {
                    return Ok(Area::AtziriArea(atziri_area));
                };
            }
        };

        Err(strum::ParseError::VariantNotFound)
    }
}

impl std::fmt::Display for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Area::TrialOfStingingDoubt => write!(f, "Trial of Stinging Doubt"),
            Area::TempleOfAtzoatl => write!(f, "The Temple of Atzoatl"),
            Area::AllVaalSideAreas => write!(f, "All Vaal side areas (need specific information)"),
            Area::VaalSideAreas => write!(f, "Vaal Side Areas"),
            Area::AreaSpecific(areaspecific) => areaspecific.fmt(f),
            Area::AtziriArea(atziri_area) => atziri_area.fmt(f),
        }
    }
}

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, EnumString, strum_macros::Display,
)]
#[serde(tag = "name")]
pub enum AtziriArea {
    #[strum(to_string = "The Apex of Sacrifice")]
    #[serde(rename = "The Apex of Sacrifice")]
    ApexOfSacrifice,
    #[strum(to_string = "The Alluring Abyss")]
    #[serde(rename = "The Alluring Abyss")]
    AlluringAbyss,
}

impl AtziriArea {
    pub fn level(&self) -> u32 {
        match self {
            AtziriArea::ApexOfSacrifice => 70,
            AtziriArea::AlluringAbyss => 80,
        }
    }
}

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, EnumString, strum_macros::Display,
)]
#[serde(tag = "name")]
pub enum BreachlordBossDomain {
    #[strum(serialize = "Xoph, Dark Embers")]
    #[serde(rename = "Xoph, Dark Embers")]
    Xoph,
    #[strum(serialize = "Tul, Creeping Avalanche")]
    #[serde(rename = "Tul, Creeping Avalanche")]
    Tul,
    #[strum(serialize = "Esh, Forked Thought")]
    #[serde(rename = "Esh, Forked Thought")]
    Esh,
    #[strum(serialize = "Chayula, Who Dreamt")]
    #[serde(rename = "Chayula, Who Dreamt")]
    Chayula,
    #[strum(serialize = "Uul-Netol, Unburdened Flesh")]
    #[serde(rename = "Uul-Netol, Unburdened Flesh")]
    UulNetol,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, EnumString, strum_macros::Display,
)]
#[serde(tag = "name")]
pub enum AreaSpecific {
    #[strum(serialize = "Chayula's Domain")]
    #[serde(rename = "Chayula's Domain")]
    ChayulasDomain,
    #[strum(serialize = "Uul-Netol's Domain")]
    #[serde(rename = "Uul-Netol's Domain")]
    UulNetolsDomain,
    #[strum(serialize = "Esh's Domain")]
    #[serde(rename = "Esh's Domain")]
    EshsDomain,
    #[strum(serialize = "Xoph's Domain")]
    #[serde(rename = "Xoph's Domain")]
    XophsDomain,
    #[strum(serialize = "Tul's Domain")]
    #[serde(rename = "Tul's Domain")]
    TulsDomain,
}
