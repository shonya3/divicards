use super::icon::FetchMapIconError;
use super::Map;
use crate::cards::Card;
use crate::maps::wiki::{FetchWikiMapsError, MapDataFromWiki};
use playwright::Playwright;
use std::sync::Arc;
use tokio::task::JoinHandle;

pub async fn fetch_maps() -> Result<Vec<Map>, FetchMapsError> {
    let cards = crate::cards::fetch::fetch().await?;
    let cards: Vec<Card> = cards.0.values().cloned().collect();
    let cards = Arc::new(cards);

    // Prepare Playwright context
    let playwright = Playwright::initialize().await.unwrap();
    let playwright = Arc::new(playwright);
    let chrome = playwright.chromium();
    let browser = chrome.launcher().headless(true).launch().await.unwrap();
    let context = browser
        .context_builder()
        .clear_user_agent()
        .build()
        .await
        .unwrap();
    let context = Arc::new(context);

    let mut tasks = vec![];
    let wiki_maplist = super::wiki::fetch_wiki_maplist().await?;
    for wiki_maps_chunked in wiki_maplist
        .chunks(20)
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<_>>()
    {
        let context = Arc::clone(&context);
        let playwright = Arc::clone(&playwright);
        let cards = cards.clone();

        let task: JoinHandle<Result<Vec<Map>, FetchMapsError>> = tokio::spawn(async move {
            let mut task_maps: Vec<Map> = vec![];
            let page = context.new_page().await.unwrap();
            for MapDataFromWiki {
                name, tier, series, ..
            } in wiki_maps_chunked
            {
                let is_unique_map = !name.ends_with(" Map");

                let icon = super::icon::get_map_icon(&name, &page, &playwright).await?;

                let atlas_cards: Vec<String> = cards
                    .iter()
                    .filter(|card| card.atlas_maps.contains(&name))
                    .map(|card| card.name.clone())
                    .collect();

                let map = Map {
                    slug: slug::slugify(&name),
                    name,
                    tier,
                    series,
                    unique: is_unique_map,
                    icon: super::icon::poecdn_icon_url(&icon),
                    atlas_cards,
                };
                task_maps.push(map);
            }

            Ok(task_maps)
        });

        tasks.push(task);
    }

    let mut maps: Vec<Map> = vec![];
    for task_handle in tasks {
        let task_maps = task_handle.await.unwrap()?;
        maps.extend(task_maps);
    }

    Ok(maps)
}

#[derive(Debug)]
pub enum FetchMapsError {
    Playwright(Arc<playwright::Error>),
    FetchWikiMaps(FetchWikiMapsError),
    FetchMapIcon(FetchMapIconError),
    Reqwest(reqwest::Error),
    MapsItemContainerNotFound,
    NameElementNotFound,
    ParseMapTier { name: String, tier_string: String },
    FetchCards(crate::cards::fetch::Error),
    EmptyMapName,
}

impl From<Arc<playwright::Error>> for FetchMapsError {
    fn from(value: Arc<playwright::Error>) -> Self {
        Self::Playwright(value)
    }
}

impl From<FetchMapIconError> for FetchMapsError {
    fn from(value: FetchMapIconError) -> Self {
        Self::FetchMapIcon(value)
    }
}

impl From<reqwest::Error> for FetchMapsError {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}

impl From<FetchWikiMapsError> for FetchMapsError {
    fn from(value: FetchWikiMapsError) -> Self {
        Self::FetchWikiMaps(value)
    }
}

impl From<crate::cards::fetch::Error> for FetchMapsError {
    fn from(value: crate::cards::fetch::Error) -> Self {
        Self::FetchCards(value)
    }
}
