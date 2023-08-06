use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use crate::event::Event;

use super::{AccessTokenStorage, Persist, AUTH_URL, CLIENT_ID, TOKEN_URL};
use axum::{extract::Query, response::Html, routing::get, Router};
use oauth2::{
    basic::BasicClient, AccessToken, AuthUrl, AuthorizationCode, ClientId, CsrfToken,
    PkceCodeChallenge, RedirectUrl, Scope, TokenUrl,
};
use serde::{Deserialize, Serialize};
use tauri::{command, AppHandle, Window};
use tokio::sync::mpsc;
use tracing::debug;

#[command]
pub async fn poe_auth(app_handle: AppHandle, window: Window) -> Result<String, String> {
    let (sender, mut receiver) = mpsc::channel::<AuthResponse>(1);
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
        get(move |query: Query<AuthResponse>| route_handler(query, sender)),
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

    Event::AuthUrl {
        url: auth_url.to_string(),
    }
    .emit(&window);

    let res = receiver.recv().await;
    tx.send(()).unwrap();

    let Some(response) = res else {
        return Err(String::from("Error during authorization. Try again."));
    };

    match response {
        AuthResponse::Code { code, csrf } => {
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
        AuthResponse::Error {
            error,
            error_description,
        } => Err(format!("Error: {error}. Description: {error_description}")),
    }
}

#[command]
pub fn poe_logout() {
    debug!("logout");

    match AccessTokenStorage::new().delete() {
        Ok(_) => {}
        Err(_) => {}
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuthResponse {
    Code {
        code: AuthorizationCode,
        #[serde(alias = "state")]
        csrf: CsrfToken,
    },
    Error {
        error: String,
        error_description: String,
    },
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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TokenResponseData {
    access_token: AccessToken,
    expires_in: usize,
    scope: String,
    sub: String,
    token_type: String,
    username: String,
}

pub async fn route_handler(
    query: Query<AuthResponse>,
    sender: mpsc::Sender<AuthResponse>,
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
