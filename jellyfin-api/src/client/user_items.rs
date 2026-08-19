use crate::Client;
use crate::error::Error;
use crate::query;
use crate::types;
use crate::util::encode_path;

impl Client {
    #[doc = "Get Item User Data\n\nSends a `GET` request to `/UserItems/{itemId}/UserData`\n\nArguments:\n- `item_id`: The item id.\n- `user_id`: The user id.\n"]
    pub async fn get_item_user_data(
        &self,
        item_id: &uuid::Uuid,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::UserItemDataDto, Error> {
        self.request(
            reqwest::Method::GET,
            format!("/UserItems/{}/UserData", encode_path(&item_id.to_string())),
        )
        .query_opt("userId", user_id)
        .send()
        .await
    }

    #[doc = "Update Item User Data\n\nSends a `POST` request to `/UserItems/{itemId}/UserData`\n\nArguments:\n- `item_id`: The item id.\n- `user_id`: The user id.\n- `body`: New user data object.\n"]
    pub async fn update_item_user_data(
        &self,
        item_id: &uuid::Uuid,
        user_id: Option<&uuid::Uuid>,
        body: &types::UpdateUserItemDataDto,
    ) -> Result<types::UserItemDataDto, Error> {
        self.request(
            reqwest::Method::POST,
            format!("/UserItems/{}/UserData", encode_path(&item_id.to_string())),
        )
        .query_opt("userId", user_id)
        .json_body(body)
        .send()
        .await
    }

    #[doc = "Gets items based on a query\n\nSends a `GET` request to `/UserItems/Resume`\n\nArguments:\n- `enable_image_types`: Optional. The image types to include in the output.\n- `enable_images`: Optional. Include image information in output.\n- `enable_total_record_count`: Optional. Enable the total record count.\n- `enable_user_data`: Optional. Include user data.\n- `exclude_active_sessions`: Optional. Whether to exclude the currently active sessions.\n- `exclude_item_types`: Optional. If specified, results will be filtered based on item type. This allows multiple, comma delimited.\n- `fields`: Optional. Specify additional fields of information to return in the output. This allows multiple, comma delimited. Options: Budget, Chapters, DateCreated, Genres, HomePageUrl, IndexOptions, MediaStreams, Overview, ParentId, Path, People, ProviderIds, PrimaryImageAspectRatio, Revenue, SortName, Studios, Taglines.\n- `image_type_limit`: Optional. The max number of images to return, per image type.\n- `include_item_types`: Optional. If specified, results will be filtered based on the item type. This allows multiple, comma delimited.\n- `limit`: The item limit.\n- `media_types`: Optional. Filter by MediaType. Allows multiple, comma delimited.\n- `parent_id`: Specify this to localize the search to a specific item or folder. Omit to use the root.\n- `search_term`: The search term.\n- `start_index`: The start index.\n- `user_id`: The user id.\n"]
    pub async fn get_resume_items(
        &self,
        query: &query::GetResumeItems<'_>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(reqwest::Method::GET, "/UserItems/Resume".into())
            .query_list_opt("enableImageTypes", query.enable_image_types)
            .query_opt("enableImages", query.enable_images)
            .query_opt("enableTotalRecordCount", query.enable_total_record_count)
            .query_opt("enableUserData", query.enable_user_data)
            .query_opt("excludeActiveSessions", query.exclude_active_sessions)
            .query_list_opt("excludeItemTypes", query.exclude_item_types)
            .query_list_opt("fields", query.fields)
            .query_opt("imageTypeLimit", query.image_type_limit)
            .query_list_opt("includeItemTypes", query.include_item_types)
            .query_opt("limit", query.limit)
            .query_list_opt("mediaTypes", query.media_types)
            .query_opt("parentId", query.parent_id)
            .query_opt("searchTerm", query.search_term)
            .query_opt("startIndex", query.start_index)
            .query_opt("userId", query.user_id)
            .send()
            .await
    }

    #[doc = "Marks an item as played for user\n\nSends a `POST` request to `/UserPlayedItems/{itemId}`\n\nArguments:\n- `item_id`: Item id.\n- `date_played`: Optional. The date the item was played.\n- `user_id`: User id.\n"]
    pub async fn mark_played_item(
        &self,
        item_id: &uuid::Uuid,
        date_played: Option<types::Timestamp>,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::UserItemDataDto, Error> {
        self.request(
            reqwest::Method::POST,
            format!("/UserPlayedItems/{}", encode_path(&item_id.to_string())),
        )
        .query_opt("datePlayed", date_played)
        .query_opt("userId", user_id)
        .send()
        .await
    }

    #[doc = "Marks an item as unplayed for user\n\nSends a `DELETE` request to `/UserPlayedItems/{itemId}`\n\nArguments:\n- `item_id`: Item id.\n- `user_id`: User id.\n"]
    pub async fn mark_unplayed_item(
        &self,
        item_id: &uuid::Uuid,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::UserItemDataDto, Error> {
        self.request(
            reqwest::Method::DELETE,
            format!("/UserPlayedItems/{}", encode_path(&item_id.to_string())),
        )
        .query_opt("userId", user_id)
        .send()
        .await
    }

    #[doc = "Marks an item as a favorite\n\nSends a `POST` request to `/UserFavoriteItems/{itemId}`\n\nArguments:\n- `item_id`: Item id.\n- `user_id`: User id.\n"]
    pub async fn mark_favorite_item(
        &self,
        item_id: &uuid::Uuid,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::UserItemDataDto, Error> {
        self.request(
            reqwest::Method::POST,
            format!("/UserFavoriteItems/{}", encode_path(&item_id.to_string())),
        )
        .query_opt("userId", user_id)
        .send()
        .await
    }

    #[doc = "Unmarks item as a favorite\n\nSends a `DELETE` request to `/UserFavoriteItems/{itemId}`\n\nArguments:\n- `item_id`: Item id.\n- `user_id`: User id.\n"]
    pub async fn unmark_favorite_item(
        &self,
        item_id: &uuid::Uuid,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::UserItemDataDto, Error> {
        self.request(
            reqwest::Method::DELETE,
            format!("/UserFavoriteItems/{}", encode_path(&item_id.to_string())),
        )
        .query_opt("userId", user_id)
        .send()
        .await
    }

    #[doc = "Updates a user's rating for an item\n\nSends a `POST` request to `/UserItems/{itemId}/Rating`\n\nArguments:\n- `item_id`: Item id.\n- `likes`: Whether this M:Jellyfin.Api.Controllers.UserLibraryController.UpdateUserItemRating(System.Nullable{System.Guid},System.Guid,System.Nullable{System.Boolean}) is likes.\n- `user_id`: User id.\n"]
    pub async fn update_user_item_rating(
        &self,
        item_id: &uuid::Uuid,
        likes: Option<bool>,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::UserItemDataDto, Error> {
        self.request(
            reqwest::Method::POST,
            format!("/UserItems/{}/Rating", encode_path(&item_id.to_string())),
        )
        .query_opt("likes", likes)
        .query_opt("userId", user_id)
        .send()
        .await
    }

    #[doc = "Deletes a user's saved personal rating for an item\n\nSends a `DELETE` request to `/UserItems/{itemId}/Rating`\n\nArguments:\n- `item_id`: Item id.\n- `user_id`: User id.\n"]
    pub async fn delete_user_item_rating(
        &self,
        item_id: &uuid::Uuid,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::UserItemDataDto, Error> {
        self.request(
            reqwest::Method::DELETE,
            format!("/UserItems/{}/Rating", encode_path(&item_id.to_string())),
        )
        .query_opt("userId", user_id)
        .send()
        .await
    }

    #[doc = "Get user views\n\nSends a `GET` request to `/UserViews`\n\nArguments:\n- `include_external_content`: Whether or not to include external views such as channels or live tv.\n- `include_hidden`: Whether or not to include hidden content.\n- `preset_views`: Preset views.\n- `user_id`: User id.\n"]
    pub async fn get_user_views(
        &self,
        include_external_content: Option<bool>,
        include_hidden: Option<bool>,
        preset_views: Option<&Vec<types::CollectionType>>,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(reqwest::Method::GET, "/UserViews".into())
            .query_opt("includeExternalContent", include_external_content)
            .query_opt("includeHidden", include_hidden)
            .query_list_opt("presetViews", preset_views)
            .query_opt("userId", user_id)
            .send()
            .await
    }

    #[doc = "Get user view grouping options\n\nSends a `GET` request to `/UserViews/GroupingOptions`\n\nArguments:\n- `user_id`: User id.\n"]
    pub async fn get_grouping_options(
        &self,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<Vec<types::SpecialViewOptionDto>, Error> {
        self.request(reqwest::Method::GET, "/UserViews/GroupingOptions".into())
            .query_opt("userId", user_id)
            .send()
            .await
    }
}
