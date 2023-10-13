use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum_macros::EnumString;

use crate::maps::Map;

use super::dropconsts::{ACT_AREA_NAMES, AREA_NAMES};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum Area {
    #[serde(alias = "Trial of Stinging Doubt")]
    TrialOfStingingDoubt,
    #[serde(alias = "The Temple of Atzoatl")]
    TempleOfAtzoatl,
    #[serde(alias = "All Vaal side areas (need specific information)")]
    AllVaalSideAreas,
    #[serde(alias = "Vaal Side Areas")]
    VaalSideAreas,
    AreaSpecific(AreaSpecific),
    Map {
        name: String,
    },
    Acts {
        name: String,
    },
}

impl FromStr for Area {
    type Err = strum::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let maps: Vec<Map> =
            serde_json::from_str(&std::fs::read_to_string("jsons/maps.json").unwrap()).unwrap();
        let maps: Vec<String> = maps.into_iter().map(|m| m.name).collect();
        let maps_without_mapword: Vec<String> =
            maps.iter().map(|m| m.replace(" Map", "")).collect();

        match s {
            "Trial of Stinging Doubt" => return Ok(Area::TrialOfStingingDoubt),
            "The Temple of Atzoatl" => return Ok(Area::TempleOfAtzoatl),
            "All Vaal side areas (need specific information)" => return Ok(Area::AllVaalSideAreas),
            "Vaal Side Areas" => return Ok(Area::VaalSideAreas),
            _ => {}
        };

        let s = s.to_string();

        if maps_without_mapword.iter().any(|map| map == &s.as_str())
            || AREA_NAMES.iter().any(|area| area == &s.as_str())
            || maps.iter().any(|map| map == &s.as_str())
        {
            return Ok(Area::Map { name: s });
        } else if let Ok(areaspecific) = s.parse::<AreaSpecific>() {
            return Ok(Area::AreaSpecific(areaspecific));
        }

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
            Area::Map { name } => name.fmt(f),
            Area::Acts { name } => name.fmt(f),
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
