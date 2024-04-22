use const_format::formatcp;
use itertools::Itertools as _;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::{
    api_request::ApiError,
    objects::{AnimeList, AnimeSort, Username},
    AnimeListItem, MalClient, WatchStatus, API_URL, RUNTIME,
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

    pub fn delete(&self) -> UserAnimeListApiDelete {
        UserAnimeListApiDelete {
            client: self.client.clone(),
            anime_id: None,
        }
    }
}

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

    pub async fn send(self) -> Result<AnimeListItem, ApiError> {
        assert!(self.anime_id.is_some(), "anime_id is a required param");

        let url = USER_ANIME_ID.replace("{ANIME_ID}", &self.anime_id.unwrap().to_string());
        self.client.http.put(url, Some(&self), true).await
    }

    pub fn send_blocking(self) -> Result<AnimeListItem, ApiError> {
        RUNTIME.block_on(self.send())
    }
}

#[derive(Debug)]
pub struct UserAnimeListApiDelete {
    client: MalClient,
    anime_id: Option<u64>,
}

impl UserAnimeListApiDelete {
    pub fn anime_id(mut self, id: u64) -> Self {
        self.anime_id = Some(id);
        self
    }

    pub async fn send(self) -> Result<(), ApiError> {
        assert!(self.anime_id.is_some(), "anime_id is a required param");

        let url = USER_ANIME_ID.replace("{ANIME_ID}", &self.anime_id.unwrap().to_string());
        self.client.http.delete(url, true).await
    }

    pub fn send_blocking(self) -> Result<(), ApiError> {
        RUNTIME.block_on(self.send())
    }
}

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
    pub fn user_name(mut self, user_name: Username) -> Self {
        self.user_name = Some(user_name);
        self
    }

    pub fn status(mut self, status: WatchStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn sort(mut self, sort: AnimeSort) -> Self {
        self.sort = Some(sort);
        self
    }

    pub fn limit(mut self, limit: u16) -> Self {
        self.limit = Some(limit.clamp(0, 1000));
        self
    }

    pub fn offset(mut self, offset: u64) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn fields<I: IntoIterator<Item = impl AsRef<str>>>(mut self, fields: I) -> Self {
        let fields = fields.into_iter().map(|f| f.as_ref().to_string()).join(",");

        self.fields = Some(fields);
        self
    }

    pub fn nsfw(mut self, nsfw: bool) -> Self {
        self.nsfw = Some(nsfw);
        self
    }

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

    pub fn send_blocking(self) -> Result<AnimeList, ApiError> {
        RUNTIME.block_on(self.send())
    }
}
