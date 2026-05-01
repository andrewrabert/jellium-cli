use crate::types;
use crate::error::Error;
use crate::util::encode_path;
use crate::Client;

impl Client {
    #[doc = "Gets available channels\n\nSends a `GET` request to `/Channels`\n\nArguments:\n- `is_favorite`: Optional. Filter by channels that are favorite.\n- `limit`: Optional. The maximum number of records to return.\n- `start_index`: Optional. The record index to start at. All items with a lower index will be dropped from the results.\n- `supports_latest_items`: Optional. Filter by channels that support getting latest items.\n- `supports_media_deletion`: Optional. Filter by channels that support media deletion.\n- `user_id`: User Id to filter by. Use System.Guid.Empty to not filter by user.\n"]
    pub async fn get_channels(
        &self,
        is_favorite: Option<bool>,
        limit: Option<i32>,
        start_index: Option<i32>,
        supports_latest_items: Option<bool>,
        supports_media_deletion: Option<bool>,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(reqwest::Method::GET, "/Channels".into())
            .query_opt("isFavorite", is_favorite)
            .query_opt("limit", limit)
            .query_opt("startIndex", start_index)
            .query_opt("supportsLatestItems", supports_latest_items)
            .query_opt("supportsMediaDeletion", supports_media_deletion)
            .query_opt("userId", user_id)
            .send()
            .await
    }

    #[doc = "Get channel features\n\nSends a `GET` request to `/Channels/{channelId}/Features`\n\nArguments:\n- `channel_id`: Channel id.\n"]
    pub async fn get_channel_features(
        &self,
        channel_id: &uuid::Uuid,
    ) -> Result<types::ChannelFeatures, Error> {
        self.request(reqwest::Method::GET, format!("/Channels/{}/Features", encode_path(&channel_id.to_string())))
            .send()
            .await
    }

    #[doc = "Get channel items\n\nSends a `GET` request to `/Channels/{channelId}/Items`\n\nArguments:\n- `channel_id`: Channel Id.\n- `fields`: Optional. Specify additional fields of information to return in the output.\n- `filters`: Optional. Specify additional filters to apply.\n- `folder_id`: Optional. Folder Id.\n- `limit`: Optional. The maximum number of records to return.\n- `sort_by`: Optional. Specify one or more sort orders, comma delimited. Options: Album, AlbumArtist, Artist, Budget, CommunityRating, CriticRating, DateCreated, DatePlayed, PlayCount, PremiereDate, ProductionYear, SortName, Random, Revenue, Runtime.\n- `sort_order`: Optional. Sort Order - Ascending,Descending.\n- `start_index`: Optional. The record index to start at. All items with a lower index will be dropped from the results.\n- `user_id`: Optional. User Id.\n"]
    pub async fn get_channel_items(
        &self,
        channel_id: &uuid::Uuid,
        fields: Option<&Vec<types::ItemFields>>,
        filters: Option<&Vec<types::ItemFilter>>,
        folder_id: Option<&uuid::Uuid>,
        limit: Option<i32>,
        sort_by: Option<&Vec<types::ItemSortBy>>,
        sort_order: Option<&Vec<types::SortOrder>>,
        start_index: Option<i32>,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(reqwest::Method::GET, format!("/Channels/{}/Items", encode_path(&channel_id.to_string())))
            .query_list_opt("fields", fields)
            .query_list_opt("filters", filters)
            .query_opt("folderId", folder_id)
            .query_opt("limit", limit)
            .query_list_opt("sortBy", sort_by)
            .query_list_opt("sortOrder", sort_order)
            .query_opt("startIndex", start_index)
            .query_opt("userId", user_id)
            .send()
            .await
    }

    #[doc = "Get all channel features\n\nSends a `GET` request to `/Channels/Features`\n\n"]
    pub async fn get_all_channel_features(
        &self,
    ) -> Result<Vec<types::ChannelFeatures>, Error> {
        self.request(reqwest::Method::GET, "/Channels/Features".into())
            .send()
            .await
    }

    #[doc = "Gets latest channel items\n\nSends a `GET` request to `/Channels/Items/Latest`\n\nArguments:\n- `channel_ids`: Optional. Specify one or more channel id's, comma delimited.\n- `fields`: Optional. Specify additional fields of information to return in the output.\n- `filters`: Optional. Specify additional filters to apply.\n- `limit`: Optional. The maximum number of records to return.\n- `start_index`: Optional. The record index to start at. All items with a lower index will be dropped from the results.\n- `user_id`: Optional. User Id.\n"]
    pub async fn get_latest_channel_items(
        &self,
        channel_ids: Option<&Vec<uuid::Uuid>>,
        fields: Option<&Vec<types::ItemFields>>,
        filters: Option<&Vec<types::ItemFilter>>,
        limit: Option<i32>,
        start_index: Option<i32>,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(reqwest::Method::GET, "/Channels/Items/Latest".into())
            .query_list_opt("channelIds", channel_ids)
            .query_list_opt("fields", fields)
            .query_list_opt("filters", filters)
            .query_opt("limit", limit)
            .query_opt("startIndex", start_index)
            .query_opt("userId", user_id)
            .send()
            .await
    }
}
