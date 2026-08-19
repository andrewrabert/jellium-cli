use crate::types;

/// What `/MusicGenres/{name}/Images/{imageType}` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetMusicGenreImage<'q> {
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
    /// Determines the output format of the image - original,gif,jpg,png.
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

/// What `/MusicGenres/{name}/Images/{imageType}` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct HeadMusicGenreImage<'q> {
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
    /// Determines the output format of the image - original,gif,jpg,png.
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

/// What `/MusicGenres/{name}/Images/{imageType}/{imageIndex}` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetMusicGenreImageByIndex<'q> {
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
    /// Determines the output format of the image - original,gif,jpg,png.
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

/// What `/MusicGenres/{name}/Images/{imageType}/{imageIndex}` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct HeadMusicGenreImageByIndex<'q> {
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
    /// Determines the output format of the image - original,gif,jpg,png.
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

/// What `/MusicGenres/{name}/InstantMix` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetInstantMixFromMusicGenreByName<'q> {
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

/// What `/MusicGenres/InstantMix` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetInstantMixFromMusicGenreById<'q> {
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

/// What `/MusicGenres` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetMusicGenres<'q> {
    /// Optional. The image types to include in the output.
    pub enable_image_types: Option<&'q Vec<types::ImageType>>,
    /// Optional, include image information in output.
    pub enable_images: Option<bool>,
    /// Optional. Include total record count.
    pub enable_total_record_count: Option<bool>,
    /// Optional. If specified, results will be filtered out based on item type. This allows multiple, comma delimited.
    pub exclude_item_types: Option<&'q Vec<types::BaseItemKind>>,
    /// Optional. Specify additional fields of information to return in the output.
    pub fields: Option<&'q Vec<types::ItemFields>>,
    /// Optional, the max number of images to return, per image type.
    pub image_type_limit: Option<i32>,
    /// Optional. If specified, results will be filtered in based on item type. This allows multiple, comma delimited.
    pub include_item_types: Option<&'q Vec<types::BaseItemKind>>,
    /// Optional filter by items that are marked as favorite, or not.
    pub is_favorite: Option<bool>,
    /// Optional. The maximum number of records to return.
    pub limit: Option<i32>,
    /// Optional filter by items whose name is equally or lesser than a given input string.
    pub name_less_than: Option<&'q str>,
    /// Optional filter by items whose name is sorted equally than a given input string.
    pub name_starts_with: Option<&'q str>,
    /// Optional filter by items whose name is sorted equally or greater than a given input string.
    pub name_starts_with_or_greater: Option<&'q str>,
    /// Specify this to localize the search to a specific item or folder. Omit to use the root.
    pub parent_id: Option<&'q uuid::Uuid>,
    /// The search term.
    pub search_term: Option<&'q str>,
    /// Optional. Specify one or more sort orders, comma delimited.
    pub sort_by: Option<&'q Vec<types::ItemSortBy>>,
    /// Sort Order - Ascending,Descending.
    pub sort_order: Option<&'q Vec<types::SortOrder>>,
    /// Optional. The record index to start at. All items with a lower index will be dropped from the results.
    pub start_index: Option<i32>,
    /// User id.
    pub user_id: Option<&'q uuid::Uuid>,
}
