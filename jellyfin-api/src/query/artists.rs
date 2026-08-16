use crate::types;

/// What `/Artists` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetArtists<'q> {
    /// Optional. The image types to include in the output.
    pub enable_image_types: Option<&'q Vec<types::ImageType>>,
    /// Optional, include image information in output.
    pub enable_images: Option<bool>,
    /// Total record count.
    pub enable_total_record_count: Option<bool>,
    /// Optional, include user data.
    pub enable_user_data: Option<bool>,
    /// Optional. If specified, results will be filtered out based on item type. This allows multiple, comma delimited.
    pub exclude_item_types: Option<&'q Vec<types::BaseItemKind>>,
    /// Optional. Specify additional fields of information to return in the output.
    pub fields: Option<&'q Vec<types::ItemFields>>,
    /// Optional. Specify additional filters to apply.
    pub filters: Option<&'q Vec<types::ItemFilter>>,
    /// Optional. If specified, results will be filtered based on genre id. This allows multiple, pipe delimited.
    pub genre_ids: Option<&'q Vec<uuid::Uuid>>,
    /// Optional. If specified, results will be filtered based on genre. This allows multiple, pipe delimited.
    pub genres: Option<&'q Vec<String>>,
    /// Optional, the max number of images to return, per image type.
    pub image_type_limit: Option<i32>,
    /// Optional. If specified, results will be filtered based on item type. This allows multiple, comma delimited.
    pub include_item_types: Option<&'q Vec<types::BaseItemKind>>,
    /// Optional filter by items that are marked as favorite, or not.
    pub is_favorite: Option<bool>,
    /// Optional. The maximum number of records to return.
    pub limit: Option<i32>,
    /// Optional filter by MediaType. Allows multiple, comma delimited.
    pub media_types: Option<&'q Vec<types::MediaType>>,
    /// Optional filter by minimum community rating.
    pub min_community_rating: Option<f64>,
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
    /// Optional. If specified, results will be filtered to include only those containing the specified person.
    pub person: Option<&'q str>,
    /// Optional. If specified, results will be filtered to include only those containing the specified person ids.
    pub person_ids: Option<&'q Vec<uuid::Uuid>>,
    /// Optional. If specified, along with Person, results will be filtered to include only those containing the specified person and PersonType. Allows multiple, comma-delimited.
    pub person_types: Option<&'q Vec<String>>,
    /// Optional. Search term.
    pub search_term: Option<&'q str>,
    /// Optional. Specify one or more sort orders, comma delimited.
    pub sort_by: Option<&'q Vec<types::ItemSortBy>>,
    /// Sort Order - Ascending,Descending.
    pub sort_order: Option<&'q Vec<types::SortOrder>>,
    /// Optional. The record index to start at. All items with a lower index will be dropped from the results.
    pub start_index: Option<i32>,
    /// Optional. If specified, results will be filtered based on studio id. This allows multiple, pipe delimited.
    pub studio_ids: Option<&'q Vec<uuid::Uuid>>,
    /// Optional. If specified, results will be filtered based on studio. This allows multiple, pipe delimited.
    pub studios: Option<&'q Vec<String>>,
    /// Optional. If specified, results will be filtered based on tag. This allows multiple, pipe delimited.
    pub tags: Option<&'q Vec<String>>,
    /// User id.
    pub user_id: Option<&'q uuid::Uuid>,
    /// Optional. If specified, results will be filtered based on production year. This allows multiple, comma delimited.
    pub years: Option<&'q Vec<i32>>,
}

/// What `/Artists/AlbumArtists` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetAlbumArtists<'q> {
    /// Optional. The image types to include in the output.
    pub enable_image_types: Option<&'q Vec<types::ImageType>>,
    /// Optional, include image information in output.
    pub enable_images: Option<bool>,
    /// Total record count.
    pub enable_total_record_count: Option<bool>,
    /// Optional, include user data.
    pub enable_user_data: Option<bool>,
    /// Optional. If specified, results will be filtered out based on item type. This allows multiple, comma delimited.
    pub exclude_item_types: Option<&'q Vec<types::BaseItemKind>>,
    /// Optional. Specify additional fields of information to return in the output.
    pub fields: Option<&'q Vec<types::ItemFields>>,
    /// Optional. Specify additional filters to apply.
    pub filters: Option<&'q Vec<types::ItemFilter>>,
    /// Optional. If specified, results will be filtered based on genre id. This allows multiple, pipe delimited.
    pub genre_ids: Option<&'q Vec<uuid::Uuid>>,
    /// Optional. If specified, results will be filtered based on genre. This allows multiple, pipe delimited.
    pub genres: Option<&'q Vec<String>>,
    /// Optional, the max number of images to return, per image type.
    pub image_type_limit: Option<i32>,
    /// Optional. If specified, results will be filtered based on item type. This allows multiple, comma delimited.
    pub include_item_types: Option<&'q Vec<types::BaseItemKind>>,
    /// Optional filter by items that are marked as favorite, or not.
    pub is_favorite: Option<bool>,
    /// Optional. The maximum number of records to return.
    pub limit: Option<i32>,
    /// Optional filter by MediaType. Allows multiple, comma delimited.
    pub media_types: Option<&'q Vec<types::MediaType>>,
    /// Optional filter by minimum community rating.
    pub min_community_rating: Option<f64>,
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
    /// Optional. If specified, results will be filtered to include only those containing the specified person.
    pub person: Option<&'q str>,
    /// Optional. If specified, results will be filtered to include only those containing the specified person ids.
    pub person_ids: Option<&'q Vec<uuid::Uuid>>,
    /// Optional. If specified, along with Person, results will be filtered to include only those containing the specified person and PersonType. Allows multiple, comma-delimited.
    pub person_types: Option<&'q Vec<String>>,
    /// Optional. Search term.
    pub search_term: Option<&'q str>,
    /// Optional. Specify one or more sort orders, comma delimited.
    pub sort_by: Option<&'q Vec<types::ItemSortBy>>,
    /// Sort Order - Ascending,Descending.
    pub sort_order: Option<&'q Vec<types::SortOrder>>,
    /// Optional. The record index to start at. All items with a lower index will be dropped from the results.
    pub start_index: Option<i32>,
    /// Optional. If specified, results will be filtered based on studio id. This allows multiple, pipe delimited.
    pub studio_ids: Option<&'q Vec<uuid::Uuid>>,
    /// Optional. If specified, results will be filtered based on studio. This allows multiple, pipe delimited.
    pub studios: Option<&'q Vec<String>>,
    /// Optional. If specified, results will be filtered based on tag. This allows multiple, pipe delimited.
    pub tags: Option<&'q Vec<String>>,
    /// User id.
    pub user_id: Option<&'q uuid::Uuid>,
    /// Optional. If specified, results will be filtered based on production year. This allows multiple, comma delimited.
    pub years: Option<&'q Vec<i32>>,
}

/// What `/Artists/{name}/Images/{imageType}/{imageIndex}` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetArtistImage<'q> {
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

/// What `/Artists/{name}/Images/{imageType}/{imageIndex}` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct HeadArtistImage<'q> {
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

/// What `/Albums/{itemId}/InstantMix` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetInstantMixFromAlbum<'q> {
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

/// What `/Artists/{itemId}/InstantMix` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetInstantMixFromArtist<'q> {
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

/// What `/Artists/InstantMix` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetInstantMixFromArtistById<'q> {
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

/// What `/Songs/{itemId}/InstantMix` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetInstantMixFromSong<'q> {
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
