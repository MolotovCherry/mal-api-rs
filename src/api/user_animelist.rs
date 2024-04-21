use const_format::formatcp;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::{
    api_request::ApiError,
    objects::{AnimeList, AnimeSort, ReadStatus, Username},
    MalClient, API_URL,
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
        }
    }

    pub fn patch(&self) -> UserAnimeListApiPatch {
        UserAnimeListApiPatch {
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
pub struct UserAnimeListApiPatch {
    #[serde(skip)]
    client: MalClient,
    #[serde(skip)]
    anime_id: Option<u64>,

    status: Option<ReadStatus>,
    is_rewatching: Option<bool>,
    score: Option<u8>,
    num_watched_episodes: Option<u64>,
    priority: Option<u8>,
    num_times_rewatched: Option<u64>,
    rewatch_value: Option<u8>,
    tags: Option<String>,
    comments: Option<String>,
}

impl UserAnimeListApiPatch {
    pub fn anime_id(mut self, id: u64) -> Self {
        self.anime_id = Some(id);
        self
    }

    pub fn status(mut self, status: ReadStatus) -> Self {
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

    pub async fn send(self) -> Result<(), ApiError> {
        assert!(self.anime_id.is_some(), "anime_id is a required param");

        let url = USER_ANIME_ID.replace("{ANIME_ID}", &self.anime_id.unwrap().to_string());
        self.client.http.patch(url, Some(&self), true).await
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
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct UserAnimeListApiGet {
    #[serde(skip)]
    client: MalClient,
    #[serde(skip)]
    user_name: Option<Username>,

    status: Option<ReadStatus>,
    sort: Option<AnimeSort>,
    limit: Option<u16>,
    offset: Option<u64>,
}

impl UserAnimeListApiGet {
    pub fn user_name(mut self, user_name: Username) -> Self {
        self.user_name = Some(user_name);
        self
    }

    pub fn status(mut self, status: ReadStatus) -> Self {
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

    pub async fn send(self) -> Result<AnimeList, ApiError> {
        assert!(self.user_name.is_some(), "user_name is a required param");

        let query = serde_qs::to_string(&self)?;
        let url = USER_ANIMELIST_URL.replace("{USER_NAME}", &self.user_name.unwrap().to_string());

        let url = format!("{url}?{query}");
        self.client.http.get(url, true).await
    }
}