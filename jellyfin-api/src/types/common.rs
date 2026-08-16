use super::*;

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum AudioSpatialFormat {
    None,
    DolbyAtmos,
    #[serde(rename = "DTSX")]
    Dtsx,
}

impl std::fmt::Display for AudioSpatialFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::None => f.write_str("None"),
            Self::DolbyAtmos => f.write_str("DolbyAtmos"),
            Self::Dtsx => f.write_str("DTSX"),
        }
    }
}

impl std::str::FromStr for AudioSpatialFormat {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "None" => Ok(Self::None),
            "DolbyAtmos" => Ok(Self::DolbyAtmos),
            "DTSX" => Ok(Self::Dtsx),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for AudioSpatialFormat {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for AudioSpatialFormat {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for AudioSpatialFormat {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "Client log document response dto."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct ClientLogDocumentResponseDto {
    #[doc = "Gets the resulting filename."]
    #[serde(rename = "FileName", default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum CodecType {
    Video,
    VideoAudio,
    Audio,
}

impl std::fmt::Display for CodecType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Video => f.write_str("Video"),
            Self::VideoAudio => f.write_str("VideoAudio"),
            Self::Audio => f.write_str("Audio"),
        }
    }
}

impl std::str::FromStr for CodecType {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Video" => Ok(Self::Video),
            "VideoAudio" => Ok(Self::VideoAudio),
            "Audio" => Ok(Self::Audio),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for CodecType {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for CodecType {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for CodecType {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "Class CountryInfo."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct CountryInfo {
    #[doc = "Gets or sets the display name."]
    #[serde(
        rename = "DisplayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[doc = "Gets or sets the name."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the name of the three letter ISO region."]
    #[serde(
        rename = "ThreeLetterISORegionName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub three_letter_iso_region_name: Option<String>,
    #[doc = "Gets or sets the name of the two letter ISO region."]
    #[serde(
        rename = "TwoLetterISORegionName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub two_letter_iso_region_name: Option<String>,
}

#[doc = "Class CultureDto."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct CultureDto {
    #[doc = "Gets the display name."]
    #[serde(
        rename = "DisplayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[doc = "Gets the name."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets the name of the three letter ISO language."]
    #[serde(
        rename = "ThreeLetterISOLanguageName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub three_letter_iso_language_name: Option<String>,
    #[serde(
        rename = "ThreeLetterISOLanguageNames",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub three_letter_iso_language_names: Vec<String>,
    #[doc = "Gets the name of the two letter ISO language."]
    #[serde(
        rename = "TwoLetterISOLanguageName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub two_letter_iso_language_name: Option<String>,
}

#[doc = "The custom value option for custom database providers."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct CustomDatabaseOption {
    #[doc = "Gets or sets the key of the value."]
    #[serde(rename = "Key", default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[doc = "Gets or sets the value."]
    #[serde(rename = "Value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[doc = "Defines the options for a custom database connector."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct CustomDatabaseOptions {
    #[doc = "Gets or sets the connection string for the custom database provider."]
    #[serde(
        rename = "ConnectionString",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub connection_string: Option<String>,
    #[doc = "Gets or sets the list of extra options for the custom provider."]
    #[serde(rename = "Options", default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<CustomDatabaseOption>,
    #[doc = "Gets or sets the plugin assembly to search for providers."]
    #[serde(
        rename = "PluginAssembly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub plugin_assembly: Option<String>,
    #[doc = "Gets or sets the Plugin name to search for database providers."]
    #[serde(
        rename = "PluginName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub plugin_name: Option<String>,
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum DatabaseLockingBehaviorTypes {
    NoLock,
    Pessimistic,
    Optimistic,
}

impl std::fmt::Display for DatabaseLockingBehaviorTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::NoLock => f.write_str("NoLock"),
            Self::Pessimistic => f.write_str("Pessimistic"),
            Self::Optimistic => f.write_str("Optimistic"),
        }
    }
}

impl std::str::FromStr for DatabaseLockingBehaviorTypes {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "NoLock" => Ok(Self::NoLock),
            "Pessimistic" => Ok(Self::Pessimistic),
            "Optimistic" => Ok(Self::Optimistic),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for DatabaseLockingBehaviorTypes {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for DatabaseLockingBehaviorTypes {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for DatabaseLockingBehaviorTypes {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum DayOfWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl std::fmt::Display for DayOfWeek {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Sunday => f.write_str("Sunday"),
            Self::Monday => f.write_str("Monday"),
            Self::Tuesday => f.write_str("Tuesday"),
            Self::Wednesday => f.write_str("Wednesday"),
            Self::Thursday => f.write_str("Thursday"),
            Self::Friday => f.write_str("Friday"),
            Self::Saturday => f.write_str("Saturday"),
        }
    }
}

impl std::str::FromStr for DayOfWeek {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Sunday" => Ok(Self::Sunday),
            "Monday" => Ok(Self::Monday),
            "Tuesday" => Ok(Self::Tuesday),
            "Wednesday" => Ok(Self::Wednesday),
            "Thursday" => Ok(Self::Thursday),
            "Friday" => Ok(Self::Friday),
            "Saturday" => Ok(Self::Saturday),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for DayOfWeek {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for DayOfWeek {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for DayOfWeek {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum DayPattern {
    Daily,
    Weekdays,
    Weekends,
}

impl std::fmt::Display for DayPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Daily => f.write_str("Daily"),
            Self::Weekdays => f.write_str("Weekdays"),
            Self::Weekends => f.write_str("Weekends"),
        }
    }
}

impl std::str::FromStr for DayPattern {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Daily" => Ok(Self::Daily),
            "Weekdays" => Ok(Self::Weekdays),
            "Weekends" => Ok(Self::Weekends),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for DayPattern {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for DayPattern {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for DayPattern {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "Default directory browser info."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct DefaultDirectoryBrowserInfoDto {
    #[doc = "Gets or sets the path."]
    #[serde(rename = "Path", default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[doc = "Defines the display preferences for any item that supports them (usually Folders)."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct DisplayPreferencesDto {
    #[doc = "Gets or sets the client."]
    #[serde(rename = "Client", default, skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,
    #[doc = "Gets or sets the custom prefs."]
    #[serde(
        rename = "CustomPrefs",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub custom_prefs: std::collections::HashMap<String, Option<String>>,
    #[doc = "Gets or sets the user id."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the index by."]
    #[serde(rename = "IndexBy", default, skip_serializing_if = "Option::is_none")]
    pub index_by: Option<String>,
    #[doc = "Gets or sets the height of the primary image."]
    #[serde(
        rename = "PrimaryImageHeight",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_image_height: Option<i32>,
    #[doc = "Gets or sets the width of the primary image."]
    #[serde(
        rename = "PrimaryImageWidth",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_image_width: Option<i32>,
    #[doc = "Gets or sets a value indicating whether [remember indexing]."]
    #[serde(
        rename = "RememberIndexing",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub remember_indexing: Option<bool>,
    #[doc = "Gets or sets a value indicating whether [remember sorting]."]
    #[serde(
        rename = "RememberSorting",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub remember_sorting: Option<bool>,
    #[serde(
        rename = "ScrollDirection",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scroll_direction: Option<ScrollDirection>,
    #[doc = "Gets or sets a value indicating whether to show backdrops on this item."]
    #[serde(
        rename = "ShowBackdrop",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub show_backdrop: Option<bool>,
    #[doc = "Gets or sets a value indicating whether [show sidebar]."]
    #[serde(
        rename = "ShowSidebar",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub show_sidebar: Option<bool>,
    #[doc = "Gets or sets the sort by."]
    #[serde(rename = "SortBy", default, skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "SortOrder", default, skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<SortOrder>,
    #[doc = "Gets or sets the type of the view."]
    #[serde(rename = "ViewType", default, skip_serializing_if = "Option::is_none")]
    pub view_type: Option<String>,
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum DynamicDayOfWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Everyday,
    Weekday,
    Weekend,
}

impl std::fmt::Display for DynamicDayOfWeek {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Sunday => f.write_str("Sunday"),
            Self::Monday => f.write_str("Monday"),
            Self::Tuesday => f.write_str("Tuesday"),
            Self::Wednesday => f.write_str("Wednesday"),
            Self::Thursday => f.write_str("Thursday"),
            Self::Friday => f.write_str("Friday"),
            Self::Saturday => f.write_str("Saturday"),
            Self::Everyday => f.write_str("Everyday"),
            Self::Weekday => f.write_str("Weekday"),
            Self::Weekend => f.write_str("Weekend"),
        }
    }
}

impl std::str::FromStr for DynamicDayOfWeek {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Sunday" => Ok(Self::Sunday),
            "Monday" => Ok(Self::Monday),
            "Tuesday" => Ok(Self::Tuesday),
            "Wednesday" => Ok(Self::Wednesday),
            "Thursday" => Ok(Self::Thursday),
            "Friday" => Ok(Self::Friday),
            "Saturday" => Ok(Self::Saturday),
            "Everyday" => Ok(Self::Everyday),
            "Weekday" => Ok(Self::Weekday),
            "Weekend" => Ok(Self::Weekend),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for DynamicDayOfWeek {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for DynamicDayOfWeek {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for DynamicDayOfWeek {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum ExtraType {
    Unknown,
    Clip,
    Trailer,
    BehindTheScenes,
    DeletedScene,
    Interview,
    Scene,
    Sample,
    ThemeSong,
    ThemeVideo,
    Featurette,
    Short,
}

impl std::fmt::Display for ExtraType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Unknown => f.write_str("Unknown"),
            Self::Clip => f.write_str("Clip"),
            Self::Trailer => f.write_str("Trailer"),
            Self::BehindTheScenes => f.write_str("BehindTheScenes"),
            Self::DeletedScene => f.write_str("DeletedScene"),
            Self::Interview => f.write_str("Interview"),
            Self::Scene => f.write_str("Scene"),
            Self::Sample => f.write_str("Sample"),
            Self::ThemeSong => f.write_str("ThemeSong"),
            Self::ThemeVideo => f.write_str("ThemeVideo"),
            Self::Featurette => f.write_str("Featurette"),
            Self::Short => f.write_str("Short"),
        }
    }
}

impl std::str::FromStr for ExtraType {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Unknown" => Ok(Self::Unknown),
            "Clip" => Ok(Self::Clip),
            "Trailer" => Ok(Self::Trailer),
            "BehindTheScenes" => Ok(Self::BehindTheScenes),
            "DeletedScene" => Ok(Self::DeletedScene),
            "Interview" => Ok(Self::Interview),
            "Scene" => Ok(Self::Scene),
            "Sample" => Ok(Self::Sample),
            "ThemeSong" => Ok(Self::ThemeSong),
            "ThemeVideo" => Ok(Self::ThemeVideo),
            "Featurette" => Ok(Self::Featurette),
            "Short" => Ok(Self::Short),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for ExtraType {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for ExtraType {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for ExtraType {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl From<GetAudioStreamAudioCodec> for String {
    fn from(value: GetAudioStreamAudioCodec) -> Self {
        value.0
    }
}

impl From<GetAudioStreamByContainerAudioCodec> for String {
    fn from(value: GetAudioStreamByContainerAudioCodec) -> Self {
        value.0
    }
}

impl From<GetAudioStreamByContainerContainer> for String {
    fn from(value: GetAudioStreamByContainerContainer) -> Self {
        value.0
    }
}

impl From<GetAudioStreamByContainerLevel> for String {
    fn from(value: GetAudioStreamByContainerLevel) -> Self {
        value.0
    }
}

impl From<GetAudioStreamByContainerSegmentContainer> for String {
    fn from(value: GetAudioStreamByContainerSegmentContainer) -> Self {
        value.0
    }
}

impl From<GetAudioStreamByContainerSubtitleCodec> for String {
    fn from(value: GetAudioStreamByContainerSubtitleCodec) -> Self {
        value.0
    }
}

impl From<GetAudioStreamByContainerVideoCodec> for String {
    fn from(value: GetAudioStreamByContainerVideoCodec) -> Self {
        value.0
    }
}

impl From<GetAudioStreamContainer> for String {
    fn from(value: GetAudioStreamContainer) -> Self {
        value.0
    }
}

impl From<GetAudioStreamLevel> for String {
    fn from(value: GetAudioStreamLevel) -> Self {
        value.0
    }
}

impl From<GetAudioStreamSegmentContainer> for String {
    fn from(value: GetAudioStreamSegmentContainer) -> Self {
        value.0
    }
}

impl From<GetAudioStreamSubtitleCodec> for String {
    fn from(value: GetAudioStreamSubtitleCodec) -> Self {
        value.0
    }
}

impl From<GetAudioStreamVideoCodec> for String {
    fn from(value: GetAudioStreamVideoCodec) -> Self {
        value.0
    }
}

impl From<GetHlsAudioSegmentAudioCodec> for String {
    fn from(value: GetHlsAudioSegmentAudioCodec) -> Self {
        value.0
    }
}

impl From<GetHlsAudioSegmentContainer> for String {
    fn from(value: GetHlsAudioSegmentContainer) -> Self {
        value.0
    }
}

impl From<GetHlsAudioSegmentLevel> for String {
    fn from(value: GetHlsAudioSegmentLevel) -> Self {
        value.0
    }
}

impl From<GetHlsAudioSegmentSegmentContainer> for String {
    fn from(value: GetHlsAudioSegmentSegmentContainer) -> Self {
        value.0
    }
}

impl From<GetHlsAudioSegmentSubtitleCodec> for String {
    fn from(value: GetHlsAudioSegmentSubtitleCodec) -> Self {
        value.0
    }
}

impl From<GetHlsAudioSegmentVideoCodec> for String {
    fn from(value: GetHlsAudioSegmentVideoCodec) -> Self {
        value.0
    }
}

impl From<GetHlsVideoSegmentAudioCodec> for String {
    fn from(value: GetHlsVideoSegmentAudioCodec) -> Self {
        value.0
    }
}

impl From<GetHlsVideoSegmentContainer> for String {
    fn from(value: GetHlsVideoSegmentContainer) -> Self {
        value.0
    }
}

impl From<GetHlsVideoSegmentLevel> for String {
    fn from(value: GetHlsVideoSegmentLevel) -> Self {
        value.0
    }
}

impl From<GetHlsVideoSegmentSegmentContainer> for String {
    fn from(value: GetHlsVideoSegmentSegmentContainer) -> Self {
        value.0
    }
}

impl From<GetHlsVideoSegmentSubtitleCodec> for String {
    fn from(value: GetHlsVideoSegmentSubtitleCodec) -> Self {
        value.0
    }
}

impl From<GetHlsVideoSegmentVideoCodec> for String {
    fn from(value: GetHlsVideoSegmentVideoCodec) -> Self {
        value.0
    }
}

impl From<GetLiveHlsStreamAudioCodec> for String {
    fn from(value: GetLiveHlsStreamAudioCodec) -> Self {
        value.0
    }
}

impl From<GetLiveHlsStreamContainer> for String {
    fn from(value: GetLiveHlsStreamContainer) -> Self {
        value.0
    }
}

impl From<GetLiveHlsStreamLevel> for String {
    fn from(value: GetLiveHlsStreamLevel) -> Self {
        value.0
    }
}

impl From<GetLiveHlsStreamSegmentContainer> for String {
    fn from(value: GetLiveHlsStreamSegmentContainer) -> Self {
        value.0
    }
}

impl From<GetLiveHlsStreamSubtitleCodec> for String {
    fn from(value: GetLiveHlsStreamSubtitleCodec) -> Self {
        value.0
    }
}

impl From<GetLiveHlsStreamVideoCodec> for String {
    fn from(value: GetLiveHlsStreamVideoCodec) -> Self {
        value.0
    }
}

impl From<GetLiveStreamFileContainer> for String {
    fn from(value: GetLiveStreamFileContainer) -> Self {
        value.0
    }
}

impl From<GetMasterHlsAudioPlaylistAudioCodec> for String {
    fn from(value: GetMasterHlsAudioPlaylistAudioCodec) -> Self {
        value.0
    }
}

impl From<GetMasterHlsAudioPlaylistLevel> for String {
    fn from(value: GetMasterHlsAudioPlaylistLevel) -> Self {
        value.0
    }
}

impl From<GetMasterHlsAudioPlaylistSegmentContainer> for String {
    fn from(value: GetMasterHlsAudioPlaylistSegmentContainer) -> Self {
        value.0
    }
}

impl From<GetMasterHlsAudioPlaylistSubtitleCodec> for String {
    fn from(value: GetMasterHlsAudioPlaylistSubtitleCodec) -> Self {
        value.0
    }
}

impl From<GetMasterHlsAudioPlaylistVideoCodec> for String {
    fn from(value: GetMasterHlsAudioPlaylistVideoCodec) -> Self {
        value.0
    }
}

impl From<GetMasterHlsVideoPlaylistAudioCodec> for String {
    fn from(value: GetMasterHlsVideoPlaylistAudioCodec) -> Self {
        value.0
    }
}

impl From<GetMasterHlsVideoPlaylistLevel> for String {
    fn from(value: GetMasterHlsVideoPlaylistLevel) -> Self {
        value.0
    }
}

impl From<GetMasterHlsVideoPlaylistSegmentContainer> for String {
    fn from(value: GetMasterHlsVideoPlaylistSegmentContainer) -> Self {
        value.0
    }
}

impl From<GetMasterHlsVideoPlaylistSubtitleCodec> for String {
    fn from(value: GetMasterHlsVideoPlaylistSubtitleCodec) -> Self {
        value.0
    }
}

impl From<GetMasterHlsVideoPlaylistVideoCodec> for String {
    fn from(value: GetMasterHlsVideoPlaylistVideoCodec) -> Self {
        value.0
    }
}

impl From<GetUniversalAudioStreamAudioCodec> for String {
    fn from(value: GetUniversalAudioStreamAudioCodec) -> Self {
        value.0
    }
}

impl From<GetUniversalAudioStreamTranscodingContainer> for String {
    fn from(value: GetUniversalAudioStreamTranscodingContainer) -> Self {
        value.0
    }
}

impl From<GetVariantHlsAudioPlaylistAudioCodec> for String {
    fn from(value: GetVariantHlsAudioPlaylistAudioCodec) -> Self {
        value.0
    }
}

impl From<GetVariantHlsAudioPlaylistLevel> for String {
    fn from(value: GetVariantHlsAudioPlaylistLevel) -> Self {
        value.0
    }
}

impl From<GetVariantHlsAudioPlaylistSegmentContainer> for String {
    fn from(value: GetVariantHlsAudioPlaylistSegmentContainer) -> Self {
        value.0
    }
}

impl From<GetVariantHlsAudioPlaylistSubtitleCodec> for String {
    fn from(value: GetVariantHlsAudioPlaylistSubtitleCodec) -> Self {
        value.0
    }
}

impl From<GetVariantHlsAudioPlaylistVideoCodec> for String {
    fn from(value: GetVariantHlsAudioPlaylistVideoCodec) -> Self {
        value.0
    }
}

impl From<GetVariantHlsVideoPlaylistAudioCodec> for String {
    fn from(value: GetVariantHlsVideoPlaylistAudioCodec) -> Self {
        value.0
    }
}

impl From<GetVariantHlsVideoPlaylistLevel> for String {
    fn from(value: GetVariantHlsVideoPlaylistLevel) -> Self {
        value.0
    }
}

impl From<GetVariantHlsVideoPlaylistSegmentContainer> for String {
    fn from(value: GetVariantHlsVideoPlaylistSegmentContainer) -> Self {
        value.0
    }
}

impl From<GetVariantHlsVideoPlaylistSubtitleCodec> for String {
    fn from(value: GetVariantHlsVideoPlaylistSubtitleCodec) -> Self {
        value.0
    }
}

impl From<GetVariantHlsVideoPlaylistVideoCodec> for String {
    fn from(value: GetVariantHlsVideoPlaylistVideoCodec) -> Self {
        value.0
    }
}

impl From<GetVideoStreamAudioCodec> for String {
    fn from(value: GetVideoStreamAudioCodec) -> Self {
        value.0
    }
}

impl From<GetVideoStreamByContainerAudioCodec> for String {
    fn from(value: GetVideoStreamByContainerAudioCodec) -> Self {
        value.0
    }
}

impl From<GetVideoStreamByContainerContainer> for String {
    fn from(value: GetVideoStreamByContainerContainer) -> Self {
        value.0
    }
}

impl From<GetVideoStreamByContainerLevel> for String {
    fn from(value: GetVideoStreamByContainerLevel) -> Self {
        value.0
    }
}

impl From<GetVideoStreamByContainerSegmentContainer> for String {
    fn from(value: GetVideoStreamByContainerSegmentContainer) -> Self {
        value.0
    }
}

impl From<GetVideoStreamByContainerSubtitleCodec> for String {
    fn from(value: GetVideoStreamByContainerSubtitleCodec) -> Self {
        value.0
    }
}

impl From<GetVideoStreamByContainerVideoCodec> for String {
    fn from(value: GetVideoStreamByContainerVideoCodec) -> Self {
        value.0
    }
}

impl From<GetVideoStreamContainer> for String {
    fn from(value: GetVideoStreamContainer) -> Self {
        value.0
    }
}

impl From<GetVideoStreamLevel> for String {
    fn from(value: GetVideoStreamLevel) -> Self {
        value.0
    }
}

impl From<GetVideoStreamSegmentContainer> for String {
    fn from(value: GetVideoStreamSegmentContainer) -> Self {
        value.0
    }
}

impl From<GetVideoStreamSubtitleCodec> for String {
    fn from(value: GetVideoStreamSubtitleCodec) -> Self {
        value.0
    }
}

impl From<GetVideoStreamVideoCodec> for String {
    fn from(value: GetVideoStreamVideoCodec) -> Self {
        value.0
    }
}

impl From<HeadAudioStreamAudioCodec> for String {
    fn from(value: HeadAudioStreamAudioCodec) -> Self {
        value.0
    }
}

impl From<HeadAudioStreamByContainerAudioCodec> for String {
    fn from(value: HeadAudioStreamByContainerAudioCodec) -> Self {
        value.0
    }
}

impl From<HeadAudioStreamByContainerContainer> for String {
    fn from(value: HeadAudioStreamByContainerContainer) -> Self {
        value.0
    }
}

impl From<HeadAudioStreamByContainerLevel> for String {
    fn from(value: HeadAudioStreamByContainerLevel) -> Self {
        value.0
    }
}

impl From<HeadAudioStreamByContainerSegmentContainer> for String {
    fn from(value: HeadAudioStreamByContainerSegmentContainer) -> Self {
        value.0
    }
}

impl From<HeadAudioStreamByContainerSubtitleCodec> for String {
    fn from(value: HeadAudioStreamByContainerSubtitleCodec) -> Self {
        value.0
    }
}

impl From<HeadAudioStreamByContainerVideoCodec> for String {
    fn from(value: HeadAudioStreamByContainerVideoCodec) -> Self {
        value.0
    }
}

impl From<HeadAudioStreamContainer> for String {
    fn from(value: HeadAudioStreamContainer) -> Self {
        value.0
    }
}

impl From<HeadAudioStreamLevel> for String {
    fn from(value: HeadAudioStreamLevel) -> Self {
        value.0
    }
}

impl From<HeadAudioStreamSegmentContainer> for String {
    fn from(value: HeadAudioStreamSegmentContainer) -> Self {
        value.0
    }
}

impl From<HeadAudioStreamSubtitleCodec> for String {
    fn from(value: HeadAudioStreamSubtitleCodec) -> Self {
        value.0
    }
}

impl From<HeadAudioStreamVideoCodec> for String {
    fn from(value: HeadAudioStreamVideoCodec) -> Self {
        value.0
    }
}

impl From<HeadMasterHlsAudioPlaylistAudioCodec> for String {
    fn from(value: HeadMasterHlsAudioPlaylistAudioCodec) -> Self {
        value.0
    }
}

impl From<HeadMasterHlsAudioPlaylistLevel> for String {
    fn from(value: HeadMasterHlsAudioPlaylistLevel) -> Self {
        value.0
    }
}

impl From<HeadMasterHlsAudioPlaylistSegmentContainer> for String {
    fn from(value: HeadMasterHlsAudioPlaylistSegmentContainer) -> Self {
        value.0
    }
}

impl From<HeadMasterHlsAudioPlaylistSubtitleCodec> for String {
    fn from(value: HeadMasterHlsAudioPlaylistSubtitleCodec) -> Self {
        value.0
    }
}

impl From<HeadMasterHlsAudioPlaylistVideoCodec> for String {
    fn from(value: HeadMasterHlsAudioPlaylistVideoCodec) -> Self {
        value.0
    }
}

impl From<HeadMasterHlsVideoPlaylistAudioCodec> for String {
    fn from(value: HeadMasterHlsVideoPlaylistAudioCodec) -> Self {
        value.0
    }
}

impl From<HeadMasterHlsVideoPlaylistLevel> for String {
    fn from(value: HeadMasterHlsVideoPlaylistLevel) -> Self {
        value.0
    }
}

impl From<HeadMasterHlsVideoPlaylistSegmentContainer> for String {
    fn from(value: HeadMasterHlsVideoPlaylistSegmentContainer) -> Self {
        value.0
    }
}

impl From<HeadMasterHlsVideoPlaylistSubtitleCodec> for String {
    fn from(value: HeadMasterHlsVideoPlaylistSubtitleCodec) -> Self {
        value.0
    }
}

impl From<HeadMasterHlsVideoPlaylistVideoCodec> for String {
    fn from(value: HeadMasterHlsVideoPlaylistVideoCodec) -> Self {
        value.0
    }
}

impl From<HeadUniversalAudioStreamAudioCodec> for String {
    fn from(value: HeadUniversalAudioStreamAudioCodec) -> Self {
        value.0
    }
}

impl From<HeadUniversalAudioStreamTranscodingContainer> for String {
    fn from(value: HeadUniversalAudioStreamTranscodingContainer) -> Self {
        value.0
    }
}

impl From<HeadVideoStreamAudioCodec> for String {
    fn from(value: HeadVideoStreamAudioCodec) -> Self {
        value.0
    }
}

impl From<HeadVideoStreamByContainerAudioCodec> for String {
    fn from(value: HeadVideoStreamByContainerAudioCodec) -> Self {
        value.0
    }
}

impl From<HeadVideoStreamByContainerContainer> for String {
    fn from(value: HeadVideoStreamByContainerContainer) -> Self {
        value.0
    }
}

impl From<HeadVideoStreamByContainerLevel> for String {
    fn from(value: HeadVideoStreamByContainerLevel) -> Self {
        value.0
    }
}

impl From<HeadVideoStreamByContainerSegmentContainer> for String {
    fn from(value: HeadVideoStreamByContainerSegmentContainer) -> Self {
        value.0
    }
}

impl From<HeadVideoStreamByContainerSubtitleCodec> for String {
    fn from(value: HeadVideoStreamByContainerSubtitleCodec) -> Self {
        value.0
    }
}

impl From<HeadVideoStreamByContainerVideoCodec> for String {
    fn from(value: HeadVideoStreamByContainerVideoCodec) -> Self {
        value.0
    }
}

impl From<HeadVideoStreamContainer> for String {
    fn from(value: HeadVideoStreamContainer) -> Self {
        value.0
    }
}

impl From<HeadVideoStreamLevel> for String {
    fn from(value: HeadVideoStreamLevel) -> Self {
        value.0
    }
}

impl From<HeadVideoStreamSegmentContainer> for String {
    fn from(value: HeadVideoStreamSegmentContainer) -> Self {
        value.0
    }
}

impl From<HeadVideoStreamSubtitleCodec> for String {
    fn from(value: HeadVideoStreamSubtitleCodec) -> Self {
        value.0
    }
}

impl From<HeadVideoStreamVideoCodec> for String {
    fn from(value: HeadVideoStreamVideoCodec) -> Self {
        value.0
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum HardwareAccelerationType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "amf")]
    Amf,
    #[serde(rename = "qsv")]
    Qsv,
    #[serde(rename = "nvenc")]
    Nvenc,
    #[serde(rename = "v4l2m2m")]
    V4l2m2m,
    #[serde(rename = "vaapi")]
    Vaapi,
    #[serde(rename = "videotoolbox")]
    Videotoolbox,
    #[serde(rename = "rkmpp")]
    Rkmpp,
}

impl std::fmt::Display for HardwareAccelerationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::None => f.write_str("none"),
            Self::Amf => f.write_str("amf"),
            Self::Qsv => f.write_str("qsv"),
            Self::Nvenc => f.write_str("nvenc"),
            Self::V4l2m2m => f.write_str("v4l2m2m"),
            Self::Vaapi => f.write_str("vaapi"),
            Self::Videotoolbox => f.write_str("videotoolbox"),
            Self::Rkmpp => f.write_str("rkmpp"),
        }
    }
}

impl std::str::FromStr for HardwareAccelerationType {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "none" => Ok(Self::None),
            "amf" => Ok(Self::Amf),
            "qsv" => Ok(Self::Qsv),
            "nvenc" => Ok(Self::Nvenc),
            "v4l2m2m" => Ok(Self::V4l2m2m),
            "vaapi" => Ok(Self::Vaapi),
            "videotoolbox" => Ok(Self::Videotoolbox),
            "rkmpp" => Ok(Self::Rkmpp),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for HardwareAccelerationType {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for HardwareAccelerationType {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for HardwareAccelerationType {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum ImageType {
    Primary,
    Art,
    Backdrop,
    Banner,
    Logo,
    Thumb,
    Disc,
    Box,
    Screenshot,
    Menu,
    Chapter,
    BoxRear,
    Profile,
}

impl std::fmt::Display for ImageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Primary => f.write_str("Primary"),
            Self::Art => f.write_str("Art"),
            Self::Backdrop => f.write_str("Backdrop"),
            Self::Banner => f.write_str("Banner"),
            Self::Logo => f.write_str("Logo"),
            Self::Thumb => f.write_str("Thumb"),
            Self::Disc => f.write_str("Disc"),
            Self::Box => f.write_str("Box"),
            Self::Screenshot => f.write_str("Screenshot"),
            Self::Menu => f.write_str("Menu"),
            Self::Chapter => f.write_str("Chapter"),
            Self::BoxRear => f.write_str("BoxRear"),
            Self::Profile => f.write_str("Profile"),
        }
    }
}

impl std::str::FromStr for ImageType {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Primary" => Ok(Self::Primary),
            "Art" => Ok(Self::Art),
            "Backdrop" => Ok(Self::Backdrop),
            "Banner" => Ok(Self::Banner),
            "Logo" => Ok(Self::Logo),
            "Thumb" => Ok(Self::Thumb),
            "Disc" => Ok(Self::Disc),
            "Box" => Ok(Self::Box),
            "Screenshot" => Ok(Self::Screenshot),
            "Menu" => Ok(Self::Menu),
            "Chapter" => Ok(Self::Chapter),
            "BoxRear" => Ok(Self::BoxRear),
            "Profile" => Ok(Self::Profile),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for ImageType {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for ImageType {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for ImageType {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum IsoType {
    Dvd,
    BluRay,
}

impl std::fmt::Display for IsoType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Dvd => f.write_str("Dvd"),
            Self::BluRay => f.write_str("BluRay"),
        }
    }
}

impl std::str::FromStr for IsoType {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Dvd" => Ok(Self::Dvd),
            "BluRay" => Ok(Self::BluRay),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for IsoType {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for IsoType {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for IsoType {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum ItemFields {
    AirTime,
    CanDelete,
    CanDownload,
    ChannelInfo,
    Chapters,
    Trickplay,
    ChildCount,
    CumulativeRunTimeTicks,
    CustomRating,
    DateCreated,
    DateLastMediaAdded,
    DisplayPreferencesId,
    Etag,
    ExternalUrls,
    Genres,
    ItemCounts,
    MediaSourceCount,
    MediaSources,
    OriginalTitle,
    Overview,
    ParentId,
    Path,
    People,
    PlayAccess,
    ProductionLocations,
    ProviderIds,
    PrimaryImageAspectRatio,
    RecursiveItemCount,
    Settings,
    SeriesStudio,
    SortName,
    SpecialEpisodeNumbers,
    Studios,
    Taglines,
    Tags,
    RemoteTrailers,
    MediaStreams,
    SeasonUserData,
    DateLastRefreshed,
    DateLastSaved,
    RefreshState,
    ChannelImage,
    EnableMediaSourceDisplay,
    Width,
    Height,
    ExtraIds,
    LocalTrailerCount,
    #[serde(rename = "IsHD")]
    IsHd,
    SpecialFeatureCount,
}

impl std::fmt::Display for ItemFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::AirTime => f.write_str("AirTime"),
            Self::CanDelete => f.write_str("CanDelete"),
            Self::CanDownload => f.write_str("CanDownload"),
            Self::ChannelInfo => f.write_str("ChannelInfo"),
            Self::Chapters => f.write_str("Chapters"),
            Self::Trickplay => f.write_str("Trickplay"),
            Self::ChildCount => f.write_str("ChildCount"),
            Self::CumulativeRunTimeTicks => f.write_str("CumulativeRunTimeTicks"),
            Self::CustomRating => f.write_str("CustomRating"),
            Self::DateCreated => f.write_str("DateCreated"),
            Self::DateLastMediaAdded => f.write_str("DateLastMediaAdded"),
            Self::DisplayPreferencesId => f.write_str("DisplayPreferencesId"),
            Self::Etag => f.write_str("Etag"),
            Self::ExternalUrls => f.write_str("ExternalUrls"),
            Self::Genres => f.write_str("Genres"),
            Self::ItemCounts => f.write_str("ItemCounts"),
            Self::MediaSourceCount => f.write_str("MediaSourceCount"),
            Self::MediaSources => f.write_str("MediaSources"),
            Self::OriginalTitle => f.write_str("OriginalTitle"),
            Self::Overview => f.write_str("Overview"),
            Self::ParentId => f.write_str("ParentId"),
            Self::Path => f.write_str("Path"),
            Self::People => f.write_str("People"),
            Self::PlayAccess => f.write_str("PlayAccess"),
            Self::ProductionLocations => f.write_str("ProductionLocations"),
            Self::ProviderIds => f.write_str("ProviderIds"),
            Self::PrimaryImageAspectRatio => f.write_str("PrimaryImageAspectRatio"),
            Self::RecursiveItemCount => f.write_str("RecursiveItemCount"),
            Self::Settings => f.write_str("Settings"),
            Self::SeriesStudio => f.write_str("SeriesStudio"),
            Self::SortName => f.write_str("SortName"),
            Self::SpecialEpisodeNumbers => f.write_str("SpecialEpisodeNumbers"),
            Self::Studios => f.write_str("Studios"),
            Self::Taglines => f.write_str("Taglines"),
            Self::Tags => f.write_str("Tags"),
            Self::RemoteTrailers => f.write_str("RemoteTrailers"),
            Self::MediaStreams => f.write_str("MediaStreams"),
            Self::SeasonUserData => f.write_str("SeasonUserData"),
            Self::DateLastRefreshed => f.write_str("DateLastRefreshed"),
            Self::DateLastSaved => f.write_str("DateLastSaved"),
            Self::RefreshState => f.write_str("RefreshState"),
            Self::ChannelImage => f.write_str("ChannelImage"),
            Self::EnableMediaSourceDisplay => f.write_str("EnableMediaSourceDisplay"),
            Self::Width => f.write_str("Width"),
            Self::Height => f.write_str("Height"),
            Self::ExtraIds => f.write_str("ExtraIds"),
            Self::LocalTrailerCount => f.write_str("LocalTrailerCount"),
            Self::IsHd => f.write_str("IsHD"),
            Self::SpecialFeatureCount => f.write_str("SpecialFeatureCount"),
        }
    }
}

impl std::str::FromStr for ItemFields {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "AirTime" => Ok(Self::AirTime),
            "CanDelete" => Ok(Self::CanDelete),
            "CanDownload" => Ok(Self::CanDownload),
            "ChannelInfo" => Ok(Self::ChannelInfo),
            "Chapters" => Ok(Self::Chapters),
            "Trickplay" => Ok(Self::Trickplay),
            "ChildCount" => Ok(Self::ChildCount),
            "CumulativeRunTimeTicks" => Ok(Self::CumulativeRunTimeTicks),
            "CustomRating" => Ok(Self::CustomRating),
            "DateCreated" => Ok(Self::DateCreated),
            "DateLastMediaAdded" => Ok(Self::DateLastMediaAdded),
            "DisplayPreferencesId" => Ok(Self::DisplayPreferencesId),
            "Etag" => Ok(Self::Etag),
            "ExternalUrls" => Ok(Self::ExternalUrls),
            "Genres" => Ok(Self::Genres),
            "ItemCounts" => Ok(Self::ItemCounts),
            "MediaSourceCount" => Ok(Self::MediaSourceCount),
            "MediaSources" => Ok(Self::MediaSources),
            "OriginalTitle" => Ok(Self::OriginalTitle),
            "Overview" => Ok(Self::Overview),
            "ParentId" => Ok(Self::ParentId),
            "Path" => Ok(Self::Path),
            "People" => Ok(Self::People),
            "PlayAccess" => Ok(Self::PlayAccess),
            "ProductionLocations" => Ok(Self::ProductionLocations),
            "ProviderIds" => Ok(Self::ProviderIds),
            "PrimaryImageAspectRatio" => Ok(Self::PrimaryImageAspectRatio),
            "RecursiveItemCount" => Ok(Self::RecursiveItemCount),
            "Settings" => Ok(Self::Settings),
            "SeriesStudio" => Ok(Self::SeriesStudio),
            "SortName" => Ok(Self::SortName),
            "SpecialEpisodeNumbers" => Ok(Self::SpecialEpisodeNumbers),
            "Studios" => Ok(Self::Studios),
            "Taglines" => Ok(Self::Taglines),
            "Tags" => Ok(Self::Tags),
            "RemoteTrailers" => Ok(Self::RemoteTrailers),
            "MediaStreams" => Ok(Self::MediaStreams),
            "SeasonUserData" => Ok(Self::SeasonUserData),
            "DateLastRefreshed" => Ok(Self::DateLastRefreshed),
            "DateLastSaved" => Ok(Self::DateLastSaved),
            "RefreshState" => Ok(Self::RefreshState),
            "ChannelImage" => Ok(Self::ChannelImage),
            "EnableMediaSourceDisplay" => Ok(Self::EnableMediaSourceDisplay),
            "Width" => Ok(Self::Width),
            "Height" => Ok(Self::Height),
            "ExtraIds" => Ok(Self::ExtraIds),
            "LocalTrailerCount" => Ok(Self::LocalTrailerCount),
            "IsHD" => Ok(Self::IsHd),
            "SpecialFeatureCount" => Ok(Self::SpecialFeatureCount),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for ItemFields {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for ItemFields {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for ItemFields {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum ItemFilter {
    IsFolder,
    IsNotFolder,
    IsUnplayed,
    IsPlayed,
    IsFavorite,
    IsResumable,
    Likes,
    Dislikes,
    IsFavoriteOrLikes,
}

impl std::fmt::Display for ItemFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::IsFolder => f.write_str("IsFolder"),
            Self::IsNotFolder => f.write_str("IsNotFolder"),
            Self::IsUnplayed => f.write_str("IsUnplayed"),
            Self::IsPlayed => f.write_str("IsPlayed"),
            Self::IsFavorite => f.write_str("IsFavorite"),
            Self::IsResumable => f.write_str("IsResumable"),
            Self::Likes => f.write_str("Likes"),
            Self::Dislikes => f.write_str("Dislikes"),
            Self::IsFavoriteOrLikes => f.write_str("IsFavoriteOrLikes"),
        }
    }
}

impl std::str::FromStr for ItemFilter {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "IsFolder" => Ok(Self::IsFolder),
            "IsNotFolder" => Ok(Self::IsNotFolder),
            "IsUnplayed" => Ok(Self::IsUnplayed),
            "IsPlayed" => Ok(Self::IsPlayed),
            "IsFavorite" => Ok(Self::IsFavorite),
            "IsResumable" => Ok(Self::IsResumable),
            "Likes" => Ok(Self::Likes),
            "Dislikes" => Ok(Self::Dislikes),
            "IsFavoriteOrLikes" => Ok(Self::IsFavoriteOrLikes),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for ItemFilter {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for ItemFilter {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for ItemFilter {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum ItemSortBy {
    Default,
    AiredEpisodeOrder,
    Album,
    AlbumArtist,
    Artist,
    DateCreated,
    OfficialRating,
    DatePlayed,
    PremiereDate,
    StartDate,
    SortName,
    Name,
    Random,
    Runtime,
    CommunityRating,
    ProductionYear,
    PlayCount,
    CriticRating,
    IsFolder,
    IsUnplayed,
    IsPlayed,
    SeriesSortName,
    VideoBitRate,
    AirTime,
    Studio,
    IsFavoriteOrLiked,
    DateLastContentAdded,
    SeriesDatePlayed,
    ParentIndexNumber,
    IndexNumber,
}

impl std::fmt::Display for ItemSortBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Default => f.write_str("Default"),
            Self::AiredEpisodeOrder => f.write_str("AiredEpisodeOrder"),
            Self::Album => f.write_str("Album"),
            Self::AlbumArtist => f.write_str("AlbumArtist"),
            Self::Artist => f.write_str("Artist"),
            Self::DateCreated => f.write_str("DateCreated"),
            Self::OfficialRating => f.write_str("OfficialRating"),
            Self::DatePlayed => f.write_str("DatePlayed"),
            Self::PremiereDate => f.write_str("PremiereDate"),
            Self::StartDate => f.write_str("StartDate"),
            Self::SortName => f.write_str("SortName"),
            Self::Name => f.write_str("Name"),
            Self::Random => f.write_str("Random"),
            Self::Runtime => f.write_str("Runtime"),
            Self::CommunityRating => f.write_str("CommunityRating"),
            Self::ProductionYear => f.write_str("ProductionYear"),
            Self::PlayCount => f.write_str("PlayCount"),
            Self::CriticRating => f.write_str("CriticRating"),
            Self::IsFolder => f.write_str("IsFolder"),
            Self::IsUnplayed => f.write_str("IsUnplayed"),
            Self::IsPlayed => f.write_str("IsPlayed"),
            Self::SeriesSortName => f.write_str("SeriesSortName"),
            Self::VideoBitRate => f.write_str("VideoBitRate"),
            Self::AirTime => f.write_str("AirTime"),
            Self::Studio => f.write_str("Studio"),
            Self::IsFavoriteOrLiked => f.write_str("IsFavoriteOrLiked"),
            Self::DateLastContentAdded => f.write_str("DateLastContentAdded"),
            Self::SeriesDatePlayed => f.write_str("SeriesDatePlayed"),
            Self::ParentIndexNumber => f.write_str("ParentIndexNumber"),
            Self::IndexNumber => f.write_str("IndexNumber"),
        }
    }
}

impl std::str::FromStr for ItemSortBy {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Default" => Ok(Self::Default),
            "AiredEpisodeOrder" => Ok(Self::AiredEpisodeOrder),
            "Album" => Ok(Self::Album),
            "AlbumArtist" => Ok(Self::AlbumArtist),
            "Artist" => Ok(Self::Artist),
            "DateCreated" => Ok(Self::DateCreated),
            "OfficialRating" => Ok(Self::OfficialRating),
            "DatePlayed" => Ok(Self::DatePlayed),
            "PremiereDate" => Ok(Self::PremiereDate),
            "StartDate" => Ok(Self::StartDate),
            "SortName" => Ok(Self::SortName),
            "Name" => Ok(Self::Name),
            "Random" => Ok(Self::Random),
            "Runtime" => Ok(Self::Runtime),
            "CommunityRating" => Ok(Self::CommunityRating),
            "ProductionYear" => Ok(Self::ProductionYear),
            "PlayCount" => Ok(Self::PlayCount),
            "CriticRating" => Ok(Self::CriticRating),
            "IsFolder" => Ok(Self::IsFolder),
            "IsUnplayed" => Ok(Self::IsUnplayed),
            "IsPlayed" => Ok(Self::IsPlayed),
            "SeriesSortName" => Ok(Self::SeriesSortName),
            "VideoBitRate" => Ok(Self::VideoBitRate),
            "AirTime" => Ok(Self::AirTime),
            "Studio" => Ok(Self::Studio),
            "IsFavoriteOrLiked" => Ok(Self::IsFavoriteOrLiked),
            "DateLastContentAdded" => Ok(Self::DateLastContentAdded),
            "SeriesDatePlayed" => Ok(Self::SeriesDatePlayed),
            "ParentIndexNumber" => Ok(Self::ParentIndexNumber),
            "IndexNumber" => Ok(Self::IndexNumber),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for ItemSortBy {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for ItemSortBy {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for ItemSortBy {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum KeepUntil {
    UntilDeleted,
    UntilSpaceNeeded,
    UntilWatched,
    UntilDate,
}

impl std::fmt::Display for KeepUntil {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::UntilDeleted => f.write_str("UntilDeleted"),
            Self::UntilSpaceNeeded => f.write_str("UntilSpaceNeeded"),
            Self::UntilWatched => f.write_str("UntilWatched"),
            Self::UntilDate => f.write_str("UntilDate"),
        }
    }
}

impl std::str::FromStr for KeepUntil {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "UntilDeleted" => Ok(Self::UntilDeleted),
            "UntilSpaceNeeded" => Ok(Self::UntilSpaceNeeded),
            "UntilWatched" => Ok(Self::UntilWatched),
            "UntilDate" => Ok(Self::UntilDate),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for KeepUntil {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for KeepUntil {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for KeepUntil {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "`LocalizationOption`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct LocalizationOption {
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum LocationType {
    FileSystem,
    Remote,
    Virtual,
    Offline,
}

impl std::fmt::Display for LocationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::FileSystem => f.write_str("FileSystem"),
            Self::Remote => f.write_str("Remote"),
            Self::Virtual => f.write_str("Virtual"),
            Self::Offline => f.write_str("Offline"),
        }
    }
}

impl std::str::FromStr for LocationType {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "FileSystem" => Ok(Self::FileSystem),
            "Remote" => Ok(Self::Remote),
            "Virtual" => Ok(Self::Virtual),
            "Offline" => Ok(Self::Offline),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for LocationType {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for LocationType {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for LocationType {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum LogLevel {
    Trace,
    Debug,
    Information,
    Warning,
    Error,
    Critical,
    None,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Trace => f.write_str("Trace"),
            Self::Debug => f.write_str("Debug"),
            Self::Information => f.write_str("Information"),
            Self::Warning => f.write_str("Warning"),
            Self::Error => f.write_str("Error"),
            Self::Critical => f.write_str("Critical"),
            Self::None => f.write_str("None"),
        }
    }
}

impl std::str::FromStr for LogLevel {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Trace" => Ok(Self::Trace),
            "Debug" => Ok(Self::Debug),
            "Information" => Ok(Self::Information),
            "Warning" => Ok(Self::Warning),
            "Error" => Ok(Self::Error),
            "Critical" => Ok(Self::Critical),
            "None" => Ok(Self::None),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for LogLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for LogLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for LogLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum MediaProtocol {
    File,
    Http,
    Rtmp,
    Rtsp,
    Udp,
    Rtp,
    Ftp,
}

impl std::fmt::Display for MediaProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::File => f.write_str("File"),
            Self::Http => f.write_str("Http"),
            Self::Rtmp => f.write_str("Rtmp"),
            Self::Rtsp => f.write_str("Rtsp"),
            Self::Udp => f.write_str("Udp"),
            Self::Rtp => f.write_str("Rtp"),
            Self::Ftp => f.write_str("Ftp"),
        }
    }
}

impl std::str::FromStr for MediaProtocol {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "File" => Ok(Self::File),
            "Http" => Ok(Self::Http),
            "Rtmp" => Ok(Self::Rtmp),
            "Rtsp" => Ok(Self::Rtsp),
            "Udp" => Ok(Self::Udp),
            "Rtp" => Ok(Self::Rtp),
            "Ftp" => Ok(Self::Ftp),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for MediaProtocol {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for MediaProtocol {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for MediaProtocol {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum MediaType {
    Unknown,
    Video,
    Audio,
    Photo,
    Book,
}

impl std::fmt::Display for MediaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Unknown => f.write_str("Unknown"),
            Self::Video => f.write_str("Video"),
            Self::Audio => f.write_str("Audio"),
            Self::Photo => f.write_str("Photo"),
            Self::Book => f.write_str("Book"),
        }
    }
}

impl std::str::FromStr for MediaType {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Unknown" => Ok(Self::Unknown),
            "Video" => Ok(Self::Video),
            "Audio" => Ok(Self::Audio),
            "Photo" => Ok(Self::Photo),
            "Book" => Ok(Self::Book),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for MediaType {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for MediaType {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for MediaType {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum MetadataField {
    Cast,
    Genres,
    ProductionLocations,
    Studios,
    Tags,
    Name,
    Overview,
    Runtime,
    OfficialRating,
}

impl std::fmt::Display for MetadataField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Cast => f.write_str("Cast"),
            Self::Genres => f.write_str("Genres"),
            Self::ProductionLocations => f.write_str("ProductionLocations"),
            Self::Studios => f.write_str("Studios"),
            Self::Tags => f.write_str("Tags"),
            Self::Name => f.write_str("Name"),
            Self::Overview => f.write_str("Overview"),
            Self::Runtime => f.write_str("Runtime"),
            Self::OfficialRating => f.write_str("OfficialRating"),
        }
    }
}

impl std::str::FromStr for MetadataField {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Cast" => Ok(Self::Cast),
            "Genres" => Ok(Self::Genres),
            "ProductionLocations" => Ok(Self::ProductionLocations),
            "Studios" => Ok(Self::Studios),
            "Tags" => Ok(Self::Tags),
            "Name" => Ok(Self::Name),
            "Overview" => Ok(Self::Overview),
            "Runtime" => Ok(Self::Runtime),
            "OfficialRating" => Ok(Self::OfficialRating),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for MetadataField {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for MetadataField {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for MetadataField {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "Class MetadataOptions."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct MetadataOptions {
    #[serde(
        rename = "DisabledImageFetchers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_image_fetchers: Option<Vec<String>>,
    #[serde(
        rename = "DisabledMetadataFetchers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_metadata_fetchers: Option<Vec<String>>,
    #[serde(
        rename = "DisabledMetadataSavers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_metadata_savers: Option<Vec<String>>,
    #[serde(
        rename = "ImageFetcherOrder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub image_fetcher_order: Option<Vec<String>>,
    #[serde(rename = "ItemType", default, skip_serializing_if = "Option::is_none")]
    pub item_type: Option<String>,
    #[serde(
        rename = "LocalMetadataReaderOrder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub local_metadata_reader_order: Option<Vec<String>>,
    #[serde(
        rename = "MetadataFetcherOrder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata_fetcher_order: Option<Vec<String>>,
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum MetadataRefreshMode {
    None,
    ValidationOnly,
    Default,
    FullRefresh,
}

impl std::fmt::Display for MetadataRefreshMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::None => f.write_str("None"),
            Self::ValidationOnly => f.write_str("ValidationOnly"),
            Self::Default => f.write_str("Default"),
            Self::FullRefresh => f.write_str("FullRefresh"),
        }
    }
}

impl std::str::FromStr for MetadataRefreshMode {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "None" => Ok(Self::None),
            "ValidationOnly" => Ok(Self::ValidationOnly),
            "Default" => Ok(Self::Default),
            "FullRefresh" => Ok(Self::FullRefresh),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for MetadataRefreshMode {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for MetadataRefreshMode {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for MetadataRefreshMode {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "`NameIdPair`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct NameIdPair {
    #[doc = "Gets or sets the identifier."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the name."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[doc = "Class ParentalRating."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct ParentalRating {
    #[doc = "Gets or sets the name."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the rating score."]
    #[serde(
        rename = "RatingScore",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rating_score: Option<ParentalRatingScore>,
    #[doc = "Gets or sets the value."]
    #[serde(rename = "Value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
}

#[doc = "A class representing an parental rating score."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct ParentalRatingScore {
    #[doc = "Gets or sets the score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<i32>,
    #[doc = "Gets or sets the sub score."]
    #[serde(rename = "subScore", default, skip_serializing_if = "Option::is_none")]
    pub sub_score: Option<i32>,
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum PersonKind {
    Unknown,
    Actor,
    Director,
    Composer,
    Writer,
    GuestStar,
    Producer,
    Conductor,
    Lyricist,
    Arranger,
    Engineer,
    Mixer,
    Remixer,
    Creator,
    Artist,
    AlbumArtist,
    Author,
    Illustrator,
    Penciller,
    Inker,
    Colorist,
    Letterer,
    CoverArtist,
    Editor,
    Translator,
}

impl std::fmt::Display for PersonKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Unknown => f.write_str("Unknown"),
            Self::Actor => f.write_str("Actor"),
            Self::Director => f.write_str("Director"),
            Self::Composer => f.write_str("Composer"),
            Self::Writer => f.write_str("Writer"),
            Self::GuestStar => f.write_str("GuestStar"),
            Self::Producer => f.write_str("Producer"),
            Self::Conductor => f.write_str("Conductor"),
            Self::Lyricist => f.write_str("Lyricist"),
            Self::Arranger => f.write_str("Arranger"),
            Self::Engineer => f.write_str("Engineer"),
            Self::Mixer => f.write_str("Mixer"),
            Self::Remixer => f.write_str("Remixer"),
            Self::Creator => f.write_str("Creator"),
            Self::Artist => f.write_str("Artist"),
            Self::AlbumArtist => f.write_str("AlbumArtist"),
            Self::Author => f.write_str("Author"),
            Self::Illustrator => f.write_str("Illustrator"),
            Self::Penciller => f.write_str("Penciller"),
            Self::Inker => f.write_str("Inker"),
            Self::Colorist => f.write_str("Colorist"),
            Self::Letterer => f.write_str("Letterer"),
            Self::CoverArtist => f.write_str("CoverArtist"),
            Self::Editor => f.write_str("Editor"),
            Self::Translator => f.write_str("Translator"),
        }
    }
}

impl std::str::FromStr for PersonKind {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Unknown" => Ok(Self::Unknown),
            "Actor" => Ok(Self::Actor),
            "Director" => Ok(Self::Director),
            "Composer" => Ok(Self::Composer),
            "Writer" => Ok(Self::Writer),
            "GuestStar" => Ok(Self::GuestStar),
            "Producer" => Ok(Self::Producer),
            "Conductor" => Ok(Self::Conductor),
            "Lyricist" => Ok(Self::Lyricist),
            "Arranger" => Ok(Self::Arranger),
            "Engineer" => Ok(Self::Engineer),
            "Mixer" => Ok(Self::Mixer),
            "Remixer" => Ok(Self::Remixer),
            "Creator" => Ok(Self::Creator),
            "Artist" => Ok(Self::Artist),
            "AlbumArtist" => Ok(Self::AlbumArtist),
            "Author" => Ok(Self::Author),
            "Illustrator" => Ok(Self::Illustrator),
            "Penciller" => Ok(Self::Penciller),
            "Inker" => Ok(Self::Inker),
            "Colorist" => Ok(Self::Colorist),
            "Letterer" => Ok(Self::Letterer),
            "CoverArtist" => Ok(Self::CoverArtist),
            "Editor" => Ok(Self::Editor),
            "Translator" => Ok(Self::Translator),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for PersonKind {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for PersonKind {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for PersonKind {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "`ProblemDetails`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ProblemDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<String, ::serde_json::Value>,
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum ProcessPriorityClass {
    Normal,
    Idle,
    High,
    RealTime,
    BelowNormal,
    AboveNormal,
}

impl std::fmt::Display for ProcessPriorityClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Normal => f.write_str("Normal"),
            Self::Idle => f.write_str("Idle"),
            Self::High => f.write_str("High"),
            Self::RealTime => f.write_str("RealTime"),
            Self::BelowNormal => f.write_str("BelowNormal"),
            Self::AboveNormal => f.write_str("AboveNormal"),
        }
    }
}

impl std::str::FromStr for ProcessPriorityClass {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Normal" => Ok(Self::Normal),
            "Idle" => Ok(Self::Idle),
            "High" => Ok(Self::High),
            "RealTime" => Ok(Self::RealTime),
            "BelowNormal" => Ok(Self::BelowNormal),
            "AboveNormal" => Ok(Self::AboveNormal),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for ProcessPriorityClass {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for ProcessPriorityClass {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for ProcessPriorityClass {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "`QueryFilters`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct QueryFilters {
    #[serde(rename = "Genres", default, skip_serializing_if = "Option::is_none")]
    pub genres: Option<Vec<NameGuidPair>>,
    #[serde(rename = "Tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

#[doc = "`QueryFiltersLegacy`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct QueryFiltersLegacy {
    #[serde(rename = "Genres", default, skip_serializing_if = "Option::is_none")]
    pub genres: Option<Vec<String>>,
    #[serde(
        rename = "OfficialRatings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub official_ratings: Option<Vec<String>>,
    #[serde(rename = "Tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "Years", default, skip_serializing_if = "Option::is_none")]
    pub years: Option<Vec<i32>>,
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum RatingType {
    Score,
    Likes,
}

impl std::fmt::Display for RatingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Score => f.write_str("Score"),
            Self::Likes => f.write_str("Likes"),
        }
    }
}

impl std::str::FromStr for RatingType {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Score" => Ok(Self::Score),
            "Likes" => Ok(Self::Likes),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for RatingType {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for RatingType {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for RatingType {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "`RecommendationDto`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct RecommendationDto {
    #[serde(
        rename = "BaselineItemName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub baseline_item_name: Option<String>,
    #[serde(
        rename = "CategoryId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub category_id: Option<uuid::Uuid>,
    #[serde(rename = "Items", default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<BaseItemDto>>,
    #[serde(
        rename = "RecommendationType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub recommendation_type: Option<RecommendationType>,
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum RecommendationType {
    SimilarToRecentlyPlayed,
    SimilarToLikedItem,
    HasDirectorFromRecentlyPlayed,
    HasActorFromRecentlyPlayed,
    HasLikedDirector,
    HasLikedActor,
}

impl std::fmt::Display for RecommendationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::SimilarToRecentlyPlayed => f.write_str("SimilarToRecentlyPlayed"),
            Self::SimilarToLikedItem => f.write_str("SimilarToLikedItem"),
            Self::HasDirectorFromRecentlyPlayed => f.write_str("HasDirectorFromRecentlyPlayed"),
            Self::HasActorFromRecentlyPlayed => f.write_str("HasActorFromRecentlyPlayed"),
            Self::HasLikedDirector => f.write_str("HasLikedDirector"),
            Self::HasLikedActor => f.write_str("HasLikedActor"),
        }
    }
}

impl std::str::FromStr for RecommendationType {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "SimilarToRecentlyPlayed" => Ok(Self::SimilarToRecentlyPlayed),
            "SimilarToLikedItem" => Ok(Self::SimilarToLikedItem),
            "HasDirectorFromRecentlyPlayed" => Ok(Self::HasDirectorFromRecentlyPlayed),
            "HasActorFromRecentlyPlayed" => Ok(Self::HasActorFromRecentlyPlayed),
            "HasLikedDirector" => Ok(Self::HasLikedDirector),
            "HasLikedActor" => Ok(Self::HasLikedActor),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for RecommendationType {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for RecommendationType {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for RecommendationType {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "Refresh progress message."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct RefreshProgressMessage {
    #[doc = "Gets or sets the data."]
    #[serde(rename = "Data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, Option<String>>>,
    #[doc = "Gets or sets the message id."]
    #[serde(rename = "MessageId", default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<uuid::Uuid>,
    #[serde(
        rename = "MessageType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_type: Option<SessionMessageType>,
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum ScrollDirection {
    Horizontal,
    Vertical,
}

impl std::fmt::Display for ScrollDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Horizontal => f.write_str("Horizontal"),
            Self::Vertical => f.write_str("Vertical"),
        }
    }
}

impl std::str::FromStr for ScrollDirection {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Horizontal" => Ok(Self::Horizontal),
            "Vertical" => Ok(Self::Vertical),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for ScrollDirection {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for ScrollDirection {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for ScrollDirection {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum SeriesStatus {
    Continuing,
    Ended,
    Unreleased,
}

impl std::fmt::Display for SeriesStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Continuing => f.write_str("Continuing"),
            Self::Ended => f.write_str("Ended"),
            Self::Unreleased => f.write_str("Unreleased"),
        }
    }
}

impl std::str::FromStr for SeriesStatus {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Continuing" => Ok(Self::Continuing),
            "Ended" => Ok(Self::Ended),
            "Unreleased" => Ok(Self::Unreleased),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for SeriesStatus {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for SeriesStatus {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for SeriesStatus {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum SortOrder {
    Ascending,
    Descending,
}

impl std::fmt::Display for SortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Ascending => f.write_str("Ascending"),
            Self::Descending => f.write_str("Descending"),
        }
    }
}

impl std::str::FromStr for SortOrder {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Ascending" => Ok(Self::Ascending),
            "Descending" => Ok(Self::Descending),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for SortOrder {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for SortOrder {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for SortOrder {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "Special view option dto."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct SpecialViewOptionDto {
    #[doc = "Gets or sets view option id."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets view option name."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum TransportStreamTimestamp {
    None,
    Zero,
    Valid,
}

impl std::fmt::Display for TransportStreamTimestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::None => f.write_str("None"),
            Self::Zero => f.write_str("Zero"),
            Self::Valid => f.write_str("Valid"),
        }
    }
}

impl std::str::FromStr for TransportStreamTimestamp {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "None" => Ok(Self::None),
            "Zero" => Ok(Self::Zero),
            "Valid" => Ok(Self::Valid),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for TransportStreamTimestamp {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for TransportStreamTimestamp {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for TransportStreamTimestamp {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum UnratedItem {
    Movie,
    Trailer,
    Series,
    Music,
    Book,
    LiveTvChannel,
    LiveTvProgram,
    ChannelContent,
    Other,
}

impl std::fmt::Display for UnratedItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Movie => f.write_str("Movie"),
            Self::Trailer => f.write_str("Trailer"),
            Self::Series => f.write_str("Series"),
            Self::Music => f.write_str("Music"),
            Self::Book => f.write_str("Book"),
            Self::LiveTvChannel => f.write_str("LiveTvChannel"),
            Self::LiveTvProgram => f.write_str("LiveTvProgram"),
            Self::ChannelContent => f.write_str("ChannelContent"),
            Self::Other => f.write_str("Other"),
        }
    }
}

impl std::str::FromStr for UnratedItem {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Movie" => Ok(Self::Movie),
            "Trailer" => Ok(Self::Trailer),
            "Series" => Ok(Self::Series),
            "Music" => Ok(Self::Music),
            "Book" => Ok(Self::Book),
            "LiveTvChannel" => Ok(Self::LiveTvChannel),
            "LiveTvProgram" => Ok(Self::LiveTvProgram),
            "ChannelContent" => Ok(Self::ChannelContent),
            "Other" => Ok(Self::Other),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for UnratedItem {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for UnratedItem {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for UnratedItem {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "Class UserDataChangeInfo."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct UserDataChangeInfo {
    #[doc = "Gets or sets the user data list."]
    #[serde(
        rename = "UserDataList",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub user_data_list: Vec<UserItemDataDto>,
    #[doc = "Gets or sets the user id."]
    #[serde(rename = "UserId", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<uuid::Uuid>,
}

#[doc = "User data changed message."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct UserDataChangedMessage {
    #[doc = "Class UserDataChangeInfo."]
    #[serde(rename = "Data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<UserDataChangeInfo>,
    #[doc = "Gets or sets the message id."]
    #[serde(rename = "MessageId", default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<uuid::Uuid>,
    #[serde(
        rename = "MessageType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_type: Option<SessionMessageType>,
}

#[doc = "Validate path object."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct ValidatePathDto {
    #[doc = "Gets or sets is path file."]
    #[serde(rename = "IsFile", default, skip_serializing_if = "Option::is_none")]
    pub is_file: Option<bool>,
    #[doc = "Gets or sets the path."]
    #[serde(rename = "Path", default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Gets or sets a value indicating whether validate if path is writable."]
    #[serde(
        rename = "ValidateWritable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub validate_writable: Option<bool>,
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum Video3DFormat {
    HalfSideBySide,
    FullSideBySide,
    FullTopAndBottom,
    HalfTopAndBottom,
    #[serde(rename = "MVC")]
    Mvc,
}

impl std::fmt::Display for Video3DFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::HalfSideBySide => f.write_str("HalfSideBySide"),
            Self::FullSideBySide => f.write_str("FullSideBySide"),
            Self::FullTopAndBottom => f.write_str("FullTopAndBottom"),
            Self::HalfTopAndBottom => f.write_str("HalfTopAndBottom"),
            Self::Mvc => f.write_str("MVC"),
        }
    }
}

impl std::str::FromStr for Video3DFormat {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "HalfSideBySide" => Ok(Self::HalfSideBySide),
            "FullSideBySide" => Ok(Self::FullSideBySide),
            "FullTopAndBottom" => Ok(Self::FullTopAndBottom),
            "HalfTopAndBottom" => Ok(Self::HalfTopAndBottom),
            "MVC" => Ok(Self::Mvc),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for Video3DFormat {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for Video3DFormat {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for Video3DFormat {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum VideoRange {
    Unknown,
    #[serde(rename = "SDR")]
    Sdr,
    #[serde(rename = "HDR")]
    Hdr,
}

impl std::fmt::Display for VideoRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Unknown => f.write_str("Unknown"),
            Self::Sdr => f.write_str("SDR"),
            Self::Hdr => f.write_str("HDR"),
        }
    }
}

impl std::str::FromStr for VideoRange {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Unknown" => Ok(Self::Unknown),
            "SDR" => Ok(Self::Sdr),
            "HDR" => Ok(Self::Hdr),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for VideoRange {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for VideoRange {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for VideoRange {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum VideoRangeType {
    Unknown,
    #[serde(rename = "SDR")]
    Sdr,
    #[serde(rename = "HDR10")]
    Hdr10,
    #[serde(rename = "HLG")]
    Hlg,
    #[serde(rename = "DOVI")]
    Dovi,
    #[serde(rename = "DOVIWithHDR10")]
    DoviWithHdr10,
    #[serde(rename = "DOVIWithHLG")]
    DoviWithHlg,
    #[serde(rename = "DOVIWithSDR")]
    DoviWithSdr,
    #[serde(rename = "DOVIWithEL")]
    DoviWithEl,
    #[serde(rename = "DOVIWithHDR10Plus")]
    DoviWithHdr10Plus,
    #[serde(rename = "DOVIWithELHDR10Plus")]
    DoviWithElhdr10Plus,
    #[serde(rename = "DOVIInvalid")]
    DoviInvalid,
    #[serde(rename = "HDR10Plus")]
    Hdr10Plus,
}

impl std::fmt::Display for VideoRangeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Unknown => f.write_str("Unknown"),
            Self::Sdr => f.write_str("SDR"),
            Self::Hdr10 => f.write_str("HDR10"),
            Self::Hlg => f.write_str("HLG"),
            Self::Dovi => f.write_str("DOVI"),
            Self::DoviWithHdr10 => f.write_str("DOVIWithHDR10"),
            Self::DoviWithHlg => f.write_str("DOVIWithHLG"),
            Self::DoviWithSdr => f.write_str("DOVIWithSDR"),
            Self::DoviWithEl => f.write_str("DOVIWithEL"),
            Self::DoviWithHdr10Plus => f.write_str("DOVIWithHDR10Plus"),
            Self::DoviWithElhdr10Plus => f.write_str("DOVIWithELHDR10Plus"),
            Self::DoviInvalid => f.write_str("DOVIInvalid"),
            Self::Hdr10Plus => f.write_str("HDR10Plus"),
        }
    }
}

impl std::str::FromStr for VideoRangeType {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Unknown" => Ok(Self::Unknown),
            "SDR" => Ok(Self::Sdr),
            "HDR10" => Ok(Self::Hdr10),
            "HLG" => Ok(Self::Hlg),
            "DOVI" => Ok(Self::Dovi),
            "DOVIWithHDR10" => Ok(Self::DoviWithHdr10),
            "DOVIWithHLG" => Ok(Self::DoviWithHlg),
            "DOVIWithSDR" => Ok(Self::DoviWithSdr),
            "DOVIWithEL" => Ok(Self::DoviWithEl),
            "DOVIWithHDR10Plus" => Ok(Self::DoviWithHdr10Plus),
            "DOVIWithELHDR10Plus" => Ok(Self::DoviWithElhdr10Plus),
            "DOVIInvalid" => Ok(Self::DoviInvalid),
            "HDR10Plus" => Ok(Self::Hdr10Plus),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for VideoRangeType {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for VideoRangeType {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for VideoRangeType {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum VideoType {
    VideoFile,
    Iso,
    Dvd,
    BluRay,
}

impl std::fmt::Display for VideoType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::VideoFile => f.write_str("VideoFile"),
            Self::Iso => f.write_str("Iso"),
            Self::Dvd => f.write_str("Dvd"),
            Self::BluRay => f.write_str("BluRay"),
        }
    }
}

impl std::str::FromStr for VideoType {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "VideoFile" => Ok(Self::VideoFile),
            "Iso" => Ok(Self::Iso),
            "Dvd" => Ok(Self::Dvd),
            "BluRay" => Ok(Self::BluRay),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for VideoType {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for VideoType {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for VideoType {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "`XbmcMetadataOptions`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct XbmcMetadataOptions {
    #[serde(
        rename = "EnableExtraThumbsDuplication",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_extra_thumbs_duplication: Option<bool>,
    #[serde(
        rename = "EnablePathSubstitution",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_path_substitution: Option<bool>,
    #[serde(
        rename = "ReleaseDateFormat",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub release_date_format: Option<String>,
    #[serde(
        rename = "SaveImagePathsInNfo",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub save_image_paths_in_nfo: Option<bool>,
    #[serde(rename = "UserId", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}
