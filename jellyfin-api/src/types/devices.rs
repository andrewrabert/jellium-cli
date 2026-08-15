use super::*;

#[doc = "A DTO representing device information."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
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

impl Default for DeviceInfoDto {
    fn default() -> Self {
        Self {
            access_token: Default::default(),
            app_name: Default::default(),
            app_version: Default::default(),
            capabilities: Default::default(),
            custom_name: Default::default(),
            date_last_activity: Default::default(),
            icon_url: Default::default(),
            id: Default::default(),
            last_user_id: Default::default(),
            last_user_name: Default::default(),
            name: Default::default(),
        }
    }
}

#[doc = "Query result container."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
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

impl Default for DeviceInfoDtoQueryResult {
    fn default() -> Self {
        Self {
            items: Default::default(),
            start_index: Default::default(),
            total_record_count: Default::default(),
        }
    }
}

#[doc = "A dto representing custom options for a device."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
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

impl Default for DeviceOptionsDto {
    fn default() -> Self {
        Self {
            custom_name: Default::default(),
            device_id: Default::default(),
            id: Default::default(),
        }
    }
}
