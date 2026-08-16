use super::*;

#[doc = "Create new playlist dto."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct CreatePlaylistDto {
    #[doc = "Gets or sets item ids to add to the playlist."]
    #[serde(rename = "Ids", default, skip_serializing_if = "Vec::is_empty")]
    pub ids: Vec<uuid::Uuid>,
    #[doc = "Gets or sets a value indicating whether the playlist is public."]
    #[serde(rename = "IsPublic", default, skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    #[doc = "Gets or sets the media type."]
    #[serde(rename = "MediaType", default, skip_serializing_if = "Option::is_none")]
    pub media_type: Option<MediaType>,
    #[doc = "Gets or sets the name of the new playlist."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the user id."]
    #[serde(rename = "UserId", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<uuid::Uuid>,
    #[doc = "Gets or sets the playlist users."]
    #[serde(rename = "Users", default, skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<PlaylistUserPermissions>,
}

#[doc = "`PlaylistCreationResult`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct PlaylistCreationResult {
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[doc = "DTO for playlists."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct PlaylistDto {
    #[doc = "Gets or sets the item ids."]
    #[serde(rename = "ItemIds", default, skip_serializing_if = "Vec::is_empty")]
    pub item_ids: Vec<uuid::Uuid>,
    #[doc = "Gets or sets a value indicating whether the playlist is publicly readable."]
    #[serde(
        rename = "OpenAccess",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub open_access: Option<bool>,
    #[doc = "Gets or sets the share permissions."]
    #[serde(rename = "Shares", default, skip_serializing_if = "Vec::is_empty")]
    pub shares: Vec<PlaylistUserPermissions>,
}

#[doc = "Class to hold data on user permissions for playlists."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct PlaylistUserPermissions {
    #[doc = "Gets or sets a value indicating whether the user has edit permissions."]
    #[serde(rename = "CanEdit", default, skip_serializing_if = "Option::is_none")]
    pub can_edit: Option<bool>,
    #[doc = "Gets or sets the user id."]
    #[serde(rename = "UserId", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<uuid::Uuid>,
}

#[doc = "Update existing playlist dto. Fields set to `null` will not be updated and keep their current values."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct UpdatePlaylistDto {
    #[doc = "Gets or sets item ids of the playlist."]
    #[serde(rename = "Ids", default, skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<uuid::Uuid>>,
    #[doc = "Gets or sets a value indicating whether the playlist is public."]
    #[serde(rename = "IsPublic", default, skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    #[doc = "Gets or sets the name of the new playlist."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the playlist users."]
    #[serde(rename = "Users", default, skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<PlaylistUserPermissions>>,
}

#[doc = "Update existing playlist user dto. Fields set to `null` will not be updated and keep their current values."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct UpdatePlaylistUserDto {
    #[doc = "Gets or sets a value indicating whether the user can edit the playlist."]
    #[serde(rename = "CanEdit", default, skip_serializing_if = "Option::is_none")]
    pub can_edit: Option<bool>,
}
