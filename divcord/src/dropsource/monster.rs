use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::EnumString;

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, strum_macros::EnumIter, Default,
)]
#[serde(tag = "type")]
pub enum UniqueMonster {
    #[default]
    #[serde(rename = "Maven's Invitation: The Feared")]
    MavensInvitationTheFeared,
    #[serde(rename = "Uul-Netol, Unburdened Flesh (in Breachstones)")]
    UulNetolInBreachstones,
    #[serde(rename = "The Vaal Omnitect")]
    VaalOmnitect,
    #[serde(rename = "Metamorph")]
    Metamorph,
    #[serde(rename = "Null Portal")]
    NullPortal,
    #[serde(rename = "Vaal Flesh Merchant")]
    VaalFleshMerchant,
    #[serde(
        rename = "All Incursion Architects in Alva missions or Alva's Memory",
        alias = "All Incursion Architects in Alva missions/Alva's Memory",
        alias = "All Incursion Architects in Alva missions or Alva's Memory",
        alias = "All Incursion Architects in Alva missions or Alva's Memory"
    )]
    AllIncursionArchitectsInAlvaMission,
    #[serde(rename = "All Abyss Monsters")]
    AllAbyssMonsters,
    #[serde(rename = "All (Scourge) beyond demons")]
    AllScourgeBeyondDemons,
    #[serde(rename = "All Rogue Exiles")]
    AllRogueExiles,
    #[serde(rename = "Venarius")]
    CortexVenarius,
    #[serde(rename = "Argus")]
    Argus,
    #[serde(rename = "All Invasion Bosses")]
    AllInvasionBosses,
    #[serde(rename = "All Vaal Side Area Bosses")]
    AllVaalSideAreaBosses,
    #[serde(rename = "Breachlord Boss Domain")]
    BreachlordBossDomain(BreachlordBossDomain),
    #[serde(rename = "Architect")]
    Architect(Architect),
    #[serde(rename = "Shaper Guardian Boss")]
    ShaperGuardianBoss(ShaperGuardianBoss),
    #[serde(rename = "Syndicate Member")]
    SyndicateMember(SyndicateMember),
    #[serde(rename = "Elder Slayer")]
    Elderslayer(Elderslayer),
    #[serde(rename = "Elder Guardian Boss")]
    ElderGuardianBoss(ElderGuardianBoss),
    #[serde(rename = "Rogue Exile")]
    RogueExile(RogueExile),
    #[serde(rename = "Female Rogue Exile")]
    FemaleRogueExile(FemaleRogueExile),
    #[serde(rename = "Abyss Lich Boss")]
    AbyssLichBoss(AbyssLichBoss),
    #[serde(rename = "Maps Only")]
    MapsOnly(MapsOnly),
    #[serde(rename = "Harbinger Portal")]
    HarbingerPortal(HarbingerPortal),
    #[serde(rename = "Endgame Boss")]
    EndgameBoss(EndgameBoss),
    #[serde(rename = "Delve Boss")]
    DelveBoss(DelveBoss),
    #[serde(rename = "Beast Boss")]
    BeastBoss(BeastBoss),
    #[serde(rename = "Heist Boss")]
    HeistBoss(HeistBoss),
    #[serde(rename = "Beyond Boss")]
    BeyondBoss(BeyondBoss),
    #[serde(rename = "Expedition Logbook Boss")]
    ExpeditionLogbookBoss(ExpeditionLogbookBoss),
    #[serde(rename = "Shaper Mini-Boss")]
    ShaperMiniBoss(ShaperMiniBoss),
    #[serde(rename = "Betrayal Catarina")]
    BetrayalCatarina(BetrayalCatarina),
    #[serde(rename = "Oshabi Boss")]
    OshabiBoss(OshabiBoss),
}

impl UniqueMonster {
    pub fn _types() -> impl Iterator<Item = String> {
        UniqueMonster::iter().map(|a| a._type().to_string())
    }
}

impl UniqueMonster {
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
            UniqueMonster::OshabiBoss(_) => "Oshabi Boss",
        }
    }
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
            "All Abyss Monsters" => Ok(Self::AllAbyssMonsters),
            "All Rogue Exiles" => Ok(Self::AllRogueExiles),
            "All Invasion Bosses" => Ok(Self::AllInvasionBosses),
            "All Vaal Side Area Bosses" => Ok(Self::AllVaalSideAreaBosses),
            "All (Scourge) beyond demons" => Ok(Self::AllScourgeBeyondDemons),
            "Venarius" => Ok(Self::CortexVenarius),
            "Argus" => Ok(Self::Argus),
            _ => BreachlordBossDomain::from_str(s)
                .map(|b| Self::BreachlordBossDomain(b))
                .or_else(|_| Architect::from_str(s).map(|b| Self::Architect(b)))
                .or_else(|_| ShaperGuardianBoss::from_str(s).map(|b| Self::ShaperGuardianBoss(b)))
                .or_else(|_| SyndicateMember::from_str(s).map(|b| Self::SyndicateMember(b)))
                .or_else(|_| Elderslayer::from_str(s).map(|b| Self::Elderslayer(b)))
                .or_else(|_| ElderGuardianBoss::from_str(s).map(|b| Self::ElderGuardianBoss(b)))
                .or_else(|_| RogueExile::from_str(s).map(|b| Self::RogueExile(b)))
                .or_else(|_| FemaleRogueExile::from_str(s).map(|b| Self::FemaleRogueExile(b)))
                .or_else(|_| AbyssLichBoss::from_str(s).map(|b| Self::AbyssLichBoss(b)))
                .or_else(|_| MapsOnly::from_str(s).map(|b| Self::MapsOnly(b)))
                .or_else(|_| HarbingerPortal::from_str(s).map(|b| Self::HarbingerPortal(b)))
                .or_else(|_| EndgameBoss::from_str(s).map(|b| Self::EndgameBoss(b)))
                .or_else(|_| DelveBoss::from_str(s).map(|b| Self::DelveBoss(b)))
                .or_else(|_| BeastBoss::from_str(s).map(|b| Self::BeastBoss(b)))
                .or_else(|_| HeistBoss::from_str(s).map(|b| Self::HeistBoss(b)))
                .or_else(|_| BeyondBoss::from_str(s).map(|b| Self::BeyondBoss(b)))
                .or_else(|_| {
                    ExpeditionLogbookBoss::from_str(s).map(|b| Self::ExpeditionLogbookBoss(b))
                })
                .or_else(|_| ShaperMiniBoss::from_str(s).map(|b| Self::ShaperMiniBoss(b)))
                .or_else(|_| BetrayalCatarina::from_str(s).map(|b| Self::BetrayalCatarina(b)))
                .or_else(|_| OshabiBoss::from_str(s).map(|b| Self::OshabiBoss(b)))
                .or_else(|_| return Err(strum::ParseError::VariantNotFound)),
        }
    }
}

impl std::fmt::Display for UniqueMonster {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UniqueMonster::MavensInvitationTheFeared => write!(f, "Maven's Invitation: The Feared"),
            UniqueMonster::UulNetolInBreachstones => {
                write!(f, "Uul-Netol, Unburdened Flesh (in Breachstones)")
            }
            UniqueMonster::VaalOmnitect => write!(f, "The Vaal Omnitect"),
            UniqueMonster::Metamorph => write!(f, "Metamorph"),
            UniqueMonster::NullPortal => write!(f, "Null Portal"),
            UniqueMonster::VaalFleshMerchant => write!(f, "Vaal Flesh Merchant"),
            UniqueMonster::AllIncursionArchitectsInAlvaMission => {
                write!(
                    f,
                    "All Incursion Architects in Alva missions or Alva's Memory"
                )
            }
            UniqueMonster::AllAbyssMonsters => write!(f, "All Abyss Monsters"),
            UniqueMonster::AllScourgeBeyondDemons => write!(f, "All (Scourge) beyond demons"),
            UniqueMonster::AllRogueExiles => write!(f, "All Rogue Exiles"),
            UniqueMonster::BreachlordBossDomain(breachlord) => breachlord.fmt(f),
            UniqueMonster::Architect(architect) => architect.fmt(f),
            UniqueMonster::ShaperGuardianBoss(shaperguard) => shaperguard.fmt(f),
            UniqueMonster::SyndicateMember(syndicate) => syndicate.fmt(f),
            UniqueMonster::Elderslayer(elderslayer) => elderslayer.fmt(f),
            UniqueMonster::ElderGuardianBoss(elderguard) => elderguard.fmt(f),
            UniqueMonster::RogueExile(rogueexile) => rogueexile.fmt(f),
            UniqueMonster::FemaleRogueExile(rogue) => rogue.fmt(f),
            UniqueMonster::AbyssLichBoss(abysslichboss) => abysslichboss.fmt(f),
            UniqueMonster::MapsOnly(mapsonly) => mapsonly.fmt(f),
            UniqueMonster::HarbingerPortal(harbingerportal) => harbingerportal.fmt(f),
            UniqueMonster::CortexVenarius => write!(f, "Venarius"),
            UniqueMonster::Argus => write!(f, "Argus"),
            UniqueMonster::EndgameBoss(boss) => boss.fmt(f),
            UniqueMonster::DelveBoss(boss) => boss.fmt(f),
            UniqueMonster::BeastBoss(boss) => boss.fmt(f),
            UniqueMonster::HeistBoss(boss) => boss.fmt(f),
            UniqueMonster::BeyondBoss(boss) => boss.fmt(f),
            UniqueMonster::ExpeditionLogbookBoss(boss) => boss.fmt(f),
            UniqueMonster::ShaperMiniBoss(boss) => boss.fmt(f),
            UniqueMonster::BetrayalCatarina(boss) => boss.fmt(f),
            UniqueMonster::OshabiBoss(boss) => boss.fmt(f),
            UniqueMonster::AllInvasionBosses => write!(f, "All Invasion Bosses"),
            UniqueMonster::AllVaalSideAreaBosses => write!(f, "All Vaal Side Area Bosses"),
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
    Default,
)]
#[serde(tag = "name")]
pub enum BreachlordBossDomain {
    #[default]
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
    Default,
)]
#[serde(tag = "name")]
pub enum Architect {
    #[default]
    #[strum(serialize = "Zilquapa, Architect of the Breach")]
    #[serde(rename = "Zilquapa, Architect of the Breach")]
    Zilquapa,
    #[strum(serialize = "Paquate, Architect of Corruption")]
    #[serde(rename = "Paquate, Architect of Corruption")]
    Paquate,
    #[strum(serialize = "Ahuana, Architect of Ceremonies")]
    #[serde(rename = "Ahuana, Architect of Ceremonies")]
    Ahuana,
    #[strum(serialize = "Zalatl, Architect of Thaumaturgy")]
    #[serde(rename = "Zalatl, Architect of Thaumaturgy")]
    Zalatl,
}

#[derive(
    Serialize,
    Deserialize,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    EnumString,
    strum_macros::Display,
    Default,
)]
#[serde(tag = "name")]
pub enum ShaperGuardianBoss {
    #[default]
    #[strum(serialize = "Guardian of the Chimera")]
    #[serde(rename = "Guardian of the Chimera")]
    Chimera,
    #[strum(serialize = "Guardian of the Hydra")]
    #[serde(rename = "Guardian of the Hydra")]
    Hydra,
    #[strum(serialize = "Guardian of the Minotaur")]
    #[serde(rename = "Guardian of the Minotaur")]
    Minotaur,
    #[strum(serialize = "Guardian of the Phoenix")]
    #[serde(rename = "Guardian of the Phoenix")]
    Phoenix,
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
    Default,
    strum_macros::EnumIter,
)]
#[serde(tag = "name")]
pub enum SyndicateMember {
    #[default]
    #[strum(serialize = "Haku", serialize = "Haku, Warmaster")]
    #[serde(rename = "Haku, Warmaster")]
    Haku,
    #[strum(serialize = "Elreon")]
    #[serde(rename = "Elreon")]
    Elreon,
    #[strum(serialize = "Tora")]
    #[serde(rename = "Tora")]
    Tora,
    #[strum(serialize = "Vagan")]
    #[serde(rename = "Vagan")]
    Vagan,
    #[strum(serialize = "Vorici")]
    #[serde(rename = "Vorici")]
    Vorici,
    #[strum(serialize = "Hillock, the Blacksmith")]
    #[serde(rename = "Hillock, the Blacksmith")]
    Hillock,
    #[strum(serialize = "Leo, Wolf of the Pits")]
    #[serde(rename = "Leo, Wolf of the Pits")]
    Leo,
    #[strum(serialize = "Guff \"Tiny\" Grenn")]
    #[serde(rename = "Guff \"Tiny\" Grenn")]
    GuffTinyGrenn,
    #[strum(serialize = "Janus Perandus")]
    #[serde(rename = "Janus Perandus")]
    JanusPerandus,
    #[strum(serialize = "It That Fled")]
    #[serde(rename = "It That Fled")]
    ItThatFled,
    #[strum(serialize = "Gravicius")]
    #[serde(rename = "Gravicius")]
    Gravicius,
    #[strum(serialize = "Thane Jorgin")]
    #[serde(rename = "Thane Jorgin")]
    ThandeJorgin,
    #[strum(serialize = "Korell Goya")]
    #[serde(rename = "Korell Goya")]
    KorellGoya,
    #[strum(serialize = "Rin Yuushu")]
    #[serde(rename = "Rin Yuushu")]
    RinYuushu,
    #[strum(serialize = "Cameria the Coldblooded")]
    #[serde(rename = "Cameria the Coldblooded")]
    CameriaTheColdblooded,
    #[strum(serialize = "Aisling Laffrey")]
    #[serde(rename = "Aisling Laffrey")]
    AislingLaffrey,
    #[strum(serialize = "Riker Maloney")]
    #[serde(rename = "Riker Maloney")]
    RikerMaloney,
}

#[derive(
    Serialize,
    Deserialize,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    EnumString,
    strum_macros::Display,
    strum_macros::EnumIter,
    Default,
)]
#[serde(tag = "name")]
pub enum Elderslayer {
    #[default]
    #[strum(serialize = "Baran, The Crusader")]
    #[serde(rename = "Baran, The Crusader")]
    Baran,
    #[strum(serialize = "Veritania, The Redeemer")]
    #[serde(rename = "Veritania, The Redeemer")]
    Veritania,
    #[strum(serialize = "Al-Hezmin, The Hunter")]
    #[serde(rename = "Al-Hezmin, The Hunter")]
    AlHezmin,
    #[strum(serialize = "Drox, The Warlord")]
    #[serde(rename = "Drox, The Warlord")]
    Drox,
    #[strum(serialize = "Sirus, Awakener of Worlds")]
    #[serde(rename = "Sirus, Awakener of Worlds")]
    Sirus,
}

#[derive(
    Serialize,
    Deserialize,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    EnumString,
    strum_macros::Display,
    strum_macros::EnumIter,
    Default,
)]
#[serde(tag = "name")]
pub enum ElderGuardianBoss {
    #[default]
    #[strum(serialize = "The Enslaver")]
    #[serde(rename = "The Enslaver")]
    Enslaver,
    #[strum(serialize = "The Eradicator")]
    #[serde(rename = "The Eradicator")]
    Eradicator,
    #[strum(serialize = "The Constrictor")]
    #[serde(rename = "The Constrictor")]
    Constrictor,
    #[strum(serialize = "The Purifier")]
    #[serde(rename = "The Purifier")]
    Purifier,
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
    Default,
)]
#[serde(tag = "name")]
pub enum RogueExile {
    #[default]
    #[strum(serialize = "Ash Lessard")]
    #[serde(rename = "Ash Lessard")]
    AshLessard,
    #[strum(serialize = "Magnus Stonethorn")]
    #[serde(rename = "Magnus Stonethorn")]
    Magnus,
    #[strum(serialize = "Minara Anemina")]
    #[serde(rename = "Minara Anemina")]
    Minara,
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
    Default,
)]
#[serde(tag = "name")]
pub enum FemaleRogueExile {
    #[default]
    #[strum(serialize = "Dena Lorenni")]
    #[serde(rename = "Dena Lorenni")]
    DenaLorenni,
    #[strum(serialize = "Igna Phoenix")]
    #[serde(rename = "Igna Phoenix")]
    IgnaPhoenix,
    #[strum(serialize = "Minara Anemina")]
    #[serde(rename = "Minara Anemina")]
    MinaraAnemina,
    #[strum(serialize = "Ultima Thule")]
    #[serde(rename = "Ultima Thule")]
    UltimaThule,
    #[strum(serialize = "Kirmes Olli")]
    #[serde(rename = "Kirmes Olli")]
    KirmesOlli,
    #[strum(serialize = "Ailentia Rac")]
    #[serde(rename = "Ailentia Rac")]
    AilentiaRac,
    #[strum(serialize = "Antalie Napora")]
    #[serde(rename = "Antalie Napora")]
    AntalieNapora,
    #[strum(serialize = "Orra Greengate")]
    #[serde(rename = "Orra Greengate")]
    OrraGreengate,
    #[strum(serialize = "Thena Moga")]
    #[serde(rename = "Thena Moga")]
    ThenaMoga,
    #[strum(serialize = "Augustina Solaria")]
    #[serde(rename = "Augustina Solaria")]
    AugustinaSolaria,
    #[strum(serialize = "Vanth Agiel")]
    #[serde(rename = "Vanth Agiel")]
    VanthAgiel,
    #[strum(serialize = "Ash Lessard")]
    #[serde(rename = "Ash Lessard")]
    AshLessard,
    #[strum(serialize = "Lael Furia")]
    #[serde(rename = "Lael Furia")]
    LaelFuria,
}

#[derive(
    Serialize,
    Deserialize,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    EnumString,
    strum_macros::Display,
    strum_macros::EnumIter,
    Default,
)]
#[serde(tag = "name")]
pub enum AbyssLichBoss {
    #[default]
    #[strum(serialize = "Ulaman, Sovereign of the Well")]
    #[serde(rename = "Ulaman, Sovereign of the Well")]
    Ulaman,
    #[strum(serialize = "Amanamu, Liege of the Lightless")]
    #[serde(rename = "Amanamu, Liege of the Lightless")]
    Amanamu,
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
    Default,
)]
#[serde(tag = "name")]
pub enum MapsOnly {
    #[default]
    #[strum(serialize = "Omniphobia, Fear Manifest (maps only)")]
    #[serde(rename = "Omniphobia, Fear Manifest (maps only)")]
    Omniphobia,
    #[strum(serialize = "Kosis, The Revelation (maps only)")]
    #[serde(rename = "Kosis, The Revelation (maps only)")]
    Kosis,
}

#[derive(
    Serialize,
    Deserialize,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    EnumString,
    strum_macros::Display,
    strum_macros::EnumIter,
    Default,
)]
#[serde(tag = "name")]
pub enum HarbingerPortal {
    #[default]
    #[strum(serialize = "HarbingerPortal")]
    #[serde(rename = "HarbingerPortal")]
    HarbingerPortal,
    #[strum(serialize = "HarbingerPortalDelve")]
    #[serde(rename = "HarbingerPortalDelve")]
    HarbingerPortalDelve,
    #[strum(serialize = "HarbingerPortalUber")]
    #[serde(rename = "HarbingerPortalUber")]
    HarbingerPortalUber,
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
    Default,
)]
#[serde(tag = "name")]
pub enum EndgameBoss {
    #[default]
    #[strum(to_string = "The Maven", serialize = "Maven")]
    #[serde(rename = "The Maven")]
    Maven,
    #[strum(to_string = "The Elder", serialize = "Elder")]
    #[serde(rename = "The Elder")]
    Elder,
    #[strum(to_string = "Uber Elder")]
    #[serde(rename = "Uber Elder")]
    UberElder,
    #[strum(to_string = "The Searing Exarch")]
    #[serde(rename = "The Searing Exarch")]
    SearingExarch,
    #[strum(to_string = "The Eater of Worlds")]
    #[serde(rename = "The Eater of Worlds")]
    EaterOfWorlds,
    #[strum(to_string = "The Infinite Hunger")]
    #[serde(rename = "The Infinite Hunger")]
    InfiniteHunger,
    #[strum(to_string = "Atziri, Queen of the Vaal (Uber)")]
    #[serde(rename = "Atziri, Queen of the Vaal (Uber)")]
    UberAtziri,
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
    Default,
)]
#[serde(tag = "name")]
pub enum DelveBoss {
    #[default]
    #[strum(to_string = "Aul, the Crystal King", serialize = "Aul")]
    #[serde(rename = "Aul, the Crystal King")]
    Aul,
    #[strum(to_string = "Kurgal, the Blackblooded", serialize = "Kurgal")]
    #[serde(rename = "Kurgal, the Blackblooded")]
    Kurgal,
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
    Default,
)]
#[serde(tag = "name")]
pub enum BeastBoss {
    #[default]
    #[strum(to_string = "Farrul, First of the Plains", serialize = "Farrul")]
    #[serde(rename = "Farrul, First of the Plains")]
    Farrul,
    #[strum(to_string = "Fenumus, First of the Night", serialize = "Fenumus")]
    #[serde(rename = "Fenumus, First of the Night")]
    Fenumus,
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
    Default,
)]
#[serde(tag = "name")]
pub enum HeistBoss {
    #[default]
    #[strum(to_string = "Flesh Sculptor")]
    #[serde(rename = "Flesh Sculptor")]
    FleshSculptor,
    #[strum(to_string = "Corpse Stitcher")]
    #[serde(rename = "Corpse Stitcher")]
    CorpseStitcher,
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
    Default,
)]
#[serde(tag = "name")]
pub enum BeyondBoss {
    #[default]
    #[strum(to_string = "Ghorr, the Grasping Maw", serialize = "Ghorr")]
    #[serde(rename = "Ghorr, the Grasping Maw")]
    Ghorr,
    #[strum(to_string = "K'tash, the Hate Shepherd", serialize = "K'tash")]
    #[serde(rename = "K'tash, the Hate Shepherd")]
    Ktash,
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
    Default,
)]
#[serde(tag = "name")]
pub enum ExpeditionLogbookBoss {
    #[default]
    #[strum(to_string = "Uhtred, Covetous Traitor", serialize = "Uhtred")]
    #[serde(rename = "Uhtred, Covetous Traitor")]
    Uhtred,
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
    Default,
)]
#[serde(tag = "name")]
pub enum ShaperMiniBoss {
    #[default]
    #[strum(to_string = "Entity of the Void")]
    #[serde(rename = "Entity of the Void")]
    EntityOfTheVoid,
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
    Default,
)]
#[serde(tag = "name")]
pub enum BetrayalCatarina {
    #[default]
    #[strum(to_string = "Catarina, Master of Undeath", serialize = "Catarina")]
    #[serde(rename = "Catarina, Master of Undeath")]
    Catarina,
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
    Default,
)]
#[serde(tag = "name")]
pub enum OshabiBoss {
    #[default]
    #[strum(to_string = "Oshabi, Avatar of the Grove", serialize = "Oshabi")]
    #[serde(rename = "Oshabi, Avatar of the Grove")]
    Oshabi,
}
