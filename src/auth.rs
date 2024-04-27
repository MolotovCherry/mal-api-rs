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
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{BASE_URL, RUNTIME};

const AUTH_URL: &str = formatcp!("{BASE_URL}/oauth2/authorize");
const TOKEN_URL: &str = formatcp!("{BASE_URL}/oauth2/token");

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

type Callback = Box<
    dyn Fn(
            reqwest::Url,
            CsrfToken,
        ) -> Pin<
            Box<
                dyn Future<
                        Output = Result<(AuthorizationCode, CsrfToken), Box<dyn std::error::Error>>,
                    > + Send
                    + 'static,
            >,
        > + Send
        + 'static,
>;

/// A (de)serializable version of [Auth]. Only serializes the access/refresh tokens and their expiry.
/// This can be converted back to [Auth] if you provide your id, secret, and redirect url.
///
/// Callbacks are not saved or converted back. You must set it again manually.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthTokens {
    pub access_token: AccessToken,
    pub refresh_token: RefreshToken,
    pub expires_at: u64,
    pub refresh_expires_at: u64,
}

impl Default for AuthTokens {
    fn default() -> Self {
        Self {
            access_token: AccessToken::new(String::new()),
            refresh_token: RefreshToken::new(String::new()),
            expires_at: 0,
            refresh_expires_at: 0,
        }
    }
}

impl AuthTokens {
    pub fn to_auth(
        &self,
        client_id: ClientId,
        client_secret: ClientSecret,
        redirect_url: RedirectUrl,
    ) -> Auth {
        let auth = Auth::new(client_id, client_secret, redirect_url);

        auth.set_access_token_unchecked(self.access_token.clone());
        auth.set_refresh_token_unchecked(self.refresh_token.clone());
        auth.set_expires_at_unchecked(self.expires_at);
        auth.set_refresh_expires_at_unchecked(self.refresh_expires_at);

        auth
    }

    pub fn into_auth(
        self,
        client_id: ClientId,
        client_secret: ClientSecret,
        redirect_url: RedirectUrl,
    ) -> Auth {
        let auth = Auth::new(client_id, client_secret, redirect_url);

        auth.set_access_token_unchecked(self.access_token);
        auth.set_refresh_token_unchecked(self.refresh_token);
        auth.set_expires_at_unchecked(self.expires_at);
        auth.set_refresh_expires_at_unchecked(self.refresh_expires_at);

        auth
    }
}

/// Manages oauth2 and client id, client secret
pub struct Auth {
    client: BasicClient,
    client_id: ClientId,
    client_secret: ClientSecret,
    access_token: Mutex<AccessToken>,
    refresh_token: Mutex<RefreshToken>,
    // time in utc seconds when access token expires
    expires_at: Mutex<u64>,
    // time in utc seconds when refresh token expires
    refresh_expires_at: Mutex<u64>,
    scopes: Mutex<Vec<Scope>>,
    callback: tokio::sync::Mutex<Callback>,
}

impl fmt::Debug for Auth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Auth {
            client,
            client_id,
            client_secret,
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
        client_id: ClientId,
        client_secret: ClientSecret,
        redirect_uri: RedirectUrl,
    ) -> Self {
        let client = BasicClient::new(
            client_id.clone(),
            Some(client_secret.clone()),
            AuthUrl::new(AUTH_URL.to_owned()).unwrap(),
            Some(TokenUrl::new(TOKEN_URL.to_owned()).unwrap()),
        )
        .set_redirect_uri(redirect_uri);

        Self {
            client,
            client_id,
            client_secret,
            access_token: Mutex::new(AccessToken::new(String::new())),
            refresh_token: Mutex::new(RefreshToken::new(String::new())),
            expires_at: Mutex::new(0),
            refresh_expires_at: Mutex::new(0),
            scopes: Mutex::new(Vec::new()),

            callback: tokio::sync::Mutex::new(Box::new(|_, _| {
                unimplemented!("oauth2 callback not implemented")
            })),
        }
    }

    /// Return client tokens to save user creds that can be serialized/deserialized.
    /// serializes access/refresh tokens, and their expiry
    /// Does not serialize client_id, client_secret, scopes, or callback
    pub fn to_tokens(&self) -> AuthTokens {
        let at = self.access_token();
        let rt = self.refresh_token();
        let ea = self.expires_at();
        let rea = self.refresh_expires_at();

        AuthTokens {
            access_token: at,
            refresh_token: rt,
            expires_at: ea,
            refresh_expires_at: rea,
        }
    }

    /// Get the client id.
    pub fn client_id(&self) -> ClientId {
        self.client_id.clone()
    }

    /// Get the client secret.
    pub fn client_secret(&self) -> ClientSecret {
        self.client_secret.clone()
    }

    /// Get the access token.
    pub fn access_token(&self) -> AccessToken {
        self.access_token.lock().unwrap().clone()
    }

    /// Get the refresh token.
    pub fn refresh_token(&self) -> RefreshToken {
        self.refresh_token.lock().unwrap().clone()
    }

    /// Manually set the refresh token. This is handled automatically by [`Self::refresh()`], [`Self::refresh_blocking()`], [`Self::regenerate()`], and [`Self::regenerate_blocking()`].
    ///
    /// This method is safe in terms of no UB, however it is unchecked because it is possible to cause inconsistent state.
    ///
    /// Caller agrees to also set the correct refresh token expiry time as well.
    pub fn set_refresh_token_unchecked(&self, token: RefreshToken) {
        let mut lock = self.refresh_token.lock().unwrap();
        *lock = token;
    }

    /// Manually set the access token. This is handled automatically by [`Self::refresh()`], [`Self::refresh_blocking()`], [`Self::regenerate()`], and [`Self::regenerate_blocking()`].
    ///
    /// This method is safe in terms of no UB, however it is unchecked because it is possible to cause inconsistent state.
    ///
    /// Caller agrees to also set the correct access token expiry time as well.
    pub fn set_access_token_unchecked(&self, token: AccessToken) {
        let mut lock = self.access_token.lock().unwrap();
        *lock = token;
    }

    /// Updates the access token expiry time. Expiry is utc seconds
    /// This is handled automatically by [`Self::refresh()`], [`Self::refresh_blocking()`], [`Self::regenerate()`], and [`Self::regenerate_blocking()`].
    ///
    /// This method is safe in terms of no UB, however it is unchecked because it is possible to cause inconsistent state.
    ///
    /// Caller agrees to also set the correct access token as well.
    pub fn set_expires_at_unchecked(&self, expiry: u64) {
        let mut lock = self.expires_at.lock().unwrap();
        *lock = expiry;
    }

    /// Updates the access token expiry time. Duration is how long from NOW it will after in.
    /// This is handled automatically by [`Self::refresh()`], [`Self::refresh_blocking()`], [`Self::regenerate()`], and [`Self::regenerate_blocking()`].
    ///
    /// This method is safe in terms of no UB, however it is unchecked because it is possible to cause inconsistent state.
    ///
    /// Caller agrees to also set the correct access token as well.
    pub fn set_expires_in_unchecked(&self, duration: Duration) {
        let mut lock = self.expires_at.lock().unwrap();
        *lock = Utc::now().timestamp() as u64 + duration.as_secs();
    }

    /// Updates the refresh token expiry time. Duration is how long from NOW it will after in.
    /// This is handled automatically by [`Self::refresh()`], [`Self::refresh_blocking()`], [`Self::regenerate()`], and [`Self::regenerate_blocking()`].
    ///
    /// This method is safe in terms of no UB, however it is unchecked because it is possible to cause inconsistent state.
    ///
    /// Caller agrees to also set the correct refresh token as well.
    pub fn set_refresh_expires_in_unchecked(&self, duration: Duration) {
        let mut lock = self.refresh_expires_at.lock().unwrap();
        *lock = Utc::now().timestamp() as u64 + duration.as_secs();
    }

    /// Updates the refresh token expiry time. Expiry is utc seconds
    /// This is handled automatically by [`Self::refresh()`], [`Self::refresh_blocking()`], [`Self::regenerate()`], and [`Self::regenerate_blocking()`].
    ///
    /// This method is safe in terms of no UB, however it is unchecked because it is possible to cause inconsistent state.
    ///
    /// Caller agrees to also set the correct access token as well.
    pub fn set_refresh_expires_at_unchecked(&self, expiry: u64) {
        let mut lock = self.refresh_expires_at.lock().unwrap();
        *lock = expiry;
    }

    /// Add an oauth2 scope. Use this before you generate a new token.
    pub fn add_scope(&self, scope: Scope) {
        let mut lock = self.scopes.lock().unwrap();
        lock.push(scope);
    }

    /// Set the callback used when running [`Self::regenerate()`].
    /// This passes in a [`CsrfToken`] representing the client state this callback is looking for.
    /// You can know which client request is the correct client because the states match each other.
    ///
    /// You may return success from this function ONLY if the state is correct.
    /// You may want to make this timeout so [`Self::regenerate()`] doesn't block forever.
    pub async fn set_callback<
        F: Fn(reqwest::Url, CsrfToken) -> Fut + Send + 'static,
        Fut: Future<Output = Result<(AuthorizationCode, CsrfToken), Box<dyn std::error::Error>>>
            + 'static
            + Send,
    >(
        &self,
        f: F,
    ) {
        let mut lock = self.callback.lock().await;
        *lock = Box::new(move |url, state| Box::pin(f(url, state)));
    }

    /// Set the callback used when running [`Self::regenerate()`].
    /// This passes in a [`CsrfToken`] representing the client state this callback is looking for.
    /// You can know which client request is the correct client because the states match each other.
    ///
    /// You may return success from this function ONLY if the state is correct.
    /// You may want to make this timeout so [`Self::regenerate()`] doesn't block forever.
    pub fn set_callback_blocking<
        F: Fn(reqwest::Url, CsrfToken) -> Fut + Send + 'static,
        Fut: Future<Output = Result<(AuthorizationCode, CsrfToken), Box<dyn std::error::Error>>>
            + 'static
            + Send,
    >(
        &self,
        f: F,
    ) {
        RUNTIME.block_on(self.set_callback(f))
    }

    /// Is the current access token valid?
    ///
    /// This checks that the current access token's expiry is valid.
    ///
    /// Unless you're manually setting access tokens and expiry times (which cause an inconsistent state),
    /// this will correctly represent whether the token is valid or not.
    ///
    /// If you want to keep state consistent if you're manually setting those, then make sure to set both
    /// the access token and its expiry time.
    pub fn is_access_valid(&self) -> bool {
        (Utc::now().timestamp() as u64) < *self.expires_at.lock().unwrap()
    }

    /// Is the current refresh token valid?
    ///
    /// This checks that the current refresh token's expiry is valid.
    ///
    /// Unless you're manually setting refresh tokens and expiry times (which cause an inconsistent state),
    /// this will correctly represent whether the token is valid or not.
    ///
    /// If you want to keep state consistent if you're manually setting those, then make sure to set both
    /// the refresh token and its expiry time.
    pub fn is_refresh_valid(&self) -> bool {
        (Utc::now().timestamp() as u64) < *self.refresh_expires_at.lock().unwrap()
    }

    /// Automatically regnerate refresh token if possible
    ///
    /// If this fails, you should regenerate the token again.
    ///
    /// This is subject to an inconsistent state if you are manually setting
    /// access/refresh token and/or their expiry times.
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

    /// Automatically regnerate refresh token if possible
    ///
    /// If this fails, you should regenerate the token again.
    ///
    /// This is subject to an inconsistent state if you are manually setting
    /// access/refresh token and/or their expiry times.
    pub fn try_refresh_blocking(&self) -> Result<(), TokenError> {
        RUNTIME.block_on(self.try_refresh())
    }

    /// Time in utc seconds when access token expires.
    pub fn expires_at(&self) -> u64 {
        *self.expires_at.lock().unwrap()
    }

    /// Time in utc seconds when refresh token expires.
    pub fn refresh_expires_at(&self) -> u64 {
        *self.expires_at.lock().unwrap()
    }

    /// How many days refresh token is valid for
    const DAYS: u64 = 31;

    /// Exchange refresh token for new access token.
    pub async fn refresh(&self) -> Result<(), TokenError> {
        let token = self.refresh_token.lock().unwrap().clone();

        let token = self
            .client
            .exchange_refresh_token(&token)
            .request_async(async_http_client)
            .await
            .map_err(|e| TokenError::OAuth2(e.to_string()))?;

        self.set_expires_at_unchecked(
            Utc::now().timestamp() as u64 + token.expires_in().unwrap().as_secs(),
        );

        // how many days the refresh token is valid for; docs say "1 month"
        self.set_refresh_expires_at_unchecked(
            Utc::now().timestamp() as u64 + (Self::DAYS * 24 * 60 * 60),
        );

        self.set_access_token_unchecked(token.access_token().clone());

        self.set_refresh_token_unchecked(token.refresh_token().unwrap().clone());

        Ok(())
    }

    /// Exchange refresh token for new access token.
    pub fn refresh_blocking(&self) -> Result<(), TokenError> {
        RUNTIME.block_on(self.refresh())
    }

    /// Begin a new oauth2 access token generation procedure.
    ///
    /// Requires client to visit the url provided to the callback.
    /// Callback receives both url and state, and knows which client is correct
    /// by matching the passed in state with the state received from your server redirect url.
    ///
    /// This forever blocks if callback does not return. It is best that you set a timeout in the callback.
    pub async fn regenerate(&self) -> Result<(), TokenError> {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_plain();

        let scopes = self.scopes.lock().unwrap().clone();

        let (auth_url, state) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scopes(scopes.into_iter())
            .set_pkce_challenge(pkce_challenge)
            .url();

        // the state that gets passed into the callback is the only one that is valid to return
        // the code back to the caller. make sure they match, or regenerate() will return an error
        let (auth_code, client_state) = {
            let callback = self.callback.lock().await;
            match callback(auth_url, state.clone()).await {
                Ok(v) => v,
                Err(e) => return Err(TokenError::Callback(e.to_string())),
            }
        };

        // ensure state is correct
        if state.secret() != client_state.secret() {
            return Err(TokenError::StateMismatch);
        }

        // now get access token
        let Ok(token) = self
            .client
            .exchange_code(auth_code)
            .set_pkce_verifier(pkce_verifier)
            .request_async(async_http_client)
            .await
        else {
            return Err(TokenError::Access);
        };

        self.set_expires_at_unchecked(
            Utc::now().timestamp() as u64 + token.expires_in().unwrap().as_secs(),
        );

        // how many days the refresh token is valid for; docs say "1 month"
        self.set_refresh_expires_at_unchecked(
            Utc::now().timestamp() as u64 + (Self::DAYS * 24 * 60 * 60),
        );

        self.set_access_token_unchecked(token.access_token().clone());

        self.set_refresh_token_unchecked(token.refresh_token().unwrap().clone());

        Ok(())
    }

    /// Begin a new oauth2 access token generation procedure.
    ///
    /// Requires client to visit the url provided to the callback.
    /// Callback receives both url and state, and knows which client is correct
    /// by matching the passed in state with the state received from your server redirect url.
    ///
    /// This forever blocks if callback does not return. It is best that you set a timeout in the callback.
    pub fn regenerate_blocking(&self) -> Result<(), TokenError> {
        RUNTIME.block_on(self.regenerate())
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
