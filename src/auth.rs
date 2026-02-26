use std::{fmt, time::Duration};

use chrono::Utc;
use const_format::formatcp;
use oauth2::{
    basic::{BasicClient, BasicErrorResponse, BasicErrorResponseType},
    reqwest::Client,
    AccessToken, EndpointNotSet, EndpointSet, PkceCodeVerifier, Scope,
};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    RefreshToken, StandardErrorResponse, TokenResponse, TokenUrl,
};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::BASE_URL;

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

/// A (de)serializable version of [Auth]. Only serializes the access/refresh tokens and their expiry.
/// This can be converted back to [Auth] if you provide your id, secret, and redirect url.
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
    #[deprecated(since = "0.2.0", note = "use Auth::from_auth_tokens()")]
    pub fn to_auth(
        &self,
        client_id: ClientId,
        client_secret: ClientSecret,
        redirect_url: RedirectUrl,
    ) -> Auth {
        Auth::from_auth_tokens(self.clone(), client_id, client_secret, redirect_url)
    }

    #[deprecated(since = "0.2.0", note = "use Auth::from_auth_tokens()")]
    pub fn into_auth(
        self,
        client_id: ClientId,
        client_secret: ClientSecret,
        redirect_url: RedirectUrl,
    ) -> Auth {
        Auth::from_auth_tokens(self, client_id, client_secret, redirect_url)
    }
}

impl From<Auth> for AuthTokens {
    fn from(value: Auth) -> Self {
        value.to_tokens()
    }
}

impl From<&Auth> for AuthTokens {
    fn from(value: &Auth) -> Self {
        value.to_tokens()
    }
}

/// Manages oauth2 and client id, client secret
#[derive(Clone)]
pub struct Auth {
    client: BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>,
    client_id: ClientId,
    client_secret: ClientSecret,
    access_token: AccessToken,
    refresh_token: RefreshToken,
    // time in utc seconds when access token expires
    expires_at: u64,
    // time in utc seconds when refresh token expires
    refresh_expires_at: u64,
    scopes: Vec<Scope>,
    async_client: Client,
    // I'd use PckeCodeVerifier here, but it's !Clone
    pkce_code_verifier: Option<String>,
    client_req: Option<ClientAuthRequest>,
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
            .finish()
    }
}

impl Auth {
    pub fn new(
        client_id: ClientId,
        client_secret: ClientSecret,
        redirect_uri: RedirectUrl,
    ) -> Self {
        let client = BasicClient::new(client_id.clone())
            .set_client_secret(client_secret.clone())
            .set_auth_uri(AuthUrl::new(AUTH_URL.to_owned()).unwrap())
            .set_token_uri(TokenUrl::new(TOKEN_URL.to_owned()).unwrap())
            .set_redirect_uri(redirect_uri);

        Self {
            client,
            client_id,
            client_secret,
            access_token: AccessToken::new(String::new()),
            refresh_token: RefreshToken::new(String::new()),
            expires_at: 0,
            refresh_expires_at: 0,
            scopes: Vec::new(),
            async_client: Client::new(),
            pkce_code_verifier: None,
            client_req: None,
        }
    }

    pub fn from_auth_tokens(
        tokens: AuthTokens,
        client_id: ClientId,
        client_secret: ClientSecret,
        redirect_url: RedirectUrl,
    ) -> Auth {
        let mut auth = Auth::new(client_id, client_secret, redirect_url);

        auth.set_access_token_unchecked(tokens.access_token);
        auth.set_refresh_token_unchecked(tokens.refresh_token);
        auth.set_expires_at_unchecked(tokens.expires_at);
        auth.set_refresh_expires_at_unchecked(tokens.refresh_expires_at);

        auth
    }

    /// Return client tokens to save user creds that can be serialized/deserialized.
    /// serializes access/refresh tokens, and their expiry
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
        self.access_token.clone()
    }

    /// Get the refresh token.
    pub fn refresh_token(&self) -> RefreshToken {
        self.refresh_token.clone()
    }

    /// Manually set the refresh token. This is handled automatically by [`Auth::refresh()`], [`Auth::refresh_blocking()`], [`Auth::authenticate()`], and [`Auth::authenticate_finish_blocking()`].
    ///
    /// This method is safe in terms of no UB, however it is unchecked because it is possible to cause inconsistent state.
    ///
    /// Caller agrees to also set the correct refresh token expiry time as well.
    pub fn set_refresh_token_unchecked(&mut self, token: RefreshToken) {
        self.refresh_token = token;
    }

    /// Manually set the access token. This is handled automatically by [`Auth::refresh()`], [`Auth::refresh_blocking()`], [`Auth::authenticate()`], and [`Auth::authenticate_finish_blocking()`].
    ///
    /// This method is safe in terms of no UB, however it is unchecked because it is possible to cause inconsistent state.
    ///
    /// Caller agrees to also set the correct access token expiry time as well.
    pub fn set_access_token_unchecked(&mut self, token: AccessToken) {
        self.access_token = token;
    }

    /// Updates the access token expiry time. Expiry is utc seconds
    /// This is handled automatically by [`Auth::refresh()`], [`Auth::refresh_blocking()`], [`Auth::authenticate()`], and [`Auth::authenticate_finish_blocking()`].
    ///
    /// This method is safe in terms of no UB, however it is unchecked because it is possible to cause inconsistent state.
    ///
    /// Caller agrees to also set the correct access token as well.
    pub fn set_expires_at_unchecked(&mut self, expiry: u64) {
        self.expires_at = expiry;
    }

    /// Updates the access token expiry time. Duration is how long from NOW it will after in.
    /// This is handled automatically by [`Auth::refresh()`], [`Auth::refresh_blocking()`], [`Auth::authenticate()`], and [`Auth::authenticate_finish_blocking()`].
    ///
    /// This method is safe in terms of no UB, however it is unchecked because it is possible to cause inconsistent state.
    ///
    /// Caller agrees to also set the correct access token as well.
    pub fn set_expires_in_unchecked(&mut self, duration: Duration) {
        self.expires_at = Utc::now().timestamp() as u64 + duration.as_secs();
    }

    /// Updates the refresh token expiry time. Duration is how long from NOW it will after in.
    /// This is handled automatically by [`Auth::refresh()`], [`Auth::refresh_blocking()`], [`Auth::authenticate()`], and [`Auth::authenticate_finish_blocking()`].
    ///
    /// This method is safe in terms of no UB, however it is unchecked because it is possible to cause inconsistent state.
    ///
    /// Caller agrees to also set the correct refresh token as well.
    pub fn set_refresh_expires_in_unchecked(&mut self, duration: Duration) {
        self.refresh_expires_at = Utc::now().timestamp() as u64 + duration.as_secs();
    }

    /// Updates the refresh token expiry time. Expiry is utc seconds
    /// This is handled automatically by [`Auth::refresh()`], [`Auth::refresh_blocking()`], [`Auth::authenticate()`], and [`Auth::authenticate_finish_blocking()`].
    ///
    /// This method is safe in terms of no UB, however it is unchecked because it is possible to cause inconsistent state.
    ///
    /// Caller agrees to also set the correct access token as well.
    pub fn set_refresh_expires_at_unchecked(&mut self, expiry: u64) {
        self.refresh_expires_at = expiry;
    }

    /// Add an oauth2 scope. Use this before you generate a new token.
    pub fn add_scope(&mut self, scope: Scope) {
        self.scopes.push(scope);
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
        (Utc::now().timestamp() as u64) < self.expires_at
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
        (Utc::now().timestamp() as u64) < self.refresh_expires_at
    }

    /// Automatically regnerate refresh token if possible
    ///
    /// If this fails, you should authenticate the token again.
    ///
    /// This is subject to an inconsistent state if you are manually setting
    /// access/refresh token and/or their expiry times.
    pub async fn try_refresh(&mut self) -> Result<(), TokenError> {
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
    /// If this fails, you should authenticate the token again.
    ///
    /// This is subject to an inconsistent state if you are manually setting
    /// access/refresh token and/or their expiry times.
    #[cfg(feature = "blocking")]
    pub fn try_refresh_blocking(&mut self) -> Result<(), TokenError> {
        crate::RUNTIME.block_on(self.try_refresh())
    }

    /// Time in utc seconds when access token expires.
    pub fn expires_at(&self) -> u64 {
        self.expires_at
    }

    /// Time in utc seconds when refresh token expires.
    pub fn refresh_expires_at(&self) -> u64 {
        self.refresh_expires_at
    }

    /// How many days refresh token is valid for
    const DAYS: u64 = 31;

    /// Exchange refresh token for new access token.
    pub async fn refresh(&mut self) -> Result<(), TokenError> {
        let token = self
            .client
            .exchange_refresh_token(&self.refresh_token)
            .request_async(&self.async_client)
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
    #[cfg(feature = "blocking")]
    pub fn refresh_blocking(&mut self) -> Result<(), TokenError> {
        crate::RUNTIME.block_on(self.refresh())
    }

    /// Begin a new oauth2 access token generation procedure.
    ///
    /// Returns both url and state belonging to this authenticate request. Client should visit the auth url and authenticate.
    /// After client auths and gets sent to the redirect url on your server, if their state matches
    /// the state returned from this method, call [`Auth::authenticate_finish`] with their authorization code.
    pub fn authenticate(&mut self) -> Result<ClientAuthRequest, TokenError> {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_plain();

        let (auth_url, state) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scopes(self.scopes.iter().cloned())
            .set_pkce_challenge(pkce_challenge)
            .url();

        let req = ClientAuthRequest { auth_url, state };

        self.pkce_code_verifier = Some(pkce_verifier.into_secret());
        self.client_req = Some(req.clone());

        Ok(req)
    }

    /// So, a client has now authorized themselves, visited the redirect url on your server, and
    /// you've verified the state you received in the url matches up to this [`ClientAuthRequest`].
    /// Please pass in the client's state and authorization code you received in the redirect url.
    ///
    /// Note: Is it valid to call this method with a state that does not match.
    ///
    /// ALWAYS input ONLY the state you received from the redirect url on your server.
    /// If you arbitrarily put in your own state and it matches, then this will verify
    /// the client even if their state was incorrect (which is a security issue)!
    pub async fn authenticate_finish(
        &mut self,
        client_state: CsrfToken,
        auth_code: AuthorizationCode,
    ) -> Result<(), TokenError> {
        // ensure state is correct
        if self
            .client_req
            .as_ref()
            .is_some_and(|state| state.state.secret() != client_state.secret())
        {
            return Err(TokenError::StateMismatch);
        }

        let pkce_verifier = {
            let Some(pkce_verifier) = &self.pkce_code_verifier else {
                return Err(TokenError::PkceCodeVerifierMissing);
            };

            PkceCodeVerifier::new(pkce_verifier.clone())
        };

        // now get access token
        let Ok(token) = self
            .client
            .exchange_code(auth_code)
            .set_pkce_verifier(pkce_verifier)
            .request_async(&self.async_client)
            .await
        else {
            return Err(TokenError::Access);
        };

        // request succeeded; we don't need these anymore
        self.pkce_code_verifier.take();
        self.client_req.take();

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

    #[cfg(feature = "blocking")]
    pub fn authenticate_finish_blocking(
        &mut self,
        client_state: CsrfToken,
        auth_code: AuthorizationCode,
    ) -> Result<(), TokenError> {
        let fut = self.authenticate_finish(client_state, auth_code);
        crate::RUNTIME.block_on(fut)
    }
}

/// This type represents a particular client's request to authenticate
/// to your service.
#[derive(Clone)]
pub struct ClientAuthRequest {
    auth_url: Url,
    state: CsrfToken,
}

impl ClientAuthRequest {
    /// The url the client should visit to authenticate
    pub fn auth_url(&self) -> &Url {
        &self.auth_url
    }

    /// After the client visits the auth url and authorizes,
    /// you will receive a state param in the redirect url on your server.
    /// If it matches the state this method returns, the client belongs
    /// to this particular request, and is verified.
    pub fn state(&self) -> &CsrfToken {
        &self.state
    }
}

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum TokenError {
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
    #[error("PkceCodeVerifier not set; please call authenticate() first")]
    PkceCodeVerifierMissing,
}
