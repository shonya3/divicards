use super::icon::FetchMapIconError;
use super::Map;
use crate::consts::POEDB_MAPS_URL;
use crate::maps::wiki::MapDataFromWiki;
use playwright::api::{DocumentLoadState, ElementHandle, Page};
use playwright::Playwright;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::task::JoinHandle;

pub async fn fetch_maps() -> Result<Vec<Map>, FetchMapsError> {
    let mut maps: Vec<Map> = vec![];
    let wiki_maplist = super::wiki::fetch_wiki_maplist().await?;

    // Prepare Playwright context
    let playwright = Playwright::initialize().await.unwrap();
    let playwright = Arc::new(playwright);
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
        load_poedb_available_non_unique_name_tier_list(&context.new_page().await?, &playwright)
            .await?;
    let poedb_available_maps = Arc::new(poedb_available_maps);
    let mut tasks = vec![];

    for wiki_maps_chunked in wiki_maplist
        .chunks(20)
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<_>>()
    {
        let context = Arc::clone(&context);
        let poedb_available_maps = Arc::clone(&poedb_available_maps);
        let playwright = Arc::clone(&playwright);

        let task: JoinHandle<Result<Vec<Map>, FetchMapsError>> = tokio::spawn(async move {
            let mut task_maps: Vec<Map> = vec![];
            let page = context.new_page().await.unwrap();
            for MapDataFromWiki { name, mut tier } in wiki_maps_chunked {
                let is_unique_map = !name.ends_with(" Map");

                let poedb = poedb_available_maps
                    .iter()
                    .find(|poedb| poedb.name == name.as_str());
                if let Some(poedb) = &poedb {
                    tier = poedb.tier;
                }

                let icon = super::icon::get_map_icon(&name, &page, &playwright).await?;
                let map = Map {
                    slug: slug::slugify(&name),
                    name,
                    tier,
                    available: is_unique_map || poedb.is_some(),
                    unique: is_unique_map,
                    icon: super::icon::poecdn_icon_url(&icon),
                };
                task_maps.push(map);
            }

            Ok(task_maps)
        });

        tasks.push(task);
    }

    for task_handle in tasks {
        let task_maps = task_handle.await.unwrap()?;
        maps.extend(task_maps);
    }

    Ok(maps)
}

#[derive(Debug)]
pub enum FetchMapsError {
    Playwright(Arc<playwright::Error>),
    FetchMapIcon(FetchMapIconError),
    Reqwest(reqwest::Error),
    MapsItemContainerNotFound,
    NameElementNotFound,
    ParseMapTier { name: String, tier_string: String },
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

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MapNameTier {
    pub name: String,
    pub tier: u32,
}

/// Loads map names and tiers from poedb https://poedb.tw/us/Maps#MapsItem
/// Skip the maps with no tier(i.e. not in current atlas).
async fn load_poedb_available_non_unique_name_tier_list(
    page: &Page,
    _playwright: &Playwright,
) -> Result<Vec<MapNameTier>, FetchMapsError> {
    page.goto_builder(POEDB_MAPS_URL)
        .wait_until(DocumentLoadState::DomContentLoaded)
        .goto()
        .await?;

    async fn extract_map_name_tier(
        map_container: &ElementHandle,
    ) -> Result<Option<MapNameTier>, FetchMapsError> {
        let Some(name) = map_container
            .query_selector(".itemclass_map")
            .await?
            .ok_or(FetchMapsError::NameElementNotFound)?
            .text_content()
            .await?
            .map(|text| text.trim().to_string())
        else {
            return Ok(None);
        };

        let re = Regex::new(r"\d+").unwrap();
        for property_block in map_container.query_selector_all(".property").await? {
            let Some(text) = property_block.text_content().await? else {
                continue;
            };
            let text = text.trim();
            if !text.starts_with("Map Tier") {
                continue;
            }
            if let Some(first_match) = re.find(text) {
                let tier: u32 =
                    first_match
                        .as_str()
                        .parse()
                        .map_err(|_| FetchMapsError::ParseMapTier {
                            name: name.clone(),
                            tier_string: text.to_owned(),
                        })?;
                return Ok(Some(MapNameTier { name, tier }));
            }
        }

        Ok(None)
    }

    let mut vec: Vec<MapNameTier> = vec![];
    for map_container in page
        .query_selector("#MapsItem")
        .await?
        .ok_or(FetchMapsError::MapsItemContainerNotFound)?
        .query_selector_all(".col")
        .await?
    {
        if let Some(name_tier) = extract_map_name_tier(&map_container).await? {
            vec.push(name_tier);
        };
    }

    Ok(vec)
}

// Run with cargo test --features "fetch"
#[cfg(test)]
#[cfg(feature = "fetch")]
mod tests {
    use playwright::{api::Page, Playwright};

    async fn create_playwright() -> (Page, Playwright) {
        let playwright = Playwright::initialize().await.unwrap();
        let chrome = playwright.chromium();
        let browser = chrome.launcher().headless(false).launch().await.unwrap();
        let context = browser
            .context_builder()
            .clear_user_agent()
            .build()
            .await
            .unwrap();
        let page = context.new_page().await.unwrap();
        (page, playwright)
    }

    #[tokio::test]
    #[cfg(feature = "fetch")]
    async fn poedb_available_maps() {
        let (page, playwright) = create_playwright().await;
        let result =
            super::load_poedb_available_non_unique_name_tier_list(&page, &playwright).await;
        assert!(result.unwrap().len() > 80);
    }
}
