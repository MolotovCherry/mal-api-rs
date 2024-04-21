mod api;
mod api_request;
mod auth;
mod objects;

use std::sync::Arc;

use reqwest::{Client, ClientBuilder};

pub use api::*;
pub use auth::*;
pub use objects::*;

use self::{
    api::{
        anime::AnimeApi, forum::ForumApi, manga::MangaApi, user::UserApi,
        user_animelist::UserAnimeListApi, user_mangalist::UserMangaListApi,
    },
    api_request::ApiRequest,
};

pub const BASE_URL: &str = "https://myanimelist.net/v1";
pub const API_URL: &str = "https://api.myanimelist.net/v2";

#[derive(Debug, Clone)]
pub struct MalClient {
    pub auth: Arc<Auth>,
    http: ApiRequest,
}

impl MalClient {
    /// Create client
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

    pub fn anime(&self) -> AnimeApi {
        AnimeApi::new(self.clone())
    }

    pub fn manga(&self) -> MangaApi {
        MangaApi::new(self.clone())
    }

    pub fn user_animelist(&self) -> UserAnimeListApi {
        UserAnimeListApi::new(self.clone())
    }

    pub fn user_mangalist(&self) -> UserMangaListApi {
        UserMangaListApi::new(self.clone())
    }

    pub fn user(&self) -> UserApi {
        UserApi::new(self.clone())
    }

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
