use super::super::id::{parseid, Identified, UnknownVariant};
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Default, EnumIter, PartialEq, Eq, Hash)]
pub enum Area {
    #[default]
    TrialOfStingingDoubt,
    TempleOfAtzoatl,
    AllVaalSideAreas,
    VaalSideAreas,
    RedeemerInfluencedMaps,
    ExpeditionLogbook,
    LabyrinthTrialAreas,
    AreaSpecific(AreaSpecific),
    AtziriArea(AtziriArea),
    UniqueHeistContractOrBoss(UniqueHeistContractOrBoss),
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
            Area::ExpeditionLogbook => "Expedition Logbook",
            Area::LabyrinthTrialAreas => "Labyrinth Trial Areas",
            Area::UniqueHeistContractOrBoss(_) => "Unique heist contract or boss",
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
            "Trial of Stinging Doubt" => Ok(Self::TrialOfStingingDoubt),
            "The Temple of Atzoatl" => Ok(Self::TempleOfAtzoatl),
            "Redeemer influenced maps" => Ok(Self::RedeemerInfluencedMaps),
            "All Vaal side areas" | "All Vaal side areas (need specific information)" => {
                Ok(Self::AllVaalSideAreas)
            }
            "Vaal Side Areas" => Ok(Self::VaalSideAreas),
            "Labyrinth Trial Areas" => Ok(Self::LabyrinthTrialAreas),
            "Expedition Logbook" | "Expedition Logbooks" => Ok(Self::ExpeditionLogbook),
            _ => AreaSpecific::from_str(s)
                .map(Self::AreaSpecific)
                .or_else(|_| AtziriArea::from_str(s).map(Self::AtziriArea))
                .or_else(|_| {
                    UniqueHeistContractOrBoss::from_str(s).map(Self::UniqueHeistContractOrBoss)
                })
                .map_err(|_| strum::ParseError::VariantNotFound),
        }
    }
}

impl Identified for Area {
    fn id(&self) -> &str {
        match self {
            Area::TrialOfStingingDoubt => "Trial of Stinging Doubt",
            Area::TempleOfAtzoatl => "The Temple of Atzoatl",
            Area::AllVaalSideAreas => "All Vaal side areas",
            Area::VaalSideAreas => "Vaal Side Areas",
            Area::RedeemerInfluencedMaps => "Redeemer influenced maps",
            Area::ExpeditionLogbook => "Expedition Logbook",
            Area::LabyrinthTrialAreas => "Labyrinth Trial Areas",
            Area::AreaSpecific(a) => a.id(),
            Area::AtziriArea(a) => a.id(),
            Area::UniqueHeistContractOrBoss(a) => a.id(),
        }
    }
}

#[derive(Debug, Clone, Default, EnumIter, PartialEq, Eq, Hash)]
pub enum AtziriArea {
    #[default]
    ApexOfSacrifice,
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

impl Identified for AtziriArea {
    fn id(&self) -> &str {
        match self {
            AtziriArea::ApexOfSacrifice => "The Apex of Sacrifice",
            AtziriArea::AlluringAbyss => "The Alluring Abyss",
        }
    }
}

impl FromStr for AtziriArea {
    type Err = UnknownVariant<Self>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parseid(s)
    }
}

#[derive(Debug, Clone, Default, EnumIter, PartialEq, Eq, Hash)]
pub enum AreaSpecific {
    #[default]
    ChayulasDomain,
    UulNetolsDomain,
    EshsDomain,
    XophsDomain,
    TulsDomain,
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

impl FromStr for AreaSpecific {
    type Err = UnknownVariant<Self>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parseid(s)
    }
}

#[derive(Debug, Clone, Default, EnumIter, PartialEq, Eq, Hash)]
pub enum UniqueHeistContractOrBoss {
    #[default]
    DeathToDarnaw,
    TheSlaverKing,
    HeartOfGlory,
    TheTwins,
    BreakingTheUnbreakable,
}

impl Identified for UniqueHeistContractOrBoss {
    fn id(&self) -> &str {
        match self {
            UniqueHeistContractOrBoss::DeathToDarnaw => "Contract: Death to Darnaw",
            UniqueHeistContractOrBoss::TheSlaverKing => "Contract: The Slaver King",
            UniqueHeistContractOrBoss::HeartOfGlory => "Contract: Heart of Glory",
            UniqueHeistContractOrBoss::TheTwins => "Contract: The Twins",
            UniqueHeistContractOrBoss::BreakingTheUnbreakable => {
                "Contract: Breaking the Unbreakable"
            }
        }
    }

    fn aliases(&self) -> Vec<&str> {
        match self {
            UniqueHeistContractOrBoss::DeathToDarnaw => vec!["Darnaw's Landing"],
            UniqueHeistContractOrBoss::TheSlaverKing => vec!["The Body Pit"],
            UniqueHeistContractOrBoss::HeartOfGlory => vec!["Pillaged Camp"],
            UniqueHeistContractOrBoss::TheTwins => vec!["The Den"],
            UniqueHeistContractOrBoss::BreakingTheUnbreakable => {
                vec!["Combat Capacity Test Chamber"]
            }
        }
    }
}

impl FromStr for UniqueHeistContractOrBoss {
    type Err = UnknownVariant<Self>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parseid(s)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn redeemer() {
        assert_eq!(
            "Redeemer influenced maps".parse::<Area>().unwrap(),
            Area::RedeemerInfluencedMaps
        )
    }

    #[test]
    fn heist_aliases() {
        assert_eq!(
            "The Den".parse::<Area>().unwrap(),
            Area::UniqueHeistContractOrBoss(UniqueHeistContractOrBoss::TheTwins)
        )
    }
}
