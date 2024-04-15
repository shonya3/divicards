use reqwest::{Client, RequestBuilder};
use serde::Deserialize;
use serde_json::Value;
use tauri::{command, State, Window};
use tokio::sync::Mutex;
use tracing::instrument;

use crate::{
    error::Error,
    poe::{types::TabWithItems, AccessTokenStorage, Persist, API_URL},
    prices::AppCardPrices,
    version::AppVersion,
};

use divi::{
    prices::Prices,
    sample::{DivinationCardsSample, SampleData},
    {League, TradeLeague},
};

#[instrument(skip(prices, window))]
#[command]
pub async fn sample_from_tab(
    league: League,
    stash_id: String,
    substash_id: Option<String>,
    prices: State<'_, Mutex<AppCardPrices>>,
    version: State<'_, AppVersion>,
    window: Window,
) -> Result<DivinationCardsSample, Error> {
    let tab = StashAPI::tab_with_items(&league, stash_id, substash_id, version.inner()).await?;

    let prices = match TradeLeague::try_from(league) {
        Ok(league) => {
            let mut guard = prices.lock().await;
            guard.get_price(&league, &window).await
        }
        Err(_) => Prices::default(),
    };

    let sample = DivinationCardsSample::create(SampleData::from(tab), Some(prices))?;
    Ok(sample)
}

#[instrument]
#[command]
pub async fn tab_with_items(
    league: League,
    stash_id: String,
    version: State<'_, AppVersion>,
) -> Result<TabWithItems, Error> {
    StashAPI::tab_with_items(&league, stash_id, None, version.inner()).await
}

#[instrument]
#[command]
pub async fn stashes(league: League, version: State<'_, AppVersion>) -> Result<Value, Error> {
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
                format!("{}/stash/{}/{}/{}", API_URL, league, stash_id, substash_id)
            }
            None => format!("{}/stash/{}/{}", API_URL, league, stash_id),
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
                    "x-rate-limit-account: {:?}, x-rate-limit-account-state: {:?}",
                    limit_account_header, limit_account_state_header
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

    async fn stashes(league: League, version: &AppVersion) -> Result<Value, Error> {
        let url = format!("{}/stash/{}", API_URL, league);
        let response = StashAPI::with_auth_headers(&url, version).send().await?;
        let value = response.json::<Value>().await?;
        Ok(value)
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
) -> Result<DivinationCardsSample, Error> {
    let prices = match TradeLeague::try_from(league) {
        Ok(league) => {
            let mut guard = prices.lock().await;
            guard.get_price(&league, &window).await
        }
        Err(_) => Prices::default(),
    };

    let sample = DivinationCardsSample::create(SampleData::from(tab), Some(prices))?;
    Ok(sample)
}
