use crate::Client;
use crate::error::Error;
use crate::query;
use crate::types;
use crate::util::encode_path;

impl Client {
    #[doc = "Gets legacy query filters\n\nSends a `GET` request to `/Items/Filters`\n\nArguments:\n- `include_item_types`: Optional. If specified, results will be filtered based on item type. This allows multiple, comma delimited.\n- `media_types`: Optional. Filter by MediaType. Allows multiple, comma delimited.\n- `parent_id`: Optional. Parent id.\n- `user_id`: Optional. User id.\n"]
    pub async fn get_query_filters_legacy(
        &self,
        include_item_types: Option<&Vec<types::BaseItemKind>>,
        media_types: Option<&Vec<types::MediaType>>,
        parent_id: Option<&uuid::Uuid>,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::QueryFiltersLegacy, Error> {
        self.request(reqwest::Method::GET, "/Items/Filters".into())
            .query_list_opt("includeItemTypes", include_item_types)
            .query_list_opt("mediaTypes", media_types)
            .query_opt("parentId", parent_id)
            .query_opt("userId", user_id)
            .send()
            .await
    }

    #[doc = "Gets query filters\n\nSends a `GET` request to `/Items/Filters2`\n\nArguments:\n- `include_item_types`: Optional. If specified, results will be filtered based on item type. This allows multiple, comma delimited.\n- `is_airing`: Optional. Is item airing.\n- `is_kids`: Optional. Is item kids.\n- `is_movie`: Optional. Is item movie.\n- `is_news`: Optional. Is item news.\n- `is_series`: Optional. Is item series.\n- `is_sports`: Optional. Is item sports.\n- `parent_id`: Optional. Specify this to localize the search to a specific item or folder. Omit to use the root.\n- `recursive`: Optional. Search recursive.\n- `user_id`: Optional. User id.\n"]
    pub async fn get_query_filters(
        &self,
        query: &query::GetQueryFilters<'_>,
    ) -> Result<types::QueryFilters, Error> {
        self.request(reqwest::Method::GET, "/Items/Filters2".into())
            .query_list_opt("includeItemTypes", query.include_item_types)
            .query_opt("isAiring", query.is_airing)
            .query_opt("isKids", query.is_kids)
            .query_opt("isMovie", query.is_movie)
            .query_opt("isNews", query.is_news)
            .query_opt("isSeries", query.is_series)
            .query_opt("isSports", query.is_sports)
            .query_opt("parentId", query.parent_id)
            .query_opt("recursive", query.recursive)
            .query_opt("userId", query.user_id)
            .send()
            .await
    }

    #[doc = "Get item image infos\n\nSends a `GET` request to `/Items/{itemId}/Images`\n\nArguments:\n- `item_id`: Item id.\n"]
    pub async fn get_item_image_infos(
        &self,
        item_id: &uuid::Uuid,
    ) -> Result<Vec<types::ImageInfo>, Error> {
        self.request(
            reqwest::Method::GET,
            format!("/Items/{}/Images", encode_path(&item_id.to_string())),
        )
        .send()
        .await
    }

    #[doc = "Gets the item's image\n\nSends a `GET` request to `/Items/{itemId}/Images/{imageType}`\n\nArguments:\n- `item_id`: Item id.\n- `image_type`: Image type.\n- `background_color`: Optional. Apply a background color for transparent images.\n- `blur`: Optional. Blur image.\n- `fill_height`: Height of box to fill.\n- `fill_width`: Width of box to fill.\n- `foreground_layer`: Optional. Apply a foreground layer on top of the image.\n- `format`: Optional. The MediaBrowser.Model.Drawing.ImageFormat of the returned image.\n- `height`: The fixed image height to return.\n- `image_index`: Image index.\n- `max_height`: The maximum image height to return.\n- `max_width`: The maximum image width to return.\n- `percent_played`: Optional. Percent to render for the percent played overlay.\n- `quality`: Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases.\n- `tag`: Optional. Supply the cache tag from the item object to receive strong caching headers.\n- `unplayed_count`: Optional. Unplayed count overlay to render.\n- `width`: The fixed image width to return.\n"]
    pub async fn get_item_image(
        &self,
        item_id: &uuid::Uuid,
        image_type: types::ImageType,
        query: &query::GetItemImage<'_>,
    ) -> Result<reqwest::Response, Error> {
        self.request(
            reqwest::Method::GET,
            format!(
                "/Items/{}/Images/{}",
                encode_path(&item_id.to_string()),
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

    #[doc = "Set item image\n\nSends a `POST` request to `/Items/{itemId}/Images/{imageType}`\n\nArguments:\n- `item_id`: Item id.\n- `image_type`: Image type.\n- `body`\n"]
    pub async fn set_item_image<B: Into<reqwest::Body>>(
        &self,
        item_id: &uuid::Uuid,
        image_type: types::ImageType,
        body: B,
    ) -> Result<(), Error> {
        self.request(
            reqwest::Method::POST,
            format!(
                "/Items/{}/Images/{}",
                encode_path(&item_id.to_string()),
                encode_path(&image_type.to_string())
            ),
        )
        .raw_body(body, "application/octet-stream")
        .send_no_content()
        .await
    }

    #[doc = "Delete an item's image\n\nSends a `DELETE` request to `/Items/{itemId}/Images/{imageType}`\n\nArguments:\n- `item_id`: Item id.\n- `image_type`: Image type.\n- `image_index`: The image index.\n"]
    pub async fn delete_item_image(
        &self,
        item_id: &uuid::Uuid,
        image_type: types::ImageType,
        image_index: Option<i32>,
    ) -> Result<(), Error> {
        self.request(
            reqwest::Method::DELETE,
            format!(
                "/Items/{}/Images/{}",
                encode_path(&item_id.to_string()),
                encode_path(&image_type.to_string())
            ),
        )
        .query_opt("imageIndex", image_index)
        .send_no_content()
        .await
    }

    #[doc = "Gets the item's image\n\nSends a `HEAD` request to `/Items/{itemId}/Images/{imageType}`\n\nArguments:\n- `item_id`: Item id.\n- `image_type`: Image type.\n- `background_color`: Optional. Apply a background color for transparent images.\n- `blur`: Optional. Blur image.\n- `fill_height`: Height of box to fill.\n- `fill_width`: Width of box to fill.\n- `foreground_layer`: Optional. Apply a foreground layer on top of the image.\n- `format`: Optional. The MediaBrowser.Model.Drawing.ImageFormat of the returned image.\n- `height`: The fixed image height to return.\n- `image_index`: Image index.\n- `max_height`: The maximum image height to return.\n- `max_width`: The maximum image width to return.\n- `percent_played`: Optional. Percent to render for the percent played overlay.\n- `quality`: Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases.\n- `tag`: Optional. Supply the cache tag from the item object to receive strong caching headers.\n- `unplayed_count`: Optional. Unplayed count overlay to render.\n- `width`: The fixed image width to return.\n"]
    pub async fn head_item_image(
        &self,
        item_id: &uuid::Uuid,
        image_type: types::ImageType,
        query: &query::HeadItemImage<'_>,
    ) -> Result<reqwest::Response, Error> {
        self.request(
            reqwest::Method::HEAD,
            format!(
                "/Items/{}/Images/{}",
                encode_path(&item_id.to_string()),
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

    #[doc = "Gets the item's image\n\nSends a `GET` request to `/Items/{itemId}/Images/{imageType}/{imageIndex}`\n\nArguments:\n- `item_id`: Item id.\n- `image_type`: Image type.\n- `image_index`: Image index.\n- `background_color`: Optional. Apply a background color for transparent images.\n- `blur`: Optional. Blur image.\n- `fill_height`: Height of box to fill.\n- `fill_width`: Width of box to fill.\n- `foreground_layer`: Optional. Apply a foreground layer on top of the image.\n- `format`: Optional. The MediaBrowser.Model.Drawing.ImageFormat of the returned image.\n- `height`: The fixed image height to return.\n- `max_height`: The maximum image height to return.\n- `max_width`: The maximum image width to return.\n- `percent_played`: Optional. Percent to render for the percent played overlay.\n- `quality`: Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases.\n- `tag`: Optional. Supply the cache tag from the item object to receive strong caching headers.\n- `unplayed_count`: Optional. Unplayed count overlay to render.\n- `width`: The fixed image width to return.\n"]
    pub async fn get_item_image_by_index(
        &self,
        item_id: &uuid::Uuid,
        image_type: types::ImageType,
        image_index: i32,
        query: &query::GetItemImageByIndex<'_>,
    ) -> Result<reqwest::Response, Error> {
        self.request(
            reqwest::Method::GET,
            format!(
                "/Items/{}/Images/{}/{}",
                encode_path(&item_id.to_string()),
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

    #[doc = "Set item image\n\nSends a `POST` request to `/Items/{itemId}/Images/{imageType}/{imageIndex}`\n\nArguments:\n- `item_id`: Item id.\n- `image_type`: Image type.\n- `image_index`: (Unused) Image index.\n- `body`\n"]
    pub async fn set_item_image_by_index<B: Into<reqwest::Body>>(
        &self,
        item_id: &uuid::Uuid,
        image_type: types::ImageType,
        image_index: i32,
        body: B,
    ) -> Result<(), Error> {
        self.request(
            reqwest::Method::POST,
            format!(
                "/Items/{}/Images/{}/{}",
                encode_path(&item_id.to_string()),
                encode_path(&image_type.to_string()),
                encode_path(&image_index.to_string())
            ),
        )
        .raw_body(body, "application/octet-stream")
        .send_no_content()
        .await
    }

    #[doc = "Delete an item's image\n\nSends a `DELETE` request to `/Items/{itemId}/Images/{imageType}/{imageIndex}`\n\nArguments:\n- `item_id`: Item id.\n- `image_type`: Image type.\n- `image_index`: The image index.\n"]
    pub async fn delete_item_image_by_index(
        &self,
        item_id: &uuid::Uuid,
        image_type: types::ImageType,
        image_index: i32,
    ) -> Result<(), Error> {
        self.request(
            reqwest::Method::DELETE,
            format!(
                "/Items/{}/Images/{}/{}",
                encode_path(&item_id.to_string()),
                encode_path(&image_type.to_string()),
                encode_path(&image_index.to_string())
            ),
        )
        .send_no_content()
        .await
    }

    #[doc = "Gets the item's image\n\nSends a `HEAD` request to `/Items/{itemId}/Images/{imageType}/{imageIndex}`\n\nArguments:\n- `item_id`: Item id.\n- `image_type`: Image type.\n- `image_index`: Image index.\n- `background_color`: Optional. Apply a background color for transparent images.\n- `blur`: Optional. Blur image.\n- `fill_height`: Height of box to fill.\n- `fill_width`: Width of box to fill.\n- `foreground_layer`: Optional. Apply a foreground layer on top of the image.\n- `format`: Optional. The MediaBrowser.Model.Drawing.ImageFormat of the returned image.\n- `height`: The fixed image height to return.\n- `max_height`: The maximum image height to return.\n- `max_width`: The maximum image width to return.\n- `percent_played`: Optional. Percent to render for the percent played overlay.\n- `quality`: Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases.\n- `tag`: Optional. Supply the cache tag from the item object to receive strong caching headers.\n- `unplayed_count`: Optional. Unplayed count overlay to render.\n- `width`: The fixed image width to return.\n"]
    pub async fn head_item_image_by_index(
        &self,
        item_id: &uuid::Uuid,
        image_type: types::ImageType,
        image_index: i32,
        query: &query::HeadItemImageByIndex<'_>,
    ) -> Result<reqwest::Response, Error> {
        self.request(
            reqwest::Method::HEAD,
            format!(
                "/Items/{}/Images/{}/{}",
                encode_path(&item_id.to_string()),
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

    #[doc = "Gets the item's image\n\nSends a `GET` request to `/Items/{itemId}/Images/{imageType}/{imageIndex}/{tag}/{format}/{maxWidth}/{maxHeight}/{percentPlayed}/{unplayedCount}`\n\nArguments:\n- `item_id`: Item id.\n- `image_type`: Image type.\n- `image_index`: Image index.\n- `tag`: Optional. Supply the cache tag from the item object to receive strong caching headers.\n- `format`: Determines the output format of the image - original,gif,jpg,png.\n- `max_width`: The maximum image width to return.\n- `max_height`: The maximum image height to return.\n- `percent_played`: Optional. Percent to render for the percent played overlay.\n- `unplayed_count`: Optional. Unplayed count overlay to render.\n- `background_color`: Optional. Apply a background color for transparent images.\n- `blur`: Optional. Blur image.\n- `fill_height`: Height of box to fill.\n- `fill_width`: Width of box to fill.\n- `foreground_layer`: Optional. Apply a foreground layer on top of the image.\n- `height`: The fixed image height to return.\n- `quality`: Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases.\n- `width`: The fixed image width to return.\n"]
    pub async fn get_item_image_sized(
        &self,
        asked: &query::GetItemImageSized<'_>,
    ) -> Result<reqwest::Response, Error> {
        self.request(
            reqwest::Method::GET,
            format!(
                "/Items/{}/Images/{}/{}/{}/{}/{}/{}/{}/{}",
                encode_path(&asked.item_id.to_string()),
                encode_path(&asked.image_type.to_string()),
                encode_path(&asked.image_index.to_string()),
                encode_path(asked.tag),
                encode_path(&asked.format.to_string()),
                encode_path(&asked.max_width.to_string()),
                encode_path(&asked.max_height.to_string()),
                encode_path(&asked.percent_played.to_string()),
                encode_path(&asked.unplayed_count.to_string())
            ),
        )
        .query_opt("backgroundColor", asked.background_color)
        .query_opt("blur", asked.blur)
        .query_opt("fillHeight", asked.fill_height)
        .query_opt("fillWidth", asked.fill_width)
        .query_opt("foregroundLayer", asked.foreground_layer)
        .query_opt("height", asked.height)
        .query_opt("quality", asked.quality)
        .query_opt("width", asked.width)
        .send_response()
        .await
    }

    #[doc = "Gets the item's image\n\nSends a `HEAD` request to `/Items/{itemId}/Images/{imageType}/{imageIndex}/{tag}/{format}/{maxWidth}/{maxHeight}/{percentPlayed}/{unplayedCount}`\n\nArguments:\n- `item_id`: Item id.\n- `image_type`: Image type.\n- `image_index`: Image index.\n- `tag`: Optional. Supply the cache tag from the item object to receive strong caching headers.\n- `format`: Determines the output format of the image - original,gif,jpg,png.\n- `max_width`: The maximum image width to return.\n- `max_height`: The maximum image height to return.\n- `percent_played`: Optional. Percent to render for the percent played overlay.\n- `unplayed_count`: Optional. Unplayed count overlay to render.\n- `background_color`: Optional. Apply a background color for transparent images.\n- `blur`: Optional. Blur image.\n- `fill_height`: Height of box to fill.\n- `fill_width`: Width of box to fill.\n- `foreground_layer`: Optional. Apply a foreground layer on top of the image.\n- `height`: The fixed image height to return.\n- `quality`: Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases.\n- `width`: The fixed image width to return.\n"]
    pub async fn head_item_image_sized(
        &self,
        asked: &query::HeadItemImageSized<'_>,
    ) -> Result<reqwest::Response, Error> {
        self.request(
            reqwest::Method::HEAD,
            format!(
                "/Items/{}/Images/{}/{}/{}/{}/{}/{}/{}/{}",
                encode_path(&asked.item_id.to_string()),
                encode_path(&asked.image_type.to_string()),
                encode_path(&asked.image_index.to_string()),
                encode_path(asked.tag),
                encode_path(&asked.format.to_string()),
                encode_path(&asked.max_width.to_string()),
                encode_path(&asked.max_height.to_string()),
                encode_path(&asked.percent_played.to_string()),
                encode_path(&asked.unplayed_count.to_string())
            ),
        )
        .query_opt("backgroundColor", asked.background_color)
        .query_opt("blur", asked.blur)
        .query_opt("fillHeight", asked.fill_height)
        .query_opt("fillWidth", asked.fill_width)
        .query_opt("foregroundLayer", asked.foreground_layer)
        .query_opt("height", asked.height)
        .query_opt("quality", asked.quality)
        .query_opt("width", asked.width)
        .send_response()
        .await
    }

    #[doc = "Updates the index for an item image\n\nSends a `POST` request to `/Items/{itemId}/Images/{imageType}/{imageIndex}/Index`\n\nArguments:\n- `item_id`: Item id.\n- `image_type`: Image type.\n- `image_index`: Old image index.\n- `new_index`: New image index.\n"]
    pub async fn update_item_image_index(
        &self,
        item_id: &uuid::Uuid,
        image_type: types::ImageType,
        image_index: i32,
        new_index: i32,
    ) -> Result<(), Error> {
        self.request(
            reqwest::Method::POST,
            format!(
                "/Items/{}/Images/{}/{}/Index",
                encode_path(&item_id.to_string()),
                encode_path(&image_type.to_string()),
                encode_path(&image_index.to_string())
            ),
        )
        .query("newIndex", new_index)
        .send_no_content()
        .await
    }

    #[doc = "Creates an instant playlist based on a given item\n\nSends a `GET` request to `/Items/{itemId}/InstantMix`\n\nArguments:\n- `item_id`: The item id.\n- `enable_image_types`: Optional. The image types to include in the output.\n- `enable_images`: Optional. Include image information in output.\n- `enable_user_data`: Optional. Include user data.\n- `fields`: Optional. Specify additional fields of information to return in the output.\n- `image_type_limit`: Optional. The max number of images to return, per image type.\n- `limit`: Optional. The maximum number of records to return.\n- `user_id`: Optional. Filter by user id, and attach user data.\n"]
    pub async fn get_instant_mix_from_item(
        &self,
        item_id: &uuid::Uuid,
        query: &query::GetInstantMixFromItem<'_>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(
            reqwest::Method::GET,
            format!("/Items/{}/InstantMix", encode_path(&item_id.to_string())),
        )
        .query_list_opt("enableImageTypes", query.enable_image_types)
        .query_opt("enableImages", query.enable_images)
        .query_opt("enableUserData", query.enable_user_data)
        .query_list_opt("fields", query.fields)
        .query_opt("imageTypeLimit", query.image_type_limit)
        .query_opt("limit", query.limit)
        .query_opt("userId", query.user_id)
        .send()
        .await
    }

    #[doc = "Get the item's external id info\n\nSends a `GET` request to `/Items/{itemId}/ExternalIdInfos`\n\nArguments:\n- `item_id`: Item id.\n"]
    pub async fn get_external_id_infos(
        &self,
        item_id: &uuid::Uuid,
    ) -> Result<Vec<types::ExternalIdInfo>, Error> {
        self.request(
            reqwest::Method::GET,
            format!(
                "/Items/{}/ExternalIdInfos",
                encode_path(&item_id.to_string())
            ),
        )
        .send()
        .await
    }

    #[doc = "Applies search criteria to an item and refreshes metadata\n\nSends a `POST` request to `/Items/RemoteSearch/Apply/{itemId}`\n\nArguments:\n- `item_id`: Item id.\n- `replace_all_images`: Optional. Whether or not to replace all images. Default: True.\n- `body`: The remote search result.\n"]
    pub async fn apply_search_criteria(
        &self,
        item_id: &uuid::Uuid,
        replace_all_images: Option<bool>,
        body: &types::RemoteSearchResult,
    ) -> Result<(), Error> {
        self.request(
            reqwest::Method::POST,
            format!(
                "/Items/RemoteSearch/Apply/{}",
                encode_path(&item_id.to_string())
            ),
        )
        .query_opt("replaceAllImages", replace_all_images)
        .json_body(body)
        .send_no_content()
        .await
    }

    #[doc = "Get book remote search\n\nSends a `POST` request to `/Items/RemoteSearch/Book`\n\nArguments:\n- `body`: Remote search query.\n"]
    pub async fn get_book_remote_search_results(
        &self,
        body: &types::BookInfoRemoteSearchQuery,
    ) -> Result<Vec<types::RemoteSearchResult>, Error> {
        self.request(reqwest::Method::POST, "/Items/RemoteSearch/Book".into())
            .json_body(body)
            .send()
            .await
    }

    #[doc = "Get box set remote search\n\nSends a `POST` request to `/Items/RemoteSearch/BoxSet`\n\nArguments:\n- `body`: Remote search query.\n"]
    pub async fn get_box_set_remote_search_results(
        &self,
        body: &types::BoxSetInfoRemoteSearchQuery,
    ) -> Result<Vec<types::RemoteSearchResult>, Error> {
        self.request(reqwest::Method::POST, "/Items/RemoteSearch/BoxSet".into())
            .json_body(body)
            .send()
            .await
    }

    #[doc = "Get movie remote search\n\nSends a `POST` request to `/Items/RemoteSearch/Movie`\n\nArguments:\n- `body`: Remote search query.\n"]
    pub async fn get_movie_remote_search_results(
        &self,
        body: &types::MovieInfoRemoteSearchQuery,
    ) -> Result<Vec<types::RemoteSearchResult>, Error> {
        self.request(reqwest::Method::POST, "/Items/RemoteSearch/Movie".into())
            .json_body(body)
            .send()
            .await
    }

    #[doc = "Get music album remote search\n\nSends a `POST` request to `/Items/RemoteSearch/MusicAlbum`\n\nArguments:\n- `body`: Remote search query.\n"]
    pub async fn get_music_album_remote_search_results(
        &self,
        body: &types::AlbumInfoRemoteSearchQuery,
    ) -> Result<Vec<types::RemoteSearchResult>, Error> {
        self.request(
            reqwest::Method::POST,
            "/Items/RemoteSearch/MusicAlbum".into(),
        )
        .json_body(body)
        .send()
        .await
    }

    #[doc = "Get music artist remote search\n\nSends a `POST` request to `/Items/RemoteSearch/MusicArtist`\n\nArguments:\n- `body`: Remote search query.\n"]
    pub async fn get_music_artist_remote_search_results(
        &self,
        body: &types::ArtistInfoRemoteSearchQuery,
    ) -> Result<Vec<types::RemoteSearchResult>, Error> {
        self.request(
            reqwest::Method::POST,
            "/Items/RemoteSearch/MusicArtist".into(),
        )
        .json_body(body)
        .send()
        .await
    }

    #[doc = "Get music video remote search\n\nSends a `POST` request to `/Items/RemoteSearch/MusicVideo`\n\nArguments:\n- `body`: Remote search query.\n"]
    pub async fn get_music_video_remote_search_results(
        &self,
        body: &types::MusicVideoInfoRemoteSearchQuery,
    ) -> Result<Vec<types::RemoteSearchResult>, Error> {
        self.request(
            reqwest::Method::POST,
            "/Items/RemoteSearch/MusicVideo".into(),
        )
        .json_body(body)
        .send()
        .await
    }

    #[doc = "Get person remote search\n\nSends a `POST` request to `/Items/RemoteSearch/Person`\n\nArguments:\n- `body`: Remote search query.\n"]
    pub async fn get_person_remote_search_results(
        &self,
        body: &types::PersonLookupInfoRemoteSearchQuery,
    ) -> Result<Vec<types::RemoteSearchResult>, Error> {
        self.request(reqwest::Method::POST, "/Items/RemoteSearch/Person".into())
            .json_body(body)
            .send()
            .await
    }

    #[doc = "Get series remote search\n\nSends a `POST` request to `/Items/RemoteSearch/Series`\n\nArguments:\n- `body`: Remote search query.\n"]
    pub async fn get_series_remote_search_results(
        &self,
        body: &types::SeriesInfoRemoteSearchQuery,
    ) -> Result<Vec<types::RemoteSearchResult>, Error> {
        self.request(reqwest::Method::POST, "/Items/RemoteSearch/Series".into())
            .json_body(body)
            .send()
            .await
    }

    #[doc = "Get trailer remote search\n\nSends a `POST` request to `/Items/RemoteSearch/Trailer`\n\nArguments:\n- `body`: Remote search query.\n"]
    pub async fn get_trailer_remote_search_results(
        &self,
        body: &types::TrailerInfoRemoteSearchQuery,
    ) -> Result<Vec<types::RemoteSearchResult>, Error> {
        self.request(reqwest::Method::POST, "/Items/RemoteSearch/Trailer".into())
            .json_body(body)
            .send()
            .await
    }

    #[doc = "Refreshes metadata for an item\n\nSends a `POST` request to `/Items/{itemId}/Refresh`\n\nArguments:\n- `item_id`: Item id.\n- `image_refresh_mode`: (Optional) Specifies the image refresh mode.\n- `metadata_refresh_mode`: (Optional) Specifies the metadata refresh mode.\n- `regenerate_trickplay`: (Optional) Determines if trickplay images should be replaced. Only applicable if mode is FullRefresh.\n- `replace_all_images`: (Optional) Determines if images should be replaced. Only applicable if mode is FullRefresh.\n- `replace_all_metadata`: (Optional) Determines if metadata should be replaced. Only applicable if mode is FullRefresh.\n"]
    pub async fn refresh_item(
        &self,
        item_id: &uuid::Uuid,
        image_refresh_mode: Option<types::MetadataRefreshMode>,
        metadata_refresh_mode: Option<types::MetadataRefreshMode>,
        regenerate_trickplay: Option<bool>,
        replace_all_images: Option<bool>,
        replace_all_metadata: Option<bool>,
    ) -> Result<(), Error> {
        self.request(
            reqwest::Method::POST,
            format!("/Items/{}/Refresh", encode_path(&item_id.to_string())),
        )
        .query_opt("imageRefreshMode", image_refresh_mode)
        .query_opt("metadataRefreshMode", metadata_refresh_mode)
        .query_opt("regenerateTrickplay", regenerate_trickplay)
        .query_opt("replaceAllImages", replace_all_images)
        .query_opt("replaceAllMetadata", replace_all_metadata)
        .send_no_content()
        .await
    }

    #[doc = "Gets items based on a query\n\nSends a `GET` request to `/Items`\n\nArguments:\n- `adjacent_to`: Optional. Return items that are siblings of a supplied item.\n- `album_artist_ids`: Optional. If specified, results will be filtered to include only those containing the specified album artist id.\n- `album_ids`: Optional. If specified, results will be filtered based on album id. This allows multiple, pipe delimited.\n- `albums`: Optional. If specified, results will be filtered based on album. This allows multiple, pipe delimited.\n- `artist_ids`: Optional. If specified, results will be filtered to include only those containing the specified artist id.\n- `artists`: Optional. If specified, results will be filtered based on artists. This allows multiple, pipe delimited.\n- `collapse_box_set_items`: Whether or not to hide items behind their boxsets.\n- `contributing_artist_ids`: Optional. If specified, results will be filtered to include only those containing the specified contributing artist id.\n- `enable_image_types`: Optional. The image types to include in the output.\n- `enable_images`: Optional, include image information in output.\n- `enable_total_record_count`: Optional. Enable the total record count.\n- `enable_user_data`: Optional, include user data.\n- `exclude_artist_ids`: Optional. If specified, results will be filtered based on artist id. This allows multiple, pipe delimited.\n- `exclude_item_ids`: Optional. If specified, results will be filtered by excluding item ids. This allows multiple, comma delimited.\n- `exclude_item_types`: Optional. If specified, results will be filtered based on item type. This allows multiple, comma delimited.\n- `exclude_location_types`: Optional. If specified, results will be filtered based on the LocationType. This allows multiple, comma delimited.\n- `fields`: Optional. Specify additional fields of information to return in the output. This allows multiple, comma delimited. Options: Budget, Chapters, DateCreated, Genres, HomePageUrl, IndexOptions, MediaStreams, Overview, ParentId, Path, People, ProviderIds, PrimaryImageAspectRatio, Revenue, SortName, Studios, Taglines.\n- `filters`: Optional. Specify additional filters to apply. This allows multiple, comma delimited. Options: IsFolder, IsNotFolder, IsUnplayed, IsPlayed, IsFavorite, IsResumable, Likes, Dislikes.\n- `genre_ids`: Optional. If specified, results will be filtered based on genre id. This allows multiple, pipe delimited.\n- `genres`: Optional. If specified, results will be filtered based on genre. This allows multiple, pipe delimited.\n- `has_imdb_id`: Optional filter by items that have an IMDb id or not.\n- `has_official_rating`: Optional filter by items that have official ratings.\n- `has_overview`: Optional filter by items that have an overview or not.\n- `has_parental_rating`: Optional filter by items that have or do not have a parental rating.\n- `has_special_feature`: Optional filter by items with special features.\n- `has_subtitles`: Optional filter by items with subtitles.\n- `has_theme_song`: Optional filter by items with theme songs.\n- `has_theme_video`: Optional filter by items with theme videos.\n- `has_tmdb_id`: Optional filter by items that have a TMDb id or not.\n- `has_trailer`: Optional filter by items with trailers.\n- `has_tvdb_id`: Optional filter by items that have a TVDb id or not.\n- `ids`: Optional. If specific items are needed, specify a list of item id's to retrieve. This allows multiple, comma delimited.\n- `image_type_limit`: Optional, the max number of images to return, per image type.\n- `image_types`: Optional. If specified, results will be filtered based on those containing image types. This allows multiple, comma delimited.\n- `include_item_types`: Optional. If specified, results will be filtered based on the item type. This allows multiple, comma delimited.\n- `index_number`: Optional filter by index number.\n- `is3_d`: Optional filter by items that are 3D, or not.\n- `is4_k`: Optional filter by items that are 4K or not.\n- `is_favorite`: Optional filter by items that are marked as favorite, or not.\n- `is_hd`: Optional filter by items that are HD or not.\n- `is_kids`: Optional filter for live tv kids.\n- `is_locked`: Optional filter by items that are locked.\n- `is_missing`: Optional filter by items that are missing episodes or not.\n- `is_movie`: Optional filter for live tv movies.\n- `is_news`: Optional filter for live tv news.\n- `is_place_holder`: Optional filter by items that are placeholders.\n- `is_played`: Optional filter by items that are played, or not.\n- `is_series`: Optional filter for live tv series.\n- `is_sports`: Optional filter for live tv sports.\n- `is_unaired`: Optional filter by items that are unaired episodes or not.\n- `limit`: Optional. The maximum number of records to return.\n- `location_types`: Optional. If specified, results will be filtered based on LocationType. This allows multiple, comma delimited.\n- `max_height`: Optional. Filter by the maximum height of the item.\n- `max_official_rating`: Optional filter by maximum official rating (PG, PG-13, TV-MA, etc).\n- `max_premiere_date`: Optional. The maximum premiere date. Format = ISO.\n- `max_width`: Optional. Filter by the maximum width of the item.\n- `media_types`: Optional filter by MediaType. Allows multiple, comma delimited.\n- `min_community_rating`: Optional filter by minimum community rating.\n- `min_critic_rating`: Optional filter by minimum critic rating.\n- `min_date_last_saved`: Optional. The minimum last saved date. Format = ISO.\n- `min_date_last_saved_for_user`: Optional. The minimum last saved date for the current user. Format = ISO.\n- `min_height`: Optional. Filter by the minimum height of the item.\n- `min_official_rating`: Optional filter by minimum official rating (PG, PG-13, TV-MA, etc).\n- `min_premiere_date`: Optional. The minimum premiere date. Format = ISO.\n- `min_width`: Optional. Filter by the minimum width of the item.\n- `name_less_than`: Optional filter by items whose name is equally or lesser than a given input string.\n- `name_starts_with`: Optional filter by items whose name is sorted equally than a given input string.\n- `name_starts_with_or_greater`: Optional filter by items whose name is sorted equally or greater than a given input string.\n- `official_ratings`: Optional. If specified, results will be filtered based on OfficialRating. This allows multiple, pipe delimited.\n- `parent_id`: Specify this to localize the search to a specific item or folder. Omit to use the root.\n- `parent_index_number`: Optional filter by parent index number.\n- `person`: Optional. If specified, results will be filtered to include only those containing the specified person.\n- `person_ids`: Optional. If specified, results will be filtered to include only those containing the specified person id.\n- `person_types`: Optional. If specified, along with Person, results will be filtered to include only those containing the specified person and PersonType. Allows multiple, comma-delimited.\n- `recursive`: When searching within folders, this determines whether or not the search will be recursive. true/false.\n- `search_term`: Optional. Filter based on a search term.\n- `series_status`: Optional filter by Series Status. Allows multiple, comma delimited.\n- `sort_by`: Optional. Specify one or more sort orders, comma delimited. Options: Album, AlbumArtist, Artist, Budget, CommunityRating, CriticRating, DateCreated, DatePlayed, PlayCount, PremiereDate, ProductionYear, SortName, Random, Revenue, Runtime.\n- `sort_order`: Sort Order - Ascending, Descending.\n- `start_index`: Optional. The record index to start at. All items with a lower index will be dropped from the results.\n- `studio_ids`: Optional. If specified, results will be filtered based on studio id. This allows multiple, pipe delimited.\n- `studios`: Optional. If specified, results will be filtered based on studio. This allows multiple, pipe delimited.\n- `tags`: Optional. If specified, results will be filtered based on tag. This allows multiple, pipe delimited.\n- `user_id`: The user id supplied as query parameter; this is required when not using an API key.\n- `video_types`: Optional filter by VideoType (videofile, dvd, bluray, iso). Allows multiple, comma delimited.\n- `years`: Optional. If specified, results will be filtered based on production year. This allows multiple, comma delimited.\n"]
    pub async fn get_items(
        &self,
        query: &query::GetItems<'_>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(reqwest::Method::GET, "/Items".into())
            .query_opt("adjacentTo", query.adjacent_to)
            .query_list_opt("albumArtistIds", query.album_artist_ids)
            .query_list_opt("albumIds", query.album_ids)
            .query_list_opt("albums", query.albums)
            .query_list_opt("artistIds", query.artist_ids)
            .query_list_opt("artists", query.artists)
            .query_opt("collapseBoxSetItems", query.collapse_box_set_items)
            .query_list_opt("contributingArtistIds", query.contributing_artist_ids)
            .query_list_opt("enableImageTypes", query.enable_image_types)
            .query_opt("enableImages", query.enable_images)
            .query_opt("enableTotalRecordCount", query.enable_total_record_count)
            .query_opt("enableUserData", query.enable_user_data)
            .query_list_opt("excludeArtistIds", query.exclude_artist_ids)
            .query_list_opt("excludeItemIds", query.exclude_item_ids)
            .query_list_opt("excludeItemTypes", query.exclude_item_types)
            .query_list_opt("excludeLocationTypes", query.exclude_location_types)
            .query_list_opt("fields", query.fields)
            .query_list_opt("filters", query.filters)
            .query_list_opt("genreIds", query.genre_ids)
            .query_list_opt("genres", query.genres)
            .query_opt("hasImdbId", query.has_imdb_id)
            .query_opt("hasOfficialRating", query.has_official_rating)
            .query_opt("hasOverview", query.has_overview)
            .query_opt("hasParentalRating", query.has_parental_rating)
            .query_opt("hasSpecialFeature", query.has_special_feature)
            .query_opt("hasSubtitles", query.has_subtitles)
            .query_opt("hasThemeSong", query.has_theme_song)
            .query_opt("hasThemeVideo", query.has_theme_video)
            .query_opt("hasTmdbId", query.has_tmdb_id)
            .query_opt("hasTrailer", query.has_trailer)
            .query_opt("hasTvdbId", query.has_tvdb_id)
            .query_list_opt("ids", query.ids)
            .query_opt("imageTypeLimit", query.image_type_limit)
            .query_list_opt("imageTypes", query.image_types)
            .query_list_opt("includeItemTypes", query.include_item_types)
            .query_opt("indexNumber", query.index_number)
            .query_opt("is3D", query.is_3d)
            .query_opt("is4K", query.is_4k)
            .query_opt("isFavorite", query.is_favorite)
            .query_opt("isHd", query.is_hd)
            .query_opt("isKids", query.is_kids)
            .query_opt("isLocked", query.is_locked)
            .query_opt("isMissing", query.is_missing)
            .query_opt("isMovie", query.is_movie)
            .query_opt("isNews", query.is_news)
            .query_opt("isPlaceHolder", query.is_place_holder)
            .query_opt("isPlayed", query.is_played)
            .query_opt("isSeries", query.is_series)
            .query_opt("isSports", query.is_sports)
            .query_opt("isUnaired", query.is_unaired)
            .query_opt("limit", query.limit)
            .query_list_opt("locationTypes", query.location_types)
            .query_opt("maxHeight", query.max_height)
            .query_opt("maxOfficialRating", query.max_official_rating)
            .query_opt("maxPremiereDate", query.max_premiere_date)
            .query_opt("maxWidth", query.max_width)
            .query_list_opt("mediaTypes", query.media_types)
            .query_opt("minCommunityRating", query.min_community_rating)
            .query_opt("minCriticRating", query.min_critic_rating)
            .query_opt("minDateLastSaved", query.min_date_last_saved)
            .query_opt(
                "minDateLastSavedForUser",
                query.min_date_last_saved_for_user,
            )
            .query_opt("minHeight", query.min_height)
            .query_opt("minOfficialRating", query.min_official_rating)
            .query_opt("minPremiereDate", query.min_premiere_date)
            .query_opt("minWidth", query.min_width)
            .query_opt("nameLessThan", query.name_less_than)
            .query_opt("nameStartsWith", query.name_starts_with)
            .query_opt("nameStartsWithOrGreater", query.name_starts_with_or_greater)
            .query_list_opt("officialRatings", query.official_ratings)
            .query_opt("parentId", query.parent_id)
            .query_opt("parentIndexNumber", query.parent_index_number)
            .query_opt("person", query.person)
            .query_list_opt("personIds", query.person_ids)
            .query_list_opt("personTypes", query.person_types)
            .query_opt("recursive", query.recursive)
            .query_opt("searchTerm", query.search_term)
            .query_list_opt("seriesStatus", query.series_status)
            .query_list_opt("sortBy", query.sort_by)
            .query_list_opt("sortOrder", query.sort_order)
            .query_opt("startIndex", query.start_index)
            .query_list_opt("studioIds", query.studio_ids)
            .query_list_opt("studios", query.studios)
            .query_list_opt("tags", query.tags)
            .query_opt("userId", query.user_id)
            .query_list_opt("videoTypes", query.video_types)
            .query_list_opt("years", query.years)
            .send()
            .await
    }

    #[doc = "Deletes items from the library and filesystem\n\nSends a `DELETE` request to `/Items`\n\nArguments:\n- `ids`: The item ids.\n"]
    pub async fn delete_items(&self, ids: Option<&Vec<uuid::Uuid>>) -> Result<(), Error> {
        self.request(reqwest::Method::DELETE, "/Items".into())
            .query_list_opt("ids", ids)
            .send_no_content()
            .await
    }

    #[doc = "Gets an item from a user's library\n\nSends a `GET` request to `/Items/{itemId}`\n\nArguments:\n- `item_id`: Item id.\n- `user_id`: User id.\n"]
    pub async fn get_item(
        &self,
        item_id: &uuid::Uuid,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::BaseItemDto, Error> {
        self.request(
            reqwest::Method::GET,
            format!("/Items/{}", encode_path(&item_id.to_string())),
        )
        .query_opt("userId", user_id)
        .send()
        .await
    }

    #[doc = "Updates an item\n\nSends a `POST` request to `/Items/{itemId}`\n\nArguments:\n- `item_id`: The item id.\n- `body`: The new item properties.\n"]
    pub async fn update_item(
        &self,
        item_id: &uuid::Uuid,
        body: &types::BaseItemDto,
    ) -> Result<(), Error> {
        self.request(
            reqwest::Method::POST,
            format!("/Items/{}", encode_path(&item_id.to_string())),
        )
        .json_body(body)
        .send_no_content()
        .await
    }

    #[doc = "Deletes an item from the library and filesystem\n\nSends a `DELETE` request to `/Items/{itemId}`\n\nArguments:\n- `item_id`: The item id.\n"]
    pub async fn delete_item(&self, item_id: &uuid::Uuid) -> Result<(), Error> {
        self.request(
            reqwest::Method::DELETE,
            format!("/Items/{}", encode_path(&item_id.to_string())),
        )
        .send_no_content()
        .await
    }

    #[doc = "Updates an item's content type\n\nSends a `POST` request to `/Items/{itemId}/ContentType`\n\nArguments:\n- `item_id`: The item id.\n- `content_type`: The content type of the item.\n"]
    pub async fn update_item_content_type(
        &self,
        item_id: &uuid::Uuid,
        content_type: Option<&str>,
    ) -> Result<(), Error> {
        self.request(
            reqwest::Method::POST,
            format!("/Items/{}/ContentType", encode_path(&item_id.to_string())),
        )
        .query_opt("contentType", content_type)
        .send_no_content()
        .await
    }

    #[doc = "Gets metadata editor info for an item\n\nSends a `GET` request to `/Items/{itemId}/MetadataEditor`\n\nArguments:\n- `item_id`: The item id.\n"]
    pub async fn get_metadata_editor_info(
        &self,
        item_id: &uuid::Uuid,
    ) -> Result<types::MetadataEditorInfo, Error> {
        self.request(
            reqwest::Method::GET,
            format!(
                "/Items/{}/MetadataEditor",
                encode_path(&item_id.to_string())
            ),
        )
        .send()
        .await
    }

    #[doc = "Gets all parents of an item\n\nSends a `GET` request to `/Items/{itemId}/Ancestors`\n\nArguments:\n- `item_id`: The item id.\n- `user_id`: Optional. Filter by user id, and attach user data.\n"]
    pub async fn get_ancestors(
        &self,
        item_id: &uuid::Uuid,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<Vec<types::BaseItemDto>, Error> {
        self.request(
            reqwest::Method::GET,
            format!("/Items/{}/Ancestors", encode_path(&item_id.to_string())),
        )
        .query_opt("userId", user_id)
        .send()
        .await
    }

    #[doc = "Gets critic review for an item\n\nSends a `GET` request to `/Items/{itemId}/CriticReviews`\n\n"]
    pub async fn get_critic_reviews(
        &self,
        item_id: &str,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(
            reqwest::Method::GET,
            format!("/Items/{}/CriticReviews", encode_path(item_id)),
        )
        .send()
        .await
    }

    #[doc = "Downloads item media\n\nSends a `GET` request to `/Items/{itemId}/Download`\n\nArguments:\n- `item_id`: The item id.\n"]
    pub async fn get_download(&self, item_id: &uuid::Uuid) -> Result<reqwest::Response, Error> {
        self.request(
            reqwest::Method::GET,
            format!("/Items/{}/Download", encode_path(&item_id.to_string())),
        )
        .send_response()
        .await
    }

    #[doc = "Get the original file of an item\n\nSends a `GET` request to `/Items/{itemId}/File`\n\nArguments:\n- `item_id`: The item id.\n"]
    pub async fn get_file(&self, item_id: &uuid::Uuid) -> Result<reqwest::Response, Error> {
        self.request(
            reqwest::Method::GET,
            format!("/Items/{}/File", encode_path(&item_id.to_string())),
        )
        .send_response()
        .await
    }

    #[doc = "Gets similar items\n\nSends a `GET` request to `/Items/{itemId}/Similar`\n\nArguments:\n- `item_id`: The item id.\n- `exclude_artist_ids`: Exclude artist ids.\n- `fields`: Optional. Specify additional fields of information to return in the output. This allows multiple, comma delimited. Options: Budget, Chapters, DateCreated, Genres, HomePageUrl, IndexOptions, MediaStreams, Overview, ParentId, Path, People, ProviderIds, PrimaryImageAspectRatio, Revenue, SortName, Studios, Taglines, TrailerUrls.\n- `limit`: Optional. The maximum number of records to return.\n- `user_id`: Optional. Filter by user id, and attach user data.\n"]
    pub async fn get_similar_items(
        &self,
        item_id: &uuid::Uuid,
        exclude_artist_ids: Option<&Vec<uuid::Uuid>>,
        fields: Option<&Vec<types::ItemFields>>,
        limit: Option<i32>,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(
            reqwest::Method::GET,
            format!("/Items/{}/Similar", encode_path(&item_id.to_string())),
        )
        .query_list_opt("excludeArtistIds", exclude_artist_ids)
        .query_list_opt("fields", fields)
        .query_opt("limit", limit)
        .query_opt("userId", user_id)
        .send()
        .await
    }

    #[doc = "Get theme songs and videos for an item\n\nSends a `GET` request to `/Items/{itemId}/ThemeMedia`\n\nArguments:\n- `item_id`: The item id.\n- `inherit_from_parent`: Optional. Determines whether or not parent items should be searched for theme media.\n- `sort_by`: Optional. Specify one or more sort orders, comma delimited. Options: Album, AlbumArtist, Artist, Budget, CommunityRating, CriticRating, DateCreated, DatePlayed, PlayCount, PremiereDate, ProductionYear, SortName, Random, Revenue, Runtime.\n- `sort_order`: Optional. Sort Order - Ascending, Descending.\n- `user_id`: Optional. Filter by user id, and attach user data.\n"]
    pub async fn get_theme_media(
        &self,
        item_id: &uuid::Uuid,
        inherit_from_parent: Option<bool>,
        sort_by: Option<&Vec<types::ItemSortBy>>,
        sort_order: Option<&Vec<types::SortOrder>>,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::AllThemeMediaResult, Error> {
        self.request(
            reqwest::Method::GET,
            format!("/Items/{}/ThemeMedia", encode_path(&item_id.to_string())),
        )
        .query_opt("inheritFromParent", inherit_from_parent)
        .query_list_opt("sortBy", sort_by)
        .query_list_opt("sortOrder", sort_order)
        .query_opt("userId", user_id)
        .send()
        .await
    }

    #[doc = "Get theme songs for an item\n\nSends a `GET` request to `/Items/{itemId}/ThemeSongs`\n\nArguments:\n- `item_id`: The item id.\n- `inherit_from_parent`: Optional. Determines whether or not parent items should be searched for theme media.\n- `sort_by`: Optional. Specify one or more sort orders, comma delimited. Options: Album, AlbumArtist, Artist, Budget, CommunityRating, CriticRating, DateCreated, DatePlayed, PlayCount, PremiereDate, ProductionYear, SortName, Random, Revenue, Runtime.\n- `sort_order`: Optional. Sort Order - Ascending, Descending.\n- `user_id`: Optional. Filter by user id, and attach user data.\n"]
    pub async fn get_theme_songs(
        &self,
        item_id: &uuid::Uuid,
        inherit_from_parent: Option<bool>,
        sort_by: Option<&Vec<types::ItemSortBy>>,
        sort_order: Option<&Vec<types::SortOrder>>,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::ThemeMediaResult, Error> {
        self.request(
            reqwest::Method::GET,
            format!("/Items/{}/ThemeSongs", encode_path(&item_id.to_string())),
        )
        .query_opt("inheritFromParent", inherit_from_parent)
        .query_list_opt("sortBy", sort_by)
        .query_list_opt("sortOrder", sort_order)
        .query_opt("userId", user_id)
        .send()
        .await
    }

    #[doc = "Get theme videos for an item\n\nSends a `GET` request to `/Items/{itemId}/ThemeVideos`\n\nArguments:\n- `item_id`: The item id.\n- `inherit_from_parent`: Optional. Determines whether or not parent items should be searched for theme media.\n- `sort_by`: Optional. Specify one or more sort orders, comma delimited. Options: Album, AlbumArtist, Artist, Budget, CommunityRating, CriticRating, DateCreated, DatePlayed, PlayCount, PremiereDate, ProductionYear, SortName, Random, Revenue, Runtime.\n- `sort_order`: Optional. Sort Order - Ascending, Descending.\n- `user_id`: Optional. Filter by user id, and attach user data.\n"]
    pub async fn get_theme_videos(
        &self,
        item_id: &uuid::Uuid,
        inherit_from_parent: Option<bool>,
        sort_by: Option<&Vec<types::ItemSortBy>>,
        sort_order: Option<&Vec<types::SortOrder>>,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::ThemeMediaResult, Error> {
        self.request(
            reqwest::Method::GET,
            format!("/Items/{}/ThemeVideos", encode_path(&item_id.to_string())),
        )
        .query_opt("inheritFromParent", inherit_from_parent)
        .query_list_opt("sortBy", sort_by)
        .query_list_opt("sortOrder", sort_order)
        .query_opt("userId", user_id)
        .send()
        .await
    }

    #[doc = "Get item counts\n\nSends a `GET` request to `/Items/Counts`\n\nArguments:\n- `is_favorite`: Optional. Get counts of favorite items.\n- `user_id`: Optional. Get counts from a specific user's library.\n"]
    pub async fn get_item_counts(
        &self,
        is_favorite: Option<bool>,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::ItemCounts, Error> {
        self.request(reqwest::Method::GET, "/Items/Counts".into())
            .query_opt("isFavorite", is_favorite)
            .query_opt("userId", user_id)
            .send()
            .await
    }

    #[doc = "Gets similar items\n\nSends a `GET` request to `/Movies/{itemId}/Similar`\n\nArguments:\n- `item_id`: The item id.\n- `exclude_artist_ids`: Exclude artist ids.\n- `fields`: Optional. Specify additional fields of information to return in the output. This allows multiple, comma delimited. Options: Budget, Chapters, DateCreated, Genres, HomePageUrl, IndexOptions, MediaStreams, Overview, ParentId, Path, People, ProviderIds, PrimaryImageAspectRatio, Revenue, SortName, Studios, Taglines, TrailerUrls.\n- `limit`: Optional. The maximum number of records to return.\n- `user_id`: Optional. Filter by user id, and attach user data.\n"]
    pub async fn get_similar_movies(
        &self,
        item_id: &uuid::Uuid,
        exclude_artist_ids: Option<&Vec<uuid::Uuid>>,
        fields: Option<&Vec<types::ItemFields>>,
        limit: Option<i32>,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(
            reqwest::Method::GET,
            format!("/Movies/{}/Similar", encode_path(&item_id.to_string())),
        )
        .query_list_opt("excludeArtistIds", exclude_artist_ids)
        .query_list_opt("fields", fields)
        .query_opt("limit", limit)
        .query_opt("userId", user_id)
        .send()
        .await
    }

    #[doc = "Gets similar items\n\nSends a `GET` request to `/Trailers/{itemId}/Similar`\n\nArguments:\n- `item_id`: The item id.\n- `exclude_artist_ids`: Exclude artist ids.\n- `fields`: Optional. Specify additional fields of information to return in the output. This allows multiple, comma delimited. Options: Budget, Chapters, DateCreated, Genres, HomePageUrl, IndexOptions, MediaStreams, Overview, ParentId, Path, People, ProviderIds, PrimaryImageAspectRatio, Revenue, SortName, Studios, Taglines, TrailerUrls.\n- `limit`: Optional. The maximum number of records to return.\n- `user_id`: Optional. Filter by user id, and attach user data.\n"]
    pub async fn get_similar_trailers(
        &self,
        item_id: &uuid::Uuid,
        exclude_artist_ids: Option<&Vec<uuid::Uuid>>,
        fields: Option<&Vec<types::ItemFields>>,
        limit: Option<i32>,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(
            reqwest::Method::GET,
            format!("/Trailers/{}/Similar", encode_path(&item_id.to_string())),
        )
        .query_list_opt("excludeArtistIds", exclude_artist_ids)
        .query_list_opt("fields", fields)
        .query_opt("limit", limit)
        .query_opt("userId", user_id)
        .send()
        .await
    }

    #[doc = "Gets live playback media info for an item\n\nSends a `GET` request to `/Items/{itemId}/PlaybackInfo`\n\nArguments:\n- `item_id`: The item id.\n- `user_id`: The user id.\n"]
    pub async fn get_playback_info(
        &self,
        item_id: &uuid::Uuid,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::PlaybackInfoResponse, Error> {
        self.request(
            reqwest::Method::GET,
            format!("/Items/{}/PlaybackInfo", encode_path(&item_id.to_string())),
        )
        .query_opt("userId", user_id)
        .send()
        .await
    }

    #[doc = "Gets live playback media info for an item\n\nFor backwards compatibility parameters can be sent via Query or Body, with Query having higher precedence.\r\nQuery parameters are obsolete.\n\nSends a `POST` request to `/Items/{itemId}/PlaybackInfo`\n\nArguments:\n- `item_id`: The item id.\n- `allow_audio_stream_copy`: Whether to allow to copy the audio stream. Default: true.\n- `allow_video_stream_copy`: Whether to allow to copy the video stream. Default: true.\n- `audio_stream_index`: The audio stream index.\n- `auto_open_live_stream`: Whether to auto open the livestream.\n- `enable_direct_play`: Whether to enable direct play. Default: true.\n- `enable_direct_stream`: Whether to enable direct stream. Default: true.\n- `enable_transcoding`: Whether to enable transcoding. Default: true.\n- `live_stream_id`: The livestream id.\n- `max_audio_channels`: The maximum number of audio channels.\n- `max_streaming_bitrate`: The maximum streaming bitrate.\n- `media_source_id`: The media source id.\n- `start_time_ticks`: The start time in ticks.\n- `subtitle_stream_index`: The subtitle stream index.\n- `user_id`: The user id.\n- `body`: The playback info.\n"]
    pub async fn get_posted_playback_info(
        &self,
        item_id: &uuid::Uuid,
        query: &query::GetPostedPlaybackInfo<'_>,
        body: &types::PlaybackInfoDto,
    ) -> Result<types::PlaybackInfoResponse, Error> {
        self.request(
            reqwest::Method::POST,
            format!("/Items/{}/PlaybackInfo", encode_path(&item_id.to_string())),
        )
        .query_opt("allowAudioStreamCopy", query.allow_audio_stream_copy)
        .query_opt("allowVideoStreamCopy", query.allow_video_stream_copy)
        .query_opt("audioStreamIndex", query.audio_stream_index)
        .query_opt("autoOpenLiveStream", query.auto_open_live_stream)
        .query_opt("enableDirectPlay", query.enable_direct_play)
        .query_opt("enableDirectStream", query.enable_direct_stream)
        .query_opt("enableTranscoding", query.enable_transcoding)
        .query_opt("liveStreamId", query.live_stream_id)
        .query_opt("maxAudioChannels", query.max_audio_channels)
        .query_opt("maxStreamingBitrate", query.max_streaming_bitrate)
        .query_opt("mediaSourceId", query.media_source_id)
        .query_opt("startTimeTicks", query.start_time_ticks)
        .query_opt("subtitleStreamIndex", query.subtitle_stream_index)
        .query_opt("userId", query.user_id)
        .json_body(body)
        .send()
        .await
    }

    #[doc = "Gets all media segments based on an itemId\n\nSends a `GET` request to `/MediaSegments/{itemId}`\n\nArguments:\n- `item_id`: The ItemId.\n- `include_segment_types`: Optional filter of requested segment types.\n"]
    pub async fn get_item_segments(
        &self,
        item_id: &uuid::Uuid,
        include_segment_types: Option<&Vec<types::MediaSegmentType>>,
    ) -> Result<types::MediaSegmentDtoQueryResult, Error> {
        self.request(
            reqwest::Method::GET,
            format!("/MediaSegments/{}", encode_path(&item_id.to_string())),
        )
        .query_list_opt("includeSegmentTypes", include_segment_types)
        .send()
        .await
    }

    #[doc = "Gets movie recommendations\n\nSends a `GET` request to `/Movies/Recommendations`\n\nArguments:\n- `category_limit`: The max number of categories to return.\n- `fields`: Optional. The fields to return.\n- `item_limit`: The max number of items to return per category.\n- `parent_id`: Specify this to localize the search to a specific item or folder. Omit to use the root.\n- `user_id`: Optional. Filter by user id, and attach user data.\n"]
    pub async fn get_movie_recommendations(
        &self,
        category_limit: Option<i32>,
        fields: Option<&Vec<types::ItemFields>>,
        item_limit: Option<i32>,
        parent_id: Option<&uuid::Uuid>,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<Vec<types::RecommendationDto>, Error> {
        self.request(reqwest::Method::GET, "/Movies/Recommendations".into())
            .query_opt("categoryLimit", category_limit)
            .query_list_opt("fields", fields)
            .query_opt("itemLimit", item_limit)
            .query_opt("parentId", parent_id)
            .query_opt("userId", user_id)
            .send()
            .await
    }

    #[doc = "Gets available remote images for an item\n\nSends a `GET` request to `/Items/{itemId}/RemoteImages`\n\nArguments:\n- `item_id`: Item Id.\n- `include_all_languages`: Optional. Include all languages.\n- `limit`: Optional. The maximum number of records to return.\n- `provider_name`: Optional. The image provider to use.\n- `start_index`: Optional. The record index to start at. All items with a lower index will be dropped from the results.\n- `type_`: The image type.\n"]
    pub async fn get_remote_images(
        &self,
        item_id: &uuid::Uuid,
        include_all_languages: Option<bool>,
        limit: Option<i32>,
        provider_name: Option<&str>,
        start_index: Option<i32>,
        type_: Option<types::ImageType>,
    ) -> Result<types::RemoteImageResult, Error> {
        self.request(
            reqwest::Method::GET,
            format!("/Items/{}/RemoteImages", encode_path(&item_id.to_string())),
        )
        .query_opt("includeAllLanguages", include_all_languages)
        .query_opt("limit", limit)
        .query_opt("providerName", provider_name)
        .query_opt("startIndex", start_index)
        .query_opt("type", type_)
        .send()
        .await
    }

    #[doc = "Downloads a remote image for an item\n\nSends a `POST` request to `/Items/{itemId}/RemoteImages/Download`\n\nArguments:\n- `item_id`: Item Id.\n- `image_url`: The image url.\n- `type_`: The image type.\n"]
    pub async fn download_remote_image(
        &self,
        item_id: &uuid::Uuid,
        image_url: Option<&str>,
        type_: types::ImageType,
    ) -> Result<(), Error> {
        self.request(
            reqwest::Method::POST,
            format!(
                "/Items/{}/RemoteImages/Download",
                encode_path(&item_id.to_string())
            ),
        )
        .query_opt("imageUrl", image_url)
        .query("type", type_)
        .send_no_content()
        .await
    }

    #[doc = "Gets available remote image providers for an item\n\nSends a `GET` request to `/Items/{itemId}/RemoteImages/Providers`\n\nArguments:\n- `item_id`: Item Id.\n"]
    pub async fn get_remote_image_providers(
        &self,
        item_id: &uuid::Uuid,
    ) -> Result<Vec<types::ImageProviderInfo>, Error> {
        self.request(
            reqwest::Method::GET,
            format!(
                "/Items/{}/RemoteImages/Providers",
                encode_path(&item_id.to_string())
            ),
        )
        .send()
        .await
    }

    #[doc = "Gets the search hint result\n\nSends a `GET` request to `/Search/Hints`\n\nArguments:\n- `exclude_item_types`: If specified, results with these item types are filtered out. This allows multiple, comma delimited.\n- `include_artists`: Optional filter whether to include artists.\n- `include_genres`: Optional filter whether to include genres.\n- `include_item_types`: If specified, only results with the specified item types are returned. This allows multiple, comma delimited.\n- `include_media`: Optional filter whether to include media.\n- `include_people`: Optional filter whether to include people.\n- `include_studios`: Optional filter whether to include studios.\n- `is_kids`: Optional filter for kids.\n- `is_movie`: Optional filter for movies.\n- `is_news`: Optional filter for news.\n- `is_series`: Optional filter for series.\n- `is_sports`: Optional filter for sports.\n- `limit`: Optional. The maximum number of records to return.\n- `media_types`: If specified, only results with the specified media types are returned. This allows multiple, comma delimited.\n- `parent_id`: If specified, only children of the parent are returned.\n- `search_term`: The search term to filter on.\n- `start_index`: Optional. The record index to start at. All items with a lower index will be dropped from the results.\n- `user_id`: Optional. Supply a user id to search within a user's library or omit to search all.\n"]
    pub async fn get_search_hints(
        &self,
        search_term: &str,
        query: &query::GetSearchHints<'_>,
    ) -> Result<types::SearchHintResult, Error> {
        self.request(reqwest::Method::GET, "/Search/Hints".into())
            .query_list_opt("excludeItemTypes", query.exclude_item_types)
            .query_opt("includeArtists", query.include_artists)
            .query_opt("includeGenres", query.include_genres)
            .query_list_opt("includeItemTypes", query.include_item_types)
            .query_opt("includeMedia", query.include_media)
            .query_opt("includePeople", query.include_people)
            .query_opt("includeStudios", query.include_studios)
            .query_opt("isKids", query.is_kids)
            .query_opt("isMovie", query.is_movie)
            .query_opt("isNews", query.is_news)
            .query_opt("isSeries", query.is_series)
            .query_opt("isSports", query.is_sports)
            .query_opt("limit", query.limit)
            .query_list_opt("mediaTypes", query.media_types)
            .query_opt("parentId", query.parent_id)
            .query("searchTerm", search_term)
            .query_opt("startIndex", query.start_index)
            .query_opt("userId", query.user_id)
            .send()
            .await
    }

    #[doc = "Search remote subtitles\n\nSends a `GET` request to `/Items/{itemId}/RemoteSearch/Subtitles/{language}`\n\nArguments:\n- `item_id`: The item id.\n- `language`: The language of the subtitles.\n- `is_perfect_match`: Optional. Only show subtitles which are a perfect match.\n"]
    pub async fn search_remote_subtitles(
        &self,
        item_id: &uuid::Uuid,
        language: &str,
        is_perfect_match: Option<bool>,
    ) -> Result<Vec<types::RemoteSubtitleInfo>, Error> {
        self.request(
            reqwest::Method::GET,
            format!(
                "/Items/{}/RemoteSearch/Subtitles/{}",
                encode_path(&item_id.to_string()),
                encode_path(language)
            ),
        )
        .query_opt("isPerfectMatch", is_perfect_match)
        .send()
        .await
    }

    #[doc = "Downloads a remote subtitle\n\nSends a `POST` request to `/Items/{itemId}/RemoteSearch/Subtitles/{subtitleId}`\n\nArguments:\n- `item_id`: The item id.\n- `subtitle_id`: The subtitle id.\n"]
    pub async fn download_remote_subtitles(
        &self,
        item_id: &uuid::Uuid,
        subtitle_id: &str,
    ) -> Result<(), Error> {
        self.request(
            reqwest::Method::POST,
            format!(
                "/Items/{}/RemoteSearch/Subtitles/{}",
                encode_path(&item_id.to_string()),
                encode_path(subtitle_id)
            ),
        )
        .send_no_content()
        .await
    }

    #[doc = "Gets suggestions\n\nSends a `GET` request to `/Items/Suggestions`\n\nArguments:\n- `enable_total_record_count`: Whether to enable the total record count.\n- `limit`: Optional. The limit.\n- `media_type`: The media types.\n- `start_index`: Optional. The start index.\n- `type_`: The type.\n- `user_id`: The user id.\n"]
    pub async fn get_suggestions(
        &self,
        enable_total_record_count: Option<bool>,
        limit: Option<i32>,
        media_type: Option<&Vec<types::MediaType>>,
        start_index: Option<i32>,
        type_: Option<&Vec<types::BaseItemKind>>,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(reqwest::Method::GET, "/Items/Suggestions".into())
            .query_opt("enableTotalRecordCount", enable_total_record_count)
            .query_opt("limit", limit)
            .query_list_opt("mediaType", media_type)
            .query_opt("startIndex", start_index)
            .query_list_opt("type", type_)
            .query_opt("userId", user_id)
            .send()
            .await
    }

    #[doc = "Finds movies and trailers similar to a given trailer\n\nSends a `GET` request to `/Trailers`\n\nArguments:\n- `adjacent_to`: Optional. Return items that are siblings of a supplied item.\n- `album_artist_ids`: Optional. If specified, results will be filtered to include only those containing the specified album artist id.\n- `album_ids`: Optional. If specified, results will be filtered based on album id. This allows multiple, pipe delimited.\n- `albums`: Optional. If specified, results will be filtered based on album. This allows multiple, pipe delimited.\n- `artist_ids`: Optional. If specified, results will be filtered to include only those containing the specified artist id.\n- `artists`: Optional. If specified, results will be filtered based on artists. This allows multiple, pipe delimited.\n- `collapse_box_set_items`: Whether or not to hide items behind their boxsets.\n- `contributing_artist_ids`: Optional. If specified, results will be filtered to include only those containing the specified contributing artist id.\n- `enable_image_types`: Optional. The image types to include in the output.\n- `enable_images`: Optional, include image information in output.\n- `enable_total_record_count`: Optional. Enable the total record count.\n- `enable_user_data`: Optional, include user data.\n- `exclude_artist_ids`: Optional. If specified, results will be filtered based on artist id. This allows multiple, pipe delimited.\n- `exclude_item_ids`: Optional. If specified, results will be filtered by excluding item ids. This allows multiple, comma delimited.\n- `exclude_item_types`: Optional. If specified, results will be filtered based on item type. This allows multiple, comma delimited.\n- `exclude_location_types`: Optional. If specified, results will be filtered based on the LocationType. This allows multiple, comma delimited.\n- `fields`: Optional. Specify additional fields of information to return in the output. This allows multiple, comma delimited. Options: Budget, Chapters, DateCreated, Genres, HomePageUrl, IndexOptions, MediaStreams, Overview, ParentId, Path, People, ProviderIds, PrimaryImageAspectRatio, Revenue, SortName, Studios, Taglines.\n- `filters`: Optional. Specify additional filters to apply. This allows multiple, comma delimited. Options: IsFolder, IsNotFolder, IsUnplayed, IsPlayed, IsFavorite, IsResumable, Likes, Dislikes.\n- `genre_ids`: Optional. If specified, results will be filtered based on genre id. This allows multiple, pipe delimited.\n- `genres`: Optional. If specified, results will be filtered based on genre. This allows multiple, pipe delimited.\n- `has_imdb_id`: Optional filter by items that have an IMDb id or not.\n- `has_official_rating`: Optional filter by items that have official ratings.\n- `has_overview`: Optional filter by items that have an overview or not.\n- `has_parental_rating`: Optional filter by items that have or do not have a parental rating.\n- `has_special_feature`: Optional filter by items with special features.\n- `has_subtitles`: Optional filter by items with subtitles.\n- `has_theme_song`: Optional filter by items with theme songs.\n- `has_theme_video`: Optional filter by items with theme videos.\n- `has_tmdb_id`: Optional filter by items that have a TMDb id or not.\n- `has_trailer`: Optional filter by items with trailers.\n- `has_tvdb_id`: Optional filter by items that have a TVDb id or not.\n- `ids`: Optional. If specific items are needed, specify a list of item id's to retrieve. This allows multiple, comma delimited.\n- `image_type_limit`: Optional, the max number of images to return, per image type.\n- `image_types`: Optional. If specified, results will be filtered based on those containing image types. This allows multiple, comma delimited.\n- `is3_d`: Optional filter by items that are 3D, or not.\n- `is4_k`: Optional filter by items that are 4K or not.\n- `is_favorite`: Optional filter by items that are marked as favorite, or not.\n- `is_hd`: Optional filter by items that are HD or not.\n- `is_kids`: Optional filter for live tv kids.\n- `is_locked`: Optional filter by items that are locked.\n- `is_missing`: Optional filter by items that are missing episodes or not.\n- `is_movie`: Optional filter for live tv movies.\n- `is_news`: Optional filter for live tv news.\n- `is_place_holder`: Optional filter by items that are placeholders.\n- `is_played`: Optional filter by items that are played, or not.\n- `is_series`: Optional filter for live tv series.\n- `is_sports`: Optional filter for live tv sports.\n- `is_unaired`: Optional filter by items that are unaired episodes or not.\n- `limit`: Optional. The maximum number of records to return.\n- `location_types`: Optional. If specified, results will be filtered based on LocationType. This allows multiple, comma delimited.\n- `max_height`: Optional. Filter by the maximum height of the item.\n- `max_official_rating`: Optional filter by maximum official rating (PG, PG-13, TV-MA, etc).\n- `max_premiere_date`: Optional. The maximum premiere date. Format = ISO.\n- `max_width`: Optional. Filter by the maximum width of the item.\n- `media_types`: Optional filter by MediaType. Allows multiple, comma delimited.\n- `min_community_rating`: Optional filter by minimum community rating.\n- `min_critic_rating`: Optional filter by minimum critic rating.\n- `min_date_last_saved`: Optional. The minimum last saved date. Format = ISO.\n- `min_date_last_saved_for_user`: Optional. The minimum last saved date for the current user. Format = ISO.\n- `min_height`: Optional. Filter by the minimum height of the item.\n- `min_official_rating`: Optional filter by minimum official rating (PG, PG-13, TV-MA, etc).\n- `min_premiere_date`: Optional. The minimum premiere date. Format = ISO.\n- `min_width`: Optional. Filter by the minimum width of the item.\n- `name_less_than`: Optional filter by items whose name is equally or lesser than a given input string.\n- `name_starts_with`: Optional filter by items whose name is sorted equally than a given input string.\n- `name_starts_with_or_greater`: Optional filter by items whose name is sorted equally or greater than a given input string.\n- `official_ratings`: Optional. If specified, results will be filtered based on OfficialRating. This allows multiple, pipe delimited.\n- `parent_id`: Specify this to localize the search to a specific item or folder. Omit to use the root.\n- `parent_index_number`: Optional filter by parent index number.\n- `person`: Optional. If specified, results will be filtered to include only those containing the specified person.\n- `person_ids`: Optional. If specified, results will be filtered to include only those containing the specified person id.\n- `person_types`: Optional. If specified, along with Person, results will be filtered to include only those containing the specified person and PersonType. Allows multiple, comma-delimited.\n- `recursive`: When searching within folders, this determines whether or not the search will be recursive. true/false.\n- `search_term`: Optional. Filter based on a search term.\n- `series_status`: Optional filter by Series Status. Allows multiple, comma delimited.\n- `sort_by`: Optional. Specify one or more sort orders, comma delimited. Options: Album, AlbumArtist, Artist, Budget, CommunityRating, CriticRating, DateCreated, DatePlayed, PlayCount, PremiereDate, ProductionYear, SortName, Random, Revenue, Runtime.\n- `sort_order`: Sort Order - Ascending, Descending.\n- `start_index`: Optional. The record index to start at. All items with a lower index will be dropped from the results.\n- `studio_ids`: Optional. If specified, results will be filtered based on studio id. This allows multiple, pipe delimited.\n- `studios`: Optional. If specified, results will be filtered based on studio. This allows multiple, pipe delimited.\n- `tags`: Optional. If specified, results will be filtered based on tag. This allows multiple, pipe delimited.\n- `user_id`: The user id supplied as query parameter; this is required when not using an API key.\n- `video_types`: Optional filter by VideoType (videofile, dvd, bluray, iso). Allows multiple, comma delimited.\n- `years`: Optional. If specified, results will be filtered based on production year. This allows multiple, comma delimited.\n"]
    pub async fn get_trailers(
        &self,
        query: &query::GetTrailers<'_>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(reqwest::Method::GET, "/Trailers".into())
            .query_opt("adjacentTo", query.adjacent_to)
            .query_list_opt("albumArtistIds", query.album_artist_ids)
            .query_list_opt("albumIds", query.album_ids)
            .query_list_opt("albums", query.albums)
            .query_list_opt("artistIds", query.artist_ids)
            .query_list_opt("artists", query.artists)
            .query_opt("collapseBoxSetItems", query.collapse_box_set_items)
            .query_list_opt("contributingArtistIds", query.contributing_artist_ids)
            .query_list_opt("enableImageTypes", query.enable_image_types)
            .query_opt("enableImages", query.enable_images)
            .query_opt("enableTotalRecordCount", query.enable_total_record_count)
            .query_opt("enableUserData", query.enable_user_data)
            .query_list_opt("excludeArtistIds", query.exclude_artist_ids)
            .query_list_opt("excludeItemIds", query.exclude_item_ids)
            .query_list_opt("excludeItemTypes", query.exclude_item_types)
            .query_list_opt("excludeLocationTypes", query.exclude_location_types)
            .query_list_opt("fields", query.fields)
            .query_list_opt("filters", query.filters)
            .query_list_opt("genreIds", query.genre_ids)
            .query_list_opt("genres", query.genres)
            .query_opt("hasImdbId", query.has_imdb_id)
            .query_opt("hasOfficialRating", query.has_official_rating)
            .query_opt("hasOverview", query.has_overview)
            .query_opt("hasParentalRating", query.has_parental_rating)
            .query_opt("hasSpecialFeature", query.has_special_feature)
            .query_opt("hasSubtitles", query.has_subtitles)
            .query_opt("hasThemeSong", query.has_theme_song)
            .query_opt("hasThemeVideo", query.has_theme_video)
            .query_opt("hasTmdbId", query.has_tmdb_id)
            .query_opt("hasTrailer", query.has_trailer)
            .query_opt("hasTvdbId", query.has_tvdb_id)
            .query_list_opt("ids", query.ids)
            .query_opt("imageTypeLimit", query.image_type_limit)
            .query_list_opt("imageTypes", query.image_types)
            .query_opt("is3D", query.is_3d)
            .query_opt("is4K", query.is_4k)
            .query_opt("isFavorite", query.is_favorite)
            .query_opt("isHd", query.is_hd)
            .query_opt("isKids", query.is_kids)
            .query_opt("isLocked", query.is_locked)
            .query_opt("isMissing", query.is_missing)
            .query_opt("isMovie", query.is_movie)
            .query_opt("isNews", query.is_news)
            .query_opt("isPlaceHolder", query.is_place_holder)
            .query_opt("isPlayed", query.is_played)
            .query_opt("isSeries", query.is_series)
            .query_opt("isSports", query.is_sports)
            .query_opt("isUnaired", query.is_unaired)
            .query_opt("limit", query.limit)
            .query_list_opt("locationTypes", query.location_types)
            .query_opt("maxHeight", query.max_height)
            .query_opt("maxOfficialRating", query.max_official_rating)
            .query_opt("maxPremiereDate", query.max_premiere_date)
            .query_opt("maxWidth", query.max_width)
            .query_list_opt("mediaTypes", query.media_types)
            .query_opt("minCommunityRating", query.min_community_rating)
            .query_opt("minCriticRating", query.min_critic_rating)
            .query_opt("minDateLastSaved", query.min_date_last_saved)
            .query_opt(
                "minDateLastSavedForUser",
                query.min_date_last_saved_for_user,
            )
            .query_opt("minHeight", query.min_height)
            .query_opt("minOfficialRating", query.min_official_rating)
            .query_opt("minPremiereDate", query.min_premiere_date)
            .query_opt("minWidth", query.min_width)
            .query_opt("nameLessThan", query.name_less_than)
            .query_opt("nameStartsWith", query.name_starts_with)
            .query_opt("nameStartsWithOrGreater", query.name_starts_with_or_greater)
            .query_list_opt("officialRatings", query.official_ratings)
            .query_opt("parentId", query.parent_id)
            .query_opt("parentIndexNumber", query.parent_index_number)
            .query_opt("person", query.person)
            .query_list_opt("personIds", query.person_ids)
            .query_list_opt("personTypes", query.person_types)
            .query_opt("recursive", query.recursive)
            .query_opt("searchTerm", query.search_term)
            .query_list_opt("seriesStatus", query.series_status)
            .query_list_opt("sortBy", query.sort_by)
            .query_list_opt("sortOrder", query.sort_order)
            .query_opt("startIndex", query.start_index)
            .query_list_opt("studioIds", query.studio_ids)
            .query_list_opt("studios", query.studios)
            .query_list_opt("tags", query.tags)
            .query_opt("userId", query.user_id)
            .query_list_opt("videoTypes", query.video_types)
            .query_list_opt("years", query.years)
            .send()
            .await
    }

    #[doc = "Gets intros to play before the main media item plays\n\nSends a `GET` request to `/Items/{itemId}/Intros`\n\nArguments:\n- `item_id`: Item id.\n- `user_id`: User id.\n"]
    pub async fn get_intros(
        &self,
        item_id: &uuid::Uuid,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(
            reqwest::Method::GET,
            format!("/Items/{}/Intros", encode_path(&item_id.to_string())),
        )
        .query_opt("userId", user_id)
        .send()
        .await
    }

    #[doc = "Gets local trailers for an item\n\nSends a `GET` request to `/Items/{itemId}/LocalTrailers`\n\nArguments:\n- `item_id`: Item id.\n- `user_id`: User id.\n"]
    pub async fn get_local_trailers(
        &self,
        item_id: &uuid::Uuid,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<Vec<types::BaseItemDto>, Error> {
        self.request(
            reqwest::Method::GET,
            format!("/Items/{}/LocalTrailers", encode_path(&item_id.to_string())),
        )
        .query_opt("userId", user_id)
        .send()
        .await
    }

    #[doc = "Gets special features for an item\n\nSends a `GET` request to `/Items/{itemId}/SpecialFeatures`\n\nArguments:\n- `item_id`: Item id.\n- `user_id`: User id.\n"]
    pub async fn get_special_features(
        &self,
        item_id: &uuid::Uuid,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<Vec<types::BaseItemDto>, Error> {
        self.request(
            reqwest::Method::GET,
            format!(
                "/Items/{}/SpecialFeatures",
                encode_path(&item_id.to_string())
            ),
        )
        .query_opt("userId", user_id)
        .send()
        .await
    }

    #[doc = "Gets latest media\n\nSends a `GET` request to `/Items/Latest`\n\nArguments:\n- `enable_image_types`: Optional. The image types to include in the output.\n- `enable_images`: Optional. include image information in output.\n- `enable_user_data`: Optional. include user data.\n- `fields`: Optional. Specify additional fields of information to return in the output.\n- `group_items`: Whether or not to group items into a parent container.\n- `image_type_limit`: Optional. the max number of images to return, per image type.\n- `include_item_types`: Optional. If specified, results will be filtered based on item type. This allows multiple, comma delimited.\n- `is_played`: Filter by items that are played, or not.\n- `limit`: Return item limit.\n- `parent_id`: Specify this to localize the search to a specific item or folder. Omit to use the root.\n- `user_id`: User id.\n"]
    pub async fn get_latest_media(
        &self,
        query: &query::GetLatestMedia<'_>,
    ) -> Result<Vec<types::BaseItemDto>, Error> {
        self.request(reqwest::Method::GET, "/Items/Latest".into())
            .query_list_opt("enableImageTypes", query.enable_image_types)
            .query_opt("enableImages", query.enable_images)
            .query_opt("enableUserData", query.enable_user_data)
            .query_list_opt("fields", query.fields)
            .query_opt("groupItems", query.group_items)
            .query_opt("imageTypeLimit", query.image_type_limit)
            .query_list_opt("includeItemTypes", query.include_item_types)
            .query_opt("isPlayed", query.is_played)
            .query_opt("limit", query.limit)
            .query_opt("parentId", query.parent_id)
            .query_opt("userId", query.user_id)
            .send()
            .await
    }

    #[doc = "Gets the root folder from a user's library\n\nSends a `GET` request to `/Items/Root`\n\nArguments:\n- `user_id`: User id.\n"]
    pub async fn get_root_folder(
        &self,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::BaseItemDto, Error> {
        self.request(reqwest::Method::GET, "/Items/Root".into())
            .query_opt("userId", user_id)
            .send()
            .await
    }

    #[doc = "Get years\n\nSends a `GET` request to `/Years`\n\nArguments:\n- `enable_image_types`: Optional. The image types to include in the output.\n- `enable_images`: Optional. Include image information in output.\n- `enable_user_data`: Optional. Include user data.\n- `exclude_item_types`: Optional. If specified, results will be excluded based on item type. This allows multiple, comma delimited.\n- `fields`: Optional. Specify additional fields of information to return in the output.\n- `image_type_limit`: Optional. The max number of images to return, per image type.\n- `include_item_types`: Optional. If specified, results will be included based on item type. This allows multiple, comma delimited.\n- `limit`: Optional. The maximum number of records to return.\n- `media_types`: Optional. Filter by MediaType. Allows multiple, comma delimited.\n- `parent_id`: Specify this to localize the search to a specific item or folder. Omit to use the root.\n- `recursive`: Search recursively.\n- `sort_by`: Optional. Specify one or more sort orders, comma delimited. Options: Album, AlbumArtist, Artist, Budget, CommunityRating, CriticRating, DateCreated, DatePlayed, PlayCount, PremiereDate, ProductionYear, SortName, Random, Revenue, Runtime.\n- `sort_order`: Sort Order - Ascending,Descending.\n- `start_index`: Skips over a given number of items within the results. Use for paging.\n- `user_id`: User Id.\n"]
    pub async fn get_years(
        &self,
        query: &query::GetYears<'_>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(reqwest::Method::GET, "/Years".into())
            .query_list_opt("enableImageTypes", query.enable_image_types)
            .query_opt("enableImages", query.enable_images)
            .query_opt("enableUserData", query.enable_user_data)
            .query_list_opt("excludeItemTypes", query.exclude_item_types)
            .query_list_opt("fields", query.fields)
            .query_opt("imageTypeLimit", query.image_type_limit)
            .query_list_opt("includeItemTypes", query.include_item_types)
            .query_opt("limit", query.limit)
            .query_list_opt("mediaTypes", query.media_types)
            .query_opt("parentId", query.parent_id)
            .query_opt("recursive", query.recursive)
            .query_list_opt("sortBy", query.sort_by)
            .query_list_opt("sortOrder", query.sort_order)
            .query_opt("startIndex", query.start_index)
            .query_opt("userId", query.user_id)
            .send()
            .await
    }

    #[doc = "Gets a year\n\nSends a `GET` request to `/Years/{year}`\n\nArguments:\n- `year`: The year.\n- `user_id`: Optional. Filter by user id, and attach user data.\n"]
    pub async fn get_year(
        &self,
        year: i32,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::BaseItemDto, Error> {
        self.request(
            reqwest::Method::GET,
            format!("/Years/{}", encode_path(&year.to_string())),
        )
        .query_opt("userId", user_id)
        .send()
        .await
    }
}
