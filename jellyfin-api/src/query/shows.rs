use crate::types;

/// What `/Shows/{seriesId}/Episodes` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetEpisodes<'q> {
    /// Optional. Return items that are siblings of a supplied item.
    pub adjacent_to: Option<&'q uuid::Uuid>,
    /// Optional. The image types to include in the output.
    pub enable_image_types: Option<&'q Vec<types::ImageType>>,
    /// Optional, include image information in output.
    pub enable_images: Option<bool>,
    /// Optional. Include user data.
    pub enable_user_data: Option<bool>,
    /// Optional. Specify additional fields of information to return in the output. This allows multiple, comma delimited. Options: Budget, Chapters, DateCreated, Genres, HomePageUrl, IndexOptions, MediaStreams, Overview, ParentId, Path, People, ProviderIds, PrimaryImageAspectRatio, Revenue, SortName, Studios, Taglines, TrailerUrls.
    pub fields: Option<&'q Vec<types::ItemFields>>,
    /// Optional, the max number of images to return, per image type.
    pub image_type_limit: Option<i32>,
    /// Optional. Filter by items that are missing episodes or not.
    pub is_missing: Option<bool>,
    /// Optional. The maximum number of records to return.
    pub limit: Option<i32>,
    /// Optional filter by season number.
    pub season: Option<i32>,
    /// Optional. Filter by season id.
    pub season_id: Option<&'q uuid::Uuid>,
    /// Optional. Specify one or more sort orders, comma delimited. Options: Album, AlbumArtist, Artist, Budget, CommunityRating, CriticRating, DateCreated, DatePlayed, PlayCount, PremiereDate, ProductionYear, SortName, Random, Revenue, Runtime.
    pub sort_by: Option<types::ItemSortBy>,
    /// Optional. The record index to start at. All items with a lower index will be dropped from the results.
    pub start_index: Option<i32>,
    /// Optional. Skip through the list until a given item is found.
    pub start_item_id: Option<&'q uuid::Uuid>,
    /// The user id.
    pub user_id: Option<&'q uuid::Uuid>,
}

/// What `/Shows/{seriesId}/Seasons` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetSeasons<'q> {
    /// Optional. Return items that are siblings of a supplied item.
    pub adjacent_to: Option<&'q uuid::Uuid>,
    /// Optional. The image types to include in the output.
    pub enable_image_types: Option<&'q Vec<types::ImageType>>,
    /// Optional. Include image information in output.
    pub enable_images: Option<bool>,
    /// Optional. Include user data.
    pub enable_user_data: Option<bool>,
    /// Optional. Specify additional fields of information to return in the output. This allows multiple, comma delimited. Options: Budget, Chapters, DateCreated, Genres, HomePageUrl, IndexOptions, MediaStreams, Overview, ParentId, Path, People, ProviderIds, PrimaryImageAspectRatio, Revenue, SortName, Studios, Taglines, TrailerUrls.
    pub fields: Option<&'q Vec<types::ItemFields>>,
    /// Optional. The max number of images to return, per image type.
    pub image_type_limit: Option<i32>,
    /// Optional. Filter by items that are missing episodes or not.
    pub is_missing: Option<bool>,
    /// Optional. Filter by special season.
    pub is_special_season: Option<bool>,
    /// The user id.
    pub user_id: Option<&'q uuid::Uuid>,
}

/// What `/Shows/NextUp` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetNextUp<'q> {
    /// Whether to disable sending the first episode in a series as next up.
    pub disable_first_episode: Option<bool>,
    /// Optional. The image types to include in the output.
    pub enable_image_types: Option<&'q Vec<types::ImageType>>,
    /// Optional. Include image information in output.
    pub enable_images: Option<bool>,
    /// Whether to include resumable episodes in next up results.
    pub enable_resumable: Option<bool>,
    /// Whether to include watched episodes in next up results.
    pub enable_rewatching: Option<bool>,
    /// Whether to enable the total records count. Defaults to true.
    pub enable_total_record_count: Option<bool>,
    /// Optional. Include user data.
    pub enable_user_data: Option<bool>,
    /// Optional. Specify additional fields of information to return in the output.
    pub fields: Option<&'q Vec<types::ItemFields>>,
    /// Optional. The max number of images to return, per image type.
    pub image_type_limit: Option<i32>,
    /// Optional. The maximum number of records to return.
    pub limit: Option<i32>,
    /// Optional. Starting date of shows to show in Next Up section.
    pub next_up_date_cutoff: Option<types::Timestamp>,
    /// Optional. Specify this to localize the search to a specific item or folder. Omit to use the root.
    pub parent_id: Option<&'q uuid::Uuid>,
    /// Optional. Filter by series id.
    pub series_id: Option<&'q uuid::Uuid>,
    /// Optional. The record index to start at. All items with a lower index will be dropped from the results.
    pub start_index: Option<i32>,
    /// The user id of the user to get the next up episodes for.
    pub user_id: Option<&'q uuid::Uuid>,
}

/// What `/Shows/Upcoming` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetUpcomingEpisodes<'q> {
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
    /// Optional. Specify this to localize the search to a specific item or folder. Omit to use the root.
    pub parent_id: Option<&'q uuid::Uuid>,
    /// Optional. The record index to start at. All items with a lower index will be dropped from the results.
    pub start_index: Option<i32>,
    /// The user id of the user to get the upcoming episodes for.
    pub user_id: Option<&'q uuid::Uuid>,
}
