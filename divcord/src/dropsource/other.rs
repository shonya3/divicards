use super::id::{parseid, Identified, UnknownVariant};
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
    DelveChest,
    DelveGemChests,
    VoltaxicSulphite,
    DelveInteractablesBehindFracturedWall,
    DelveCityLightJewelleryChest,
    MavenCrucible,
    HeistMapChest,
    BreachClaspedHand,
    IzaroTreasure,
    VaalVessel,
    UberlabChests,
    MercilessChests,
    HiddenCoffer,
    Darkshrine,
    BootyChestMaoKun,
}

impl Identified for Chest {
    fn id(&self) -> &str {
        match self {
            Chest::AbyssalTrove => "Abyssal Trove",
            Chest::DelveChest => "Delve chest",
            Chest::DelveGemChests => "Delve Gem Chests",
            Chest::VoltaxicSulphite => "Voltaxic Sulphite",
            Chest::DelveInteractablesBehindFracturedWall => "Delve Interactables behind Fractured Wall",
            Chest::DelveCityLightJewelleryChest => "Light Jewellery chest (Primeval Ruins, Abyssal City, Vaal Outpost)",
            Chest::MavenCrucible => "The Maven's Crucible",
            Chest::HeistMapChest => "Map Reward Heist Chests",
            Chest::BreachClaspedHand => "Breach Clasped Hand",
            Chest::IzaroTreasure => "Izaro's Treasure",
            Chest::VaalVessel => "Vaal Vessel (Vaal Side Areas)",
            Chest::UberlabChests => "Uber Labyrinth or Enriched Labyrinth (Izaro's Treasure, Labyrinth Trove, Curious Lockbox)",
            Chest::Darkshrine => "Labyrinth Darkshrines",
            Chest::BootyChestMaoKun => "Booty Chest (Mao Kun)",
            Chest::MercilessChests => "Merciless Labyrinth (Izaro's Treasure, Labyrinth Trove, Curious Lockbox, Hidden Coffer)",
            Chest::HiddenCoffer => "Hidden Coffer",
        }
    }

    fn aliases(&self) -> Vec<&str> {
        match self {
            Chest::AbyssalTrove => vec![],
            Chest::DelveChest => vec![],
            Chest::DelveGemChests => vec![],
            Chest::VoltaxicSulphite => vec![],
            Chest::DelveInteractablesBehindFracturedWall => vec![],
            Chest::DelveCityLightJewelleryChest => vec![],
            Chest::MavenCrucible => vec![],
            Chest::HeistMapChest => vec![],
            Chest::BreachClaspedHand => vec![],
            Chest::IzaroTreasure => vec![],
            Chest::VaalVessel => vec![],
            Chest::UberlabChests => vec!["Uber Labyrinth/Enriched Labyrinth (Izaro's Treasure, Labyrinth Trove, Curious Lockbox)"],
            Chest::MercilessChests => vec!["Merciless Labyrinth"],
            Chest::HiddenCoffer => vec![],
            Chest::Darkshrine => vec![],
            Chest::BootyChestMaoKun => vec![],
        }
    }
}

impl FromStr for Chest {
    type Err = UnknownVariant<Self>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parseid(s)
    }
}
