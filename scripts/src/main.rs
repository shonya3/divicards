pub mod cards;
pub mod consts;
pub mod error;
pub mod maps;
pub mod reward;
pub mod scripts;

use std::collections::HashSet;
#[allow(unused)]
use std::{collections::HashMap, fmt::Display, slice::Iter};

use divi::{league::TradeLeague, prices::NinjaCardData, sample::fix_name, IsCard};
use maps::Map;
use reward::reward_to_html;
use serde::{Deserialize, Serialize};

use error::Error;
use serde_json::{json, Value};

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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum DropSource {
    ExpeditionLogbook,
    GlobalDrop,
    ChestObject,
    Map(String),
    MapBoss {
        boss: String,
        map: String,
    },
    Disabled,
    Unknown,
    Delirium,
    Vendor(Option<Vendor>),
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

pub const BOSS_NAMES: &'static [&'static str] = &[
    "Tormented Temptress (Whakawairua Tuahu)",
    "Avatar of the Huntress",
    "Rose",
    "Malachai's Omen",
    "Caer Blaidd",
    "Caer Blaidd, Wolfpack's Den",
    "Helial, the Day Unending",
    "Ghorr, the Grasping Maw",
    "The Vindicated Queen",
    "Kurgal, the Blackblooded",
    "Aul, the Crystal King",
    "The Infinite Hunger",
    "Fenumus, First of the Night",
    "Infector of Dreams",
    "Tahsin, Warmaker",
    "Stone of the Currents",
    "Nightmare's Omen",
    "Fragment of Winter",
    "Gorulis, Will-Thief (Infested Valley)",
    "Titan of the Grove",
    "Calderus",
    "Avatar of the Skies",
    "Avatar of Undoing",
    "Varhesh, Shimmering Aberration",
    "Erebix, Light's Bane",
    "Lord of the Hollows",
    "Messenger of the Hollows",
    "Champion of the Hollows",
    "Entity of the Void",
    "Opid, Helial's Herald",
    "Skullbeak",
    "Headmistress Braeta",
    "Eyepecker",
    "Gorulis, Will-Thief",
    "Mephod, the Earth Scorcher",
    "Ara, Sister of Light",
    "Flesh Sculptor",
    "Corpse Stitcher",
    "Nightwane",
    "The Animal Pack",
    "Stonebeak, Battle Fowl",
    "Jorus, Sky's Edge",
    "Shrieker Eihal",
    "Gisale, Thought Thief",
    "K'aj Y'ara'az",
    "Uul-Netol, Unburdened Flesh",
    "Olof, Son of the Headsman",
    "Chayula, Who Dreamt",
    "Farrul, First of the Plains",
    "Uhtred, Covetous Traitor",
    "Omniphobia, Fear Manifest",
    "Kosis, The Revelation",
    "Merveil, the Reflection (Underground Sea)",
    "Fairgraves",
    "Avatar of the Forge",
    "Eater of Souls",
    "The Cleansing Light",
    "Xixic, High Necromancer",
    "Armala, the Widow",
    "Ambrius, Legion Slayer",
    "Drek, Apex Hunter",
    "Bazur",
    "Thought Thief",
    "Prodigy of Darkness",
    "Shavronne the Sickening",
    "Amalgam of Nightmares",
    "Wolf of the Pits",
    "The High Templar",
    "The Bone Sculptor",
    "The Maven",
    "Oshabi, Avatar of the Grove",
    "Beast of the Pits",
    "Tolman, the Exhumer",
    "Burtok, Shaper of Bones",
    "K'aj Q'ura",
    "The Brittle Emperor",
    "Uber Elder",
    "The Searing Exarch",
    "Catarina, Master of Undeath",
    "Gnar, Eater of Carrion",
    "Argus",
    "The Hallowed Husk",
    "The Cursed King",
    "The Eater of Worlds",
    "Drought-Maddened Rhoa",
    "Doedre the Defiler",
    "Khor, Sister of Shadows",
    "Murgeth Bogsong",
    "Stalker of the Endless Dunes",
    "Telvar, the Inebriated",
    "Nassar, Lion of the Seas",
    "Venarius",
    "Void Anomaly",
    "Maligaro the Mutilator",
    "Merveil, the Returned",
    "The Elder",
    "Ryslatha, the Puppet Mistress",
    "Voll, Emperor of Purity",
    "Portentia, the Foul",
    "Portentia, the Foul (Waste Pool)",
    "Erythrophagia (Phantasmagoria)",
    "Erythrophagia",
    "Hephaeus, The Hammer",
    "Selenia, the Endless Night",
    "The Brine King's Reef",
    "K'tash, the Hate Shepherd",
    "Prodigy of Hexes",
    "Excellis Aurafix",
    "Guardian of the Vault",
    "Terror of the Infinite Drifts",
    "The Eroding One",
    "The Winged Death",
    "Elida Blisterclaw (Bramble Valley)",
    "Fire and Fury (Lava Chamber)",
    "Merveil, the Reflection (Maelstrom of Chaos)",
    "Shock and Horror (Mineral Pools)",
    "Steelpoint the Avenger",
    "The Sanguine Siren",
    "K'aj A'alai (Vaal Temple Trio)",
    "The Dishonoured Queen",
    "Steelpoint the Avenger",
    "Mirage of Bones",
    "Lord of the Ashen Arrow",
    "Wolfpack's Den",
];

pub const AREA_NAMES: &'static [&'static str] = &["The Apex of Sacrifice", "The Alluring Abyss"];

pub const ACT_AREA_NAMES: &'static [&'static str] = &[
    "Kitava, The Destroyer (The Destroyer's Heart)",
    "The Imperial Fields",
    "The Fellshrine Ruins",
    "The Quay",
    "The Ascent",
    "The Causeway",
    "The Grand Arena",
    "Shavronne's Tower",
    "The Sarn Ramparts",
    "The Docks",
    "Oriath Square",
    "The High Gardens",
    "The Slums",
    "The Vastiri Desert",
    "The Feeding Trough",
    "The Oasis",
    "The Ebony Barracks",
    "The Grain Gate",
    "Kaom's Dream",
    "The Lunaris Concourse",
    "The Ancient Pyramid",
    "The Aqueduct",
    "The Grand Promenade",
    "The Boiling Lake",
    "Reliquary",
    "The Catacombs",
    "The Toxic Conduits",
    "The Sewers",
    "The Archives",
    "Daresso's Dream",
    "The Marketplace",
    "The Sceptre of God",
    "The Refinery",
    "The Solaris Temple Level 1 (A3)",
    "The Lunaris Temple Level 2 (Act 3)",
    "The Solaris Concourse",
    "The Mines Level 1",
    "The Mines Level 2",
    "The Belly of the Beast (A4/A9)",
    "The Belly of the Beast (A4/9)",
    "The Belly of the Beast",
    "The Cathedral Rooftop (A5)",
    "The Mud Flats (A6)",
    "The Mud Flats (Act 6)",
    "The Lower Prison (Act 6)",
    "Lower Prison (Act 6)",
    "The Riverways (Act 6)",
    "The Tidal Island (A6)",
    "Prisoner's Gate (A6)",
    "The Twilight Strand (A6)",
    "The Chamber of Sins Level 1 (Act 7)",
    "Chamber of Sins Level 1/2 (Act 7)",
    "The Chamber of Sins Level 1/2 (Act 7)",
    "The Crossroads",
    "The Crossroads (A7)",
    "The Crossroads (Act 7)",
    "The Crypt (Act 7)",
    "The Crypt",
    "The Solaris Temple Level 1 (Act 8)",
    "The Solaris Temple 1/2 (A8)",
    "The Lunaris Temple Level 2 (Act 8)",
    "The Fellshrine Ruins (A7)",
    "The Ossuary (Act 5)",
    "The Ossuary (Act 10)",
    "The Ossuary (Act 5/10)",
    "The Ossuary",
    "Maligaro's Sanctum",
    "The Bath House",
    "The Battlefront",
    "The Ashen Fields",
    "The Beacon",
    "The Blood Aqueduct",
    "The Broken Bridge",
    "Control BLocks",
    "The Control Blocks",
    "The Control Blocks (Act 5)",
    "The Harbour Bridge, The Solaris Temple Level 2 (Act 8)",
    "The Solaris Temple Level 2 (Act 8)",
    "Temple, The Twilight Temple, Cold River, The Solaris Temple Level 1 (Act 8)",
    "The Coast (A6)",
    "The Cavern of Anger (Act 6)",
    "The Southern Forest (Act 6)",
    "The Den (Act 7)",
    "The Torched Courts (A5/10)",
    "Lunaris Temple Level 1/2 (A8)",
    "The Library",
    "The Crystal Veins",
    "The Reliquary",
    "The Reliquary (Act 5/10)",
    "The Desecrated Chambers",
    "The Foothills",
    "The Harbour Bridge",
    "The Ravaged Square",
    "The Ruined Square",
    "The Slave Pens",
    "The Quarry",
    "The Temple of Decay Level 1",
    "The Temple of Decay Level 2",
    "The Temple of Decay Level 1/2",
    "The Vasitri Desert",
    "The Dread Thicket (A7)",
    "The Northern Forest (A7)",
    "Chamber of Innocence (A5/A10)",
    "The Descent",
    "The Crematorium",
    "The Tunnel",
    "The Harvest",
    "The Chamber of Innocence",
    "Kaom's Stronghold",
    "The Northern Forest",
    "The Karui Fortress",
    "The Vaal City",
    "The Imperial Gardens",
    "The Rotting Core",
    "The City of Sarn",
    "The Upper Sceptre of God",
];

pub const CHESTS: &'static [&'static str] =
    &["Light Jewellery chest (Primeval Ruins, Abyssal City, Vaal Outpost)"];

#[tokio::main]
async fn main() {
    scripts::update_all_jsons().await;
    let maps: Vec<Map> =
        serde_json::from_str(&std::fs::read_to_string("maps.json").unwrap()).unwrap();
    let maps = maps.into_iter().map(|m| m.name).collect::<Vec<String>>();
    let maps_without_the: Vec<String> = maps.iter().map(|m| m.replace(" Map", "")).collect();

    let drops: Vec<Option<String>> =
        serde_json::from_str(&std::fs::read_to_string("drops-from.json").unwrap()).unwrap();

    let mut sources = Vec::new();
    for drop in drops {
        let Some(drop_str) = drop else {
            continue;
        };

        let drop_str = drop_str.replace(";\r\n", ";");
        let drop_str = drop_str.replace(";\n", ";");
        let drop_str = drop_str.replace(",\r\n", ",");
        let drop_str = drop_str.replace(",\n", ",");

        let mut separator = ",";
        if drop_str.contains("\n") {
            separator = "\n"
        };

        if drop_str.contains(";") {
            separator = ";"
        }

        if drop_str.contains(";") && drop_str.contains("\n") {
            panic!("Drop string contains ; and \\n at same time {}", &drop_str);
        }

        if separator == "," {
            if drop_str.matches(",").count() == 1 {
                if BOSS_NAMES.contains(&drop_str.as_str()) {
                    continue;
                }

                let dropsource = serde_json::from_str::<DropSource>(&json!(drop_str).to_string());
                if dropsource.is_ok() {
                    dbg!(&drop_str);
                    continue;
                }
            };
        };

        let droplist = drop_str.split(separator);
        for s in droplist {
            let s = s.trim().to_string();

            if maps.contains(&s) || maps_without_the.contains(&s) {
                continue;
            }

            if CHESTS.contains(&s.as_str()) {
                continue;
            }

            if ACT_AREA_NAMES.contains(&s.as_str()) {
                continue;
            }

            if BOSS_NAMES.contains(&s.as_str()) {
                continue;
            }

            if AREA_NAMES.contains(&s.as_str()) {
                continue;
            }

            let area_specific = serde_json::from_str::<AreaSpecific>(&json!(s).to_string());
            if area_specific.is_ok() {
                continue;
            }

            let betryal_syndycate_member =
                serde_json::from_str::<BetrayalSyndicateMember>(&json!(s).to_string());
            if betryal_syndycate_member.is_ok() {
                continue;
            }

            let story_boss = serde_json::from_str::<StoryBoss>(&json!(s).to_string());
            if story_boss.is_ok() {
                continue;
            }

            let abyss_lich_boss = serde_json::from_str::<AbyssLichBoss>(&json!(s).to_string());
            if abyss_lich_boss.is_ok() {
                continue;
            }

            let breachlord_boss_domain =
                serde_json::from_str::<BreachlordBossDomain>(&json!(s).to_string());
            if breachlord_boss_domain.is_ok() {
                continue;
            }

            let architect = serde_json::from_str::<Architect>(&json!(s).to_string());
            if architect.is_ok() {
                continue;
            }

            let rogue_exile = serde_json::from_str::<RogueExile>(&json!(s).to_string());
            if rogue_exile.is_ok() {
                continue;
            }

            let dropsource = serde_json::from_str::<DropSource>(&json!(s).to_string());
            if dropsource.is_ok() {
                // dbg!(&dropsource);
                continue;
            }

            sources.push(s.to_owned());
        }
    }

    let sources: HashSet<String> = HashSet::from_iter(sources.iter().cloned());
    dbg!(sources.len());

    std::fs::write(
        "dropsources.json",
        &serde_json::to_string_pretty(&sources).unwrap(),
    )
    .unwrap();
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DivinationCardElementData {
    pub name: String,
    pub art_filename: String,
    pub reward_html: String,
    pub flavour_text: String,
    pub stack_size: Option<usize>,
}

impl DivinationCardElementData {
    pub async fn write_data() {
        let vec: Vec<NinjaCardData> = NinjaCardData::fetch(&TradeLeague::default()).await.unwrap();
        let v: Vec<DivinationCardElementData> = vec
            .into_iter()
            .map(|data| {
                let mut fl = data.flavour_text;
                if fl.starts_with("<size:") {
                    fl = fl[10..fl.len() - 1].to_string();
                }

                if fl.starts_with("<smaller>{") {
                    fl = fl[10..fl.len() - 1].to_string();
                }

                DivinationCardElementData {
                    name: data.name,
                    art_filename: data.art_filename,
                    flavour_text: fl,
                    stack_size: data.stack_size,
                    reward_html: reward_to_html(&data.explicit_modifiers[0].text),
                }
            })
            .collect();

        std::fs::write("data.json", serde_json::to_string(&v).unwrap()).unwrap();
    }
}

#[allow(unused)]
use crate::scripts::{parse_table, read_original_table_sheet};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum GreyNote {
    #[serde(alias = "Monster-specific")]
    MonsterSpecific,
    #[serde(alias = "Area-specific")]
    AreaSpecific,
    #[serde(alias = "disabled", alias = "Drop disabled")]
    Disabled,
    #[serde(alias = "story")]
    Story,
    #[serde(alias = "Delirium_reward")]
    Delirium,
    #[serde(alias = "Chest_object", alias = "Chest_obkect")]
    ChestObject,
    #[serde(alias = "strongbox")]
    Strongbox,
    #[serde(alias = "Global Drop")]
    GlobalDrop,
    #[serde(alias = "Vendor")]
    Vendor,
}

impl GreyNote {
    pub fn parse(val: &Value) -> Result<Option<Self>, Error> {
        let Some(s) = val.as_str() else {
            return Ok(None);
        };
        if s.is_empty() || s == "n/a" {
            return Ok(None);
        } else {
            let greynote = serde_json::from_str(&val.to_string())?;
            Ok(greynote)
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Confidence {
    #[serde(alias = "none")]
    None,
    #[serde(alias = "Low", alias = "low")]
    Low,
    #[serde(alias = "OK", alias = "ok")]
    Ok,
    #[serde(alias = "DONE", alias = "Done")]
    Done,
}

impl Confidence {
    pub fn parse(val: &Value) -> Result<Self, Error> {
        let conf: Confidence = serde_json::from_str(&val.to_string())?;
        Ok(conf)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum RemainingWork {
    #[serde(alias = "confirm")]
    Confirm,
    #[serde(alias = "unclear hypothesis")]
    UnclearHypothesis,
    #[serde(alias = "no hypothesis")]
    NoHypothesis,
    #[serde(alias = "story only")]
    StoryOnly,
    #[serde(alias = "legacy tag")]
    LegacyTag,
    #[serde(alias = "open ended")]
    OpenEnded,
}

impl RemainingWork {
    pub fn parse(val: &Value) -> Result<Option<Self>, Error> {
        let Some(s) = val.as_str() else {
            return Ok(None);
        };
        if s.is_empty() || s == "n/a" {
            return Ok(None);
        } else {
            let remaining_work = serde_json::from_str(&val.to_string())?;
            Ok(remaining_work)
        }
    }
}

pub fn parse_greynote(val: &Value) -> Result<Option<GreyNote>, Error> {
    GreyNote::parse(val)
}

pub fn parse_name(val: &Value) -> Result<String, Error> {
    let Some(second_column_contents) = val.as_str() else {
        return Err(Error::ValueNotStr(val.to_owned()));
    };

    match second_column_contents.is_card() {
        true => Ok(second_column_contents.to_string()),
        false => match fix_name(second_column_contents) {
            Some(s) => Ok(s),
            None => Err(Error::ParseNameError(second_column_contents.to_string())),
        },
    }
}

pub fn parse_confidence(val: &Value) -> Result<Confidence, Error> {
    Confidence::parse(val)
}

pub fn parse_remaining_work(val: &Value) -> Result<Option<RemainingWork>, Error> {
    RemainingWork::parse(val)
}

pub fn parse_string_cell(val: &Value) -> Option<String> {
    let Some(s) = val.as_str() else { return None };
    if s.is_empty() || s == "n/a" {
        return None;
    } else {
        return Some(s.to_string());
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CardDropRecord {
    pub greynote: Option<GreyNote>,
    pub name: String,
    pub tag_hypothesis: Option<String>,
    pub confidence: Confidence,
    pub remaining_work: Option<RemainingWork>,
    pub drops_from: Option<String>,
    pub wiki_disagreements: Option<String>,
    pub sources_with_tag_but_not_on_wiki: Option<String>,
    pub notes: Option<String>,
}

pub fn parse_row(row: &[Value]) -> Result<CardDropRecord, Error> {
    let greynote = parse_greynote(&row[0])?;
    let name = parse_name(&row[1])?;
    let tag_hypothesis = parse_string_cell(&row[2]);
    let confidence = parse_confidence(&row[3])?;
    let remaining_work = parse_remaining_work(&row[4])?;
    let drops_from = row.get(5).map(|val| parse_string_cell(val)).flatten();
    let wiki_disagreements = row.get(6).map(|val| parse_string_cell(val)).flatten();
    let sources_with_tag_but_not_on_wiki = row.get(7).map(|val| parse_string_cell(val)).flatten();
    let notes = row.get(8).map(|val| parse_string_cell(val)).flatten();

    Ok(CardDropRecord {
        greynote,
        name,
        tag_hypothesis,
        confidence,
        remaining_work,
        drops_from,
        wiki_disagreements,
        sources_with_tag_but_not_on_wiki,
        notes,
    })
}

pub fn parse_drop_source(record: &CardDropRecord) -> Result<Vec<DropSource>, Error> {
    let mut sources: Vec<DropSource> = Vec::new();

    if let Some(tag_hypothesis) = &record.tag_hypothesis {
        if tag_hypothesis.contains("logbook") {
            sources.push(DropSource::ExpeditionLogbook);
        }
    }

    if let Some(greynote) = &record.greynote {
        if greynote == &GreyNote::Disabled {
            sources.push(DropSource::Disabled);
        }

        if greynote == &GreyNote::Vendor {
            if let Some(_drops_from) = &record.drops_from {}
            // return Ok(DropSource::Vendor());
        }
    }

    // match greynote {
    //     GreyNote::Disabled => return Ok(DropSource::Disabled),
    //     GreyNote::Delirium => return Ok(DropSource::Delirium),
    //     GreyNote::ChestObject => return Ok(DropSource::ChestObject),
    //     GreyNote::GlobalDrop => return Ok(DropSource::GlobalDrop),
    //     GreyNote::Vendor => return Ok(DropSource::Vendor),
    //     GreyNote::Strongbox => return Ok(DropSource::Strongbox),
    //     GreyNote::AreaSpecific => todo!(),
    //     GreyNote::MonsterSpecific => todo!(),
    //     GreyNote::Story => todo!(),
    // }

    // let sources = sources.into_iter().unique();

    Ok(sources)
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
    use serde_json::json;

    use crate::parse_greynote;

    use super::*;

    #[test]
    fn parses_table_without_errors() {
        let sheet = read_original_table_sheet("sheet.json").unwrap();
        for row in &sheet.values[2..] {
            parse_row(row).unwrap();
        }
    }

    #[test]
    fn test_parse_greynote() {
        let sheet = read_original_table_sheet("sheet.json").unwrap();
        let mut vec: Vec<Vec<Value>> = vec![];
        for val in &sheet.values {
            if let Err(_) = parse_greynote(&val[0]) {
                vec.push(val.to_owned());
                dbg!(val);
            }
        }
        assert_eq!(vec.len(), 0);

        assert_eq!(
            Some(GreyNote::AreaSpecific),
            parse_greynote(&json!("Area-specific")).unwrap()
        );
        assert_eq!(
            Some(GreyNote::MonsterSpecific),
            parse_greynote(&json!("Monster-specific")).unwrap()
        );
        assert_eq!(
            Some(GreyNote::Disabled),
            parse_greynote(&json!("disabled")).unwrap()
        );
        assert_eq!(
            Some(GreyNote::Disabled),
            parse_greynote(&json!("disabled")).unwrap()
        );
        assert_eq!(
            Some(GreyNote::Disabled),
            parse_greynote(&json!("Drop disabled")).unwrap()
        );
        assert_eq!(
            Some(GreyNote::Story),
            parse_greynote(&json!("story")).unwrap()
        );
        assert_eq!(
            Some(GreyNote::Delirium),
            parse_greynote(&json!("Delirium_reward")).unwrap()
        );
        assert_eq!(
            Some(GreyNote::ChestObject),
            parse_greynote(&json!("Chest_object")).unwrap()
        );
        assert_eq!(
            Some(GreyNote::ChestObject),
            parse_greynote(&json!("Chest_obkect")).unwrap()
        );
        assert_eq!(
            Some(GreyNote::Strongbox),
            parse_greynote(&json!("strongbox")).unwrap()
        );
        assert_eq!(
            Some(GreyNote::GlobalDrop),
            parse_greynote(&json!("Global Drop")).unwrap()
        );
        assert_eq!(
            Some(GreyNote::Vendor),
            parse_greynote(&json!("Vendor")).unwrap()
        );
        assert_eq!(None, parse_greynote(&json!("")).unwrap());
        assert_eq!(None, parse_greynote(&json!("n/a")).unwrap());
    }

    #[test]
    fn test_parse_name() {
        let sheet = read_original_table_sheet("sheet.json").unwrap();
        let mut vec: Vec<Vec<Value>> = vec![];
        for val in &sheet.values[2..] {
            if let Err(_) = super::parse_name(&val[1]) {
                vec.push(val.to_owned());
            }
        }

        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn test_parse_confidence() {
        assert_eq!(Confidence::Done, parse_confidence(&json!("DONE")).unwrap());
        assert_eq!(Confidence::Low, parse_confidence(&json!("Low")).unwrap());
        assert_eq!(Confidence::Low, parse_confidence(&json!("low")).unwrap());
        assert_eq!(Confidence::None, parse_confidence(&json!("none")).unwrap());
        assert_eq!(Confidence::Ok, parse_confidence(&json!("OK")).unwrap());
        assert_eq!(Confidence::Ok, parse_confidence(&json!("ok")).unwrap());
    }

    #[test]
    fn test_parse_remaining_work() {
        let sheet = read_original_table_sheet("sheet.json").unwrap();
        let mut vec: Vec<Vec<Value>> = vec![];
        for val in &sheet.values[2..] {
            if val.len() < 5 {
                continue;
            }
            if let Err(_) = parse_remaining_work(&val[4]) {
                vec.push(val.to_owned());
            }
        }
        assert_eq!(vec.len(), 0);

        assert_eq!(
            Some(RemainingWork::Confirm),
            parse_remaining_work(&json!("confirm")).unwrap()
        );
        assert_eq!(
            Some(RemainingWork::UnclearHypothesis),
            parse_remaining_work(&json!("unclear hypothesis")).unwrap()
        );
        assert_eq!(
            Some(RemainingWork::NoHypothesis),
            parse_remaining_work(&json!("no hypothesis")).unwrap()
        );
        assert_eq!(
            Some(RemainingWork::StoryOnly),
            parse_remaining_work(&json!("story only")).unwrap()
        );
        assert_eq!(None, parse_remaining_work(&json!("n/a")).unwrap());
        assert_eq!(
            Some(RemainingWork::LegacyTag),
            parse_remaining_work(&json!("legacy tag")).unwrap()
        );

        assert_eq!(
            Some(RemainingWork::OpenEnded),
            parse_remaining_work(&json!("open ended")).unwrap()
        );

        assert_eq!(None, parse_remaining_work(&json!("")).unwrap());
    }
}

pub fn temp_main() {
    let sheet = read_original_table_sheet("sheet.json").unwrap();
    let records = parse_table(&sheet.values[2..]).unwrap();

    let mut confidence_map: HashMap<Confidence, u16> = HashMap::new();
    for record in &records {
        let counter = confidence_map.entry(record.confidence.clone()).or_insert(0);
        *counter += 1;
    }

    dbg!(confidence_map);

    let mut map: HashMap<String, Vec<CardDropRecord>> = HashMap::new();
    for record in records {
        let vec = map.entry(record.name.as_str().to_owned()).or_insert(vec![]);
        vec.push(record);
    }

    dbg!(map.keys().len());
    std::fs::write("map.json", serde_json::to_string_pretty(&map).unwrap()).unwrap();

    let mut multiple_map: HashMap<String, Vec<CardDropRecord>> = HashMap::new();
    for (name, record) in map {
        if record.len() > 1 {
            multiple_map.insert(name.clone(), record.clone());
        }
    }

    dbg!(multiple_map.keys().len());
    std::fs::write(
        "multiple-map.json",
        serde_json::to_string_pretty(&multiple_map).unwrap(),
    )
    .unwrap();

    let mut _map: HashMap<&CardDropRecord, Vec<HashSet<DropSource>>> = HashMap::new();

    let mut set: HashSet<DropSource> = HashSet::new();
    set.insert(DropSource::ChestObject);
    set.insert(DropSource::ExpeditionLogbook);
    set.insert(DropSource::Vendor(Some(Vendor::KiracShop)));
    set.insert(DropSource::Vendor(Some(Vendor::KiracShop)));
    dbg!(set);

    let sheet = read_original_table_sheet("sheet.json").unwrap();
    let records = parse_table(&sheet.values[2..]).unwrap();

    for record in records {
        let drop_source = parse_drop_source(&record).unwrap();
        if drop_source.contains(&DropSource::ExpeditionLogbook) {
            dbg!(record);
        }
    }
    // std::fs::write("map.json", &serde_json::to_string_pretty(&map).unwrap()).unwrap();
}

pub fn write_sized_rewards() {
    let vec: Vec<NinjaCardData> =
        serde_json::from_str(&std::fs::read_to_string("ninja-data.json").unwrap()).unwrap();
    let mut with_size: Vec<String> = Vec::new();
    for card_data in vec {
        let reward = &card_data.explicit_modifiers[0].text;
        if reward.contains("<size:") {
            with_size.push(reward.clone());
        }
    }

    std::fs::write(
        "rewards-with-size.json",
        serde_json::to_string(&with_size).unwrap(),
    )
    .unwrap();
}
