use keyring::Entry;

pub mod auth;
pub mod error;
pub mod stash;
pub mod types;

pub const API_URL: &'static str = "https://api.pathofexile.com";
const PROVIDER_LABEL: &'static str = "poe";
const CLIENT_ID: &'static str = "divicards";
const AUTH_URL: &'static str = "https://www.pathofexile.com/oauth/authorize";
const TOKEN_URL: &'static str = "https://www.pathofexile.com/oauth/token";

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
