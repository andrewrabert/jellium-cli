use crate::Client;
use crate::error::Error;
use crate::query;
use crate::types;
use crate::util::encode_path;

impl Client {
    #[doc = "Get person image by name\n\nSends a `GET` request to `/Persons/{name}/Images/{imageType}`\n\nArguments:\n- `name`: Person name.\n- `image_type`: Image type.\n- `background_color`: Optional. Apply a background color for transparent images.\n- `blur`: Optional. Blur image.\n- `fill_height`: Height of box to fill.\n- `fill_width`: Width of box to fill.\n- `foreground_layer`: Optional. Apply a foreground layer on top of the image.\n- `format`: Determines the output format of the image - original,gif,jpg,png.\n- `height`: The fixed image height to return.\n- `image_index`: Image index.\n- `max_height`: The maximum image height to return.\n- `max_width`: The maximum image width to return.\n- `percent_played`: Optional. Percent to render for the percent played overlay.\n- `quality`: Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases.\n- `tag`: Optional. Supply the cache tag from the item object to receive strong caching headers.\n- `unplayed_count`: Optional. Unplayed count overlay to render.\n- `width`: The fixed image width to return.\n"]
    pub async fn get_person_image(
        &self,
        name: &str,
        image_type: types::ImageType,
        query: &query::GetPersonImage<'_>,
    ) -> Result<reqwest::Response, Error> {
        self.request(
            reqwest::Method::GET,
            format!(
                "/Persons/{}/Images/{}",
                encode_path(name),
                encode_path(&image_type.to_string())
            ),
        )
        .query_opt("backgroundColor", query.background_color)
        .query_opt("blur", query.blur)
        .query_opt("fillHeight", query.fill_height)
        .query_opt("fillWidth", query.fill_width)
        .query_opt("foregroundLayer", query.foreground_layer)
        .query_opt("format", query.format)
        .query_opt("height", query.height)
        .query_opt("imageIndex", query.image_index)
        .query_opt("maxHeight", query.max_height)
        .query_opt("maxWidth", query.max_width)
        .query_opt("percentPlayed", query.percent_played)
        .query_opt("quality", query.quality)
        .query_opt("tag", query.tag)
        .query_opt("unplayedCount", query.unplayed_count)
        .query_opt("width", query.width)
        .send_response()
        .await
    }

    #[doc = "Get person image by name\n\nSends a `HEAD` request to `/Persons/{name}/Images/{imageType}`\n\nArguments:\n- `name`: Person name.\n- `image_type`: Image type.\n- `background_color`: Optional. Apply a background color for transparent images.\n- `blur`: Optional. Blur image.\n- `fill_height`: Height of box to fill.\n- `fill_width`: Width of box to fill.\n- `foreground_layer`: Optional. Apply a foreground layer on top of the image.\n- `format`: Determines the output format of the image - original,gif,jpg,png.\n- `height`: The fixed image height to return.\n- `image_index`: Image index.\n- `max_height`: The maximum image height to return.\n- `max_width`: The maximum image width to return.\n- `percent_played`: Optional. Percent to render for the percent played overlay.\n- `quality`: Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases.\n- `tag`: Optional. Supply the cache tag from the item object to receive strong caching headers.\n- `unplayed_count`: Optional. Unplayed count overlay to render.\n- `width`: The fixed image width to return.\n"]
    pub async fn head_person_image(
        &self,
        name: &str,
        image_type: types::ImageType,
        query: &query::HeadPersonImage<'_>,
    ) -> Result<reqwest::Response, Error> {
        self.request(
            reqwest::Method::HEAD,
            format!(
                "/Persons/{}/Images/{}",
                encode_path(name),
                encode_path(&image_type.to_string())
            ),
        )
        .query_opt("backgroundColor", query.background_color)
        .query_opt("blur", query.blur)
        .query_opt("fillHeight", query.fill_height)
        .query_opt("fillWidth", query.fill_width)
        .query_opt("foregroundLayer", query.foreground_layer)
        .query_opt("format", query.format)
        .query_opt("height", query.height)
        .query_opt("imageIndex", query.image_index)
        .query_opt("maxHeight", query.max_height)
        .query_opt("maxWidth", query.max_width)
        .query_opt("percentPlayed", query.percent_played)
        .query_opt("quality", query.quality)
        .query_opt("tag", query.tag)
        .query_opt("unplayedCount", query.unplayed_count)
        .query_opt("width", query.width)
        .send_response()
        .await
    }

    #[doc = "Get person image by name\n\nSends a `GET` request to `/Persons/{name}/Images/{imageType}/{imageIndex}`\n\nArguments:\n- `name`: Person name.\n- `image_type`: Image type.\n- `image_index`: Image index.\n- `background_color`: Optional. Apply a background color for transparent images.\n- `blur`: Optional. Blur image.\n- `fill_height`: Height of box to fill.\n- `fill_width`: Width of box to fill.\n- `foreground_layer`: Optional. Apply a foreground layer on top of the image.\n- `format`: Determines the output format of the image - original,gif,jpg,png.\n- `height`: The fixed image height to return.\n- `max_height`: The maximum image height to return.\n- `max_width`: The maximum image width to return.\n- `percent_played`: Optional. Percent to render for the percent played overlay.\n- `quality`: Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases.\n- `tag`: Optional. Supply the cache tag from the item object to receive strong caching headers.\n- `unplayed_count`: Optional. Unplayed count overlay to render.\n- `width`: The fixed image width to return.\n"]
    pub async fn get_person_image_by_index(
        &self,
        name: &str,
        image_type: types::ImageType,
        image_index: i32,
        query: &query::GetPersonImageByIndex<'_>,
    ) -> Result<reqwest::Response, Error> {
        self.request(
            reqwest::Method::GET,
            format!(
                "/Persons/{}/Images/{}/{}",
                encode_path(name),
                encode_path(&image_type.to_string()),
                encode_path(&image_index.to_string())
            ),
        )
        .query_opt("backgroundColor", query.background_color)
        .query_opt("blur", query.blur)
        .query_opt("fillHeight", query.fill_height)
        .query_opt("fillWidth", query.fill_width)
        .query_opt("foregroundLayer", query.foreground_layer)
        .query_opt("format", query.format)
        .query_opt("height", query.height)
        .query_opt("maxHeight", query.max_height)
        .query_opt("maxWidth", query.max_width)
        .query_opt("percentPlayed", query.percent_played)
        .query_opt("quality", query.quality)
        .query_opt("tag", query.tag)
        .query_opt("unplayedCount", query.unplayed_count)
        .query_opt("width", query.width)
        .send_response()
        .await
    }

    #[doc = "Get person image by name\n\nSends a `HEAD` request to `/Persons/{name}/Images/{imageType}/{imageIndex}`\n\nArguments:\n- `name`: Person name.\n- `image_type`: Image type.\n- `image_index`: Image index.\n- `background_color`: Optional. Apply a background color for transparent images.\n- `blur`: Optional. Blur image.\n- `fill_height`: Height of box to fill.\n- `fill_width`: Width of box to fill.\n- `foreground_layer`: Optional. Apply a foreground layer on top of the image.\n- `format`: Determines the output format of the image - original,gif,jpg,png.\n- `height`: The fixed image height to return.\n- `max_height`: The maximum image height to return.\n- `max_width`: The maximum image width to return.\n- `percent_played`: Optional. Percent to render for the percent played overlay.\n- `quality`: Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases.\n- `tag`: Optional. Supply the cache tag from the item object to receive strong caching headers.\n- `unplayed_count`: Optional. Unplayed count overlay to render.\n- `width`: The fixed image width to return.\n"]
    pub async fn head_person_image_by_index(
        &self,
        name: &str,
        image_type: types::ImageType,
        image_index: i32,
        query: &query::HeadPersonImageByIndex<'_>,
    ) -> Result<reqwest::Response, Error> {
        self.request(
            reqwest::Method::HEAD,
            format!(
                "/Persons/{}/Images/{}/{}",
                encode_path(name),
                encode_path(&image_type.to_string()),
                encode_path(&image_index.to_string())
            ),
        )
        .query_opt("backgroundColor", query.background_color)
        .query_opt("blur", query.blur)
        .query_opt("fillHeight", query.fill_height)
        .query_opt("fillWidth", query.fill_width)
        .query_opt("foregroundLayer", query.foreground_layer)
        .query_opt("format", query.format)
        .query_opt("height", query.height)
        .query_opt("maxHeight", query.max_height)
        .query_opt("maxWidth", query.max_width)
        .query_opt("percentPlayed", query.percent_played)
        .query_opt("quality", query.quality)
        .query_opt("tag", query.tag)
        .query_opt("unplayedCount", query.unplayed_count)
        .query_opt("width", query.width)
        .send_response()
        .await
    }

    #[doc = "Gets all persons\n\nSends a `GET` request to `/Persons`\n\nArguments:\n- `appears_in_item_id`: Optional. If specified, person results will be filtered on items related to said persons.\n- `enable_image_types`: Optional. The image types to include in the output.\n- `enable_images`: Optional, include image information in output.\n- `enable_user_data`: Optional, include user data.\n- `exclude_person_types`: Optional. If specified results will be filtered to exclude those containing the specified PersonType. Allows multiple, comma-delimited.\n- `fields`: Optional. Specify additional fields of information to return in the output.\n- `filters`: Optional. Specify additional filters to apply.\n- `image_type_limit`: Optional, the max number of images to return, per image type.\n- `is_favorite`: Optional filter by items that are marked as favorite, or not. userId is required.\n- `limit`: Optional. The maximum number of records to return.\n- `person_types`: Optional. If specified results will be filtered to include only those containing the specified PersonType. Allows multiple, comma-delimited.\n- `search_term`: The search term.\n- `user_id`: User id.\n"]
    pub async fn get_persons(
        &self,
        query: &query::GetPersons<'_>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(reqwest::Method::GET, "/Persons".into())
            .query_opt("appearsInItemId", query.appears_in_item_id)
            .query_list_opt("enableImageTypes", query.enable_image_types)
            .query_opt("enableImages", query.enable_images)
            .query_opt("enableUserData", query.enable_user_data)
            .query_list_opt("excludePersonTypes", query.exclude_person_types)
            .query_list_opt("fields", query.fields)
            .query_list_opt("filters", query.filters)
            .query_opt("imageTypeLimit", query.image_type_limit)
            .query_opt("isFavorite", query.is_favorite)
            .query_opt("limit", query.limit)
            .query_list_opt("personTypes", query.person_types)
            .query_opt("searchTerm", query.search_term)
            .query_opt("userId", query.user_id)
            .send()
            .await
    }

    #[doc = "Get person by name\n\nSends a `GET` request to `/Persons/{name}`\n\nArguments:\n- `name`: Person name.\n- `user_id`: Optional. Filter by user id, and attach user data.\n"]
    pub async fn get_person(
        &self,
        name: &str,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::BaseItemDto, Error> {
        self.request(
            reqwest::Method::GET,
            format!("/Persons/{}", encode_path(name)),
        )
        .query_opt("userId", user_id)
        .send()
        .await
    }
}
