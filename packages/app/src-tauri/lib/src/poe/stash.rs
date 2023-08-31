use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;
use tauri::{command, AppHandle, State, Window};
use tokio::sync::Mutex;
use tracing::instrument;

use crate::{
    error::Error,
    poe::{types::TabWithItems, AccessTokenStorage, Persist, API_URL, PROVIDER_LABEL},
    prices::AppCardPrices,
};

use divi::{
    league::{League, TradeLeague},
    prices::Prices,
    sample::{DivinationCardsSample, SampleData},
};

#[instrument(skip(app_handle, state, window))]
#[command]
pub async fn sample_from_tab(
    league: League,
    stash_id: String,
    substash_id: Option<String>,
    app_handle: AppHandle,
    state: State<'_, Mutex<AppCardPrices>>,
    window: Window,
) -> Result<DivinationCardsSample, Error> {
    let tab = StashAPI::tab_with_items(
        &league,
        stash_id,
        substash_id,
        app_handle.config().package.version.clone().unwrap(),
    )
    .await?;

    let prices = match TradeLeague::try_from(league) {
        Ok(league) => {
            let mut guard = state.lock().await;
            guard.get_price(&league, &window).await
        }
        Err(_) => Prices::default(),
    };

    let sample = DivinationCardsSample::create(SampleData::from(tab), Some(prices))?;
    Ok(sample)
}

#[instrument(skip(app_handle))]
#[command]
pub async fn stashes(league: League, app_handle: AppHandle) -> Result<Value, Error> {
    StashAPI::stashes(league, app_handle.config().package.version.clone().unwrap()).await
}

pub fn access_token_label() -> String {
    format!("{}_access_token", { PROVIDER_LABEL })
}

pub struct StashAPI;
impl StashAPI {
    async fn tab_with_items(
        league: &League,
        stash_id: String,
        substash_id: Option<String>,
        version: String,
    ) -> Result<TabWithItems, Error> {
        let url = match substash_id {
            Some(substash_id) => {
                format!("{}/stash/{}/{}/{}", API_URL, league, stash_id, substash_id)
            }
            None => format!("{}/stash/{}/{}", API_URL, league, stash_id),
        };

        dbg!(&url);
        let response = Client::new()
            .get(url)
            .header(
                "Authorization",
                format!("Bearer {}", { AccessTokenStorage::new().get().unwrap() }),
            )
            .header(
                "User-Agent",
                format!("OAuth divicards/{} (contact: poeshonya3@gmail.com)", {
                    version
                }),
            )
            .send()
            .await?;

        let headers = &response.headers();

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

    async fn stashes(league: League, version: String) -> Result<Value, Error> {
        let url = format!("{}/stash/{}", API_URL, league);
        let response = Client::new()
            .get(format!("{}/stash/{}", API_URL, league))
            .header(
                "Authorization",
                format!("Bearer {}", { AccessTokenStorage::new().get().unwrap() }),
            )
            .header(
                "User-Agent",
                format!("OAuth divicards/{} (contact: poeshonya3@gmail.com)", {
                    version
                }),
            )
            .send()
            .await?;
        let value = response.json::<Value>().await?;
        Ok(value)
    }
}
