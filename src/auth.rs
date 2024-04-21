use std::{fmt, future::Future, pin::Pin, sync::Mutex, time::Duration};

use chrono::Utc;
use const_format::formatcp;
use oauth2::reqwest::async_http_client;
use oauth2::{
    basic::{BasicClient, BasicErrorResponse, BasicErrorResponseType},
    AccessToken, Scope,
};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    RefreshToken, StandardErrorResponse, TokenResponse, TokenUrl,
};
use thiserror::Error;

use crate::BASE_URL;

pub const AUTH_URL: &str = formatcp!("{BASE_URL}/oauth2/authorize");
pub const TOKEN_URL: &str = formatcp!("{BASE_URL}/oauth2/token");

/// Error type for Authorization methods
#[derive(Error, Debug, PartialEq)]
pub enum AuthError {
    #[error("Unknown error occurred")]
    UnknownError(String),
    #[error("The network request timed out")]
    NetworkTimeout,
    #[error("Received invalid response from API")]
    StateMismatch(String),
    #[error("No auth present. Please run create_auth_code()")]
    AuthNotPresent,
    #[error("Token not found")]
    TokenNotPresent,
    #[error("OAuth Error: `{0:?}`")]
    OAuthError(StandardErrorResponse<BasicErrorResponseType>),
    #[error("Request Token Error: `{0:?}`")]
    RequestTokenError(BasicErrorResponse),
    #[error("Refresh token expired")]
    RefreshTokenExpiredError,
}

#[derive(Debug)]
pub struct Code(pub String);
#[derive(Debug)]
pub struct State(pub String);

#[derive(Clone)]
pub struct AppToken(String);

impl AppToken {
    /// Get the secret token
    /// This could compromise safety. Use carefully
    pub fn secret(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for AppToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppToken(_) => f.debug_tuple("AppToken").field(&"[Redacted]").finish(),
        }
    }
}

type Callback = Box<
    dyn Fn(
            reqwest::Url,
            State,
        ) -> Pin<
            Box<
                dyn Future<Output = Result<(Code, State), Box<dyn std::error::Error>>>
                    + Send
                    + 'static,
            >,
        > + Send
        + 'static,
>;

/// Manages OAuth 2 and all client and app tokens, ids, secrets, etc
pub struct Auth {
    client: BasicClient,
    client_id: ClientId,
    client_secret: ClientSecret,
    app_token: AppToken,
    access_token: Mutex<Option<AccessToken>>,
    refresh_token: Mutex<Option<RefreshToken>>,
    // time in utc seconds when access token expires
    expires_at: Mutex<Option<u64>>,
    // time in utc seconds when refresh token expires
    refresh_expires_at: Mutex<Option<u64>>,
    scopes: Mutex<Vec<Scope>>,
    callback: tokio::sync::Mutex<Callback>,
}

impl fmt::Debug for Auth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Auth {
            client,
            client_id,
            client_secret,
            app_token,
            access_token,
            refresh_token,
            expires_at,
            refresh_expires_at,
            scopes,
            ..
        } = self;

        f.debug_struct("Token")
            .field("client", &client)
            .field("client_id", &client_id)
            .field("client_secret", &client_secret)
            .field("app_token", &app_token)
            .field("access_token", &access_token)
            .field("refresh_token", &refresh_token)
            .field("expires_at", &expires_at)
            .field("refresh_expires_at", &refresh_expires_at)
            .field("scopes", &scopes)
            .field("callback", &"unknown")
            .finish()
    }
}

impl Auth {
    pub fn new(
        client_id: &str,
        client_secret: &str,
        app_token: &str,
        redirect_uri: &str,
    ) -> Result<Self, TokenError> {
        let client = BasicClient::new(
            ClientId::new(client_id.to_owned()),
            Some(ClientSecret::new(client_secret.to_owned())),
            AuthUrl::new(AUTH_URL.to_owned()).unwrap(),
            Some(TokenUrl::new(TOKEN_URL.to_owned()).unwrap()),
        )
        .set_redirect_uri(RedirectUrl::new(redirect_uri.to_owned())?);

        let slf = Self {
            client,
            client_id: ClientId::new(client_id.to_owned()),
            client_secret: ClientSecret::new(client_secret.to_owned()),
            app_token: AppToken(app_token.to_owned()),
            access_token: Mutex::new(None),
            refresh_token: Mutex::new(None),
            expires_at: Mutex::new(None),
            scopes: Mutex::new(Vec::new()),
            refresh_expires_at: Mutex::new(None),

            callback: tokio::sync::Mutex::new(Box::new(|_, _| {
                unimplemented!("oauth2 callback not implemented")
            })),
        };

        Ok(slf)
    }

    pub fn app_token(&self) -> AppToken {
        self.app_token.clone()
    }

    pub fn client_id(&self) -> ClientId {
        self.client_id.clone()
    }

    pub fn client_secret(&self) -> ClientSecret {
        self.client_secret.clone()
    }

    /// get access token
    pub fn access_token(&self) -> Option<AccessToken> {
        self.access_token.lock().unwrap().clone()
    }

    /// get refresh token
    pub fn refresh_token(&self) -> Option<RefreshToken> {
        self.refresh_token.lock().unwrap().clone()
    }

    pub fn set_refresh_token(&self, token: Option<&str>) {
        let mut lock = self.refresh_token.lock().unwrap();
        *lock = token.map(|t| RefreshToken::new(t.to_owned()));
    }

    pub fn set_access_token(&self, token: &str) {
        let mut lock = self.access_token.lock().unwrap();
        *lock = Some(AccessToken::new(token.to_owned()));
    }

    /// Updates the access token expiry time. Duration is how long after NOW it will after in
    pub fn set_expires_in(&self, duration: Option<Duration>) {
        let mut lock = self.expires_at.lock().unwrap();
        *lock = duration.map(|d| Utc::now().timestamp() as u64 + d.as_secs());
    }

    /// Updates the access token expiry time. Duration is how long after NOW it will after in
    pub fn set_refresh_expires_in(&self, duration: Option<Duration>) {
        let mut lock = self.refresh_expires_at.lock().unwrap();
        *lock = duration.map(|d| Utc::now().timestamp() as u64 + d.as_secs());
    }

    pub fn add_scope(&self, scope: &str) {
        let mut lock = self.scopes.lock().unwrap();
        lock.push(Scope::new(scope.to_owned()));
    }

    /// Set the callback used when running regenerate.
    /// this passes in the user id that was passed into the regenerate function
    /// You may successfully from this function ONLY if the user id is correct
    pub async fn set_callback<
        F: Fn(reqwest::Url, State) -> Fut + Send + 'static,
        Fut: Future<Output = Result<(Code, State), Box<dyn std::error::Error>>> + 'static + Send,
    >(
        &mut self,
        f: F,
    ) {
        let mut lock = self.callback.lock().await;
        *lock = Box::new(move |url, state| Box::pin(f(url, state)));
    }

    /// Is the current state of this access token valid?
    ///
    /// This checks that the access token exists and its expiry is still valid.
    ///
    /// Unless you're doing manual setup, this will correctly represent whether it's valid or not
    ///
    /// (Manual setup is, for example, manually setting the refresh token and running refresh on it)
    pub fn is_access_valid(&self) -> bool {
        let has_access_token = self.access_token.lock().unwrap().is_some();

        let is_active = self
            .expires_at
            .lock()
            .unwrap()
            .is_some_and(|t| (Utc::now().timestamp() as u64) < t);

        has_access_token && is_active
    }

    /// Is the current state of this refresh token valid?
    ///
    /// This checks that the refresh token exists and its expiry is still valid.
    ///
    /// Unless you're doing manual setup, this will correctly represent whether it's valid or not
    ///
    /// (Manual setup is, for example, manually setting the refresh token and running refresh on it)
    pub fn is_refresh_valid(&self) -> bool {
        let has_refresh_access_token = self.refresh_token.lock().unwrap().is_some();

        let is_refresh_active = self
            .refresh_expires_at
            .lock()
            .unwrap()
            .is_some_and(|t| (Utc::now().timestamp() as u64) < t);

        has_refresh_access_token && is_refresh_active
    }

    /// Automatically regnerate refresh token if possible
    ///
    /// If this fails, you should regenerate the token again
    pub async fn try_refresh(&self) -> Result<(), TokenError> {
        let refresh_valid = self.is_refresh_valid();
        let access_valid = self.is_access_valid();

        if !access_valid && refresh_valid {
            self.refresh().await?;
        }

        // nothing is valid, and can't refresh, report to user
        if !access_valid && !refresh_valid {
            Err(TokenError::RefreshExpired)
        } else {
            Ok(())
        }
    }

    /// time in utc seconds when access token expires
    pub fn expires_at(&self) -> Option<u64> {
        *self.expires_at.lock().unwrap()
    }

    /// time in utc seconds when refresh token expires
    pub fn refresh_expires_at(&self) -> Option<u64> {
        *self.expires_at.lock().unwrap()
    }

    /// exchange refresh token for new access token
    pub async fn refresh(&self) -> Result<(), TokenError> {
        let token = self.refresh_token.lock().unwrap().clone();
        if let Some(refresh_token) = token {
            let token = self
                .client
                .exchange_refresh_token(&refresh_token)
                .request_async(async_http_client)
                .await
                .map_err(|e| TokenError::OAuth2(e.to_string()))?;

            let mut lock = self.access_token.lock().unwrap();
            *lock = Some(token.access_token().clone());

            let mut lock = self.refresh_token.lock().unwrap();
            *lock = token.refresh_token().cloned();

            let mut lock = self.expires_at.lock().unwrap();
            *lock = token
                .expires_in()
                .map(|d| (Utc::now().timestamp() as u64) + d.as_secs());

            Ok(())
        } else {
            Err(TokenError::Refresh)
        }
    }

    /// regenerate fresh access and refresh tokens
    pub async fn regenerate(&self) -> Result<(), TokenError> {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let scopes = self.scopes.lock().unwrap().clone();

        let (auth_url, state) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scopes(scopes.into_iter())
            .set_pkce_challenge(pkce_challenge)
            .url();

        // the state that gets passed into the callback is the only one that is valid to return
        // the code back to the caller. make sure they match, or regenerate() will return an error
        let (res_code, res_state) = {
            let callback = self.callback.lock().await;
            match callback(auth_url, State(state.secret().clone())).await {
                Ok(v) => v,
                Err(e) => return Err(TokenError::Callback(e.to_string())),
            }
        };

        // ensure state is correct
        if state.secret() != &res_state.0 {
            return Err(TokenError::StateMismatch);
        }

        // now get access token
        let Ok(token) = self
            .client
            .exchange_code(AuthorizationCode::new(res_code.0))
            .set_pkce_verifier(pkce_verifier)
            .request_async(async_http_client)
            .await
        else {
            return Err(TokenError::Access);
        };

        let mut expires_at = self.expires_at.lock().unwrap();
        *expires_at = token
            .expires_in()
            .map(|d| Utc::now().timestamp() as u64 + d.as_secs());

        // how many days the refresh token is valid for; docs say "1 month"
        const DAYS: u64 = 31;
        let mut refresh_expires_at = self.refresh_expires_at.lock().unwrap();
        *refresh_expires_at = Some(Utc::now().timestamp() as u64 + (DAYS * 24 * 60 * 60));

        let mut access_token = self.access_token.lock().unwrap();
        *access_token = Some(token.access_token().clone());

        let mut refresh_token = self.refresh_token.lock().unwrap();
        *refresh_token = token.refresh_token().cloned();

        Ok(())
    }
}

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum TokenError {
    #[error("callback failed")]
    Callback(String),
    #[error("refresh token is already expired")]
    RefreshExpired,
    #[error("{0}")]
    OAuth2(String),
    #[error("failed to refresh token")]
    Refresh,
    #[error("failed to generate access token")]
    Access,
    #[error("failed to parse uri: {0}")]
    Parse(#[from] ::oauth2::url::ParseError),
    #[error("state verification failed")]
    StateMismatch,
}