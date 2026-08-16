use crate::types;

/// What `/Playlists/{itemId}/InstantMix` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetInstantMixFromPlaylist<'q> {
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

/// What `/Playlists/{playlistId}/Items` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetPlaylistItems<'q> {
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
    /// Optional. The record index to start at. All items with a lower index will be dropped from the results.
    pub start_index: Option<i32>,
    /// User id.
    pub user_id: Option<&'q uuid::Uuid>,
}
