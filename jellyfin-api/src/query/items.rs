use crate::types;

/// What `/Items/Filters2` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetQueryFilters<'q> {
    /// Optional. If specified, results will be filtered based on item type. This allows multiple, comma delimited.
    pub include_item_types: Option<&'q Vec<types::BaseItemKind>>,
    /// Optional. Is item airing.
    pub is_airing: Option<bool>,
    /// Optional. Is item kids.
    pub is_kids: Option<bool>,
    /// Optional. Is item movie.
    pub is_movie: Option<bool>,
    /// Optional. Is item news.
    pub is_news: Option<bool>,
    /// Optional. Is item series.
    pub is_series: Option<bool>,
    /// Optional. Is item sports.
    pub is_sports: Option<bool>,
    /// Optional. Specify this to localize the search to a specific item or folder. Omit to use the root.
    pub parent_id: Option<&'q uuid::Uuid>,
    /// Optional. Search recursive.
    pub recursive: Option<bool>,
    /// Optional. User id.
    pub user_id: Option<&'q uuid::Uuid>,
}

/// What `/Items/{itemId}/Images/{imageType}` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetItemImage<'q> {
    /// Optional. Apply a background color for transparent images.
    pub background_color: Option<&'q str>,
    /// Optional. Blur image.
    pub blur: Option<i32>,
    /// Height of box to fill.
    pub fill_height: Option<i32>,
    /// Width of box to fill.
    pub fill_width: Option<i32>,
    /// Optional. Apply a foreground layer on top of the image.
    pub foreground_layer: Option<&'q str>,
    /// Optional. The MediaBrowser.Model.Drawing.ImageFormat of the returned image.
    pub format: Option<types::ImageFormat>,
    /// The fixed image height to return.
    pub height: Option<i32>,
    /// Image index.
    pub image_index: Option<i32>,
    /// The maximum image height to return.
    pub max_height: Option<i32>,
    /// The maximum image width to return.
    pub max_width: Option<i32>,
    /// Optional. Percent to render for the percent played overlay.
    pub percent_played: Option<f64>,
    /// Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases.
    pub quality: Option<i32>,
    /// Optional. Supply the cache tag from the item object to receive strong caching headers.
    pub tag: Option<&'q str>,
    /// Optional. Unplayed count overlay to render.
    pub unplayed_count: Option<i32>,
    /// The fixed image width to return.
    pub width: Option<i32>,
}

/// What `/Items/{itemId}/Images/{imageType}` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct HeadItemImage<'q> {
    /// Optional. Apply a background color for transparent images.
    pub background_color: Option<&'q str>,
    /// Optional. Blur image.
    pub blur: Option<i32>,
    /// Height of box to fill.
    pub fill_height: Option<i32>,
    /// Width of box to fill.
    pub fill_width: Option<i32>,
    /// Optional. Apply a foreground layer on top of the image.
    pub foreground_layer: Option<&'q str>,
    /// Optional. The MediaBrowser.Model.Drawing.ImageFormat of the returned image.
    pub format: Option<types::ImageFormat>,
    /// The fixed image height to return.
    pub height: Option<i32>,
    /// Image index.
    pub image_index: Option<i32>,
    /// The maximum image height to return.
    pub max_height: Option<i32>,
    /// The maximum image width to return.
    pub max_width: Option<i32>,
    /// Optional. Percent to render for the percent played overlay.
    pub percent_played: Option<f64>,
    /// Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases.
    pub quality: Option<i32>,
    /// Optional. Supply the cache tag from the item object to receive strong caching headers.
    pub tag: Option<&'q str>,
    /// Optional. Unplayed count overlay to render.
    pub unplayed_count: Option<i32>,
    /// The fixed image width to return.
    pub width: Option<i32>,
}

/// What `/Items/{itemId}/Images/{imageType}/{imageIndex}` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetItemImageByIndex<'q> {
    /// Optional. Apply a background color for transparent images.
    pub background_color: Option<&'q str>,
    /// Optional. Blur image.
    pub blur: Option<i32>,
    /// Height of box to fill.
    pub fill_height: Option<i32>,
    /// Width of box to fill.
    pub fill_width: Option<i32>,
    /// Optional. Apply a foreground layer on top of the image.
    pub foreground_layer: Option<&'q str>,
    /// Optional. The MediaBrowser.Model.Drawing.ImageFormat of the returned image.
    pub format: Option<types::ImageFormat>,
    /// The fixed image height to return.
    pub height: Option<i32>,
    /// The maximum image height to return.
    pub max_height: Option<i32>,
    /// The maximum image width to return.
    pub max_width: Option<i32>,
    /// Optional. Percent to render for the percent played overlay.
    pub percent_played: Option<f64>,
    /// Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases.
    pub quality: Option<i32>,
    /// Optional. Supply the cache tag from the item object to receive strong caching headers.
    pub tag: Option<&'q str>,
    /// Optional. Unplayed count overlay to render.
    pub unplayed_count: Option<i32>,
    /// The fixed image width to return.
    pub width: Option<i32>,
}

/// What `/Items/{itemId}/Images/{imageType}/{imageIndex}` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct HeadItemImageByIndex<'q> {
    /// Optional. Apply a background color for transparent images.
    pub background_color: Option<&'q str>,
    /// Optional. Blur image.
    pub blur: Option<i32>,
    /// Height of box to fill.
    pub fill_height: Option<i32>,
    /// Width of box to fill.
    pub fill_width: Option<i32>,
    /// Optional. Apply a foreground layer on top of the image.
    pub foreground_layer: Option<&'q str>,
    /// Optional. The MediaBrowser.Model.Drawing.ImageFormat of the returned image.
    pub format: Option<types::ImageFormat>,
    /// The fixed image height to return.
    pub height: Option<i32>,
    /// The maximum image height to return.
    pub max_height: Option<i32>,
    /// The maximum image width to return.
    pub max_width: Option<i32>,
    /// Optional. Percent to render for the percent played overlay.
    pub percent_played: Option<f64>,
    /// Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases.
    pub quality: Option<i32>,
    /// Optional. Supply the cache tag from the item object to receive strong caching headers.
    pub tag: Option<&'q str>,
    /// Optional. Unplayed count overlay to render.
    pub unplayed_count: Option<i32>,
    /// The fixed image width to return.
    pub width: Option<i32>,
}

/// The whole of `/Items/{itemId}/Images/{imageType}/{imageIndex}/{tag}/{format}/{maxWidth}/{maxHeight}/{percentPlayed}/{unplayedCount}`.
#[derive(Debug, Clone)]
pub struct GetItemImageSized<'q> {
    pub item_id: &'q uuid::Uuid,
    pub image_type: types::ImageType,
    pub image_index: i32,
    pub tag: &'q str,
    pub format: types::ImageFormat,
    pub max_width: i32,
    pub max_height: i32,
    pub percent_played: f64,
    pub unplayed_count: i32,
    pub background_color: Option<&'q str>,
    pub blur: Option<i32>,
    pub fill_height: Option<i32>,
    pub fill_width: Option<i32>,
    pub foreground_layer: Option<&'q str>,
    pub height: Option<i32>,
    pub quality: Option<i32>,
    pub width: Option<i32>,
}

/// The whole of `/Items/{itemId}/Images/{imageType}/{imageIndex}/{tag}/{format}/{maxWidth}/{maxHeight}/{percentPlayed}/{unplayedCount}`.
#[derive(Debug, Clone)]
pub struct HeadItemImageSized<'q> {
    pub item_id: &'q uuid::Uuid,
    pub image_type: types::ImageType,
    pub image_index: i32,
    pub tag: &'q str,
    pub format: types::ImageFormat,
    pub max_width: i32,
    pub max_height: i32,
    pub percent_played: f64,
    pub unplayed_count: i32,
    pub background_color: Option<&'q str>,
    pub blur: Option<i32>,
    pub fill_height: Option<i32>,
    pub fill_width: Option<i32>,
    pub foreground_layer: Option<&'q str>,
    pub height: Option<i32>,
    pub quality: Option<i32>,
    pub width: Option<i32>,
}

/// What `/Items/{itemId}/InstantMix` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetInstantMixFromItem<'q> {
    /// Optional. The image types to include in the output.
    pub enable_image_types: Option<&'q Vec<types::ImageType>>,
    /// Optional. Include image information in output.
    pub enable_images: Option<bool>,
    /// Optional. Include user data.
    pub enable_user_data: Option<bool>,
    /// Optional. Specify additional fields of information to return in the output.
    pub fields: Option<&'q Vec<types::ItemFields>>,
    /// Optional. The max number of images to return, per image type.
    pub image_type_limit: Option<i32>,
    /// Optional. The maximum number of records to return.
    pub limit: Option<i32>,
    /// Optional. Filter by user id, and attach user data.
    pub user_id: Option<&'q uuid::Uuid>,
}

/// What `/Items` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetItems<'q> {
    /// Optional. Return items that are siblings of a supplied item.
    pub adjacent_to: Option<&'q uuid::Uuid>,
    /// Optional. If specified, results will be filtered to include only those containing the specified album artist id.
    pub album_artist_ids: Option<&'q Vec<uuid::Uuid>>,
    /// Optional. If specified, results will be filtered based on album id. This allows multiple, pipe delimited.
    pub album_ids: Option<&'q Vec<uuid::Uuid>>,
    /// Optional. If specified, results will be filtered based on album. This allows multiple, pipe delimited.
    pub albums: Option<&'q Vec<String>>,
    /// Optional. If specified, results will be filtered to include only those containing the specified artist id.
    pub artist_ids: Option<&'q Vec<uuid::Uuid>>,
    /// Optional. If specified, results will be filtered based on artists. This allows multiple, pipe delimited.
    pub artists: Option<&'q Vec<String>>,
    /// Whether or not to hide items behind their boxsets.
    pub collapse_box_set_items: Option<bool>,
    /// Optional. If specified, results will be filtered to include only those containing the specified contributing artist id.
    pub contributing_artist_ids: Option<&'q Vec<uuid::Uuid>>,
    /// Optional. The image types to include in the output.
    pub enable_image_types: Option<&'q Vec<types::ImageType>>,
    /// Optional, include image information in output.
    pub enable_images: Option<bool>,
    /// Optional. Enable the total record count.
    pub enable_total_record_count: Option<bool>,
    /// Optional, include user data.
    pub enable_user_data: Option<bool>,
    /// Optional. If specified, results will be filtered based on artist id. This allows multiple, pipe delimited.
    pub exclude_artist_ids: Option<&'q Vec<uuid::Uuid>>,
    /// Optional. If specified, results will be filtered by excluding item ids. This allows multiple, comma delimited.
    pub exclude_item_ids: Option<&'q Vec<uuid::Uuid>>,
    /// Optional. If specified, results will be filtered based on item type. This allows multiple, comma delimited.
    pub exclude_item_types: Option<&'q Vec<types::BaseItemKind>>,
    /// Optional. If specified, results will be filtered based on the LocationType. This allows multiple, comma delimited.
    pub exclude_location_types: Option<&'q Vec<types::LocationType>>,
    /// Optional. Specify additional fields of information to return in the output. This allows multiple, comma delimited. Options: Budget, Chapters, DateCreated, Genres, HomePageUrl, IndexOptions, MediaStreams, Overview, ParentId, Path, People, ProviderIds, PrimaryImageAspectRatio, Revenue, SortName, Studios, Taglines.
    pub fields: Option<&'q Vec<types::ItemFields>>,
    /// Optional. Specify additional filters to apply. This allows multiple, comma delimited. Options: IsFolder, IsNotFolder, IsUnplayed, IsPlayed, IsFavorite, IsResumable, Likes, Dislikes.
    pub filters: Option<&'q Vec<types::ItemFilter>>,
    /// Optional. If specified, results will be filtered based on genre id. This allows multiple, pipe delimited.
    pub genre_ids: Option<&'q Vec<uuid::Uuid>>,
    /// Optional. If specified, results will be filtered based on genre. This allows multiple, pipe delimited.
    pub genres: Option<&'q Vec<String>>,
    /// Optional filter by items that have an IMDb id or not.
    pub has_imdb_id: Option<bool>,
    /// Optional filter by items that have official ratings.
    pub has_official_rating: Option<bool>,
    /// Optional filter by items that have an overview or not.
    pub has_overview: Option<bool>,
    /// Optional filter by items that have or do not have a parental rating.
    pub has_parental_rating: Option<bool>,
    /// Optional filter by items with special features.
    pub has_special_feature: Option<bool>,
    /// Optional filter by items with subtitles.
    pub has_subtitles: Option<bool>,
    /// Optional filter by items with theme songs.
    pub has_theme_song: Option<bool>,
    /// Optional filter by items with theme videos.
    pub has_theme_video: Option<bool>,
    /// Optional filter by items that have a TMDb id or not.
    pub has_tmdb_id: Option<bool>,
    /// Optional filter by items with trailers.
    pub has_trailer: Option<bool>,
    /// Optional filter by items that have a TVDb id or not.
    pub has_tvdb_id: Option<bool>,
    /// Optional. If specific items are needed, specify a list of item id's to retrieve. This allows multiple, comma delimited.
    pub ids: Option<&'q Vec<uuid::Uuid>>,
    /// Optional, the max number of images to return, per image type.
    pub image_type_limit: Option<i32>,
    /// Optional. If specified, results will be filtered based on those containing image types. This allows multiple, comma delimited.
    pub image_types: Option<&'q Vec<types::ImageType>>,
    /// Optional. If specified, results will be filtered based on the item type. This allows multiple, comma delimited.
    pub include_item_types: Option<&'q Vec<types::BaseItemKind>>,
    /// Optional filter by index number.
    pub index_number: Option<i32>,
    /// Optional filter by items that are 3D, or not.
    pub is_3d: Option<bool>,
    /// Optional filter by items that are 4K or not.
    pub is_4k: Option<bool>,
    /// Optional filter by items that are marked as favorite, or not.
    pub is_favorite: Option<bool>,
    /// Optional filter by items that are HD or not.
    pub is_hd: Option<bool>,
    /// Optional filter for live tv kids.
    pub is_kids: Option<bool>,
    /// Optional filter by items that are locked.
    pub is_locked: Option<bool>,
    /// Optional filter by items that are missing episodes or not.
    pub is_missing: Option<bool>,
    /// Optional filter for live tv movies.
    pub is_movie: Option<bool>,
    /// Optional filter for live tv news.
    pub is_news: Option<bool>,
    /// Optional filter by items that are placeholders.
    pub is_place_holder: Option<bool>,
    /// Optional filter by items that are played, or not.
    pub is_played: Option<bool>,
    /// Optional filter for live tv series.
    pub is_series: Option<bool>,
    /// Optional filter for live tv sports.
    pub is_sports: Option<bool>,
    /// Optional filter by items that are unaired episodes or not.
    pub is_unaired: Option<bool>,
    /// Optional. The maximum number of records to return.
    pub limit: Option<i32>,
    /// Optional. If specified, results will be filtered based on LocationType. This allows multiple, comma delimited.
    pub location_types: Option<&'q Vec<types::LocationType>>,
    /// Optional. Filter by the maximum height of the item.
    pub max_height: Option<i32>,
    /// Optional filter by maximum official rating (PG, PG-13, TV-MA, etc).
    pub max_official_rating: Option<&'q str>,
    /// Optional. The maximum premiere date. Format = ISO.
    pub max_premiere_date: Option<types::Timestamp>,
    /// Optional. Filter by the maximum width of the item.
    pub max_width: Option<i32>,
    /// Optional filter by MediaType. Allows multiple, comma delimited.
    pub media_types: Option<&'q Vec<types::MediaType>>,
    /// Optional filter by minimum community rating.
    pub min_community_rating: Option<f64>,
    /// Optional filter by minimum critic rating.
    pub min_critic_rating: Option<f64>,
    /// Optional. The minimum last saved date. Format = ISO.
    pub min_date_last_saved: Option<types::Timestamp>,
    /// Optional. The minimum last saved date for the current user. Format = ISO.
    pub min_date_last_saved_for_user: Option<types::Timestamp>,
    /// Optional. Filter by the minimum height of the item.
    pub min_height: Option<i32>,
    /// Optional filter by minimum official rating (PG, PG-13, TV-MA, etc).
    pub min_official_rating: Option<&'q str>,
    /// Optional. The minimum premiere date. Format = ISO.
    pub min_premiere_date: Option<types::Timestamp>,
    /// Optional. Filter by the minimum width of the item.
    pub min_width: Option<i32>,
    /// Optional filter by items whose name is equally or lesser than a given input string.
    pub name_less_than: Option<&'q str>,
    /// Optional filter by items whose name is sorted equally than a given input string.
    pub name_starts_with: Option<&'q str>,
    /// Optional filter by items whose name is sorted equally or greater than a given input string.
    pub name_starts_with_or_greater: Option<&'q str>,
    /// Optional. If specified, results will be filtered based on OfficialRating. This allows multiple, pipe delimited.
    pub official_ratings: Option<&'q Vec<String>>,
    /// Specify this to localize the search to a specific item or folder. Omit to use the root.
    pub parent_id: Option<&'q uuid::Uuid>,
    /// Optional filter by parent index number.
    pub parent_index_number: Option<i32>,
    /// Optional. If specified, results will be filtered to include only those containing the specified person.
    pub person: Option<&'q str>,
    /// Optional. If specified, results will be filtered to include only those containing the specified person id.
    pub person_ids: Option<&'q Vec<uuid::Uuid>>,
    /// Optional. If specified, along with Person, results will be filtered to include only those containing the specified person and PersonType. Allows multiple, comma-delimited.
    pub person_types: Option<&'q Vec<String>>,
    /// When searching within folders, this determines whether or not the search will be recursive. true/false.
    pub recursive: Option<bool>,
    /// Optional. Filter based on a search term.
    pub search_term: Option<&'q str>,
    /// Optional filter by Series Status. Allows multiple, comma delimited.
    pub series_status: Option<&'q Vec<types::SeriesStatus>>,
    /// Optional. Specify one or more sort orders, comma delimited. Options: Album, AlbumArtist, Artist, Budget, CommunityRating, CriticRating, DateCreated, DatePlayed, PlayCount, PremiereDate, ProductionYear, SortName, Random, Revenue, Runtime.
    pub sort_by: Option<&'q Vec<types::ItemSortBy>>,
    /// Sort Order - Ascending, Descending.
    pub sort_order: Option<&'q Vec<types::SortOrder>>,
    /// Optional. The record index to start at. All items with a lower index will be dropped from the results.
    pub start_index: Option<i32>,
    /// Optional. If specified, results will be filtered based on studio id. This allows multiple, pipe delimited.
    pub studio_ids: Option<&'q Vec<uuid::Uuid>>,
    /// Optional. If specified, results will be filtered based on studio. This allows multiple, pipe delimited.
    pub studios: Option<&'q Vec<String>>,
    /// Optional. If specified, results will be filtered based on tag. This allows multiple, pipe delimited.
    pub tags: Option<&'q Vec<String>>,
    /// The user id supplied as query parameter; this is required when not using an API key.
    pub user_id: Option<&'q uuid::Uuid>,
    /// Optional filter by VideoType (videofile, dvd, bluray, iso). Allows multiple, comma delimited.
    pub video_types: Option<&'q Vec<types::VideoType>>,
    /// Optional. If specified, results will be filtered based on production year. This allows multiple, comma delimited.
    pub years: Option<&'q Vec<i32>>,
}

/// What `/Items/{itemId}/PlaybackInfo` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetPostedPlaybackInfo<'q> {
    /// Whether to allow to copy the audio stream. Default: true.
    pub allow_audio_stream_copy: Option<bool>,
    /// Whether to allow to copy the video stream. Default: true.
    pub allow_video_stream_copy: Option<bool>,
    /// The audio stream index.
    pub audio_stream_index: Option<i32>,
    /// Whether to auto open the livestream.
    pub auto_open_live_stream: Option<bool>,
    /// Whether to enable direct play. Default: true.
    pub enable_direct_play: Option<bool>,
    /// Whether to enable direct stream. Default: true.
    pub enable_direct_stream: Option<bool>,
    /// Whether to enable transcoding. Default: true.
    pub enable_transcoding: Option<bool>,
    /// The livestream id.
    pub live_stream_id: Option<&'q str>,
    /// The maximum number of audio channels.
    pub max_audio_channels: Option<i32>,
    /// The maximum streaming bitrate.
    pub max_streaming_bitrate: Option<i32>,
    /// The media source id.
    pub media_source_id: Option<&'q str>,
    /// The start time in ticks.
    pub start_time_ticks: Option<i64>,
    /// The subtitle stream index.
    pub subtitle_stream_index: Option<i32>,
    /// The user id.
    pub user_id: Option<&'q uuid::Uuid>,
}

/// What `/Search/Hints` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetSearchHints<'q> {
    /// If specified, results with these item types are filtered out. This allows multiple, comma delimited.
    pub exclude_item_types: Option<&'q Vec<types::BaseItemKind>>,
    /// Optional filter whether to include artists.
    pub include_artists: Option<bool>,
    /// Optional filter whether to include genres.
    pub include_genres: Option<bool>,
    /// If specified, only results with the specified item types are returned. This allows multiple, comma delimited.
    pub include_item_types: Option<&'q Vec<types::BaseItemKind>>,
    /// Optional filter whether to include media.
    pub include_media: Option<bool>,
    /// Optional filter whether to include people.
    pub include_people: Option<bool>,
    /// Optional filter whether to include studios.
    pub include_studios: Option<bool>,
    /// Optional filter for kids.
    pub is_kids: Option<bool>,
    /// Optional filter for movies.
    pub is_movie: Option<bool>,
    /// Optional filter for news.
    pub is_news: Option<bool>,
    /// Optional filter for series.
    pub is_series: Option<bool>,
    /// Optional filter for sports.
    pub is_sports: Option<bool>,
    /// Optional. The maximum number of records to return.
    pub limit: Option<i32>,
    /// If specified, only results with the specified media types are returned. This allows multiple, comma delimited.
    pub media_types: Option<&'q Vec<types::MediaType>>,
    /// If specified, only children of the parent are returned.
    pub parent_id: Option<&'q uuid::Uuid>,
    /// Optional. The record index to start at. All items with a lower index will be dropped from the results.
    pub start_index: Option<i32>,
    /// Optional. Supply a user id to search within a user's library or omit to search all.
    pub user_id: Option<&'q uuid::Uuid>,
}

/// What `/Trailers` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetTrailers<'q> {
    /// Optional. Return items that are siblings of a supplied item.
    pub adjacent_to: Option<&'q uuid::Uuid>,
    /// Optional. If specified, results will be filtered to include only those containing the specified album artist id.
    pub album_artist_ids: Option<&'q Vec<uuid::Uuid>>,
    /// Optional. If specified, results will be filtered based on album id. This allows multiple, pipe delimited.
    pub album_ids: Option<&'q Vec<uuid::Uuid>>,
    /// Optional. If specified, results will be filtered based on album. This allows multiple, pipe delimited.
    pub albums: Option<&'q Vec<String>>,
    /// Optional. If specified, results will be filtered to include only those containing the specified artist id.
    pub artist_ids: Option<&'q Vec<uuid::Uuid>>,
    /// Optional. If specified, results will be filtered based on artists. This allows multiple, pipe delimited.
    pub artists: Option<&'q Vec<String>>,
    /// Whether or not to hide items behind their boxsets.
    pub collapse_box_set_items: Option<bool>,
    /// Optional. If specified, results will be filtered to include only those containing the specified contributing artist id.
    pub contributing_artist_ids: Option<&'q Vec<uuid::Uuid>>,
    /// Optional. The image types to include in the output.
    pub enable_image_types: Option<&'q Vec<types::ImageType>>,
    /// Optional, include image information in output.
    pub enable_images: Option<bool>,
    /// Optional. Enable the total record count.
    pub enable_total_record_count: Option<bool>,
    /// Optional, include user data.
    pub enable_user_data: Option<bool>,
    /// Optional. If specified, results will be filtered based on artist id. This allows multiple, pipe delimited.
    pub exclude_artist_ids: Option<&'q Vec<uuid::Uuid>>,
    /// Optional. If specified, results will be filtered by excluding item ids. This allows multiple, comma delimited.
    pub exclude_item_ids: Option<&'q Vec<uuid::Uuid>>,
    /// Optional. If specified, results will be filtered based on item type. This allows multiple, comma delimited.
    pub exclude_item_types: Option<&'q Vec<types::BaseItemKind>>,
    /// Optional. If specified, results will be filtered based on the LocationType. This allows multiple, comma delimited.
    pub exclude_location_types: Option<&'q Vec<types::LocationType>>,
    /// Optional. Specify additional fields of information to return in the output. This allows multiple, comma delimited. Options: Budget, Chapters, DateCreated, Genres, HomePageUrl, IndexOptions, MediaStreams, Overview, ParentId, Path, People, ProviderIds, PrimaryImageAspectRatio, Revenue, SortName, Studios, Taglines.
    pub fields: Option<&'q Vec<types::ItemFields>>,
    /// Optional. Specify additional filters to apply. This allows multiple, comma delimited. Options: IsFolder, IsNotFolder, IsUnplayed, IsPlayed, IsFavorite, IsResumable, Likes, Dislikes.
    pub filters: Option<&'q Vec<types::ItemFilter>>,
    /// Optional. If specified, results will be filtered based on genre id. This allows multiple, pipe delimited.
    pub genre_ids: Option<&'q Vec<uuid::Uuid>>,
    /// Optional. If specified, results will be filtered based on genre. This allows multiple, pipe delimited.
    pub genres: Option<&'q Vec<String>>,
    /// Optional filter by items that have an IMDb id or not.
    pub has_imdb_id: Option<bool>,
    /// Optional filter by items that have official ratings.
    pub has_official_rating: Option<bool>,
    /// Optional filter by items that have an overview or not.
    pub has_overview: Option<bool>,
    /// Optional filter by items that have or do not have a parental rating.
    pub has_parental_rating: Option<bool>,
    /// Optional filter by items with special features.
    pub has_special_feature: Option<bool>,
    /// Optional filter by items with subtitles.
    pub has_subtitles: Option<bool>,
    /// Optional filter by items with theme songs.
    pub has_theme_song: Option<bool>,
    /// Optional filter by items with theme videos.
    pub has_theme_video: Option<bool>,
    /// Optional filter by items that have a TMDb id or not.
    pub has_tmdb_id: Option<bool>,
    /// Optional filter by items with trailers.
    pub has_trailer: Option<bool>,
    /// Optional filter by items that have a TVDb id or not.
    pub has_tvdb_id: Option<bool>,
    /// Optional. If specific items are needed, specify a list of item id's to retrieve. This allows multiple, comma delimited.
    pub ids: Option<&'q Vec<uuid::Uuid>>,
    /// Optional, the max number of images to return, per image type.
    pub image_type_limit: Option<i32>,
    /// Optional. If specified, results will be filtered based on those containing image types. This allows multiple, comma delimited.
    pub image_types: Option<&'q Vec<types::ImageType>>,
    /// Optional filter by items that are 3D, or not.
    pub is_3d: Option<bool>,
    /// Optional filter by items that are 4K or not.
    pub is_4k: Option<bool>,
    /// Optional filter by items that are marked as favorite, or not.
    pub is_favorite: Option<bool>,
    /// Optional filter by items that are HD or not.
    pub is_hd: Option<bool>,
    /// Optional filter for live tv kids.
    pub is_kids: Option<bool>,
    /// Optional filter by items that are locked.
    pub is_locked: Option<bool>,
    /// Optional filter by items that are missing episodes or not.
    pub is_missing: Option<bool>,
    /// Optional filter for live tv movies.
    pub is_movie: Option<bool>,
    /// Optional filter for live tv news.
    pub is_news: Option<bool>,
    /// Optional filter by items that are placeholders.
    pub is_place_holder: Option<bool>,
    /// Optional filter by items that are played, or not.
    pub is_played: Option<bool>,
    /// Optional filter for live tv series.
    pub is_series: Option<bool>,
    /// Optional filter for live tv sports.
    pub is_sports: Option<bool>,
    /// Optional filter by items that are unaired episodes or not.
    pub is_unaired: Option<bool>,
    /// Optional. The maximum number of records to return.
    pub limit: Option<i32>,
    /// Optional. If specified, results will be filtered based on LocationType. This allows multiple, comma delimited.
    pub location_types: Option<&'q Vec<types::LocationType>>,
    /// Optional. Filter by the maximum height of the item.
    pub max_height: Option<i32>,
    /// Optional filter by maximum official rating (PG, PG-13, TV-MA, etc).
    pub max_official_rating: Option<&'q str>,
    /// Optional. The maximum premiere date. Format = ISO.
    pub max_premiere_date: Option<types::Timestamp>,
    /// Optional. Filter by the maximum width of the item.
    pub max_width: Option<i32>,
    /// Optional filter by MediaType. Allows multiple, comma delimited.
    pub media_types: Option<&'q Vec<types::MediaType>>,
    /// Optional filter by minimum community rating.
    pub min_community_rating: Option<f64>,
    /// Optional filter by minimum critic rating.
    pub min_critic_rating: Option<f64>,
    /// Optional. The minimum last saved date. Format = ISO.
    pub min_date_last_saved: Option<types::Timestamp>,
    /// Optional. The minimum last saved date for the current user. Format = ISO.
    pub min_date_last_saved_for_user: Option<types::Timestamp>,
    /// Optional. Filter by the minimum height of the item.
    pub min_height: Option<i32>,
    /// Optional filter by minimum official rating (PG, PG-13, TV-MA, etc).
    pub min_official_rating: Option<&'q str>,
    /// Optional. The minimum premiere date. Format = ISO.
    pub min_premiere_date: Option<types::Timestamp>,
    /// Optional. Filter by the minimum width of the item.
    pub min_width: Option<i32>,
    /// Optional filter by items whose name is equally or lesser than a given input string.
    pub name_less_than: Option<&'q str>,
    /// Optional filter by items whose name is sorted equally than a given input string.
    pub name_starts_with: Option<&'q str>,
    /// Optional filter by items whose name is sorted equally or greater than a given input string.
    pub name_starts_with_or_greater: Option<&'q str>,
    /// Optional. If specified, results will be filtered based on OfficialRating. This allows multiple, pipe delimited.
    pub official_ratings: Option<&'q Vec<String>>,
    /// Specify this to localize the search to a specific item or folder. Omit to use the root.
    pub parent_id: Option<&'q uuid::Uuid>,
    /// Optional filter by parent index number.
    pub parent_index_number: Option<i32>,
    /// Optional. If specified, results will be filtered to include only those containing the specified person.
    pub person: Option<&'q str>,
    /// Optional. If specified, results will be filtered to include only those containing the specified person id.
    pub person_ids: Option<&'q Vec<uuid::Uuid>>,
    /// Optional. If specified, along with Person, results will be filtered to include only those containing the specified person and PersonType. Allows multiple, comma-delimited.
    pub person_types: Option<&'q Vec<String>>,
    /// When searching within folders, this determines whether or not the search will be recursive. true/false.
    pub recursive: Option<bool>,
    /// Optional. Filter based on a search term.
    pub search_term: Option<&'q str>,
    /// Optional filter by Series Status. Allows multiple, comma delimited.
    pub series_status: Option<&'q Vec<types::SeriesStatus>>,
    /// Optional. Specify one or more sort orders, comma delimited. Options: Album, AlbumArtist, Artist, Budget, CommunityRating, CriticRating, DateCreated, DatePlayed, PlayCount, PremiereDate, ProductionYear, SortName, Random, Revenue, Runtime.
    pub sort_by: Option<&'q Vec<types::ItemSortBy>>,
    /// Sort Order - Ascending, Descending.
    pub sort_order: Option<&'q Vec<types::SortOrder>>,
    /// Optional. The record index to start at. All items with a lower index will be dropped from the results.
    pub start_index: Option<i32>,
    /// Optional. If specified, results will be filtered based on studio id. This allows multiple, pipe delimited.
    pub studio_ids: Option<&'q Vec<uuid::Uuid>>,
    /// Optional. If specified, results will be filtered based on studio. This allows multiple, pipe delimited.
    pub studios: Option<&'q Vec<String>>,
    /// Optional. If specified, results will be filtered based on tag. This allows multiple, pipe delimited.
    pub tags: Option<&'q Vec<String>>,
    /// The user id supplied as query parameter; this is required when not using an API key.
    pub user_id: Option<&'q uuid::Uuid>,
    /// Optional filter by VideoType (videofile, dvd, bluray, iso). Allows multiple, comma delimited.
    pub video_types: Option<&'q Vec<types::VideoType>>,
    /// Optional. If specified, results will be filtered based on production year. This allows multiple, comma delimited.
    pub years: Option<&'q Vec<i32>>,
}

/// What `/Items/Latest` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetLatestMedia<'q> {
    /// Optional. The image types to include in the output.
    pub enable_image_types: Option<&'q Vec<types::ImageType>>,
    /// Optional. include image information in output.
    pub enable_images: Option<bool>,
    /// Optional. include user data.
    pub enable_user_data: Option<bool>,
    /// Optional. Specify additional fields of information to return in the output.
    pub fields: Option<&'q Vec<types::ItemFields>>,
    /// Whether or not to group items into a parent container.
    pub group_items: Option<bool>,
    /// Optional. the max number of images to return, per image type.
    pub image_type_limit: Option<i32>,
    /// Optional. If specified, results will be filtered based on item type. This allows multiple, comma delimited.
    pub include_item_types: Option<&'q Vec<types::BaseItemKind>>,
    /// Filter by items that are played, or not.
    pub is_played: Option<bool>,
    /// Return item limit.
    pub limit: Option<i32>,
    /// Specify this to localize the search to a specific item or folder. Omit to use the root.
    pub parent_id: Option<&'q uuid::Uuid>,
    /// User id.
    pub user_id: Option<&'q uuid::Uuid>,
}

/// What `/Years` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetYears<'q> {
    /// Optional. The image types to include in the output.
    pub enable_image_types: Option<&'q Vec<types::ImageType>>,
    /// Optional. Include image information in output.
    pub enable_images: Option<bool>,
    /// Optional. Include user data.
    pub enable_user_data: Option<bool>,
    /// Optional. If specified, results will be excluded based on item type. This allows multiple, comma delimited.
    pub exclude_item_types: Option<&'q Vec<types::BaseItemKind>>,
    /// Optional. Specify additional fields of information to return in the output.
    pub fields: Option<&'q Vec<types::ItemFields>>,
    /// Optional. The max number of images to return, per image type.
    pub image_type_limit: Option<i32>,
    /// Optional. If specified, results will be included based on item type. This allows multiple, comma delimited.
    pub include_item_types: Option<&'q Vec<types::BaseItemKind>>,
    /// Optional. The maximum number of records to return.
    pub limit: Option<i32>,
    /// Optional. Filter by MediaType. Allows multiple, comma delimited.
    pub media_types: Option<&'q Vec<types::MediaType>>,
    /// Specify this to localize the search to a specific item or folder. Omit to use the root.
    pub parent_id: Option<&'q uuid::Uuid>,
    /// Search recursively.
    pub recursive: Option<bool>,
    /// Optional. Specify one or more sort orders, comma delimited. Options: Album, AlbumArtist, Artist, Budget, CommunityRating, CriticRating, DateCreated, DatePlayed, PlayCount, PremiereDate, ProductionYear, SortName, Random, Revenue, Runtime.
    pub sort_by: Option<&'q Vec<types::ItemSortBy>>,
    /// Sort Order - Ascending,Descending.
    pub sort_order: Option<&'q Vec<types::SortOrder>>,
    /// Skips over a given number of items within the results. Use for paging.
    pub start_index: Option<i32>,
    /// User Id.
    pub user_id: Option<&'q uuid::Uuid>,
}
