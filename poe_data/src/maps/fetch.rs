use super::icon::FetchMapIconError;
use super::Map;
use crate::consts::POEDB_MAPS_URL;
use crate::maps::wiki::MapDataFromWiki;
use playwright::Playwright;
use std::sync::Arc;
use tokio::task::JoinHandle;

pub async fn fetch_maps() -> Result<Vec<Map>, FetchMapsError> {
    let mut maps: Vec<Map> = vec![];
    let wiki_maplist = super::wiki::fetch_wiki_maplist().await?;

    // Prepare Playwright context
    let playwright = Playwright::initialize().await.unwrap();
    let chrome = playwright.chromium();
    let browser = chrome.launcher().headless(false).launch().await.unwrap();
    let context = browser
        .context_builder()
        .clear_user_agent()
        .build()
        .await
        .unwrap();
    let context = Arc::new(context);

    let poedb_available_maps =
        load_poedb_non_unique_available_maplist(&context.new_page().await.unwrap()).await?;
    let poedb_available_maps = Arc::new(poedb_available_maps);
    let mut tasks = vec![];

    for wiki_maps_chunked in wiki_maplist
        .chunks(20)
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<_>>()
    {
        let context = Arc::clone(&context);
        let poedb_available_maps = Arc::clone(&poedb_available_maps);

        let task: JoinHandle<Result<Vec<Map>, FetchMapsError>> = tokio::spawn(async move {
            let mut task_maps: Vec<Map> = vec![];
            let page = context.new_page().await.unwrap();
            for MapDataFromWiki { name, tier } in wiki_maps_chunked {
                let is_unique_map = !name.ends_with(" Map");
                let is_available = is_unique_map || poedb_available_maps.contains(&name);
                let icon = super::icon::get_map_icon(&name, &page).await?;
                let map = Map {
                    slug: slug::slugify(&name),
                    name,
                    tier,
                    available: is_available,
                    unique: is_unique_map,
                    icon: super::icon::poecdn_icon_url(&icon),
                };
                task_maps.push(map);
            }

            Ok(task_maps)
        });

        tasks.push(task);
    }

    for task in tasks {
        maps.extend(task.await.unwrap().unwrap());
    }

    Ok(maps)
}

#[derive(Debug)]
pub enum FetchMapsError {
    Playwright(Arc<playwright::Error>),
    FetchMapIcon(FetchMapIconError),
    Reqwest(reqwest::Error),
    MapsItemContainerNotFound,
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

/// Loads map names from poedb
async fn load_poedb_non_unique_available_maplist(
    page: &playwright::api::Page,
) -> Result<Vec<String>, FetchMapsError> {
    page.goto_builder(POEDB_MAPS_URL)
        .wait_until(playwright::api::DocumentLoadState::DomContentLoaded)
        .goto()
        .await?;
    let mut maps: Vec<String> = vec![];

    for map_block in page
        .query_selector("#MapsItem")
        .await?
        .ok_or(FetchMapsError::MapsItemContainerNotFound)?
        .query_selector_all(".itemclass_map")
        .await?
    {
        if let Some(map_name) = map_block.text_content().await.unwrap() {
            maps.push(map_name);
        };
    }
    maps.sort();
    Ok(maps)
}
