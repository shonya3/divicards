use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::IntoEnumIterator;

use strum_macros::EnumString;

use super::Identified;

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, strum_macros::EnumIter, Default,
)]
#[serde(tag = "type")]
pub enum Area {
    #[default]
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
    #[serde(rename = "Redeemer influenced maps")]
    RedeemerInfluencedMaps,
}

impl Identified for Area {
    fn id(&self) -> &str {
        match self {
            Area::TrialOfStingingDoubt => "Trial of Stinging Doubt",
            Area::TempleOfAtzoatl => "The Temple of Atzoatl",
            Area::AllVaalSideAreas => "All Vaal side areas (need specific information)",
            Area::VaalSideAreas => "Vaal Side Areas",
            Area::AtziriArea(a) => a.id(),
            Area::AreaSpecific(a) => a.id(),
            Area::RedeemerInfluencedMaps => "Redeemer influenced maps",
        }
    }
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
            Area::RedeemerInfluencedMaps => "Redeemer influenced maps",
        }
    }

    pub fn _types() -> impl Iterator<Item = String> {
        Area::iter().map(|a| a._type().to_string())
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
        write!(f, "{}", self.id())
        // match self {
        //     Area::TrialOfStingingDoubt => write!(f, "Trial of Stinging Doubt"),
        //     Area::TempleOfAtzoatl => write!(f, "The Temple of Atzoatl"),
        //     Area::AllVaalSideAreas => write!(f, "All Vaal side areas (need specific information)"),
        //     Area::VaalSideAreas => write!(f, "Vaal Side Areas"),
        //     Area::AreaSpecific(areaspecific) => areaspecific.fmt(f),
        //     Area::AtziriArea(atziri_area) => atziri_area.fmt(f),
        // }
    }
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Hash,
    EnumString,
    strum_macros::Display,
    strum_macros::EnumIter,
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

impl Default for AtziriArea {
    fn default() -> Self {
        AtziriArea::AlluringAbyss
    }
}

impl Identified for AtziriArea {
    fn id(&self) -> &str {
        match self {
            AtziriArea::ApexOfSacrifice => "The Apex of Sacrifice",
            AtziriArea::AlluringAbyss => "The Alluring Abyss",
        }
    }
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Hash,
    EnumString,
    strum_macros::Display,
    strum_macros::EnumIter,
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

impl Identified for BreachlordBossDomain {
    fn id(&self) -> &str {
        match self {
            BreachlordBossDomain::Xoph => "Xoph, Dark Embers",
            BreachlordBossDomain::Tul => "Tul, Creeping Avalanche",
            BreachlordBossDomain::Esh => "Esh, Forked Thought",
            BreachlordBossDomain::Chayula => "Chayula, Who Dreamt",
            BreachlordBossDomain::UulNetol => "Uul-Netol, Unburdened Flesh",
        }
    }
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Hash,
    EnumString,
    strum_macros::Display,
    strum_macros::EnumIter,
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

impl Default for AreaSpecific {
    fn default() -> Self {
        AreaSpecific::ChayulasDomain
    }
}

impl Identified for AreaSpecific {
    fn id(&self) -> &str {
        match self {
            AreaSpecific::ChayulasDomain => "Chayula's Domain",
            AreaSpecific::UulNetolsDomain => "Uul-Netol's Domain",
            AreaSpecific::EshsDomain => "Esh's Domain",
            AreaSpecific::XophsDomain => "Xoph's Domain",
            AreaSpecific::TulsDomain => "Tul's Domain",
        }
    }
}
