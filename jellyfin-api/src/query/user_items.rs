use crate::types;

/// What `/UserItems/Resume` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetResumeItems<'q> {
    /// Optional. The image types to include in the output.
    pub enable_image_types: Option<&'q Vec<types::ImageType>>,
    /// Optional. Include image information in output.
    pub enable_images: Option<bool>,
    /// Optional. Enable the total record count.
    pub enable_total_record_count: Option<bool>,
    /// Optional. Include user data.
    pub enable_user_data: Option<bool>,
    /// Optional. Whether to exclude the currently active sessions.
    pub exclude_active_sessions: Option<bool>,
    /// Optional. If specified, results will be filtered based on item type. This allows multiple, comma delimited.
    pub exclude_item_types: Option<&'q Vec<types::BaseItemKind>>,
    /// Optional. Specify additional fields of information to return in the output. This allows multiple, comma delimited. Options: Budget, Chapters, DateCreated, Genres, HomePageUrl, IndexOptions, MediaStreams, Overview, ParentId, Path, People, ProviderIds, PrimaryImageAspectRatio, Revenue, SortName, Studios, Taglines.
    pub fields: Option<&'q Vec<types::ItemFields>>,
    /// Optional. The max number of images to return, per image type.
    pub image_type_limit: Option<i32>,
    /// Optional. If specified, results will be filtered based on the item type. This allows multiple, comma delimited.
    pub include_item_types: Option<&'q Vec<types::BaseItemKind>>,
    /// The item limit.
    pub limit: Option<i32>,
    /// Optional. Filter by MediaType. Allows multiple, comma delimited.
    pub media_types: Option<&'q Vec<types::MediaType>>,
    /// Specify this to localize the search to a specific item or folder. Omit to use the root.
    pub parent_id: Option<&'q uuid::Uuid>,
    /// The search term.
    pub search_term: Option<&'q str>,
    /// The start index.
    pub start_index: Option<i32>,
    /// The user id.
    pub user_id: Option<&'q uuid::Uuid>,
}
