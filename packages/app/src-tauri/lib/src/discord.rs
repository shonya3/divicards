use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use crate::oauth::{AuthCodeResponse, OAuthProvider};
use axum::{async_trait, extract::Query, response::Html, routing::get, Router};
use keyring::Entry;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, CsrfToken, PkceCodeChallenge, RedirectUrl,
    Scope, TokenResponse,
};
use serde::{Deserialize, Serialize};
use tauri::command;
use tokio::sync::mpsc;

#[derive(Default)]
pub struct DiscordProvider;

impl DiscordProvider {
    pub fn new() -> DiscordProvider {
        DiscordProvider::default()
    }

    pub fn access_token_label() -> String {
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

#[async_trait]
impl OAuthProvider for DiscordProvider {
    const PROVIDER_LABEL: &'static str = "discord";
    const CLIENT_ID: &'static str = "1105146245221191872";
    const AUTH_URL: &'static str = "https://discord.com/oauth2/authorize";
    const TOKEN_URL: &'static str = "https://discord.com/api/oauth2/token";

    async fn oauth(&self, _: Option<String>) -> Result<String, String> {
        let (sender, mut receiver) = mpsc::channel::<AuthCodeResponse>(1);
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 50151);
        let redirect_url =
            RedirectUrl::new(format!("http://localhost:{}/callback", addr.port())).unwrap();

        let client = BasicClient::new(
            DiscordProvider::client_id(),
            None,
            DiscordProvider::auth_url().unwrap(),
            DiscordProvider::token_url().ok(),
        )
        .set_redirect_uri(redirect_url);

        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let (auth_url, csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("identify".to_string()))
            .set_pkce_challenge(pkce_challenge)
            .url();

        let app = Router::new().route(
            "/callback",
            get(move |query: Query<AuthCodeResponse>| route_handler(query, sender)),
        );

        let (tx, rx) = tokio::sync::oneshot::channel::<()>();
        let server = axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .with_graceful_shutdown(async {
                rx.await.ok();
                println!("shutdown");
            });

        tokio::spawn(async move {
            server.await.unwrap();
        });

        open::that(auth_url.to_string()).unwrap();

        let AuthCodeResponse { code, csrf } = match receiver.recv().await {
            Some(params) => params,
            None => {
                dbg!("Could not get auth code response");
                tx.send(()).unwrap();
                panic!("No auth code");
            }
        };
        tx.send(()).unwrap();

        match csrf.secret() == csrf_token.secret() {
            true => println!("csrf is fine"),
            false => {
                println!("csrf is not fine");
                return Err("csrf is failed".to_string());
            }
        }

        match client
            .exchange_code(code)
            .set_pkce_verifier(pkce_verifier)
            .request_async(async_http_client)
            .await
        {
            Ok(tokens) => {
                let t = tokens;
                AccessTokenStorage::new()
                    .set(t.access_token().secret())
                    .unwrap();

                dbg!(t);
                Ok("got token".to_string())
            }
            Err(_) => Err("no token".to_string()),
        }
    }
}

pub async fn route_handler(
    query: Query<AuthCodeResponse>,
    sender: mpsc::Sender<AuthCodeResponse>,
) -> Result<Html<&'static str>, Html<&'static str>> {
    match sender.send(query.0).await {
        Ok(_) => Ok(Html(
            r#"
            <meta name="color-scheme" content="dark" />
            <p style="font-size: 9rem;font-family: sans-serif">Now u can close this page</p>"#,
        )),
        Err(err) => {
            dbg!(err);
            Err(Html(
                r#"
            <meta name="color-scheme" content="dark" />
            <script>location.reload()</script>
            "#,
            ))
        }
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
pub async fn discord_auth() -> Result<String, String> {
    DiscordProvider::new().oauth(None).await
}

#[command]
pub async fn discord_authenticated() -> bool {
    match AccessTokenStorage::new().get() {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[command]
pub async fn discord_identity() -> Option<Identity> {
    let storage = AccessTokenStorage::new();
    match storage.get().ok() {
        Some(token) => Some(DiscordProvider::identity(token).await),
        None => None,
    }
}

#[command]
pub fn discord_logout() {
    dbg!("logout");
    AccessTokenStorage::new().delete().unwrap()
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
    const KEY_NAME: &'static str = "discord_access_token";
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