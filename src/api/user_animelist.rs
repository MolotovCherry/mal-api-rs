use const_format::formatcp;
use itertools::Itertools as _;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::{
    api_request::ApiError,
    objects::{AnimeList, AnimeListItem, AnimeSort, Username, WatchStatus},
    MalClient, API_URL, RUNTIME,
};

pub const USER_ANIMELIST_URL: &str = formatcp!("{API_URL}/users/{{USER_NAME}}/animelist");
pub const USER_ANIME_ID: &str = formatcp!("{API_URL}/anime/{{ANIME_ID}}/my_list_status");

#[derive(Debug, Clone)]
pub struct UserAnimeListApi {
    client: MalClient,
}

impl UserAnimeListApi {
    pub(crate) fn new(mal_client: MalClient) -> Self {
        Self { client: mal_client }
    }

    /// GET user animelist
    ///
    /// <https://myanimelist.net/apiconfig/references/api/v2#operation/users_user_id_animelist_get>
    pub fn get(&self) -> UserAnimeListApiGet {
        UserAnimeListApiGet {
            client: self.client.clone(),
            user_name: None,
            status: None,
            sort: None,
            limit: None,
            offset: None,
            fields: None,
            nsfw: None,
        }
    }

    /// PATCH user animelist
    ///
    /// Add specified anime to my anime list.
    ///
    /// If specified anime already exists, update its status.
    ///
    /// This endpoint updates only values specified by the parameter.
    ///
    /// <https://myanimelist.net/apiconfig/references/api/v2#operation/anime_anime_id_my_list_status_put>
    pub fn put(&self) -> UserAnimeListApiPut {
        UserAnimeListApiPut {
            client: self.client.clone(),
            anime_id: None,
            status: None,
            is_rewatching: None,
            score: None,
            num_watched_episodes: None,
            priority: None,
            num_times_rewatched: None,
            rewatch_value: None,
            tags: None,
            comments: None,
        }
    }

    /// DELETE user animelist item
    ///
    /// If the specified anime does not exist in user's anime list, this endpoint does nothing and returns 404 Not Found.
    ///
    /// So be careful when retrying.
    ///
    /// <https://myanimelist.net/apiconfig/references/api/v2#operation/anime_anime_id_my_list_status_delete>
    pub fn delete(&self) -> UserAnimeListApiDelete {
        UserAnimeListApiDelete {
            client: self.client.clone(),
            anime_id: None,
        }
    }
}

/// PATCH user animelist
///
/// Add specified anime to my anime list.
///
/// If specified anime already exists, update its status.
///
/// This endpoint updates only values specified by the parameter.
///
/// <https://myanimelist.net/apiconfig/references/api/v2#operation/anime_anime_id_my_list_status_put>
#[skip_serializing_none]
#[derive(Serialize, Debug)]
pub struct UserAnimeListApiPut {
    #[serde(skip)]
    client: MalClient,
    #[serde(skip)]
    anime_id: Option<u64>,

    status: Option<WatchStatus>,
    is_rewatching: Option<bool>,
    score: Option<u8>,
    num_watched_episodes: Option<u64>,
    priority: Option<u8>,
    num_times_rewatched: Option<u64>,
    rewatch_value: Option<u8>,
    tags: Option<String>,
    comments: Option<String>,
}

impl UserAnimeListApiPut {
    /// The anime id to update. This parameter is required.
    pub fn anime_id(mut self, id: u64) -> Self {
        self.anime_id = Some(id);
        self
    }

    pub fn status(mut self, status: WatchStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn is_rewatching(mut self, is_rewatching: bool) -> Self {
        self.is_rewatching = Some(is_rewatching);
        self
    }

    pub fn score(mut self, score: u8) -> Self {
        self.score = Some(score.clamp(0, 10));
        self
    }

    pub fn num_watched_episodes(mut self, num: u64) -> Self {
        self.num_watched_episodes = Some(num);
        self
    }

    pub fn priority(mut self, priority: u8) -> Self {
        self.priority = Some(priority.clamp(0, 2));
        self
    }

    pub fn num_times_rewatched(mut self, num: u64) -> Self {
        self.num_times_rewatched = Some(num);
        self
    }

    pub fn rewatch_value(mut self, value: u8) -> Self {
        self.rewatch_value = Some(value.clamp(0, 5));
        self
    }

    pub fn tags(mut self, tags: &str) -> Self {
        self.tags = Some(tags.to_owned());
        self
    }

    pub fn comments(mut self, comments: &str) -> Self {
        self.comments = Some(comments.to_owned());
        self
    }

    /// Send the request.
    pub async fn send(self) -> Result<AnimeListItem, ApiError> {
        assert!(self.anime_id.is_some(), "anime_id is a required param");

        let url = USER_ANIME_ID.replace("{ANIME_ID}", &self.anime_id.unwrap().to_string());
        self.client.http.put(url, Some(&self), true).await
    }

    /// Send the request.
    pub fn send_blocking(self) -> Result<AnimeListItem, ApiError> {
        RUNTIME.block_on(self.send())
    }
}

/// DELETE user animelist item
///
/// If the specified anime does not exist in user's anime list, this endpoint does nothing and returns 404 Not Found.
///
/// So be careful when retrying.
///
/// <https://myanimelist.net/apiconfig/references/api/v2#operation/anime_anime_id_my_list_status_delete>
#[derive(Debug)]
pub struct UserAnimeListApiDelete {
    client: MalClient,
    anime_id: Option<u64>,
}

impl UserAnimeListApiDelete {
    /// The anime id in the list to delete. This parameter is required.
    pub fn anime_id(mut self, id: u64) -> Self {
        self.anime_id = Some(id);
        self
    }

    /// Send the request.
    pub async fn send(self) -> Result<(), ApiError> {
        assert!(self.anime_id.is_some(), "anime_id is a required param");

        let url = USER_ANIME_ID.replace("{ANIME_ID}", &self.anime_id.unwrap().to_string());
        self.client.http.delete(url, true).await
    }

    /// Send the request.
    pub fn send_blocking(self) -> Result<(), ApiError> {
        RUNTIME.block_on(self.send())
    }
}

/// GET user animelist
///
/// <https://myanimelist.net/apiconfig/references/api/v2#operation/users_user_id_animelist_get>
#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct UserAnimeListApiGet {
    #[serde(skip)]
    client: MalClient,
    #[serde(skip)]
    user_name: Option<Username>,

    status: Option<WatchStatus>,
    sort: Option<AnimeSort>,
    limit: Option<u16>,
    offset: Option<u64>,
    fields: Option<String>,
    nsfw: Option<bool>,
}

impl UserAnimeListApiGet {
    /// User name. This parameter is required.
    pub fn user_name(mut self, user_name: Username) -> Self {
        self.user_name = Some(user_name);
        self
    }

    /// Filters returned anime list by these statuses.
    /// To return all anime, don't specify this field.
    pub fn status(mut self, status: WatchStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn sort(mut self, sort: AnimeSort) -> Self {
        self.sort = Some(sort);
        self
    }

    /// Default: 100
    /// The maximum value is 1000.
    pub fn limit(mut self, limit: u16) -> Self {
        self.limit = Some(limit.clamp(0, 1000));
        self
    }

    /// Default: 0
    pub fn offset(mut self, offset: u64) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn fields<I: IntoIterator<Item = impl AsRef<str>>>(mut self, fields: I) -> Self {
        let fields = fields.into_iter().map(|f| f.as_ref().to_string()).join(",");

        self.fields = Some(fields);
        self
    }

    /// Whether to return nsfw material.
    pub fn nsfw(mut self, nsfw: bool) -> Self {
        self.nsfw = Some(nsfw);
        self
    }

    /// Send the request.
    pub async fn send(self) -> Result<AnimeList, ApiError> {
        assert!(self.user_name.is_some(), "user_name is a required param");

        let username = self.user_name.as_ref().unwrap().to_string();

        let query = serde_qs::to_string(&self)?;
        let url = USER_ANIMELIST_URL.replace("{USER_NAME}", &username);

        let url = format!("{url}?{query}");

        // use access token when Me, and client token when other users
        let is_auth = matches!(self.user_name.as_ref().unwrap(), Username::Me);

        self.client.http.get(url, is_auth).await
    }

    /// Send the request.
    pub fn send_blocking(self) -> Result<AnimeList, ApiError> {
        RUNTIME.block_on(self.send())
    }
}
