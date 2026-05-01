use crate::types;
use crate::error::Error;
use crate::util::encode_path;
use crate::Client;

impl Client {
    #[doc = "Get person image by name\n\nSends a `GET` request to `/Persons/{name}/Images/{imageType}`\n\nArguments:\n- `name`: Person name.\n- `image_type`: Image type.\n- `background_color`: Optional. Apply a background color for transparent images.\n- `blur`: Optional. Blur image.\n- `fill_height`: Height of box to fill.\n- `fill_width`: Width of box to fill.\n- `foreground_layer`: Optional. Apply a foreground layer on top of the image.\n- `format`: Determines the output format of the image - original,gif,jpg,png.\n- `height`: The fixed image height to return.\n- `image_index`: Image index.\n- `max_height`: The maximum image height to return.\n- `max_width`: The maximum image width to return.\n- `percent_played`: Optional. Percent to render for the percent played overlay.\n- `quality`: Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases.\n- `tag`: Optional. Supply the cache tag from the item object to receive strong caching headers.\n- `unplayed_count`: Optional. Unplayed count overlay to render.\n- `width`: The fixed image width to return.\n"]
    pub async fn get_person_image(
        &self,
        name: &str,
        image_type: types::ImageType,
        background_color: Option<&str>,
        blur: Option<i32>,
        fill_height: Option<i32>,
        fill_width: Option<i32>,
        foreground_layer: Option<&str>,
        format: Option<types::ImageFormat>,
        height: Option<i32>,
        image_index: Option<i32>,
        max_height: Option<i32>,
        max_width: Option<i32>,
        percent_played: Option<f64>,
        quality: Option<i32>,
        tag: Option<&str>,
        unplayed_count: Option<i32>,
        width: Option<i32>,
    ) -> Result<reqwest::Response, Error> {
        self.request(reqwest::Method::GET, format!("/Persons/{}/Images/{}", encode_path(name), encode_path(&image_type.to_string())))
            .query_opt("backgroundColor", background_color)
            .query_opt("blur", blur)
            .query_opt("fillHeight", fill_height)
            .query_opt("fillWidth", fill_width)
            .query_opt("foregroundLayer", foreground_layer)
            .query_opt("format", format)
            .query_opt("height", height)
            .query_opt("imageIndex", image_index)
            .query_opt("maxHeight", max_height)
            .query_opt("maxWidth", max_width)
            .query_opt("percentPlayed", percent_played)
            .query_opt("quality", quality)
            .query_opt("tag", tag)
            .query_opt("unplayedCount", unplayed_count)
            .query_opt("width", width)
            .send_response()
            .await
    }

    #[doc = "Get person image by name\n\nSends a `HEAD` request to `/Persons/{name}/Images/{imageType}`\n\nArguments:\n- `name`: Person name.\n- `image_type`: Image type.\n- `background_color`: Optional. Apply a background color for transparent images.\n- `blur`: Optional. Blur image.\n- `fill_height`: Height of box to fill.\n- `fill_width`: Width of box to fill.\n- `foreground_layer`: Optional. Apply a foreground layer on top of the image.\n- `format`: Determines the output format of the image - original,gif,jpg,png.\n- `height`: The fixed image height to return.\n- `image_index`: Image index.\n- `max_height`: The maximum image height to return.\n- `max_width`: The maximum image width to return.\n- `percent_played`: Optional. Percent to render for the percent played overlay.\n- `quality`: Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases.\n- `tag`: Optional. Supply the cache tag from the item object to receive strong caching headers.\n- `unplayed_count`: Optional. Unplayed count overlay to render.\n- `width`: The fixed image width to return.\n"]
    pub async fn head_person_image(
        &self,
        name: &str,
        image_type: types::ImageType,
        background_color: Option<&str>,
        blur: Option<i32>,
        fill_height: Option<i32>,
        fill_width: Option<i32>,
        foreground_layer: Option<&str>,
        format: Option<types::ImageFormat>,
        height: Option<i32>,
        image_index: Option<i32>,
        max_height: Option<i32>,
        max_width: Option<i32>,
        percent_played: Option<f64>,
        quality: Option<i32>,
        tag: Option<&str>,
        unplayed_count: Option<i32>,
        width: Option<i32>,
    ) -> Result<reqwest::Response, Error> {
        self.request(reqwest::Method::HEAD, format!("/Persons/{}/Images/{}", encode_path(name), encode_path(&image_type.to_string())))
            .query_opt("backgroundColor", background_color)
            .query_opt("blur", blur)
            .query_opt("fillHeight", fill_height)
            .query_opt("fillWidth", fill_width)
            .query_opt("foregroundLayer", foreground_layer)
            .query_opt("format", format)
            .query_opt("height", height)
            .query_opt("imageIndex", image_index)
            .query_opt("maxHeight", max_height)
            .query_opt("maxWidth", max_width)
            .query_opt("percentPlayed", percent_played)
            .query_opt("quality", quality)
            .query_opt("tag", tag)
            .query_opt("unplayedCount", unplayed_count)
            .query_opt("width", width)
            .send_response()
            .await
    }

    #[doc = "Get person image by name\n\nSends a `GET` request to `/Persons/{name}/Images/{imageType}/{imageIndex}`\n\nArguments:\n- `name`: Person name.\n- `image_type`: Image type.\n- `image_index`: Image index.\n- `background_color`: Optional. Apply a background color for transparent images.\n- `blur`: Optional. Blur image.\n- `fill_height`: Height of box to fill.\n- `fill_width`: Width of box to fill.\n- `foreground_layer`: Optional. Apply a foreground layer on top of the image.\n- `format`: Determines the output format of the image - original,gif,jpg,png.\n- `height`: The fixed image height to return.\n- `max_height`: The maximum image height to return.\n- `max_width`: The maximum image width to return.\n- `percent_played`: Optional. Percent to render for the percent played overlay.\n- `quality`: Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases.\n- `tag`: Optional. Supply the cache tag from the item object to receive strong caching headers.\n- `unplayed_count`: Optional. Unplayed count overlay to render.\n- `width`: The fixed image width to return.\n"]
    pub async fn get_person_image_by_index(
        &self,
        name: &str,
        image_type: types::ImageType,
        image_index: i32,
        background_color: Option<&str>,
        blur: Option<i32>,
        fill_height: Option<i32>,
        fill_width: Option<i32>,
        foreground_layer: Option<&str>,
        format: Option<types::ImageFormat>,
        height: Option<i32>,
        max_height: Option<i32>,
        max_width: Option<i32>,
        percent_played: Option<f64>,
        quality: Option<i32>,
        tag: Option<&str>,
        unplayed_count: Option<i32>,
        width: Option<i32>,
    ) -> Result<reqwest::Response, Error> {
        self.request(reqwest::Method::GET, format!("/Persons/{}/Images/{}/{}", encode_path(name), encode_path(&image_type.to_string()), encode_path(&image_index.to_string())))
            .query_opt("backgroundColor", background_color)
            .query_opt("blur", blur)
            .query_opt("fillHeight", fill_height)
            .query_opt("fillWidth", fill_width)
            .query_opt("foregroundLayer", foreground_layer)
            .query_opt("format", format)
            .query_opt("height", height)
            .query_opt("maxHeight", max_height)
            .query_opt("maxWidth", max_width)
            .query_opt("percentPlayed", percent_played)
            .query_opt("quality", quality)
            .query_opt("tag", tag)
            .query_opt("unplayedCount", unplayed_count)
            .query_opt("width", width)
            .send_response()
            .await
    }

    #[doc = "Get person image by name\n\nSends a `HEAD` request to `/Persons/{name}/Images/{imageType}/{imageIndex}`\n\nArguments:\n- `name`: Person name.\n- `image_type`: Image type.\n- `image_index`: Image index.\n- `background_color`: Optional. Apply a background color for transparent images.\n- `blur`: Optional. Blur image.\n- `fill_height`: Height of box to fill.\n- `fill_width`: Width of box to fill.\n- `foreground_layer`: Optional. Apply a foreground layer on top of the image.\n- `format`: Determines the output format of the image - original,gif,jpg,png.\n- `height`: The fixed image height to return.\n- `max_height`: The maximum image height to return.\n- `max_width`: The maximum image width to return.\n- `percent_played`: Optional. Percent to render for the percent played overlay.\n- `quality`: Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases.\n- `tag`: Optional. Supply the cache tag from the item object to receive strong caching headers.\n- `unplayed_count`: Optional. Unplayed count overlay to render.\n- `width`: The fixed image width to return.\n"]
    pub async fn head_person_image_by_index(
        &self,
        name: &str,
        image_type: types::ImageType,
        image_index: i32,
        background_color: Option<&str>,
        blur: Option<i32>,
        fill_height: Option<i32>,
        fill_width: Option<i32>,
        foreground_layer: Option<&str>,
        format: Option<types::ImageFormat>,
        height: Option<i32>,
        max_height: Option<i32>,
        max_width: Option<i32>,
        percent_played: Option<f64>,
        quality: Option<i32>,
        tag: Option<&str>,
        unplayed_count: Option<i32>,
        width: Option<i32>,
    ) -> Result<reqwest::Response, Error> {
        self.request(reqwest::Method::HEAD, format!("/Persons/{}/Images/{}/{}", encode_path(name), encode_path(&image_type.to_string()), encode_path(&image_index.to_string())))
            .query_opt("backgroundColor", background_color)
            .query_opt("blur", blur)
            .query_opt("fillHeight", fill_height)
            .query_opt("fillWidth", fill_width)
            .query_opt("foregroundLayer", foreground_layer)
            .query_opt("format", format)
            .query_opt("height", height)
            .query_opt("maxHeight", max_height)
            .query_opt("maxWidth", max_width)
            .query_opt("percentPlayed", percent_played)
            .query_opt("quality", quality)
            .query_opt("tag", tag)
            .query_opt("unplayedCount", unplayed_count)
            .query_opt("width", width)
            .send_response()
            .await
    }

    #[doc = "Gets all persons\n\nSends a `GET` request to `/Persons`\n\nArguments:\n- `appears_in_item_id`: Optional. If specified, person results will be filtered on items related to said persons.\n- `enable_image_types`: Optional. The image types to include in the output.\n- `enable_images`: Optional, include image information in output.\n- `enable_user_data`: Optional, include user data.\n- `exclude_person_types`: Optional. If specified results will be filtered to exclude those containing the specified PersonType. Allows multiple, comma-delimited.\n- `fields`: Optional. Specify additional fields of information to return in the output.\n- `filters`: Optional. Specify additional filters to apply.\n- `image_type_limit`: Optional, the max number of images to return, per image type.\n- `is_favorite`: Optional filter by items that are marked as favorite, or not. userId is required.\n- `limit`: Optional. The maximum number of records to return.\n- `person_types`: Optional. If specified results will be filtered to include only those containing the specified PersonType. Allows multiple, comma-delimited.\n- `search_term`: The search term.\n- `user_id`: User id.\n"]
    pub async fn get_persons(
        &self,
        appears_in_item_id: Option<&uuid::Uuid>,
        enable_image_types: Option<&Vec<types::ImageType>>,
        enable_images: Option<bool>,
        enable_user_data: Option<bool>,
        exclude_person_types: Option<&Vec<String>>,
        fields: Option<&Vec<types::ItemFields>>,
        filters: Option<&Vec<types::ItemFilter>>,
        image_type_limit: Option<i32>,
        is_favorite: Option<bool>,
        limit: Option<i32>,
        person_types: Option<&Vec<String>>,
        search_term: Option<&str>,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(reqwest::Method::GET, "/Persons".into())
            .query_opt("appearsInItemId", appears_in_item_id)
            .query_list_opt("enableImageTypes", enable_image_types)
            .query_opt("enableImages", enable_images)
            .query_opt("enableUserData", enable_user_data)
            .query_list_opt("excludePersonTypes", exclude_person_types)
            .query_list_opt("fields", fields)
            .query_list_opt("filters", filters)
            .query_opt("imageTypeLimit", image_type_limit)
            .query_opt("isFavorite", is_favorite)
            .query_opt("limit", limit)
            .query_list_opt("personTypes", person_types)
            .query_opt("searchTerm", search_term)
            .query_opt("userId", user_id)
            .send()
            .await
    }

    #[doc = "Get person by name\n\nSends a `GET` request to `/Persons/{name}`\n\nArguments:\n- `name`: Person name.\n- `user_id`: Optional. Filter by user id, and attach user data.\n"]
    pub async fn get_person(
        &self,
        name: &str,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::BaseItemDto, Error> {
        self.request(reqwest::Method::GET, format!("/Persons/{}", encode_path(name)))
            .query_opt("userId", user_id)
            .send()
            .await
    }
}
