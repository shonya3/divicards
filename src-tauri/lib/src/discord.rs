use crate::oauth::{auth, OAuthProvider, Persistent};
use keyring::Entry;
use serde::{Deserialize, Serialize};
use tauri::command;

#[derive(Default)]
pub struct DiscordProvider;

impl DiscordProvider {
    fn access_token_label() -> String {
        format!("{}_access_token", { Self::PROVIDER_LABEL })
    }

    pub async fn identity(acces_token: String) -> Identity {
        reqwest::Client::new()
            .get("https://discord.com/api/users/@me")
            .header("authorization", format!("Bearer {}", { acces_token }))
            .send()
            .await
            .unwrap()
            .json::<Identity>()
            .await
            .unwrap()
    }
}

impl OAuthProvider for DiscordProvider {
    const PROVIDER_LABEL: &'static str = "discord";
    const CLIENT_ID: &'static str = "1105146245221191872";
    const AUTH_URL: &'static str = "https://discord.com/oauth2/authorize";
    const TOKEN_URL: &'static str = "https://discord.com/api/oauth2/token";
}

impl Persistent for DiscordProvider {
    fn access_token() -> Option<String> {
        Entry::new("divicards", &Self::access_token_label())
            .unwrap()
            .get_password()
            .ok()
    }

    fn set_access_token(token: String) -> Result<(), keyring::Error> {
        let entry = Entry::new("divicards", &Self::access_token_label()).unwrap();
        entry.set_password(&token)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Identity {
    pub id: String,
    pub username: String,
    pub global_name: Option<String>,
    pub display_name: Option<String>,
    pub avatar: Option<String>,
    pub discriminator: Option<String>,
    pub public_flags: usize,
    pub flags: usize,
    pub banner: Option<String>,
    pub banner_color: Option<String>,
    pub accent_color: Option<String>,
    pub locale: String,
    pub mfa_enabled: bool,
    pub premium_type: usize,
    pub avatar_decoration: Option<String>,
}

#[command]
pub async fn discord_auth() -> String {
    auth(DiscordProvider::default()).await
}

#[command]
pub async fn discord_authenticated() -> bool {
    match DiscordProvider::access_token() {
        Some(_) => true,
        None => false,
    }
}

#[command]
pub async fn discord_identity() -> Identity {
    DiscordProvider::identity(DiscordProvider::access_token().unwrap()).await
}
