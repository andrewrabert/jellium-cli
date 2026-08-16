use crate::Client;
use crate::error::Error;
use crate::query;
use crate::types;
use crate::util::encode_path;

impl Client {
    #[doc = "Gets similar items\n\nSends a `GET` request to `/Shows/{itemId}/Similar`\n\nArguments:\n- `item_id`: The item id.\n- `exclude_artist_ids`: Exclude artist ids.\n- `fields`: Optional. Specify additional fields of information to return in the output. This allows multiple, comma delimited. Options: Budget, Chapters, DateCreated, Genres, HomePageUrl, IndexOptions, MediaStreams, Overview, ParentId, Path, People, ProviderIds, PrimaryImageAspectRatio, Revenue, SortName, Studios, Taglines, TrailerUrls.\n- `limit`: Optional. The maximum number of records to return.\n- `user_id`: Optional. Filter by user id, and attach user data.\n"]
    pub async fn get_similar_shows(
        &self,
        item_id: &uuid::Uuid,
        exclude_artist_ids: Option<&Vec<uuid::Uuid>>,
        fields: Option<&Vec<types::ItemFields>>,
        limit: Option<i32>,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(
            reqwest::Method::GET,
            format!("/Shows/{}/Similar", encode_path(&item_id.to_string())),
        )
        .query_list_opt("excludeArtistIds", exclude_artist_ids)
        .query_list_opt("fields", fields)
        .query_opt("limit", limit)
        .query_opt("userId", user_id)
        .send()
        .await
    }

    #[doc = "Gets episodes for a tv season\n\nSends a `GET` request to `/Shows/{seriesId}/Episodes`\n\nArguments:\n- `series_id`: The series id.\n- `adjacent_to`: Optional. Return items that are siblings of a supplied item.\n- `enable_image_types`: Optional. The image types to include in the output.\n- `enable_images`: Optional, include image information in output.\n- `enable_user_data`: Optional. Include user data.\n- `fields`: Optional. Specify additional fields of information to return in the output. This allows multiple, comma delimited. Options: Budget, Chapters, DateCreated, Genres, HomePageUrl, IndexOptions, MediaStreams, Overview, ParentId, Path, People, ProviderIds, PrimaryImageAspectRatio, Revenue, SortName, Studios, Taglines, TrailerUrls.\n- `image_type_limit`: Optional, the max number of images to return, per image type.\n- `is_missing`: Optional. Filter by items that are missing episodes or not.\n- `limit`: Optional. The maximum number of records to return.\n- `season`: Optional filter by season number.\n- `season_id`: Optional. Filter by season id.\n- `sort_by`: Optional. Specify one or more sort orders, comma delimited. Options: Album, AlbumArtist, Artist, Budget, CommunityRating, CriticRating, DateCreated, DatePlayed, PlayCount, PremiereDate, ProductionYear, SortName, Random, Revenue, Runtime.\n- `start_index`: Optional. The record index to start at. All items with a lower index will be dropped from the results.\n- `start_item_id`: Optional. Skip through the list until a given item is found.\n- `user_id`: The user id.\n"]
    pub async fn get_episodes(
        &self,
        series_id: &uuid::Uuid,
        query: &query::GetEpisodes<'_>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(
            reqwest::Method::GET,
            format!("/Shows/{}/Episodes", encode_path(&series_id.to_string())),
        )
        .query_opt("adjacentTo", query.adjacent_to)
        .query_list_opt("enableImageTypes", query.enable_image_types)
        .query_opt("enableImages", query.enable_images)
        .query_opt("enableUserData", query.enable_user_data)
        .query_list_opt("fields", query.fields)
        .query_opt("imageTypeLimit", query.image_type_limit)
        .query_opt("isMissing", query.is_missing)
        .query_opt("limit", query.limit)
        .query_opt("season", query.season)
        .query_opt("seasonId", query.season_id)
        .query_opt("sortBy", query.sort_by)
        .query_opt("startIndex", query.start_index)
        .query_opt("startItemId", query.start_item_id)
        .query_opt("userId", query.user_id)
        .send()
        .await
    }

    #[doc = "Gets seasons for a tv series\n\nSends a `GET` request to `/Shows/{seriesId}/Seasons`\n\nArguments:\n- `series_id`: The series id.\n- `adjacent_to`: Optional. Return items that are siblings of a supplied item.\n- `enable_image_types`: Optional. The image types to include in the output.\n- `enable_images`: Optional. Include image information in output.\n- `enable_user_data`: Optional. Include user data.\n- `fields`: Optional. Specify additional fields of information to return in the output. This allows multiple, comma delimited. Options: Budget, Chapters, DateCreated, Genres, HomePageUrl, IndexOptions, MediaStreams, Overview, ParentId, Path, People, ProviderIds, PrimaryImageAspectRatio, Revenue, SortName, Studios, Taglines, TrailerUrls.\n- `image_type_limit`: Optional. The max number of images to return, per image type.\n- `is_missing`: Optional. Filter by items that are missing episodes or not.\n- `is_special_season`: Optional. Filter by special season.\n- `user_id`: The user id.\n"]
    pub async fn get_seasons(
        &self,
        series_id: &uuid::Uuid,
        query: &query::GetSeasons<'_>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(
            reqwest::Method::GET,
            format!("/Shows/{}/Seasons", encode_path(&series_id.to_string())),
        )
        .query_opt("adjacentTo", query.adjacent_to)
        .query_list_opt("enableImageTypes", query.enable_image_types)
        .query_opt("enableImages", query.enable_images)
        .query_opt("enableUserData", query.enable_user_data)
        .query_list_opt("fields", query.fields)
        .query_opt("imageTypeLimit", query.image_type_limit)
        .query_opt("isMissing", query.is_missing)
        .query_opt("isSpecialSeason", query.is_special_season)
        .query_opt("userId", query.user_id)
        .send()
        .await
    }

    #[doc = "Gets a list of next up episodes\n\nSends a `GET` request to `/Shows/NextUp`\n\nArguments:\n- `disable_first_episode`: Whether to disable sending the first episode in a series as next up.\n- `enable_image_types`: Optional. The image types to include in the output.\n- `enable_images`: Optional. Include image information in output.\n- `enable_resumable`: Whether to include resumable episodes in next up results.\n- `enable_rewatching`: Whether to include watched episodes in next up results.\n- `enable_total_record_count`: Whether to enable the total records count. Defaults to true.\n- `enable_user_data`: Optional. Include user data.\n- `fields`: Optional. Specify additional fields of information to return in the output.\n- `image_type_limit`: Optional. The max number of images to return, per image type.\n- `limit`: Optional. The maximum number of records to return.\n- `next_up_date_cutoff`: Optional. Starting date of shows to show in Next Up section.\n- `parent_id`: Optional. Specify this to localize the search to a specific item or folder. Omit to use the root.\n- `series_id`: Optional. Filter by series id.\n- `start_index`: Optional. The record index to start at. All items with a lower index will be dropped from the results.\n- `user_id`: The user id of the user to get the next up episodes for.\n"]
    pub async fn get_next_up(
        &self,
        query: &query::GetNextUp<'_>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(reqwest::Method::GET, "/Shows/NextUp".into())
            .query_opt("disableFirstEpisode", query.disable_first_episode)
            .query_list_opt("enableImageTypes", query.enable_image_types)
            .query_opt("enableImages", query.enable_images)
            .query_opt("enableResumable", query.enable_resumable)
            .query_opt("enableRewatching", query.enable_rewatching)
            .query_opt("enableTotalRecordCount", query.enable_total_record_count)
            .query_opt("enableUserData", query.enable_user_data)
            .query_list_opt("fields", query.fields)
            .query_opt("imageTypeLimit", query.image_type_limit)
            .query_opt("limit", query.limit)
            .query_opt("nextUpDateCutoff", query.next_up_date_cutoff)
            .query_opt("parentId", query.parent_id)
            .query_opt("seriesId", query.series_id)
            .query_opt("startIndex", query.start_index)
            .query_opt("userId", query.user_id)
            .send()
            .await
    }

    #[doc = "Gets a list of upcoming episodes\n\nSends a `GET` request to `/Shows/Upcoming`\n\nArguments:\n- `enable_image_types`: Optional. The image types to include in the output.\n- `enable_images`: Optional. Include image information in output.\n- `enable_user_data`: Optional. Include user data.\n- `fields`: Optional. Specify additional fields of information to return in the output.\n- `image_type_limit`: Optional. The max number of images to return, per image type.\n- `limit`: Optional. The maximum number of records to return.\n- `parent_id`: Optional. Specify this to localize the search to a specific item or folder. Omit to use the root.\n- `start_index`: Optional. The record index to start at. All items with a lower index will be dropped from the results.\n- `user_id`: The user id of the user to get the upcoming episodes for.\n"]
    pub async fn get_upcoming_episodes(
        &self,
        query: &query::GetUpcomingEpisodes<'_>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(reqwest::Method::GET, "/Shows/Upcoming".into())
            .query_list_opt("enableImageTypes", query.enable_image_types)
            .query_opt("enableImages", query.enable_images)
            .query_opt("enableUserData", query.enable_user_data)
            .query_list_opt("fields", query.fields)
            .query_opt("imageTypeLimit", query.image_type_limit)
            .query_opt("limit", query.limit)
            .query_opt("parentId", query.parent_id)
            .query_opt("startIndex", query.start_index)
            .query_opt("userId", query.user_id)
            .send()
            .await
    }
}
