use axum::async_trait;
use oauth2::{url::ParseError, AuthUrl, AuthorizationCode, ClientId, CsrfToken, TokenUrl};
use serde::{Deserialize, Serialize};

#[async_trait]
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

    async fn oauth(self: &Self, app_version: Option<String>) -> Result<String, String>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthCodeResponse {
    pub code: AuthorizationCode,
    #[serde(alias = "state")]
    pub csrf: CsrfToken,
}
