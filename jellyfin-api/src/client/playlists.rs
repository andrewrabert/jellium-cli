use crate::types;
use crate::error::Error;
use crate::util::encode_path;
use crate::Client;

impl Client {
    #[doc = "Creates an instant playlist based on a given playlist\n\nSends a `GET` request to `/Playlists/{itemId}/InstantMix`\n\nArguments:\n- `item_id`: The item id.\n- `enable_image_types`: Optional. The image types to include in the output.\n- `enable_images`: Optional. Include image information in output.\n- `enable_user_data`: Optional. Include user data.\n- `fields`: Optional. Specify additional fields of information to return in the output.\n- `image_type_limit`: Optional. The max number of images to return, per image type.\n- `limit`: Optional. The maximum number of records to return.\n- `user_id`: Optional. Filter by user id, and attach user data.\n"]
    pub async fn get_instant_mix_from_playlist(
        &self,
        item_id: &uuid::Uuid,
        enable_image_types: Option<&Vec<types::ImageType>>,
        enable_images: Option<bool>,
        enable_user_data: Option<bool>,
        fields: Option<&Vec<types::ItemFields>>,
        image_type_limit: Option<i32>,
        limit: Option<i32>,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(reqwest::Method::GET, format!("/Playlists/{}/InstantMix", encode_path(&item_id.to_string())))
            .query_list_opt("enableImageTypes", enable_image_types)
            .query_opt("enableImages", enable_images)
            .query_opt("enableUserData", enable_user_data)
            .query_list_opt("fields", fields)
            .query_opt("imageTypeLimit", image_type_limit)
            .query_opt("limit", limit)
            .query_opt("userId", user_id)
            .send()
            .await
    }

    #[doc = "Creates a new playlist\n\nFor backwards compatibility parameters can be sent via Query or Body, with Query having higher precedence.\r\nQuery parameters are obsolete.\n\nSends a `POST` request to `/Playlists`\n\nArguments:\n- `ids`: The item ids.\n- `media_type`: The media type.\n- `name`: The playlist name.\n- `user_id`: The user id.\n- `body`: The create playlist payload.\n"]
    pub async fn create_playlist(
        &self,
        ids: Option<&Vec<uuid::Uuid>>,
        media_type: Option<types::MediaType>,
        name: Option<&str>,
        user_id: Option<&uuid::Uuid>,
        body: &types::CreatePlaylistDto,
    ) -> Result<types::PlaylistCreationResult, Error> {
        self.request(reqwest::Method::POST, "/Playlists".into())
            .query_list_opt("ids", ids)
            .query_opt("mediaType", media_type)
            .query_opt("name", name)
            .query_opt("userId", user_id)
            .json_body(body)
            .send()
            .await
    }

    #[doc = "Get a playlist\n\nSends a `GET` request to `/Playlists/{playlistId}`\n\nArguments:\n- `playlist_id`: The playlist id.\n"]
    pub async fn get_playlist(
        &self,
        playlist_id: &uuid::Uuid,
    ) -> Result<types::PlaylistDto, Error> {
        self.request(reqwest::Method::GET, format!("/Playlists/{}", encode_path(&playlist_id.to_string())))
            .send()
            .await
    }

    #[doc = "Updates a playlist\n\nSends a `POST` request to `/Playlists/{playlistId}`\n\nArguments:\n- `playlist_id`: The playlist id.\n- `body`: The Jellyfin.Api.Models.PlaylistDtos.UpdatePlaylistDto id.\n"]
    pub async fn update_playlist(
        &self,
        playlist_id: &uuid::Uuid,
        body: &types::UpdatePlaylistDto,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, format!("/Playlists/{}", encode_path(&playlist_id.to_string())))
            .json_body(body)
            .send_no_content()
            .await
    }

    #[doc = "Gets the original items of a playlist\n\nSends a `GET` request to `/Playlists/{playlistId}/Items`\n\nArguments:\n- `playlist_id`: The playlist id.\n- `enable_image_types`: Optional. The image types to include in the output.\n- `enable_images`: Optional. Include image information in output.\n- `enable_user_data`: Optional. Include user data.\n- `fields`: Optional. Specify additional fields of information to return in the output.\n- `image_type_limit`: Optional. The max number of images to return, per image type.\n- `limit`: Optional. The maximum number of records to return.\n- `start_index`: Optional. The record index to start at. All items with a lower index will be dropped from the results.\n- `user_id`: User id.\n"]
    pub async fn get_playlist_items(
        &self,
        playlist_id: &uuid::Uuid,
        enable_image_types: Option<&Vec<types::ImageType>>,
        enable_images: Option<bool>,
        enable_user_data: Option<bool>,
        fields: Option<&Vec<types::ItemFields>>,
        image_type_limit: Option<i32>,
        limit: Option<i32>,
        start_index: Option<i32>,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(reqwest::Method::GET, format!("/Playlists/{}/Items", encode_path(&playlist_id.to_string())))
            .query_list_opt("enableImageTypes", enable_image_types)
            .query_opt("enableImages", enable_images)
            .query_opt("enableUserData", enable_user_data)
            .query_list_opt("fields", fields)
            .query_opt("imageTypeLimit", image_type_limit)
            .query_opt("limit", limit)
            .query_opt("startIndex", start_index)
            .query_opt("userId", user_id)
            .send()
            .await
    }

    #[doc = "Adds items to a playlist\n\nSends a `POST` request to `/Playlists/{playlistId}/Items`\n\nArguments:\n- `playlist_id`: The playlist id.\n- `ids`: Item id, comma delimited.\n- `user_id`: The userId.\n"]
    pub async fn add_item_to_playlist(
        &self,
        playlist_id: &uuid::Uuid,
        ids: Option<&Vec<uuid::Uuid>>,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, format!("/Playlists/{}/Items", encode_path(&playlist_id.to_string())))
            .query_list_opt("ids", ids)
            .query_opt("userId", user_id)
            .send_no_content()
            .await
    }

    #[doc = "Removes items from a playlist\n\nSends a `DELETE` request to `/Playlists/{playlistId}/Items`\n\nArguments:\n- `playlist_id`: The playlist id.\n- `entry_ids`: The item ids, comma delimited.\n"]
    pub async fn remove_item_from_playlist(
        &self,
        playlist_id: &str,
        entry_ids: Option<&Vec<String>>,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::DELETE, format!("/Playlists/{}/Items", encode_path(playlist_id)))
            .query_list_opt("entryIds", entry_ids)
            .send_no_content()
            .await
    }

    #[doc = "Moves a playlist item\n\nSends a `POST` request to `/Playlists/{playlistId}/Items/{itemId}/Move/{newIndex}`\n\nArguments:\n- `playlist_id`: The playlist id.\n- `item_id`: The item id.\n- `new_index`: The new index.\n"]
    pub async fn move_item(
        &self,
        playlist_id: &str,
        item_id: &str,
        new_index: i32,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, format!("/Playlists/{}/Items/{}/Move/{}", encode_path(playlist_id), encode_path(item_id), encode_path(&new_index.to_string())))
            .send_no_content()
            .await
    }

    #[doc = "Get a playlist's users\n\nSends a `GET` request to `/Playlists/{playlistId}/Users`\n\nArguments:\n- `playlist_id`: The playlist id.\n"]
    pub async fn get_playlist_users(
        &self,
        playlist_id: &uuid::Uuid,
    ) -> Result<Vec<types::PlaylistUserPermissions>, Error> {
        self.request(reqwest::Method::GET, format!("/Playlists/{}/Users", encode_path(&playlist_id.to_string())))
            .send()
            .await
    }

    #[doc = "Get a playlist user\n\nSends a `GET` request to `/Playlists/{playlistId}/Users/{userId}`\n\nArguments:\n- `playlist_id`: The playlist id.\n- `user_id`: The user id.\n"]
    pub async fn get_playlist_user(
        &self,
        playlist_id: &uuid::Uuid,
        user_id: &uuid::Uuid,
    ) -> Result<types::PlaylistUserPermissions, Error> {
        self.request(reqwest::Method::GET, format!("/Playlists/{}/Users/{}", encode_path(&playlist_id.to_string()), encode_path(&user_id.to_string())))
            .send()
            .await
    }

    #[doc = "Modify a user of a playlist's users\n\nSends a `POST` request to `/Playlists/{playlistId}/Users/{userId}`\n\nArguments:\n- `playlist_id`: The playlist id.\n- `user_id`: The user id.\n- `body`: The Jellyfin.Api.Models.PlaylistDtos.UpdatePlaylistUserDto.\n"]
    pub async fn update_playlist_user(
        &self,
        playlist_id: &uuid::Uuid,
        user_id: &uuid::Uuid,
        body: &types::UpdatePlaylistUserDto,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, format!("/Playlists/{}/Users/{}", encode_path(&playlist_id.to_string()), encode_path(&user_id.to_string())))
            .json_body(body)
            .send_no_content()
            .await
    }

    #[doc = "Remove a user from a playlist's users\n\nSends a `DELETE` request to `/Playlists/{playlistId}/Users/{userId}`\n\nArguments:\n- `playlist_id`: The playlist id.\n- `user_id`: The user id.\n"]
    pub async fn remove_user_from_playlist(
        &self,
        playlist_id: &uuid::Uuid,
        user_id: &uuid::Uuid,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::DELETE, format!("/Playlists/{}/Users/{}", encode_path(&playlist_id.to_string()), encode_path(&user_id.to_string())))
            .send_no_content()
            .await
    }
}
