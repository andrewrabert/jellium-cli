use crate::Client;
use crate::error::Error;
use crate::query;
use crate::types;
use crate::util::encode_path;

impl Client {
    #[doc = "Gets all artists from a given item, folder, or the entire library\n\nSends a `GET` request to `/Artists`\n\nArguments:\n- `enable_image_types`: Optional. The image types to include in the output.\n- `enable_images`: Optional, include image information in output.\n- `enable_total_record_count`: Total record count.\n- `enable_user_data`: Optional, include user data.\n- `exclude_item_types`: Optional. If specified, results will be filtered out based on item type. This allows multiple, comma delimited.\n- `fields`: Optional. Specify additional fields of information to return in the output.\n- `filters`: Optional. Specify additional filters to apply.\n- `genre_ids`: Optional. If specified, results will be filtered based on genre id. This allows multiple, pipe delimited.\n- `genres`: Optional. If specified, results will be filtered based on genre. This allows multiple, pipe delimited.\n- `image_type_limit`: Optional, the max number of images to return, per image type.\n- `include_item_types`: Optional. If specified, results will be filtered based on item type. This allows multiple, comma delimited.\n- `is_favorite`: Optional filter by items that are marked as favorite, or not.\n- `limit`: Optional. The maximum number of records to return.\n- `media_types`: Optional filter by MediaType. Allows multiple, comma delimited.\n- `min_community_rating`: Optional filter by minimum community rating.\n- `name_less_than`: Optional filter by items whose name is equally or lesser than a given input string.\n- `name_starts_with`: Optional filter by items whose name is sorted equally than a given input string.\n- `name_starts_with_or_greater`: Optional filter by items whose name is sorted equally or greater than a given input string.\n- `official_ratings`: Optional. If specified, results will be filtered based on OfficialRating. This allows multiple, pipe delimited.\n- `parent_id`: Specify this to localize the search to a specific item or folder. Omit to use the root.\n- `person`: Optional. If specified, results will be filtered to include only those containing the specified person.\n- `person_ids`: Optional. If specified, results will be filtered to include only those containing the specified person ids.\n- `person_types`: Optional. If specified, along with Person, results will be filtered to include only those containing the specified person and PersonType. Allows multiple, comma-delimited.\n- `search_term`: Optional. Search term.\n- `sort_by`: Optional. Specify one or more sort orders, comma delimited.\n- `sort_order`: Sort Order - Ascending,Descending.\n- `start_index`: Optional. The record index to start at. All items with a lower index will be dropped from the results.\n- `studio_ids`: Optional. If specified, results will be filtered based on studio id. This allows multiple, pipe delimited.\n- `studios`: Optional. If specified, results will be filtered based on studio. This allows multiple, pipe delimited.\n- `tags`: Optional. If specified, results will be filtered based on tag. This allows multiple, pipe delimited.\n- `user_id`: User id.\n- `years`: Optional. If specified, results will be filtered based on production year. This allows multiple, comma delimited.\n"]
    pub async fn get_artists(
        &self,
        query: &query::GetArtists<'_>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(reqwest::Method::GET, "/Artists".into())
            .query_list_opt("enableImageTypes", query.enable_image_types)
            .query_opt("enableImages", query.enable_images)
            .query_opt("enableTotalRecordCount", query.enable_total_record_count)
            .query_opt("enableUserData", query.enable_user_data)
            .query_list_opt("excludeItemTypes", query.exclude_item_types)
            .query_list_opt("fields", query.fields)
            .query_list_opt("filters", query.filters)
            .query_list_opt("genreIds", query.genre_ids)
            .query_list_opt("genres", query.genres)
            .query_opt("imageTypeLimit", query.image_type_limit)
            .query_list_opt("includeItemTypes", query.include_item_types)
            .query_opt("isFavorite", query.is_favorite)
            .query_opt("limit", query.limit)
            .query_list_opt("mediaTypes", query.media_types)
            .query_opt("minCommunityRating", query.min_community_rating)
            .query_opt("nameLessThan", query.name_less_than)
            .query_opt("nameStartsWith", query.name_starts_with)
            .query_opt("nameStartsWithOrGreater", query.name_starts_with_or_greater)
            .query_list_opt("officialRatings", query.official_ratings)
            .query_opt("parentId", query.parent_id)
            .query_opt("person", query.person)
            .query_list_opt("personIds", query.person_ids)
            .query_list_opt("personTypes", query.person_types)
            .query_opt("searchTerm", query.search_term)
            .query_list_opt("sortBy", query.sort_by)
            .query_list_opt("sortOrder", query.sort_order)
            .query_opt("startIndex", query.start_index)
            .query_list_opt("studioIds", query.studio_ids)
            .query_list_opt("studios", query.studios)
            .query_list_opt("tags", query.tags)
            .query_opt("userId", query.user_id)
            .query_list_opt("years", query.years)
            .send()
            .await
    }

    #[doc = "Gets an artist by name\n\nSends a `GET` request to `/Artists/{name}`\n\nArguments:\n- `name`: Studio name.\n- `user_id`: Optional. Filter by user id, and attach user data.\n"]
    pub async fn get_artist_by_name(
        &self,
        name: &str,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::BaseItemDto, Error> {
        self.request(
            reqwest::Method::GET,
            format!("/Artists/{}", encode_path(name)),
        )
        .query_opt("userId", user_id)
        .send()
        .await
    }

    #[doc = "Gets all album artists from a given item, folder, or the entire library\n\nSends a `GET` request to `/Artists/AlbumArtists`\n\nArguments:\n- `enable_image_types`: Optional. The image types to include in the output.\n- `enable_images`: Optional, include image information in output.\n- `enable_total_record_count`: Total record count.\n- `enable_user_data`: Optional, include user data.\n- `exclude_item_types`: Optional. If specified, results will be filtered out based on item type. This allows multiple, comma delimited.\n- `fields`: Optional. Specify additional fields of information to return in the output.\n- `filters`: Optional. Specify additional filters to apply.\n- `genre_ids`: Optional. If specified, results will be filtered based on genre id. This allows multiple, pipe delimited.\n- `genres`: Optional. If specified, results will be filtered based on genre. This allows multiple, pipe delimited.\n- `image_type_limit`: Optional, the max number of images to return, per image type.\n- `include_item_types`: Optional. If specified, results will be filtered based on item type. This allows multiple, comma delimited.\n- `is_favorite`: Optional filter by items that are marked as favorite, or not.\n- `limit`: Optional. The maximum number of records to return.\n- `media_types`: Optional filter by MediaType. Allows multiple, comma delimited.\n- `min_community_rating`: Optional filter by minimum community rating.\n- `name_less_than`: Optional filter by items whose name is equally or lesser than a given input string.\n- `name_starts_with`: Optional filter by items whose name is sorted equally than a given input string.\n- `name_starts_with_or_greater`: Optional filter by items whose name is sorted equally or greater than a given input string.\n- `official_ratings`: Optional. If specified, results will be filtered based on OfficialRating. This allows multiple, pipe delimited.\n- `parent_id`: Specify this to localize the search to a specific item or folder. Omit to use the root.\n- `person`: Optional. If specified, results will be filtered to include only those containing the specified person.\n- `person_ids`: Optional. If specified, results will be filtered to include only those containing the specified person ids.\n- `person_types`: Optional. If specified, along with Person, results will be filtered to include only those containing the specified person and PersonType. Allows multiple, comma-delimited.\n- `search_term`: Optional. Search term.\n- `sort_by`: Optional. Specify one or more sort orders, comma delimited.\n- `sort_order`: Sort Order - Ascending,Descending.\n- `start_index`: Optional. The record index to start at. All items with a lower index will be dropped from the results.\n- `studio_ids`: Optional. If specified, results will be filtered based on studio id. This allows multiple, pipe delimited.\n- `studios`: Optional. If specified, results will be filtered based on studio. This allows multiple, pipe delimited.\n- `tags`: Optional. If specified, results will be filtered based on tag. This allows multiple, pipe delimited.\n- `user_id`: User id.\n- `years`: Optional. If specified, results will be filtered based on production year. This allows multiple, comma delimited.\n"]
    pub async fn get_album_artists(
        &self,
        query: &query::GetAlbumArtists<'_>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(reqwest::Method::GET, "/Artists/AlbumArtists".into())
            .query_list_opt("enableImageTypes", query.enable_image_types)
            .query_opt("enableImages", query.enable_images)
            .query_opt("enableTotalRecordCount", query.enable_total_record_count)
            .query_opt("enableUserData", query.enable_user_data)
            .query_list_opt("excludeItemTypes", query.exclude_item_types)
            .query_list_opt("fields", query.fields)
            .query_list_opt("filters", query.filters)
            .query_list_opt("genreIds", query.genre_ids)
            .query_list_opt("genres", query.genres)
            .query_opt("imageTypeLimit", query.image_type_limit)
            .query_list_opt("includeItemTypes", query.include_item_types)
            .query_opt("isFavorite", query.is_favorite)
            .query_opt("limit", query.limit)
            .query_list_opt("mediaTypes", query.media_types)
            .query_opt("minCommunityRating", query.min_community_rating)
            .query_opt("nameLessThan", query.name_less_than)
            .query_opt("nameStartsWith", query.name_starts_with)
            .query_opt("nameStartsWithOrGreater", query.name_starts_with_or_greater)
            .query_list_opt("officialRatings", query.official_ratings)
            .query_opt("parentId", query.parent_id)
            .query_opt("person", query.person)
            .query_list_opt("personIds", query.person_ids)
            .query_list_opt("personTypes", query.person_types)
            .query_opt("searchTerm", query.search_term)
            .query_list_opt("sortBy", query.sort_by)
            .query_list_opt("sortOrder", query.sort_order)
            .query_opt("startIndex", query.start_index)
            .query_list_opt("studioIds", query.studio_ids)
            .query_list_opt("studios", query.studios)
            .query_list_opt("tags", query.tags)
            .query_opt("userId", query.user_id)
            .query_list_opt("years", query.years)
            .send()
            .await
    }

    #[doc = "Get artist image by name\n\nSends a `GET` request to `/Artists/{name}/Images/{imageType}/{imageIndex}`\n\nArguments:\n- `name`: Artist name.\n- `image_type`: Image type.\n- `image_index`: Image index.\n- `background_color`: Optional. Apply a background color for transparent images.\n- `blur`: Optional. Blur image.\n- `fill_height`: Height of box to fill.\n- `fill_width`: Width of box to fill.\n- `foreground_layer`: Optional. Apply a foreground layer on top of the image.\n- `format`: Determines the output format of the image - original,gif,jpg,png.\n- `height`: The fixed image height to return.\n- `max_height`: The maximum image height to return.\n- `max_width`: The maximum image width to return.\n- `percent_played`: Optional. Percent to render for the percent played overlay.\n- `quality`: Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases.\n- `tag`: Optional. Supply the cache tag from the item object to receive strong caching headers.\n- `unplayed_count`: Optional. Unplayed count overlay to render.\n- `width`: The fixed image width to return.\n"]
    pub async fn get_artist_image(
        &self,
        name: &str,
        image_type: types::ImageType,
        image_index: i32,
        query: &query::GetArtistImage<'_>,
    ) -> Result<reqwest::Response, Error> {
        self.request(
            reqwest::Method::GET,
            format!(
                "/Artists/{}/Images/{}/{}",
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

    #[doc = "Get artist image by name\n\nSends a `HEAD` request to `/Artists/{name}/Images/{imageType}/{imageIndex}`\n\nArguments:\n- `name`: Artist name.\n- `image_type`: Image type.\n- `image_index`: Image index.\n- `background_color`: Optional. Apply a background color for transparent images.\n- `blur`: Optional. Blur image.\n- `fill_height`: Height of box to fill.\n- `fill_width`: Width of box to fill.\n- `foreground_layer`: Optional. Apply a foreground layer on top of the image.\n- `format`: Determines the output format of the image - original,gif,jpg,png.\n- `height`: The fixed image height to return.\n- `max_height`: The maximum image height to return.\n- `max_width`: The maximum image width to return.\n- `percent_played`: Optional. Percent to render for the percent played overlay.\n- `quality`: Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases.\n- `tag`: Optional. Supply the cache tag from the item object to receive strong caching headers.\n- `unplayed_count`: Optional. Unplayed count overlay to render.\n- `width`: The fixed image width to return.\n"]
    pub async fn head_artist_image(
        &self,
        name: &str,
        image_type: types::ImageType,
        image_index: i32,
        query: &query::HeadArtistImage<'_>,
    ) -> Result<reqwest::Response, Error> {
        self.request(
            reqwest::Method::HEAD,
            format!(
                "/Artists/{}/Images/{}/{}",
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

    #[doc = "Creates an instant playlist based on a given album\n\nSends a `GET` request to `/Albums/{itemId}/InstantMix`\n\nArguments:\n- `item_id`: The item id.\n- `enable_image_types`: Optional. The image types to include in the output.\n- `enable_images`: Optional. Include image information in output.\n- `enable_user_data`: Optional. Include user data.\n- `fields`: Optional. Specify additional fields of information to return in the output.\n- `image_type_limit`: Optional. The max number of images to return, per image type.\n- `limit`: Optional. The maximum number of records to return.\n- `user_id`: Optional. Filter by user id, and attach user data.\n"]
    pub async fn get_instant_mix_from_album(
        &self,
        item_id: &uuid::Uuid,
        query: &query::GetInstantMixFromAlbum<'_>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(
            reqwest::Method::GET,
            format!("/Albums/{}/InstantMix", encode_path(&item_id.to_string())),
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

    #[doc = "Creates an instant playlist based on a given artist\n\nSends a `GET` request to `/Artists/{itemId}/InstantMix`\n\nArguments:\n- `item_id`: The item id.\n- `enable_image_types`: Optional. The image types to include in the output.\n- `enable_images`: Optional. Include image information in output.\n- `enable_user_data`: Optional. Include user data.\n- `fields`: Optional. Specify additional fields of information to return in the output.\n- `image_type_limit`: Optional. The max number of images to return, per image type.\n- `limit`: Optional. The maximum number of records to return.\n- `user_id`: Optional. Filter by user id, and attach user data.\n"]
    pub async fn get_instant_mix_from_artist(
        &self,
        item_id: &uuid::Uuid,
        query: &query::GetInstantMixFromArtist<'_>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(
            reqwest::Method::GET,
            format!("/Artists/{}/InstantMix", encode_path(&item_id.to_string())),
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

    #[doc = "Creates an instant playlist based on a given artist\n\nSends a `GET` request to `/Artists/InstantMix`\n\nArguments:\n- `enable_image_types`: Optional. The image types to include in the output.\n- `enable_images`: Optional. Include image information in output.\n- `enable_user_data`: Optional. Include user data.\n- `fields`: Optional. Specify additional fields of information to return in the output.\n- `id`: The item id.\n- `image_type_limit`: Optional. The max number of images to return, per image type.\n- `limit`: Optional. The maximum number of records to return.\n- `user_id`: Optional. Filter by user id, and attach user data.\n"]
    pub async fn get_instant_mix_from_artist_by_id(
        &self,
        id: &uuid::Uuid,
        query: &query::GetInstantMixFromArtistById<'_>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(reqwest::Method::GET, "/Artists/InstantMix".into())
            .query_list_opt("enableImageTypes", query.enable_image_types)
            .query_opt("enableImages", query.enable_images)
            .query_opt("enableUserData", query.enable_user_data)
            .query_list_opt("fields", query.fields)
            .query("id", id)
            .query_opt("imageTypeLimit", query.image_type_limit)
            .query_opt("limit", query.limit)
            .query_opt("userId", query.user_id)
            .send()
            .await
    }

    #[doc = "Creates an instant playlist based on a given song\n\nSends a `GET` request to `/Songs/{itemId}/InstantMix`\n\nArguments:\n- `item_id`: The item id.\n- `enable_image_types`: Optional. The image types to include in the output.\n- `enable_images`: Optional. Include image information in output.\n- `enable_user_data`: Optional. Include user data.\n- `fields`: Optional. Specify additional fields of information to return in the output.\n- `image_type_limit`: Optional. The max number of images to return, per image type.\n- `limit`: Optional. The maximum number of records to return.\n- `user_id`: Optional. Filter by user id, and attach user data.\n"]
    pub async fn get_instant_mix_from_song(
        &self,
        item_id: &uuid::Uuid,
        query: &query::GetInstantMixFromSong<'_>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(
            reqwest::Method::GET,
            format!("/Songs/{}/InstantMix", encode_path(&item_id.to_string())),
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

    #[doc = "Gets similar items\n\nSends a `GET` request to `/Albums/{itemId}/Similar`\n\nArguments:\n- `item_id`: The item id.\n- `exclude_artist_ids`: Exclude artist ids.\n- `fields`: Optional. Specify additional fields of information to return in the output. This allows multiple, comma delimited. Options: Budget, Chapters, DateCreated, Genres, HomePageUrl, IndexOptions, MediaStreams, Overview, ParentId, Path, People, ProviderIds, PrimaryImageAspectRatio, Revenue, SortName, Studios, Taglines, TrailerUrls.\n- `limit`: Optional. The maximum number of records to return.\n- `user_id`: Optional. Filter by user id, and attach user data.\n"]
    pub async fn get_similar_albums(
        &self,
        item_id: &uuid::Uuid,
        exclude_artist_ids: Option<&Vec<uuid::Uuid>>,
        fields: Option<&Vec<types::ItemFields>>,
        limit: Option<i32>,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(
            reqwest::Method::GET,
            format!("/Albums/{}/Similar", encode_path(&item_id.to_string())),
        )
        .query_list_opt("excludeArtistIds", exclude_artist_ids)
        .query_list_opt("fields", fields)
        .query_opt("limit", limit)
        .query_opt("userId", user_id)
        .send()
        .await
    }

    #[doc = "Gets similar items\n\nSends a `GET` request to `/Artists/{itemId}/Similar`\n\nArguments:\n- `item_id`: The item id.\n- `exclude_artist_ids`: Exclude artist ids.\n- `fields`: Optional. Specify additional fields of information to return in the output. This allows multiple, comma delimited. Options: Budget, Chapters, DateCreated, Genres, HomePageUrl, IndexOptions, MediaStreams, Overview, ParentId, Path, People, ProviderIds, PrimaryImageAspectRatio, Revenue, SortName, Studios, Taglines, TrailerUrls.\n- `limit`: Optional. The maximum number of records to return.\n- `user_id`: Optional. Filter by user id, and attach user data.\n"]
    pub async fn get_similar_artists(
        &self,
        item_id: &uuid::Uuid,
        exclude_artist_ids: Option<&Vec<uuid::Uuid>>,
        fields: Option<&Vec<types::ItemFields>>,
        limit: Option<i32>,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(
            reqwest::Method::GET,
            format!("/Artists/{}/Similar", encode_path(&item_id.to_string())),
        )
        .query_list_opt("excludeArtistIds", exclude_artist_ids)
        .query_list_opt("fields", fields)
        .query_opt("limit", limit)
        .query_opt("userId", user_id)
        .send()
        .await
    }
}
