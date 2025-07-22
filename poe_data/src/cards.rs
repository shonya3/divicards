use crate::league::LeagueReleaseInfo;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const POE_CDN_CARDS: &str = "https://web.poecdn.com/image/divination-card/";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub slug: String,
    pub name: String,
    pub min_level: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_level: Option<u32>,
    pub weight: Option<f32>,
    pub price: Option<f32>,
    #[serde(alias = "release version")]
    pub league: Option<LeagueReleaseInfo>,
    pub pre_rework_weight: Option<f32>,
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
    use std::collections::HashMap;
    use std::{fmt::Display, num::ParseIntError};

    use super::CardsData;
    use crate::{
        cards::Card,
        consts::{
            SHEET_RANGES_OF_TOTAL_CARDS_FROM_LATEST_LEAGUE,
            SHEET_RANGES_OF_TOTAL_CARDS_FROM_PRE_REWORK_WEIGHT_LEAGUE, WEIGHT_SPREADSHEET_ID,
            WIKI_API_URL,
        },
        league::{self, fetch::Error as LeagueError, ReleaseVersion},
        HTTP_CLIENT,
    };
    use divi::{
        prices::Prices,
        sample::{Input, Sample},
        IsCard,
    };
    use googlesheets::sheet::Credential;
    use serde::{Deserialize, Serialize};

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

    pub async fn fetch() -> Result<CardsData, Error> {
        println!("Fetching cards");
        dotenv::dotenv().ok();
        let key = std::env::var("GOOGLE_API_KEY").expect("GOOGLE_API_KEY is expected.");

        let (prices_res, pre_rework_weight_sample_res, wikicards_res, league_info_vec_res) = tokio::join!(
            Prices::fetch(&divi::TradeLeague::Standard),
            load_sample_with_pre_rework_weight(key.clone()),
            load_wiki_cards(),
            league::fetch::fetch()
        );
        let prices = prices_res.map_err(Error::Ninja)?;
        let pre_rework_weight_sample = pre_rework_weight_sample_res?;
        let league_info = league_info_vec_res?;
        let sample = load_league_amounts_sample(key.clone(), Some(prices)).await?;
        let mut wikicards = wikicards_res?;

        let mut cards: Vec<Card> = sample
            .cards
            .into_iter()
            .map(|card| {
                let (min_level, max_level, release_version) = wikicards
                    .remove(&card.name)
                    .map(|w| (w.min_level, w.max_level, w.release_version))
                    .unwrap_or_default();

                Card {
                    slug: slug::slugify(&card.name),
                    min_level,
                    max_level,
                    weight: card.weight,
                    pre_rework_weight: pre_rework_weight_sample
                        .cards
                        .get(&card.name)
                        .and_then(|card| card.weight),
                    price: card.price,
                    league: release_version
                        .and_then(|version| {
                            league_info
                                .iter()
                                .find(|info| info.version.is_equal(&version))
                        })
                        .cloned(),
                    disabled: card.is_legacy_card(),
                    name: card.name,
                }
            })
            .collect();

        let big_value = 1_000_000.0;
        cards.sort_by(|a, b| {
            let a_weight = a.weight.unwrap_or(big_value);
            let b_weight = b.weight.unwrap_or(big_value);
            a_weight.partial_cmp(&b_weight).unwrap()
        });

        let cards_hashmap = cards.into_iter().map(|c| (c.name.clone(), c)).collect();
        Ok(CardsData(cards_hashmap))
    }

    /// Loads Total amounts from latest league, constructs Sample from them https://docs.google.com/spreadsheets/d/1PmGES_e1on6K7O5ghHuoorEjruAVb7dQ5m7PGrW7t80/edit#gid=898101079
    async fn load_league_amounts_sample(
        api_key: String,
        prices: Option<Prices>,
    ) -> Result<Sample, Error> {
        let batch_read = googlesheets::read_batch(
            WEIGHT_SPREADSHEET_ID,
            SHEET_RANGES_OF_TOTAL_CARDS_FROM_LATEST_LEAGUE,
            Credential::ApiKey(api_key),
        )
        .await?;
        let data = Input::try_from(batch_read)?;
        let sample = Sample::create(data, prices)?;
        Ok(sample)
    }

    async fn load_sample_with_pre_rework_weight(api_key: String) -> Result<Sample, Error> {
        let batch_read = googlesheets::read_batch(
            WEIGHT_SPREADSHEET_ID,
            SHEET_RANGES_OF_TOTAL_CARDS_FROM_PRE_REWORK_WEIGHT_LEAGUE,
            Credential::ApiKey(api_key),
        )
        .await?;
        let data = Input::try_from(batch_read)?;
        let sample = Sample::create(data, None)?;
        Ok(sample)
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
