use serde::{Deserialize, Serialize};

pub const POE_CDN_CARDS: &'static str = "https://web.poecdn.com/image/divination-card/";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub name: String,
    pub min_level: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_level: Option<u32>,
    pub weight: Option<f32>,
    pub price: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardsData(Vec<Card>);
impl CardsData {
    pub fn card(&self, s: &str) -> &Card {
        let Some(card) = self.0.iter().find(|card| card.name == s) else {
            panic!("Card not exist {s}");
        };

        card
    }
}

#[cfg(feature = "fetch")]
pub async fn fetch() -> Result<CardsData, crate::error::Error> {
    use divi::{
        prices::Prices,
        sample::{DivinationCardsSample, SampleData},
    };
    use googlesheets::sheet::Credential;
    use reqwest::Client;

    use crate::{
        consts::{WEIGHT_RANGES, WIKI_API_URL},
        error::Error,
    };

    println!("Fetching cards");
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct WikiCard {
        pub name: String,
        #[serde(alias = "drop level")]
        pub min_level: Option<u32>,
        #[serde(alias = "drop areas")]
        pub drop_areas: Option<Vec<String>>,
        #[serde(alias = "drop monsters")]
        pub drop_monsters: Option<Vec<String>>,
        #[serde(alias = "drop level maximum")]
        pub max_level: Option<u32>,
    }

    pub async fn load_wiki() -> Result<Vec<WikiCard>, Error> {
        #[derive(Serialize, Deserialize, Debug, Clone)]
        pub struct WikiCardsResponse {
            pub cargoquery: Vec<WikiCardWrapper>,
        }

        #[derive(Serialize, Deserialize, Debug, Clone)]
        pub struct WikiCardRaw {
            name: String,
            #[serde(alias = "drop level")]
            min_level: Option<String>,
            #[serde(alias = "drop areas")]
            pub drop_areas: Option<String>,
            #[serde(alias = "drop monsters")]
            pub drop_monsters: Option<String>,
            #[serde(alias = "drop level maximum")]
            pub max_level: Option<String>,
        }

        #[derive(Serialize, Deserialize, Debug, Clone)]
        pub struct WikiCardWrapper {
            title: WikiCardRaw,
        }

        let url = format!("{WIKI_API_URL}?action=cargoquery&format=json&smaxage=1&maxage=1&tables=items&limit=500&fields=items.name,items.drop_level,items.drop_level_maximum,items.drop_areas,items.drop_monsters&where=items.class_id='DivinationCard'");

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
                        .map(|s| s.split(",").into_iter().map(|s| s.to_string()).collect()),
                    drop_monsters: raw
                        .drop_monsters
                        .map(|s| s.split(",").into_iter().map(|s| s.to_string()).collect()),
                    max_level: raw.max_level.map(|s| s.parse().unwrap()),
                }
            })
            .collect();

        Ok(vec)
    }

    pub async fn load_total_sample(
        api_key: String,
        prices: Option<Prices>,
    ) -> Result<DivinationCardsSample, Error> {
        let batch_read = googlesheets::read_batch(
            "1NDTZqLcwrKjR3CflLU7B7IGJtzynNoWTe7AVQ-gUA-c",
            WEIGHT_RANGES,
            Credential::ApiKey(api_key),
        )
        .await?;
        let data = SampleData::try_from(batch_read)?;
        let sample = DivinationCardsSample::create(data, prices)?;
        Ok(sample)
    }

    dotenv::dotenv().ok();
    let key = std::env::var("GOOGLE_API_KEY").expect("No google api key");
    let prices = Prices::fetch(&divi::league::TradeLeague::Standard).await?;
    let sample = load_total_sample(key, Some(prices)).await?;
    let mut wiki_vec = load_wiki().await.unwrap();

    Ok(CardsData(
        sample
            .cards
            .into_iter()
            .map(|card| {
                let (min_level, max_level) = wiki_vec
                    .iter()
                    .position(|w| w.name == card.name)
                    .and_then(|index| Some(wiki_vec.swap_remove(index)))
                    .map(|w| (w.min_level, w.max_level))
                    .unwrap_or_default();

                Card {
                    name: card.name,
                    min_level,
                    max_level,
                    weight: card.weight,
                    price: card.price,
                }
            })
            .collect(),
    ))
}
