use std::collections::HashMap;

use csv::{ReaderBuilder, Trim};
use serde::{Deserialize, Serialize};

use crate::{
    card_record::DivinationCardRecord,
    cards::Cards,
    consts::{CARDS, RAIN_OF_CHAOS_WEIGHT},
    error::Error,
    prices::Prices,
    IsCard,
};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DivinationCardsSample {
    pub cards: Cards,
    pub not_cards: Vec<String>,
    pub fixed_names: Vec<FixedCardName>,
    pub csv: String,
}

impl DivinationCardsSample {
    pub fn new(
        cards: Cards,
        not_cards: Vec<String>,
        fixed_names: Vec<FixedCardName>,
        csv: String,
    ) -> DivinationCardsSample {
        DivinationCardsSample {
            cards,
            not_cards,
            fixed_names,
            csv,
        }
    }

    /// Create a new sample.
    /// # Examples
    /// ```
    /// # use divi::sample::{DivinationCardsSample, SampleData, CardNameAmount};
    /// # use divi::prices::Prices;
    /// # fn main() -> Result<(), divi::error::Error> {
    ///     // create sample from csv
    ///     let sample = DivinationCardsSample::create(
    ///         SampleData::Csv(String::from("name,amount\rRain of Chaos,2\rThe Doctor,1")),
    ///         None,
    ///     )?;
    /// #    Ok(())
    /// # }
    /// ```
    ///
    /// ```
    /// # use divi::sample::{DivinationCardsSample, SampleData, CardNameAmount};
    /// # use divi::prices::Prices;
    /// # fn main() -> Result<(), divi::error::Error> {
    ///     // create sample from name-amount list
    ///    let sample = DivinationCardsSample::create(
    ///        SampleData::CardNameAmountList(vec![
    ///             CardNameAmount::new(String::from("Rain of Chaos"), 25),
    ///             CardNameAmount::new(String::from("The Doctor"), 1),
    ///        ]),
    ///        Some(Prices::default()),
    ///    )?;
    /// #    Ok(())
    /// # }
    /// ```
    #[tracing::instrument(skip(source, prices))]
    pub fn create(
        source: SampleData,
        prices: Option<Prices>,
    ) -> Result<DivinationCardsSample, Error> {
        let mut sample = DivinationCardsSample::from_prices(prices);
        let parsed = sample.parse_data(source)?;
        parsed.get_sample_ready();
        Ok(sample)
    }

    /// Merge samples into one sample
    /// # Examples
    /// ```
    ///# use divi::sample::{CardNameAmount, DivinationCardsSample, SampleData};
    ///# fn main() -> Result<(), divi::error::Error> {
    ///     let s1 = DivinationCardsSample::create(
    ///         SampleData::Csv(String::from("name,amount\rRain of Chaos,30")),
    ///         None,
    ///     )?;
    ///     let vec: Vec<CardNameAmount> = vec![CardNameAmount::new(String::from("Rain of Caos"), 25)];
    ///     let s2 = DivinationCardsSample::create(SampleData::CardNameAmountList(vec), None)?;
    ///     let merged = DivinationCardsSample::merge(None, &[s1, s2]);
    ///     assert_eq!(merged.cards.get_card("Rain of Chaos").amount, 55);
    ///#     Ok(())
    ///# }
    /// ```
    pub fn merge(
        prices: Option<Prices>,
        samples: &[DivinationCardsSample],
    ) -> DivinationCardsSample {
        let mut merged = DivinationCardsSample::from_prices(prices);

        for card in merged.cards.iter_mut() {
            let amount = samples
                .iter()
                .map(|sample| sample.cards.get_card(&card.name).amount)
                .sum::<u32>();
            card.set_amount_and_sum(amount);
        }

        merged.get_sample_ready();
        merged
    }

    pub fn print_not_nullish(&self) {
        let sample = self.to_owned();
        let not_nullish = NotNullishSample::from(sample);
        println!("{}", not_nullish.csv);
    }

    pub fn into_not_nullish(self) -> NotNullishSample {
        self.into()
    }

    /// Consumes Prices structure to set prices for Cards
    fn from_prices(prices: Option<Prices>) -> Self {
        DivinationCardsSample {
            cards: Cards::from(prices.unwrap_or_default()),
            ..Default::default()
        }
    }

    /// Parsing helper. Uses for CSV data
    fn remove_lines_before_headers(s: &str) -> Result<String, Error> {
        match s.lines().enumerate().into_iter().find(|(_index, line)| {
            line.contains("name")
                && ["amount", "stackSize"]
                    .iter()
                    .any(|variant| line.contains(variant))
        }) {
            Some((index, _line)) => Ok(s
                .lines()
                .into_iter()
                .skip(index)
                .collect::<Vec<&str>>()
                .join("\r\n")),
            None => Err(Error::MissingHeaders),
        }
    }

    /// Reads the source to extract an amount of cards for each card name
    fn parse_data(&mut self, source: SampleData) -> Result<&mut Self, Error> {
        match source {
            SampleData::Csv(s) => {
                let data = Self::remove_lines_before_headers(&s)?;
                let mut rdr = ReaderBuilder::new()
                    .trim(Trim::All)
                    .from_reader(data.as_bytes());

                for result in rdr.deserialize::<CardNameAmount>() {
                    match result {
                        Ok(card_name_amount) => self.extract_amount(card_name_amount),
                        Err(err) => println!("{:?}", err),
                    }
                }
                Ok(self)
            }
            SampleData::CardNameAmountList(vec) => {
                for card_name_amount in vec {
                    self.extract_amount(card_name_amount)
                }
                Ok(self)
            }
        }
    }

    /// The part of parsing data process. Extracts  amount from individual name-amount source. If name is not card, tries to fix the name
    /// and pushes to fixed_names or to not_cards if fails.
    fn extract_amount(&mut self, source: CardNameAmount) {
        if source.name.as_str().is_card() {
            let mut_card = self.cards.get_card_mut(&source.name);
            mut_card.set_amount_and_sum(mut_card.amount + source.amount);
        } else {
            match fix_name(&source.name) {
                Some(fixed) => {
                    self.fixed_names.push(FixedCardName {
                        old: source.name.clone(),
                        fixed: fixed.clone(),
                    });
                    let mut_card = self.cards.get_card_mut(&fixed);
                    mut_card.set_amount_and_sum(mut_card.amount + source.amount);
                }
                None => self.not_cards.push(source.name),
            }
        }
    }

    /// Writes weights for cards and writes final csv - write_weight and write_csv in one function
    fn get_sample_ready(&mut self) -> &mut Self {
        self.write_weight().write_csv()
    }

    /// Helper function for write_weight
    fn weight_multiplier(&self) -> f32 {
        let rain_of_chaos = self
            .cards
            .get("Rain of Chaos")
            .expect("no rain of chaos card");
        RAIN_OF_CHAOS_WEIGHT / rain_of_chaos.amount as f32
    }

    /// (After parsing) Calculates special weight for each card and mutates it. Runs at the end of parsing.
    fn write_weight(&mut self) -> &mut Self {
        let weight_multiplier = self.weight_multiplier();

        for card in self.cards.iter_mut() {
            card.set_weight(weight_multiplier);
        }

        self
    }

    /// (After weight) Sets .csv field. Must be used when everything is set and ready.
    fn write_csv(&mut self) -> &mut Self {
        let mut writer = csv::Writer::from_writer(vec![]);
        for card in self.cards.iter() {
            writer.serialize(card).unwrap();
        }
        self.csv = String::from_utf8(writer.into_inner().expect("Error with csv serialize"))
            .expect("Error");
        self
    }
}

pub fn fix_name(name: &str) -> Option<String> {
    if name.is_card() {
        return None;
    }

    let (most_similar, score) = most_similar_card(name);

    match score >= 0.75 {
        true => Some(String::from(most_similar)),
        false => {
            // Try to prefix name with "The" - a lot of cards start with "The"
            let the = format!("The {name}");
            let (most_similar, score) = most_similar_card(&the);
            match score >= 0.75 {
                true => Some(String::from(most_similar)),
                false => None,
            }
        }
    }
}

fn most_similar_card(name: &str) -> (&str, f64) {
    let mut similarity_map = HashMap::<&str, f64>::new();
    for card in CARDS {
        let similarity = strsim::normalized_damerau_levenshtein(name, card);
        similarity_map.insert(card, similarity);
    }

    let most_similar = similarity_map
        .iter()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .unwrap();

    (most_similar.0, most_similar.1.to_owned())
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FixedCardName {
    pub old: String,
    pub fixed: String,
}

impl FixedCardName {
    pub fn new(old: &str, fixed: &str) -> FixedCardName {
        FixedCardName {
            old: String::from(old),
            fixed: String::from(fixed),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardNameAmount {
    pub name: String,
    #[serde(alias = "stackSize")]
    pub amount: u32,
}

impl CardNameAmount {
    pub const fn new(name: String, amount: u32) -> CardNameAmount {
        CardNameAmount { name, amount }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SampleData {
    Csv(String),
    CardNameAmountList(Vec<CardNameAmount>),
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn trim() {
        let s = "something,something\r\nname,stackSize\r\nA Dab of Ink,2\r\nA Familiar Call,1\r\nA Fate Worse than Death,2\r\nA Mother's Parting Gift,15\r\nA Sea of Blue,22\r\nA Stone Perfected,2\r\nAbandoned Wealth,4\r\nAccumitisation,14\r\nAlluring Bounty,3\r\nAlone in the Darkness,30\r\nAnarchy's Price,5\r\nArrogance of the Vaal,5\r\nAssassin's Favour,44\r\nAstral Projection,7\r\nAtziri's Arsenal,6\r\nAudacity,3\r\nAzure Rage,14\r\nAzyran's Reward,4\r\nBaited Expectations,4\r\nBijoux,2\r\nBlind Venture,11\r\nBoon of Justice,20\r\nBoon of the First Ones,3\r\nBoundless Realms,23\r\nBroken Truce,15\r\nBrotherhood in Exile,1\r\n\"Brush, Paint and Palette\",6\r\nBuried Treasure,7\r\nCall to the First Ones,13\r\nCameria's Cut,3\r\nCartographer's Delight,20\r\nChaotic Disposition,13\r\nChasing Risk,6\r\nCheckmate,5\r\nCostly Curio,3\r\nCouncil of Cats,5\r\nCoveted Possession,7\r\nCursed Words,12\r\nDark Dreams,3\r\nDark Temptation,23\r\nDeadly Joy,1\r\nDeath,5\r\nDeathly Designs,4\r\nDementophobia,1\r\nDemigod's Wager,5\r\nDesperate Crusade,2\r\nDestined to Crumble,80\r\nDialla's Subjugation,12\r\nDisdain,1\r\nDivine Justice,3\r\nDoedre's Madness,44\r\nDoryani's Epiphany,1\r\nDying Anguish,16\r\nDying Light,1\r\nEarth Drinker,8\r\nEchoes of Love,2\r\nEmperor of Purity,14\r\nEmperor's Luck,79\r\nEndless Night,3\r\nForbidden Power,16\r\nFrom Bone to Ash,1\r\nFurther Invention,1\r\nGemcutter's Mercy,2\r\nGemcutter's Promise,21\r\nGift of Asenath,3\r\nGift of the Gemling Queen,10\r\nGlimmer of Hope,34\r\nGrave Knowledge,13\r\nGuardian's Challenge,13\r\nHarmony of Souls ,1\r\nHer Mask,40\r\nHeterochromia,8\r\nHome,1\r\nHope,8\r\nHubris,24\r\nHumility,27\r\nHunter's Resolve,25\r\nHunter's Reward,4\r\nImmortal Resolve,5\r\nImperfect Memories,1\r\nImperial Legacy,36\r\nJack in the Box,11\r\nJudging Voices,2\r\nJustified Ambition,6\r\nLachrymal Necrosis,2\r\nLantador's Lost Love,50\r\nLast Hope,29\r\nLeft to Fate,9\r\nLight and Truth,4\r\nLingering Remnants,12\r\nLost Worlds,30\r\nLove Through Ice,1\r\nLoyalty,96\r\nLucky Connections,25\r\nLucky Deck,2\r\nLuminous Trove,1\r\nLysah's Respite,18\r\nMawr Blaidd,2\r\nMerciless Armament,2\r\nMight is Right,16\r\nMisery in Darkness,3\r\nMitts,27\r\nMonochrome,4\r\nMore is Never Enough,4\r\nNo Traces,15\r\nParasitic Passengers,4\r\nPeaceful Moments,4\r\nPrejudice,20\r\nPride before the Fall,2\r\nPride of the First Ones,1\r\nPrometheus' Armoury,2\r\nProsperity,31\r\nRain of Chaos,188\r\nRain Tempter,34\r\nRats,48\r\nRebirth,2\r\nRebirth and Renewal,3\r\nReckless Ambition,4\r\nRemembrance,1\r\nSambodhi's Vow,31\r\nSambodhi's Wisdom,11\r\nScholar of the Seas,9\r\nSeven Years Bad Luck,1\r\nShard of Fate,27\r\nSilence and Frost,4\r\nSociety's Remorse,11\r\nSomething Dark,4\r\nStruck by 
Lightning,31\r\nSuccor of the Sinless,1\r\nTerrible Secret of Space,3\r\nThe Academic,2\r\nThe Admirer,7\r\nThe Adventuring Spirit,34\r\nThe Aesthet,20\r\nThe Apothecary,1\r\nThe Archmage's Right Hand,6\r\nThe Arena Champion,36\r\nThe Army of Blood,34\r\nThe Artist,1\r\nThe Aspirant,1\r\nThe Avenger,2\r\nThe Awakened,1\r\nThe Bargain,2\r\nThe Battle Born,23\r\nThe Bear Woman,7\r\nThe Beast,6\r\nThe Betrayal,15\r\nThe Bitter Blossom,1\r\nThe Blazing Fire,33\r\nThe Blessing of Moosh,9\r\nThe Body,15\r\nThe Brawny Battlemage,3\r\nThe Breach,2\r\nThe Brittle Emperor,2\r\nThe Cache,31\r\nThe Cacophany,1\r\nThe Calling,13\r\nThe Card Sharp,7\r\nThe Carrion Crow,61\r\nThe Cartographer,24\r\nThe Cataclysm,10\r\nThe Catalyst,56\r\nThe Celestial Justicar,7\r\nThe Celestial Stone,2\r\nThe Chains That Bind,38\r\nThe Chosen,2\r\nThe Coming Storm,11\r\nThe Conduit,6\r\nThe Craving,1\r\nThe Cursed King,4\r\nThe Damned,2\r\nThe Dapper Prodify,15\r\nThe Dark Mage,2\r\nThe Darkest Dream,4\r\nThe Deal,7\r\nThe Deceiver,13\r\nThe Deep Ones,2\r\nThe Demoness,30\r\nThe Destination,1\r\nThe Doppelganger,52\r\nThe Cragon,23\r\nThe Dreamer,4\r\nThe Dreamland,17\r\nThe Drunken Aristocrat,17\r\nThe Dungeon Master,4\r\nThe Easy Stroll,9\r\nThe Eldritch Decay,2\r\nThe 
Encroaching Darkness,5\r\nThe Endless Darkness,1\r\nThe Endurance,19\r\nThe Enforcer,1\r\nThe Enlightened,1\r\nThe Enthusiasts,2\r\nThe Escape,1\r\nThe Ethereal,3\r\nThe Explorer,28\r\nThe Eye of Terror,1\r\nThe Eye of the Dragon,10\r\nThe Fathomless Depths,6\r\nThe Feast,11\r\nThe Fletcher,11\r\nThe Flora's Gift,50\r\nThe Fool,21\r\nThe Forgotten Treasure,1\r\nThe Formless Sea,13\r\nThe Forsaken,15\r\nThe Forward Gaze,13\r\nThe Fox,19\r\nThe Fox in the Brambles,4\r\nThe Gambler,19\r\nThe Garish Power,17\r\nThe Gemcutter,54\r\nThe Gentleman,4\r\nThe Gladiator,12\r\nThe Golden Era,7\r\nThe Harvester,44\r\nThe Hermit,47\r\nThe Heroic Shot,10\r\nThe Hoarder,14\r\nHook,2\r\nThe Hunger,3\r\nThe Immortal,1\r\nThe Incantation,6\r\nThe Innocent,11\r\nThe Innoculated,20\r\nThe Insatiable,10\r\nThe Inventor,24\r\nThe Jester,3\r\nThe Jeweller's Boon,17\r\nThe Journalist,40\r\nThe Journey,3\r\nThe King's Blade,83\r\nThe King's Heart,2\r\nThe Landing,7\r\nThe Last One Standing,4\r\nThe Last Supper,10\r\nThe Leviathan,1\r\nThe Lich,16\r\nThe Life Thief,1\r\nThe Lion,20\r\nThe Long Watch,9\r\nThe Lord in Black,4\r\nThe Lord of Celebration,3\r\nThe Lover,96\r\nTHe Lunaris Priestess,31\r\nThe Magma Crab,3\r\nThe Master,6\r\nThe Master Artisan,23\r\nThe Mercenary,7\r\nThe Messenger,7\r\nThe Metalsmith's Gift,75\r\nTHe Mind's Eye,2\r\nThe Mountain,17\r\nThe Nurse,4\r\nThe Oath,4\r\nThe Obscured,4\r\nThe Offering,4\r\nThe Offpring,4\r\nThe One with All,15\r\nThe Opulent,37\r\nThe Pack Leader,13\r\nThe Pact,4\r\nThe Patient,9\r\nThe Penitent,16\r\nThe Poet,6\r\nThe Polymath,4\r\nThe Porcupine,13\r\nThe Price of Projection,8\r\nThe Primordial,8\r\nThe Prince of Darkness,2\r\nThe Professor,4\r\nThe Puzzle,22\r\nThe Rabbit's Foot,1\r\nThe Rabid Rhoa,16\r\nThe Realm,2\r\nThe Risk,13\r\nThe Rite of Elements,11\r\nThe Road to Power,1\r\nTHe Ruthless Ceinture,16\r\nThe Sacrifice,1\r\nThe Saint's Treasure,7\r\nThe Scarred Meadow,46\r\nThe Scavenger,11\r\nThe Scholar,88\r\nThe Scout,2\r\nThe Seeker,2\r\nThe Sephirot,2\r\nThe Shepherd's Sandals,6\r\nThe Shortcut,1\r\nThe Side Quest,5\r\nThe Sigil,26\r\nThe Siren,6\r\nThe Skeleton,23\r\nThe Soul,3\r\nThe Spark and the Flame,3\r\nThe Spoiled Prince,6\r\nThe Standoff,24\r\nThe Stormcaller,22\r\nThe Strategist,2\r\nThe Summoner,18\r\nThe Sun,32\r\nThe Surgeon,22\r\nThe Surveyor,18\r\nThe Survivalist,24\r\nThe Sustenance,1\r\nThe Sword King's Salute,33\r\nThe Thaumaturgist,1\r\nThe Throne,4\r\nThe Tinkerer's Table,6\r\nThe Tireless Extractor,28\r\nThe Tower,21\r\nThe Traitor,9\r\nThe Trial,21\r\nThe Twilight Moon,7\r\nThe Twins,16\r\nThe Tyrant,5\r\nThe Undaunted,3\r\nThe Undisputed,1\r\nThe Unexpected Prize,4\r\nThe Union,18\r\nThe Valkyrie,14\r\nThe Visionary,30\r\nThe Void,27\r\nThe Warden,31\r\nThe Warlord,2\r\nThe Watcher,12\r\nThe Web,11\r\nThe White Knight,1\r\nTHe Whiteout,1\r\nThe Wilted Rose,12\r\nThe Wind,5\r\nThe Witch,31\r\nThe Wolf,14\r\nThe Wolf's Legacy,6\r\nThe Wolf's Shadow,18\r\nTHe Wolven King's Bite,1\r\nThe Wolverine,7\r\nThe World Eater,1\r\nThe Wrath,23\r\nThe Wretched,17\r\nThirst for Knowledge,27\r\nThree Faces in the Dark,54\r\nThree Voices,37\r\nThunderous Skies,36\r\nTime Lost Relic,27\r\nTranquility,19\r\nTreasure Hunter,8\r\nTriskaidekophobia,7\r\nTurn the other Cheek,38\r\nUnchained,2\r\nUnderground Forest,11\r\nVanity,13\r\nVinia's Token,35\r\nViolatile Power,15\r\nWinter's Embrace,3";
        let trimmed = super::DivinationCardsSample::remove_lines_before_headers(s).unwrap();

        assert_eq!(trimmed.lines().next().unwrap(), "name,stackSize");
    }

    #[test]
    fn merge() {
        use std::fs::read_to_string;

        let csv1 = read_to_string("example-1.csv").unwrap();
        let csv2 = read_to_string("example-2.csv").unwrap();
        let csv3 = read_to_string("example-3.csv").unwrap();

        let s1 = DivinationCardsSample::create(SampleData::Csv(csv1), None).unwrap();
        let s2 = DivinationCardsSample::create(SampleData::Csv(csv2), None).unwrap();
        let s3 = DivinationCardsSample::create(SampleData::Csv(csv3), None).unwrap();

        let s = DivinationCardsSample::merge(None, &[s1, s2, s3]);
        let rain_of_chaos = s
            .cards
            .iter()
            .find(|card| card.name == "Rain of Chaos")
            .unwrap();

        assert_eq!(rain_of_chaos.amount, 1779);
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NotNullishSample {
    pub cards: Vec<DivinationCardRecord>,
    pub not_cards: Vec<String>,
    pub fixed_names: Vec<FixedCardName>,
    pub csv: String,
}

impl From<DivinationCardsSample> for NotNullishSample {
    fn from(value: DivinationCardsSample) -> Self {
        let cards = value.cards.into_not_nullish();

        let mut writer = csv::Writer::from_writer(vec![]);
        for card in cards.iter() {
            writer.serialize(card).unwrap()
        }

        let csv = String::from_utf8(writer.into_inner().expect("Error with csv serialize"))
            .expect("Error");

        NotNullishSample {
            cards,
            not_cards: value.not_cards,
            fixed_names: value.fixed_names,
            csv,
        }
    }
}
