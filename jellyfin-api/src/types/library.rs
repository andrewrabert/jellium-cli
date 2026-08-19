use super::*;

#[doc = "Add virtual folder dto."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct AddVirtualFolderDto {
    #[doc = "Gets or sets library options."]
    #[serde(
        rename = "LibraryOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub library_options: Option<LibraryOptions>,
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum CollectionType {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "movies")]
    Movies,
    #[serde(rename = "tvshows")]
    Tvshows,
    #[serde(rename = "music")]
    Music,
    #[serde(rename = "musicvideos")]
    Musicvideos,
    #[serde(rename = "trailers")]
    Trailers,
    #[serde(rename = "homevideos")]
    Homevideos,
    #[serde(rename = "boxsets")]
    Boxsets,
    #[serde(rename = "books")]
    Books,
    #[serde(rename = "photos")]
    Photos,
    #[serde(rename = "livetv")]
    Livetv,
    #[serde(rename = "playlists")]
    Playlists,
    #[serde(rename = "folders")]
    Folders,
}

impl std::fmt::Display for CollectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Unknown => f.write_str("unknown"),
            Self::Movies => f.write_str("movies"),
            Self::Tvshows => f.write_str("tvshows"),
            Self::Music => f.write_str("music"),
            Self::Musicvideos => f.write_str("musicvideos"),
            Self::Trailers => f.write_str("trailers"),
            Self::Homevideos => f.write_str("homevideos"),
            Self::Boxsets => f.write_str("boxsets"),
            Self::Books => f.write_str("books"),
            Self::Photos => f.write_str("photos"),
            Self::Livetv => f.write_str("livetv"),
            Self::Playlists => f.write_str("playlists"),
            Self::Folders => f.write_str("folders"),
        }
    }
}

impl std::str::FromStr for CollectionType {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "unknown" => Ok(Self::Unknown),
            "movies" => Ok(Self::Movies),
            "tvshows" => Ok(Self::Tvshows),
            "music" => Ok(Self::Music),
            "musicvideos" => Ok(Self::Musicvideos),
            "trailers" => Ok(Self::Trailers),
            "homevideos" => Ok(Self::Homevideos),
            "boxsets" => Ok(Self::Boxsets),
            "books" => Ok(Self::Books),
            "photos" => Ok(Self::Photos),
            "livetv" => Ok(Self::Livetv),
            "playlists" => Ok(Self::Playlists),
            "folders" => Ok(Self::Folders),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for CollectionType {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for CollectionType {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for CollectionType {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum CollectionTypeOptions {
    #[serde(rename = "movies")]
    Movies,
    #[serde(rename = "tvshows")]
    Tvshows,
    #[serde(rename = "music")]
    Music,
    #[serde(rename = "musicvideos")]
    Musicvideos,
    #[serde(rename = "homevideos")]
    Homevideos,
    #[serde(rename = "boxsets")]
    Boxsets,
    #[serde(rename = "books")]
    Books,
    #[serde(rename = "mixed")]
    Mixed,
}

impl std::fmt::Display for CollectionTypeOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Movies => f.write_str("movies"),
            Self::Tvshows => f.write_str("tvshows"),
            Self::Music => f.write_str("music"),
            Self::Musicvideos => f.write_str("musicvideos"),
            Self::Homevideos => f.write_str("homevideos"),
            Self::Boxsets => f.write_str("boxsets"),
            Self::Books => f.write_str("books"),
            Self::Mixed => f.write_str("mixed"),
        }
    }
}

impl std::str::FromStr for CollectionTypeOptions {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "movies" => Ok(Self::Movies),
            "tvshows" => Ok(Self::Tvshows),
            "music" => Ok(Self::Music),
            "musicvideos" => Ok(Self::Musicvideos),
            "homevideos" => Ok(Self::Homevideos),
            "boxsets" => Ok(Self::Boxsets),
            "books" => Ok(Self::Books),
            "mixed" => Ok(Self::Mixed),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for CollectionTypeOptions {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for CollectionTypeOptions {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for CollectionTypeOptions {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "`ConfigImageTypes`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct ConfigImageTypes {
    #[serde(
        rename = "BackdropSizes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub backdrop_sizes: Option<Vec<String>>,
    #[serde(rename = "BaseUrl", default, skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    #[serde(rename = "LogoSizes", default, skip_serializing_if = "Option::is_none")]
    pub logo_sizes: Option<Vec<String>>,
    #[serde(
        rename = "PosterSizes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub poster_sizes: Option<Vec<String>>,
    #[serde(
        rename = "ProfileSizes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub profile_sizes: Option<Vec<String>>,
    #[serde(
        rename = "SecureBaseUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub secure_base_url: Option<String>,
    #[serde(
        rename = "StillSizes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub still_sizes: Option<Vec<String>>,
}

#[doc = "Contains information about a specific folder."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct FolderStorageDto {
    #[doc = "Gets the Device Identifier."]
    #[serde(rename = "DeviceId", default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[doc = "Gets the free space of the underlying storage device of the Jellyfin.Api.Models.SystemInfoDtos.FolderStorageDto.Path."]
    #[serde(rename = "FreeSpace", default, skip_serializing_if = "Option::is_none")]
    pub free_space: Option<i64>,
    #[doc = "Gets the path of the folder in question."]
    #[serde(rename = "Path", default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Gets the kind of storage device of the Jellyfin.Api.Models.SystemInfoDtos.FolderStorageDto.Path."]
    #[serde(
        rename = "StorageType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_type: Option<String>,
    #[doc = "Gets the used space of the underlying storage device of the Jellyfin.Api.Models.SystemInfoDtos.FolderStorageDto.Path."]
    #[serde(rename = "UsedSpace", default, skip_serializing_if = "Option::is_none")]
    pub used_space: Option<i64>,
}

#[doc = "Library changed message."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct LibraryChangedMessage {
    #[doc = "Class LibraryUpdateInfo."]
    #[serde(rename = "Data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<LibraryUpdateInfo>,
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

#[doc = "Library option info dto."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct LibraryOptionInfoDto {
    #[doc = "Gets or sets a value indicating whether default enabled."]
    #[serde(
        rename = "DefaultEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_enabled: Option<bool>,
    #[doc = "Gets or sets name."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[doc = "`LibraryOptions`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct LibraryOptions {
    #[serde(
        rename = "AllowEmbeddedSubtitles",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_embedded_subtitles: Option<EmbeddedSubtitleOptions>,
    #[serde(
        rename = "AutomaticRefreshIntervalDays",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub automatic_refresh_interval_days: Option<i32>,
    #[serde(
        rename = "AutomaticallyAddToCollection",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub automatically_add_to_collection: Option<bool>,
    #[serde(
        rename = "CustomTagDelimiters",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub custom_tag_delimiters: Vec<String>,
    #[serde(
        rename = "DelimiterWhitelist",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub delimiter_whitelist: Vec<String>,
    #[serde(
        rename = "DisabledLocalMetadataReaders",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub disabled_local_metadata_readers: Vec<String>,
    #[serde(
        rename = "DisabledLyricFetchers",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub disabled_lyric_fetchers: Vec<String>,
    #[serde(
        rename = "DisabledMediaSegmentProviders",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub disabled_media_segment_providers: Vec<String>,
    #[serde(
        rename = "DisabledSubtitleFetchers",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub disabled_subtitle_fetchers: Vec<String>,
    #[serde(
        rename = "EnableAutomaticSeriesGrouping",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_automatic_series_grouping: Option<bool>,
    #[serde(
        rename = "EnableChapterImageExtraction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_chapter_image_extraction: Option<bool>,
    #[serde(
        rename = "EnableEmbeddedEpisodeInfos",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_embedded_episode_infos: Option<bool>,
    #[serde(
        rename = "EnableEmbeddedExtrasTitles",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_embedded_extras_titles: Option<bool>,
    #[serde(
        rename = "EnableEmbeddedTitles",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_embedded_titles: Option<bool>,
    #[serde(
        rename = "EnableInternetProviders",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_internet_providers: Option<bool>,
    #[serde(
        rename = "EnableLUFSScan",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_lufs_scan: Option<bool>,
    #[serde(
        rename = "EnablePhotos",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_photos: Option<bool>,
    #[serde(
        rename = "EnableRealtimeMonitor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_realtime_monitor: Option<bool>,
    #[serde(
        rename = "EnableTrickplayImageExtraction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_trickplay_image_extraction: Option<bool>,
    #[serde(rename = "Enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "ExtractChapterImagesDuringLibraryScan",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub extract_chapter_images_during_library_scan: Option<bool>,
    #[serde(
        rename = "ExtractTrickplayImagesDuringLibraryScan",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub extract_trickplay_images_during_library_scan: Option<bool>,
    #[serde(
        rename = "LocalMetadataReaderOrder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub local_metadata_reader_order: Option<Vec<String>>,
    #[serde(
        rename = "LyricFetcherOrder",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub lyric_fetcher_order: Vec<String>,
    #[serde(
        rename = "MediaSegmentProviderOrder",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub media_segment_provider_order: Vec<String>,
    #[doc = "Gets or sets the metadata country code."]
    #[serde(
        rename = "MetadataCountryCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata_country_code: Option<String>,
    #[serde(
        rename = "MetadataSavers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata_savers: Option<Vec<String>>,
    #[serde(rename = "PathInfos", default, skip_serializing_if = "Vec::is_empty")]
    pub path_infos: Vec<MediaPathInfo>,
    #[serde(rename = "PreferNonstandardArtistsTag", default)]
    pub prefer_nonstandard_artists_tag: bool,
    #[doc = "Gets or sets the preferred metadata language."]
    #[serde(
        rename = "PreferredMetadataLanguage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub preferred_metadata_language: Option<String>,
    #[serde(
        rename = "RequirePerfectSubtitleMatch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub require_perfect_subtitle_match: Option<bool>,
    #[serde(
        rename = "SaveLocalMetadata",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub save_local_metadata: Option<bool>,
    #[serde(rename = "SaveLyricsWithMedia", default)]
    pub save_lyrics_with_media: bool,
    #[serde(
        rename = "SaveSubtitlesWithMedia",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub save_subtitles_with_media: Option<bool>,
    #[serde(rename = "SaveTrickplayWithMedia", default)]
    pub save_trickplay_with_media: bool,
    #[serde(
        rename = "SeasonZeroDisplayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub season_zero_display_name: Option<String>,
    #[serde(
        rename = "SkipSubtitlesIfAudioTrackMatches",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub skip_subtitles_if_audio_track_matches: Option<bool>,
    #[serde(
        rename = "SkipSubtitlesIfEmbeddedSubtitlesPresent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub skip_subtitles_if_embedded_subtitles_present: Option<bool>,
    #[serde(
        rename = "SubtitleDownloadLanguages",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub subtitle_download_languages: Option<Vec<String>>,
    #[serde(
        rename = "SubtitleFetcherOrder",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub subtitle_fetcher_order: Vec<String>,
    #[serde(rename = "TypeOptions", default, skip_serializing_if = "Vec::is_empty")]
    pub type_options: Vec<TypeOptions>,
    #[serde(rename = "UseCustomTagDelimiters", default)]
    pub use_custom_tag_delimiters: bool,
}

#[doc = "Library options result dto."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct LibraryOptionsResultDto {
    #[doc = "Gets or sets the list of lyric fetchers."]
    #[serde(
        rename = "LyricFetchers",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub lyric_fetchers: Vec<LibraryOptionInfoDto>,
    #[doc = "Gets or sets the list of MediaSegment Providers."]
    #[serde(
        rename = "MediaSegmentProviders",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub media_segment_providers: Vec<LibraryOptionInfoDto>,
    #[doc = "Gets or sets the metadata readers."]
    #[serde(
        rename = "MetadataReaders",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub metadata_readers: Vec<LibraryOptionInfoDto>,
    #[doc = "Gets or sets the metadata savers."]
    #[serde(
        rename = "MetadataSavers",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub metadata_savers: Vec<LibraryOptionInfoDto>,
    #[doc = "Gets or sets the subtitle fetchers."]
    #[serde(
        rename = "SubtitleFetchers",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub subtitle_fetchers: Vec<LibraryOptionInfoDto>,
    #[doc = "Gets or sets the type options."]
    #[serde(rename = "TypeOptions", default, skip_serializing_if = "Vec::is_empty")]
    pub type_options: Vec<LibraryTypeOptionsDto>,
}

#[doc = "Contains informations about a libraries storage informations."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct LibraryStorageDto {
    #[doc = "Gets or sets the storage informations about the folders used in a library."]
    #[serde(rename = "Folders", default, skip_serializing_if = "Vec::is_empty")]
    pub folders: Vec<FolderStorageDto>,
    #[doc = "Gets or sets the Library Id."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[doc = "Gets or sets the name of the library."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[doc = "Library type options dto."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct LibraryTypeOptionsDto {
    #[doc = "Gets or sets the default image options."]
    #[serde(
        rename = "DefaultImageOptions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub default_image_options: Vec<ImageOption>,
    #[doc = "Gets or sets the image fetchers."]
    #[serde(
        rename = "ImageFetchers",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub image_fetchers: Vec<LibraryOptionInfoDto>,
    #[doc = "Gets or sets the metadata fetchers."]
    #[serde(
        rename = "MetadataFetchers",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub metadata_fetchers: Vec<LibraryOptionInfoDto>,
    #[doc = "Gets or sets the supported image types."]
    #[serde(
        rename = "SupportedImageTypes",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_image_types: Vec<ImageType>,
    #[doc = "Gets or sets the type."]
    #[serde(rename = "Type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[doc = "Class LibraryUpdateInfo."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct LibraryUpdateInfo {
    #[serde(
        rename = "CollectionFolders",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub collection_folders: Vec<String>,
    #[doc = "Gets or sets the folders added to."]
    #[serde(
        rename = "FoldersAddedTo",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub folders_added_to: Vec<String>,
    #[doc = "Gets or sets the folders removed from."]
    #[serde(
        rename = "FoldersRemovedFrom",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub folders_removed_from: Vec<String>,
    #[serde(rename = "IsEmpty", default, skip_serializing_if = "Option::is_none")]
    pub is_empty: Option<bool>,
    #[doc = "Gets or sets the items added."]
    #[serde(rename = "ItemsAdded", default, skip_serializing_if = "Vec::is_empty")]
    pub items_added: Vec<String>,
    #[doc = "Gets or sets the items removed."]
    #[serde(
        rename = "ItemsRemoved",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items_removed: Vec<String>,
    #[doc = "Gets or sets the items updated."]
    #[serde(
        rename = "ItemsUpdated",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items_updated: Vec<String>,
}

#[doc = "Media Path dto."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct MediaPathDto {
    #[doc = "Gets or sets the name of the library."]
    #[serde(rename = "Name")]
    pub name: String,
    #[doc = "Gets or sets the path to add."]
    #[serde(rename = "Path", default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Gets or sets the path info."]
    #[serde(rename = "PathInfo", default, skip_serializing_if = "Option::is_none")]
    pub path_info: Option<MediaPathInfo>,
}

#[doc = "`MediaPathInfo`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct MediaPathInfo {
    #[serde(rename = "Path", default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[doc = "Media Update Info Dto."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct MediaUpdateInfoDto {
    #[doc = "Gets or sets the list of updates."]
    #[serde(rename = "Updates", default, skip_serializing_if = "Vec::is_empty")]
    pub updates: Vec<MediaUpdateInfoPathDto>,
}

#[doc = "The media update info path."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct MediaUpdateInfoPathDto {
    #[doc = "Gets or sets media path."]
    #[serde(rename = "Path", default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Gets or sets media update type.\r\nCreated, Modified, Deleted."]
    #[serde(
        rename = "UpdateType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_type: Option<String>,
}

#[doc = "`TypeOptions`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct TypeOptions {
    #[serde(
        rename = "ImageFetcherOrder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub image_fetcher_order: Option<Vec<String>>,
    #[serde(
        rename = "ImageFetchers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub image_fetchers: Option<Vec<String>>,
    #[serde(
        rename = "ImageOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub image_options: Option<Vec<ImageOption>>,
    #[serde(
        rename = "MetadataFetcherOrder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata_fetcher_order: Option<Vec<String>>,
    #[serde(
        rename = "MetadataFetchers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata_fetchers: Option<Vec<String>>,
    #[serde(rename = "Type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[doc = "Update library options dto."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct UpdateLibraryOptionsDto {
    #[doc = "Gets or sets the library item id."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[doc = "Gets or sets library options."]
    #[serde(
        rename = "LibraryOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub library_options: Option<LibraryOptions>,
}

#[doc = "Update library options dto."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct UpdateMediaPathRequestDto {
    #[doc = "Gets or sets the library name."]
    #[serde(rename = "Name")]
    pub name: String,
    #[doc = "Gets or sets library folder path information."]
    #[serde(rename = "PathInfo")]
    pub path_info: MediaPathInfo,
}

#[doc = "Used to hold information about a user's list of configured virtual folders."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct VirtualFolderInfo {
    #[doc = "Gets or sets the type of the collection."]
    #[serde(
        rename = "CollectionType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub collection_type: Option<CollectionTypeOptions>,
    #[doc = "Gets or sets the item identifier."]
    #[serde(rename = "ItemId", default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(
        rename = "LibraryOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub library_options: Option<LibraryOptions>,
    #[doc = "Gets or sets the locations."]
    #[serde(rename = "Locations", default, skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<String>>,
    #[doc = "Gets or sets the name."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the primary image item identifier."]
    #[serde(
        rename = "PrimaryImageItemId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_image_item_id: Option<String>,
    #[serde(
        rename = "RefreshProgress",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub refresh_progress: Option<f64>,
    #[serde(
        rename = "RefreshStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub refresh_status: Option<String>,
}
