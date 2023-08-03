use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use crate::oauth::AuthCodeResponse;
use axum::{extract::Query, response::Html, routing::get, Router};
use divi::league::League;
use keyring::Entry;
use oauth2::{
    basic::BasicClient, AccessToken, AuthUrl, ClientId, CsrfToken, PkceCodeChallenge, RedirectUrl,
    Scope, TokenUrl,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::{command, AppHandle, Window};
use tokio::sync::mpsc;

pub const API_URL: &'static str = "https://api.pathofexile.com";
const PROVIDER_LABEL: &'static str = "poe";
const CLIENT_ID: &'static str = "divicards";
const AUTH_URL: &'static str = "https://www.pathofexile.com/oauth/authorize";
const TOKEN_URL: &'static str = "https://www.pathofexile.com/oauth/token";

#[command]
pub async fn poe_auth(app_handle: AppHandle, window: Window) -> Result<String, String> {
    let (sender, mut receiver) = mpsc::channel::<AuthCodeResponse>(1);
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 50151);
    let redirect_uri =
        RedirectUrl::new(format!("http://localhost:{}/callback", addr.port())).unwrap();

    let client = BasicClient::new(
        ClientId::new(CLIENT_ID.into()),
        None,
        AuthUrl::new(AUTH_URL.into()).unwrap(),
        TokenUrl::new(TOKEN_URL.into()).ok(),
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
    dbg!(auth_url.to_string());
    let auth_url_string = auth_url.to_string();
    window.emit("auth-url", auth_url_string).unwrap();

    let AuthCodeResponse { code, csrf } = match receiver.recv().await {
        Some(params) => params,
        None => {
            dbg!("Could not get auth code response");
            tx.send(()).unwrap();
            panic!("No auth code");
        }
    };
    tx.send(()).unwrap();

    if csrf.secret() != csrf_token.secret() {
        return Err(String::from("csrf is failed"));
    }

    let TokenResponseData {
        username,
        access_token,
        ..
    } = fetch_token(
        code.secret(),
        pkce_verifier.secret(),
        &redirect_uri,
        app_handle.config().package.version.clone().unwrap(),
    )
    .await
    .unwrap();

    AccessTokenStorage::new()
        .set(&access_token.secret())
        .unwrap();

    Ok(username)
}

async fn fetch_token(
    code: &str,
    pkce_verifier: &str,
    redirect_uri: &str,
    version: String,
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
        .post(TOKEN_URL)
        .body(payload)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header(
            "User-Agent",
            format!("OAuth divicards/{} (contact: poeshonya3@gmail.com)", {
                version
            }),
        )
        .send()
        .await
        .unwrap()
        .json::<TokenResponseData>()
        .await
        .unwrap())
}

// #[command]
// pub async fn poe_auth(app_handle: AppHandle) -> Result<String, String> {
//     PoeProvider::new()
//         .oauth(app_handle.config().package.version.clone())
//         .await
// }

#[command]
pub fn poe_logout() {
    dbg!("logout");

    match AccessTokenStorage::new().delete() {
        Ok(_) => {}
        Err(_) => {}
    }
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TokenResponseData {
    access_token: AccessToken,
    expires_in: usize,
    scope: String,
    sub: String,
    token_type: String,
    username: String,
}

// #[async_trait]
// impl OAuthProvider for PoeProvider {
//     const PROVIDER_LABEL: &'static str = "poe";
//     const CLIENT_ID: &'static str = "divicards";
//     const AUTH_URL: &'static str = "https://www.pathofexile.com/oauth/authorize";
//     const TOKEN_URL: &'static str = "https://www.pathofexile.com/oauth/token";

//     async fn oauth(
//         &self,
//         version: Option<String>,
//         tx: tokio::sync::oneshot::Sender<String>,
//     ) -> Result<String, String> {
//         let (sender, mut receiver) = mpsc::channel::<AuthCodeResponse>(1);
//         let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 50151);
//         let redirect_uri =
//             RedirectUrl::new(format!("http://localhost:{}/callback", addr.port())).unwrap();

//         let client = BasicClient::new(
//             PoeProvider::client_id(),
//             None,
//             PoeProvider::auth_url().unwrap(),
//             PoeProvider::token_url().ok(),
//         )
//         .set_redirect_uri(redirect_uri.clone());

//         let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

//         let (auth_url, csrf_token) = client
//             .authorize_url(CsrfToken::new_random)
//             .add_scope(Scope::new("account:stashes".to_string()))
//             .set_pkce_challenge(pkce_challenge)
//             .url();

//         let app = Router::new().route(
//             "/callback",
//             get(move |query: Query<AuthCodeResponse>| route_handler(query, sender)),
//         );

//         let (tx, rx) = tokio::sync::oneshot::channel::<()>();
//         let server = axum::Server::bind(&addr)
//             .serve(app.into_make_service())
//             .with_graceful_shutdown(async {
//                 rx.await.ok();
//                 println!("shutdown");
//             });

//         tokio::spawn(async move {
//             server.await.unwrap();
//         });

//         open::that(auth_url.to_string()).unwrap();
//         dbg!(auth_url.to_string());

//         let AuthCodeResponse { code, csrf } = match receiver.recv().await {
//             Some(params) => params,
//             None => {
//                 dbg!("Could not get auth code response");
//                 tx.send(()).unwrap();
//                 panic!("No auth code");
//             }
//         };
//         tx.send(()).unwrap();

//         if csrf.secret() != csrf_token.secret() {
//             return Err(String::from("csrf is failed"));
//         }

//         let TokenResponseData {
//             username,
//             access_token,
//             ..
//         } = Self::fetch_token(
//             code.secret(),
//             pkce_verifier.secret(),
//             &redirect_uri,
//             version.unwrap(),
//         )
//         .await
//         .unwrap();

//         AccessTokenStorage::new()
//             .set(&access_token.secret())
//             .unwrap();

//         Ok(username)
//     }
// }

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
