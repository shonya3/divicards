use crate::league::LeagueReleaseInfo;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const POE_CDN_CARDS: &str = "https://web.poecdn.com/image/divination-card/";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub name: String,
    pub min_level: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_level: Option<u32>,
    pub weight: Option<f32>,
    pub price: Option<f32>,
    #[serde(alias = "release version")]
    pub league: Option<LeagueReleaseInfo>,
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

#[cfg(feature = "fetch")]
pub mod fetch {
    use super::CardsData;
    use crate::{
        cards::Card,
        consts::{
            SHEET_RANGES_OF_TOTAL_CARDS_FROM_LATEST_LEAGUE, WEIGHT_SPREADSHEET_ID, WIKI_API_URL,
        },
        error::Error,
        league::ReleaseVersion,
    };
    use divi::{
        prices::Prices,
        sample::{Input, Sample},
        Error as DiviError,
    };
    use googlesheets::sheet::Credential;
    use reqwest::Client;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct WikiCard {
        name: String,
        #[serde(alias = "drop level")]
        min_level: Option<u32>,
        #[serde(alias = "drop areas")]
        drop_areas: Option<Vec<String>>,
        #[serde(alias = "drop monsters")]
        drop_monsters: Option<Vec<String>>,
        #[serde(alias = "drop level maximum")]
        max_level: Option<u32>,
        #[serde(alias = "release version")]
        release_version: Option<ReleaseVersion>,
    }

    pub async fn fetch() -> Result<CardsData, crate::error::Error> {
        println!("Fetching cards");
        dotenv::dotenv().ok();
        let key = std::env::var("GOOGLE_API_KEY").expect("No google api key");
        let prices = Prices::fetch(&divi::TradeLeague::Standard)
            .await
            .map_err(DiviError::NinjaError)?;
        let sample = load_total_sample(key, Some(prices)).await?;
        let mut wiki_vec = load_wiki().await?;

        let league_info_vec = crate::league::LeagueReleaseInfo::fetch().await?;
        let vec_json = serde_json::to_string(&league_info_vec).unwrap();
        println!("{vec_json}");
        let mut vec: Vec<Card> = sample
            .cards
            .into_iter()
            .map(|card| {
                let (min_level, max_level, release_version) = wiki_vec
                    .iter()
                    .position(|w| w.name == card.name)
                    .map(|index| wiki_vec.swap_remove(index))
                    .map(|w| (w.min_level, w.max_level, w.release_version))
                    .unwrap_or_default();

                let league = release_version
                    .and_then(|version| {
                        league_info_vec
                            .iter()
                            .find(|info| info.version.same_league(&version))
                    })
                    .cloned();

                Card {
                    name: card.name,
                    min_level,
                    max_level,
                    weight: card.weight,
                    price: card.price,
                    league,
                }
            })
            .collect();

        let big_value = 1_000_000.0;
        vec.sort_by(|a, b| {
            let a_weight = a.weight.unwrap_or(big_value);
            let b_weight = b.weight.unwrap_or(big_value);
            a_weight.partial_cmp(&b_weight).unwrap()
        });

        Ok(CardsData(HashMap::from_iter(
            vec.into_iter().map(|c| (c.name.clone(), c)),
        )))
    }

    /// Loads Total amounts from latest league, constructs Sample from them https://docs.google.com/spreadsheets/d/1PmGES_e1on6K7O5ghHuoorEjruAVb7dQ5m7PGrW7t80/edit#gid=898101079
    async fn load_total_sample(api_key: String, prices: Option<Prices>) -> Result<Sample, Error> {
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

    async fn load_wiki() -> Result<Vec<WikiCard>, Error> {
        #[derive(Serialize, Deserialize, Debug, Clone)]
        struct WikiCardsResponse {
            cargoquery: Vec<WikiCardWrapper>,
        }

        #[derive(Serialize, Deserialize, Debug, Clone)]
        struct WikiCardRaw {
            name: String,
            #[serde(alias = "drop level")]
            min_level: Option<String>,
            #[serde(alias = "drop areas")]
            drop_areas: Option<String>,
            #[serde(alias = "drop monsters")]
            drop_monsters: Option<String>,
            #[serde(alias = "drop level maximum")]
            max_level: Option<String>,
            #[serde(alias = "release version")]
            release_version: Option<ReleaseVersion>,
        }

        #[derive(Serialize, Deserialize, Debug, Clone)]
        struct WikiCardWrapper {
            title: WikiCardRaw,
        }

        let url = format!("{WIKI_API_URL}?action=cargoquery&format=json&smaxage=1&maxage=1&tables=items&limit=500&fields=items.release_version,items.name,items.drop_level,items.drop_level_maximum,items.drop_areas,items.drop_monsters&where=items.class_id='DivinationCard'");

        dbg!(&url);

        let response = Client::new().get(url).send().await?;
        let wiki_maps: WikiCardsResponse = response.json().await?;

        let vec: Vec<WikiCard> = wiki_maps
            .cargoquery
            .into_iter()
            .map(|wrapper| {
                let raw = wrapper.title;
                WikiCard {
                    name: raw.name.clone(),
                    min_level: raw.min_level.map(|s| s.parse().unwrap()),
                    drop_areas: raw
                        .drop_areas
                        .map(|s| s.split(',').map(|s| s.to_string()).collect()),
                    drop_monsters: raw
                        .drop_monsters
                        .map(|s| s.split(',').map(|s| s.to_string()).collect()),
                    max_level: raw.max_level.map(|s| s.parse().unwrap()),
                    release_version: raw.release_version,
                }
            })
            .collect();

        Ok(vec)
    }
}
