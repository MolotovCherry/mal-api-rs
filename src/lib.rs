pub mod api;
pub mod api_request;
pub mod auth;
pub mod objects;
mod utils;

use std::sync::Arc;

pub use oauth2::{
    AccessToken, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, RefreshToken,
    Scope,
};
use reqwest::{Client, ClientBuilder};
use tokio::runtime::{Builder, Runtime};

use crate::{
    api::{
        anime::AnimeApi, forum::ForumApi, manga::MangaApi, user::UserApi,
        user_animelist::UserAnimeListApi, user_mangalist::UserMangaListApi,
    },
    api_request::ApiRequest,
    auth::{Auth, TokenError},
    utils::LazyLock,
};

const BASE_URL: &str = "https://myanimelist.net/v1";
const API_URL: &str = "https://api.myanimelist.net/v2";

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
    /// Holds oauth2 information. You are required to call functions on this to handle
    /// oauth2 token generation, token refreshing, and webserver redirect callback
    pub auth: Arc<Auth>,
    http: ApiRequest,
}

impl MalClient {
    /// The anime endpoint
    ///
    /// <https://myanimelist.net/apiconfig/references/api/v2#tag/anime>
    pub fn anime(&self) -> AnimeApi {
        AnimeApi::new(self.clone())
    }

    /// The manga endpoint
    ///
    /// <https://myanimelist.net/apiconfig/references/api/v2#tag/manga>
    pub fn manga(&self) -> MangaApi {
        MangaApi::new(self.clone())
    }

    /// The user-animelist endpoint
    ///
    /// <https://myanimelist.net/apiconfig/references/api/v2#tag/user-animelist>
    pub fn user_animelist(&self) -> UserAnimeListApi {
        UserAnimeListApi::new(self.clone())
    }

    /// The user-mangalist endpoint
    ///
    /// <https://myanimelist.net/apiconfig/references/api/v2#tag/user-mangalist>
    pub fn user_mangalist(&self) -> UserMangaListApi {
        UserMangaListApi::new(self.clone())
    }

    /// The user endpoint
    ///
    /// <https://myanimelist.net/apiconfig/references/api/v2#tag/user>
    pub fn user(&self) -> UserApi {
        UserApi::new(self.clone())
    }

    /// The forum endpoint
    ///
    /// <https://myanimelist.net/apiconfig/references/api/v2#tag/forum>
    pub fn forum(&self) -> ForumApi {
        ForumApi::new(self.clone())
    }
}

/// A builder for [MalClient]
#[derive(Default)]
pub struct MalClientBuilder {
    auth: Option<Arc<Auth>>,
    client_id: Option<ClientId>,
    client_secret: Option<ClientSecret>,
    redirect_url: Option<RedirectUrl>,
    #[allow(clippy::complexity)]
    http_cb: Option<Box<dyn FnOnce(ClientBuilder) -> Result<Client, reqwest::Error> + 'static>>,
}

impl MalClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Use your own [Auth] value.
    ///
    /// If [Auth] is not provided, you must set client_id, client_secret, and redirect_url.
    pub fn auth(mut self, auth: Auth) -> Self {
        self.auth = Some(Arc::new(auth));
        self
    }

    /// Use a shared [Auth] value you have.
    ///
    /// If [Auth] is not provided, you must set client_id, client_secret, and redirect_url.
    pub fn auth_shared(mut self, auth: Arc<Auth>) -> Self {
        self.auth = Some(auth);
        self
    }

    /// The client id used to make a new [Auth]. No need to specify if you provided an [Auth] to the builder.
    pub fn client_id(mut self, client_id: ClientId) -> Self {
        self.client_id = Some(client_id);
        self
    }

    /// The client secret used to make a new [Auth]. No need to specify if you provided an [Auth] to the builder.
    pub fn client_secret(mut self, client_secret: ClientSecret) -> Self {
        self.client_secret = Some(client_secret);
        self
    }

    /// The redirect_url used to make a new [Auth]. No need to specify if you provided an [Auth] to the builder.
    pub fn redirect_url(mut self, redirect_url: RedirectUrl) -> Self {
        self.redirect_url = Some(redirect_url);
        self
    }

    /// Customize the reqwest client (e.g. change the useragent).
    pub fn http_builder(
        mut self,
        cb: impl FnOnce(ClientBuilder) -> Result<Client, reqwest::Error> + 'static,
    ) -> Self {
        self.http_cb = Some(Box::new(cb));
        self
    }

    pub fn build(self) -> Result<MalClient, MalClientError> {
        let auth = if let Some(auth) = self.auth {
            auth
        } else {
            let Some(client_id) = self.client_id else {
                return Err(MalClientError::Builder("client_id".to_owned()));
            };

            let Some(client_secret) = self.client_secret else {
                return Err(MalClientError::Builder("client_secret".to_owned()));
            };

            let Some(redirect_url) = self.redirect_url else {
                return Err(MalClientError::Builder("redirect_url".to_owned()));
            };

            Arc::new(Auth::new(client_id, client_secret, redirect_url))
        };

        let http = if let Some(cb) = self.http_cb {
            let builder = ClientBuilder::new();
            cb(builder)?
        } else {
            ClientBuilder::new()
                .user_agent(concat!(
                    env!("CARGO_PKG_NAME"),
                    "/",
                    env!("CARGO_PKG_VERSION"),
                ))
                .build()?
        };

        let http = ApiRequest::new(auth.clone(), http);

        let mal_client = MalClient { auth, http };

        Ok(mal_client)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MalClientError {
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("field '{0}' is required")]
    Builder(String),
    #[error("{0}")]
    Token(#[from] TokenError),
}
