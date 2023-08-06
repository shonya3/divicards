pub mod auth;

use divi::league::League;

use keyring::Entry;
use reqwest::Client;
use serde_json::Value;
use tauri::{command, AppHandle};

pub const API_URL: &'static str = "https://api.pathofexile.com";
const PROVIDER_LABEL: &'static str = "poe";
const CLIENT_ID: &'static str = "divicards";
const AUTH_URL: &'static str = "https://www.pathofexile.com/oauth/authorize";
const TOKEN_URL: &'static str = "https://www.pathofexile.com/oauth/token";

#[command]
pub async fn stashes(league: League, app_handle: AppHandle) -> Value {
    let val =
        PoeProvider::stashes(league, app_handle.config().package.version.clone().unwrap()).await;
    // dbg!(&val);
    val
}

#[command]
pub async fn stash(
    league: League,
    stash_id: String,
    substash_id: Option<String>,
    app_handle: AppHandle,
) -> Value {
    PoeProvider::stash(
        league,
        stash_id,
        substash_id,
        app_handle.config().package.version.clone().unwrap(),
    )
    .await
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

    async fn stash(
        league: League,
        stash_id: String,
        substash_id: Option<String>,
        version: String,
    ) -> Value {
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
            .await
            .unwrap();

        let headers = &response.headers();
        let limit_account_header = headers.get("x-rate-limit-account").unwrap();
        let limit_account_state_header = headers.get("x-rate-limit-account-state").unwrap();

        println!(
            "x-rate-limit-account: {:?}, x-rate-limit-account-state: {:?}",
            limit_account_header, limit_account_state_header
        );

        response.json::<Value>().await.unwrap()
    }

    async fn stashes(league: League, version: String) -> Value {
        let url = format!("{}/stash/{}", API_URL, league);
        dbg!(url);
        Client::new()
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
            .await
            .unwrap()
            .json::<Value>()
            .await
            .unwrap()
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
