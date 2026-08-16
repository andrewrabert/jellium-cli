use super::*;

#[doc = "A DTO representing device information."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct DeviceInfoDto {
    #[doc = "Gets or sets the access token."]
    #[serde(
        rename = "AccessToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub access_token: Option<String>,
    #[doc = "Gets or sets the name of the application."]
    #[serde(rename = "AppName", default, skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    #[doc = "Gets or sets the application version."]
    #[serde(
        rename = "AppVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub app_version: Option<String>,
    #[doc = "Gets or sets the capabilities."]
    #[serde(
        rename = "Capabilities",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub capabilities: Option<ClientCapabilitiesDto>,
    #[doc = "Gets or sets the custom name."]
    #[serde(
        rename = "CustomName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_name: Option<String>,
    #[doc = "Gets or sets the date last modified."]
    #[serde(
        rename = "DateLastActivity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub date_last_activity: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets the icon URL."]
    #[serde(rename = "IconUrl", default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[doc = "Gets or sets the identifier."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the last user identifier."]
    #[serde(
        rename = "LastUserId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_user_id: Option<uuid::Uuid>,
    #[doc = "Gets or sets the last name of the user."]
    #[serde(
        rename = "LastUserName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_user_name: Option<String>,
    #[doc = "Gets or sets the name."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[doc = "Query result container."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct DeviceInfoDtoQueryResult {
    #[doc = "Gets or sets the items."]
    #[serde(rename = "Items", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<DeviceInfoDto>,
    #[doc = "Gets or sets the index of the first record in Items."]
    #[serde(
        rename = "StartIndex",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_index: Option<i32>,
    #[doc = "Gets or sets the total number of records available."]
    #[serde(
        rename = "TotalRecordCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_record_count: Option<i32>,
}

#[doc = "A dto representing custom options for a device."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct DeviceOptionsDto {
    #[doc = "Gets or sets the custom name."]
    #[serde(
        rename = "CustomName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_name: Option<String>,
    #[doc = "Gets or sets the device id."]
    #[serde(rename = "DeviceId", default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[doc = "Gets or sets the id."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}
