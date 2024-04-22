pub mod api;
pub mod api_request;
pub mod auth;
pub mod objects;
mod utils;

use std::sync::Arc;

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
/// <https://myanimelist.net/apiconfig/references/api/v2>
#[derive(Debug, Clone)]
pub struct MalClient {
    /// Holds oauth2 information. You are required to call functions on this to handle
    /// oauth2 token generation, token refreshing, and webserver redirect callback
    pub auth: Arc<Auth>,
    http: ApiRequest,
}

impl MalClient {
    /// Create a client with default useragent CARGO_PKG_NAME/CARGO_PKG_VERSION
    pub fn new(
        client_id: &str,
        client_secret: &str,
        redirect_uri: &str,
    ) -> Result<Self, MalClientError> {
        Self::new_with(client_id, client_secret, redirect_uri, |builder| {
            builder
                .user_agent(concat!(
                    env!("CARGO_PKG_NAME"),
                    "/",
                    env!("CARGO_PKG_VERSION"),
                ))
                .build()
        })
    }

    /// Create client with custom reqwest settings (user agent for example)
    pub fn new_with(
        client_id: &str,
        client_secret: &str,
        redirect_uri: &str,
        builder_cb: impl Fn(ClientBuilder) -> Result<Client, reqwest::Error>,
    ) -> Result<Self, MalClientError> {
        let builder = reqwest::Client::builder();
        let http = builder_cb(builder)?;

        let auth = Arc::new(Auth::new(client_id, client_secret, redirect_uri)?);
        let http = ApiRequest::new(auth.clone(), http);

        let slf = Self { auth, http };

        Ok(slf)
    }

    /// The anime endpoint
    /// <https://myanimelist.net/apiconfig/references/api/v2#tag/anime>
    pub fn anime(&self) -> AnimeApi {
        AnimeApi::new(self.clone())
    }

    /// The manga endpoint
    /// <https://myanimelist.net/apiconfig/references/api/v2#tag/manga>
    pub fn manga(&self) -> MangaApi {
        MangaApi::new(self.clone())
    }

    /// The user-animelist endpoint
    /// <https://myanimelist.net/apiconfig/references/api/v2#tag/user-animelist>
    pub fn user_animelist(&self) -> UserAnimeListApi {
        UserAnimeListApi::new(self.clone())
    }

    /// The user-mangalist endpoint
    /// <https://myanimelist.net/apiconfig/references/api/v2#tag/user-mangalist>
    pub fn user_mangalist(&self) -> UserMangaListApi {
        UserMangaListApi::new(self.clone())
    }

    /// The user endpoint
    /// <https://myanimelist.net/apiconfig/references/api/v2#tag/user>
    pub fn user(&self) -> UserApi {
        UserApi::new(self.clone())
    }

    /// The forum endpoint
    /// <https://myanimelist.net/apiconfig/references/api/v2#tag/forum>
    pub fn forum(&self) -> ForumApi {
        ForumApi::new(self.clone())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MalClientError {
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("{0}")]
    Token(#[from] TokenError),
}
