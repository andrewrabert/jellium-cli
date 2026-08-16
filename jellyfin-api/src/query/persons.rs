use crate::types;

/// What `/Persons/{name}/Images/{imageType}` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetPersonImage<'q> {
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

/// What `/Persons/{name}/Images/{imageType}` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct HeadPersonImage<'q> {
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

/// What `/Persons/{name}/Images/{imageType}/{imageIndex}` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetPersonImageByIndex<'q> {
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

/// What `/Persons/{name}/Images/{imageType}/{imageIndex}` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct HeadPersonImageByIndex<'q> {
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

/// What `/Persons` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetPersons<'q> {
    /// Optional. If specified, person results will be filtered on items related to said persons.
    pub appears_in_item_id: Option<&'q uuid::Uuid>,
    /// Optional. The image types to include in the output.
    pub enable_image_types: Option<&'q Vec<types::ImageType>>,
    /// Optional, include image information in output.
    pub enable_images: Option<bool>,
    /// Optional, include user data.
    pub enable_user_data: Option<bool>,
    /// Optional. If specified results will be filtered to exclude those containing the specified PersonType. Allows multiple, comma-delimited.
    pub exclude_person_types: Option<&'q Vec<String>>,
    /// Optional. Specify additional fields of information to return in the output.
    pub fields: Option<&'q Vec<types::ItemFields>>,
    /// Optional. Specify additional filters to apply.
    pub filters: Option<&'q Vec<types::ItemFilter>>,
    /// Optional, the max number of images to return, per image type.
    pub image_type_limit: Option<i32>,
    /// Optional filter by items that are marked as favorite, or not. userId is required.
    pub is_favorite: Option<bool>,
    /// Optional. The maximum number of records to return.
    pub limit: Option<i32>,
    /// Optional. If specified results will be filtered to include only those containing the specified PersonType. Allows multiple, comma-delimited.
    pub person_types: Option<&'q Vec<String>>,
    /// The search term.
    pub search_term: Option<&'q str>,
    /// User id.
    pub user_id: Option<&'q uuid::Uuid>,
}
