use super::*;

#[doc = "`AllThemeMediaResult`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AllThemeMediaResult {
    #[doc = "Class ThemeMediaResult."]
    #[serde(
        rename = "SoundtrackSongsResult",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub soundtrack_songs_result: Option<ThemeMediaResult>,
    #[doc = "Class ThemeMediaResult."]
    #[serde(
        rename = "ThemeSongsResult",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub theme_songs_result: Option<ThemeMediaResult>,
    #[doc = "Class ThemeMediaResult."]
    #[serde(
        rename = "ThemeVideosResult",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub theme_videos_result: Option<ThemeMediaResult>,
}

impl Default for AllThemeMediaResult {
    fn default() -> Self {
        Self {
            soundtrack_songs_result: Default::default(),
            theme_songs_result: Default::default(),
            theme_videos_result: Default::default(),
        }
    }
}

#[doc = "This is strictly used as a data transfer object from the api layer.\r\nThis holds information about a BaseItem in a format that is convenient for the client."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct BaseItemDto {
    #[doc = "Gets or sets the air days."]
    #[serde(
        rename = "AirDays",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub air_days: Option<Vec<DayOfWeek>>,
    #[doc = "Gets or sets the air time."]
    #[serde(
        rename = "AirTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub air_time: Option<String>,
    #[serde(
        rename = "AirsAfterSeasonNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub airs_after_season_number: Option<i32>,
    #[serde(
        rename = "AirsBeforeEpisodeNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub airs_before_episode_number: Option<i32>,
    #[serde(
        rename = "AirsBeforeSeasonNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub airs_before_season_number: Option<i32>,
    #[doc = "Gets or sets the album."]
    #[serde(
        rename = "Album",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub album: Option<String>,
    #[doc = "Gets or sets the album artist."]
    #[serde(
        rename = "AlbumArtist",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub album_artist: Option<String>,
    #[doc = "Gets or sets the album artists."]
    #[serde(
        rename = "AlbumArtists",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub album_artists: Option<Vec<NameGuidPair>>,
    #[doc = "Gets or sets the album count."]
    #[serde(
        rename = "AlbumCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub album_count: Option<i32>,
    #[doc = "Gets or sets the album id."]
    #[serde(
        rename = "AlbumId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub album_id: Option<uuid::Uuid>,
    #[doc = "Gets or sets the album image tag."]
    #[serde(
        rename = "AlbumPrimaryImageTag",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub album_primary_image_tag: Option<String>,
    #[serde(
        rename = "Altitude",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub altitude: Option<f64>,
    #[serde(
        rename = "Aperture",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub aperture: Option<f64>,
    #[serde(
        rename = "ArtistCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artist_count: Option<i32>,
    #[doc = "Gets or sets the artist items."]
    #[serde(
        rename = "ArtistItems",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artist_items: Option<Vec<NameGuidPair>>,
    #[doc = "Gets or sets the artists."]
    #[serde(
        rename = "Artists",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artists: Option<Vec<String>>,
    #[doc = "Gets or sets the aspect ratio."]
    #[serde(
        rename = "AspectRatio",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub aspect_ratio: Option<String>,
    #[doc = "Gets or sets the audio."]
    #[serde(
        rename = "Audio",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub audio: Option<ProgramAudio>,
    #[doc = "Gets or sets the backdrop image tags."]
    #[serde(
        rename = "BackdropImageTags",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub backdrop_image_tags: Option<Vec<String>>,
    #[serde(
        rename = "CameraMake",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub camera_make: Option<String>,
    #[serde(
        rename = "CameraModel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub camera_model: Option<String>,
    #[serde(
        rename = "CanDelete",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub can_delete: Option<bool>,
    #[serde(
        rename = "CanDownload",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub can_download: Option<bool>,
    #[doc = "Gets or sets the channel identifier."]
    #[serde(
        rename = "ChannelId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_id: Option<uuid::Uuid>,
    #[serde(
        rename = "ChannelName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_name: Option<String>,
    #[serde(
        rename = "ChannelNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_number: Option<String>,
    #[doc = "Gets or sets the channel primary image tag."]
    #[serde(
        rename = "ChannelPrimaryImageTag",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_primary_image_tag: Option<String>,
    #[doc = "Gets or sets the type of the channel."]
    #[serde(
        rename = "ChannelType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_type: Option<ChannelType>,
    #[doc = "Gets or sets the chapters."]
    #[serde(
        rename = "Chapters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub chapters: Option<Vec<ChapterInfo>>,
    #[doc = "Gets or sets the child count."]
    #[serde(
        rename = "ChildCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub child_count: Option<i32>,
    #[doc = "Gets or sets the type of the collection."]
    #[serde(
        rename = "CollectionType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub collection_type: Option<CollectionType>,
    #[doc = "Gets or sets the community rating."]
    #[serde(
        rename = "CommunityRating",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub community_rating: Option<f32>,
    #[doc = "Gets or sets the completion percentage."]
    #[serde(
        rename = "CompletionPercentage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub completion_percentage: Option<f64>,
    #[serde(
        rename = "Container",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub container: Option<String>,
    #[doc = "Gets or sets the critic rating."]
    #[serde(
        rename = "CriticRating",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub critic_rating: Option<f32>,
    #[doc = "Gets or sets the cumulative run time ticks."]
    #[serde(
        rename = "CumulativeRunTimeTicks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cumulative_run_time_ticks: Option<i64>,
    #[doc = "Gets or sets the current program."]
    #[serde(rename = "CurrentProgram", default)]
    pub current_program: Box<Option<BaseItemDto>>,
    #[doc = "Gets or sets the custom rating."]
    #[serde(
        rename = "CustomRating",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_rating: Option<String>,
    #[doc = "Gets or sets the date created."]
    #[serde(
        rename = "DateCreated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub date_created: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        rename = "DateLastMediaAdded",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub date_last_media_added: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets the display order."]
    #[serde(
        rename = "DisplayOrder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_order: Option<String>,
    #[doc = "Gets or sets the display preferences id."]
    #[serde(
        rename = "DisplayPreferencesId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_preferences_id: Option<String>,
    #[serde(
        rename = "EnableMediaSourceDisplay",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_media_source_display: Option<bool>,
    #[doc = "Gets or sets the end date."]
    #[serde(
        rename = "EndDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets the episode count."]
    #[serde(
        rename = "EpisodeCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub episode_count: Option<i32>,
    #[doc = "Gets or sets the episode title."]
    #[serde(
        rename = "EpisodeTitle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub episode_title: Option<String>,
    #[doc = "Gets or sets the etag."]
    #[serde(
        rename = "Etag",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub etag: Option<String>,
    #[serde(
        rename = "ExposureTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposure_time: Option<f64>,
    #[doc = "Gets or sets the external urls."]
    #[serde(
        rename = "ExternalUrls",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub external_urls: Option<Vec<ExternalUrl>>,
    #[serde(
        rename = "ExtraType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub extra_type: Option<ExtraType>,
    #[serde(
        rename = "FocalLength",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub focal_length: Option<f64>,
    #[serde(
        rename = "ForcedSortName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub forced_sort_name: Option<String>,
    #[serde(
        rename = "GenreItems",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub genre_items: Option<Vec<NameGuidPair>>,
    #[doc = "Gets or sets the genres."]
    #[serde(
        rename = "Genres",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub genres: Option<Vec<String>>,
    #[serde(
        rename = "HasLyrics",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub has_lyrics: Option<bool>,
    #[serde(
        rename = "HasSubtitles",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub has_subtitles: Option<bool>,
    #[serde(
        rename = "Height",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub height: Option<i32>,
    #[doc = "Gets or sets the id."]
    #[serde(
        rename = "Id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<uuid::Uuid>,
    #[doc = "Gets or sets the blurhashes for the image tags.\r\nMaps image type to dictionary mapping image tag to blurhash value."]
    #[serde(
        rename = "ImageBlurHashes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub image_blur_hashes: Option<BaseItemDtoImageBlurHashes>,
    #[serde(
        rename = "ImageOrientation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub image_orientation: Option<ImageOrientation>,
    #[doc = "Gets or sets the image tags."]
    #[serde(
        rename = "ImageTags",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub image_tags: Option<
        std::collections::HashMap<String, String>,
    >,
    #[doc = "Gets or sets the index number."]
    #[serde(
        rename = "IndexNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub index_number: Option<i32>,
    #[doc = "Gets or sets the index number end."]
    #[serde(
        rename = "IndexNumberEnd",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub index_number_end: Option<i32>,
    #[doc = "Gets or sets a value indicating whether this instance is folder."]
    #[serde(
        rename = "IsFolder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_folder: Option<bool>,
    #[doc = "Gets or sets a value indicating whether this instance is HD."]
    #[serde(
        rename = "IsHD",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_hd: Option<bool>,
    #[doc = "Gets or sets a value indicating whether this instance is kids."]
    #[serde(
        rename = "IsKids",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_kids: Option<bool>,
    #[doc = "Gets or sets a value indicating whether this instance is live."]
    #[serde(
        rename = "IsLive",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_live: Option<bool>,
    #[doc = "Gets or sets a value indicating whether this instance is movie."]
    #[serde(
        rename = "IsMovie",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_movie: Option<bool>,
    #[doc = "Gets or sets a value indicating whether this instance is news."]
    #[serde(
        rename = "IsNews",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_news: Option<bool>,
    #[doc = "Gets or sets a value indicating whether this instance is place holder."]
    #[serde(
        rename = "IsPlaceHolder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_place_holder: Option<bool>,
    #[doc = "Gets or sets a value indicating whether this instance is premiere."]
    #[serde(
        rename = "IsPremiere",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_premiere: Option<bool>,
    #[doc = "Gets or sets a value indicating whether this instance is repeat."]
    #[serde(
        rename = "IsRepeat",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_repeat: Option<bool>,
    #[doc = "Gets or sets a value indicating whether this instance is series."]
    #[serde(
        rename = "IsSeries",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_series: Option<bool>,
    #[doc = "Gets or sets a value indicating whether this instance is sports."]
    #[serde(
        rename = "IsSports",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_sports: Option<bool>,
    #[serde(
        rename = "IsoSpeedRating",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub iso_speed_rating: Option<i32>,
    #[doc = "Gets or sets the type of the iso."]
    #[serde(
        rename = "IsoType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub iso_type: Option<IsoType>,
    #[serde(
        rename = "Latitude",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub latitude: Option<f64>,
    #[doc = "Gets or sets the local trailer count."]
    #[serde(
        rename = "LocalTrailerCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub local_trailer_count: Option<i32>,
    #[doc = "Gets or sets the type of the location."]
    #[serde(
        rename = "LocationType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub location_type: Option<LocationType>,
    #[doc = "Gets or sets a value indicating whether [enable internet providers]."]
    #[serde(
        rename = "LockData",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub lock_data: Option<bool>,
    #[doc = "Gets or sets the locked fields."]
    #[serde(
        rename = "LockedFields",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub locked_fields: Option<Vec<MetadataField>>,
    #[serde(
        rename = "Longitude",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub longitude: Option<f64>,
    #[serde(
        rename = "MediaSourceCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub media_source_count: Option<i32>,
    #[doc = "Gets or sets the media versions."]
    #[serde(
        rename = "MediaSources",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub media_sources: Option<Vec<MediaSourceInfo>>,
    #[doc = "Gets or sets the media streams."]
    #[serde(
        rename = "MediaStreams",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub media_streams: Option<Vec<MediaStream>>,
    #[serde(
        rename = "MediaType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub media_type: Option<MediaType>,
    #[doc = "Gets or sets the movie count."]
    #[serde(
        rename = "MovieCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub movie_count: Option<i32>,
    #[doc = "Gets or sets the music video count."]
    #[serde(
        rename = "MusicVideoCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub music_video_count: Option<i32>,
    #[doc = "Gets or sets the name."]
    #[serde(
        rename = "Name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,
    #[doc = "Gets or sets the gain required for audio normalization."]
    #[serde(
        rename = "NormalizationGain",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub normalization_gain: Option<f32>,
    #[doc = "Gets or sets the number."]
    #[serde(
        rename = "Number",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub number: Option<String>,
    #[doc = "Gets or sets the official rating."]
    #[serde(
        rename = "OfficialRating",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub official_rating: Option<String>,
    #[serde(
        rename = "OriginalTitle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub original_title: Option<String>,
    #[doc = "Gets or sets the overview."]
    #[serde(
        rename = "Overview",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub overview: Option<String>,
    #[doc = "Gets or sets the parent art image tag."]
    #[serde(
        rename = "ParentArtImageTag",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_art_image_tag: Option<String>,
    #[doc = "Gets or sets whether the item has fan art, this will hold the Id of the Parent that has one."]
    #[serde(
        rename = "ParentArtItemId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_art_item_id: Option<uuid::Uuid>,
    #[doc = "Gets or sets the parent backdrop image tags."]
    #[serde(
        rename = "ParentBackdropImageTags",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_backdrop_image_tags:
        Option<Vec<String>>,
    #[doc = "Gets or sets whether the item has any backdrops, this will hold the Id of the Parent that has one."]
    #[serde(
        rename = "ParentBackdropItemId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_backdrop_item_id: Option<uuid::Uuid>,
    #[doc = "Gets or sets the parent id."]
    #[serde(
        rename = "ParentId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_id: Option<uuid::Uuid>,
    #[doc = "Gets or sets the parent index number."]
    #[serde(
        rename = "ParentIndexNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_index_number: Option<i32>,
    #[doc = "Gets or sets the parent logo image tag."]
    #[serde(
        rename = "ParentLogoImageTag",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_logo_image_tag: Option<String>,
    #[doc = "Gets or sets whether the item has a logo, this will hold the Id of the Parent that has one."]
    #[serde(
        rename = "ParentLogoItemId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_logo_item_id: Option<uuid::Uuid>,
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
    pub parent_thumb_item_id: Option<uuid::Uuid>,
    #[doc = "Gets or sets the part count."]
    #[serde(
        rename = "PartCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub part_count: Option<i32>,
    #[doc = "Gets or sets the path."]
    #[serde(
        rename = "Path",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub path: Option<String>,
    #[doc = "Gets or sets the people."]
    #[serde(
        rename = "People",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub people: Option<Vec<BaseItemPerson>>,
    #[doc = "Gets or sets the play access."]
    #[serde(
        rename = "PlayAccess",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub play_access: Option<PlayAccess>,
    #[doc = "Gets or sets the playlist item identifier."]
    #[serde(
        rename = "PlaylistItemId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub playlist_item_id: Option<String>,
    #[serde(
        rename = "PreferredMetadataCountryCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub preferred_metadata_country_code: Option<String>,
    #[serde(
        rename = "PreferredMetadataLanguage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub preferred_metadata_language: Option<String>,
    #[doc = "Gets or sets the premiere date."]
    #[serde(
        rename = "PremiereDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub premiere_date: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets the primary image aspect ratio, after image enhancements."]
    #[serde(
        rename = "PrimaryImageAspectRatio",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_image_aspect_ratio: Option<f64>,
    #[serde(
        rename = "ProductionLocations",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub production_locations: Option<Vec<String>>,
    #[doc = "Gets or sets the production year."]
    #[serde(
        rename = "ProductionYear",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub production_year: Option<i32>,
    #[serde(
        rename = "ProgramCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub program_count: Option<i32>,
    #[doc = "Gets or sets the program identifier."]
    #[serde(
        rename = "ProgramId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub program_id: Option<String>,
    #[doc = "Gets or sets the provider ids."]
    #[serde(
        rename = "ProviderIds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provider_ids: Option<
        std::collections::HashMap<
            String,
            Option<String>,
        >,
    >,
    #[doc = "Gets or sets the recursive item count."]
    #[serde(
        rename = "RecursiveItemCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub recursive_item_count: Option<i32>,
    #[doc = "Gets or sets the trailer urls."]
    #[serde(
        rename = "RemoteTrailers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub remote_trailers: Option<Vec<MediaUrl>>,
    #[doc = "Gets or sets the run time ticks."]
    #[serde(
        rename = "RunTimeTicks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub run_time_ticks: Option<i64>,
    #[doc = "Gets or sets the screenshot image tags."]
    #[serde(
        rename = "ScreenshotImageTags",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub screenshot_image_tags: Option<Vec<String>>,
    #[doc = "Gets or sets the season identifier."]
    #[serde(
        rename = "SeasonId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub season_id: Option<uuid::Uuid>,
    #[doc = "Gets or sets the name of the season."]
    #[serde(
        rename = "SeasonName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub season_name: Option<String>,
    #[doc = "Gets or sets the series count."]
    #[serde(
        rename = "SeriesCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub series_count: Option<i32>,
    #[doc = "Gets or sets the series id."]
    #[serde(
        rename = "SeriesId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub series_id: Option<uuid::Uuid>,
    #[doc = "Gets or sets the name of the series."]
    #[serde(
        rename = "SeriesName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub series_name: Option<String>,
    #[doc = "Gets or sets the series primary image tag."]
    #[serde(
        rename = "SeriesPrimaryImageTag",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub series_primary_image_tag: Option<String>,
    #[doc = "Gets or sets the series studio."]
    #[serde(
        rename = "SeriesStudio",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub series_studio: Option<String>,
    #[doc = "Gets or sets the series thumb image tag."]
    #[serde(
        rename = "SeriesThumbImageTag",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub series_thumb_image_tag: Option<String>,
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
    #[serde(
        rename = "ShutterSpeed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub shutter_speed: Option<f64>,
    #[serde(
        rename = "Software",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub software: Option<String>,
    #[doc = "Gets or sets the song count."]
    #[serde(
        rename = "SongCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub song_count: Option<i32>,
    #[doc = "Gets or sets the name of the sort."]
    #[serde(
        rename = "SortName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sort_name: Option<String>,
    #[doc = "Gets or sets the type of the source."]
    #[serde(
        rename = "SourceType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_type: Option<String>,
    #[doc = "Gets or sets the special feature count."]
    #[serde(
        rename = "SpecialFeatureCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub special_feature_count: Option<i32>,
    #[doc = "Gets or sets the start date of the recording, in UTC."]
    #[serde(
        rename = "StartDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets the status."]
    #[serde(
        rename = "Status",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub status: Option<String>,
    #[doc = "Gets or sets the studios."]
    #[serde(
        rename = "Studios",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub studios: Option<Vec<NameGuidPair>>,
    #[doc = "Gets or sets the taglines."]
    #[serde(
        rename = "Taglines",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub taglines: Option<Vec<String>>,
    #[doc = "Gets or sets the tags."]
    #[serde(
        rename = "Tags",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tags: Option<Vec<String>>,
    #[doc = "Gets or sets the timer identifier."]
    #[serde(
        rename = "TimerId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub timer_id: Option<String>,
    #[doc = "Gets or sets the trailer count."]
    #[serde(
        rename = "TrailerCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailer_count: Option<i32>,
    #[doc = "Gets or sets the trickplay manifest."]
    #[serde(
        rename = "Trickplay",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trickplay: Option<
        std::collections::HashMap<
            String,
            std::collections::HashMap<String, TrickplayInfoDto>,
        >,
    >,
    #[serde(
        rename = "Type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_: Option<BaseItemKind>,
    #[doc = "Gets or sets the user data for this item based on the user it's being requested for."]
    #[serde(
        rename = "UserData",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_data: Option<UserItemDataDto>,
    #[doc = "Gets or sets the video3 D format."]
    #[serde(
        rename = "Video3DFormat",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub video3_d_format: Option<Video3DFormat>,
    #[doc = "Gets or sets the type of the video."]
    #[serde(
        rename = "VideoType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub video_type: Option<VideoType>,
    #[serde(
        rename = "Width",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub width: Option<i32>,
}

impl Default for BaseItemDto {
    fn default() -> Self {
        Self {
            air_days: Default::default(),
            air_time: Default::default(),
            airs_after_season_number: Default::default(),
            airs_before_episode_number: Default::default(),
            airs_before_season_number: Default::default(),
            album: Default::default(),
            album_artist: Default::default(),
            album_artists: Default::default(),
            album_count: Default::default(),
            album_id: Default::default(),
            album_primary_image_tag: Default::default(),
            altitude: Default::default(),
            aperture: Default::default(),
            artist_count: Default::default(),
            artist_items: Default::default(),
            artists: Default::default(),
            aspect_ratio: Default::default(),
            audio: Default::default(),
            backdrop_image_tags: Default::default(),
            camera_make: Default::default(),
            camera_model: Default::default(),
            can_delete: Default::default(),
            can_download: Default::default(),
            channel_id: Default::default(),
            channel_name: Default::default(),
            channel_number: Default::default(),
            channel_primary_image_tag: Default::default(),
            channel_type: Default::default(),
            chapters: Default::default(),
            child_count: Default::default(),
            collection_type: Default::default(),
            community_rating: Default::default(),
            completion_percentage: Default::default(),
            container: Default::default(),
            critic_rating: Default::default(),
            cumulative_run_time_ticks: Default::default(),
            current_program: Default::default(),
            custom_rating: Default::default(),
            date_created: Default::default(),
            date_last_media_added: Default::default(),
            display_order: Default::default(),
            display_preferences_id: Default::default(),
            enable_media_source_display: Default::default(),
            end_date: Default::default(),
            episode_count: Default::default(),
            episode_title: Default::default(),
            etag: Default::default(),
            exposure_time: Default::default(),
            external_urls: Default::default(),
            extra_type: Default::default(),
            focal_length: Default::default(),
            forced_sort_name: Default::default(),
            genre_items: Default::default(),
            genres: Default::default(),
            has_lyrics: Default::default(),
            has_subtitles: Default::default(),
            height: Default::default(),
            id: Default::default(),
            image_blur_hashes: Default::default(),
            image_orientation: Default::default(),
            image_tags: Default::default(),
            index_number: Default::default(),
            index_number_end: Default::default(),
            is_folder: Default::default(),
            is_hd: Default::default(),
            is_kids: Default::default(),
            is_live: Default::default(),
            is_movie: Default::default(),
            is_news: Default::default(),
            is_place_holder: Default::default(),
            is_premiere: Default::default(),
            is_repeat: Default::default(),
            is_series: Default::default(),
            is_sports: Default::default(),
            iso_speed_rating: Default::default(),
            iso_type: Default::default(),
            latitude: Default::default(),
            local_trailer_count: Default::default(),
            location_type: Default::default(),
            lock_data: Default::default(),
            locked_fields: Default::default(),
            longitude: Default::default(),
            media_source_count: Default::default(),
            media_sources: Default::default(),
            media_streams: Default::default(),
            media_type: Default::default(),
            movie_count: Default::default(),
            music_video_count: Default::default(),
            name: Default::default(),
            normalization_gain: Default::default(),
            number: Default::default(),
            official_rating: Default::default(),
            original_title: Default::default(),
            overview: Default::default(),
            parent_art_image_tag: Default::default(),
            parent_art_item_id: Default::default(),
            parent_backdrop_image_tags: Default::default(),
            parent_backdrop_item_id: Default::default(),
            parent_id: Default::default(),
            parent_index_number: Default::default(),
            parent_logo_image_tag: Default::default(),
            parent_logo_item_id: Default::default(),
            parent_primary_image_item_id: Default::default(),
            parent_primary_image_tag: Default::default(),
            parent_thumb_image_tag: Default::default(),
            parent_thumb_item_id: Default::default(),
            part_count: Default::default(),
            path: Default::default(),
            people: Default::default(),
            play_access: Default::default(),
            playlist_item_id: Default::default(),
            preferred_metadata_country_code: Default::default(),
            preferred_metadata_language: Default::default(),
            premiere_date: Default::default(),
            primary_image_aspect_ratio: Default::default(),
            production_locations: Default::default(),
            production_year: Default::default(),
            program_count: Default::default(),
            program_id: Default::default(),
            provider_ids: Default::default(),
            recursive_item_count: Default::default(),
            remote_trailers: Default::default(),
            run_time_ticks: Default::default(),
            screenshot_image_tags: Default::default(),
            season_id: Default::default(),
            season_name: Default::default(),
            series_count: Default::default(),
            series_id: Default::default(),
            series_name: Default::default(),
            series_primary_image_tag: Default::default(),
            series_studio: Default::default(),
            series_thumb_image_tag: Default::default(),
            series_timer_id: Default::default(),
            server_id: Default::default(),
            shutter_speed: Default::default(),
            software: Default::default(),
            song_count: Default::default(),
            sort_name: Default::default(),
            source_type: Default::default(),
            special_feature_count: Default::default(),
            start_date: Default::default(),
            status: Default::default(),
            studios: Default::default(),
            taglines: Default::default(),
            tags: Default::default(),
            timer_id: Default::default(),
            trailer_count: Default::default(),
            trickplay: Default::default(),
            type_: Default::default(),
            user_data: Default::default(),
            video3_d_format: Default::default(),
            video_type: Default::default(),
            width: Default::default(),
        }
    }
}

#[doc = "Gets or sets the blurhashes for the image tags.\r\nMaps image type to dictionary mapping image tag to blurhash value."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct BaseItemDtoImageBlurHashes {
    #[serde(
        rename = "Art",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub art: std::collections::HashMap<String, String>,
    #[serde(
        rename = "Backdrop",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub backdrop: std::collections::HashMap<String, String>,
    #[serde(
        rename = "Banner",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub banner: std::collections::HashMap<String, String>,
    #[serde(
        rename = "Box",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub box_: std::collections::HashMap<String, String>,
    #[serde(
        rename = "BoxRear",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub box_rear: std::collections::HashMap<String, String>,
    #[serde(
        rename = "Chapter",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub chapter: std::collections::HashMap<String, String>,
    #[serde(
        rename = "Disc",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub disc: std::collections::HashMap<String, String>,
    #[serde(
        rename = "Logo",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub logo: std::collections::HashMap<String, String>,
    #[serde(
        rename = "Menu",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub menu: std::collections::HashMap<String, String>,
    #[serde(
        rename = "Primary",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub primary: std::collections::HashMap<String, String>,
    #[serde(
        rename = "Profile",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub profile: std::collections::HashMap<String, String>,
    #[serde(
        rename = "Screenshot",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub screenshot: std::collections::HashMap<String, String>,
    #[serde(
        rename = "Thumb",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub thumb: std::collections::HashMap<String, String>,
}

impl Default for BaseItemDtoImageBlurHashes {
    fn default() -> Self {
        Self {
            art: Default::default(),
            backdrop: Default::default(),
            banner: Default::default(),
            box_: Default::default(),
            box_rear: Default::default(),
            chapter: Default::default(),
            disc: Default::default(),
            logo: Default::default(),
            menu: Default::default(),
            primary: Default::default(),
            profile: Default::default(),
            screenshot: Default::default(),
            thumb: Default::default(),
        }
    }
}

#[doc = "Query result container."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct BaseItemDtoQueryResult {
    #[doc = "Gets or sets the items."]
    #[serde(
        rename = "Items",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<BaseItemDto>,
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

impl Default for BaseItemDtoQueryResult {
    fn default() -> Self {
        Self {
            items: Default::default(),
            start_index: Default::default(),
            total_record_count: Default::default(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BaseItemKind {
    AggregateFolder,
    Audio,
    AudioBook,
    BasePluginFolder,
    Book,
    BoxSet,
    Channel,
    ChannelFolderItem,
    CollectionFolder,
    Episode,
    Folder,
    Genre,
    ManualPlaylistsFolder,
    Movie,
    LiveTvChannel,
    LiveTvProgram,
    MusicAlbum,
    MusicArtist,
    MusicGenre,
    MusicVideo,
    Person,
    Photo,
    PhotoAlbum,
    Playlist,
    PlaylistsFolder,
    Program,
    Recording,
    Season,
    Series,
    Studio,
    Trailer,
    TvChannel,
    TvProgram,
    UserRootFolder,
    UserView,
    Video,
    Year,
}

impl std::fmt::Display for BaseItemKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::AggregateFolder => f.write_str("AggregateFolder"),
            Self::Audio => f.write_str("Audio"),
            Self::AudioBook => f.write_str("AudioBook"),
            Self::BasePluginFolder => f.write_str("BasePluginFolder"),
            Self::Book => f.write_str("Book"),
            Self::BoxSet => f.write_str("BoxSet"),
            Self::Channel => f.write_str("Channel"),
            Self::ChannelFolderItem => f.write_str("ChannelFolderItem"),
            Self::CollectionFolder => f.write_str("CollectionFolder"),
            Self::Episode => f.write_str("Episode"),
            Self::Folder => f.write_str("Folder"),
            Self::Genre => f.write_str("Genre"),
            Self::ManualPlaylistsFolder => f.write_str("ManualPlaylistsFolder"),
            Self::Movie => f.write_str("Movie"),
            Self::LiveTvChannel => f.write_str("LiveTvChannel"),
            Self::LiveTvProgram => f.write_str("LiveTvProgram"),
            Self::MusicAlbum => f.write_str("MusicAlbum"),
            Self::MusicArtist => f.write_str("MusicArtist"),
            Self::MusicGenre => f.write_str("MusicGenre"),
            Self::MusicVideo => f.write_str("MusicVideo"),
            Self::Person => f.write_str("Person"),
            Self::Photo => f.write_str("Photo"),
            Self::PhotoAlbum => f.write_str("PhotoAlbum"),
            Self::Playlist => f.write_str("Playlist"),
            Self::PlaylistsFolder => f.write_str("PlaylistsFolder"),
            Self::Program => f.write_str("Program"),
            Self::Recording => f.write_str("Recording"),
            Self::Season => f.write_str("Season"),
            Self::Series => f.write_str("Series"),
            Self::Studio => f.write_str("Studio"),
            Self::Trailer => f.write_str("Trailer"),
            Self::TvChannel => f.write_str("TvChannel"),
            Self::TvProgram => f.write_str("TvProgram"),
            Self::UserRootFolder => f.write_str("UserRootFolder"),
            Self::UserView => f.write_str("UserView"),
            Self::Video => f.write_str("Video"),
            Self::Year => f.write_str("Year"),
        }
    }
}

impl std::str::FromStr for BaseItemKind {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "AggregateFolder" => Ok(Self::AggregateFolder),
            "Audio" => Ok(Self::Audio),
            "AudioBook" => Ok(Self::AudioBook),
            "BasePluginFolder" => Ok(Self::BasePluginFolder),
            "Book" => Ok(Self::Book),
            "BoxSet" => Ok(Self::BoxSet),
            "Channel" => Ok(Self::Channel),
            "ChannelFolderItem" => Ok(Self::ChannelFolderItem),
            "CollectionFolder" => Ok(Self::CollectionFolder),
            "Episode" => Ok(Self::Episode),
            "Folder" => Ok(Self::Folder),
            "Genre" => Ok(Self::Genre),
            "ManualPlaylistsFolder" => Ok(Self::ManualPlaylistsFolder),
            "Movie" => Ok(Self::Movie),
            "LiveTvChannel" => Ok(Self::LiveTvChannel),
            "LiveTvProgram" => Ok(Self::LiveTvProgram),
            "MusicAlbum" => Ok(Self::MusicAlbum),
            "MusicArtist" => Ok(Self::MusicArtist),
            "MusicGenre" => Ok(Self::MusicGenre),
            "MusicVideo" => Ok(Self::MusicVideo),
            "Person" => Ok(Self::Person),
            "Photo" => Ok(Self::Photo),
            "PhotoAlbum" => Ok(Self::PhotoAlbum),
            "Playlist" => Ok(Self::Playlist),
            "PlaylistsFolder" => Ok(Self::PlaylistsFolder),
            "Program" => Ok(Self::Program),
            "Recording" => Ok(Self::Recording),
            "Season" => Ok(Self::Season),
            "Series" => Ok(Self::Series),
            "Studio" => Ok(Self::Studio),
            "Trailer" => Ok(Self::Trailer),
            "TvChannel" => Ok(Self::TvChannel),
            "TvProgram" => Ok(Self::TvProgram),
            "UserRootFolder" => Ok(Self::UserRootFolder),
            "UserView" => Ok(Self::UserView),
            "Video" => Ok(Self::Video),
            "Year" => Ok(Self::Year),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for BaseItemKind {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for BaseItemKind {
    type Error = super::error::ConversionError;
    fn try_from(
        value: &String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for BaseItemKind {
    type Error = super::error::ConversionError;
    fn try_from(
        value: String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "This is used by the api to get information about a Person within a BaseItem."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct BaseItemPerson {
    #[doc = "Gets or sets the identifier."]
    #[serde(
        rename = "Id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<uuid::Uuid>,
    #[doc = "Gets or sets the primary image blurhash."]
    #[serde(
        rename = "ImageBlurHashes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub image_blur_hashes: Option<BaseItemPersonImageBlurHashes>,
    #[doc = "Gets or sets the name."]
    #[serde(
        rename = "Name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,
    #[doc = "Gets or sets the primary image tag."]
    #[serde(
        rename = "PrimaryImageTag",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_image_tag: Option<String>,
    #[doc = "Gets or sets the role."]
    #[serde(
        rename = "Role",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub role: Option<String>,
    #[serde(
        rename = "Type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_: Option<PersonKind>,
}

impl Default for BaseItemPerson {
    fn default() -> Self {
        Self {
            id: Default::default(),
            image_blur_hashes: Default::default(),
            name: Default::default(),
            primary_image_tag: Default::default(),
            role: Default::default(),
            type_: Default::default(),
        }
    }
}

#[doc = "Gets or sets the primary image blurhash."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct BaseItemPersonImageBlurHashes {
    #[serde(
        rename = "Art",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub art: std::collections::HashMap<String, String>,
    #[serde(
        rename = "Backdrop",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub backdrop: std::collections::HashMap<String, String>,
    #[serde(
        rename = "Banner",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub banner: std::collections::HashMap<String, String>,
    #[serde(
        rename = "Box",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub box_: std::collections::HashMap<String, String>,
    #[serde(
        rename = "BoxRear",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub box_rear: std::collections::HashMap<String, String>,
    #[serde(
        rename = "Chapter",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub chapter: std::collections::HashMap<String, String>,
    #[serde(
        rename = "Disc",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub disc: std::collections::HashMap<String, String>,
    #[serde(
        rename = "Logo",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub logo: std::collections::HashMap<String, String>,
    #[serde(
        rename = "Menu",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub menu: std::collections::HashMap<String, String>,
    #[serde(
        rename = "Primary",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub primary: std::collections::HashMap<String, String>,
    #[serde(
        rename = "Profile",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub profile: std::collections::HashMap<String, String>,
    #[serde(
        rename = "Screenshot",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub screenshot: std::collections::HashMap<String, String>,
    #[serde(
        rename = "Thumb",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub thumb: std::collections::HashMap<String, String>,
}

impl Default for BaseItemPersonImageBlurHashes {
    fn default() -> Self {
        Self {
            art: Default::default(),
            backdrop: Default::default(),
            banner: Default::default(),
            box_: Default::default(),
            box_rear: Default::default(),
            chapter: Default::default(),
            disc: Default::default(),
            logo: Default::default(),
            menu: Default::default(),
            primary: Default::default(),
            profile: Default::default(),
            screenshot: Default::default(),
            thumb: Default::default(),
        }
    }
}

#[doc = "Class ChapterInfo."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ChapterInfo {
    #[serde(
        rename = "ImageDateModified",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub image_date_modified: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets the image path."]
    #[serde(
        rename = "ImagePath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub image_path: Option<String>,
    #[serde(
        rename = "ImageTag",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub image_tag: Option<String>,
    #[doc = "Gets or sets the name."]
    #[serde(
        rename = "Name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,
    #[doc = "Gets or sets the start position ticks."]
    #[serde(
        rename = "StartPositionTicks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_position_ticks: Option<i64>,
}

impl Default for ChapterInfo {
    fn default() -> Self {
        Self {
            image_date_modified: Default::default(),
            image_path: Default::default(),
            image_tag: Default::default(),
            name: Default::default(),
            start_position_ticks: Default::default(),
        }
    }
}

#[doc = "Represents the external id information for serialization to the client."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ExternalIdInfo {
    #[doc = "Gets or sets the unique key for this id. This key should be unique across all providers."]
    #[serde(
        rename = "Key",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub key: Option<String>,
    #[doc = "Gets or sets the display name of the external id provider (IE: IMDB, MusicBrainz, etc)."]
    #[serde(
        rename = "Name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,
    #[doc = "Gets or sets the specific media type for this id. This is used to distinguish between the different\r\nexternal id types for providers with multiple ids.\r\nA null value indicates there is no specific media type associated with the external id, or this is the\r\ndefault id for the external provider so there is no need to specify a type."]
    #[serde(
        rename = "Type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_: Option<ExternalIdMediaType>,
}

impl Default for ExternalIdInfo {
    fn default() -> Self {
        Self {
            key: Default::default(),
            name: Default::default(),
            type_: Default::default(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ExternalIdMediaType {
    Album,
    AlbumArtist,
    Artist,
    BoxSet,
    Episode,
    Movie,
    OtherArtist,
    Person,
    ReleaseGroup,
    Season,
    Series,
    Track,
    Book,
    Recording,
}

impl std::fmt::Display for ExternalIdMediaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Album => f.write_str("Album"),
            Self::AlbumArtist => f.write_str("AlbumArtist"),
            Self::Artist => f.write_str("Artist"),
            Self::BoxSet => f.write_str("BoxSet"),
            Self::Episode => f.write_str("Episode"),
            Self::Movie => f.write_str("Movie"),
            Self::OtherArtist => f.write_str("OtherArtist"),
            Self::Person => f.write_str("Person"),
            Self::ReleaseGroup => f.write_str("ReleaseGroup"),
            Self::Season => f.write_str("Season"),
            Self::Series => f.write_str("Series"),
            Self::Track => f.write_str("Track"),
            Self::Book => f.write_str("Book"),
            Self::Recording => f.write_str("Recording"),
        }
    }
}

impl std::str::FromStr for ExternalIdMediaType {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Album" => Ok(Self::Album),
            "AlbumArtist" => Ok(Self::AlbumArtist),
            "Artist" => Ok(Self::Artist),
            "BoxSet" => Ok(Self::BoxSet),
            "Episode" => Ok(Self::Episode),
            "Movie" => Ok(Self::Movie),
            "OtherArtist" => Ok(Self::OtherArtist),
            "Person" => Ok(Self::Person),
            "ReleaseGroup" => Ok(Self::ReleaseGroup),
            "Season" => Ok(Self::Season),
            "Series" => Ok(Self::Series),
            "Track" => Ok(Self::Track),
            "Book" => Ok(Self::Book),
            "Recording" => Ok(Self::Recording),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for ExternalIdMediaType {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for ExternalIdMediaType {
    type Error = super::error::ConversionError;
    fn try_from(
        value: &String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for ExternalIdMediaType {
    type Error = super::error::ConversionError;
    fn try_from(
        value: String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "`ExternalUrl`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ExternalUrl {
    #[doc = "Gets or sets the name."]
    #[serde(
        rename = "Name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,
    #[doc = "Gets or sets the type of the item."]
    #[serde(
        rename = "Url",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub url: Option<String>,
}

impl Default for ExternalUrl {
    fn default() -> Self {
        Self {
            name: Default::default(),
            url: Default::default(),
        }
    }
}

#[doc = "Class LibrarySummary."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ItemCounts {
    #[doc = "Gets or sets the album count."]
    #[serde(
        rename = "AlbumCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub album_count: Option<i32>,
    #[doc = "Gets or sets the artist count."]
    #[serde(
        rename = "ArtistCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artist_count: Option<i32>,
    #[doc = "Gets or sets the book count."]
    #[serde(
        rename = "BookCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub book_count: Option<i32>,
    #[doc = "Gets or sets the box set count."]
    #[serde(
        rename = "BoxSetCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub box_set_count: Option<i32>,
    #[doc = "Gets or sets the episode count."]
    #[serde(
        rename = "EpisodeCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub episode_count: Option<i32>,
    #[doc = "Gets or sets the item count."]
    #[serde(
        rename = "ItemCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub item_count: Option<i32>,
    #[doc = "Gets or sets the movie count."]
    #[serde(
        rename = "MovieCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub movie_count: Option<i32>,
    #[doc = "Gets or sets the music video count."]
    #[serde(
        rename = "MusicVideoCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub music_video_count: Option<i32>,
    #[doc = "Gets or sets the program count."]
    #[serde(
        rename = "ProgramCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub program_count: Option<i32>,
    #[doc = "Gets or sets the series count."]
    #[serde(
        rename = "SeriesCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub series_count: Option<i32>,
    #[doc = "Gets or sets the song count."]
    #[serde(
        rename = "SongCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub song_count: Option<i32>,
    #[doc = "Gets or sets the trailer count."]
    #[serde(
        rename = "TrailerCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailer_count: Option<i32>,
}

impl Default for ItemCounts {
    fn default() -> Self {
        Self {
            album_count: Default::default(),
            artist_count: Default::default(),
            book_count: Default::default(),
            box_set_count: Default::default(),
            episode_count: Default::default(),
            item_count: Default::default(),
            movie_count: Default::default(),
            music_video_count: Default::default(),
            program_count: Default::default(),
            series_count: Default::default(),
            song_count: Default::default(),
            trailer_count: Default::default(),
        }
    }
}

#[doc = "Class MediaAttachment."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct MediaAttachment {
    #[doc = "Gets or sets the codec."]
    #[serde(
        rename = "Codec",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub codec: Option<String>,
    #[doc = "Gets or sets the codec tag."]
    #[serde(
        rename = "CodecTag",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub codec_tag: Option<String>,
    #[doc = "Gets or sets the comment."]
    #[serde(
        rename = "Comment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub comment: Option<String>,
    #[doc = "Gets or sets the delivery URL."]
    #[serde(
        rename = "DeliveryUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_url: Option<String>,
    #[doc = "Gets or sets the filename."]
    #[serde(
        rename = "FileName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub file_name: Option<String>,
    #[doc = "Gets or sets the index."]
    #[serde(
        rename = "Index",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub index: Option<i32>,
    #[doc = "Gets or sets the MIME type."]
    #[serde(
        rename = "MimeType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mime_type: Option<String>,
}

impl Default for MediaAttachment {
    fn default() -> Self {
        Self {
            codec: Default::default(),
            codec_tag: Default::default(),
            comment: Default::default(),
            delivery_url: Default::default(),
            file_name: Default::default(),
            index: Default::default(),
            mime_type: Default::default(),
        }
    }
}

#[doc = "Api model for MediaSegment's."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct MediaSegmentDto {
    #[doc = "Gets or sets the end of the segment."]
    #[serde(
        rename = "EndTicks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub end_ticks: Option<i64>,
    #[doc = "Gets or sets the id of the media segment."]
    #[serde(
        rename = "Id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<uuid::Uuid>,
    #[doc = "Gets or sets the id of the associated item."]
    #[serde(
        rename = "ItemId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub item_id: Option<uuid::Uuid>,
    #[doc = "Gets or sets the start of the segment."]
    #[serde(
        rename = "StartTicks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_ticks: Option<i64>,
    #[serde(
        rename = "Type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_: Option<MediaSegmentType>,
}

impl Default for MediaSegmentDto {
    fn default() -> Self {
        Self {
            end_ticks: Default::default(),
            id: Default::default(),
            item_id: Default::default(),
            start_ticks: Default::default(),
            type_: Default::default(),
        }
    }
}

#[doc = "Query result container."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct MediaSegmentDtoQueryResult {
    #[doc = "Gets or sets the items."]
    #[serde(
        rename = "Items",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<MediaSegmentDto>,
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

impl Default for MediaSegmentDtoQueryResult {
    fn default() -> Self {
        Self {
            items: Default::default(),
            start_index: Default::default(),
            total_record_count: Default::default(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MediaSegmentType {
    Unknown,
    Commercial,
    Preview,
    Recap,
    Outro,
    Intro,
}

impl std::fmt::Display for MediaSegmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Unknown => f.write_str("Unknown"),
            Self::Commercial => f.write_str("Commercial"),
            Self::Preview => f.write_str("Preview"),
            Self::Recap => f.write_str("Recap"),
            Self::Outro => f.write_str("Outro"),
            Self::Intro => f.write_str("Intro"),
        }
    }
}

impl std::str::FromStr for MediaSegmentType {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Unknown" => Ok(Self::Unknown),
            "Commercial" => Ok(Self::Commercial),
            "Preview" => Ok(Self::Preview),
            "Recap" => Ok(Self::Recap),
            "Outro" => Ok(Self::Outro),
            "Intro" => Ok(Self::Intro),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for MediaSegmentType {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for MediaSegmentType {
    type Error = super::error::ConversionError;
    fn try_from(
        value: &String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for MediaSegmentType {
    type Error = super::error::ConversionError;
    fn try_from(
        value: String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "`MediaSourceInfo`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct MediaSourceInfo {
    #[serde(
        rename = "AnalyzeDurationMs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub analyze_duration_ms: Option<i32>,
    #[serde(
        rename = "Bitrate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bitrate: Option<i32>,
    #[serde(
        rename = "BufferMs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub buffer_ms: Option<i32>,
    #[serde(
        rename = "Container",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub container: Option<String>,
    #[serde(
        rename = "DefaultAudioStreamIndex",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_audio_stream_index: Option<i32>,
    #[serde(
        rename = "DefaultSubtitleStreamIndex",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_subtitle_stream_index: Option<i32>,
    #[serde(
        rename = "ETag",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub e_tag: Option<String>,
    #[serde(
        rename = "EncoderPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub encoder_path: Option<String>,
    #[serde(
        rename = "EncoderProtocol",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub encoder_protocol: Option<MediaProtocol>,
    #[serde(
        rename = "FallbackMaxStreamingBitrate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fallback_max_streaming_bitrate: Option<i32>,
    #[serde(
        rename = "Formats",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub formats: Option<Vec<String>>,
    #[serde(
        rename = "GenPtsInput",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub gen_pts_input: Option<bool>,
    #[serde(
        rename = "HasSegments",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub has_segments: Option<bool>,
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
        rename = "IgnoreIndex",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_index: Option<bool>,
    #[serde(
        rename = "IsInfiniteStream",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_infinite_stream: Option<bool>,
    #[doc = "Gets or sets a value indicating whether the media is remote.\r\nDifferentiate internet url vs local network."]
    #[serde(
        rename = "IsRemote",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_remote: Option<bool>,
    #[serde(
        rename = "IsoType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub iso_type: Option<IsoType>,
    #[serde(
        rename = "LiveStreamId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub live_stream_id: Option<String>,
    #[serde(
        rename = "MediaAttachments",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub media_attachments: Option<Vec<MediaAttachment>>,
    #[serde(
        rename = "MediaStreams",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub media_streams: Option<Vec<MediaStream>>,
    #[serde(
        rename = "Name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,
    #[serde(
        rename = "OpenToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub open_token: Option<String>,
    #[serde(
        rename = "Path",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub path: Option<String>,
    #[serde(
        rename = "Protocol",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub protocol: Option<MediaProtocol>,
    #[serde(
        rename = "ReadAtNativeFramerate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub read_at_native_framerate: Option<bool>,
    #[serde(
        rename = "RequiredHttpHeaders",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub required_http_headers: Option<
        std::collections::HashMap<
            String,
            Option<String>,
        >,
    >,
    #[serde(
        rename = "RequiresClosing",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub requires_closing: Option<bool>,
    #[serde(
        rename = "RequiresLooping",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub requires_looping: Option<bool>,
    #[serde(
        rename = "RequiresOpening",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub requires_opening: Option<bool>,
    #[serde(
        rename = "RunTimeTicks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub run_time_ticks: Option<i64>,
    #[serde(
        rename = "Size",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub size: Option<i64>,
    #[serde(
        rename = "SupportsDirectPlay",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_direct_play: Option<bool>,
    #[serde(
        rename = "SupportsDirectStream",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_direct_stream: Option<bool>,
    #[serde(
        rename = "SupportsProbing",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_probing: Option<bool>,
    #[serde(
        rename = "SupportsTranscoding",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_transcoding: Option<bool>,
    #[serde(
        rename = "Timestamp",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub timestamp: Option<TransportStreamTimestamp>,
    #[serde(
        rename = "TranscodingContainer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transcoding_container: Option<String>,
    #[serde(
        rename = "TranscodingSubProtocol",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transcoding_sub_protocol: Option<MediaStreamProtocol>,
    #[serde(
        rename = "TranscodingUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transcoding_url: Option<String>,
    #[serde(
        rename = "Type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_: Option<MediaSourceType>,
    #[serde(rename = "UseMostCompatibleTranscodingProfile", default)]
    pub use_most_compatible_transcoding_profile: bool,
    #[serde(
        rename = "Video3DFormat",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub video3_d_format: Option<Video3DFormat>,
    #[serde(
        rename = "VideoType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub video_type: Option<VideoType>,
}

impl Default for MediaSourceInfo {
    fn default() -> Self {
        Self {
            analyze_duration_ms: Default::default(),
            bitrate: Default::default(),
            buffer_ms: Default::default(),
            container: Default::default(),
            default_audio_stream_index: Default::default(),
            default_subtitle_stream_index: Default::default(),
            e_tag: Default::default(),
            encoder_path: Default::default(),
            encoder_protocol: Default::default(),
            fallback_max_streaming_bitrate: Default::default(),
            formats: Default::default(),
            gen_pts_input: Default::default(),
            has_segments: Default::default(),
            id: Default::default(),
            ignore_dts: Default::default(),
            ignore_index: Default::default(),
            is_infinite_stream: Default::default(),
            is_remote: Default::default(),
            iso_type: Default::default(),
            live_stream_id: Default::default(),
            media_attachments: Default::default(),
            media_streams: Default::default(),
            name: Default::default(),
            open_token: Default::default(),
            path: Default::default(),
            protocol: Default::default(),
            read_at_native_framerate: Default::default(),
            required_http_headers: Default::default(),
            requires_closing: Default::default(),
            requires_looping: Default::default(),
            requires_opening: Default::default(),
            run_time_ticks: Default::default(),
            size: Default::default(),
            supports_direct_play: Default::default(),
            supports_direct_stream: Default::default(),
            supports_probing: Default::default(),
            supports_transcoding: Default::default(),
            timestamp: Default::default(),
            transcoding_container: Default::default(),
            transcoding_sub_protocol: Default::default(),
            transcoding_url: Default::default(),
            type_: Default::default(),
            use_most_compatible_transcoding_profile: Default::default(),
            video3_d_format: Default::default(),
            video_type: Default::default(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MediaSourceType {
    Default,
    Grouping,
    Placeholder,
}

impl std::fmt::Display for MediaSourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Default => f.write_str("Default"),
            Self::Grouping => f.write_str("Grouping"),
            Self::Placeholder => f.write_str("Placeholder"),
        }
    }
}

impl std::str::FromStr for MediaSourceType {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Default" => Ok(Self::Default),
            "Grouping" => Ok(Self::Grouping),
            "Placeholder" => Ok(Self::Placeholder),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for MediaSourceType {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for MediaSourceType {
    type Error = super::error::ConversionError;
    fn try_from(
        value: &String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for MediaSourceType {
    type Error = super::error::ConversionError;
    fn try_from(
        value: String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "Class MediaStream."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct MediaStream {
    #[doc = "Gets or sets the aspect ratio."]
    #[serde(
        rename = "AspectRatio",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub aspect_ratio: Option<String>,
    #[serde(
        rename = "AudioSpatialFormat",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub audio_spatial_format: Option<AudioSpatialFormat>,
    #[doc = "Gets or sets the average frame rate."]
    #[serde(
        rename = "AverageFrameRate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub average_frame_rate: Option<f32>,
    #[doc = "Gets or sets the bit depth."]
    #[serde(
        rename = "BitDepth",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bit_depth: Option<i32>,
    #[doc = "Gets or sets the bit rate."]
    #[serde(
        rename = "BitRate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bit_rate: Option<i32>,
    #[doc = "Gets or sets the Dolby Vision bl present flag."]
    #[serde(
        rename = "BlPresentFlag",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bl_present_flag: Option<i32>,
    #[doc = "Gets or sets the channel layout."]
    #[serde(
        rename = "ChannelLayout",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_layout: Option<String>,
    #[doc = "Gets or sets the channels."]
    #[serde(
        rename = "Channels",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub channels: Option<i32>,
    #[doc = "Gets or sets the codec."]
    #[serde(
        rename = "Codec",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub codec: Option<String>,
    #[doc = "Gets or sets the codec tag."]
    #[serde(
        rename = "CodecTag",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub codec_tag: Option<String>,
    #[doc = "Gets or sets the codec time base."]
    #[serde(
        rename = "CodecTimeBase",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub codec_time_base: Option<String>,
    #[doc = "Gets or sets the color primaries."]
    #[serde(
        rename = "ColorPrimaries",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub color_primaries: Option<String>,
    #[doc = "Gets or sets the color range."]
    #[serde(
        rename = "ColorRange",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub color_range: Option<String>,
    #[doc = "Gets or sets the color space."]
    #[serde(
        rename = "ColorSpace",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub color_space: Option<String>,
    #[doc = "Gets or sets the color transfer."]
    #[serde(
        rename = "ColorTransfer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub color_transfer: Option<String>,
    #[doc = "Gets or sets the comment."]
    #[serde(
        rename = "Comment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub comment: Option<String>,
    #[doc = "Gets or sets the method."]
    #[serde(
        rename = "DeliveryMethod",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_method: Option<SubtitleDeliveryMethod>,
    #[doc = "Gets or sets the delivery URL."]
    #[serde(
        rename = "DeliveryUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_url: Option<String>,
    #[serde(
        rename = "DisplayTitle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_title: Option<String>,
    #[doc = "Gets or sets the Dolby Vision bl signal compatibility id."]
    #[serde(
        rename = "DvBlSignalCompatibilityId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dv_bl_signal_compatibility_id: Option<i32>,
    #[doc = "Gets or sets the Dolby Vision level."]
    #[serde(
        rename = "DvLevel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dv_level: Option<i32>,
    #[doc = "Gets or sets the Dolby Vision profile."]
    #[serde(
        rename = "DvProfile",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dv_profile: Option<i32>,
    #[doc = "Gets or sets the Dolby Vision version major."]
    #[serde(
        rename = "DvVersionMajor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dv_version_major: Option<i32>,
    #[doc = "Gets or sets the Dolby Vision version minor."]
    #[serde(
        rename = "DvVersionMinor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dv_version_minor: Option<i32>,
    #[doc = "Gets or sets the Dolby Vision el present flag."]
    #[serde(
        rename = "ElPresentFlag",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub el_present_flag: Option<i32>,
    #[serde(
        rename = "Hdr10PlusPresentFlag",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hdr10_plus_present_flag: Option<bool>,
    #[doc = "Gets or sets the height."]
    #[serde(
        rename = "Height",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub height: Option<i32>,
    #[doc = "Gets or sets the index."]
    #[serde(
        rename = "Index",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub index: Option<i32>,
    #[doc = "Gets or sets whether this instance is anamorphic."]
    #[serde(
        rename = "IsAnamorphic",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_anamorphic: Option<bool>,
    #[serde(
        rename = "IsAVC",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_avc: Option<bool>,
    #[doc = "Gets or sets a value indicating whether this instance is default."]
    #[serde(
        rename = "IsDefault",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_default: Option<bool>,
    #[doc = "Gets or sets a value indicating whether this instance is external."]
    #[serde(
        rename = "IsExternal",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_external: Option<bool>,
    #[doc = "Gets or sets a value indicating whether this instance is external URL."]
    #[serde(
        rename = "IsExternalUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_external_url: Option<bool>,
    #[doc = "Gets or sets a value indicating whether this instance is forced."]
    #[serde(
        rename = "IsForced",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_forced: Option<bool>,
    #[doc = "Gets or sets a value indicating whether this instance is for the hearing impaired."]
    #[serde(
        rename = "IsHearingImpaired",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_hearing_impaired: Option<bool>,
    #[doc = "Gets or sets a value indicating whether this instance is interlaced."]
    #[serde(
        rename = "IsInterlaced",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_interlaced: Option<bool>,
    #[serde(
        rename = "IsTextSubtitleStream",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_text_subtitle_stream: Option<bool>,
    #[doc = "Gets or sets the language."]
    #[serde(
        rename = "Language",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub language: Option<String>,
    #[doc = "Gets or sets the level."]
    #[serde(
        rename = "Level",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub level: Option<f64>,
    #[serde(
        rename = "LocalizedDefault",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub localized_default: Option<String>,
    #[serde(
        rename = "LocalizedExternal",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub localized_external: Option<String>,
    #[serde(
        rename = "LocalizedForced",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub localized_forced: Option<String>,
    #[serde(
        rename = "LocalizedHearingImpaired",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub localized_hearing_impaired: Option<String>,
    #[serde(
        rename = "LocalizedUndefined",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub localized_undefined: Option<String>,
    #[serde(
        rename = "NalLengthSize",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub nal_length_size: Option<String>,
    #[doc = "Gets or sets the length of the packet."]
    #[serde(
        rename = "PacketLength",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub packet_length: Option<i32>,
    #[doc = "Gets or sets the filename."]
    #[serde(
        rename = "Path",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub path: Option<String>,
    #[doc = "Gets or sets the pixel format."]
    #[serde(
        rename = "PixelFormat",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pixel_format: Option<String>,
    #[doc = "Gets or sets the profile."]
    #[serde(
        rename = "Profile",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub profile: Option<String>,
    #[doc = "Gets or sets the real frame rate."]
    #[serde(
        rename = "RealFrameRate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub real_frame_rate: Option<f32>,
    #[doc = "Gets or sets the reference frames."]
    #[serde(
        rename = "RefFrames",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ref_frames: Option<i32>,
    #[doc = "Gets the framerate used as reference.\r\nPrefer AverageFrameRate, if that is null or an unrealistic value\r\nthen fallback to RealFrameRate."]
    #[serde(
        rename = "ReferenceFrameRate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reference_frame_rate: Option<f32>,
    #[doc = "Gets or sets the Rotation in degrees."]
    #[serde(
        rename = "Rotation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rotation: Option<i32>,
    #[doc = "Gets or sets the Dolby Vision rpu present flag."]
    #[serde(
        rename = "RpuPresentFlag",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rpu_present_flag: Option<i32>,
    #[doc = "Gets or sets the sample rate."]
    #[serde(
        rename = "SampleRate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sample_rate: Option<i32>,
    #[doc = "Gets or sets the score."]
    #[serde(
        rename = "Score",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub score: Option<i32>,
    #[doc = "Gets or sets a value indicating whether [supports external stream]."]
    #[serde(
        rename = "SupportsExternalStream",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_external_stream: Option<bool>,
    #[doc = "Gets or sets the time base."]
    #[serde(
        rename = "TimeBase",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_base: Option<String>,
    #[doc = "Gets or sets the title."]
    #[serde(
        rename = "Title",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub title: Option<String>,
    #[serde(
        rename = "Type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_: Option<MediaStreamType>,
    #[doc = "Gets the video dovi title."]
    #[serde(
        rename = "VideoDoViTitle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub video_do_vi_title: Option<String>,
    #[serde(
        rename = "VideoRange",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub video_range: Option<VideoRange>,
    #[serde(
        rename = "VideoRangeType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub video_range_type: Option<VideoRangeType>,
    #[doc = "Gets or sets the width."]
    #[serde(
        rename = "Width",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub width: Option<i32>,
}

impl Default for MediaStream {
    fn default() -> Self {
        Self {
            aspect_ratio: Default::default(),
            audio_spatial_format: Default::default(),
            average_frame_rate: Default::default(),
            bit_depth: Default::default(),
            bit_rate: Default::default(),
            bl_present_flag: Default::default(),
            channel_layout: Default::default(),
            channels: Default::default(),
            codec: Default::default(),
            codec_tag: Default::default(),
            codec_time_base: Default::default(),
            color_primaries: Default::default(),
            color_range: Default::default(),
            color_space: Default::default(),
            color_transfer: Default::default(),
            comment: Default::default(),
            delivery_method: Default::default(),
            delivery_url: Default::default(),
            display_title: Default::default(),
            dv_bl_signal_compatibility_id: Default::default(),
            dv_level: Default::default(),
            dv_profile: Default::default(),
            dv_version_major: Default::default(),
            dv_version_minor: Default::default(),
            el_present_flag: Default::default(),
            hdr10_plus_present_flag: Default::default(),
            height: Default::default(),
            index: Default::default(),
            is_anamorphic: Default::default(),
            is_avc: Default::default(),
            is_default: Default::default(),
            is_external: Default::default(),
            is_external_url: Default::default(),
            is_forced: Default::default(),
            is_hearing_impaired: Default::default(),
            is_interlaced: Default::default(),
            is_text_subtitle_stream: Default::default(),
            language: Default::default(),
            level: Default::default(),
            localized_default: Default::default(),
            localized_external: Default::default(),
            localized_forced: Default::default(),
            localized_hearing_impaired: Default::default(),
            localized_undefined: Default::default(),
            nal_length_size: Default::default(),
            packet_length: Default::default(),
            path: Default::default(),
            pixel_format: Default::default(),
            profile: Default::default(),
            real_frame_rate: Default::default(),
            ref_frames: Default::default(),
            reference_frame_rate: Default::default(),
            rotation: Default::default(),
            rpu_present_flag: Default::default(),
            sample_rate: Default::default(),
            score: Default::default(),
            supports_external_stream: Default::default(),
            time_base: Default::default(),
            title: Default::default(),
            type_: Default::default(),
            video_do_vi_title: Default::default(),
            video_range: Default::default(),
            video_range_type: Default::default(),
            width: Default::default(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MediaStreamProtocol {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "hls")]
    Hls,
}

impl std::fmt::Display for MediaStreamProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Http => f.write_str("http"),
            Self::Hls => f.write_str("hls"),
        }
    }
}

impl std::str::FromStr for MediaStreamProtocol {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "http" => Ok(Self::Http),
            "hls" => Ok(Self::Hls),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for MediaStreamProtocol {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for MediaStreamProtocol {
    type Error = super::error::ConversionError;
    fn try_from(
        value: &String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for MediaStreamProtocol {
    type Error = super::error::ConversionError;
    fn try_from(
        value: String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MediaStreamType {
    Audio,
    Video,
    Subtitle,
    EmbeddedImage,
    Data,
    Lyric,
}

impl std::fmt::Display for MediaStreamType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Audio => f.write_str("Audio"),
            Self::Video => f.write_str("Video"),
            Self::Subtitle => f.write_str("Subtitle"),
            Self::EmbeddedImage => f.write_str("EmbeddedImage"),
            Self::Data => f.write_str("Data"),
            Self::Lyric => f.write_str("Lyric"),
        }
    }
}

impl std::str::FromStr for MediaStreamType {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Audio" => Ok(Self::Audio),
            "Video" => Ok(Self::Video),
            "Subtitle" => Ok(Self::Subtitle),
            "EmbeddedImage" => Ok(Self::EmbeddedImage),
            "Data" => Ok(Self::Data),
            "Lyric" => Ok(Self::Lyric),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for MediaStreamType {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for MediaStreamType {
    type Error = super::error::ConversionError;
    fn try_from(
        value: &String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for MediaStreamType {
    type Error = super::error::ConversionError;
    fn try_from(
        value: String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "`MediaUrl`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct MediaUrl {
    #[serde(
        rename = "Name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,
    #[serde(
        rename = "Url",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub url: Option<String>,
}

impl Default for MediaUrl {
    fn default() -> Self {
        Self {
            name: Default::default(),
            url: Default::default(),
        }
    }
}

#[doc = "A class representing metadata editor information."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct MetadataEditorInfo {
    #[doc = "Gets or sets the content type."]
    #[serde(
        rename = "ContentType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub content_type: Option<CollectionType>,
    #[doc = "Gets or sets the content type options."]
    #[serde(
        rename = "ContentTypeOptions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub content_type_options: Vec<NameValuePair>,
    #[doc = "Gets or sets the countries."]
    #[serde(
        rename = "Countries",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub countries: Vec<CountryInfo>,
    #[doc = "Gets or sets the cultures."]
    #[serde(
        rename = "Cultures",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub cultures: Vec<CultureDto>,
    #[doc = "Gets or sets the external id infos."]
    #[serde(
        rename = "ExternalIdInfos",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub external_id_infos: Vec<ExternalIdInfo>,
    #[doc = "Gets or sets the parental rating options."]
    #[serde(
        rename = "ParentalRatingOptions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub parental_rating_options: Vec<ParentalRating>,
}

impl Default for MetadataEditorInfo {
    fn default() -> Self {
        Self {
            content_type: Default::default(),
            content_type_options: Default::default(),
            countries: Default::default(),
            cultures: Default::default(),
            external_id_infos: Default::default(),
            parental_rating_options: Default::default(),
        }
    }
}

#[doc = "`NameGuidPair`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct NameGuidPair {
    #[serde(
        rename = "Id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<uuid::Uuid>,
    #[serde(
        rename = "Name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,
}

impl Default for NameGuidPair {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
        }
    }
}

#[doc = "`NameValuePair`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct NameValuePair {
    #[doc = "Gets or sets the name."]
    #[serde(
        rename = "Name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,
    #[doc = "Gets or sets the value."]
    #[serde(
        rename = "Value",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub value: Option<String>,
}

impl Default for NameValuePair {
    fn default() -> Self {
        Self {
            name: Default::default(),
            value: Default::default(),
        }
    }
}

#[doc = "Class ThemeMediaResult."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ThemeMediaResult {
    #[doc = "Gets or sets the items."]
    #[serde(
        rename = "Items",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<BaseItemDto>,
    #[doc = "Gets or sets the owner id."]
    #[serde(
        rename = "OwnerId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub owner_id: Option<uuid::Uuid>,
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

impl Default for ThemeMediaResult {
    fn default() -> Self {
        Self {
            items: Default::default(),
            owner_id: Default::default(),
            start_index: Default::default(),
            total_record_count: Default::default(),
        }
    }
}

#[doc = "This is used by the api to get information about a item user data."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct UpdateUserItemDataDto {
    #[doc = "Gets or sets a value indicating whether this instance is favorite."]
    #[serde(
        rename = "IsFavorite",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_favorite: Option<bool>,
    #[doc = "Gets or sets the item identifier."]
    #[serde(
        rename = "ItemId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub item_id: Option<String>,
    #[doc = "Gets or sets the key."]
    #[serde(
        rename = "Key",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub key: Option<String>,
    #[doc = "Gets or sets the last played date."]
    #[serde(
        rename = "LastPlayedDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_played_date: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets a value indicating whether this MediaBrowser.Model.Dto.UpdateUserItemDataDto is likes."]
    #[serde(
        rename = "Likes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub likes: Option<bool>,
    #[doc = "Gets or sets the play count."]
    #[serde(
        rename = "PlayCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub play_count: Option<i32>,
    #[doc = "Gets or sets the playback position ticks."]
    #[serde(
        rename = "PlaybackPositionTicks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub playback_position_ticks: Option<i64>,
    #[doc = "Gets or sets a value indicating whether this MediaBrowser.Model.Dto.UserItemDataDto is played."]
    #[serde(
        rename = "Played",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub played: Option<bool>,
    #[doc = "Gets or sets the played percentage."]
    #[serde(
        rename = "PlayedPercentage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub played_percentage: Option<f64>,
    #[doc = "Gets or sets the rating."]
    #[serde(
        rename = "Rating",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rating: Option<f64>,
    #[doc = "Gets or sets the unplayed item count."]
    #[serde(
        rename = "UnplayedItemCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unplayed_item_count: Option<i32>,
}

impl Default for UpdateUserItemDataDto {
    fn default() -> Self {
        Self {
            is_favorite: Default::default(),
            item_id: Default::default(),
            key: Default::default(),
            last_played_date: Default::default(),
            likes: Default::default(),
            play_count: Default::default(),
            playback_position_ticks: Default::default(),
            played: Default::default(),
            played_percentage: Default::default(),
            rating: Default::default(),
            unplayed_item_count: Default::default(),
        }
    }
}

#[doc = "Class UserItemDataDto."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct UserItemDataDto {
    #[doc = "Gets or sets a value indicating whether this instance is favorite."]
    #[serde(
        rename = "IsFavorite",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_favorite: Option<bool>,
    #[doc = "Gets or sets the item identifier."]
    #[serde(
        rename = "ItemId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub item_id: Option<uuid::Uuid>,
    #[doc = "Gets or sets the key."]
    #[serde(
        rename = "Key",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub key: Option<String>,
    #[doc = "Gets or sets the last played date."]
    #[serde(
        rename = "LastPlayedDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_played_date: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets a value indicating whether this MediaBrowser.Model.Dto.UserItemDataDto is likes."]
    #[serde(
        rename = "Likes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub likes: Option<bool>,
    #[doc = "Gets or sets the play count."]
    #[serde(
        rename = "PlayCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub play_count: Option<i32>,
    #[doc = "Gets or sets the playback position ticks."]
    #[serde(
        rename = "PlaybackPositionTicks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub playback_position_ticks: Option<i64>,
    #[doc = "Gets or sets a value indicating whether this MediaBrowser.Model.Dto.UserItemDataDto is played."]
    #[serde(
        rename = "Played",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub played: Option<bool>,
    #[doc = "Gets or sets the played percentage."]
    #[serde(
        rename = "PlayedPercentage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub played_percentage: Option<f64>,
    #[doc = "Gets or sets the rating."]
    #[serde(
        rename = "Rating",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rating: Option<f64>,
    #[doc = "Gets or sets the unplayed item count."]
    #[serde(
        rename = "UnplayedItemCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unplayed_item_count: Option<i32>,
}

impl Default for UserItemDataDto {
    fn default() -> Self {
        Self {
            is_favorite: Default::default(),
            item_id: Default::default(),
            key: Default::default(),
            last_played_date: Default::default(),
            likes: Default::default(),
            play_count: Default::default(),
            playback_position_ticks: Default::default(),
            played: Default::default(),
            played_percentage: Default::default(),
            rating: Default::default(),
            unplayed_item_count: Default::default(),
        }
    }
}

