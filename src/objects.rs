use derive_more::Display as DeriveDisplay;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum_macros::{Display, EnumString, IntoStaticStr};

#[derive(Clone, Debug, Deserialize, Serialize, DeriveDisplay, PartialEq)]
pub enum Username {
    #[display(fmt = "@me")]
    #[serde(rename = "@me")]
    Me,
    #[display(fmt = "{}", _0)]
    User(String),
}

#[derive(
    Copy, Clone, Debug, Deserialize, Serialize, IntoStaticStr, EnumString, Display, PartialEq,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum MangaSort {
    ListScore,
    ListUpdatedAt,
    MangaTitle,
    MangaStartDate,
    MangaId,
}

#[derive(
    Copy, Clone, Debug, Deserialize, Serialize, IntoStaticStr, EnumString, Display, PartialEq,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum AnimeSort {
    ListScore,
    ListUpdatedAt,
    AnimeTitle,
    AnimeStartDate,
    AnimeId,
}

#[derive(
    Copy, Clone, Deserialize, Serialize, Debug, IntoStaticStr, EnumString, Display, PartialEq,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum WatchStatus {
    Watching,
    Completed,
    OnHold,
    Dropped,
    PlanToWatch,
}

#[derive(
    Copy, Clone, Deserialize, Serialize, Debug, IntoStaticStr, EnumString, Display, PartialEq,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ReadStatus {
    Reading,
    Completed,
    OnHold,
    Dropped,
    PlanToRead,
}

#[skip_serializing_none]
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Default)]
pub struct RankingList {
    pub data: Vec<MangaRankItem>,
    pub paging: Option<Paging>,
}

#[skip_serializing_none]
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Default)]
pub struct SeasonList {
    pub data: Vec<SingleAnimeItem>,
    pub paging: Option<Paging>,
    pub season: Option<Season>,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct SingleAnimeItem {
    node: AnimeNode,
}

#[skip_serializing_none]
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Default)]
pub struct MangaList {
    pub data: Vec<MangaItem>,
    pub paging: Option<Paging>,
}

#[skip_serializing_none]
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Default)]
pub struct AnimeList {
    pub data: Vec<AnimeItem>,
    pub paging: Option<Paging>,
}

#[skip_serializing_none]
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Default)]
pub struct AnimeSingleList {
    pub data: Vec<SingleAnimeItem>,
    pub paging: Option<Paging>,
}

#[skip_serializing_none]
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Default)]
pub struct Paging {
    pub previous: Option<String>,
    pub next: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Default)]
pub struct MangaItem {
    pub node: MangaNode,
    pub list_status: Option<MangaListStatus>,
}

#[skip_serializing_none]
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Default)]
pub struct MangaRankItem {
    pub node: MangaNode,
    pub ranking: Rank,
}

#[skip_serializing_none]
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Default)]
pub struct Rank {
    pub rank: u64,
}

#[skip_serializing_none]
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Default)]
pub struct AnimeItem {
    pub node: AnimeNode,
    pub list_status: Option<AnimeListStatus>,
}

#[skip_serializing_none]
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Default)]
pub struct MangaNode {
    pub id: u32,
    pub title: String,
    pub main_picture: Option<Picture>,
    pub alternative_titles: Option<AlternativeTitles>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub synopsis: Option<String>,
    pub mean: Option<f64>,
    pub rank: Option<u32>,
    pub popularity: Option<u32>,
    pub num_list_users: Option<u32>,
    pub num_scoring_users: Option<u32>,
    pub nsfw: Option<Nsfw>,
    pub genres: Option<Vec<Genre>>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub media_type: Option<MediaTypeManga>,
    pub status: Option<PublishingStatus>,
    pub my_list_status: Option<MangaListStatus>,
    pub num_volumes: Option<u32>,
    pub num_chapters: Option<u32>,
    pub authors: Option<Vec<Author>>,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct Author {
    node: Person,
    role: String,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct Person {
    id: u32,
    first_name: String,
    last_name: String,
}

#[skip_serializing_none]
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Default)]
pub struct AnimeNode {
    pub id: u32,
    pub title: String,
    pub main_picture: Option<Picture>,
    pub alternative_titles: Option<AlternativeTitles>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub synopsis: Option<String>,
    pub mean: Option<f64>,
    pub rank: Option<u32>,
    pub popularity: Option<u32>,
    pub num_list_users: Option<u32>,
    pub num_scoring_users: Option<u32>,
    pub nsfw: Option<Nsfw>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub media_type: Option<MediaTypeAnime>,
    pub status: Option<AiringStatus>,
    pub genres: Option<Vec<Genre>>,
    pub my_list_status: Option<AnimeListStatus>,
    pub num_episodes: Option<u32>,
    pub start_season: Option<Season>,
    pub broadcast: Option<Broadcast>,
    pub source: Option<Source>,
    pub average_episode_duration: Option<u32>,
    pub rating: Option<Rating>,
    pub studios: Option<Vec<Studio>>,
    pub pictures: Option<Vec<Picture>>,
    pub background: Option<String>,
    pub related_anime: Option<Vec<AnimeRelation>>,
    pub relation_type: Option<RelationType>,
    pub statistics: Option<AnimeNodeStatistics>,
    pub recommendations: Option<Vec<AnimeRecommendation>>,
    pub related_manga: Option<Vec<MangaRelation>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AnimeRelation {
    node: AnimeNode,
    relation_type: RelationType,
    relation_type_formatted: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MangaRelation {
    node: MangaNode,
    relation_type: RelationType,
    relation_type_formatted: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AnimeRecommendation {
    node: AnimeNode,
    num_recommendations: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RelationType {
    Prequel,
    Sequel,
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AnimeNodeStatistics {
    status: AnimeNodeStatus,
    num_list_users: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AnimeNodeStatus {
    watching: u64,
    completed: u64,
    on_hold: u64,
    dropped: u64,
    plan_to_watch: u64,
}

#[derive(Copy, Clone, Deserialize, Serialize, Debug, IntoStaticStr, EnumString, PartialEq)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Source {
    Other,
    Original,
    Manga,
    #[serde(rename = "4_koma_manga")]
    #[strum(serialize = "4_koma_manga")]
    FourKomaManga,
    WebManga,
    DigitalManga,
    Novel,
    LightNovel,
    VisualNovel,
    Game,
    CardGame,
    Book,
    PictureBook,
    Radio,
    Music,
}

#[derive(Copy, Clone, Deserialize, Serialize, Debug, IntoStaticStr, EnumString, PartialEq)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Rating {
    G,
    PG,
    #[strum(serialize = "pg_13")]
    #[serde(rename = "pg_13")]
    PG13,
    R,
    #[strum(serialize = "r+")]
    #[serde(rename = "r+")]
    RPlus,
    RX,
}

#[derive(Copy, Clone, Deserialize, Serialize, Debug, IntoStaticStr, EnumString, PartialEq)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum MediaTypeAnime {
    Unknown,
    TV,
    Ova,
    Movie,
    Special,
    Ona,
    Music,
}

#[derive(Copy, Clone, Deserialize, Serialize, Debug, IntoStaticStr, EnumString, PartialEq)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum MediaTypeManga {
    Unknown,
    Manga,
    Novel,
    OneShot,
    Doujinshi,
    Manhwa,
    Manhua,
    Oel,
}

#[derive(Copy, Clone, Deserialize, Serialize, Debug, IntoStaticStr, EnumString, PartialEq)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Nsfw {
    White,
    Gray,
    Black,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct Studio {
    id: u32,
    name: String,
}

#[skip_serializing_none]
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Default)]
pub struct Broadcast {
    day_of_the_week: String,
    start_time: Option<String>,
}

#[derive(Copy, Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct Season {
    year: u32,
    season: SeasonType,
}

#[derive(Copy, Clone, Deserialize, Serialize, Debug, IntoStaticStr, EnumString, PartialEq)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum SeasonType {
    Winter,
    Spring,
    Summer,
    Fall,
}

#[derive(Copy, Clone, Deserialize, Serialize, Debug, IntoStaticStr, EnumString, PartialEq)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum AiringStatus {
    FinishedAiring,
    CurrentlyAiring,
    NotYetAired,
}

#[derive(Copy, Clone, Deserialize, Serialize, Debug, IntoStaticStr, EnumString, PartialEq)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PublishingStatus {
    Finished,
    CurrentlyPublishing,
    NotYetPublished,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct Genre {
    id: u32,
    name: String,
}

#[skip_serializing_none]
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Default)]
pub struct AlternativeTitles {
    pub synonyms: Option<Vec<String>>,
    pub en: Option<String>,
    pub ja: Option<String>,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct Picture {
    pub medium: String,
    pub large: String,
}

#[skip_serializing_none]
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Default)]
pub struct AnimeListStatus {
    pub status: Option<WatchStatus>,
    pub score: Option<u32>,
    pub num_episodes_watched: Option<u32>,
    pub is_rewatching: Option<bool>,
    pub start_date: Option<String>,
    pub finish_date: Option<String>,
    pub priority: Option<u32>,
    pub num_times_rewatched: Option<u32>,
    pub rewatch_value: Option<u32>,
    pub tags: Option<Vec<String>>,
    pub comments: Option<String>,
    pub updated_at: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Default)]
pub struct MangaListStatus {
    pub status: Option<ReadStatus>,
    pub score: Option<u32>,
    pub num_episodes_watched: Option<u32>,
    pub num_volumes_read: Option<u32>,
    pub num_chapters_read: Option<u32>,
    pub is_rereading: Option<bool>,
    pub start_date: Option<String>,
    pub finish_date: Option<String>,
    pub priority: Option<u32>,
    pub num_times_reread: Option<u32>,
    pub reread_value: Option<u32>,
    pub tags: Option<Vec<String>>,
    pub comments: Option<String>,
    pub updated_at: Option<String>,
}

// for parameter input on user animelist
#[skip_serializing_none]
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct AnimeListItem {
    pub status: Option<WatchStatus>,
    pub is_rewatching: Option<bool>,
    pub score: Option<u8>,
    pub num_watched_episodes: Option<u32>,
    pub priority: Option<u8>,
    pub num_times_rewatched: Option<u32>,
    pub rewatch_value: Option<u8>,
    pub tags: Option<String>,
    pub comments: Option<String>,
}

// for parameter input on user mangalist
#[skip_serializing_none]
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct MangaListItem {
    pub status: Option<ReadStatus>,
    pub is_rereading: Option<bool>,
    pub score: Option<u8>,
    pub num_volumes_read: Option<u32>,
    pub num_chapters_read: Option<u32>,
    pub priority: Option<u8>,
    pub num_times_reread: Option<u32>,
    pub reread_value: Option<u8>,
    pub tags: Option<String>,
    pub comments: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Default)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub picture: String,
    pub gender: Option<String>,
    pub birthday: Option<String>,
    pub location: Option<String>,
    pub joined_at: Option<String>,
    pub time_zone: Option<String>,
    pub is_supporter: Option<bool>,
    pub anime_statistics: Option<AnimeStatistics>,
}

#[skip_serializing_none]
#[derive(Copy, Clone, Deserialize, Serialize, Debug, PartialEq, Default)]
pub struct AnimeStatistics {
    pub num_items_watching: Option<u32>,
    pub num_items_completed: Option<u32>,
    pub num_items_on_hold: Option<u32>,
    pub num_items_dropped: Option<u32>,
    pub num_items_plan_to_watch: Option<u32>,
    pub num_items: Option<u32>,
    pub num_days_watched: Option<f64>,
    pub num_days_watching: Option<f64>,
    pub num_days_completed: Option<f64>,
    pub num_days_on_hold: Option<u32>,
    pub num_days_dropped: Option<u32>,
    pub num_days: Option<f64>,
    pub num_episodes: Option<u32>,
    pub num_times_rewatched: Option<u32>,
    pub mean_score: Option<f64>,
}

#[derive(Copy, Clone, Deserialize, Serialize, Debug, IntoStaticStr, EnumString, PartialEq)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum RankingType {
    All,
    Airing,
    Upcoming,
    Tv,
    Ova,
    Movie,
    Special,
    ByPopularity,
    Favorite,
}

#[derive(Copy, Clone, Deserialize, Serialize, Debug, IntoStaticStr, EnumString, PartialEq)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum AnimeSeasonSort {
    AnimeScore,
    AnimeNumListUsers,
}
