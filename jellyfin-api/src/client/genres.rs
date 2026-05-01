use crate::types;
use crate::error::Error;
use crate::util::encode_path;
use crate::Client;

impl Client {
    #[doc = "Gets all genres from a given item, folder, or the entire library\n\nSends a `GET` request to `/Genres`\n\nArguments:\n- `enable_image_types`: Optional. The image types to include in the output.\n- `enable_images`: Optional, include image information in output.\n- `enable_total_record_count`: Optional. Include total record count.\n- `exclude_item_types`: Optional. If specified, results will be filtered out based on item type. This allows multiple, comma delimited.\n- `fields`: Optional. Specify additional fields of information to return in the output.\n- `image_type_limit`: Optional, the max number of images to return, per image type.\n- `include_item_types`: Optional. If specified, results will be filtered in based on item type. This allows multiple, comma delimited.\n- `is_favorite`: Optional filter by items that are marked as favorite, or not.\n- `limit`: Optional. The maximum number of records to return.\n- `name_less_than`: Optional filter by items whose name is equally or lesser than a given input string.\n- `name_starts_with`: Optional filter by items whose name is sorted equally than a given input string.\n- `name_starts_with_or_greater`: Optional filter by items whose name is sorted equally or greater than a given input string.\n- `parent_id`: Specify this to localize the search to a specific item or folder. Omit to use the root.\n- `search_term`: The search term.\n- `sort_by`: Optional. Specify one or more sort orders, comma delimited.\n- `sort_order`: Sort Order - Ascending,Descending.\n- `start_index`: Optional. The record index to start at. All items with a lower index will be dropped from the results.\n- `user_id`: User id.\n"]
    pub async fn get_genres(
        &self,
        enable_image_types: Option<&Vec<types::ImageType>>,
        enable_images: Option<bool>,
        enable_total_record_count: Option<bool>,
        exclude_item_types: Option<&Vec<types::BaseItemKind>>,
        fields: Option<&Vec<types::ItemFields>>,
        image_type_limit: Option<i32>,
        include_item_types: Option<&Vec<types::BaseItemKind>>,
        is_favorite: Option<bool>,
        limit: Option<i32>,
        name_less_than: Option<&str>,
        name_starts_with: Option<&str>,
        name_starts_with_or_greater: Option<&str>,
        parent_id: Option<&uuid::Uuid>,
        search_term: Option<&str>,
        sort_by: Option<&Vec<types::ItemSortBy>>,
        sort_order: Option<&Vec<types::SortOrder>>,
        start_index: Option<i32>,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(reqwest::Method::GET, "/Genres".into())
            .query_list_opt("enableImageTypes", enable_image_types)
            .query_opt("enableImages", enable_images)
            .query_opt("enableTotalRecordCount", enable_total_record_count)
            .query_list_opt("excludeItemTypes", exclude_item_types)
            .query_list_opt("fields", fields)
            .query_opt("imageTypeLimit", image_type_limit)
            .query_list_opt("includeItemTypes", include_item_types)
            .query_opt("isFavorite", is_favorite)
            .query_opt("limit", limit)
            .query_opt("nameLessThan", name_less_than)
            .query_opt("nameStartsWith", name_starts_with)
            .query_opt("nameStartsWithOrGreater", name_starts_with_or_greater)
            .query_opt("parentId", parent_id)
            .query_opt("searchTerm", search_term)
            .query_list_opt("sortBy", sort_by)
            .query_list_opt("sortOrder", sort_order)
            .query_opt("startIndex", start_index)
            .query_opt("userId", user_id)
            .send()
            .await
    }

    #[doc = "Gets a genre, by name\n\nSends a `GET` request to `/Genres/{genreName}`\n\nArguments:\n- `genre_name`: The genre name.\n- `user_id`: The user id.\n"]
    pub async fn get_genre(
        &self,
        genre_name: &str,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::BaseItemDto, Error> {
        self.request(reqwest::Method::GET, format!("/Genres/{}", encode_path(genre_name)))
            .query_opt("userId", user_id)
            .send()
            .await
    }

    #[doc = "Get genre image by name\n\nSends a `GET` request to `/Genres/{name}/Images/{imageType}`\n\nArguments:\n- `name`: Genre name.\n- `image_type`: Image type.\n- `background_color`: Optional. Apply a background color for transparent images.\n- `blur`: Optional. Blur image.\n- `fill_height`: Height of box to fill.\n- `fill_width`: Width of box to fill.\n- `foreground_layer`: Optional. Apply a foreground layer on top of the image.\n- `format`: Determines the output format of the image - original,gif,jpg,png.\n- `height`: The fixed image height to return.\n- `image_index`: Image index.\n- `max_height`: The maximum image height to return.\n- `max_width`: The maximum image width to return.\n- `percent_played`: Optional. Percent to render for the percent played overlay.\n- `quality`: Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases.\n- `tag`: Optional. Supply the cache tag from the item object to receive strong caching headers.\n- `unplayed_count`: Optional. Unplayed count overlay to render.\n- `width`: The fixed image width to return.\n"]
    pub async fn get_genre_image(
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
        self.request(reqwest::Method::GET, format!("/Genres/{}/Images/{}", encode_path(name), encode_path(&image_type.to_string())))
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

    #[doc = "Get genre image by name\n\nSends a `HEAD` request to `/Genres/{name}/Images/{imageType}`\n\nArguments:\n- `name`: Genre name.\n- `image_type`: Image type.\n- `background_color`: Optional. Apply a background color for transparent images.\n- `blur`: Optional. Blur image.\n- `fill_height`: Height of box to fill.\n- `fill_width`: Width of box to fill.\n- `foreground_layer`: Optional. Apply a foreground layer on top of the image.\n- `format`: Determines the output format of the image - original,gif,jpg,png.\n- `height`: The fixed image height to return.\n- `image_index`: Image index.\n- `max_height`: The maximum image height to return.\n- `max_width`: The maximum image width to return.\n- `percent_played`: Optional. Percent to render for the percent played overlay.\n- `quality`: Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases.\n- `tag`: Optional. Supply the cache tag from the item object to receive strong caching headers.\n- `unplayed_count`: Optional. Unplayed count overlay to render.\n- `width`: The fixed image width to return.\n"]
    pub async fn head_genre_image(
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
        self.request(reqwest::Method::HEAD, format!("/Genres/{}/Images/{}", encode_path(name), encode_path(&image_type.to_string())))
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

    #[doc = "Get genre image by name\n\nSends a `GET` request to `/Genres/{name}/Images/{imageType}/{imageIndex}`\n\nArguments:\n- `name`: Genre name.\n- `image_type`: Image type.\n- `image_index`: Image index.\n- `background_color`: Optional. Apply a background color for transparent images.\n- `blur`: Optional. Blur image.\n- `fill_height`: Height of box to fill.\n- `fill_width`: Width of box to fill.\n- `foreground_layer`: Optional. Apply a foreground layer on top of the image.\n- `format`: Determines the output format of the image - original,gif,jpg,png.\n- `height`: The fixed image height to return.\n- `max_height`: The maximum image height to return.\n- `max_width`: The maximum image width to return.\n- `percent_played`: Optional. Percent to render for the percent played overlay.\n- `quality`: Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases.\n- `tag`: Optional. Supply the cache tag from the item object to receive strong caching headers.\n- `unplayed_count`: Optional. Unplayed count overlay to render.\n- `width`: The fixed image width to return.\n"]
    pub async fn get_genre_image_by_index(
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
        self.request(reqwest::Method::GET, format!("/Genres/{}/Images/{}/{}", encode_path(name), encode_path(&image_type.to_string()), encode_path(&image_index.to_string())))
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

    #[doc = "Get genre image by name\n\nSends a `HEAD` request to `/Genres/{name}/Images/{imageType}/{imageIndex}`\n\nArguments:\n- `name`: Genre name.\n- `image_type`: Image type.\n- `image_index`: Image index.\n- `background_color`: Optional. Apply a background color for transparent images.\n- `blur`: Optional. Blur image.\n- `fill_height`: Height of box to fill.\n- `fill_width`: Width of box to fill.\n- `foreground_layer`: Optional. Apply a foreground layer on top of the image.\n- `format`: Determines the output format of the image - original,gif,jpg,png.\n- `height`: The fixed image height to return.\n- `max_height`: The maximum image height to return.\n- `max_width`: The maximum image width to return.\n- `percent_played`: Optional. Percent to render for the percent played overlay.\n- `quality`: Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases.\n- `tag`: Optional. Supply the cache tag from the item object to receive strong caching headers.\n- `unplayed_count`: Optional. Unplayed count overlay to render.\n- `width`: The fixed image width to return.\n"]
    pub async fn head_genre_image_by_index(
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
        self.request(reqwest::Method::HEAD, format!("/Genres/{}/Images/{}/{}", encode_path(name), encode_path(&image_type.to_string()), encode_path(&image_index.to_string())))
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
}
