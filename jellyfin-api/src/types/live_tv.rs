use super::*;

#[doc = "Channel mapping options dto."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ChannelMappingOptionsDto {
    #[doc = "Gets or sets list of mappings."]
    #[serde(
        rename = "Mappings",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub mappings: Vec<NameValuePair>,
    #[doc = "Gets or sets list of provider channels."]
    #[serde(
        rename = "ProviderChannels",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub provider_channels: Vec<NameIdPair>,
    #[doc = "Gets or sets provider name."]
    #[serde(
        rename = "ProviderName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provider_name: Option<String>,
    #[doc = "Gets or sets list of tuner channels."]
    #[serde(
        rename = "TunerChannels",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tuner_channels: Vec<TunerChannelMapping>,
}

impl Default for ChannelMappingOptionsDto {
    fn default() -> Self {
        Self {
            mappings: Default::default(),
            provider_channels: Default::default(),
            provider_name: Default::default(),
            tuner_channels: Default::default(),
        }
    }
}

#[doc = "Get programs dto."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct GetProgramsDto {
    #[doc = "Gets or sets the channels to return guide information for."]
    #[serde(
        rename = "ChannelIds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_ids: Option<Vec<uuid::Uuid>>,
    #[doc = "Gets or sets the image types to include in the output."]
    #[serde(
        rename = "EnableImageTypes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_image_types: Option<Vec<ImageType>>,
    #[doc = "Gets or sets include image information in output."]
    #[serde(
        rename = "EnableImages",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_images: Option<bool>,
    #[doc = "Gets or sets a value indicating whether retrieve total record count."]
    #[serde(
        rename = "EnableTotalRecordCount",
        default = "crate::types::defaults::default_bool::<true>"
    )]
    pub enable_total_record_count: bool,
    #[doc = "Gets or sets include user data."]
    #[serde(
        rename = "EnableUserData",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_user_data: Option<bool>,
    #[doc = "Gets or sets specify additional fields of information to return in the output."]
    #[serde(
        rename = "Fields",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fields: Option<Vec<ItemFields>>,
    #[doc = "Gets or sets the genre ids to return guide information for."]
    #[serde(
        rename = "GenreIds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub genre_ids: Option<Vec<uuid::Uuid>>,
    #[doc = "Gets or sets the genres to return guide information for."]
    #[serde(
        rename = "Genres",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub genres: Option<Vec<String>>,
    #[doc = "Gets or sets filter by programs that have completed airing, or not."]
    #[serde(
        rename = "HasAired",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub has_aired: Option<bool>,
    #[doc = "Gets or sets the max number of images to return, per image type."]
    #[serde(
        rename = "ImageTypeLimit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub image_type_limit: Option<i32>,
    #[doc = "Gets or sets filter by programs that are currently airing, or not."]
    #[serde(
        rename = "IsAiring",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_airing: Option<bool>,
    #[doc = "Gets or sets filter for kids."]
    #[serde(
        rename = "IsKids",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_kids: Option<bool>,
    #[doc = "Gets or sets filter for movies."]
    #[serde(
        rename = "IsMovie",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_movie: Option<bool>,
    #[doc = "Gets or sets filter for news."]
    #[serde(
        rename = "IsNews",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_news: Option<bool>,
    #[doc = "Gets or sets filter for series."]
    #[serde(
        rename = "IsSeries",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_series: Option<bool>,
    #[doc = "Gets or sets filter for sports."]
    #[serde(
        rename = "IsSports",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_sports: Option<bool>,
    #[doc = "Gets or sets filter by library series id."]
    #[serde(
        rename = "LibrarySeriesId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub library_series_id: Option<uuid::Uuid>,
    #[doc = "Gets or sets the maximum number of records to return."]
    #[serde(
        rename = "Limit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub limit: Option<i32>,
    #[doc = "Gets or sets the maximum premiere end date."]
    #[serde(
        rename = "MaxEndDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_end_date: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets the maximum premiere start date."]
    #[serde(
        rename = "MaxStartDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_start_date: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets the minimum premiere end date."]
    #[serde(
        rename = "MinEndDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub min_end_date: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets the minimum premiere start date."]
    #[serde(
        rename = "MinStartDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub min_start_date: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets filter by series timer id."]
    #[serde(
        rename = "SeriesTimerId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub series_timer_id: Option<String>,
    #[doc = "Gets or sets specify one or more sort orders, comma delimited. Options: Name, StartDate."]
    #[serde(
        rename = "SortBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sort_by: Option<Vec<ItemSortBy>>,
    #[doc = "Gets or sets sort order."]
    #[serde(
        rename = "SortOrder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sort_order: Option<Vec<SortOrder>>,
    #[doc = "Gets or sets the record index to start at. All items with a lower index will be dropped from the results."]
    #[serde(
        rename = "StartIndex",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_index: Option<i32>,
    #[doc = "Gets or sets optional. Filter by user id."]
    #[serde(
        rename = "UserId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_id: Option<uuid::Uuid>,
}

impl Default for GetProgramsDto {
    fn default() -> Self {
        Self {
            channel_ids: Default::default(),
            enable_image_types: Default::default(),
            enable_images: Default::default(),
            enable_total_record_count: super::defaults::default_bool::<true>(),
            enable_user_data: Default::default(),
            fields: Default::default(),
            genre_ids: Default::default(),
            genres: Default::default(),
            has_aired: Default::default(),
            image_type_limit: Default::default(),
            is_airing: Default::default(),
            is_kids: Default::default(),
            is_movie: Default::default(),
            is_news: Default::default(),
            is_series: Default::default(),
            is_sports: Default::default(),
            library_series_id: Default::default(),
            limit: Default::default(),
            max_end_date: Default::default(),
            max_start_date: Default::default(),
            min_end_date: Default::default(),
            min_start_date: Default::default(),
            series_timer_id: Default::default(),
            sort_by: Default::default(),
            sort_order: Default::default(),
            start_index: Default::default(),
            user_id: Default::default(),
        }
    }
}

#[doc = "`GuideInfo`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct GuideInfo {
    #[doc = "Gets or sets the end date."]
    #[serde(
        rename = "EndDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets the start date."]
    #[serde(
        rename = "StartDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
}

impl Default for GuideInfo {
    fn default() -> Self {
        Self {
            end_date: Default::default(),
            start_date: Default::default(),
        }
    }
}

#[doc = "`ListingsProviderInfo`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ListingsProviderInfo {
    #[serde(
        rename = "ChannelMappings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_mappings: Option<Vec<NameValuePair>>,
    #[serde(
        rename = "Country",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub country: Option<String>,
    #[serde(
        rename = "EnableAllTuners",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_all_tuners: Option<bool>,
    #[serde(
        rename = "EnabledTuners",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enabled_tuners: Option<Vec<String>>,
    #[serde(
        rename = "Id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<String>,
    #[serde(
        rename = "KidsCategories",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub kids_categories: Option<Vec<String>>,
    #[serde(
        rename = "ListingsId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub listings_id: Option<String>,
    #[serde(
        rename = "MovieCategories",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub movie_categories: Option<Vec<String>>,
    #[serde(
        rename = "MoviePrefix",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub movie_prefix: Option<String>,
    #[serde(
        rename = "NewsCategories",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub news_categories: Option<Vec<String>>,
    #[serde(
        rename = "Password",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub password: Option<String>,
    #[serde(
        rename = "Path",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub path: Option<String>,
    #[serde(
        rename = "PreferredLanguage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub preferred_language: Option<String>,
    #[serde(
        rename = "SportsCategories",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sports_categories: Option<Vec<String>>,
    #[serde(
        rename = "Type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_: Option<String>,
    #[serde(
        rename = "UserAgent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_agent: Option<String>,
    #[serde(
        rename = "Username",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub username: Option<String>,
    #[serde(
        rename = "ZipCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub zip_code: Option<String>,
}

impl Default for ListingsProviderInfo {
    fn default() -> Self {
        Self {
            channel_mappings: Default::default(),
            country: Default::default(),
            enable_all_tuners: Default::default(),
            enabled_tuners: Default::default(),
            id: Default::default(),
            kids_categories: Default::default(),
            listings_id: Default::default(),
            movie_categories: Default::default(),
            movie_prefix: Default::default(),
            news_categories: Default::default(),
            password: Default::default(),
            path: Default::default(),
            preferred_language: Default::default(),
            sports_categories: Default::default(),
            type_: Default::default(),
            user_agent: Default::default(),
            username: Default::default(),
            zip_code: Default::default(),
        }
    }
}

#[doc = "`LiveTvInfo`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct LiveTvInfo {
    #[doc = "Gets or sets the enabled users."]
    #[serde(
        rename = "EnabledUsers",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub enabled_users: Vec<String>,
    #[doc = "Gets or sets a value indicating whether this instance is enabled."]
    #[serde(
        rename = "IsEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_enabled: Option<bool>,
    #[doc = "Gets or sets the services."]
    #[serde(
        rename = "Services",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub services: Vec<LiveTvServiceInfo>,
}

impl Default for LiveTvInfo {
    fn default() -> Self {
        Self {
            enabled_users: Default::default(),
            is_enabled: Default::default(),
            services: Default::default(),
        }
    }
}

#[doc = "`LiveTvOptions`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct LiveTvOptions {
    #[serde(
        rename = "EnableOriginalAudioWithEncodedRecordings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_original_audio_with_encoded_recordings: Option<bool>,
    #[serde(
        rename = "EnableRecordingSubfolders",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_recording_subfolders: Option<bool>,
    #[serde(
        rename = "GuideDays",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guide_days: Option<i32>,
    #[serde(
        rename = "ListingProviders",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub listing_providers: Option<Vec<ListingsProviderInfo>>,
    #[serde(
        rename = "MediaLocationsCreated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub media_locations_created: Option<Vec<String>>,
    #[serde(
        rename = "MovieRecordingPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub movie_recording_path: Option<String>,
    #[serde(
        rename = "PostPaddingSeconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub post_padding_seconds: Option<i32>,
    #[serde(
        rename = "PrePaddingSeconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pre_padding_seconds: Option<i32>,
    #[serde(
        rename = "RecordingPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub recording_path: Option<String>,
    #[serde(
        rename = "RecordingPostProcessor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub recording_post_processor: Option<String>,
    #[serde(
        rename = "RecordingPostProcessorArguments",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub recording_post_processor_arguments: Option<String>,
    #[serde(
        rename = "SaveRecordingImages",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub save_recording_images: Option<bool>,
    #[serde(
        rename = "SaveRecordingNFO",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub save_recording_nfo: Option<bool>,
    #[serde(
        rename = "SeriesRecordingPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub series_recording_path: Option<String>,
    #[serde(
        rename = "TunerHosts",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tuner_hosts: Option<Vec<TunerHostInfo>>,
}

impl Default for LiveTvOptions {
    fn default() -> Self {
        Self {
            enable_original_audio_with_encoded_recordings: Default::default(),
            enable_recording_subfolders: Default::default(),
            guide_days: Default::default(),
            listing_providers: Default::default(),
            media_locations_created: Default::default(),
            movie_recording_path: Default::default(),
            post_padding_seconds: Default::default(),
            pre_padding_seconds: Default::default(),
            recording_path: Default::default(),
            recording_post_processor: Default::default(),
            recording_post_processor_arguments: Default::default(),
            save_recording_images: Default::default(),
            save_recording_nfo: Default::default(),
            series_recording_path: Default::default(),
            tuner_hosts: Default::default(),
        }
    }
}

#[doc = "Class ServiceInfo."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct LiveTvServiceInfo {
    #[doc = "Gets or sets a value indicating whether this instance has update available."]
    #[serde(
        rename = "HasUpdateAvailable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub has_update_available: Option<bool>,
    #[doc = "Gets or sets the home page URL."]
    #[serde(
        rename = "HomePageUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub home_page_url: Option<String>,
    #[doc = "Gets or sets a value indicating whether this instance is visible."]
    #[serde(
        rename = "IsVisible",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_visible: Option<bool>,
    #[doc = "Gets or sets the name."]
    #[serde(
        rename = "Name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,
    #[serde(
        rename = "Status",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub status: Option<LiveTvServiceStatus>,
    #[doc = "Gets or sets the status message."]
    #[serde(
        rename = "StatusMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub status_message: Option<String>,
    #[serde(
        rename = "Tuners",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tuners: Option<Vec<String>>,
    #[doc = "Gets or sets the version."]
    #[serde(
        rename = "Version",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub version: Option<String>,
}

impl Default for LiveTvServiceInfo {
    fn default() -> Self {
        Self {
            has_update_available: Default::default(),
            home_page_url: Default::default(),
            is_visible: Default::default(),
            name: Default::default(),
            status: Default::default(),
            status_message: Default::default(),
            tuners: Default::default(),
            version: Default::default(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LiveTvServiceStatus {
    Ok,
    Unavailable,
}

impl std::fmt::Display for LiveTvServiceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Ok => f.write_str("Ok"),
            Self::Unavailable => f.write_str("Unavailable"),
        }
    }
}

impl std::str::FromStr for LiveTvServiceStatus {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Ok" => Ok(Self::Ok),
            "Unavailable" => Ok(Self::Unavailable),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for LiveTvServiceStatus {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for LiveTvServiceStatus {
    type Error = super::error::ConversionError;
    fn try_from(
        value: &String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for LiveTvServiceStatus {
    type Error = super::error::ConversionError;
    fn try_from(
        value: String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ProgramAudio {
    Mono,
    Stereo,
    Dolby,
    DolbyDigital,
    Thx,
    Atmos,
}

impl std::fmt::Display for ProgramAudio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Mono => f.write_str("Mono"),
            Self::Stereo => f.write_str("Stereo"),
            Self::Dolby => f.write_str("Dolby"),
            Self::DolbyDigital => f.write_str("DolbyDigital"),
            Self::Thx => f.write_str("Thx"),
            Self::Atmos => f.write_str("Atmos"),
        }
    }
}

impl std::str::FromStr for ProgramAudio {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Mono" => Ok(Self::Mono),
            "Stereo" => Ok(Self::Stereo),
            "Dolby" => Ok(Self::Dolby),
            "DolbyDigital" => Ok(Self::DolbyDigital),
            "Thx" => Ok(Self::Thx),
            "Atmos" => Ok(Self::Atmos),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for ProgramAudio {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for ProgramAudio {
    type Error = super::error::ConversionError;
    fn try_from(
        value: &String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for ProgramAudio {
    type Error = super::error::ConversionError;
    fn try_from(
        value: String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RecordingStatus {
    New,
    InProgress,
    Completed,
    Cancelled,
    ConflictedOk,
    ConflictedNotOk,
    Error,
}

impl std::fmt::Display for RecordingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::New => f.write_str("New"),
            Self::InProgress => f.write_str("InProgress"),
            Self::Completed => f.write_str("Completed"),
            Self::Cancelled => f.write_str("Cancelled"),
            Self::ConflictedOk => f.write_str("ConflictedOk"),
            Self::ConflictedNotOk => f.write_str("ConflictedNotOk"),
            Self::Error => f.write_str("Error"),
        }
    }
}

impl std::str::FromStr for RecordingStatus {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "New" => Ok(Self::New),
            "InProgress" => Ok(Self::InProgress),
            "Completed" => Ok(Self::Completed),
            "Cancelled" => Ok(Self::Cancelled),
            "ConflictedOk" => Ok(Self::ConflictedOk),
            "ConflictedNotOk" => Ok(Self::ConflictedNotOk),
            "Error" => Ok(Self::Error),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for RecordingStatus {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for RecordingStatus {
    type Error = super::error::ConversionError;
    fn try_from(
        value: &String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for RecordingStatus {
    type Error = super::error::ConversionError;
    fn try_from(
        value: String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "Series timer cancelled message."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct SeriesTimerCancelledMessage {
    #[doc = "Gets or sets the data."]
    #[serde(
        rename = "Data",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data: Option<TimerEventInfo>,
    #[doc = "Gets or sets the message id."]
    #[serde(
        rename = "MessageId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_id: Option<uuid::Uuid>,
    #[serde(
        rename = "MessageType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_type: Option<SessionMessageType>,
}

impl Default for SeriesTimerCancelledMessage {
    fn default() -> Self {
        Self {
            data: Default::default(),
            message_id: Default::default(),
            message_type: Default::default(),
        }
    }
}

#[doc = "Series timer created message."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct SeriesTimerCreatedMessage {
    #[doc = "Gets or sets the data."]
    #[serde(
        rename = "Data",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data: Option<TimerEventInfo>,
    #[doc = "Gets or sets the message id."]
    #[serde(
        rename = "MessageId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_id: Option<uuid::Uuid>,
    #[serde(
        rename = "MessageType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_type: Option<SessionMessageType>,
}

impl Default for SeriesTimerCreatedMessage {
    fn default() -> Self {
        Self {
            data: Default::default(),
            message_id: Default::default(),
            message_type: Default::default(),
        }
    }
}

#[doc = "Class SeriesTimerInfoDto."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct SeriesTimerInfoDto {
    #[doc = "Gets or sets the channel id of the recording."]
    #[serde(
        rename = "ChannelId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_id: Option<uuid::Uuid>,
    #[doc = "Gets or sets the channel name of the recording."]
    #[serde(
        rename = "ChannelName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_name: Option<String>,
    #[serde(
        rename = "ChannelPrimaryImageTag",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_primary_image_tag: Option<String>,
    #[doc = "Gets or sets the day pattern."]
    #[serde(
        rename = "DayPattern",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub day_pattern: Option<DayPattern>,
    #[doc = "Gets or sets the days."]
    #[serde(
        rename = "Days",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub days: Option<Vec<DayOfWeek>>,
    #[doc = "Gets or sets the end date of the recording, in UTC."]
    #[serde(
        rename = "EndDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets the external channel identifier."]
    #[serde(
        rename = "ExternalChannelId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub external_channel_id: Option<String>,
    #[doc = "Gets or sets the external identifier."]
    #[serde(
        rename = "ExternalId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub external_id: Option<String>,
    #[doc = "Gets or sets the external program identifier."]
    #[serde(
        rename = "ExternalProgramId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub external_program_id: Option<String>,
    #[doc = "Gets or sets the Id of the recording."]
    #[serde(
        rename = "Id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<String>,
    #[doc = "Gets or sets the image tags."]
    #[serde(
        rename = "ImageTags",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub image_tags: Option<
        std::collections::HashMap<String, String>,
    >,
    #[doc = "Gets or sets a value indicating whether this instance is post padding required."]
    #[serde(
        rename = "IsPostPaddingRequired",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_post_padding_required: Option<bool>,
    #[doc = "Gets or sets a value indicating whether this instance is pre padding required."]
    #[serde(
        rename = "IsPrePaddingRequired",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_pre_padding_required: Option<bool>,
    #[serde(
        rename = "KeepUntil",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub keep_until: Option<KeepUntil>,
    #[serde(
        rename = "KeepUpTo",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub keep_up_to: Option<i32>,
    #[doc = "Gets or sets the name of the recording."]
    #[serde(
        rename = "Name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,
    #[doc = "Gets or sets the description of the recording."]
    #[serde(
        rename = "Overview",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub overview: Option<String>,
    #[doc = "Gets or sets the parent backdrop image tags."]
    #[serde(
        rename = "ParentBackdropImageTags",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_backdrop_image_tags:
        Option<Vec<String>>,
    #[doc = "Gets or sets the Id of the Parent that has a backdrop if the item does not have one."]
    #[serde(
        rename = "ParentBackdropItemId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_backdrop_item_id: Option<String>,
    #[doc = "Gets or sets the parent primary image item identifier."]
    #[serde(
        rename = "ParentPrimaryImageItemId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_primary_image_item_id: Option<uuid::Uuid>,
    #[doc = "Gets or sets the parent primary image tag."]
    #[serde(
        rename = "ParentPrimaryImageTag",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_primary_image_tag: Option<String>,
    #[doc = "Gets or sets the parent thumb image tag."]
    #[serde(
        rename = "ParentThumbImageTag",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_thumb_image_tag: Option<String>,
    #[doc = "Gets or sets the parent thumb item id."]
    #[serde(
        rename = "ParentThumbItemId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_thumb_item_id: Option<String>,
    #[doc = "Gets or sets the post padding seconds."]
    #[serde(
        rename = "PostPaddingSeconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub post_padding_seconds: Option<i32>,
    #[doc = "Gets or sets the pre padding seconds."]
    #[serde(
        rename = "PrePaddingSeconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pre_padding_seconds: Option<i32>,
    #[doc = "Gets or sets the priority."]
    #[serde(
        rename = "Priority",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub priority: Option<i32>,
    #[doc = "Gets or sets the program identifier."]
    #[serde(
        rename = "ProgramId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub program_id: Option<String>,
    #[doc = "Gets or sets a value indicating whether [record any channel]."]
    #[serde(
        rename = "RecordAnyChannel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub record_any_channel: Option<bool>,
    #[doc = "Gets or sets a value indicating whether [record any time]."]
    #[serde(
        rename = "RecordAnyTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub record_any_time: Option<bool>,
    #[doc = "Gets or sets a value indicating whether [record new only]."]
    #[serde(
        rename = "RecordNewOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub record_new_only: Option<bool>,
    #[doc = "Gets or sets the server identifier."]
    #[serde(
        rename = "ServerId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub server_id: Option<String>,
    #[doc = "Gets or sets the name of the service."]
    #[serde(
        rename = "ServiceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_name: Option<String>,
    #[serde(
        rename = "SkipEpisodesInLibrary",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub skip_episodes_in_library: Option<bool>,
    #[doc = "Gets or sets the start date of the recording, in UTC."]
    #[serde(
        rename = "StartDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        rename = "Type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_: Option<String>,
}

impl Default for SeriesTimerInfoDto {
    fn default() -> Self {
        Self {
            channel_id: Default::default(),
            channel_name: Default::default(),
            channel_primary_image_tag: Default::default(),
            day_pattern: Default::default(),
            days: Default::default(),
            end_date: Default::default(),
            external_channel_id: Default::default(),
            external_id: Default::default(),
            external_program_id: Default::default(),
            id: Default::default(),
            image_tags: Default::default(),
            is_post_padding_required: Default::default(),
            is_pre_padding_required: Default::default(),
            keep_until: Default::default(),
            keep_up_to: Default::default(),
            name: Default::default(),
            overview: Default::default(),
            parent_backdrop_image_tags: Default::default(),
            parent_backdrop_item_id: Default::default(),
            parent_primary_image_item_id: Default::default(),
            parent_primary_image_tag: Default::default(),
            parent_thumb_image_tag: Default::default(),
            parent_thumb_item_id: Default::default(),
            post_padding_seconds: Default::default(),
            pre_padding_seconds: Default::default(),
            priority: Default::default(),
            program_id: Default::default(),
            record_any_channel: Default::default(),
            record_any_time: Default::default(),
            record_new_only: Default::default(),
            server_id: Default::default(),
            service_name: Default::default(),
            skip_episodes_in_library: Default::default(),
            start_date: Default::default(),
            type_: Default::default(),
        }
    }
}

#[doc = "Query result container."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct SeriesTimerInfoDtoQueryResult {
    #[doc = "Gets or sets the items."]
    #[serde(
        rename = "Items",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<SeriesTimerInfoDto>,
    #[doc = "Gets or sets the index of the first record in Items."]
    #[serde(
        rename = "StartIndex",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_index: Option<i32>,
    #[doc = "Gets or sets the total number of records available."]
    #[serde(
        rename = "TotalRecordCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_record_count: Option<i32>,
}

impl Default for SeriesTimerInfoDtoQueryResult {
    fn default() -> Self {
        Self {
            items: Default::default(),
            start_index: Default::default(),
            total_record_count: Default::default(),
        }
    }
}

#[doc = "Set channel mapping dto."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct SetChannelMappingDto {
    #[doc = "Gets or sets the provider channel id."]
    #[serde(rename = "ProviderChannelId")]
    pub provider_channel_id: String,
    #[doc = "Gets or sets the provider id."]
    #[serde(rename = "ProviderId")]
    pub provider_id: String,
    #[doc = "Gets or sets the tuner channel id."]
    #[serde(rename = "TunerChannelId")]
    pub tuner_channel_id: String,
}

#[doc = "Timer cancelled message."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct TimerCancelledMessage {
    #[doc = "Gets or sets the data."]
    #[serde(
        rename = "Data",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data: Option<TimerEventInfo>,
    #[doc = "Gets or sets the message id."]
    #[serde(
        rename = "MessageId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_id: Option<uuid::Uuid>,
    #[serde(
        rename = "MessageType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_type: Option<SessionMessageType>,
}

impl Default for TimerCancelledMessage {
    fn default() -> Self {
        Self {
            data: Default::default(),
            message_id: Default::default(),
            message_type: Default::default(),
        }
    }
}

#[doc = "Timer created message."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct TimerCreatedMessage {
    #[doc = "Gets or sets the data."]
    #[serde(
        rename = "Data",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data: Option<TimerEventInfo>,
    #[doc = "Gets or sets the message id."]
    #[serde(
        rename = "MessageId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_id: Option<uuid::Uuid>,
    #[serde(
        rename = "MessageType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_type: Option<SessionMessageType>,
}

impl Default for TimerCreatedMessage {
    fn default() -> Self {
        Self {
            data: Default::default(),
            message_id: Default::default(),
            message_type: Default::default(),
        }
    }
}

#[doc = "`TimerEventInfo`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct TimerEventInfo {
    #[serde(
        rename = "Id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<String>,
    #[serde(
        rename = "ProgramId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub program_id: Option<uuid::Uuid>,
}

impl Default for TimerEventInfo {
    fn default() -> Self {
        Self {
            id: Default::default(),
            program_id: Default::default(),
        }
    }
}

#[doc = "`TimerInfoDto`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct TimerInfoDto {
    #[doc = "Gets or sets the channel id of the recording."]
    #[serde(
        rename = "ChannelId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_id: Option<uuid::Uuid>,
    #[doc = "Gets or sets the channel name of the recording."]
    #[serde(
        rename = "ChannelName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_name: Option<String>,
    #[serde(
        rename = "ChannelPrimaryImageTag",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_primary_image_tag: Option<String>,
    #[doc = "Gets or sets the end date of the recording, in UTC."]
    #[serde(
        rename = "EndDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets the external channel identifier."]
    #[serde(
        rename = "ExternalChannelId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub external_channel_id: Option<String>,
    #[doc = "Gets or sets the external identifier."]
    #[serde(
        rename = "ExternalId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub external_id: Option<String>,
    #[doc = "Gets or sets the external program identifier."]
    #[serde(
        rename = "ExternalProgramId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub external_program_id: Option<String>,
    #[doc = "Gets or sets the external series timer identifier."]
    #[serde(
        rename = "ExternalSeriesTimerId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub external_series_timer_id: Option<String>,
    #[doc = "Gets or sets the Id of the recording."]
    #[serde(
        rename = "Id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<String>,
    #[doc = "Gets or sets a value indicating whether this instance is post padding required."]
    #[serde(
        rename = "IsPostPaddingRequired",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_post_padding_required: Option<bool>,
    #[doc = "Gets or sets a value indicating whether this instance is pre padding required."]
    #[serde(
        rename = "IsPrePaddingRequired",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_pre_padding_required: Option<bool>,
    #[serde(
        rename = "KeepUntil",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub keep_until: Option<KeepUntil>,
    #[doc = "Gets or sets the name of the recording."]
    #[serde(
        rename = "Name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,
    #[doc = "Gets or sets the description of the recording."]
    #[serde(
        rename = "Overview",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub overview: Option<String>,
    #[doc = "Gets or sets the parent backdrop image tags."]
    #[serde(
        rename = "ParentBackdropImageTags",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_backdrop_image_tags:
        Option<Vec<String>>,
    #[doc = "Gets or sets the Id of the Parent that has a backdrop if the item does not have one."]
    #[serde(
        rename = "ParentBackdropItemId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_backdrop_item_id: Option<String>,
    #[doc = "Gets or sets the post padding seconds."]
    #[serde(
        rename = "PostPaddingSeconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub post_padding_seconds: Option<i32>,
    #[doc = "Gets or sets the pre padding seconds."]
    #[serde(
        rename = "PrePaddingSeconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pre_padding_seconds: Option<i32>,
    #[doc = "Gets or sets the priority."]
    #[serde(
        rename = "Priority",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub priority: Option<i32>,
    #[doc = "Gets or sets the program identifier."]
    #[serde(
        rename = "ProgramId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub program_id: Option<String>,
    #[doc = "Gets or sets the program information."]
    #[serde(
        rename = "ProgramInfo",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub program_info: Option<BaseItemDto>,
    #[doc = "Gets or sets the run time ticks."]
    #[serde(
        rename = "RunTimeTicks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub run_time_ticks: Option<i64>,
    #[doc = "Gets or sets the series timer identifier."]
    #[serde(
        rename = "SeriesTimerId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub series_timer_id: Option<String>,
    #[doc = "Gets or sets the server identifier."]
    #[serde(
        rename = "ServerId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub server_id: Option<String>,
    #[doc = "Gets or sets the name of the service."]
    #[serde(
        rename = "ServiceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_name: Option<String>,
    #[doc = "Gets or sets the start date of the recording, in UTC."]
    #[serde(
        rename = "StartDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        rename = "Status",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub status: Option<RecordingStatus>,
    #[serde(
        rename = "Type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_: Option<String>,
}

impl Default for TimerInfoDto {
    fn default() -> Self {
        Self {
            channel_id: Default::default(),
            channel_name: Default::default(),
            channel_primary_image_tag: Default::default(),
            end_date: Default::default(),
            external_channel_id: Default::default(),
            external_id: Default::default(),
            external_program_id: Default::default(),
            external_series_timer_id: Default::default(),
            id: Default::default(),
            is_post_padding_required: Default::default(),
            is_pre_padding_required: Default::default(),
            keep_until: Default::default(),
            name: Default::default(),
            overview: Default::default(),
            parent_backdrop_image_tags: Default::default(),
            parent_backdrop_item_id: Default::default(),
            post_padding_seconds: Default::default(),
            pre_padding_seconds: Default::default(),
            priority: Default::default(),
            program_id: Default::default(),
            program_info: Default::default(),
            run_time_ticks: Default::default(),
            series_timer_id: Default::default(),
            server_id: Default::default(),
            service_name: Default::default(),
            start_date: Default::default(),
            status: Default::default(),
            type_: Default::default(),
        }
    }
}

#[doc = "Query result container."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct TimerInfoDtoQueryResult {
    #[doc = "Gets or sets the items."]
    #[serde(
        rename = "Items",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<TimerInfoDto>,
    #[doc = "Gets or sets the index of the first record in Items."]
    #[serde(
        rename = "StartIndex",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_index: Option<i32>,
    #[doc = "Gets or sets the total number of records available."]
    #[serde(
        rename = "TotalRecordCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_record_count: Option<i32>,
}

impl Default for TimerInfoDtoQueryResult {
    fn default() -> Self {
        Self {
            items: Default::default(),
            start_index: Default::default(),
            total_record_count: Default::default(),
        }
    }
}

#[doc = "`TunerChannelMapping`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct TunerChannelMapping {
    #[serde(
        rename = "Id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<String>,
    #[serde(
        rename = "Name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,
    #[serde(
        rename = "ProviderChannelId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provider_channel_id: Option<String>,
    #[serde(
        rename = "ProviderChannelName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provider_channel_name: Option<String>,
}

impl Default for TunerChannelMapping {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
            provider_channel_id: Default::default(),
            provider_channel_name: Default::default(),
        }
    }
}

#[doc = "`TunerHostInfo`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct TunerHostInfo {
    #[serde(
        rename = "AllowFmp4TranscodingContainer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_fmp4_transcoding_container: Option<bool>,
    #[serde(
        rename = "AllowHWTranscoding",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_hw_transcoding: Option<bool>,
    #[serde(
        rename = "AllowStreamSharing",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_stream_sharing: Option<bool>,
    #[serde(
        rename = "DeviceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub device_id: Option<String>,
    #[serde(
        rename = "EnableStreamLooping",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_stream_looping: Option<bool>,
    #[serde(
        rename = "FallbackMaxStreamingBitrate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fallback_max_streaming_bitrate: Option<i32>,
    #[serde(
        rename = "FriendlyName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub friendly_name: Option<String>,
    #[serde(
        rename = "Id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<String>,
    #[serde(
        rename = "IgnoreDts",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_dts: Option<bool>,
    #[serde(
        rename = "ImportFavoritesOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub import_favorites_only: Option<bool>,
    #[serde(
        rename = "ReadAtNativeFramerate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub read_at_native_framerate: Option<bool>,
    #[serde(
        rename = "Source",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source: Option<String>,
    #[serde(
        rename = "TunerCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tuner_count: Option<i32>,
    #[serde(
        rename = "Type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_: Option<String>,
    #[serde(
        rename = "Url",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub url: Option<String>,
    #[serde(
        rename = "UserAgent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_agent: Option<String>,
}

impl Default for TunerHostInfo {
    fn default() -> Self {
        Self {
            allow_fmp4_transcoding_container: Default::default(),
            allow_hw_transcoding: Default::default(),
            allow_stream_sharing: Default::default(),
            device_id: Default::default(),
            enable_stream_looping: Default::default(),
            fallback_max_streaming_bitrate: Default::default(),
            friendly_name: Default::default(),
            id: Default::default(),
            ignore_dts: Default::default(),
            import_favorites_only: Default::default(),
            read_at_native_framerate: Default::default(),
            source: Default::default(),
            tuner_count: Default::default(),
            type_: Default::default(),
            url: Default::default(),
            user_agent: Default::default(),
        }
    }
}

#[doc = "Class UtcTimeResponse."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct UtcTimeResponse {
    #[doc = "Gets the UTC time when request has been received."]
    #[serde(
        rename = "RequestReceptionTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub request_reception_time:
        Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets the UTC time when response has been sent."]
    #[serde(
        rename = "ResponseTransmissionTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub response_transmission_time:
        Option<chrono::DateTime<chrono::Utc>>,
}

impl Default for UtcTimeResponse {
    fn default() -> Self {
        Self {
            request_reception_time: Default::default(),
            response_transmission_time: Default::default(),
        }
    }
}

