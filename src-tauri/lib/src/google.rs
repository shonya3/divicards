use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use crate::oauth::{AuthCodeResponse, OAuthProvider};
use axum::{async_trait, extract::Query, response::Html, routing::get, Router};
use keyring::Entry;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, ClientSecret, CsrfToken, PkceCodeChallenge,
    RedirectUrl, Scope, TokenResponse,
};
use serde::{Deserialize, Serialize};
use tauri::command;
use tokio::sync::mpsc;

#[derive(Default)]
pub struct GoogleProvider;

impl GoogleProvider {
    pub fn new() -> GoogleProvider {
        GoogleProvider::default()
    }

    pub fn access_token_label() -> String {
        format!("{}_access_token", { Self::PROVIDER_LABEL })
    }

    pub async fn identity(acces_token: String) -> Identity {
        reqwest::Client::new()
            .get("https://www.googleapis.com/oauth2/v2/userinfo?alt=json")
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
impl OAuthProvider for GoogleProvider {
    const PROVIDER_LABEL: &'static str = "google";
    const CLIENT_ID: &'static str =
        "752206000922-1gpkcoplrjqpfgg8pr4sb4tnrlvauomp.apps.googleusercontent.com";
    const AUTH_URL: &'static str = "https://accounts.google.com/o/oauth2/auth";
    const TOKEN_URL: &'static str = "https://oauth2.googleapis.com/token";

    async fn oauth(&self) -> Result<String, String> {
        let (sender, mut receiver) = mpsc::channel::<AuthCodeResponse>(1);
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let redirect_url =
            RedirectUrl::new(format!("http://localhost:{}/callback", addr.port())).unwrap();

        let client = BasicClient::new(
            GoogleProvider::client_id(),
            Some(ClientSecret::new(
                "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".to_string(),
            )),
            GoogleProvider::auth_url().unwrap(),
            GoogleProvider::token_url().ok(),
        )
        .set_redirect_uri(redirect_url);

        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let (auth_url, csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("profile".to_string()))
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

        dbg!(&code);

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
            Err(err) => {
                dbg!(err);
                Err("no token".to_string())
            }
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
    given_name: Option<String>,
    name: Option<String>,
    id: String,
    picture: Option<String>,
    locale: Option<String>,
}

#[command]
pub async fn google_auth() -> Result<String, String> {
    // auth(GoogleProvider::default()).await
    let result = GoogleProvider::new().oauth().await;
    result
}

#[command]
pub async fn google_authenticated() -> bool {
    match AccessTokenStorage::new().get() {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[command]
pub async fn google_identity() -> Option<Identity> {
    let storage = AccessTokenStorage::new();
    match storage.get().ok() {
        Some(token) => Some(GoogleProvider::identity(token).await),
        None => None,
    }
}

#[command]
pub fn google_logout() {
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
