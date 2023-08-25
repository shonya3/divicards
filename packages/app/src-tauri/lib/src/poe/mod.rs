pub mod auth;
pub mod error;
pub mod types;

use divi::{
    league::{League, TradeLeague},
    prices::Prices,
    sample::{DivinationCardsSample, SampleData},
};

use keyring::Entry;
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;
use tauri::{command, AppHandle, State, Window};
use tokio::sync::Mutex;

use crate::{error::Error, js_result::JSResult, prices::AppCardPrices};

use self::types::TabWithItems;

pub const API_URL: &'static str = "https://api.pathofexile.com";
const PROVIDER_LABEL: &'static str = "poe";
const CLIENT_ID: &'static str = "divicards";
const AUTH_URL: &'static str = "https://www.pathofexile.com/oauth/authorize";
const TOKEN_URL: &'static str = "https://www.pathofexile.com/oauth/token";

#[command]
pub async fn sample_from_tab(
    league: League,
    stash_id: String,
    substash_id: Option<String>,
    app_handle: AppHandle,
    state: State<'_, Mutex<AppCardPrices>>,
    window: Window,
) -> Result<JSResult<DivinationCardsSample>, Error> {
    let tab = PoeProvider::tab_with_items(
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

    Ok(JSResult::from(DivinationCardsSample::create(
        SampleData::from(tab),
        Some(prices),
    )))
}

#[command]
pub async fn stashes(league: League, app_handle: AppHandle) -> Result<Value, Error> {
    PoeProvider::stashes(league, app_handle.config().package.version.clone().unwrap()).await
}

#[derive(Default)]
pub struct PoeProvider;

impl PoeProvider {
    pub fn new() -> PoeProvider {
        PoeProvider::default()
    }

    pub fn access_token_label() -> String {
        format!("{}_access_token", { PROVIDER_LABEL })
    }

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
        dbg!(url);
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

#[derive(Debug)]
pub struct AccessTokenStorage(Entry);

impl AccessTokenStorage {
    pub fn new() -> Self {
        AccessTokenStorage::default()
    }
}

impl Default for AccessTokenStorage {
    fn default() -> Self {
        AccessTokenStorage(Entry::new("divicards", Self::KEY_NAME).unwrap())
    }
}

impl Persist for AccessTokenStorage {
    const KEY_NAME: &'static str = "poe_access_token";
    fn get(&self) -> Result<String, keyring::Error> {
        self.0.get_password()
    }

    fn set(&self, value: &str) -> Result<(), keyring::Error> {
        self.0.set_password(value)
    }

    fn delete(&self) -> Result<(), keyring::Error> {
        self.0.delete_password()
    }
}

pub trait Persist {
    const KEY_NAME: &'static str;
    fn get(self: &Self) -> Result<String, keyring::Error>;
    fn set(self: &Self, value: &str) -> Result<(), keyring::Error>;
    fn delete(self: &Self) -> Result<(), keyring::Error>;
}
