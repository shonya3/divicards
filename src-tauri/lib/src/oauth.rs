use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::{extract::Query, response::Html};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, url::ParseError, AuthUrl, AuthorizationCode,
    ClientId, CsrfToken, PkceCodeChallenge, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};

use axum::{routing::get, Router};
use tokio::sync::mpsc;

pub const CLIENT_ID: &'static str = "1105146245221191872";
pub const AUTH_URL: &'static str = "https://discord.com/oauth2/authorize";
pub const TOKEN_URL: &'static str = "https://discord.com/api/oauth2/token";

pub trait OAuthProvider {
    const PROVIDER_LABEL: &'static str;
    const CLIENT_ID: &'static str;
    const AUTH_URL: &'static str;
    const TOKEN_URL: &'static str;

    fn client_id() -> ClientId {
        ClientId::new(Self::CLIENT_ID.to_string())
    }

    fn auth_url() -> Result<AuthUrl, ParseError> {
        AuthUrl::new(Self::AUTH_URL.to_string())
    }

    fn token_url() -> Result<TokenUrl, ParseError> {
        TokenUrl::new(Self::TOKEN_URL.to_string())
    }
}

pub trait Persistent {
    fn access_token() -> Option<String>;
    fn set_access_token(token: String) -> Result<(), keyring::Error>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthCodeResponse {
    pub code: AuthorizationCode,
    #[serde(alias = "state")]
    pub csrf: CsrfToken,
}

pub async fn route_handler(
    query: Query<AuthCodeResponse>,
    sender: mpsc::Sender<AuthCodeResponse>,
) -> Result<Html<&'static str>, Html<&'static str>> {
    match sender.send(query.0).await {
        Ok(_) => {
            println!("We are fine");
            Ok(Html("Now u can close this page"))
        }
        Err(err) => {
            dbg!(err);
            Err(Html("Auth is failed"))
        }
    }
}

pub async fn auth<P: OAuthProvider + Persistent>(_: P) -> String {
    let (sender, mut receiver) = mpsc::channel::<AuthCodeResponse>(1);
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let redirect_url =
        RedirectUrl::new(format!("http://localhost:{}/callback", addr.port())).unwrap();

    let client = BasicClient::new(
        P::client_id(),
        None,
        P::auth_url().unwrap(),
        P::token_url().ok(),
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
        Some(params) => {
            tx.send(()).unwrap();
            params
        }
        None => {
            panic!("No auth code");
        }
    };

    match csrf.secret() == csrf_token.secret() {
        true => println!("csrf is fine"),
        false => {
            println!("csrf is not fine");
            return "csrf is failed".to_string();
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
            P::set_access_token(t.access_token().secret().to_string()).unwrap();
            dbg!(t);
            "got token".to_string()
        }
        Err(_) => "no token".to_string(),
    }
}
