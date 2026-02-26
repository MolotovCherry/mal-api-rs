pub mod api;
pub mod api_request;
pub mod auth;
pub mod objects;

#[cfg(feature = "blocking")]
use std::sync::LazyLock;

pub use oauth2::{
    AccessToken, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, RefreshToken,
    Scope,
};
use reqwest::{Client, ClientBuilder};
#[cfg(feature = "blocking")]
use tokio::runtime::{Builder, Runtime};

use crate::{
    api::{
        anime::AnimeApi, forum::ForumApi, manga::MangaApi, user::UserApi,
        user_animelist::UserAnimeListApi, user_mangalist::UserMangaListApi,
    },
    api_request::ApiRequest,
    auth::AuthTokens,
};

const BASE_URL: &str = "https://myanimelist.net/v1";
const API_URL: &str = "https://api.myanimelist.net/v2";

#[cfg(feature = "blocking")]
static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| {
    Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed building the Runtime")
});

/// A MyAnimeList Client.
///
/// For proper usage of the api, please read the myanimelist docs:
///
/// <https://myanimelist.net/apiconfig/references/api/v2>
#[derive(Debug, Clone)]
pub struct MalClient {
    auth_tokens: AuthTokens,
    http: Client,
    client_id: ClientId,
}

impl MalClient {
    pub fn builder() -> MalClientBuilder {
        MalClientBuilder::new()
    }

    pub(crate) fn api_request(&self) -> ApiRequest<'_> {
        ApiRequest::new(self)
    }

    pub fn set_tokens(&mut self, tokens: AuthTokens) {
        self.auth_tokens = tokens;
    }

    /// The anime endpoint
    ///
    /// <https://myanimelist.net/apiconfig/references/api/v2#tag/anime>
    pub fn anime(&self) -> AnimeApi<'_> {
        AnimeApi::new(self)
    }

    /// The manga endpoint
    ///
    /// <https://myanimelist.net/apiconfig/references/api/v2#tag/manga>
    pub fn manga(&self) -> MangaApi<'_> {
        MangaApi::new(self)
    }

    /// The user-animelist endpoint
    ///
    /// <https://myanimelist.net/apiconfig/references/api/v2#tag/user-animelist>
    pub fn user_animelist(&self) -> UserAnimeListApi<'_> {
        UserAnimeListApi::new(self)
    }

    /// The user-mangalist endpoint
    ///
    /// <https://myanimelist.net/apiconfig/references/api/v2#tag/user-mangalist>
    pub fn user_mangalist(&self) -> UserMangaListApi<'_> {
        UserMangaListApi::new(self)
    }

    /// The user endpoint
    ///
    /// <https://myanimelist.net/apiconfig/references/api/v2#tag/user>
    pub fn user(&self) -> UserApi<'_> {
        UserApi::new(self)
    }

    /// The forum endpoint
    ///
    /// <https://myanimelist.net/apiconfig/references/api/v2#tag/forum>
    pub fn forum(&self) -> ForumApi<'_> {
        ForumApi::new(self)
    }
}

/// A builder for [MalClient]
#[derive(Default)]
pub struct MalClientBuilder {
    auth_tokens: Option<AuthTokens>,
    client_id: Option<ClientId>,
    client: Option<Client>,
    #[allow(clippy::complexity)]
    http_cb: Option<Box<dyn FnOnce(ClientBuilder) -> Result<Client, reqwest::Error> + 'static>>,
}

impl MalClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Provide [AuthTokens] for the client.
    ///
    /// ```rust
    /// let auth: Auth = /*value*/;
    /// MalClientBuilder::new().auth_tokens(&auth);
    /// MalClientBuilder::new().auth_tokens(auth);
    /// MalClientBuilder::new().auth_tokens(auth.to_tokens());
    /// ```
    pub fn auth_tokens(mut self, auth: impl Into<AuthTokens>) -> Self {
        self.auth_tokens = Some(auth.into());
        self
    }

    /// Your myanimelist client id.
    pub fn client_id(mut self, client_id: ClientId) -> Self {
        self.client_id = Some(client_id);
        self
    }

    /// Provide a reqwest client. If specified, [MalClientBuilder::http_builder] takes precedence over this one. If both are empty, one will be automatically created.
    pub fn http(mut self, client: Client) -> Self {
        self.client = Some(client);
        self
    }

    /// Customize a reqwest client (e.g. change the useragent). If empty, uses [MalClientBuilder::http], or if both empty, one is automatically created.
    pub fn http_builder(
        mut self,
        cb: impl FnOnce(ClientBuilder) -> Result<Client, reqwest::Error> + 'static,
    ) -> Self {
        self.http_cb = Some(Box::new(cb));
        self
    }

    pub fn build(self) -> Result<MalClient, MalClientError> {
        let Some(client_id) = self.client_id else {
            return Err(MalClientError::Builder("client_id".to_owned()));
        };

        let Some(auth) = self.auth_tokens else {
            return Err(MalClientError::Builder("auth_tokens".to_owned()));
        };

        let http = if let Some(cb) = self.http_cb {
            let builder = ClientBuilder::new();
            cb(builder)?
        } else if let Some(client) = self.client {
            client
        } else {
            ClientBuilder::new()
                .user_agent(concat!(
                    env!("CARGO_PKG_NAME"),
                    "/",
                    env!("CARGO_PKG_VERSION"),
                ))
                .build()?
        };

        let mal_client = MalClient {
            auth_tokens: auth,
            http,
            client_id,
        };

        Ok(mal_client)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MalClientError {
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("field '{0}' is required")]
    Builder(String),
}
