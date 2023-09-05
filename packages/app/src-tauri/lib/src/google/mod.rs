pub mod auth;

use keyring::Entry;
use serde::{Deserialize, Serialize};

// const PROVIDER_LABEL: &'static str = "google";
const CLIENT_ID: &'static str =
    "752206000922-1gpkcoplrjqpfgg8pr4sb4tnrlvauomp.apps.googleusercontent.com";
const AUTH_URL: &'static str = "https://accounts.google.com/o/oauth2/auth";
const TOKEN_URL: &'static str = "https://oauth2.googleapis.com/token";

#[derive(Debug)]
pub struct AccessTokenStorage(Entry);

impl AccessTokenStorage {
    pub fn new() -> Self {
        AccessTokenStorage::default()
    }
}

impl Default for AccessTokenStorage {
    fn default() -> Self {
        AccessTokenStorage(Entry::new("divicards", "google_access_token").unwrap())
    }
}

impl Persist for AccessTokenStorage {
    const KEY_NAME: &'static str = "google_access_token";
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Identity {
    pub given_name: Option<String>,
    pub name: Option<String>,
    pub id: String,
    pub picture: Option<String>,
    pub locale: Option<String>,
}
