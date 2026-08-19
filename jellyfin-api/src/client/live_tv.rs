use crate::Client;
use crate::error::Error;
use crate::query;
use crate::types;
use crate::util::encode_path;

impl Client {
    #[doc = "Get channel mapping options\n\nSends a `GET` request to `/LiveTv/ChannelMappingOptions`\n\nArguments:\n- `provider_id`: Provider id.\n"]
    pub async fn get_channel_mapping_options(
        &self,
        provider_id: Option<&str>,
    ) -> Result<types::ChannelMappingOptionsDto, Error> {
        self.request(reqwest::Method::GET, "/LiveTv/ChannelMappingOptions".into())
            .query_opt("providerId", provider_id)
            .send()
            .await
    }

    #[doc = "Set channel mappings\n\nSends a `POST` request to `/LiveTv/ChannelMappings`\n\nArguments:\n- `body`: The set channel mapping dto.\n"]
    pub async fn set_channel_mapping(
        &self,
        body: &types::SetChannelMappingDto,
    ) -> Result<types::TunerChannelMapping, Error> {
        self.request(reqwest::Method::POST, "/LiveTv/ChannelMappings".into())
            .json_body(body)
            .send()
            .await
    }

    #[doc = "Gets available live tv channels\n\nSends a `GET` request to `/LiveTv/Channels`\n\nArguments:\n- `add_current_program`: Optional. Adds current program info to each channel.\n- `enable_favorite_sorting`: Optional. Incorporate favorite and like status into channel sorting.\n- `enable_image_types`: \"Optional. The image types to include in the output.\n- `enable_images`: Optional. Include image information in output.\n- `enable_user_data`: Optional. Include user data.\n- `fields`: Optional. Specify additional fields of information to return in the output.\n- `image_type_limit`: Optional. The max number of images to return, per image type.\n- `is_disliked`: Optional. Filter by channels that are disliked, or not.\n- `is_favorite`: Optional. Filter by channels that are favorites, or not.\n- `is_kids`: Optional. Filter for kids.\n- `is_liked`: Optional. Filter by channels that are liked, or not.\n- `is_movie`: Optional. Filter for movies.\n- `is_news`: Optional. Filter for news.\n- `is_series`: Optional. Filter for series.\n- `is_sports`: Optional. Filter for sports.\n- `limit`: Optional. The maximum number of records to return.\n- `sort_by`: Optional. Key to sort by.\n- `sort_order`: Optional. Sort order.\n- `start_index`: Optional. The record index to start at. All items with a lower index will be dropped from the results.\n- `type_`: Optional. Filter by channel type.\n- `user_id`: Optional. Filter by user and attach user data.\n"]
    pub async fn get_live_tv_channels(
        &self,
        query: &query::GetLiveTvChannels<'_>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(reqwest::Method::GET, "/LiveTv/Channels".into())
            .query_opt("addCurrentProgram", query.add_current_program)
            .query_opt("enableFavoriteSorting", query.enable_favorite_sorting)
            .query_list_opt("enableImageTypes", query.enable_image_types)
            .query_opt("enableImages", query.enable_images)
            .query_opt("enableUserData", query.enable_user_data)
            .query_list_opt("fields", query.fields)
            .query_opt("imageTypeLimit", query.image_type_limit)
            .query_opt("isDisliked", query.is_disliked)
            .query_opt("isFavorite", query.is_favorite)
            .query_opt("isKids", query.is_kids)
            .query_opt("isLiked", query.is_liked)
            .query_opt("isMovie", query.is_movie)
            .query_opt("isNews", query.is_news)
            .query_opt("isSeries", query.is_series)
            .query_opt("isSports", query.is_sports)
            .query_opt("limit", query.limit)
            .query_list_opt("sortBy", query.sort_by)
            .query_opt("sortOrder", query.sort_order)
            .query_opt("startIndex", query.start_index)
            .query_opt("type", query.type_)
            .query_opt("userId", query.user_id)
            .send()
            .await
    }

    #[doc = "Gets a live tv channel\n\nSends a `GET` request to `/LiveTv/Channels/{channelId}`\n\nArguments:\n- `channel_id`: Channel id.\n- `user_id`: Optional. Attach user data.\n"]
    pub async fn get_channel(
        &self,
        channel_id: &uuid::Uuid,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::BaseItemDto, Error> {
        self.request(
            reqwest::Method::GET,
            format!("/LiveTv/Channels/{}", encode_path(&channel_id.to_string())),
        )
        .query_opt("userId", user_id)
        .send()
        .await
    }

    #[doc = "Get guide info\n\nSends a `GET` request to `/LiveTv/GuideInfo`\n\n"]
    pub async fn get_guide_info(&self) -> Result<types::GuideInfo, Error> {
        self.request(reqwest::Method::GET, "/LiveTv/GuideInfo".into())
            .send()
            .await
    }

    #[doc = "Gets available live tv services\n\nSends a `GET` request to `/LiveTv/Info`\n\n"]
    pub async fn get_live_tv_info(&self) -> Result<types::LiveTvInfo, Error> {
        self.request(reqwest::Method::GET, "/LiveTv/Info".into())
            .send()
            .await
    }

    #[doc = "Adds a listings provider\n\nSends a `POST` request to `/LiveTv/ListingProviders`\n\nArguments:\n- `pw`: Password.\n- `validate_listings`: Validate listings.\n- `validate_login`: Validate login.\n- `body`: New listings info.\n"]
    pub async fn add_listing_provider(
        &self,
        pw: Option<&str>,
        validate_listings: Option<bool>,
        validate_login: Option<bool>,
        body: &types::ListingsProviderInfo,
    ) -> Result<types::ListingsProviderInfo, Error> {
        self.request(reqwest::Method::POST, "/LiveTv/ListingProviders".into())
            .query_opt("pw", pw)
            .query_opt("validateListings", validate_listings)
            .query_opt("validateLogin", validate_login)
            .json_body(body)
            .send()
            .await
    }

    #[doc = "Delete listing provider\n\nSends a `DELETE` request to `/LiveTv/ListingProviders`\n\nArguments:\n- `id`: Listing provider id.\n"]
    pub async fn delete_listing_provider(&self, id: Option<&str>) -> Result<(), Error> {
        self.request(reqwest::Method::DELETE, "/LiveTv/ListingProviders".into())
            .query_opt("id", id)
            .send_no_content()
            .await
    }

    #[doc = "Gets default listings provider info\n\nSends a `GET` request to `/LiveTv/ListingProviders/Default`\n\n"]
    pub async fn get_default_listing_provider(&self) -> Result<types::ListingsProviderInfo, Error> {
        self.request(
            reqwest::Method::GET,
            "/LiveTv/ListingProviders/Default".into(),
        )
        .send()
        .await
    }

    #[doc = "Gets available lineups\n\nSends a `GET` request to `/LiveTv/ListingProviders/Lineups`\n\nArguments:\n- `country`: Country.\n- `id`: Provider id.\n- `location`: Location.\n- `type_`: Provider type.\n"]
    pub async fn get_lineups(
        &self,
        country: Option<&str>,
        id: Option<&str>,
        location: Option<&str>,
        type_: Option<&str>,
    ) -> Result<Vec<types::NameIdPair>, Error> {
        self.request(
            reqwest::Method::GET,
            "/LiveTv/ListingProviders/Lineups".into(),
        )
        .query_opt("country", country)
        .query_opt("id", id)
        .query_opt("location", location)
        .query_opt("type", type_)
        .send()
        .await
    }

    #[doc = "Gets available countries\n\nSends a `GET` request to `/LiveTv/ListingProviders/SchedulesDirect/Countries`\n\n"]
    pub async fn get_schedules_direct_countries(&self) -> Result<serde_json::Value, Error> {
        self.request(
            reqwest::Method::GET,
            "/LiveTv/ListingProviders/SchedulesDirect/Countries".into(),
        )
        .send()
        .await
    }

    #[doc = "Gets a live tv recording stream\n\nSends a `GET` request to `/LiveTv/LiveRecordings/{recordingId}/stream`\n\nArguments:\n- `recording_id`: Recording id.\n"]
    pub async fn get_live_recording_file(
        &self,
        recording_id: &str,
    ) -> Result<reqwest::Response, Error> {
        self.request(
            reqwest::Method::GET,
            format!(
                "/LiveTv/LiveRecordings/{}/stream",
                encode_path(recording_id)
            ),
        )
        .send_response()
        .await
    }

    #[doc = "Gets a live tv channel stream\n\nSends a `GET` request to `/LiveTv/LiveStreamFiles/{streamId}/stream.{container}`\n\nArguments:\n- `stream_id`: Stream id.\n- `container`: Container type.\n"]
    pub async fn get_live_stream_file(
        &self,
        stream_id: &str,
        container: &types::GetLiveStreamFileContainer,
    ) -> Result<reqwest::Response, Error> {
        self.request(
            reqwest::Method::GET,
            format!(
                "/LiveTv/LiveStreamFiles/{}/stream.{}",
                encode_path(stream_id),
                encode_path(&container.to_string())
            ),
        )
        .send_response()
        .await
    }

    #[doc = "Gets available live tv epgs\n\nSends a `GET` request to `/LiveTv/Programs`\n\nArguments:\n- `channel_ids`: The channels to return guide information for.\n- `enable_image_types`: Optional. The image types to include in the output.\n- `enable_images`: Optional. Include image information in output.\n- `enable_total_record_count`: Retrieve total record count.\n- `enable_user_data`: Optional. Include user data.\n- `fields`: Optional. Specify additional fields of information to return in the output.\n- `genre_ids`: The genre ids to return guide information for.\n- `genres`: The genres to return guide information for.\n- `has_aired`: Optional. Filter by programs that have completed airing, or not.\n- `image_type_limit`: Optional. The max number of images to return, per image type.\n- `is_airing`: Optional. Filter by programs that are currently airing, or not.\n- `is_kids`: Optional. Filter for kids.\n- `is_movie`: Optional. Filter for movies.\n- `is_news`: Optional. Filter for news.\n- `is_series`: Optional. Filter for series.\n- `is_sports`: Optional. Filter for sports.\n- `library_series_id`: Optional. Filter by library series id.\n- `limit`: Optional. The maximum number of records to return.\n- `max_end_date`: Optional. The maximum premiere end date.\n- `max_start_date`: Optional. The maximum premiere start date.\n- `min_end_date`: Optional. The minimum premiere end date.\n- `min_start_date`: Optional. The minimum premiere start date.\n- `series_timer_id`: Optional. Filter by series timer id.\n- `sort_by`: Optional. Specify one or more sort orders, comma delimited. Options: Name, StartDate.\n- `sort_order`: Sort Order - Ascending,Descending.\n- `start_index`: Optional. The record index to start at. All items with a lower index will be dropped from the results.\n- `user_id`: Optional. Filter by user id.\n"]
    pub async fn get_live_tv_programs(
        &self,
        query: &query::GetLiveTvPrograms<'_>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(reqwest::Method::GET, "/LiveTv/Programs".into())
            .query_list_opt("channelIds", query.channel_ids)
            .query_list_opt("enableImageTypes", query.enable_image_types)
            .query_opt("enableImages", query.enable_images)
            .query_opt("enableTotalRecordCount", query.enable_total_record_count)
            .query_opt("enableUserData", query.enable_user_data)
            .query_list_opt("fields", query.fields)
            .query_list_opt("genreIds", query.genre_ids)
            .query_list_opt("genres", query.genres)
            .query_opt("hasAired", query.has_aired)
            .query_opt("imageTypeLimit", query.image_type_limit)
            .query_opt("isAiring", query.is_airing)
            .query_opt("isKids", query.is_kids)
            .query_opt("isMovie", query.is_movie)
            .query_opt("isNews", query.is_news)
            .query_opt("isSeries", query.is_series)
            .query_opt("isSports", query.is_sports)
            .query_opt("librarySeriesId", query.library_series_id)
            .query_opt("limit", query.limit)
            .query_opt("maxEndDate", query.max_end_date)
            .query_opt("maxStartDate", query.max_start_date)
            .query_opt("minEndDate", query.min_end_date)
            .query_opt("minStartDate", query.min_start_date)
            .query_opt("seriesTimerId", query.series_timer_id)
            .query_list_opt("sortBy", query.sort_by)
            .query_list_opt("sortOrder", query.sort_order)
            .query_opt("startIndex", query.start_index)
            .query_opt("userId", query.user_id)
            .send()
            .await
    }

    #[doc = "Gets available live tv epgs\n\nSends a `POST` request to `/LiveTv/Programs`\n\nArguments:\n- `body`: Request body.\n"]
    pub async fn get_programs(
        &self,
        body: &types::GetProgramsDto,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(reqwest::Method::POST, "/LiveTv/Programs".into())
            .json_body(body)
            .send()
            .await
    }

    #[doc = "Gets a live tv program\n\nSends a `GET` request to `/LiveTv/Programs/{programId}`\n\nArguments:\n- `program_id`: Program id.\n- `user_id`: Optional. Attach user data.\n"]
    pub async fn get_program(
        &self,
        program_id: &str,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::BaseItemDto, Error> {
        self.request(
            reqwest::Method::GET,
            format!("/LiveTv/Programs/{}", encode_path(program_id)),
        )
        .query_opt("userId", user_id)
        .send()
        .await
    }

    #[doc = "Gets recommended live tv epgs\n\nSends a `GET` request to `/LiveTv/Programs/Recommended`\n\nArguments:\n- `enable_image_types`: Optional. The image types to include in the output.\n- `enable_images`: Optional. Include image information in output.\n- `enable_total_record_count`: Retrieve total record count.\n- `enable_user_data`: Optional. include user data.\n- `fields`: Optional. Specify additional fields of information to return in the output.\n- `genre_ids`: The genres to return guide information for.\n- `has_aired`: Optional. Filter by programs that have completed airing, or not.\n- `image_type_limit`: Optional. The max number of images to return, per image type.\n- `is_airing`: Optional. Filter by programs that are currently airing, or not.\n- `is_kids`: Optional. Filter for kids.\n- `is_movie`: Optional. Filter for movies.\n- `is_news`: Optional. Filter for news.\n- `is_series`: Optional. Filter for series.\n- `is_sports`: Optional. Filter for sports.\n- `limit`: Optional. The maximum number of records to return.\n- `start_index`: Optional. The record index to start at. All items with a lower index will be dropped from the results.\n- `user_id`: Optional. filter by user id.\n"]
    pub async fn get_recommended_programs(
        &self,
        query: &query::GetRecommendedPrograms<'_>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(reqwest::Method::GET, "/LiveTv/Programs/Recommended".into())
            .query_list_opt("enableImageTypes", query.enable_image_types)
            .query_opt("enableImages", query.enable_images)
            .query_opt("enableTotalRecordCount", query.enable_total_record_count)
            .query_opt("enableUserData", query.enable_user_data)
            .query_list_opt("fields", query.fields)
            .query_list_opt("genreIds", query.genre_ids)
            .query_opt("hasAired", query.has_aired)
            .query_opt("imageTypeLimit", query.image_type_limit)
            .query_opt("isAiring", query.is_airing)
            .query_opt("isKids", query.is_kids)
            .query_opt("isMovie", query.is_movie)
            .query_opt("isNews", query.is_news)
            .query_opt("isSeries", query.is_series)
            .query_opt("isSports", query.is_sports)
            .query_opt("limit", query.limit)
            .query_opt("startIndex", query.start_index)
            .query_opt("userId", query.user_id)
            .send()
            .await
    }

    #[doc = "Gets live tv recordings\n\nSends a `GET` request to `/LiveTv/Recordings`\n\nArguments:\n- `channel_id`: Optional. Filter by channel id.\n- `enable_image_types`: Optional. The image types to include in the output.\n- `enable_images`: Optional. Include image information in output.\n- `enable_total_record_count`: Optional. Return total record count.\n- `enable_user_data`: Optional. Include user data.\n- `fields`: Optional. Specify additional fields of information to return in the output.\n- `image_type_limit`: Optional. The max number of images to return, per image type.\n- `is_in_progress`: Optional. Filter by recordings that are in progress, or not.\n- `is_kids`: Optional. Filter for kids.\n- `is_library_item`: Optional. Filter for is library item.\n- `is_movie`: Optional. Filter for movies.\n- `is_news`: Optional. Filter for news.\n- `is_series`: Optional. Filter for series.\n- `is_sports`: Optional. Filter for sports.\n- `limit`: Optional. The maximum number of records to return.\n- `series_timer_id`: Optional. Filter by recordings belonging to a series timer.\n- `start_index`: Optional. The record index to start at. All items with a lower index will be dropped from the results.\n- `status`: Optional. Filter by recording status.\n- `user_id`: Optional. Filter by user and attach user data.\n"]
    pub async fn get_recordings(
        &self,
        query: &query::GetRecordings<'_>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(reqwest::Method::GET, "/LiveTv/Recordings".into())
            .query_opt("channelId", query.channel_id)
            .query_list_opt("enableImageTypes", query.enable_image_types)
            .query_opt("enableImages", query.enable_images)
            .query_opt("enableTotalRecordCount", query.enable_total_record_count)
            .query_opt("enableUserData", query.enable_user_data)
            .query_list_opt("fields", query.fields)
            .query_opt("imageTypeLimit", query.image_type_limit)
            .query_opt("isInProgress", query.is_in_progress)
            .query_opt("isKids", query.is_kids)
            .query_opt("isLibraryItem", query.is_library_item)
            .query_opt("isMovie", query.is_movie)
            .query_opt("isNews", query.is_news)
            .query_opt("isSeries", query.is_series)
            .query_opt("isSports", query.is_sports)
            .query_opt("limit", query.limit)
            .query_opt("seriesTimerId", query.series_timer_id)
            .query_opt("startIndex", query.start_index)
            .query_opt("status", query.status)
            .query_opt("userId", query.user_id)
            .send()
            .await
    }

    #[doc = "Gets a live tv recording\n\nSends a `GET` request to `/LiveTv/Recordings/{recordingId}`\n\nArguments:\n- `recording_id`: Recording id.\n- `user_id`: Optional. Attach user data.\n"]
    pub async fn get_recording(
        &self,
        recording_id: &uuid::Uuid,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::BaseItemDto, Error> {
        self.request(
            reqwest::Method::GET,
            format!(
                "/LiveTv/Recordings/{}",
                encode_path(&recording_id.to_string())
            ),
        )
        .query_opt("userId", user_id)
        .send()
        .await
    }

    #[doc = "Deletes a live tv recording\n\nSends a `DELETE` request to `/LiveTv/Recordings/{recordingId}`\n\nArguments:\n- `recording_id`: Recording id.\n"]
    pub async fn delete_recording(&self, recording_id: &uuid::Uuid) -> Result<(), Error> {
        self.request(
            reqwest::Method::DELETE,
            format!(
                "/LiveTv/Recordings/{}",
                encode_path(&recording_id.to_string())
            ),
        )
        .send_no_content()
        .await
    }

    #[doc = "Gets recording folders\n\nSends a `GET` request to `/LiveTv/Recordings/Folders`\n\nArguments:\n- `user_id`: Optional. Filter by user and attach user data.\n"]
    pub async fn get_recording_folders(
        &self,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(reqwest::Method::GET, "/LiveTv/Recordings/Folders".into())
            .query_opt("userId", user_id)
            .send()
            .await
    }

    #[doc = "Gets live tv recording groups\n\nSends a `GET` request to `/LiveTv/Recordings/Groups`\n\nArguments:\n- `user_id`: Optional. Filter by user and attach user data.\n"]
    pub async fn get_recording_groups(
        &self,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(reqwest::Method::GET, "/LiveTv/Recordings/Groups".into())
            .query_opt("userId", user_id)
            .send()
            .await
    }

    #[doc = "Get recording group\n\nSends a `GET` request to `/LiveTv/Recordings/Groups/{groupId}`\n\nArguments:\n- `group_id`: Group id.\n"]
    pub async fn get_recording_group(
        &self,
        group_id: &uuid::Uuid,
    ) -> Result<reqwest::Response, Error> {
        self.request(
            reqwest::Method::GET,
            format!(
                "/LiveTv/Recordings/Groups/{}",
                encode_path(&group_id.to_string())
            ),
        )
        .send_response()
        .await
    }

    #[doc = "Gets live tv recording series\n\nSends a `GET` request to `/LiveTv/Recordings/Series`\n\nArguments:\n- `channel_id`: Optional. Filter by channel id.\n- `enable_image_types`: Optional. The image types to include in the output.\n- `enable_images`: Optional. Include image information in output.\n- `enable_total_record_count`: Optional. Return total record count.\n- `enable_user_data`: Optional. Include user data.\n- `fields`: Optional. Specify additional fields of information to return in the output.\n- `group_id`: Optional. Filter by recording group.\n- `image_type_limit`: Optional. The max number of images to return, per image type.\n- `is_in_progress`: Optional. Filter by recordings that are in progress, or not.\n- `limit`: Optional. The maximum number of records to return.\n- `series_timer_id`: Optional. Filter by recordings belonging to a series timer.\n- `start_index`: Optional. The record index to start at. All items with a lower index will be dropped from the results.\n- `status`: Optional. Filter by recording status.\n- `user_id`: Optional. Filter by user and attach user data.\n"]
    pub async fn get_recordings_series(
        &self,
        query: &query::GetRecordingsSeries<'_>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(reqwest::Method::GET, "/LiveTv/Recordings/Series".into())
            .query_opt("channelId", query.channel_id)
            .query_list_opt("enableImageTypes", query.enable_image_types)
            .query_opt("enableImages", query.enable_images)
            .query_opt("enableTotalRecordCount", query.enable_total_record_count)
            .query_opt("enableUserData", query.enable_user_data)
            .query_list_opt("fields", query.fields)
            .query_opt("groupId", query.group_id)
            .query_opt("imageTypeLimit", query.image_type_limit)
            .query_opt("isInProgress", query.is_in_progress)
            .query_opt("limit", query.limit)
            .query_opt("seriesTimerId", query.series_timer_id)
            .query_opt("startIndex", query.start_index)
            .query_opt("status", query.status)
            .query_opt("userId", query.user_id)
            .send()
            .await
    }

    #[doc = "Gets live tv series timers\n\nSends a `GET` request to `/LiveTv/SeriesTimers`\n\nArguments:\n- `sort_by`: Optional. Sort by SortName or Priority.\n- `sort_order`: Optional. Sort in Ascending or Descending order.\n"]
    pub async fn get_series_timers(
        &self,
        sort_by: Option<&str>,
        sort_order: Option<types::SortOrder>,
    ) -> Result<types::SeriesTimerInfoDtoQueryResult, Error> {
        self.request(reqwest::Method::GET, "/LiveTv/SeriesTimers".into())
            .query_opt("sortBy", sort_by)
            .query_opt("sortOrder", sort_order)
            .send()
            .await
    }

    #[doc = "Creates a live tv series timer\n\nSends a `POST` request to `/LiveTv/SeriesTimers`\n\nArguments:\n- `body`: New series timer info.\n"]
    pub async fn create_series_timer(&self, body: &types::SeriesTimerInfoDto) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/LiveTv/SeriesTimers".into())
            .json_body(body)
            .send_no_content()
            .await
    }

    #[doc = "Gets a live tv series timer\n\nSends a `GET` request to `/LiveTv/SeriesTimers/{timerId}`\n\nArguments:\n- `timer_id`: Timer id.\n"]
    pub async fn get_series_timer(
        &self,
        timer_id: &str,
    ) -> Result<types::SeriesTimerInfoDto, Error> {
        self.request(
            reqwest::Method::GET,
            format!("/LiveTv/SeriesTimers/{}", encode_path(timer_id)),
        )
        .send()
        .await
    }

    #[doc = "Updates a live tv series timer\n\nSends a `POST` request to `/LiveTv/SeriesTimers/{timerId}`\n\nArguments:\n- `timer_id`: Timer id.\n- `body`: New series timer info.\n"]
    pub async fn update_series_timer(
        &self,
        timer_id: &str,
        body: &types::SeriesTimerInfoDto,
    ) -> Result<(), Error> {
        self.request(
            reqwest::Method::POST,
            format!("/LiveTv/SeriesTimers/{}", encode_path(timer_id)),
        )
        .json_body(body)
        .send_no_content()
        .await
    }

    #[doc = "Cancels a live tv series timer\n\nSends a `DELETE` request to `/LiveTv/SeriesTimers/{timerId}`\n\nArguments:\n- `timer_id`: Timer id.\n"]
    pub async fn cancel_series_timer(&self, timer_id: &str) -> Result<(), Error> {
        self.request(
            reqwest::Method::DELETE,
            format!("/LiveTv/SeriesTimers/{}", encode_path(timer_id)),
        )
        .send_no_content()
        .await
    }

    #[doc = "Gets the live tv timers\n\nSends a `GET` request to `/LiveTv/Timers`\n\nArguments:\n- `channel_id`: Optional. Filter by channel id.\n- `is_active`: Optional. Filter by timers that are active.\n- `is_scheduled`: Optional. Filter by timers that are scheduled.\n- `series_timer_id`: Optional. Filter by timers belonging to a series timer.\n"]
    pub async fn get_timers(
        &self,
        channel_id: Option<&str>,
        is_active: Option<bool>,
        is_scheduled: Option<bool>,
        series_timer_id: Option<&str>,
    ) -> Result<types::TimerInfoDtoQueryResult, Error> {
        self.request(reqwest::Method::GET, "/LiveTv/Timers".into())
            .query_opt("channelId", channel_id)
            .query_opt("isActive", is_active)
            .query_opt("isScheduled", is_scheduled)
            .query_opt("seriesTimerId", series_timer_id)
            .send()
            .await
    }

    #[doc = "Creates a live tv timer\n\nSends a `POST` request to `/LiveTv/Timers`\n\nArguments:\n- `body`: New timer info.\n"]
    pub async fn create_timer(&self, body: &types::TimerInfoDto) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/LiveTv/Timers".into())
            .json_body(body)
            .send_no_content()
            .await
    }

    #[doc = "Gets a timer\n\nSends a `GET` request to `/LiveTv/Timers/{timerId}`\n\nArguments:\n- `timer_id`: Timer id.\n"]
    pub async fn get_timer(&self, timer_id: &str) -> Result<types::TimerInfoDto, Error> {
        self.request(
            reqwest::Method::GET,
            format!("/LiveTv/Timers/{}", encode_path(timer_id)),
        )
        .send()
        .await
    }

    #[doc = "Updates a live tv timer\n\nSends a `POST` request to `/LiveTv/Timers/{timerId}`\n\nArguments:\n- `timer_id`: Timer id.\n- `body`: New timer info.\n"]
    pub async fn update_timer(
        &self,
        timer_id: &str,
        body: &types::TimerInfoDto,
    ) -> Result<(), Error> {
        self.request(
            reqwest::Method::POST,
            format!("/LiveTv/Timers/{}", encode_path(timer_id)),
        )
        .json_body(body)
        .send_no_content()
        .await
    }

    #[doc = "Cancels a live tv timer\n\nSends a `DELETE` request to `/LiveTv/Timers/{timerId}`\n\nArguments:\n- `timer_id`: Timer id.\n"]
    pub async fn cancel_timer(&self, timer_id: &str) -> Result<(), Error> {
        self.request(
            reqwest::Method::DELETE,
            format!("/LiveTv/Timers/{}", encode_path(timer_id)),
        )
        .send_no_content()
        .await
    }

    #[doc = "Gets the default values for a new timer\n\nSends a `GET` request to `/LiveTv/Timers/Defaults`\n\nArguments:\n- `program_id`: Optional. To attach default values based on a program.\n"]
    pub async fn get_default_timer(
        &self,
        program_id: Option<&str>,
    ) -> Result<types::SeriesTimerInfoDto, Error> {
        self.request(reqwest::Method::GET, "/LiveTv/Timers/Defaults".into())
            .query_opt("programId", program_id)
            .send()
            .await
    }

    #[doc = "Adds a tuner host\n\nSends a `POST` request to `/LiveTv/TunerHosts`\n\nArguments:\n- `body`: New tuner host.\n"]
    pub async fn add_tuner_host(
        &self,
        body: &types::TunerHostInfo,
    ) -> Result<types::TunerHostInfo, Error> {
        self.request(reqwest::Method::POST, "/LiveTv/TunerHosts".into())
            .json_body(body)
            .send()
            .await
    }

    #[doc = "Deletes a tuner host\n\nSends a `DELETE` request to `/LiveTv/TunerHosts`\n\nArguments:\n- `id`: Tuner host id.\n"]
    pub async fn delete_tuner_host(&self, id: Option<&str>) -> Result<(), Error> {
        self.request(reqwest::Method::DELETE, "/LiveTv/TunerHosts".into())
            .query_opt("id", id)
            .send_no_content()
            .await
    }

    #[doc = "Get tuner host types\n\nSends a `GET` request to `/LiveTv/TunerHosts/Types`\n\n"]
    pub async fn get_tuner_host_types(&self) -> Result<Vec<types::NameIdPair>, Error> {
        self.request(reqwest::Method::GET, "/LiveTv/TunerHosts/Types".into())
            .send()
            .await
    }

    #[doc = "Resets a tv tuner\n\nSends a `POST` request to `/LiveTv/Tuners/{tunerId}/Reset`\n\nArguments:\n- `tuner_id`: Tuner id.\n"]
    pub async fn reset_tuner(&self, tuner_id: &str) -> Result<(), Error> {
        self.request(
            reqwest::Method::POST,
            format!("/LiveTv/Tuners/{}/Reset", encode_path(tuner_id)),
        )
        .send_no_content()
        .await
    }

    #[doc = "Discover tuners\n\nSends a `GET` request to `/LiveTv/Tuners/Discover`\n\nArguments:\n- `new_devices_only`: Only discover new tuners.\n"]
    pub async fn discover_tuners(
        &self,
        new_devices_only: Option<bool>,
    ) -> Result<Vec<types::TunerHostInfo>, Error> {
        self.request(reqwest::Method::GET, "/LiveTv/Tuners/Discover".into())
            .query_opt("newDevicesOnly", new_devices_only)
            .send()
            .await
    }

    #[doc = "Discover tuners\n\nSends a `GET` request to `/LiveTv/Tuners/Discvover`\n\nArguments:\n- `new_devices_only`: Only discover new tuners.\n"]
    pub async fn discvover_tuners(
        &self,
        new_devices_only: Option<bool>,
    ) -> Result<Vec<types::TunerHostInfo>, Error> {
        self.request(reqwest::Method::GET, "/LiveTv/Tuners/Discvover".into())
            .query_opt("newDevicesOnly", new_devices_only)
            .send()
            .await
    }
}
