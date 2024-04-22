use chrono::prelude::{DateTime, NaiveTime, Utc};
use derive_more::Display as DeriveDisplay;
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use strum::{Display, EnumString, IntoStaticStr};

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
    pub node: AnimeNode,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct SingleMangaItem {
    pub node: MangaNode,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct SingleMangaSerializationItem {
    pub node: MangaSerialization,
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
pub struct MangaSingleList {
    pub data: Vec<SingleMangaItem>,
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
#[serde(default)]
pub struct MangaNode {
    pub id: u32,
    pub title: String,
    pub main_picture: Option<Picture>,
    pub alternative_titles: Option<AlternativeTitles>,
    #[serde(deserialize_with = "date_opt")]
    pub start_date: Option<PartialDate>,
    #[serde(deserialize_with = "date_opt")]
    pub end_date: Option<PartialDate>,
    pub synopsis: Option<String>,
    pub mean: Option<f64>,
    pub rank: Option<u32>,
    pub popularity: Option<u32>,
    pub num_list_users: Option<u32>,
    pub num_scoring_users: Option<u32>,
    pub nsfw: Option<Nsfw>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub media_type: Option<MediaTypeManga>,
    pub status: Option<PublishingStatus>,
    pub genres: Option<Vec<Genre>>,
    pub my_list_status: Option<MangaListStatus>,
    pub num_volumes: Option<u32>,
    pub num_chapters: Option<u32>,
    pub authors: Option<Vec<Author>>,
    pub pictures: Option<Vec<Picture>>,
    pub background: Option<String>,
    pub related_anime: Option<Vec<AnimeRelation>>,
    pub related_manga: Option<Vec<MangaRelation>>,
    pub recommendations: Option<Vec<MangaRecommendation>>,
    pub serialization: Option<Vec<SingleMangaSerializationItem>>,
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
#[serde(default)]
pub struct AnimeNode {
    pub id: u32,
    pub title: String,
    pub main_picture: Option<Picture>,
    pub alternative_titles: Option<AlternativeTitles>,
    #[serde(deserialize_with = "date_opt")]
    pub start_date: Option<PartialDate>,
    #[serde(deserialize_with = "date_opt")]
    pub end_date: Option<PartialDate>,
    pub synopsis: Option<String>,
    pub mean: Option<f64>,
    pub rank: Option<u32>,
    pub popularity: Option<u32>,
    pub num_list_users: Option<u32>,
    pub num_scoring_users: Option<u32>,
    pub nsfw: Option<Nsfw>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
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
    pub node: AnimeNode,
    pub relation_type: RelationType,
    pub relation_type_formatted: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MangaRelation {
    pub node: MangaNode,
    pub relation_type: RelationType,
    pub relation_type_formatted: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AnimeRecommendation {
    pub node: AnimeNode,
    pub num_recommendations: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MangaRecommendation {
    pub node: MangaNode,
    pub num_recommendations: u64,
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
    pub status: AnimeNodeStatus,
    pub num_list_users: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AnimeNodeStatus {
    pub watching: u64,
    pub completed: u64,
    pub on_hold: u64,
    pub dropped: u64,
    pub plan_to_watch: u64,
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
    pub id: u32,
    pub name: String,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct MangaSerialization {
    pub id: u32,
    pub name: String,
}

#[skip_serializing_none]
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct Broadcast {
    pub day_of_the_week: DayOfWeek,
    pub start_time: NaiveTime,
}

#[derive(Copy, Clone, Deserialize, Serialize, Debug, IntoStaticStr, EnumString, PartialEq)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum DayOfWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[derive(Copy, Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct Season {
    pub year: u32,
    pub season: SeasonType,
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
    pub id: u32,
    pub name: GenreType,
}

#[derive(Copy, Clone, Deserialize, Serialize, Debug, IntoStaticStr, EnumString, PartialEq)]
pub enum GenreType {
    // genres
    Action,
    Adventure,
    #[serde(rename = "Avant Garde")]
    AvantGarde,
    #[serde(rename = "Award Winning")]
    AwardWinning,
    #[serde(rename = "Boys Love")]
    BoysLove,
    Comedy,
    Drama,
    Fantasy,
    #[serde(rename = "Girls Love")]
    GirlsLove,
    Gourmet,
    Horror,
    Mystery,
    Romance,
    #[serde(rename = "Sci-Fi")]
    SciFi,
    #[serde(rename = "Slice of Life")]
    SliceOfLife,
    Sports,
    Supernatural,
    Suspense,
    // explicit genres
    Ecchi,
    Erotica,
    Hentai,
    // themes
    #[serde(rename = "Adult Cast")]
    AdultCast,
    Anthropomorphic,
    CGDCT,
    #[serde(rename = "Combat Sports")]
    CombatSports,
    CrossDressing,
    Delinquents,
    Detective,
    Educational,
    #[serde(rename = "Gag Humor")]
    GagHumor,
    Gore,
    Harem,
    #[serde(rename = "High Stakes Game")]
    HighStakesGame,
    Historical,
    #[serde(rename = "Idols (Female)")]
    IdolsFemale,
    #[serde(rename = "Idols (Male)")]
    IdolsMale,
    Isekai,
    Iyashikei,
    #[serde(rename = "Love Polygon")]
    LovePolygon,
    #[serde(rename = "Magical Sex Shift")]
    MagicalSexShift,
    #[serde(rename = "MahouShoujo")]
    MahouShoujo,
    #[serde(rename = "Martial Arts")]
    MartialArts,
    Mecha,
    Medical,
    Military,
    Music,
    Mythology,
    #[serde(rename = "Organized Crime")]
    OrganizedCrime,
    #[serde(rename = "Otaku Culture")]
    OtakuCulture,
    Parody,
    #[serde(rename = "Performing Arts")]
    PerformingArts,
    Psychological,
    Racing,
    Reincarnation,
    #[serde(rename = "Reverse Harem")]
    ReverseHarem,
    #[serde(rename = "Romantic Subtext")]
    RomanticSubtext,
    Samurai,
    School,
    Showbiz,
    Space,
    #[serde(rename = "Strategy Game")]
    StrategyGame,
    #[serde(rename = "Super Power")]
    SuperPower,
    Survival,
    #[serde(rename = "Team Sports")]
    TeamSports,
    #[serde(rename = "Time Travel")]
    TimeTravel,
    Vampire,
    #[serde(rename = "Video Game")]
    VideoGame,
    #[serde(rename = "Visual Arts")]
    VisualArts,
    Workplace,
    // demographics
    Josei,
    Kids,
    Seinen,
    Shoujo,
    Shounen,
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
    pub updated_at: Option<DateTime<Utc>>,
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
    pub updated_at: Option<DateTime<Utc>>,
}

// for parameter input on user animelist
#[skip_serializing_none]
#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct AnimeListItem {
    pub status: WatchStatus,
    pub is_rewatching: bool,
    pub score: u8,
    pub num_episodes_watched: u32,
    pub priority: u8,
    pub num_times_rewatched: u32,
    pub rewatch_value: u8,
    pub tags: Vec<String>,
    pub comments: String,
    pub updated_at: DateTime<Utc>,
    #[serde(deserialize_with = "date")]
    pub start_date: PartialDate,
}

// for parameter input on user mangalist
#[skip_serializing_none]
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct MangaListItem {
    pub status: ReadStatus,
    pub is_rereading: bool,
    pub score: u8,
    pub num_volumes_read: u32,
    pub num_chapters_read: u32,
    pub priority: u8,
    pub num_times_reread: u32,
    pub reread_value: u8,
    pub tags: Vec<String>,
    pub comments: String,
    pub updated_at: DateTime<Utc>,
    #[serde(deserialize_with = "date")]
    pub start_date: PartialDate,
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
    pub joined_at: Option<DateTime<Utc>>,
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
pub enum AnimeRankingType {
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
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum MangaRankingType {
    All,
    Manga,
    Novels,
    OneShots,
    Doujin,
    Manhwa,
    Manhua,
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

#[derive(Clone, Deserialize, Debug, PartialEq, Default)]
pub struct ForumBoards {
    pub categories: Vec<ForumBoard>,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Default)]
pub struct ForumBoard {
    pub title: String,
    pub boards: Vec<Board>,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Default)]
pub struct Board {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub subboards: Vec<SubBoard>,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Default)]
pub struct SubBoard {
    pub id: u32,
    pub title: String,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Default)]
pub struct TopicDetail {
    pub data: Topic,
    pub paging: Paging,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Default)]
pub struct Topic {
    pub title: String,
    pub posts: Vec<Post>,
    pub poll: Poll,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Default)]
pub struct Post {
    pub id: u64,
    pub number: u64,
    pub created_at: DateTime<Utc>,
    pub created_by: ForumUser,
    pub body: String,
    pub signature: String,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Default)]
pub struct ForumUser {
    pub id: u64,
    pub name: String,
    pub forum_avatar: Option<String>,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Default)]
pub struct Poll {
    pub id: u64,
    pub question: String,
    pub closed: bool,
    pub options: Vec<PollOption>,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Default)]
pub struct PollOption {
    pub id: u64,
    pub text: String,
    pub votes: u64,
}

#[derive(Copy, Clone, Serialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ForumSort {
    Recent,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Default)]
pub struct ForumTopics {
    pub data: Vec<ForumTopic>,
    pub paging: Paging,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Default)]
pub struct ForumTopic {
    pub id: u64,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub created_by: ForumUser,
    pub number_of_posts: u32,
    pub last_post_created_at: DateTime<Utc>,
    pub last_post_created_by: ForumUser,
    pub is_locked: bool,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PartialDate {
    pub year: u16,
    pub month: Option<u16>,
    pub day: Option<u16>,
}

fn date_opt<'de, D>(deserializer: D) -> Result<Option<PartialDate>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Some(date(deserializer)?))
}

fn date<'de, D>(deserializer: D) -> Result<PartialDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let num_hyphens = s.chars().filter(|c| *c == '-').count();
    let split = s.split('-').collect::<Vec<_>>();

    let date = PartialDate {
        year: if num_hyphens >= 1 {
            split[0].parse().unwrap()
        } else {
            s.parse().unwrap()
        },
        month: if num_hyphens >= 1 {
            split[1].parse().ok()
        } else {
            None
        },
        day: if num_hyphens == 2 {
            split[2].parse().ok()
        } else {
            None
        },
    };

    Ok(date)
}
