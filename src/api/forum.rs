use const_format::formatcp;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::{
    api_request::ApiError, ForumBoards, ForumSort, ForumTopics, MalClient, TopicDetail, API_URL,
};

pub const FORUM_BOARDS: &str = formatcp!("{API_URL}/forum/boards");
pub const FORUM_TID: &str = formatcp!("{API_URL}/forum/topic/{{TOPIC_ID}}");
pub const FORUM_TOPICS: &str = formatcp!("{API_URL}/forum/topics");

#[derive(Debug)]
pub struct ForumApi {
    client: MalClient,
}

impl ForumApi {
    pub(crate) fn new(client: MalClient) -> Self {
        Self { client }
    }

    pub fn get(&self) -> ForumApiGet {
        ForumApiGet {
            client: self.client.clone(),
        }
    }
}

#[derive(Debug)]
pub struct ForumApiGet {
    client: MalClient,
}

impl ForumApiGet {
    pub fn boards(self) -> ForumApiGetBoards {
        ForumApiGetBoards {
            client: self.client,
        }
    }

    pub fn topic_detail(self) -> ForumApiGetTopicDetail {
        ForumApiGetTopicDetail {
            client: self.client,
            offset: None,
            topic_id: None,
            limit: None,
        }
    }

    pub fn topics(self) -> ForumApiGetTopics {
        ForumApiGetTopics {
            client: self.client,
            board_id: None,
            subboard_id: None,
            sort: None,
            q: None,
            topic_user_name: None,
            user_name: None,
            limit: None,
            offset: None,
        }
    }
}

#[derive(Debug)]
pub struct ForumApiGetBoards {
    client: MalClient,
}

impl ForumApiGetBoards {
    pub async fn send(self) -> Result<ForumBoards, ApiError> {
        self.client.http.get(FORUM_BOARDS, false).await
    }
}

#[skip_serializing_none]
#[derive(Serialize, Debug)]
pub struct ForumApiGetTopicDetail {
    #[serde(skip)]
    client: MalClient,
    #[serde(skip)]
    topic_id: Option<u64>,

    limit: Option<u8>,
    offset: Option<u64>,
}

impl ForumApiGetTopicDetail {
    pub fn topic_id(mut self, id: u64) -> Self {
        self.topic_id = Some(id);
        self
    }

    pub fn limit(mut self, limit: u8) -> Self {
        self.limit = Some(limit.clamp(0, 100));
        self
    }

    pub fn offset(mut self, offset: u64) -> Self {
        self.offset = Some(offset);
        self
    }

    pub async fn send(self) -> Result<TopicDetail, ApiError> {
        assert!(self.topic_id.is_some(), "topic_id is a required param");

        let query = serde_qs::to_string(&self)?;
        let url = FORUM_TID.replace("{TOPIC_ID}", &self.topic_id.unwrap().to_string());

        let url = format!("{url}?{query}");
        self.client.http.get(url, false).await
    }
}

#[skip_serializing_none]
#[derive(Serialize, Debug)]
pub struct ForumApiGetTopics {
    #[serde(skip)]
    client: MalClient,

    board_id: Option<u64>,
    subboard_id: Option<u64>,
    limit: Option<u8>,
    offset: Option<u64>,
    sort: Option<ForumSort>,
    q: Option<String>,
    topic_user_name: Option<String>,
    user_name: Option<String>,
}

impl ForumApiGetTopics {
    pub fn board_id(mut self, id: u64) -> Self {
        self.board_id = Some(id);
        self
    }

    pub fn subboard_id(mut self, id: u64) -> Self {
        self.subboard_id = Some(id);
        self
    }

    pub fn limit(mut self, limit: u8) -> Self {
        self.limit = Some(limit.clamp(0, 100));
        self
    }

    pub fn offset(mut self, offset: u64) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn sort(mut self, sort: ForumSort) -> Self {
        self.sort = Some(sort);
        self
    }

    pub fn q(mut self, q: &str) -> Self {
        self.q = Some(q.to_owned());
        self
    }

    pub fn topic_user_name(mut self, topic_user_name: &str) -> Self {
        self.topic_user_name = Some(topic_user_name.to_owned());
        self
    }

    pub fn user_name(mut self, user_name: &str) -> Self {
        self.user_name = Some(user_name.to_owned());
        self
    }

    pub async fn send(self) -> Result<ForumTopics, ApiError> {
        let query = serde_qs::to_string(&self)?;
        let url = format!("{FORUM_TOPICS}?{query}");

        self.client.http.get(url, false).await
    }
}
