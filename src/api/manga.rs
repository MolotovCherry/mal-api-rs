use const_format::formatcp;
use itertools::Itertools as _;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::{
    api_request::ApiError,
    objects::{MangaNode, MangaRankingType, MangaSingleList},
    MalClient, API_URL,
};

pub const MANGA: &str = formatcp!("{API_URL}/manga");
pub const MANGA_ID: &str = formatcp!("{API_URL}/manga/{{MANGA_ID}}");
pub const MANGA_RANKING: &str = formatcp!("{API_URL}/manga/ranking");

#[derive(Debug)]
pub struct MangaApi<'a> {
    client: &'a MalClient,
}

impl<'a> MangaApi<'a> {
    pub(crate) fn new(client: &'a MalClient) -> Self {
        Self { client }
    }

    /// The manga GET endpoints.
    ///
    /// <https://myanimelist.net/apiconfig/references/api/v2#tag/manga>
    pub fn get(&self) -> MangaApiGet<'a> {
        MangaApiGet {
            client: self.client,
        }
    }
}

#[derive(Debug)]
pub struct MangaApiGet<'a> {
    client: &'a MalClient,
}

impl<'a> MangaApiGet<'a> {
    /// GET manga list.
    ///
    /// <https://myanimelist.net/apiconfig/references/api/v2#operation/manga_get>
    pub fn list(self) -> MangaApiGetList<'a> {
        MangaApiGetList {
            client: self.client,
            q: None,
            limit: None,
            offset: None,
            fields: None,
            nsfw: None,
        }
    }

    /// GET manga details.
    ///
    /// <https://myanimelist.net/apiconfig/references/api/v2#operation/manga_manga_id_get>
    pub fn details(self) -> MangaApiGetDetails<'a> {
        MangaApiGetDetails {
            client: self.client,
            manga_id: None,
            fields: None,
        }
    }

    /// GET manga ranking.
    ///
    /// <https://myanimelist.net/apiconfig/references/api/v2#operation/manga_ranking_get>
    pub fn ranking(self) -> MangaApiGetRanking<'a> {
        MangaApiGetRanking {
            client: self.client,
            ranking_type: None,
            limit: None,
            offset: None,
            fields: None,
        }
    }
}

/// GET manga list.
///
/// <https://myanimelist.net/apiconfig/references/api/v2#operation/manga_get>
#[skip_serializing_none]
#[derive(Serialize, Debug)]
pub struct MangaApiGetList<'a> {
    #[serde(skip)]
    client: &'a MalClient,

    q: Option<String>,
    limit: Option<u16>,
    offset: Option<u64>,
    fields: Option<String>,
    nsfw: Option<bool>,
}

impl<'a> MangaApiGetList<'a> {
    /// Search.
    pub fn q(mut self, q: &str) -> Self {
        self.q = Some(q.to_owned());
        self
    }

    /// Default: 100
    /// The maximum value is 100.
    pub fn limit(mut self, limit: u16) -> Self {
        self.limit = Some(limit);
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
    pub async fn send(self) -> Result<MangaSingleList, ApiError> {
        let query = serde_qs::to_string(&self)?;

        let url = format!("{MANGA}?{query}");
        self.client.api_request().get(url, false).await
    }

    /// Send the request.
    #[cfg(feature = "blocking")]
    pub fn send_blocking(self) -> Result<MangaSingleList, ApiError> {
        crate::RUNTIME.block_on(self.send())
    }
}

/// GET manga details.
///
/// <https://myanimelist.net/apiconfig/references/api/v2#operation/manga_manga_id_get>
#[skip_serializing_none]
#[derive(Serialize, Debug)]
pub struct MangaApiGetDetails<'a> {
    #[serde(skip)]
    client: &'a MalClient,
    #[serde(skip)]
    manga_id: Option<u64>,

    fields: Option<String>,
}

impl<'a> MangaApiGetDetails<'a> {
    /// The manga id. This parameter is required.
    pub fn manga_id(mut self, id: u64) -> Self {
        self.manga_id = Some(id);
        self
    }

    pub fn fields<I: IntoIterator<Item = impl AsRef<str>>>(mut self, fields: I) -> Self {
        let fields = fields.into_iter().map(|f| f.as_ref().to_string()).join(",");

        self.fields = Some(fields);
        self
    }

    /// Send the request.
    pub async fn send(self) -> Result<MangaNode, ApiError> {
        assert!(self.manga_id.is_some(), "manga_id is a required param");

        let query = serde_qs::to_string(&self)?;
        let url = MANGA_ID.replace("{MANGA_ID}", &self.manga_id.unwrap().to_string());

        let url = format!("{url}?{query}");
        self.client.api_request().get(url, false).await
    }

    /// Send the request.
    #[cfg(feature = "blocking")]
    pub fn send_blocking(self) -> Result<MangaNode, ApiError> {
        crate::RUNTIME.block_on(self.send())
    }
}

/// GET manga ranking.
///
/// <https://myanimelist.net/apiconfig/references/api/v2#operation/manga_ranking_get>
#[skip_serializing_none]
#[derive(Serialize, Debug)]
pub struct MangaApiGetRanking<'a> {
    #[serde(skip)]
    client: &'a MalClient,

    ranking_type: Option<MangaRankingType>,
    limit: Option<u16>,
    offset: Option<u64>,
    fields: Option<String>,
}

impl<'a> MangaApiGetRanking<'a> {
    /// The ranking type. This parameter is required.
    pub fn ranking_type(mut self, ranking: MangaRankingType) -> Self {
        self.ranking_type = Some(ranking);
        self
    }

    /// Default: 100
    /// The maximum value is 500.
    pub fn limit(mut self, limit: u16) -> Self {
        self.limit = Some(limit.clamp(0, 500));
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

    /// Send the request.
    pub async fn send(self) -> Result<(), ApiError> {
        assert!(
            self.ranking_type.is_some(),
            "ranking_type is a required param"
        );

        let query = serde_qs::to_string(&self)?;
        let url = format!("{MANGA_RANKING}?{query}");

        self.client.api_request().get(url, false).await
    }

    /// Send the request.
    #[cfg(feature = "blocking")]
    pub fn send_blocking(self) -> Result<(), ApiError> {
        crate::RUNTIME.block_on(self.send())
    }
}
