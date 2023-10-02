use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum_macros::EnumString;

use super::dropconsts::BOSS_NAMES;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(tag = "uniqueMonster")]
pub enum UniqueMonster {
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
    #[serde(rename = "All Incursion Architects in Alva missions/Alva's Memory")]
    AllIncursionArchitectsInAlvaMission,
    #[serde(rename = "All Abyss Monsters")]
    AllAbyssMonsters,
    #[serde(rename = "All (Scourge) beyond demons")]
    AllScourgeBeyondDemons,
    #[serde(rename = "All Rogue Exiles")]
    AllRogueExiles,
    BreachlordBossDomain(BreachlordBossDomain),
    Architect(Architect),
    ShaperGuardianBoss(ShaperGuardianBoss),
    SyndicateMember(SyndicateMember),
    Elderslayer(Elderslayer),
    ElderGuardianBoss(ElderGuardianBoss),
    RogueExile(RogueExile),
    AbyssLichBoss(AbyssLichBoss),
    MapsOnly(MapsOnly),
    StoryBoss(StoryBoss),
    HarbingerPortal(HarbingerPortal),
    Boss {
        name: String,
    },
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
            "All Incursion Architects in Alva missions/Alva's Memory" => {
                Ok(Self::AllIncursionArchitectsInAlvaMission)
            }
            "All Abyss Monsters" => Ok(Self::AllAbyssMonsters),
            "All Rogue Exiles" => Ok(Self::AllRogueExiles),
            "All (Scourge) beyond demons" => Ok(Self::AllScourgeBeyondDemons),
            _ => BreachlordBossDomain::from_str(s)
                .map(|b| Self::BreachlordBossDomain(b))
                .or_else(|_| Architect::from_str(s).map(|b| Self::Architect(b)))
                .or_else(|_| ShaperGuardianBoss::from_str(s).map(|b| Self::ShaperGuardianBoss(b)))
                .or_else(|_| SyndicateMember::from_str(s).map(|b| Self::SyndicateMember(b)))
                .or_else(|_| Elderslayer::from_str(s).map(|b| Self::Elderslayer(b)))
                .or_else(|_| ElderGuardianBoss::from_str(s).map(|b| Self::ElderGuardianBoss(b)))
                .or_else(|_| RogueExile::from_str(s).map(|b| Self::RogueExile(b)))
                .or_else(|_| AbyssLichBoss::from_str(s).map(|b| Self::AbyssLichBoss(b)))
                .or_else(|_| MapsOnly::from_str(s).map(|b| Self::MapsOnly(b)))
                .or_else(|_| StoryBoss::from_str(s).map(|b| Self::StoryBoss(b)))
                .or_else(|_| HarbingerPortal::from_str(s).map(|b| Self::HarbingerPortal(b)))
                .or_else(|_| match BOSS_NAMES.contains(&s) {
                    true => Ok(Self::Boss {
                        name: s.to_string(),
                    }),
                    false => Err(strum::ParseError::VariantNotFound),
                }),
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
                write!(f, "All Incursion Architects in Alva missions/Alva's Memory")
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
            UniqueMonster::AbyssLichBoss(abysslichboss) => abysslichboss.fmt(f),
            UniqueMonster::MapsOnly(mapsonly) => mapsonly.fmt(f),
            UniqueMonster::StoryBoss(storyboss) => storyboss.fmt(f),
            UniqueMonster::HarbingerPortal(harbingerportal) => harbingerportal.fmt(f),
            UniqueMonster::Boss { name } => name.fmt(f),
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
pub enum Architect {
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
    Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, EnumString, strum_macros::Display,
)]
#[serde(tag = "name")]
pub enum ShaperGuardianBoss {
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
    Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, EnumString, strum_macros::Display,
)]
#[serde(tag = "name")]
pub enum SyndicateMember {
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
    Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, EnumString, strum_macros::Display,
)]
#[serde(tag = "name")]
pub enum Elderslayer {
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
    Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, EnumString, strum_macros::Display,
)]
#[serde(tag = "name")]
pub enum ElderGuardianBoss {
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
    Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, EnumString, strum_macros::Display,
)]
#[serde(tag = "name")]
pub enum RogueExile {
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
    Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, EnumString, strum_macros::Display,
)]
#[serde(tag = "name")]
pub enum AbyssLichBoss {
    #[strum(serialize = "Ulaman, Sovereign of the Well")]
    #[serde(rename = "Ulaman, Sovereign of the Well")]
    Ulaman,
    #[strum(serialize = "Amanamu, Liege of the Lightless")]
    #[serde(rename = "Amanamu, Liege of the Lightless")]
    Amanamu,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, EnumString, strum_macros::Display,
)]
#[serde(tag = "name")]
pub enum MapsOnly {
    #[strum(serialize = "Omniphobia, Fear Manifest (maps only)")]
    #[serde(rename = "Omniphobia, Fear Manifest (maps only)")]
    Omniphobia,
    #[strum(serialize = "Kosis, The Revelation (maps only)")]
    #[serde(rename = "Kosis, The Revelation (maps only)")]
    Kosis,
}

#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, EnumString, strum_macros::Display,
)]
#[serde(tag = "name")]
pub enum StoryBoss {
    #[strum(serialize = "Reassembled Brutus")]
    #[serde(rename = "Reassembled Brutus")]
    ReassembledBrutus,
    #[strum(serialize = "Shavronne, Unbound")]
    #[serde(rename = "Shavronne, Unbound")]
    ShavronneUnbound,
    #[strum(serialize = "Dawn, Harbinger of Solaris")]
    #[serde(rename = "Dawn, Harbinger of Solaris")]
    Dawn,
    #[strum(serialize = "Solaris, Eternal Sun")]
    #[serde(rename = "Solaris, Eternal Sun")]
    Solaris,
}

#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, EnumString, strum_macros::Display,
)]
#[serde(tag = "name")]
pub enum HarbingerPortal {
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
