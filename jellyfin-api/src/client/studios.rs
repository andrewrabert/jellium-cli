use crate::Client;
use crate::error::Error;
use crate::query;
use crate::types;
use crate::util::encode_path;

impl Client {
    #[doc = "Get studio image by name\n\nSends a `GET` request to `/Studios/{name}/Images/{imageType}`\n\nArguments:\n- `name`: Studio name.\n- `image_type`: Image type.\n- `background_color`: Optional. Apply a background color for transparent images.\n- `blur`: Optional. Blur image.\n- `fill_height`: Height of box to fill.\n- `fill_width`: Width of box to fill.\n- `foreground_layer`: Optional. Apply a foreground layer on top of the image.\n- `format`: Determines the output format of the image - original,gif,jpg,png.\n- `height`: The fixed image height to return.\n- `image_index`: Image index.\n- `max_height`: The maximum image height to return.\n- `max_width`: The maximum image width to return.\n- `percent_played`: Optional. Percent to render for the percent played overlay.\n- `quality`: Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases.\n- `tag`: Optional. Supply the cache tag from the item object to receive strong caching headers.\n- `unplayed_count`: Optional. Unplayed count overlay to render.\n- `width`: The fixed image width to return.\n"]
    pub async fn get_studio_image(
        &self,
        name: &str,
        image_type: types::ImageType,
        query: &query::GetStudioImage<'_>,
    ) -> Result<reqwest::Response, Error> {
        self.request(
            reqwest::Method::GET,
            format!(
                "/Studios/{}/Images/{}",
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

    #[doc = "Get studio image by name\n\nSends a `HEAD` request to `/Studios/{name}/Images/{imageType}`\n\nArguments:\n- `name`: Studio name.\n- `image_type`: Image type.\n- `background_color`: Optional. Apply a background color for transparent images.\n- `blur`: Optional. Blur image.\n- `fill_height`: Height of box to fill.\n- `fill_width`: Width of box to fill.\n- `foreground_layer`: Optional. Apply a foreground layer on top of the image.\n- `format`: Determines the output format of the image - original,gif,jpg,png.\n- `height`: The fixed image height to return.\n- `image_index`: Image index.\n- `max_height`: The maximum image height to return.\n- `max_width`: The maximum image width to return.\n- `percent_played`: Optional. Percent to render for the percent played overlay.\n- `quality`: Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases.\n- `tag`: Optional. Supply the cache tag from the item object to receive strong caching headers.\n- `unplayed_count`: Optional. Unplayed count overlay to render.\n- `width`: The fixed image width to return.\n"]
    pub async fn head_studio_image(
        &self,
        name: &str,
        image_type: types::ImageType,
        query: &query::HeadStudioImage<'_>,
    ) -> Result<reqwest::Response, Error> {
        self.request(
            reqwest::Method::HEAD,
            format!(
                "/Studios/{}/Images/{}",
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

    #[doc = "Get studio image by name\n\nSends a `GET` request to `/Studios/{name}/Images/{imageType}/{imageIndex}`\n\nArguments:\n- `name`: Studio name.\n- `image_type`: Image type.\n- `image_index`: Image index.\n- `background_color`: Optional. Apply a background color for transparent images.\n- `blur`: Optional. Blur image.\n- `fill_height`: Height of box to fill.\n- `fill_width`: Width of box to fill.\n- `foreground_layer`: Optional. Apply a foreground layer on top of the image.\n- `format`: Determines the output format of the image - original,gif,jpg,png.\n- `height`: The fixed image height to return.\n- `max_height`: The maximum image height to return.\n- `max_width`: The maximum image width to return.\n- `percent_played`: Optional. Percent to render for the percent played overlay.\n- `quality`: Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases.\n- `tag`: Optional. Supply the cache tag from the item object to receive strong caching headers.\n- `unplayed_count`: Optional. Unplayed count overlay to render.\n- `width`: The fixed image width to return.\n"]
    pub async fn get_studio_image_by_index(
        &self,
        name: &str,
        image_type: types::ImageType,
        image_index: i32,
        query: &query::GetStudioImageByIndex<'_>,
    ) -> Result<reqwest::Response, Error> {
        self.request(
            reqwest::Method::GET,
            format!(
                "/Studios/{}/Images/{}/{}",
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

    #[doc = "Get studio image by name\n\nSends a `HEAD` request to `/Studios/{name}/Images/{imageType}/{imageIndex}`\n\nArguments:\n- `name`: Studio name.\n- `image_type`: Image type.\n- `image_index`: Image index.\n- `background_color`: Optional. Apply a background color for transparent images.\n- `blur`: Optional. Blur image.\n- `fill_height`: Height of box to fill.\n- `fill_width`: Width of box to fill.\n- `foreground_layer`: Optional. Apply a foreground layer on top of the image.\n- `format`: Determines the output format of the image - original,gif,jpg,png.\n- `height`: The fixed image height to return.\n- `max_height`: The maximum image height to return.\n- `max_width`: The maximum image width to return.\n- `percent_played`: Optional. Percent to render for the percent played overlay.\n- `quality`: Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases.\n- `tag`: Optional. Supply the cache tag from the item object to receive strong caching headers.\n- `unplayed_count`: Optional. Unplayed count overlay to render.\n- `width`: The fixed image width to return.\n"]
    pub async fn head_studio_image_by_index(
        &self,
        name: &str,
        image_type: types::ImageType,
        image_index: i32,
        query: &query::HeadStudioImageByIndex<'_>,
    ) -> Result<reqwest::Response, Error> {
        self.request(
            reqwest::Method::HEAD,
            format!(
                "/Studios/{}/Images/{}/{}",
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

    #[doc = "Gets all studios from a given item, folder, or the entire library\n\nSends a `GET` request to `/Studios`\n\nArguments:\n- `enable_image_types`: Optional. The image types to include in the output.\n- `enable_images`: Optional, include image information in output.\n- `enable_total_record_count`: Total record count.\n- `enable_user_data`: Optional, include user data.\n- `exclude_item_types`: Optional. If specified, results will be filtered out based on item type. This allows multiple, comma delimited.\n- `fields`: Optional. Specify additional fields of information to return in the output.\n- `image_type_limit`: Optional, the max number of images to return, per image type.\n- `include_item_types`: Optional. If specified, results will be filtered based on item type. This allows multiple, comma delimited.\n- `is_favorite`: Optional filter by items that are marked as favorite, or not.\n- `limit`: Optional. The maximum number of records to return.\n- `name_less_than`: Optional filter by items whose name is equally or lesser than a given input string.\n- `name_starts_with`: Optional filter by items whose name is sorted equally than a given input string.\n- `name_starts_with_or_greater`: Optional filter by items whose name is sorted equally or greater than a given input string.\n- `parent_id`: Specify this to localize the search to a specific item or folder. Omit to use the root.\n- `search_term`: Optional. Search term.\n- `start_index`: Optional. The record index to start at. All items with a lower index will be dropped from the results.\n- `user_id`: User id.\n"]
    pub async fn get_studios(
        &self,
        query: &query::GetStudios<'_>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(reqwest::Method::GET, "/Studios".into())
            .query_list_opt("enableImageTypes", query.enable_image_types)
            .query_opt("enableImages", query.enable_images)
            .query_opt("enableTotalRecordCount", query.enable_total_record_count)
            .query_opt("enableUserData", query.enable_user_data)
            .query_list_opt("excludeItemTypes", query.exclude_item_types)
            .query_list_opt("fields", query.fields)
            .query_opt("imageTypeLimit", query.image_type_limit)
            .query_list_opt("includeItemTypes", query.include_item_types)
            .query_opt("isFavorite", query.is_favorite)
            .query_opt("limit", query.limit)
            .query_opt("nameLessThan", query.name_less_than)
            .query_opt("nameStartsWith", query.name_starts_with)
            .query_opt("nameStartsWithOrGreater", query.name_starts_with_or_greater)
            .query_opt("parentId", query.parent_id)
            .query_opt("searchTerm", query.search_term)
            .query_opt("startIndex", query.start_index)
            .query_opt("userId", query.user_id)
            .send()
            .await
    }

    #[doc = "Gets a studio by name\n\nSends a `GET` request to `/Studios/{name}`\n\nArguments:\n- `name`: Studio name.\n- `user_id`: Optional. Filter by user id, and attach user data.\n"]
    pub async fn get_studio(
        &self,
        name: &str,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::BaseItemDto, Error> {
        self.request(
            reqwest::Method::GET,
            format!("/Studios/{}", encode_path(name)),
        )
        .query_opt("userId", user_id)
        .send()
        .await
    }
}
