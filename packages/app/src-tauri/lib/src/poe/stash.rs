use super::types::TabNoItems;
use crate::{
    error::Error,
    poe::{types::TabWithItems, AccessTokenStorage, Persist, API_URL},
    prices::AppCardPrices,
    version::AppVersion,
};
use divi::{
    prices::Prices,
    sample::{Input, Sample},
    {League, TradeLeague},
};
use reqwest::{Client, RequestBuilder};
use serde::Deserialize;
use tauri::{command, State, Window};
use tokio::sync::Mutex;
use tracing::instrument;

#[instrument(skip(prices, window))]
#[command]
pub async fn sample_from_tab(
    league: League,
    stash_id: String,
    substash_id: Option<String>,
    prices: State<'_, Mutex<AppCardPrices>>,
    version: State<'_, AppVersion>,
    window: Window,
) -> Result<Sample, Error> {
    let tab =
        StashAPI::tab_with_items(&league, stash_id.clone(), substash_id, version.inner()).await?;

    let prices = match TradeLeague::try_from(league.clone()) {
        Ok(league) => {
            let mut guard = prices.lock().await;
            guard.get_price(&league, &window).await
        }
        Err(_) => Prices::default(),
    };

    let sample = Sample::create(Input::from(tab), Some(prices)).map_err(|divi_err| {
        Error::StashTabError {
            stash_id,
            league,
            message: divi_err.to_string(),
        }
    })?;
    Ok(sample)
}

#[instrument]
#[command]
pub async fn tab_with_items(
    league: League,
    stash_id: String,
    substash_id: Option<String>,
    version: State<'_, AppVersion>,
    ) -> Result<TabWithItems, Error> {
    let tab = StashAPI::tab_with_items(&league, stash_id.clone(), substash_id.clone(), version.inner()).await?;
    let item_count = tab.items().count();
    let map_count = tab
        .items()
        .filter(|i| i.base_type().is_some_and(|b| b.ends_with(" Map")))
        .count();
    tracing::info!(
        league = %league,
        stash_id = %stash_id,
        substash_id = ?substash_id,
        items = item_count,
        maps = map_count,
        "tab_with_items response"
    );
    Ok(tab)
}

#[command]
pub async fn extract_cards(
    tab: TabWithItems,
    league: League,
    prices: State<'_, Mutex<AppCardPrices>>,
    window: Window,
) -> Result<Sample, Error> {
    let prices = match TradeLeague::try_from(league.clone()) {
        Ok(league) => {
            let mut guard = prices.lock().await;
            guard.get_price(&league, &window).await
        }
        Err(_) => Prices::default(),
    };

    let tab_id = tab.id().unwrap_or_else(|_| "No tab id".to_string());
    let sample = Sample::create(Input::from(tab), Some(prices)).map_err(|divi_err| {
        Error::StashTabError {
            stash_id: tab_id,
            league,
            message: divi_err.to_string(),
        }
    })?;
    Ok(sample)
}

#[instrument]
#[command]
pub async fn stashes(league: League, version: State<'_, AppVersion>) -> Result<TabNoItems, Error> {
    StashAPI::stashes(league, version.inner()).await
}

pub struct StashAPI;
impl StashAPI {
    async fn tab_with_items(
        league: &League,
        stash_id: String,
        substash_id: Option<String>,
        version: &AppVersion,
    ) -> Result<TabWithItems, Error> {
        let url = match substash_id {
            Some(substash_id) => {
                format!("{API_URL}/stash/{league}/{stash_id}/{substash_id}")
            }
            None => format!("{API_URL}/stash/{league}/{stash_id}"),
        };

        let response = StashAPI::with_auth_headers(&url, version).send().await?;

        let headers = &response.headers();
        if let Some(s) = headers.get("retry-after") {
            let s = s.to_str().unwrap().to_owned();
            return Err(Error::RetryAfter(s));
        }
        if let Some(limit_account_header) = headers.get("x-rate-limit-account") {
            if let Some(limit_account_state_header) = headers.get("x-rate-limit-account-state") {
                println!(
                    "x-rate-limit-account: {limit_account_header:?}, x-rate-limit-account-state: {limit_account_state_header:?}"
                );
            };
        };

        #[derive(Deserialize)]
        struct ResponseShape {
            stash: TabWithItems,
        }

        let response_shape = response.json::<ResponseShape>().await?;
        Ok(response_shape.stash)
    }

    async fn stashes(league: League, version: &AppVersion) -> Result<TabNoItems, Error> {
        let url = format!("{API_URL}/stash/{league}");
        let response = StashAPI::with_auth_headers(&url, version).send().await?;
        Ok(response.json().await?)
    }

    fn with_auth_headers(url: &str, version: &AppVersion) -> RequestBuilder {
        Client::new()
            .get(url)
            .header("Authorization", format!("Bearer {}", { access_token() }))
            .header(
                "User-Agent",
                format!("OAuth divicards/{} (contact: poeshonya3@gmail.com)", {
                    version
                }),
            )
    }
}

fn access_token() -> String {
    AccessTokenStorage::new().get().unwrap()
}

#[instrument]
#[command]
pub async fn tab(
    league: League,
    stash_id: String,
    version: State<'_, AppVersion>,
) -> Result<TabWithItems, Error> {
    StashAPI::tab_with_items(&league, stash_id, None, &version).await
}

#[instrument(skip(prices, window, tab))]
#[command]
pub async fn sample_from_tab_with_items(
    league: League,
    tab: TabWithItems,
    prices: State<'_, Mutex<AppCardPrices>>,
    window: Window,
) -> Result<Sample, Error> {
    let prices = match TradeLeague::try_from(league) {
        Ok(league) => {
            let mut guard = prices.lock().await;
            guard.get_price(&league, &window).await
        }
        Err(_) => Prices::default(),
    };

    let sample = Sample::create(Input::from(tab), Some(prices))?;
    Ok(sample)
}
