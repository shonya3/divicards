use crate::{
    cards::{Cards, FixedCardName, GetRecordMut},
    consts::RAIN_OF_CHAOS_WEIGHT,
    error::Error,
    prices::Prices,
};
use csv::{ReaderBuilder, Trim};
use googlesheets::sheet::ReadBatchResponse;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{fmt::Display, iter::zip};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DivinationCardsSample {
    pub cards: Cards,
    pub not_cards: Vec<String>,
    pub fixed_names: Vec<FixedCardName>,
}

impl DivinationCardsSample {
    pub fn new(
        cards: Cards,
        not_cards: Vec<String>,
        fixed_names: Vec<FixedCardName>,
    ) -> DivinationCardsSample {
        DivinationCardsSample {
            cards,
            not_cards,
            fixed_names,
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
        let mut sample = Self::from_prices(prices);
        let name_amount_pairs = match source {
            SampleData::Csv(csv_data) => parse_csv(&csv_data)?,
            SampleData::CardNameAmountList(vec) => vec,
        };

        for CardNameAmount { name, amount } in name_amount_pairs {
            match sample.cards.get_record_mut(&name) {
                GetRecordMut::Valid(record) => record.add_amount(amount),
                GetRecordMut::TypoFixed(record, fixed) => {
                    record.add_amount(amount);
                    sample.fixed_names.push(fixed);
                }
                GetRecordMut::NotACard => sample.not_cards.push(name),
            }
        }

        sample.write_weight();
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
            card.set_amount(amount);
        }

        merged.write_weight();
        merged
    }

    /// Consumes Prices structure to set prices for Cards
    fn from_prices(prices: Option<Prices>) -> Self {
        DivinationCardsSample {
            cards: Cards::from(prices.unwrap_or_default()),
            ..Default::default()
        }
    }

    /// Helper function for write_weight
    fn weight_multiplier(&self) -> f32 {
        let rain_of_chaos = self.cards.get_card("Rain of Chaos");
        RAIN_OF_CHAOS_WEIGHT / rain_of_chaos.amount as f32
    }

    /// (After parsing) Calculates special weight for each card and mutates it. Runs at the end of parsing.
    fn write_weight(&mut self) {
        let weight_multiplier = self.weight_multiplier();
        self.cards
            .iter_mut()
            .for_each(|card| card.set_weight(weight_multiplier));
    }

    pub fn into_serde_values(mut self, preferences: Option<TablePreferences>) -> Vec<Vec<Value>> {
        let preferences = preferences.unwrap_or_default();

        if preferences.cards_must_have_amount {
            self.cards.0.retain(|c| c.amount > 0);
        }

        self.cards
            .order_by(preferences.ordered_by, preferences.order);

        let columns = preserve_column_order(&preferences.columns);
        let mut values: Vec<Vec<Value>> = vec![];
        let headers: Vec<Value> = columns.iter().map(|c| json!(c)).collect();
        values.push(headers);

        for card in self.cards.iter() {
            if card.price.unwrap_or_default() < preferences.min_price {
                continue;
            }
            values.push(
                columns
                    .iter()
                    .map(|column| match column {
                        Column::Name => json!(&card.name),
                        Column::Amount => json!(card.amount),
                        Column::Weight => json!(card.weight),
                        Column::Price => json!(card.price),
                        Column::Sum => json!(card.sum),
                    })
                    .collect::<Vec<Value>>(),
            );
        }

        values
    }

    pub fn into_csv(self, preferences: Option<TablePreferences>) -> String {
        let values = self.into_serde_values(preferences);
        let mut writer = csv::Writer::from_writer(vec![]);
        for val in values {
            writer.serialize(val).unwrap();
        }

        String::from_utf8(writer.into_inner().unwrap()).unwrap()
    }
}

fn parse_csv(csv_data: &str) -> Result<Vec<CardNameAmount>, Error> {
    let data = remove_lines_before_headers(&csv_data)?;
    let mut rdr = ReaderBuilder::new()
        .trim(Trim::All)
        .from_reader(data.as_bytes());

    let mut vec = vec![];
    for result in rdr.deserialize::<CardNameAmount>() {
        let name_amount_pair = result?;
        vec.push(name_amount_pair);
    }

    Ok(vec)
}

#[derive(Debug)]
pub struct MissingHeadersError;
impl Display for MissingHeadersError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("File should contain headers: name, amount.")
    }
}

/// Parsing helper. Uses for CSV data
fn remove_lines_before_headers(s: &str) -> Result<String, MissingHeadersError> {
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
        None => Err(MissingHeadersError),
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TablePreferences {
    pub columns: Vec<Column>,
    pub ordered_by: Column,
    pub order: Order,
    pub cards_must_have_amount: bool,
    pub min_price: f32,
}

impl Default for TablePreferences {
    fn default() -> Self {
        Self {
            columns: vec![Column::Name, Column::Amount],
            ordered_by: Column::Amount,
            order: Order::Desc,
            cards_must_have_amount: false,
            min_price: 0.,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Order {
    Asc,
    Desc,
    Unordered,
}

impl Default for Order {
    fn default() -> Self {
        Order::Desc
    }
}

/// name > amount > weight > price > sum
fn preserve_column_order(columns: &[Column]) -> Vec<Column> {
    let mut vec: Vec<Column> = vec![];
    columns
        .iter()
        .find(|c| c == &&Column::Name)
        .and_then(|_| Some(vec.push(Column::Name)));

    columns
        .iter()
        .find(|c| c == &&Column::Amount)
        .and_then(|_| Some(vec.push(Column::Amount)));

    columns
        .iter()
        .find(|c| c == &&Column::Weight)
        .and_then(|_| Some(vec.push(Column::Weight)));

    columns
        .iter()
        .find(|c| c == &&Column::Price)
        .and_then(|_| Some(vec.push(Column::Price)));

    columns
        .iter()
        .find(|c| c == &&Column::Sum)
        .and_then(|_| Some(vec.push(Column::Sum)));

    vec
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Column {
    Name,
    Amount,
    Weight,
    Price,
    Sum,
}

impl Default for Column {
    fn default() -> Self {
        Column::Amount
    }
}

impl Display for Column {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Column::Name => write!(f, "name"),
            Column::Amount => write!(f, "amount"),
            Column::Weight => write!(f, "weight"),
            Column::Price => write!(f, "price"),
            Column::Sum => write!(f, "sum"),
        }
    }
}

#[cfg(test)]
mod tests {

    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn into_serde_values() {
        let csv = read_to_string("examples/example-2.csv").unwrap();
        let prices = Prices::default();
        let sample = DivinationCardsSample::create(SampleData::Csv(csv), Some(prices)).unwrap();
        let values = sample.into_serde_values(Some(TablePreferences {
            columns: vec![
                Column::Sum,
                Column::Weight,
                Column::Price,
                Column::Amount,
                Column::Name,
            ],
            ordered_by: Column::Name,
            order: Order::Desc,
            cards_must_have_amount: false,
            min_price: 0.,
        }));
        let _json = serde_json::to_string(&values).unwrap();
        // write("serde-values.json", &json).unwrap();
    }

    #[test]
    fn column_order() {
        let columns = preserve_column_order(&[
            Column::Amount,
            Column::Sum,
            Column::Weight,
            Column::Price,
            Column::Name,
        ]);
        assert_eq!(
            columns,
            [
                Column::Name,
                Column::Amount,
                Column::Weight,
                Column::Price,
                Column::Sum,
            ]
        );
    }

    #[test]
    fn into_serde_values_2() {
        let sample = DivinationCardsSample::create(
            SampleData::Csv(String::from("name,amount\rRain of Chaos,1\rThe Doctor,1")),
            None,
        )
        .unwrap();
        let values = sample.into_serde_values(Some(TablePreferences {
            cards_must_have_amount: true,
            ..Default::default()
        }));
        dbg!(&values);
        assert_eq!(values.len(), 3);
        let _json = serde_json::to_string(&values).unwrap();
        // write("values2.json", &json).unwrap();
    }

    #[test]
    fn trim() {
        let s = "something,something\r\nname,stackSize\r\nA Dab of Ink,2\r\nA Familiar Call,1\r\nA Fate Worse than Death,2\r\nA Mother's Parting Gift,15\r\nA Sea of Blue,22\r\nA Stone Perfected,2\r\nAbandoned Wealth,4\r\nAccumitisation,14\r\nAlluring Bounty,3\r\nAlone in the Darkness,30\r\nAnarchy's Price,5\r\nArrogance of the Vaal,5\r\nAssassin's Favour,44\r\nAstral Projection,7\r\nAtziri's Arsenal,6\r\nAudacity,3\r\nAzure Rage,14\r\nAzyran's Reward,4\r\nBaited Expectations,4\r\nBijoux,2\r\nBlind Venture,11\r\nBoon of Justice,20\r\nBoon of the First Ones,3\r\nBoundless Realms,23\r\nBroken Truce,15\r\nBrotherhood in Exile,1\r\n\"Brush, Paint and Palette\",6\r\nBuried Treasure,7\r\nCall to the First Ones,13\r\nCameria's Cut,3\r\nCartographer's Delight,20\r\nChaotic Disposition,13\r\nChasing Risk,6\r\nCheckmate,5\r\nCostly Curio,3\r\nCouncil of Cats,5\r\nCoveted Possession,7\r\nCursed Words,12\r\nDark Dreams,3\r\nDark Temptation,23\r\nDeadly Joy,1\r\nDeath,5\r\nDeathly Designs,4\r\nDementophobia,1\r\nDemigod's Wager,5\r\nDesperate Crusade,2\r\nDestined to Crumble,80\r\nDialla's Subjugation,12\r\nDisdain,1\r\nDivine Justice,3\r\nDoedre's Madness,44\r\nDoryani's Epiphany,1\r\nDying Anguish,16\r\nDying Light,1\r\nEarth Drinker,8\r\nEchoes of Love,2\r\nEmperor of Purity,14\r\nEmperor's Luck,79\r\nEndless Night,3\r\nForbidden Power,16\r\nFrom Bone to Ash,1\r\nFurther Invention,1\r\nGemcutter's Mercy,2\r\nGemcutter's Promise,21\r\nGift of Asenath,3\r\nGift of the Gemling Queen,10\r\nGlimmer of Hope,34\r\nGrave Knowledge,13\r\nGuardian's Challenge,13\r\nHarmony of Souls ,1\r\nHer Mask,40\r\nHeterochromia,8\r\nHome,1\r\nHope,8\r\nHubris,24\r\nHumility,27\r\nHunter's Resolve,25\r\nHunter's Reward,4\r\nImmortal Resolve,5\r\nImperfect Memories,1\r\nImperial Legacy,36\r\nJack in the Box,11\r\nJudging Voices,2\r\nJustified Ambition,6\r\nLachrymal Necrosis,2\r\nLantador's Lost Love,50\r\nLast Hope,29\r\nLeft to Fate,9\r\nLight and Truth,4\r\nLingering Remnants,12\r\nLost Worlds,30\r\nLove Through Ice,1\r\nLoyalty,96\r\nLucky Connections,25\r\nLucky Deck,2\r\nLuminous Trove,1\r\nLysah's Respite,18\r\nMawr Blaidd,2\r\nMerciless Armament,2\r\nMight is Right,16\r\nMisery in Darkness,3\r\nMitts,27\r\nMonochrome,4\r\nMore is Never Enough,4\r\nNo Traces,15\r\nParasitic Passengers,4\r\nPeaceful Moments,4\r\nPrejudice,20\r\nPride before the Fall,2\r\nPride of the First Ones,1\r\nPrometheus' Armoury,2\r\nProsperity,31\r\nRain of Chaos,188\r\nRain Tempter,34\r\nRats,48\r\nRebirth,2\r\nRebirth and Renewal,3\r\nReckless Ambition,4\r\nRemembrance,1\r\nSambodhi's Vow,31\r\nSambodhi's Wisdom,11\r\nScholar of the Seas,9\r\nSeven Years Bad Luck,1\r\nShard of Fate,27\r\nSilence and Frost,4\r\nSociety's Remorse,11\r\nSomething Dark,4\r\nStruck by 
Lightning,31\r\nSuccor of the Sinless,1\r\nTerrible Secret of Space,3\r\nThe Academic,2\r\nThe Admirer,7\r\nThe Adventuring Spirit,34\r\nThe Aesthet,20\r\nThe Apothecary,1\r\nThe Archmage's Right Hand,6\r\nThe Arena Champion,36\r\nThe Army of Blood,34\r\nThe Artist,1\r\nThe Aspirant,1\r\nThe Avenger,2\r\nThe Awakened,1\r\nThe Bargain,2\r\nThe Battle Born,23\r\nThe Bear Woman,7\r\nThe Beast,6\r\nThe Betrayal,15\r\nThe Bitter Blossom,1\r\nThe Blazing Fire,33\r\nThe Blessing of Moosh,9\r\nThe Body,15\r\nThe Brawny Battlemage,3\r\nThe Breach,2\r\nThe Brittle Emperor,2\r\nThe Cache,31\r\nThe Cacophany,1\r\nThe Calling,13\r\nThe Card Sharp,7\r\nThe Carrion Crow,61\r\nThe Cartographer,24\r\nThe Cataclysm,10\r\nThe Catalyst,56\r\nThe Celestial Justicar,7\r\nThe Celestial Stone,2\r\nThe Chains That Bind,38\r\nThe Chosen,2\r\nThe Coming Storm,11\r\nThe Conduit,6\r\nThe Craving,1\r\nThe Cursed King,4\r\nThe Damned,2\r\nThe Dapper Prodify,15\r\nThe Dark Mage,2\r\nThe Darkest Dream,4\r\nThe Deal,7\r\nThe Deceiver,13\r\nThe Deep Ones,2\r\nThe Demoness,30\r\nThe Destination,1\r\nThe Doppelganger,52\r\nThe Cragon,23\r\nThe Dreamer,4\r\nThe Dreamland,17\r\nThe Drunken Aristocrat,17\r\nThe Dungeon Master,4\r\nThe Easy Stroll,9\r\nThe Eldritch Decay,2\r\nThe 
Encroaching Darkness,5\r\nThe Endless Darkness,1\r\nThe Endurance,19\r\nThe Enforcer,1\r\nThe Enlightened,1\r\nThe Enthusiasts,2\r\nThe Escape,1\r\nThe Ethereal,3\r\nThe Explorer,28\r\nThe Eye of Terror,1\r\nThe Eye of the Dragon,10\r\nThe Fathomless Depths,6\r\nThe Feast,11\r\nThe Fletcher,11\r\nThe Flora's Gift,50\r\nThe Fool,21\r\nThe Forgotten Treasure,1\r\nThe Formless Sea,13\r\nThe Forsaken,15\r\nThe Forward Gaze,13\r\nThe Fox,19\r\nThe Fox in the Brambles,4\r\nThe Gambler,19\r\nThe Garish Power,17\r\nThe Gemcutter,54\r\nThe Gentleman,4\r\nThe Gladiator,12\r\nThe Golden Era,7\r\nThe Harvester,44\r\nThe Hermit,47\r\nThe Heroic Shot,10\r\nThe Hoarder,14\r\nHook,2\r\nThe Hunger,3\r\nThe Immortal,1\r\nThe Incantation,6\r\nThe Innocent,11\r\nThe Innoculated,20\r\nThe Insatiable,10\r\nThe Inventor,24\r\nThe Jester,3\r\nThe Jeweller's Boon,17\r\nThe Journalist,40\r\nThe Journey,3\r\nThe King's Blade,83\r\nThe King's Heart,2\r\nThe Landing,7\r\nThe Last One Standing,4\r\nThe Last Supper,10\r\nThe Leviathan,1\r\nThe Lich,16\r\nThe Life Thief,1\r\nThe Lion,20\r\nThe Long Watch,9\r\nThe Lord in Black,4\r\nThe Lord of Celebration,3\r\nThe Lover,96\r\nTHe Lunaris Priestess,31\r\nThe Magma Crab,3\r\nThe Master,6\r\nThe Master Artisan,23\r\nThe Mercenary,7\r\nThe Messenger,7\r\nThe Metalsmith's Gift,75\r\nTHe Mind's Eye,2\r\nThe Mountain,17\r\nThe Nurse,4\r\nThe Oath,4\r\nThe Obscured,4\r\nThe Offering,4\r\nThe Offpring,4\r\nThe One with All,15\r\nThe Opulent,37\r\nThe Pack Leader,13\r\nThe Pact,4\r\nThe Patient,9\r\nThe Penitent,16\r\nThe Poet,6\r\nThe Polymath,4\r\nThe Porcupine,13\r\nThe Price of Projection,8\r\nThe Primordial,8\r\nThe Prince of Darkness,2\r\nThe Professor,4\r\nThe Puzzle,22\r\nThe Rabbit's Foot,1\r\nThe Rabid Rhoa,16\r\nThe Realm,2\r\nThe Risk,13\r\nThe Rite of Elements,11\r\nThe Road to Power,1\r\nTHe Ruthless Ceinture,16\r\nThe Sacrifice,1\r\nThe Saint's Treasure,7\r\nThe Scarred Meadow,46\r\nThe Scavenger,11\r\nThe Scholar,88\r\nThe Scout,2\r\nThe Seeker,2\r\nThe Sephirot,2\r\nThe Shepherd's Sandals,6\r\nThe Shortcut,1\r\nThe Side Quest,5\r\nThe Sigil,26\r\nThe Siren,6\r\nThe Skeleton,23\r\nThe Soul,3\r\nThe Spark and the Flame,3\r\nThe Spoiled Prince,6\r\nThe Standoff,24\r\nThe Stormcaller,22\r\nThe Strategist,2\r\nThe Summoner,18\r\nThe Sun,32\r\nThe Surgeon,22\r\nThe Surveyor,18\r\nThe Survivalist,24\r\nThe Sustenance,1\r\nThe Sword King's Salute,33\r\nThe Thaumaturgist,1\r\nThe Throne,4\r\nThe Tinkerer's Table,6\r\nThe Tireless Extractor,28\r\nThe Tower,21\r\nThe Traitor,9\r\nThe Trial,21\r\nThe Twilight Moon,7\r\nThe Twins,16\r\nThe Tyrant,5\r\nThe Undaunted,3\r\nThe Undisputed,1\r\nThe Unexpected Prize,4\r\nThe Union,18\r\nThe Valkyrie,14\r\nThe Visionary,30\r\nThe Void,27\r\nThe Warden,31\r\nThe Warlord,2\r\nThe Watcher,12\r\nThe Web,11\r\nThe White Knight,1\r\nTHe Whiteout,1\r\nThe Wilted Rose,12\r\nThe Wind,5\r\nThe Witch,31\r\nThe Wolf,14\r\nThe Wolf's Legacy,6\r\nThe Wolf's Shadow,18\r\nTHe Wolven King's Bite,1\r\nThe Wolverine,7\r\nThe World Eater,1\r\nThe Wrath,23\r\nThe Wretched,17\r\nThirst for Knowledge,27\r\nThree Faces in the Dark,54\r\nThree Voices,37\r\nThunderous Skies,36\r\nTime Lost Relic,27\r\nTranquility,19\r\nTreasure Hunter,8\r\nTriskaidekophobia,7\r\nTurn the other Cheek,38\r\nUnchained,2\r\nUnderground Forest,11\r\nVanity,13\r\nVinia's Token,35\r\nViolatile Power,15\r\nWinter's Embrace,3";
        let trimmed = super::remove_lines_before_headers(s).unwrap();

        assert_eq!(trimmed.lines().next().unwrap(), "name,stackSize");
    }

    #[test]
    fn merge() {
        use std::fs::read_to_string;

        let csv1 = read_to_string("examples/example-1.csv").unwrap();
        let csv2 = read_to_string("examples/example-2.csv").unwrap();
        let csv3 = read_to_string("examples/example-3.csv").unwrap();

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

impl TryFrom<ReadBatchResponse> for SampleData {
    type Error = crate::error::Error;

    fn try_from(response: ReadBatchResponse) -> Result<Self, Self::Error> {
        let names = response.value_ranges[0].values.clone();
        let amounts = response.value_ranges[1].values.clone();

        let v = zip(names, amounts)
            .into_iter()
            .map(|(name, amount)| {
                let name: String = serde_json::from_str(&name[0].to_string())?;
                let amount: u32 =
                    serde_json::from_str::<String>(&amount[0].to_string())?.parse::<u32>()?;
                Ok(CardNameAmount { name, amount })
            })
            .collect::<Result<Vec<CardNameAmount>, Error>>()?;
        Ok(SampleData::CardNameAmountList(v))
    }
}
