use const_format::formatcp;
use itertools::Itertools as _;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::objects::{
    AnimeList, AnimeNode, AnimeSeasonSort, AnimeSingleList, RankingList, RankingType, SeasonList,
    SeasonType,
};
use crate::API_URL;
use crate::{api_request::ApiError, MalClient};

const ANIME_URL: &str = formatcp!("{API_URL}/anime");
const ANIME_ID: &str = formatcp!("{API_URL}/anime/{{ANIME_ID}}");
const ANIME_RANKING: &str = formatcp!("{API_URL}/anime/ranking");
const ANIME_SEASON: &str = formatcp!("{API_URL}/anime/season/{{YEAR}}/{{SEASON}}");
const ANIME_SUGGESTIONS: &str = formatcp!("{API_URL}/anime/suggestions");

#[derive(Debug, Clone)]
pub struct AnimeApi {
    client: MalClient,
}

impl AnimeApi {
    pub(crate) fn new(mal_client: MalClient) -> Self {
        Self { client: mal_client }
    }

    pub fn get(&self) -> AnimeApiGet {
        AnimeApiGet {
            client: self.client.clone(),
        }
    }
}

#[derive(Debug)]
pub struct AnimeApiGet {
    client: MalClient,
}

impl AnimeApiGet {
    pub fn list(self) -> AnimeListGet {
        AnimeListGet {
            client: self.client,
            q: None,
            limit: None,
            offset: None,
            fields: None,
        }
    }

    pub fn details(self) -> AnimeDetailsGet {
        AnimeDetailsGet {
            client: self.client,
            anime_id: None,
            fields: None,
        }
    }

    pub fn ranking(self) -> AnimeRankingGet {
        AnimeRankingGet {
            client: self.client,
            ranking_type: None,
            limit: None,
            offset: None,
            fields: None,
        }
    }

    pub fn seasonal(self) -> AnimeSeasonalGet {
        AnimeSeasonalGet {
            client: self.client,
            year: None,
            season: None,
            sort: None,
            limit: None,
            offset: None,
            fields: None,
        }
    }

    pub fn suggested(self) -> AnimeSuggestedGet {
        AnimeSuggestedGet {
            client: self.client,
            limit: None,
            offset: None,
            fields: None,
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct AnimeListGet {
    #[serde(skip)]
    client: MalClient,

    q: Option<String>,
    limit: Option<u32>,
    offset: Option<u32>,
    fields: Option<String>,
}

impl AnimeListGet {
    pub fn q(mut self, q: &str) -> Self {
        self.q = Some(q.to_owned());
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn fields<I: IntoIterator<Item = impl AsRef<str>>>(mut self, fields: I) -> Self {
        let fields = fields.into_iter().map(|f| f.as_ref().to_string()).join(",");

        self.fields = Some(fields);
        self
    }

    pub async fn send(self) -> Result<AnimeList, ApiError> {
        let query = serde_qs::to_string(&self)?;

        let url = format!("{ANIME_URL}?{query}");

        self.client.http.get(url, false).await
    }
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct AnimeDetailsGet {
    #[serde(skip)]
    client: MalClient,
    #[serde(skip)]
    anime_id: Option<u64>,

    fields: Option<String>,
}

impl AnimeDetailsGet {
    pub fn anime_id(mut self, id: u64) -> Self {
        self.anime_id = Some(id);
        self
    }

    pub fn fields<I: IntoIterator<Item = impl AsRef<str>>>(mut self, fields: I) -> Self {
        let fields = fields.into_iter().map(|f| f.as_ref().to_string()).join(",");

        self.fields = Some(fields);
        self
    }

    pub async fn send(self) -> Result<AnimeNode, ApiError> {
        assert!(self.anime_id.is_some(), "anime_id is a required param");

        let url = ANIME_ID.replace("{ANIME_ID}", &self.anime_id.unwrap().to_string());
        let query = serde_qs::to_string(&self)?;
        let url = format!("{url}?{query}");

        self.client.http.get(url, false).await
    }
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct AnimeRankingGet {
    #[serde(skip)]
    client: MalClient,

    ranking_type: Option<RankingType>,
    limit: Option<u16>,
    offset: Option<u64>,
    fields: Option<String>,
}

impl AnimeRankingGet {
    pub fn ranking_type(mut self, ranking_type: RankingType) -> Self {
        self.ranking_type = Some(ranking_type);
        self
    }

    pub fn limit(mut self, limit: u16) -> Self {
        self.limit = Some(limit.clamp(0, 500));
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

    pub async fn send(self) -> Result<RankingList, ApiError> {
        self.client.http.get(ANIME_RANKING, false).await
    }
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct AnimeSeasonalGet {
    #[serde(skip)]
    client: MalClient,
    #[serde(skip)]
    year: Option<u16>,
    #[serde(skip)]
    season: Option<SeasonType>,

    sort: Option<AnimeSeasonSort>,
    limit: Option<u16>,
    offset: Option<u64>,
    fields: Option<String>,
}

impl AnimeSeasonalGet {
    pub fn year(mut self, year: u16) -> Self {
        self.year = Some(year);
        self
    }

    pub fn season(mut self, season: SeasonType) -> Self {
        self.season = Some(season);
        self
    }

    pub fn sort(mut self, sort: AnimeSeasonSort) -> Self {
        self.sort = Some(sort);
        self
    }

    pub fn limit(mut self, limit: u16) -> Self {
        self.limit = Some(limit.clamp(0, 500));
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

    pub async fn send(self) -> Result<SeasonList, ApiError> {
        assert!(self.year.is_some(), "year is a required param");
        assert!(self.season.is_some(), "season is a required param");

        let query = serde_qs::to_string(&self)?;
        let season: &str = self.season.unwrap().into();
        let url = ANIME_SEASON
            .replace("{YEAR}", &self.year.unwrap().to_string())
            .replace("{SEASON}", season);

        let url = format!("{url}?{query}");

        self.client.http.get(url, false).await
    }
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct AnimeSuggestedGet {
    #[serde(skip)]
    client: MalClient,

    limit: Option<u16>,
    offset: Option<u64>,
    fields: Option<String>,
}

impl AnimeSuggestedGet {
    pub fn limit(mut self, limit: u16) -> Self {
        self.limit = Some(limit.clamp(0, 100));
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

    pub async fn send(self) -> Result<AnimeSingleList, ApiError> {
        let query = serde_qs::to_string(&self)?;
        let url = format!("{ANIME_SUGGESTIONS}?{query}");
        self.client.http.get(url, true).await
    }
}
