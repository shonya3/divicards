use crate::league::LeagueReleaseInfo;
use crate::league::ReleaseVersion;
#[cfg(feature = "fs_cache_fetcher")]
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const POE_CDN_CARDS: &str = "https://web.poecdn.com/image/divination-card/";

#[cfg(feature = "fs_cache_fetcher")]
/// Represents the Google Sheets ranges for a specific league's divination card data.
///
/// This is used to fetch card names and their corresponding weights from a spreadsheet
/// for different Path of Exile league versions.
pub struct LeagueRanges {
    /// The league version, e.g., "3.26".
    pub version: ReleaseVersion,
    /// The sheet range for card names, e.g., "3.26!H3:H".
    pub names_range: String,
    /// The sheet range for card weights, e.g., "3.26!S3:S".
    pub weights_range: String,
}

#[cfg(feature = "fs_cache_fetcher")]
pub static LEAGUE_RANGES: Lazy<[LeagueRanges; 4]> = Lazy::new(|| {
    [
        LeagueRanges {
            version: ReleaseVersion::new("3.26".to_string()),
            names_range: "3.26!H3:H".to_string(),
            weights_range: "3.26!S3:S".to_string(),
        },
        LeagueRanges {
            version: ReleaseVersion::new("3.25".to_string()),
            names_range: "3.25!F3:F".to_string(),
            weights_range: "3.25!Q3:Q".to_string(),
        },
        LeagueRanges {
            version: ReleaseVersion::new("3.24".to_string()),
            names_range: "3.24!D3:D".to_string(),
            weights_range: "3.24!O3:O".to_string(),
        },
        LeagueRanges {
            version: ReleaseVersion::new("3.23".to_string()),
            names_range: "3.23!D3:D".to_string(),
            weights_range: "3.23!P3:P".to_string(),
        },
    ]
});

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub slug: String,
    pub name: String,
    pub min_level: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_level: Option<u32>,
    pub weights: HashMap<String, f32>,
    pub price: Option<f32>,
    #[serde(alias = "release version")]
    pub league: Option<LeagueReleaseInfo>,
    pub disabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardsData(pub HashMap<String, Card>);
impl CardsData {
    pub fn card(&self, s: &str) -> &Card {
        let Some(card) = self.0.get(s) else {
            panic!("Card not exists {s}");
        };
        card
    }
}

#[cfg(feature = "fs_cache_fetcher")]
pub mod fetch {
    use std::{collections::HashSet, fmt::Display, num::ParseIntError};

    use super::LeagueRanges;
    use crate::{
        cards::{Card, CardsData, LEAGUE_RANGES},
        consts::{WEIGHT_SPREADSHEET_ID, WIKI_API_URL},
        league::{self, fetch::Error as LeagueError, ReleaseVersion},
        HTTP_CLIENT,
    };
    use divi::{
        prices::Prices,
        sample::{Input, Sample},
        IsCard,
    };
    use futures::future::try_join_all;
    use googlesheets::sheet::Credential;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct WikiCard {
        name: String,
        #[serde(alias = "drop level")]
        min_level: Option<u32>,
        #[serde(alias = "drop level maximum")]
        max_level: Option<u32>,
        #[serde(alias = "release version")]
        release_version: Option<ReleaseVersion>,
    }

    #[derive(Debug)]
    pub enum Error {
        Ninja(divi::error::NinjaError),
        GoogleSheets(googlesheets::error::Error),
        Divi(divi::error::Error),
        Wiki(WikiError),
        League(LeagueError),
    }

    impl Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Error::Ninja(e) => write!(f, "Failed to fetch prices from poe.ninja: {e}"),
                Error::GoogleSheets(e) => write!(f, "Failed to read from Google Sheets: {e}"),
                Error::Divi(e) => write!(f, "Failed to process divination card sample data: {e}"),
                Error::Wiki(e) => write!(f, "Failed to load wiki card data: {e}"),
                Error::League(e) => write!(f, "Failed to load league info: {e}"),
            }
        }
    }

    impl From<divi::error::Error> for Error {
        fn from(err: divi::error::Error) -> Self {
            Error::Divi(err)
        }
    }
    impl From<googlesheets::error::Error> for Error {
        fn from(err: googlesheets::error::Error) -> Self {
            Error::GoogleSheets(err)
        }
    }
    impl From<WikiError> for Error {
        fn from(err: WikiError) -> Self {
            Error::Wiki(err)
        }
    }
    impl From<LeagueError> for Error {
        fn from(err: LeagueError) -> Self {
            Error::League(err)
        }
    }

    /// Loads Total amounts from a given league, constructs Sample from them https://docs.google.com/spreadsheets/d/1PmGES_e1on6K7O5ghHuoorEjruAVb7dQ5m7PGrW7t80/edit#gid=898101079
    async fn load_sample_from_ranges(
        api_key: String,
        league_ranges: &LeagueRanges,
        prices: Option<Prices>,
    ) -> Result<Sample, Error> {
        let batch_read = googlesheets::read_batch(
            WEIGHT_SPREADSHEET_ID,
            &[&league_ranges.names_range, &league_ranges.weights_range],
            Credential::ApiKey(api_key),
        )
        .await?;
        let data = Input::try_from(batch_read)?;
        let sample = Sample::create(data, prices)?;
        Ok(sample)
    }

    pub async fn fetch() -> Result<CardsData, Error> {
        println!("Fetching cards");
        dotenv::dotenv().ok();
        let key = std::env::var("GOOGLE_API_KEY").expect("GOOGLE_API_KEY is expected.");

        let (prices_res, other_samples_res, wikicards_res, league_info_vec_res) = tokio::join!(
            Prices::fetch(&divi::TradeLeague::Standard),
            try_join_all(
                LEAGUE_RANGES
                    .iter()
                    .skip(1) // Skip latest, it's fetched separately
                    .map(|league_range| load_sample_from_ranges(key.clone(), league_range, None))
            ),
            load_wiki_cards(),
            league::fetch::fetch()
        );

        let prices = prices_res.map_err(Error::Ninja)?;
        let mut other_samples = other_samples_res?;
        let mut wikicards = wikicards_res?;
        let league_info_vec = league_info_vec_res?;

        // Fetch latest sample separately with prices
        let latest_sample =
            load_sample_from_ranges(key, &LEAGUE_RANGES[0], Some(prices.clone())).await?;

        let mut samples = vec![latest_sample];
        samples.append(&mut other_samples);

        let versions: Vec<String> = LEAGUE_RANGES
            .iter()
            .map(|lr| lr.version.to_string())
            .collect();

        let all_card_names: HashSet<String> = samples
            .iter()
            .flat_map(|sample| &sample.cards)
            .map(|card| card.name.clone())
            .collect();

        let mut cards: Vec<Card> = all_card_names
            .into_iter()
            .filter_map(|card_name| {
                let mut weights = HashMap::new();
                for (i, sample) in samples.iter().enumerate() {
                    if let Some(weight) = sample.cards.get(&card_name).and_then(|c| c.weight) {
                        weights.insert(versions[i].clone(), weight);
                    }
                }

                // A card might not exist in all samples.
                // We need to get the card record from *any* sample to get price, is_legacy, etc.
                // We use filter_map to discard if not found in any sample.
                let divi_card = samples
                    .iter()
                    .find_map(|sample| sample.cards.get(&card_name))?;

                let (min_level, max_level, release_version) = wikicards
                    .remove(&card_name)
                    .map(|w| (w.min_level, w.max_level, w.release_version))
                    .unwrap_or_default();

                Some(Card {
                    slug: slug::slugify(&card_name),
                    name: card_name,
                    min_level,
                    max_level,
                    weights,
                    price: divi_card.price,
                    league: release_version.and_then(|version| {
                        league_info_vec
                            .iter()
                            .find(|info| info.version.is_equal(&version))
                            .cloned()
                    }),
                    disabled: divi_card.is_legacy_card(),
                })
            })
            .collect();

        let big_value = 1_000_000.0;
        cards.sort_by(|a, b| {
            let latest_version = &versions[0];
            let a_weight = a.weights.get(latest_version).copied().unwrap_or(big_value);
            let b_weight = b.weights.get(latest_version).copied().unwrap_or(big_value);
            a_weight.partial_cmp(&b_weight).unwrap()
        });

        let cards_hashmap = cards.into_iter().map(|c| (c.name.clone(), c)).collect();
        Ok(CardsData(cards_hashmap))
    }

    #[derive(Debug)]
    pub enum WikiError {
        Http(reqwest::Error),
        ParseCardLevel {
            given_str: String,
            card: String,
            field_name: String,
            err: ParseIntError,
        },
    }

    impl Display for WikiError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                WikiError::Http(error) => write!(f, "HTTP error while fetching wiki cards data: {error}"),
                WikiError::ParseCardLevel {
                    given_str,
                    card,
                    field_name,
                    err,
                } => write!(
                    f,
                    "Failed to parse {field_name} for card '{card}'. Value was: '{given_str}'. Error: {err}"
                ),
            }
        }
    }

    async fn load_wiki_cards() -> Result<HashMap<String, WikiCard>, WikiError> {
        #[derive(Serialize, Deserialize, Debug, Clone)]
        struct WikiCardsResponse {
            cargoquery: Vec<WikiCardWrapper>,
        }

        #[derive(Serialize, Deserialize, Debug, Clone)]
        struct WikiCardRaw {
            name: String,
            #[serde(alias = "drop level")]
            min_level: Option<String>,
            #[serde(alias = "drop level maximum")]
            max_level: Option<String>,
            #[serde(alias = "release version")]
            release_version: Option<ReleaseVersion>,
        }

        #[derive(Serialize, Deserialize, Debug, Clone)]
        struct WikiCardWrapper {
            title: WikiCardRaw,
        }

        let params = [
            ("action", "cargoquery"),
            ("format", "json"),
            ("smaxage", "1"),
            ("maxage", "1"),
            ("tables", "items"),
            ("limit", "500"),
            ("fields", "items.release_version,items.name,items.drop_level,items.drop_level_maximum,items.drop_areas,items.drop_monsters"),
            ("where", "items.class_id='DivinationCard'"),
        ];

        let response = HTTP_CLIENT
            .get(WIKI_API_URL)
            .query(&params)
            .send()
            .await
            .map_err(WikiError::Http)?
            .json::<WikiCardsResponse>()
            .await
            .map_err(WikiError::Http)?;

        let parse_level = |level_str: Option<String>,
                           card_name: &str,
                           field_name: &str|
         -> Result<Option<u32>, WikiError> {
            level_str
                .map(|s| {
                    s.parse::<u32>().map_err(|err| WikiError::ParseCardLevel {
                        given_str: s,
                        card: card_name.to_string(),
                        field_name: field_name.to_string(),
                        err,
                    })
                })
                .transpose()
        };

        response
            .cargoquery
            .into_iter()
            .map(|WikiCardWrapper { title: raw }| {
                let name = raw.name.clone();
                Ok((
                    name,
                    WikiCard {
                        min_level: parse_level(raw.min_level, &raw.name, "min_level")?,
                        max_level: parse_level(raw.max_level, &raw.name, "max_level")?,
                        release_version: raw.release_version,
                        name: raw.name,
                    },
                ))
            })
            .collect::<Result<HashMap<String, WikiCard>, _>>()
    }
}
