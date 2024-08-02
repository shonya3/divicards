use super::id::{parseid, Identified, UnknownVariant};
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Default, EnumIter, PartialEq, Eq, Hash)]
pub enum UniqueMonster {
    #[default]
    MavensInvitationTheFeared,
    UulNetolInBreachstones,
    VaalOmnitect,
    Metamorph,
    NullPortal,
    VaalFleshMerchant,
    AllIncursionArchitectsInAlvaMission,
    AllIncursionArchitectsInTemple,
    AllAbyssMonsters,
    AllScourgeBeyondDemons,
    AllRogueExiles,
    CortexVenarius,
    Argus,
    AllInvasionBosses,
    AllVaalSideAreaBosses,
    BreachlordBossDomain(BreachlordBossDomain),
    Architect(Architect),
    ShaperGuardianBoss(ShaperGuardianBoss),
    SyndicateMember(SyndicateMember),
    Elderslayer(Elderslayer),
    ElderGuardianBoss(ElderGuardianBoss),
    RogueExile(RogueExile),
    FemaleRogueExile(FemaleRogueExile),
    AbyssLichBoss(AbyssLichBoss),
    MapsOnly(MapsOnly),
    HarbingerPortal(HarbingerPortal),
    EndgameBoss(EndgameBoss),
    DelveBoss(DelveBoss),
    BeastBoss(BeastBoss),
    HeistBoss(HeistBoss),
    BeyondBoss(BeyondBoss),
    ExpeditionLogbookBoss(ExpeditionLogbookBoss),
    BetrayalCatarina(BetrayalCatarina),
    OshabiBoss(OshabiBoss),
    EldritchPerfectionMonster(EldritchPerfectionMonster),
    ShaperMiniBoss(ShaperMiniBoss),
}

impl FromStr for UniqueMonster {
    type Err = strum::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Maven's Invitation: The Feared" => Ok(Self::MavensInvitationTheFeared),
            "Uul-Netol, Unburdened Flesh (in Breachstones)" => Ok(Self::UulNetolInBreachstones),
            "The Vaal Omnitect" => Ok(Self::VaalOmnitect),
            "Metamorph" => Ok(Self::Metamorph),
            "Null Portal" => Ok(Self::NullPortal),
            "Vaal Flesh Merchant" => Ok(Self::VaalFleshMerchant),
            "All Incursion Architects in Alva missions/Alva's Memory"
            | "All Incursion Architects in Alva missions or Alva's Memory" => {
                Ok(Self::AllIncursionArchitectsInAlvaMission)
            }
            "All Incursion Architects (The Temple of Atzoatl)"
            | "Architects in Temple of Atzoatl" => Ok(Self::AllIncursionArchitectsInTemple),
            "All Abyss Monsters" => Ok(Self::AllAbyssMonsters),
            "All Rogue Exiles" => Ok(Self::AllRogueExiles),
            "All Invasion Bosses" => Ok(Self::AllInvasionBosses),
            "All Vaal Side Area Bosses" | "Vaal side area bosses (including Vaal Temple)" => {
                Ok(Self::AllVaalSideAreaBosses)
            }
            "All (Scourge) beyond demons"
            | "All (Scourge) beyond demons, including from Settlers Crimson Iron/Orichalcum"
            | "All (Scourge) beyond demons, including from Settlers (Crimson Iron/Orichalcum)" => {
                Ok(Self::AllScourgeBeyondDemons)
            }
            "Venarius" => Ok(Self::CortexVenarius),
            "Argus" => Ok(Self::Argus),
            _ => BreachlordBossDomain::from_str(s)
                .map(Self::BreachlordBossDomain)
                .or_else(|_| Architect::from_str(s).map(Self::Architect))
                .or_else(|_| ShaperGuardianBoss::from_str(s).map(Self::ShaperGuardianBoss))
                .or_else(|_| SyndicateMember::from_str(s).map(Self::SyndicateMember))
                .or_else(|_| Elderslayer::from_str(s).map(Self::Elderslayer))
                .or_else(|_| ElderGuardianBoss::from_str(s).map(Self::ElderGuardianBoss))
                .or_else(|_| RogueExile::from_str(s).map(Self::RogueExile))
                .or_else(|_| FemaleRogueExile::from_str(s).map(Self::FemaleRogueExile))
                .or_else(|_| AbyssLichBoss::from_str(s).map(Self::AbyssLichBoss))
                .or_else(|_| MapsOnly::from_str(s).map(Self::MapsOnly))
                .or_else(|_| HarbingerPortal::from_str(s).map(Self::HarbingerPortal))
                .or_else(|_| EndgameBoss::from_str(s).map(Self::EndgameBoss))
                .or_else(|_| DelveBoss::from_str(s).map(Self::DelveBoss))
                .or_else(|_| BeastBoss::from_str(s).map(Self::BeastBoss))
                .or_else(|_| HeistBoss::from_str(s).map(Self::HeistBoss))
                .or_else(|_| BeyondBoss::from_str(s).map(Self::BeyondBoss))
                .or_else(|_| ExpeditionLogbookBoss::from_str(s).map(Self::ExpeditionLogbookBoss))
                .or_else(|_| ShaperMiniBoss::from_str(s).map(Self::ShaperMiniBoss))
                .or_else(|_| BetrayalCatarina::from_str(s).map(Self::BetrayalCatarina))
                .or_else(|_| OshabiBoss::from_str(s).map(Self::OshabiBoss))
                .or_else(|_| {
                    EldritchPerfectionMonster::from_str(s).map(Self::EldritchPerfectionMonster)
                })
                .map_err(|_| strum::ParseError::VariantNotFound),
        }
    }
}

impl Identified for UniqueMonster {
    fn id(&self) -> &str {
        match self {
            UniqueMonster::MavensInvitationTheFeared => "Maven's Invitation: The Feared",
            UniqueMonster::UulNetolInBreachstones => {
                "Uul-Netol, Unburdened Flesh (in Breachstones)"
            }
            UniqueMonster::VaalOmnitect => "The Vaal Omnitect",
            UniqueMonster::Metamorph => "Metamorph",
            UniqueMonster::NullPortal => "Null Portal",
            UniqueMonster::VaalFleshMerchant => "Vaal Flesh Merchant",
            UniqueMonster::AllIncursionArchitectsInAlvaMission => {
                "All Incursion Architects in Alva missions or Alva's Memory"
            }
            UniqueMonster::AllIncursionArchitectsInTemple => {
                "All Incursion Architects (The Temple of Atzoatl)"
            }
            UniqueMonster::AllAbyssMonsters => "All Abyss Monsters",
            UniqueMonster::AllScourgeBeyondDemons => "All (Scourge) beyond demons",
            UniqueMonster::AllRogueExiles => "All Rogue Exiles",
            UniqueMonster::CortexVenarius => "Venarius",
            UniqueMonster::Argus => "Argus",
            UniqueMonster::AllInvasionBosses => "All Invasion Bosses",
            UniqueMonster::AllVaalSideAreaBosses => "All Vaal Side Area Bosses",
            UniqueMonster::BreachlordBossDomain(m) => m.id(),
            UniqueMonster::Architect(m) => m.id(),
            UniqueMonster::ShaperGuardianBoss(m) => m.id(),
            UniqueMonster::SyndicateMember(m) => m.id(),
            UniqueMonster::Elderslayer(m) => m.id(),
            UniqueMonster::ElderGuardianBoss(m) => m.id(),
            UniqueMonster::RogueExile(m) => m.id(),
            UniqueMonster::FemaleRogueExile(m) => m.id(),
            UniqueMonster::AbyssLichBoss(m) => m.id(),
            UniqueMonster::MapsOnly(m) => m.id(),
            UniqueMonster::HarbingerPortal(m) => m.id(),
            UniqueMonster::EndgameBoss(m) => m.id(),
            UniqueMonster::DelveBoss(m) => m.id(),
            UniqueMonster::BeastBoss(m) => m.id(),
            UniqueMonster::HeistBoss(m) => m.id(),
            UniqueMonster::BeyondBoss(m) => m.id(),
            UniqueMonster::ExpeditionLogbookBoss(m) => m.id(),
            UniqueMonster::ShaperMiniBoss(m) => m.id(),
            UniqueMonster::BetrayalCatarina(m) => m.id(),
            UniqueMonster::OshabiBoss(m) => m.id(),
            UniqueMonster::EldritchPerfectionMonster(m) => m.id(),
        }
    }
}

impl std::fmt::Display for UniqueMonster {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.id())
    }
}

impl UniqueMonster {
    pub fn _types() -> impl Iterator<Item = String> {
        UniqueMonster::iter().map(|a| a._type().to_string())
    }

    pub fn _type(&self) -> &str {
        match self {
            UniqueMonster::MavensInvitationTheFeared => "Maven's Invitation: The Feared",
            UniqueMonster::UulNetolInBreachstones => {
                "Uul-Netol, Unburdened Flesh (in Breachstones)"
            }
            UniqueMonster::VaalOmnitect => "The Vaal Omnitect",
            UniqueMonster::Metamorph => "Metamorph",
            UniqueMonster::NullPortal => "Null Portal",
            UniqueMonster::VaalFleshMerchant => "Vaal Flesh Merchant",
            UniqueMonster::AllIncursionArchitectsInAlvaMission => {
                "All Incursion Architects in Alva missions or Alva's Memory"
            }
            UniqueMonster::AllAbyssMonsters => "All Abyss Monsters",
            UniqueMonster::AllScourgeBeyondDemons => "All (Scourge) beyond demons",
            UniqueMonster::AllRogueExiles => "All Rogue Exiles",
            UniqueMonster::CortexVenarius => "Venarius",
            UniqueMonster::Argus => "Argus",
            UniqueMonster::AllInvasionBosses => "All Invasion Bosses",
            UniqueMonster::AllVaalSideAreaBosses => "All Vaal Side Area Bosses",
            UniqueMonster::BreachlordBossDomain(_) => "Breachlord Boss Domain",
            UniqueMonster::Architect(_) => "Architect",
            UniqueMonster::ShaperGuardianBoss(_) => "Shaper Guardian Boss",
            UniqueMonster::SyndicateMember(_) => "Syndicate Member",
            UniqueMonster::Elderslayer(_) => "Elder Slayer",
            UniqueMonster::ElderGuardianBoss(_) => "Elder Guardian Boss",
            UniqueMonster::RogueExile(_) => "Rogue Exile",
            UniqueMonster::FemaleRogueExile(_) => "Female Rogue Exile",
            UniqueMonster::AbyssLichBoss(_) => "Abyss Lich Boss",
            UniqueMonster::MapsOnly(_) => "Maps Only",
            UniqueMonster::HarbingerPortal(_) => "Harbinger Portal",
            UniqueMonster::EndgameBoss(_) => "Endgame Boss",
            UniqueMonster::DelveBoss(_) => "Delve Boss",
            UniqueMonster::BeastBoss(_) => "Beast Boss",
            UniqueMonster::HeistBoss(_) => "Heist Boss",
            UniqueMonster::BeyondBoss(_) => "Beyond Boss",
            UniqueMonster::ExpeditionLogbookBoss(_) => "Expedition Logbook Boss",
            UniqueMonster::ShaperMiniBoss(_) => "Shaper Mini-Boss",
            UniqueMonster::BetrayalCatarina(_) => "Betrayal Catarina",
            UniqueMonster::AllIncursionArchitectsInTemple => {
                "All Incursion Architects (The Temple of Atzoatl)"
            }
            UniqueMonster::OshabiBoss(_) => "Oshabi Boss",
            UniqueMonster::EldritchPerfectionMonster(_) => "Eldritch Perfection Monster",
        }
    }
}

#[derive(Debug, Clone, Default, EnumIter, PartialEq, Eq, Hash)]
pub enum BreachlordBossDomain {
    #[default]
    Xoph,
    Tul,
    Esh,
    Chayula,
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

impl FromStr for BreachlordBossDomain {
    type Err = UnknownVariant<Self>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parseid(s)
    }
}

#[derive(Debug, Clone, Default, EnumIter, PartialEq, Eq, Hash)]
pub enum Architect {
    #[default]
    Zilquapa,
    Paquate,
    Ahuana,
    Zalatl,
}

impl Identified for Architect {
    fn id(&self) -> &str {
        match self {
            Architect::Zilquapa => "Zilquapa, Architect of the Breach",
            Architect::Paquate => "Paquate, Architect of Corruption",
            Architect::Ahuana => "Ahuana, Architect of Ceremonies",
            Architect::Zalatl => "Zalatl, Architect of Thaumaturgy",
        }
    }
}

impl FromStr for Architect {
    type Err = UnknownVariant<Self>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parseid(s)
    }
}

#[derive(Debug, Clone, Default, EnumIter, PartialEq, Eq, Hash)]
pub enum ShaperGuardianBoss {
    #[default]
    Chimera,
    Hydra,
    Minotaur,
    Phoenix,
}

impl Identified for ShaperGuardianBoss {
    fn id(&self) -> &str {
        match self {
            ShaperGuardianBoss::Chimera => "Guardian of the Chimera",
            ShaperGuardianBoss::Hydra => "Guardian of the Hydra",
            ShaperGuardianBoss::Minotaur => "Guardian of the Minotaur",
            ShaperGuardianBoss::Phoenix => "Guardian of the Phoenix",
        }
    }
}

impl FromStr for ShaperGuardianBoss {
    type Err = UnknownVariant<Self>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parseid(s)
    }
}

#[derive(Debug, Clone, Default, EnumIter, PartialEq, Eq, Hash)]
pub enum SyndicateMember {
    #[default]
    Haku,
    Elreon,
    Tora,
    Vagan,
    Vorici,
    Hillock,
    Leo,
    GuffTinyGrenn,
    JanusPerandus,
    ItThatFled,
    Gravicius,
    ThandeJorgin,
    KorellGoya,
    RinYuushu,
    CameriaTheColdblooded,
    AislingLaffrey,
    RikerMaloney,
}

impl Identified for SyndicateMember {
    fn id(&self) -> &str {
        match self {
            SyndicateMember::Haku => "Haku, Warmaster",
            SyndicateMember::Elreon => "Elreon",
            SyndicateMember::Tora => "Tora",
            SyndicateMember::Vagan => "Vagan",
            SyndicateMember::Vorici => "Vorici",
            SyndicateMember::Hillock => "Hillock, the Blacksmith",
            SyndicateMember::Leo => "Leo, Wolf of the Pits",
            SyndicateMember::GuffTinyGrenn => "Guff \"Tiny\" Grenn",
            SyndicateMember::JanusPerandus => "Janus Perandus",
            SyndicateMember::ItThatFled => "It That Fled",
            SyndicateMember::Gravicius => "Gravicius",
            SyndicateMember::ThandeJorgin => "Thane Jorgin",
            SyndicateMember::KorellGoya => "Korell Goya",
            SyndicateMember::RinYuushu => "Rin Yuushu",
            SyndicateMember::CameriaTheColdblooded => "Cameria the Coldblooded",
            SyndicateMember::AislingLaffrey => "Aisling Laffrey",
            SyndicateMember::RikerMaloney => "Riker Maloney",
        }
    }

    fn aliases(&self) -> Vec<&str> {
        match self {
            SyndicateMember::Haku => vec!["Haku"],
            _ => vec![],
        }
    }
}

impl FromStr for SyndicateMember {
    type Err = UnknownVariant<Self>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parseid(s)
    }
}

#[derive(Debug, Clone, Default, EnumIter, PartialEq, Eq, Hash)]
pub enum Elderslayer {
    #[default]
    Baran,
    Veritania,
    AlHezmin,
    Drox,
    Sirus,
}

impl Identified for Elderslayer {
    fn id(&self) -> &str {
        match self {
            Elderslayer::Baran => "Baran, The Crusader",
            Elderslayer::Veritania => "Veritania, The Redeemer",
            Elderslayer::AlHezmin => "Al-Hezmin, The Hunter",
            Elderslayer::Drox => "Drox, The Warlord",
            Elderslayer::Sirus => "Sirus, Awakener of Worlds",
        }
    }
}

impl FromStr for Elderslayer {
    type Err = UnknownVariant<Self>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parseid(s)
    }
}

#[derive(Debug, Clone, Default, EnumIter, PartialEq, Eq, Hash)]
pub enum ElderGuardianBoss {
    #[default]
    Enslaver,
    Eradicator,
    Constrictor,
    Purifier,
}

impl Identified for ElderGuardianBoss {
    fn id(&self) -> &str {
        match self {
            ElderGuardianBoss::Enslaver => "The Enslaver",
            ElderGuardianBoss::Eradicator => "The Eradicator",
            ElderGuardianBoss::Constrictor => "The Constrictor",
            ElderGuardianBoss::Purifier => "The Purifier",
        }
    }
}

impl FromStr for ElderGuardianBoss {
    type Err = UnknownVariant<Self>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parseid(s)
    }
}

#[derive(Debug, Clone, Default, EnumIter, PartialEq, Eq, Hash)]
pub enum RogueExile {
    #[default]
    AshLessard,
    Magnus,
    Minara,
}

impl Identified for RogueExile {
    fn id(&self) -> &str {
        match self {
            RogueExile::AshLessard => "Ash Lessard",
            RogueExile::Magnus => "Magnus Stonethorn",
            RogueExile::Minara => "Minara Anemina",
        }
    }
}

impl FromStr for RogueExile {
    type Err = UnknownVariant<Self>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parseid(s)
    }
}

#[derive(Debug, Clone, Default, EnumIter, PartialEq, Eq, Hash)]
pub enum FemaleRogueExile {
    #[default]
    DenaLorenni,
    IgnaPhoenix,
    MinaraAnemina,
    UltimaThule,
    KirmesOlli,
    AilentiaRac,
    AntalieNapora,
    OrraGreengate,
    ThenaMoga,
    AugustinaSolaria,
    VanthAgiel,
    AshLessard,
    LaelFuria,
}

impl Identified for FemaleRogueExile {
    fn id(&self) -> &str {
        match self {
            FemaleRogueExile::DenaLorenni => "Dena Lorenni",
            FemaleRogueExile::IgnaPhoenix => "Igna Phoenix",
            FemaleRogueExile::MinaraAnemina => "Minara Anemina",
            FemaleRogueExile::UltimaThule => "Ultima Thule",
            FemaleRogueExile::KirmesOlli => "Kirmes Olli",
            FemaleRogueExile::AilentiaRac => "Ailentia Rac",
            FemaleRogueExile::AntalieNapora => "Antalie Napora",
            FemaleRogueExile::OrraGreengate => "Orra Greengate",
            FemaleRogueExile::ThenaMoga => "Thena Moga",
            FemaleRogueExile::AugustinaSolaria => "Augustina Solaria",
            FemaleRogueExile::VanthAgiel => "Vanth Agiel",
            FemaleRogueExile::AshLessard => "Ash Lessard",
            FemaleRogueExile::LaelFuria => "Lael Furia",
        }
    }
}

impl FromStr for FemaleRogueExile {
    type Err = UnknownVariant<Self>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parseid(s)
    }
}

#[derive(Debug, Clone, Default, EnumIter, PartialEq, Eq, Hash)]
pub enum AbyssLichBoss {
    #[default]
    Ulaman,
    Amanamu,
}

impl Identified for AbyssLichBoss {
    fn id(&self) -> &str {
        match self {
            AbyssLichBoss::Ulaman => "Ulaman, Sovereign of the Well",
            AbyssLichBoss::Amanamu => "Amanamu, Liege of the Lightless",
        }
    }
}

impl FromStr for AbyssLichBoss {
    type Err = UnknownVariant<Self>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parseid(s)
    }
}

#[derive(Debug, Clone, Default, EnumIter, PartialEq, Eq, Hash)]
pub enum MapsOnly {
    #[default]
    Omniphobia,
    Kosis,
}

impl Identified for MapsOnly {
    fn id(&self) -> &str {
        match self {
            MapsOnly::Omniphobia => "Omniphobia, Fear Manifest (maps only)",
            MapsOnly::Kosis => "Kosis, The Revelation (maps only)",
        }
    }
}

impl FromStr for MapsOnly {
    type Err = UnknownVariant<Self>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parseid(s)
    }
}

#[derive(Debug, Clone, Default, EnumIter, PartialEq, Eq, Hash)]
pub enum HarbingerPortal {
    #[default]
    HarbingerPortal,
    HarbingerPortalDelve,
    HarbingerPortalUber,
}

impl Identified for HarbingerPortal {
    fn id(&self) -> &str {
        match self {
            HarbingerPortal::HarbingerPortal => "HarbingerPortal",
            HarbingerPortal::HarbingerPortalDelve => "HarbingerPortalDelve",
            HarbingerPortal::HarbingerPortalUber => "HarbingerPortalUber",
        }
    }
}

impl FromStr for HarbingerPortal {
    type Err = UnknownVariant<Self>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parseid(s)
    }
}

#[derive(Debug, Clone, Default, EnumIter, PartialEq, Eq, Hash)]
pub enum EndgameBoss {
    #[default]
    Maven,
    Elder,
    UberElder,
    SearingExarch,
    EaterOfWorlds,
    InfiniteHunger,
    UberAtziri,
}

impl Identified for EndgameBoss {
    fn id(&self) -> &str {
        match self {
            EndgameBoss::Maven => "The Maven",
            EndgameBoss::Elder => "The Elder",
            EndgameBoss::UberElder => "Uber Elder",
            EndgameBoss::SearingExarch => "The Searing Exarch",
            EndgameBoss::EaterOfWorlds => "The Eater of Worlds",
            EndgameBoss::InfiniteHunger => "The Infinite Hunger",
            EndgameBoss::UberAtziri => "Atziri, Queen of the Vaal (Uber)",
        }
    }
}

impl FromStr for EndgameBoss {
    type Err = UnknownVariant<Self>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parseid(s)
    }
}

#[derive(Debug, Clone, Default, EnumIter, PartialEq, Eq, Hash)]
pub enum DelveBoss {
    #[default]
    Aul,
    Kurgal,
}

impl Identified for DelveBoss {
    fn id(&self) -> &str {
        match self {
            DelveBoss::Aul => "Aul, the Crystal King",
            DelveBoss::Kurgal => "Kurgal, the Blackblooded",
        }
    }
}

impl FromStr for DelveBoss {
    type Err = UnknownVariant<Self>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parseid(s)
    }
}

#[derive(Debug, Clone, Default, EnumIter, PartialEq, Eq, Hash)]
pub enum BeastBoss {
    #[default]
    Farrul,
    Fenumus,
    Saqawal,
    Craiceann,
}

impl Identified for BeastBoss {
    fn id(&self) -> &str {
        match self {
            BeastBoss::Farrul => "Farrul, First of the Plains",
            BeastBoss::Fenumus => "Fenumus, First of the Night",
            BeastBoss::Saqawal => "Saqawal, First of the Sky",
            BeastBoss::Craiceann => "Craiceann, First of the Deep",
        }
    }
}

impl FromStr for BeastBoss {
    type Err = UnknownVariant<Self>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parseid(s)
    }
}

#[derive(Debug, Clone, Default, EnumIter, PartialEq, Eq, Hash)]
pub enum HeistBoss {
    #[default]
    FleshSculptor,
    CorpseStitcher,
}

impl Identified for HeistBoss {
    fn id(&self) -> &str {
        match self {
            HeistBoss::FleshSculptor => "Flesh Sculptor",
            HeistBoss::CorpseStitcher => "Corpse Stitcher",
        }
    }
}

impl FromStr for HeistBoss {
    type Err = UnknownVariant<Self>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parseid(s)
    }
}

#[derive(Debug, Clone, Default, EnumIter, PartialEq, Eq, Hash)]
pub enum BeyondBoss {
    #[default]
    Ghorr,
    Ktash,
}

impl Identified for BeyondBoss {
    fn id(&self) -> &str {
        match self {
            BeyondBoss::Ghorr => "Ghorr, the Grasping Maw",
            BeyondBoss::Ktash => "K'tash, the Hate Shepherd",
        }
    }
}

impl FromStr for BeyondBoss {
    type Err = UnknownVariant<Self>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parseid(s)
    }
}

#[derive(Debug, Clone, Default, EnumIter, PartialEq, Eq, Hash)]
pub enum ExpeditionLogbookBoss {
    #[default]
    Uhtred,
}

impl Identified for ExpeditionLogbookBoss {
    fn id(&self) -> &str {
        match self {
            ExpeditionLogbookBoss::Uhtred => "Uhtred, Covetous Traitor",
        }
    }
}

impl FromStr for ExpeditionLogbookBoss {
    type Err = UnknownVariant<Self>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parseid(s)
    }
}

#[derive(Debug, Clone, Default, EnumIter, PartialEq, Eq, Hash)]
pub enum BetrayalCatarina {
    #[default]
    Catarina,
}

impl Identified for BetrayalCatarina {
    fn id(&self) -> &str {
        "Catarina, Master of Undeath"
    }
}

impl FromStr for BetrayalCatarina {
    type Err = UnknownVariant<Self>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parseid(s)
    }
}

#[derive(Debug, Clone, Default, EnumIter, PartialEq, Eq, Hash)]
pub enum OshabiBoss {
    #[default]
    Oshabi,
}

impl Identified for OshabiBoss {
    fn id(&self) -> &str {
        "Oshabi, Avatar of the Grove"
    }

    fn aliases(&self) -> Vec<&str> {
        vec!["Oshabi"]
    }
}

impl FromStr for OshabiBoss {
    type Err = UnknownVariant<Self>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parseid(s)
    }
}

#[derive(Debug, Clone, Default, EnumIter, PartialEq, Eq, Hash)]
pub enum EldritchPerfectionMonster {
    #[default]
    ConsumingBearer,
    ConsumingParasite,
    ConsumingThrall,
    GrotesqueCavedweller,
    GrotesqueMangler,
    GrotesqueMauler,
    GrotesqueMaw,
    MoltenGolem,
    MoltenMinotaur,
    MoltenWretch,
    VoidFlayer,
    VoidJaguar,
    VoidSkulker,
}

impl Identified for EldritchPerfectionMonster {
    fn id(&self) -> &str {
        match self {
            EldritchPerfectionMonster::ConsumingBearer => "Consuming Bearer",
            EldritchPerfectionMonster::ConsumingParasite => "Consuming Parasite",
            EldritchPerfectionMonster::ConsumingThrall => "Consuming Thrall",
            EldritchPerfectionMonster::GrotesqueCavedweller => "Grotesque Cavedweller",
            EldritchPerfectionMonster::GrotesqueMangler => "Grotesque Mangler",
            EldritchPerfectionMonster::GrotesqueMauler => "Grotesque Mauler",
            EldritchPerfectionMonster::GrotesqueMaw => "Grotesque Maw",
            EldritchPerfectionMonster::MoltenGolem => "Molten Golem",
            EldritchPerfectionMonster::MoltenMinotaur => "Molten Minotaur",
            EldritchPerfectionMonster::MoltenWretch => "Molten Wretch",
            EldritchPerfectionMonster::VoidFlayer => "Void Flayer",
            EldritchPerfectionMonster::VoidJaguar => "Void Jaguar",
            EldritchPerfectionMonster::VoidSkulker => "Void Skulker",
        }
    }
}

impl FromStr for EldritchPerfectionMonster {
    type Err = UnknownVariant<Self>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parseid(s)
    }
}

#[derive(Debug, Clone, Default, EnumIter, PartialEq, Eq, Hash)]
pub enum ShaperMiniBoss {
    #[default]
    EntityOfTheVoid,
}

impl Identified for ShaperMiniBoss {
    fn id(&self) -> &str {
        "Entity of the Void"
    }
}

impl FromStr for ShaperMiniBoss {
    type Err = UnknownVariant<Self>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parseid(s)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_unique_monster() {
        let veritania: UniqueMonster = "Veritania, The Redeemer".parse().unwrap();
        assert_eq!(
            veritania,
            UniqueMonster::Elderslayer(Elderslayer::Veritania)
        );
    }
}
