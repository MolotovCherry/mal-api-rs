use const_format::formatcp;
use itertools::Itertools as _;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::{
    api_request::ApiError,
    objects::{
        AnimeList, AnimeNode, AnimeRankingType, AnimeSeasonSort, AnimeSingleList, RankingList,
        SeasonList, SeasonType,
    },
    MalClient, API_URL,
};

const ANIME_URL: &str = formatcp!("{API_URL}/anime");
const ANIME_ID: &str = formatcp!("{API_URL}/anime/{{ANIME_ID}}");
const ANIME_RANKING: &str = formatcp!("{API_URL}/anime/ranking");
const ANIME_SEASON: &str = formatcp!("{API_URL}/anime/season/{{YEAR}}/{{SEASON}}");
const ANIME_SUGGESTIONS: &str = formatcp!("{API_URL}/anime/suggestions");

#[derive(Debug, Clone)]
pub struct AnimeApi<'a> {
    client: &'a MalClient,
}

impl<'a> AnimeApi<'a> {
    pub(crate) fn new(mal_client: &'a MalClient) -> Self {
        Self { client: mal_client }
    }

    /// Anime GET endpoints
    ///
    /// <https://myanimelist.net/apiconfig/references/api/v2#tag/anime>
    pub fn get(&self) -> AnimeApiGet<'a> {
        AnimeApiGet {
            client: self.client,
        }
    }
}

/// Anime GET endpoints
///
/// <https://myanimelist.net/apiconfig/references/api/v2#tag/anime>
#[derive(Debug)]
pub struct AnimeApiGet<'a> {
    client: &'a MalClient,
}

impl<'a> AnimeApiGet<'a> {
    /// GET anime list.
    ///
    /// <https://myanimelist.net/apiconfig/references/api/v2#operation/anime_get>
    pub fn list(self) -> AnimeListGet<'a> {
        AnimeListGet {
            client: self.client,
            q: None,
            limit: None,
            offset: None,
            fields: None,
            nsfw: None,
        }
    }

    /// GET anime details.
    ///
    /// <https://myanimelist.net/apiconfig/references/api/v2#operation/anime_anime_id_get>
    pub fn details(self) -> AnimeDetailsGet<'a> {
        AnimeDetailsGet {
            client: self.client,
            anime_id: None,
            fields: None,
        }
    }

    /// GET anime ranking.
    ///
    /// <https://myanimelist.net/apiconfig/references/api/v2#operation/anime_ranking_get>
    pub fn ranking(self) -> AnimeRankingGet<'a> {
        AnimeRankingGet {
            client: self.client,
            ranking_type: None,
            limit: None,
            offset: None,
            fields: None,
        }
    }

    /// GET seasonal anime.
    ///
    /// <https://myanimelist.net/apiconfig/references/api/v2#operation/anime_season_year_season_get>
    pub fn seasonal(self) -> AnimeSeasonalGet<'a> {
        AnimeSeasonalGet {
            client: self.client,
            year: None,
            season: None,
            sort: None,
            limit: None,
            offset: None,
            fields: None,
            nsfw: None,
        }
    }

    /// GET suggested anime.
    ///
    /// <https://myanimelist.net/apiconfig/references/api/v2#operation/anime_suggestions_get>
    pub fn suggested(self) -> AnimeSuggestedGet<'a> {
        AnimeSuggestedGet {
            client: self.client,
            limit: None,
            offset: None,
            fields: None,
            nsfw: None,
        }
    }
}

/// GET anime list.
///
/// <https://myanimelist.net/apiconfig/references/api/v2#operation/anime_get>
#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct AnimeListGet<'a> {
    #[serde(skip)]
    client: &'a MalClient,

    q: Option<String>,
    limit: Option<u32>,
    offset: Option<u32>,
    fields: Option<String>,
    nsfw: Option<bool>,
}

impl<'a> AnimeListGet<'a> {
    /// Search.
    pub fn q(mut self, q: &str) -> Self {
        self.q = Some(q.to_owned());
        self
    }

    /// Default: 100
    /// The maximum value is 100.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Default: 0
    pub fn offset(mut self, offset: u32) -> Self {
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
        let query = serde_qs::to_string(&self)?;

        let url = format!("{ANIME_URL}?{query}");

        self.client.api_request().get(url, false).await
    }

    /// Send the request.
    #[cfg(feature = "blocking")]
    pub fn send_blocking(self) -> Result<AnimeList, ApiError> {
        crate::RUNTIME.block_on(self.send())
    }
}

/// GET anime details.
///
/// <https://myanimelist.net/apiconfig/references/api/v2#operation/anime_anime_id_get>
#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct AnimeDetailsGet<'a> {
    #[serde(skip)]
    client: &'a MalClient,
    #[serde(skip)]
    anime_id: Option<u64>,

    fields: Option<String>,
}

impl<'a> AnimeDetailsGet<'a> {
    /// The anime id.
    pub fn anime_id(mut self, id: u64) -> Self {
        self.anime_id = Some(id);
        self
    }

    pub fn fields<I: IntoIterator<Item = impl AsRef<str>>>(mut self, fields: I) -> Self {
        let fields = fields.into_iter().map(|f| f.as_ref().to_string()).join(",");

        self.fields = Some(fields);
        self
    }

    /// Send the request.
    pub async fn send(self) -> Result<AnimeNode, ApiError> {
        assert!(self.anime_id.is_some(), "anime_id is a required param");

        let url = ANIME_ID.replace("{ANIME_ID}", &self.anime_id.unwrap().to_string());
        let query = serde_qs::to_string(&self)?;
        let url = format!("{url}?{query}");

        self.client.api_request().get(url, false).await
    }

    /// Send the request.
    #[cfg(feature = "blocking")]
    pub fn send_blocking(self) -> Result<AnimeNode, ApiError> {
        crate::RUNTIME.block_on(self.send())
    }
}

/// GET anime ranking.
///
/// <https://myanimelist.net/apiconfig/references/api/v2#operation/anime_ranking_get>
#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct AnimeRankingGet<'a> {
    #[serde(skip)]
    client: &'a MalClient,

    ranking_type: Option<AnimeRankingType>,
    limit: Option<u16>,
    offset: Option<u64>,
    fields: Option<String>,
}

impl<'a> AnimeRankingGet<'a> {
    /// The ranking type. This parameter is required.
    pub fn ranking_type(mut self, ranking_type: AnimeRankingType) -> Self {
        self.ranking_type = Some(ranking_type);
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
    pub async fn send(self) -> Result<RankingList, ApiError> {
        assert!(
            self.ranking_type.is_some(),
            "ranking_type is a required param"
        );

        let query = serde_qs::to_string(&self)?;
        let url = format!("{ANIME_RANKING}?{query}");

        self.client.api_request().get(url, false).await
    }

    /// Send the request.
    #[cfg(feature = "blocking")]
    pub fn send_blocking(self) -> Result<RankingList, ApiError> {
        crate::RUNTIME.block_on(self.send())
    }
}

/// GET seasonal anime.
///
/// <https://myanimelist.net/apiconfig/references/api/v2#operation/anime_season_year_season_get>
#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct AnimeSeasonalGet<'a> {
    #[serde(skip)]
    client: &'a MalClient,
    #[serde(skip)]
    year: Option<u16>,
    #[serde(skip)]
    season: Option<SeasonType>,

    sort: Option<AnimeSeasonSort>,
    limit: Option<u16>,
    offset: Option<u64>,
    fields: Option<String>,
    nsfw: Option<bool>,
}

impl<'a> AnimeSeasonalGet<'a> {
    /// Year. This parameter is required.
    pub fn year(mut self, year: u16) -> Self {
        self.year = Some(year);
        self
    }

    /// Season. This parameter is required.
    pub fn season(mut self, season: SeasonType) -> Self {
        self.season = Some(season);
        self
    }

    pub fn sort(mut self, sort: AnimeSeasonSort) -> Self {
        self.sort = Some(sort);
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

    /// Whether to return nsfw material.
    pub fn nsfw(mut self, nsfw: bool) -> Self {
        self.nsfw = Some(nsfw);
        self
    }

    /// Send the request.
    pub async fn send(self) -> Result<SeasonList, ApiError> {
        assert!(self.year.is_some(), "year is a required param");
        assert!(self.season.is_some(), "season is a required param");

        let query = serde_qs::to_string(&self)?;
        let season: &str = self.season.unwrap().into();
        let url = ANIME_SEASON
            .replace("{YEAR}", &self.year.unwrap().to_string())
            .replace("{SEASON}", season);

        let url = format!("{url}?{query}");

        self.client.api_request().get(url, false).await
    }

    /// Send the request.
    #[cfg(feature = "blocking")]
    pub fn send_blocking(self) -> Result<SeasonList, ApiError> {
        crate::RUNTIME.block_on(self.send())
    }
}

/// GET suggested anime.
///
/// <https://myanimelist.net/apiconfig/references/api/v2#operation/anime_suggestions_get>
#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct AnimeSuggestedGet<'a> {
    #[serde(skip)]
    client: &'a MalClient,

    limit: Option<u16>,
    offset: Option<u64>,
    fields: Option<String>,
    nsfw: Option<bool>,
}

impl<'a> AnimeSuggestedGet<'a> {
    /// Default: 100
    /// The maximum value is 100.
    pub fn limit(mut self, limit: u16) -> Self {
        self.limit = Some(limit.clamp(0, 100));
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
    pub async fn send(self) -> Result<AnimeSingleList, ApiError> {
        let query = serde_qs::to_string(&self)?;
        let url = format!("{ANIME_SUGGESTIONS}?{query}");
        self.client.api_request().get(url, true).await
    }

    /// Send the request.
    #[cfg(feature = "blocking")]
    pub fn send_blocking(self) -> Result<AnimeSingleList, ApiError> {
        crate::RUNTIME.block_on(self.send())
    }
}
