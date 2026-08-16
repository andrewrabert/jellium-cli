#[doc = "`ChannelFeatures`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct ChannelFeatures {
    #[doc = "Gets or sets the automatic refresh levels."]
    #[serde(
        rename = "AutoRefreshLevels",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_refresh_levels: Option<i32>,
    #[doc = "Gets or sets a value indicating whether this instance can filter."]
    #[serde(rename = "CanFilter", default, skip_serializing_if = "Option::is_none")]
    pub can_filter: Option<bool>,
    #[doc = "Gets or sets a value indicating whether this instance can search."]
    #[serde(rename = "CanSearch", default, skip_serializing_if = "Option::is_none")]
    pub can_search: Option<bool>,
    #[doc = "Gets or sets the content types."]
    #[serde(
        rename = "ContentTypes",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub content_types: Vec<ChannelMediaContentType>,
    #[doc = "Gets or sets the default sort orders."]
    #[serde(
        rename = "DefaultSortFields",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub default_sort_fields: Vec<ChannelItemSortField>,
    #[doc = "Gets or sets the identifier."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[doc = "Gets or sets the maximum number of records the channel allows retrieving at a time."]
    #[serde(
        rename = "MaxPageSize",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_page_size: Option<i32>,
    #[doc = "Gets or sets the media types."]
    #[serde(rename = "MediaTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub media_types: Vec<ChannelMediaType>,
    #[doc = "Gets or sets the name."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets a value indicating whether [supports content downloading]."]
    #[serde(
        rename = "SupportsContentDownloading",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_content_downloading: Option<bool>,
    #[doc = "Gets or sets a value indicating whether [supports latest media]."]
    #[serde(
        rename = "SupportsLatestMedia",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_latest_media: Option<bool>,
    #[doc = "Gets or sets a value indicating whether a sort ascending/descending toggle is supported."]
    #[serde(
        rename = "SupportsSortOrderToggle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_sort_order_toggle: Option<bool>,
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum ChannelItemSortField {
    Name,
    CommunityRating,
    PremiereDate,
    DateCreated,
    Runtime,
    PlayCount,
    CommunityPlayCount,
}

impl std::fmt::Display for ChannelItemSortField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Name => f.write_str("Name"),
            Self::CommunityRating => f.write_str("CommunityRating"),
            Self::PremiereDate => f.write_str("PremiereDate"),
            Self::DateCreated => f.write_str("DateCreated"),
            Self::Runtime => f.write_str("Runtime"),
            Self::PlayCount => f.write_str("PlayCount"),
            Self::CommunityPlayCount => f.write_str("CommunityPlayCount"),
        }
    }
}

impl std::str::FromStr for ChannelItemSortField {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Name" => Ok(Self::Name),
            "CommunityRating" => Ok(Self::CommunityRating),
            "PremiereDate" => Ok(Self::PremiereDate),
            "DateCreated" => Ok(Self::DateCreated),
            "Runtime" => Ok(Self::Runtime),
            "PlayCount" => Ok(Self::PlayCount),
            "CommunityPlayCount" => Ok(Self::CommunityPlayCount),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for ChannelItemSortField {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for ChannelItemSortField {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for ChannelItemSortField {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum ChannelMediaContentType {
    Clip,
    Podcast,
    Trailer,
    Movie,
    Episode,
    Song,
    MovieExtra,
    TvExtra,
}

impl std::fmt::Display for ChannelMediaContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Clip => f.write_str("Clip"),
            Self::Podcast => f.write_str("Podcast"),
            Self::Trailer => f.write_str("Trailer"),
            Self::Movie => f.write_str("Movie"),
            Self::Episode => f.write_str("Episode"),
            Self::Song => f.write_str("Song"),
            Self::MovieExtra => f.write_str("MovieExtra"),
            Self::TvExtra => f.write_str("TvExtra"),
        }
    }
}

impl std::str::FromStr for ChannelMediaContentType {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Clip" => Ok(Self::Clip),
            "Podcast" => Ok(Self::Podcast),
            "Trailer" => Ok(Self::Trailer),
            "Movie" => Ok(Self::Movie),
            "Episode" => Ok(Self::Episode),
            "Song" => Ok(Self::Song),
            "MovieExtra" => Ok(Self::MovieExtra),
            "TvExtra" => Ok(Self::TvExtra),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for ChannelMediaContentType {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for ChannelMediaContentType {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for ChannelMediaContentType {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum ChannelMediaType {
    Audio,
    Video,
    Photo,
}

impl std::fmt::Display for ChannelMediaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Audio => f.write_str("Audio"),
            Self::Video => f.write_str("Video"),
            Self::Photo => f.write_str("Photo"),
        }
    }
}

impl std::str::FromStr for ChannelMediaType {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Audio" => Ok(Self::Audio),
            "Video" => Ok(Self::Video),
            "Photo" => Ok(Self::Photo),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for ChannelMediaType {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for ChannelMediaType {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for ChannelMediaType {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum ChannelType {
    #[serde(rename = "TV")]
    Tv,
    Radio,
}

impl std::fmt::Display for ChannelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Tv => f.write_str("TV"),
            Self::Radio => f.write_str("Radio"),
        }
    }
}

impl std::str::FromStr for ChannelType {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "TV" => Ok(Self::Tv),
            "Radio" => Ok(Self::Radio),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for ChannelType {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for ChannelType {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for ChannelType {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}
