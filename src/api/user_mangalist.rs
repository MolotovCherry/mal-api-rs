use const_format::formatcp;
use itertools::Itertools as _;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::{
    api_request::ApiError,
    objects::{MangaList, MangaSort, ReadStatus, Username},
    MalClient, MangaListItem, API_URL, RUNTIME,
};

pub const USER_MANGALIST_URL: &str = formatcp!("{API_URL}/users/{{USER_NAME}}/mangalist");
pub const USER_MANGA_ID: &str = formatcp!("{API_URL}/manga/{{MANGA_ID}}/my_list_status");

#[derive(Debug, Clone)]
pub struct UserMangaListApi {
    client: MalClient,
}

impl UserMangaListApi {
    pub(crate) fn new(mal_client: MalClient) -> Self {
        Self { client: mal_client }
    }

    pub fn get(&self) -> UserMangaListApiGet {
        UserMangaListApiGet {
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

    pub fn put(&self) -> UserMangaListApiPut {
        UserMangaListApiPut {
            client: self.client.clone(),
            manga_id: None,
            status: None,
            is_rereading: None,
            score: None,
            num_volumes_read: None,
            num_chapters_read: None,
            priority: None,
            num_times_reread: None,
            reread_value: None,
            tags: None,
            comments: None,
        }
    }

    pub fn delete(&self) -> UserMangaListApiDelete {
        UserMangaListApiDelete {
            client: self.client.clone(),
            manga_id: None,
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Debug)]
pub struct UserMangaListApiPut {
    #[serde(skip)]
    client: MalClient,
    #[serde(skip)]
    manga_id: Option<u64>,

    status: Option<ReadStatus>,
    is_rereading: Option<bool>,
    score: Option<u8>,
    num_volumes_read: Option<u64>,
    num_chapters_read: Option<u64>,
    priority: Option<u8>,
    num_times_reread: Option<u64>,
    reread_value: Option<u8>,
    tags: Option<String>,
    comments: Option<String>,
}

impl UserMangaListApiPut {
    pub fn manga_id(mut self, id: u64) -> Self {
        self.manga_id = Some(id);
        self
    }

    pub fn status(mut self, status: ReadStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn is_rereading(mut self, is_rereading: bool) -> Self {
        self.is_rereading = Some(is_rereading);
        self
    }

    pub fn score(mut self, score: u8) -> Self {
        self.score = Some(score.clamp(0, 10));
        self
    }

    pub fn num_volumes_read(mut self, num: u64) -> Self {
        self.num_volumes_read = Some(num);
        self
    }

    pub fn num_chapters_read(mut self, num: u64) -> Self {
        self.num_chapters_read = Some(num);
        self
    }

    pub fn priority(mut self, priority: u8) -> Self {
        self.priority = Some(priority.clamp(0, 2));
        self
    }

    pub fn num_times_reread(mut self, num: u64) -> Self {
        self.num_times_reread = Some(num);
        self
    }

    pub fn reread_value(mut self, value: u8) -> Self {
        self.reread_value = Some(value.clamp(0, 5));
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

    pub async fn send(self) -> Result<MangaListItem, ApiError> {
        assert!(self.manga_id.is_some(), "manga_id is a required param");

        let url = USER_MANGA_ID.replace("{MANGA_ID}", &self.manga_id.unwrap().to_string());
        self.client.http.put(url, Some(&self), true).await
    }

    pub fn send_blocking(self) -> Result<MangaListItem, ApiError> {
        RUNTIME.block_on(self.send())
    }
}

#[derive(Debug)]
pub struct UserMangaListApiDelete {
    client: MalClient,
    manga_id: Option<u64>,
}

impl UserMangaListApiDelete {
    pub fn manga_id(mut self, id: u64) -> Self {
        self.manga_id = Some(id);
        self
    }

    pub async fn send(self) -> Result<(), ApiError> {
        assert!(self.manga_id.is_some(), "manga_id is a required param");

        let url = USER_MANGA_ID.replace("{MANGA_ID}", &self.manga_id.unwrap().to_string());
        self.client.http.delete(url, true).await
    }

    pub fn send_blocking(self) -> Result<(), ApiError> {
        RUNTIME.block_on(self.send())
    }
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct UserMangaListApiGet {
    #[serde(skip)]
    client: MalClient,
    #[serde(skip)]
    user_name: Option<Username>,

    status: Option<ReadStatus>,
    sort: Option<MangaSort>,
    limit: Option<u16>,
    offset: Option<u64>,
    fields: Option<String>,
    nsfw: Option<bool>,
}

impl UserMangaListApiGet {
    pub fn user_name(mut self, user_name: Username) -> Self {
        self.user_name = Some(user_name);
        self
    }

    pub fn status(mut self, status: ReadStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn sort(mut self, sort: MangaSort) -> Self {
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

    pub async fn send(self) -> Result<MangaList, ApiError> {
        assert!(self.user_name.is_some(), "user_name is a required param");

        let username = self.user_name.as_ref().unwrap().to_string();

        let query = serde_qs::to_string(&self)?;
        let url = USER_MANGALIST_URL.replace("{USER_NAME}", &username);

        let url = format!("{url}?{query}");

        // use access token when Me, and client token when other users
        let is_auth = matches!(self.user_name.as_ref().unwrap(), Username::Me);

        self.client.http.get(url, is_auth).await
    }

    pub fn send_blocking(self) -> Result<MangaList, ApiError> {
        RUNTIME.block_on(self.send())
    }
}
