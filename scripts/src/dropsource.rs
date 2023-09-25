use std::{collections::HashSet, fmt::Display, slice::Iter};

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    dropconsts::{ACT_AREA_NAMES, AREA_NAMES, BOSS_NAMES, CHESTS},
    error::Error,
    maps::Map,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum DropSource {
    ExpeditionLogbook,
    GlobalDrop,
    ChestObject,
    Disabled,
    Unknown,
    Delirium,
    Strongbox,
    #[serde(alias = "All (Scourge) beyond demons")]
    ScourgeBeyondDemons,
    #[serde(alias = "All Rogue Exiles")]
    AllRogueExiles,
    #[serde(alias = "HarbingerPortal")]
    HarbingerPortal,
    #[serde(alias = "HarbingerPortalDelve")]
    HarbingerPortalDelve,
    #[serde(alias = "HarbingerPortalUber")]
    HarbingerPortalUber,
    #[serde(alias = "Metamorph")]
    Metamorph,
    #[serde(alias = "Uul-Netol, Unburdened Flesh (in Breachstones)")]
    UulNetolInBreachstones,
    #[serde(alias = "All Abyss Monsters")]
    AllAbyssMonsters,

    #[serde(alias = "All Incursion Architects in Alva missions/Alva's Memory")]
    AllIncursionArchitectsInAlvaMission,
    #[serde(alias = "Vaal Flesh Merchant")]
    VaalFleshMerchant,
    #[serde(alias = "Null Portal")]
    BreachNullPortal,

    #[serde(alias = "Pirate Treasure")]
    PirateTreasure,

    #[serde(alias = "Trial of Stinging Doubt")]
    TrialOfStingingDoubt,

    #[serde(alias = "Maven's Invitation: The Feared")]
    MavensInvitationTheFeared,

    #[serde(alias = "The Temple of Atzoatl")]
    TempleOfAtzoatl,

    #[serde(alias = "The Vaal Omnitect")]
    VaalOmnitect,

    #[serde(alias = "All Vaal side areas (need specific information)")]
    AllVaalSideAreas,
    #[serde(alias = "Vaal Side Areas")]
    VaalSideAreas,

    #[serde(untagged)]
    Vendor(Option<Vendor>),

    #[serde(untagged)]
    MapName(String),
    #[serde(untagged)]
    BossName(String),
    #[serde(untagged)]
    Story(String),

    #[serde(untagged)]
    MapsOnly(MapsOnly),
    #[serde(untagged)]
    BetrayalSyndicateMember(BetrayalSyndicateMember),
    #[serde(untagged)]
    StoryBoss(StoryBoss),
    #[serde(untagged)]
    AbyssLichBoss(AbyssLichBoss),
    #[serde(untagged)]
    BreachlordBossDomain(BreachlordBossDomain),
    #[serde(untagged)]
    RogueExile(RogueExile),
    #[serde(untagged)]
    Elderslayer(Elderslayer),
    #[serde(untagged)]
    Architect(Architect),
    #[serde(untagged)]
    AreaSpecific(AreaSpecific),
    #[serde(untagged)]
    ElderGuardianBoss(ElderGuardianBoss),
    #[serde(untagged)]
    ShaperGuardianBoss(ShaperGuardianBoss),
}

impl DropSource {
    pub fn parse(drops_from: &str) -> Result<HashSet<DropSource>, Error> {
        let maps: Vec<Map> =
            serde_json::from_str(&std::fs::read_to_string("maps.json").unwrap()).unwrap();
        let maps: Vec<String> = maps.into_iter().map(|m| m.name).collect();
        let maps_without_the: Vec<String> = maps.iter().map(|m| m.replace(" Map", "")).collect();

        let drops_from = drops_from.replace("\r\n", "");
        let mut drops_from = drops_from.replace("\n", "");

        if drops_from.ends_with(";") {
            println!("drops_from ends with ; {}", &drops_from);
            drops_from.drain(drops_from.len() - 1..);
        }

        let mut sources: HashSet<DropSource> = HashSet::new();
        for s in drops_from.split(";") {
            let s = s.trim();
            let string = s.to_string();

            let source = match s {
                s if BOSS_NAMES.contains(&s) => DropSource::BossName(string),
                s if ACT_AREA_NAMES.contains(&s) => DropSource::Story(string),
                s if AREA_NAMES.contains(&s) => DropSource::MapName(string),
                s if CHESTS.contains(&s) => DropSource::ChestObject,
                _ => {
                    if maps.contains(&string) || maps_without_the.contains(&string) {
                        DropSource::MapName(string)
                    } else if let Ok(dropsource) =
                        serde_json::from_str::<DropSource>(&json!(s).to_string())
                    {
                        dropsource
                    } else {
                        dbg!(&s);
                        DropSource::Unknown
                    }
                }
            };

            sources.insert(source);
        }

        Ok(sources)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum StoryBoss {
    #[serde(alias = "Reassembled Brutus")]
    ReassembledBrutus,
    #[serde(alias = "Shavronne, Unbound")]
    ShavronneUnbound,
    #[serde(alias = "Dawn, Harbinger of Solaris")]
    Dawn,
    #[serde(alias = "Solaris, Eternal Sun")]
    Solaris,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum AbyssLichBoss {
    #[serde(alias = "Ulaman, Sovereign of the Well")]
    Ulaman,
    #[serde(alias = "Amanamu, Liege of the Lightless")]
    Amanamu,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ElderGuardianBoss {
    #[serde(alias = "The Enslaver")]
    Enslaver,
    #[serde(alias = "The Eradicator")]
    Eradicator,
    #[serde(alias = "The Constrictor")]
    Constrictor,
    #[serde(alias = "The Purifier")]
    Purifier,
}

// Guardian of the Minotaur; Guardian of the Phoenix

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ShaperGuardianBoss {
    #[serde(alias = "Guardian of the Chimera")]
    Chimera,
    #[serde(alias = "Guardian of the Hydra")]
    Hydra,
    #[serde(alias = "Guardian of the Minotaur")]
    Minotaur,
    #[serde(alias = "Guardian of the Phoenix")]
    Phoenix,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Elderslayer {
    #[serde(alias = "Baran, The Crusader")]
    Baran,
    #[serde(alias = "Veritania, The Redeemer")]
    Veritania,
    #[serde(alias = "Al-Hezmin, The Hunter")]
    AlHezmin,
    #[serde(alias = "Drox, The Warlord")]
    Drox,
    #[serde(alias = "Sirus, Awakener of Worlds")]
    Sirus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MapsOnly {
    #[serde(alias = "Omniphobia, Fear Manifest")]
    Omniphobia,
    #[serde(alias = "Kosis, The Revelation (both in maps only)")]
    Kosis,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AreaSpecific {
    #[serde(alias = "Chayula's Domain")]
    ChayulasDomain,
    #[serde(alias = "Uul-Netol's Domain")]
    UulNetolsDomain,
    #[serde(alias = "Esh's Domain")]
    EshsDomain,
    #[serde(alias = "Xoph's Domain")]
    XophsDomain,
    #[serde(alias = "Tul's Domain")]
    TulsDomain,
}

//   Chayula, Who Dreamt; Uul-Netol, Unburdened Flesh
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BreachlordBossDomain {
    #[serde(alias = "Xoph, Dark Embers")]
    Xoph,
    #[serde(alias = "Tul, Creeping Avalanche")]
    Tul,
    #[serde(alias = "Esh, Forked Thought")]
    Esh,
    #[serde(alias = "Chayula, Who Dreamt")]
    Chayula,
    #[serde(alias = "Uul-Netol, Unburdened Flesh")]
    UulNetol,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Architect {
    #[serde(alias = "Zilquapa, Architect of the Breach")]
    Zilquapa,
    #[serde(alias = "Paquate, Architect of Corruption")]
    Paquate,
    #[serde(alias = "Ahuana, Architect of Ceremonies")]
    Ahuana,
    #[serde(alias = "Zalatl, Architect of Thaumaturgy")]
    Zalatl,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BetrayalSyndicateMember {
    #[serde(alias = "Haku", alias = "Haku, Warmaster")]
    Haku,
    #[serde(alias = "Elreon")]
    Elreon,
    #[serde(alias = "Tora")]
    Tora,
    #[serde(alias = "Vagan")]
    Vagan,
    #[serde(alias = "Vorici")]
    Vorici,
    #[serde(alias = "Hillock, the Blacksmith")]
    Hillock,
    #[serde(alias = "Leo, Wolf of the Pits")]
    Leo,
    #[serde(alias = "Guff \"Tiny\" Grenn")]
    GuffTinyGrenn,
    #[serde(alias = "Janus Perandus")]
    JanusPerandus,
    #[serde(alias = "It That Fled")]
    ItThatFled,
    #[serde(alias = "Gravicius")]
    Gravicius,
    #[serde(alias = "Thane Jorgin")]
    ThandeJorgin,
    #[serde(alias = "Korell Goya")]
    KorellGoya,
    #[serde(alias = "Rin Yuushu")]
    RinYuushu,
    #[serde(alias = "Cameria the Coldblooded")]
    CameriaTheColdblooded,
    #[serde(alias = "Aisling Laffrey")]
    AislingLaffrey,
    #[serde(alias = "Riker Maloney")]
    RikerMaloney,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RogueExile {
    #[serde(alias = "Ash Lessard")]
    AshLessard,
    #[serde(alias = "Magnus Stonethorn")]
    Magnus,
    #[serde(alias = "Minara Anemina")]
    Minara,
}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Clone, Debug)]
pub enum Vendor {
    #[serde(alias = "Kirac shop")]
    KiracShop,
}

impl Vendor {
    pub fn iter() -> Iter<'static, Vendor> {
        static VENDORS: [Vendor; 1] = [Vendor::KiracShop];
        VENDORS.iter()
    }
}

impl Display for Vendor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Vendor::KiracShop => write!(f, "Kirac shop"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use serde_json::json;

    use crate::{
        dropconsts::{ACT_AREA_NAMES, AREA_NAMES, BOSS_NAMES, CHESTS},
        dropsource::DropSource,
        maps::Map,
    };

    #[tokio::test]
    async fn parses_dropsources_from_wiki_map_monster_agreements_column() {
        crate::scripts::update_all_jsons().await;
        let maps: Vec<Map> =
            serde_json::from_str(&std::fs::read_to_string("maps.json").unwrap()).unwrap();
        let maps: Vec<String> = maps.into_iter().map(|m| m.name).collect();
        let maps_without_the: Vec<String> = maps.iter().map(|m| m.replace(" Map", "")).collect();

        let drops: Vec<Option<String>> =
            serde_json::from_str(&std::fs::read_to_string("drops-from.json").unwrap()).unwrap();

        let mut sources = Vec::new();
        for drop in drops {
            let Some(drop_str) = drop else {
                continue;
            };

            let drop_str = drop_str.replace("\r\n", "");
            let mut drop_str = drop_str.replace("\n", "");

            if drop_str.ends_with(";") {
                println!("drop_str ends with ; {}", &drop_str);
                drop_str.drain(drop_str.len() - 1..);
            }

            for s in drop_str.split(";") {
                let s = s.trim();

                if CHESTS.contains(&s) {
                    continue;
                }

                if ACT_AREA_NAMES.contains(&s) {
                    continue;
                }

                if BOSS_NAMES.contains(&s) {
                    continue;
                }

                if AREA_NAMES.contains(&s) {
                    continue;
                }

                if let Ok(_dropsource) = serde_json::from_str::<DropSource>(&json!(s).to_string()) {
                    continue;
                }

                let string = s.to_string();
                if maps.contains(&string) || maps_without_the.contains(&string) {
                    continue;
                }

                sources.push(s.to_owned());
            }
        }

        let sources: HashSet<String> = HashSet::from_iter(sources.iter().cloned());

        // std::fs::write(
        //     "dropsources.json",
        //     &serde_json::to_string_pretty(&sources).unwrap(),
        // )
        // .unwrap();

        assert_eq!(sources.len(), 0);
    }
}
