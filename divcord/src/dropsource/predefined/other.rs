use super::super::id::{parseid, Identified, UnknownVariant};
use std::str::FromStr;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Default, EnumIter, PartialEq, Eq, Hash)]
pub enum Vendor {
    #[default]
    KiracShop,
}

impl Identified for Vendor {
    fn id(&self) -> &str {
        match self {
            Vendor::KiracShop => "Kirac shop",
        }
    }
}

impl FromStr for Vendor {
    type Err = UnknownVariant<Self>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parseid(s)
    }
}

#[derive(Debug, Clone, Default, EnumIter, PartialEq, Eq, Hash)]
pub enum Strongbox {
    #[default]
    Jeweller,
    Armourer,
    Cartographer,
    Gemcutter,
    Arcanist,
    Artisan,
}

impl Identified for Strongbox {
    fn id(&self) -> &str {
        match self {
            Strongbox::Jeweller => "Jeweller's Strongbox",
            Strongbox::Armourer => "Armourer's Strongbox",
            Strongbox::Cartographer => "Cartographer's Strongbox",
            Strongbox::Gemcutter => "Gemcutter's Strongbox",
            Strongbox::Arcanist => "Arcanist's Strongbox",
            Strongbox::Artisan => "Artisan's Strongbox",
        }
    }
}

impl FromStr for Strongbox {
    type Err = UnknownVariant<Self>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parseid(s)
    }
}

#[derive(Debug, Clone, Default, EnumIter, PartialEq, Eq, Hash)]
pub enum Chest {
    #[default]
    AbyssalTrove,
    Delve,
    DelveGemChests,
    VoltaxicSulphite,
    DelveInteractablesBehindFracturedWall,
    DelveCityLightJewellery,
    MavenCrucible,
    HeistMap,
    BreachClaspedHand,
    IzaroTreasure,
    VaalVessel,
    UberlabChests,
    MercilessChests,
    HiddenCoffer,
    Darkshrine,
    BootyChestMaoKun,
    MercilessOrUberOrEnrichedLabChests,
}

impl Identified for Chest {
    fn id(&self) -> &str {
        match self {
            Chest::AbyssalTrove => "Abyssal Trove",
            Chest::Delve => "Delve chest",
            Chest::DelveGemChests => "Delve Gem Chests",
            Chest::VoltaxicSulphite => "Voltaxic Sulphite",
            Chest::DelveInteractablesBehindFracturedWall => "Delve Interactables behind Fractured Wall",
            Chest::DelveCityLightJewellery => "Light Jewellery chest (Primeval Ruins, Abyssal City, Vaal Outpost)",
            Chest::MavenCrucible => "The Maven's Crucible",
            Chest::HeistMap => "Map Reward Heist Chests",
            Chest::BreachClaspedHand => "Breach Clasped Hand",
            Chest::IzaroTreasure => "Izaro's Treasure",
            Chest::VaalVessel => "Vaal Vessel (Vaal Side Areas)",
            Chest::UberlabChests => "Uber Labyrinth or Enriched Labyrinth (Izaro's Treasure, Labyrinth Trove, Curious Lockbox)",
            Chest::Darkshrine => "Labyrinth Darkshrines",
            Chest::BootyChestMaoKun => "Booty Chest (Mao Kun)",
            Chest::MercilessChests => "Merciless Labyrinth (Izaro's Treasure, Labyrinth Trove, Curious Lockbox, Hidden Coffer)",
            Chest::HiddenCoffer => "Hidden Coffer",
            Chest::MercilessOrUberOrEnrichedLabChests => "Izaro's Treasure, Labyrinth Trove, Curious Lockbox (Merciless/Uber/Enriched Labyrinth)",
        }
    }

    fn aliases(&self) -> Vec<&str> {
        match self {
            Chest::AbyssalTrove => vec![],
            Chest::Delve => vec!["Delve Chest"],
            Chest::DelveGemChests => vec![],
            Chest::VoltaxicSulphite => vec![],
            Chest::DelveInteractablesBehindFracturedWall => vec![],
            Chest::DelveCityLightJewellery => vec![],
            Chest::MavenCrucible => vec![],
            Chest::HeistMap => vec![],
            Chest::BreachClaspedHand => vec![],
            Chest::IzaroTreasure => vec![],
            Chest::VaalVessel => vec![],
            Chest::UberlabChests => vec!["Uber Labyrinth/Enriched Labyrinth (Izaro's Treasure, Labyrinth Trove, Curious Lockbox)"],
            Chest::MercilessChests => vec!["Merciless Labyrinth"],
            Chest::HiddenCoffer => vec![],
            Chest::Darkshrine => vec![],
            Chest::BootyChestMaoKun => vec![],
            Chest::MercilessOrUberOrEnrichedLabChests => vec!["Merciless Labyrinth/Uber Labyrinth/Enriched Labyrinth (Izaro's Treasure, Labyrinth Trove, Curious Lockbox)"],
        }
    }
}

impl FromStr for Chest {
    type Err = UnknownVariant<Self>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parseid(s)
    }
}
