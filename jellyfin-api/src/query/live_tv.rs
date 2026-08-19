use crate::types;

/// What `/LiveTv/Channels` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetLiveTvChannels<'q> {
    /// Optional. Adds current program info to each channel.
    pub add_current_program: Option<bool>,
    /// Optional. Incorporate favorite and like status into channel sorting.
    pub enable_favorite_sorting: Option<bool>,
    /// \"Optional. The image types to include in the output.
    pub enable_image_types: Option<&'q Vec<types::ImageType>>,
    /// Optional. Include image information in output.
    pub enable_images: Option<bool>,
    /// Optional. Include user data.
    pub enable_user_data: Option<bool>,
    /// Optional. Specify additional fields of information to return in the output.
    pub fields: Option<&'q Vec<types::ItemFields>>,
    /// Optional. The max number of images to return, per image type.
    pub image_type_limit: Option<i32>,
    /// Optional. Filter by channels that are disliked, or not.
    pub is_disliked: Option<bool>,
    /// Optional. Filter by channels that are favorites, or not.
    pub is_favorite: Option<bool>,
    /// Optional. Filter for kids.
    pub is_kids: Option<bool>,
    /// Optional. Filter by channels that are liked, or not.
    pub is_liked: Option<bool>,
    /// Optional. Filter for movies.
    pub is_movie: Option<bool>,
    /// Optional. Filter for news.
    pub is_news: Option<bool>,
    /// Optional. Filter for series.
    pub is_series: Option<bool>,
    /// Optional. Filter for sports.
    pub is_sports: Option<bool>,
    /// Optional. The maximum number of records to return.
    pub limit: Option<i32>,
    /// Optional. Key to sort by.
    pub sort_by: Option<&'q Vec<types::ItemSortBy>>,
    /// Optional. Sort order.
    pub sort_order: Option<types::SortOrder>,
    /// Optional. The record index to start at. All items with a lower index will be dropped from the results.
    pub start_index: Option<i32>,
    /// Optional. Filter by channel type.
    pub type_: Option<types::ChannelType>,
    /// Optional. Filter by user and attach user data.
    pub user_id: Option<&'q uuid::Uuid>,
}

/// What `/LiveTv/Programs` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetLiveTvPrograms<'q> {
    /// The channels to return guide information for.
    pub channel_ids: Option<&'q Vec<uuid::Uuid>>,
    /// Optional. The image types to include in the output.
    pub enable_image_types: Option<&'q Vec<types::ImageType>>,
    /// Optional. Include image information in output.
    pub enable_images: Option<bool>,
    /// Retrieve total record count.
    pub enable_total_record_count: Option<bool>,
    /// Optional. Include user data.
    pub enable_user_data: Option<bool>,
    /// Optional. Specify additional fields of information to return in the output.
    pub fields: Option<&'q Vec<types::ItemFields>>,
    /// The genre ids to return guide information for.
    pub genre_ids: Option<&'q Vec<uuid::Uuid>>,
    /// The genres to return guide information for.
    pub genres: Option<&'q Vec<String>>,
    /// Optional. Filter by programs that have completed airing, or not.
    pub has_aired: Option<bool>,
    /// Optional. The max number of images to return, per image type.
    pub image_type_limit: Option<i32>,
    /// Optional. Filter by programs that are currently airing, or not.
    pub is_airing: Option<bool>,
    /// Optional. Filter for kids.
    pub is_kids: Option<bool>,
    /// Optional. Filter for movies.
    pub is_movie: Option<bool>,
    /// Optional. Filter for news.
    pub is_news: Option<bool>,
    /// Optional. Filter for series.
    pub is_series: Option<bool>,
    /// Optional. Filter for sports.
    pub is_sports: Option<bool>,
    /// Optional. Filter by library series id.
    pub library_series_id: Option<&'q uuid::Uuid>,
    /// Optional. The maximum number of records to return.
    pub limit: Option<i32>,
    /// Optional. The maximum premiere end date.
    pub max_end_date: Option<types::Timestamp>,
    /// Optional. The maximum premiere start date.
    pub max_start_date: Option<types::Timestamp>,
    /// Optional. The minimum premiere end date.
    pub min_end_date: Option<types::Timestamp>,
    /// Optional. The minimum premiere start date.
    pub min_start_date: Option<types::Timestamp>,
    /// Optional. Filter by series timer id.
    pub series_timer_id: Option<&'q str>,
    /// Optional. Specify one or more sort orders, comma delimited. Options: Name, StartDate.
    pub sort_by: Option<&'q Vec<types::ItemSortBy>>,
    /// Sort Order - Ascending,Descending.
    pub sort_order: Option<&'q Vec<types::SortOrder>>,
    /// Optional. The record index to start at. All items with a lower index will be dropped from the results.
    pub start_index: Option<i32>,
    /// Optional. Filter by user id.
    pub user_id: Option<&'q uuid::Uuid>,
}

/// What `/LiveTv/Programs/Recommended` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetRecommendedPrograms<'q> {
    /// Optional. The image types to include in the output.
    pub enable_image_types: Option<&'q Vec<types::ImageType>>,
    /// Optional. Include image information in output.
    pub enable_images: Option<bool>,
    /// Retrieve total record count.
    pub enable_total_record_count: Option<bool>,
    /// Optional. include user data.
    pub enable_user_data: Option<bool>,
    /// Optional. Specify additional fields of information to return in the output.
    pub fields: Option<&'q Vec<types::ItemFields>>,
    /// The genres to return guide information for.
    pub genre_ids: Option<&'q Vec<uuid::Uuid>>,
    /// Optional. Filter by programs that have completed airing, or not.
    pub has_aired: Option<bool>,
    /// Optional. The max number of images to return, per image type.
    pub image_type_limit: Option<i32>,
    /// Optional. Filter by programs that are currently airing, or not.
    pub is_airing: Option<bool>,
    /// Optional. Filter for kids.
    pub is_kids: Option<bool>,
    /// Optional. Filter for movies.
    pub is_movie: Option<bool>,
    /// Optional. Filter for news.
    pub is_news: Option<bool>,
    /// Optional. Filter for series.
    pub is_series: Option<bool>,
    /// Optional. Filter for sports.
    pub is_sports: Option<bool>,
    /// Optional. The maximum number of records to return.
    pub limit: Option<i32>,
    /// Optional. The record index to start at. All items with a lower index will be dropped from the results.
    pub start_index: Option<i32>,
    /// Optional. filter by user id.
    pub user_id: Option<&'q uuid::Uuid>,
}

/// What `/LiveTv/Recordings` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetRecordings<'q> {
    /// Optional. Filter by channel id.
    pub channel_id: Option<&'q str>,
    /// Optional. The image types to include in the output.
    pub enable_image_types: Option<&'q Vec<types::ImageType>>,
    /// Optional. Include image information in output.
    pub enable_images: Option<bool>,
    /// Optional. Return total record count.
    pub enable_total_record_count: Option<bool>,
    /// Optional. Include user data.
    pub enable_user_data: Option<bool>,
    /// Optional. Specify additional fields of information to return in the output.
    pub fields: Option<&'q Vec<types::ItemFields>>,
    /// Optional. The max number of images to return, per image type.
    pub image_type_limit: Option<i32>,
    /// Optional. Filter by recordings that are in progress, or not.
    pub is_in_progress: Option<bool>,
    /// Optional. Filter for kids.
    pub is_kids: Option<bool>,
    /// Optional. Filter for is library item.
    pub is_library_item: Option<bool>,
    /// Optional. Filter for movies.
    pub is_movie: Option<bool>,
    /// Optional. Filter for news.
    pub is_news: Option<bool>,
    /// Optional. Filter for series.
    pub is_series: Option<bool>,
    /// Optional. Filter for sports.
    pub is_sports: Option<bool>,
    /// Optional. The maximum number of records to return.
    pub limit: Option<i32>,
    /// Optional. Filter by recordings belonging to a series timer.
    pub series_timer_id: Option<&'q str>,
    /// Optional. The record index to start at. All items with a lower index will be dropped from the results.
    pub start_index: Option<i32>,
    /// Optional. Filter by recording status.
    pub status: Option<types::RecordingStatus>,
    /// Optional. Filter by user and attach user data.
    pub user_id: Option<&'q uuid::Uuid>,
}

/// What `/LiveTv/Recordings/Series` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetRecordingsSeries<'q> {
    /// Optional. Filter by channel id.
    pub channel_id: Option<&'q str>,
    /// Optional. The image types to include in the output.
    pub enable_image_types: Option<&'q Vec<types::ImageType>>,
    /// Optional. Include image information in output.
    pub enable_images: Option<bool>,
    /// Optional. Return total record count.
    pub enable_total_record_count: Option<bool>,
    /// Optional. Include user data.
    pub enable_user_data: Option<bool>,
    /// Optional. Specify additional fields of information to return in the output.
    pub fields: Option<&'q Vec<types::ItemFields>>,
    /// Optional. Filter by recording group.
    pub group_id: Option<&'q str>,
    /// Optional. The max number of images to return, per image type.
    pub image_type_limit: Option<i32>,
    /// Optional. Filter by recordings that are in progress, or not.
    pub is_in_progress: Option<bool>,
    /// Optional. The maximum number of records to return.
    pub limit: Option<i32>,
    /// Optional. Filter by recordings belonging to a series timer.
    pub series_timer_id: Option<&'q str>,
    /// Optional. The record index to start at. All items with a lower index will be dropped from the results.
    pub start_index: Option<i32>,
    /// Optional. Filter by recording status.
    pub status: Option<types::RecordingStatus>,
    /// Optional. Filter by user and attach user data.
    pub user_id: Option<&'q uuid::Uuid>,
}
