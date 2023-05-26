use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use crate::oauth::{AuthCodeResponse, OAuthProvider};
use axum::{async_trait, extract::Query, response::Html, routing::get, Router};
use divi::League;
use keyring::Entry;
use oauth2::{basic::BasicClient, AccessToken, CsrfToken, PkceCodeChallenge, RedirectUrl, Scope};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::command;
use tokio::sync::mpsc;

#[command]
pub async fn poe_auth() -> Result<String, String> {
    PoeProvider::new().oauth().await
}

#[command]
pub async fn poe_authenticated() -> bool {
    match AccessTokenStorage::new().get() {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[command]
pub fn poe_logout() {
    dbg!("logout");
    AccessTokenStorage::new().delete().unwrap()
}

#[command]
pub async fn stashes(league: League) -> Value {
    let val = PoeProvider::stashes(league).await;
    // dbg!(&val);
    val
}

#[command]
pub async fn stash(league: League, stash_id: String, substash_id: Option<String>) -> Value {
    let val = PoeProvider::stash(league, stash_id, substash_id).await;
    // dbg!(&val);
    val
}

pub const API_URL: &'static str = "https://api.pathofexile.com";

#[derive(Default)]
pub struct PoeProvider;

impl PoeProvider {
    pub fn new() -> PoeProvider {
        PoeProvider::default()
    }

    pub fn access_token_label() -> String {
        format!("{}_access_token", { Self::PROVIDER_LABEL })
    }

    async fn stash(league: League, stash_id: String, substash_id: Option<String>) -> Value {
        let url = match substash_id {
            Some(substash_id) => {
                format!("{}/stash/{}/{}/{}", API_URL, league, stash_id, substash_id)
            }
            None => format!("{}/stash/{}/{}", API_URL, league, stash_id),
        };

        dbg!(&url);
        Client::new()
            .get(url)
            .header(
                "Authorization",
                format!("Bearer {}", { AccessTokenStorage::new().get().unwrap() }),
            )
            .header(
                "User-Agent",
                "OAuth divicards/0.1.8 (contact: poeshonya3@gmail.com)",
            )
            .send()
            .await
            .unwrap()
            .json::<Value>()
            .await
            .unwrap()
    }

    async fn stashes(league: League) -> Value {
        Client::new()
            .get(format!("{}/stash/{}", API_URL, league))
            .header(
                "Authorization",
                format!("Bearer {}", { AccessTokenStorage::new().get().unwrap() }),
            )
            .header(
                "User-Agent",
                "OAuth divicards/0.1.8 (contact: poeshonya3@gmail.com)",
            )
            .send()
            .await
            .unwrap()
            .json::<Value>()
            .await
            .unwrap()
    }

    async fn fetch_token(
        code: &str,
        pkce_verifier: &str,
        redirect_uri: &str,
    ) -> Result<TokenResponseData, String> {
        let payload = url::form_urlencoded::Serializer::new(String::new())
            .append_pair("client_id", "divicards")
            .append_pair("grant_type", "authorization_code")
            .append_pair("code", code)
            .append_pair("redirect_uri", redirect_uri)
            .append_pair("scope", "account:stashes")
            .append_pair("code_verifier", pkce_verifier)
            .finish();

        dbg!(&payload);

        let client = reqwest::Client::new();
        Ok(client
            .post(PoeProvider::TOKEN_URL)
            .body(payload)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header(
                "User-Agent",
                "OAuth divicards/0.1.8 (contact: poeshonya3@gmail.com)",
            )
            .send()
            .await
            .unwrap()
            .json::<TokenResponseData>()
            .await
            .unwrap())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TokenResponseData {
    access_token: AccessToken,
    expires_in: usize,
    scope: String,
    sub: String,
    token_type: String,
    username: String,
}

#[async_trait]
impl OAuthProvider for PoeProvider {
    const PROVIDER_LABEL: &'static str = "poe";
    const CLIENT_ID: &'static str = "divicards";
    const AUTH_URL: &'static str = "https://www.pathofexile.com/oauth/authorize";
    const TOKEN_URL: &'static str = "https://www.pathofexile.com/oauth/token";

    async fn oauth(&self) -> Result<String, String> {
        let (sender, mut receiver) = mpsc::channel::<AuthCodeResponse>(1);
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 50151);
        let redirect_uri =
            RedirectUrl::new(format!("http://localhost:{}/callback", addr.port())).unwrap();

        let client = BasicClient::new(
            PoeProvider::client_id(),
            None,
            PoeProvider::auth_url().unwrap(),
            PoeProvider::token_url().ok(),
        )
        .set_redirect_uri(redirect_uri.clone());

        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let (auth_url, csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("account:stashes".to_string()))
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
        };

        let TokenResponseData {
            username,
            access_token,
            ..
        } = Self::fetch_token(code.secret(), pkce_verifier.secret(), &redirect_uri)
            .await
            .unwrap();

        AccessTokenStorage::new()
            .set(&access_token.secret())
            .unwrap();

        Ok(username)
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
