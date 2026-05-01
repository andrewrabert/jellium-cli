use super::*;

#[doc = "The authenticate user by name request body."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AuthenticateUserByName {
    #[doc = "Gets or sets the plain text password."]
    #[serde(
        rename = "Pw",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pw: Option<String>,
    #[doc = "Gets or sets the username."]
    #[serde(
        rename = "Username",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub username: Option<String>,
}

impl Default for AuthenticateUserByName {
    fn default() -> Self {
        Self {
            pw: Default::default(),
            username: Default::default(),
        }
    }
}

#[doc = "`AuthenticationInfo`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AuthenticationInfo {
    #[doc = "Gets or sets the access token."]
    #[serde(
        rename = "AccessToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub access_token: Option<String>,
    #[doc = "Gets or sets the name of the application."]
    #[serde(
        rename = "AppName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub app_name: Option<String>,
    #[doc = "Gets or sets the application version."]
    #[serde(
        rename = "AppVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub app_version: Option<String>,
    #[doc = "Gets or sets the date created."]
    #[serde(
        rename = "DateCreated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub date_created: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        rename = "DateLastActivity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub date_last_activity: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets the date revoked."]
    #[serde(
        rename = "DateRevoked",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub date_revoked: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets the device identifier."]
    #[serde(
        rename = "DeviceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub device_id: Option<String>,
    #[doc = "Gets or sets the name of the device."]
    #[serde(
        rename = "DeviceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub device_name: Option<String>,
    #[doc = "Gets or sets the identifier."]
    #[serde(
        rename = "Id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<i64>,
    #[doc = "Gets or sets a value indicating whether this instance is active."]
    #[serde(
        rename = "IsActive",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_active: Option<bool>,
    #[doc = "Gets or sets the user identifier."]
    #[serde(
        rename = "UserId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_id: Option<uuid::Uuid>,
    #[serde(
        rename = "UserName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_name: Option<String>,
}

impl Default for AuthenticationInfo {
    fn default() -> Self {
        Self {
            access_token: Default::default(),
            app_name: Default::default(),
            app_version: Default::default(),
            date_created: Default::default(),
            date_last_activity: Default::default(),
            date_revoked: Default::default(),
            device_id: Default::default(),
            device_name: Default::default(),
            id: Default::default(),
            is_active: Default::default(),
            user_id: Default::default(),
            user_name: Default::default(),
        }
    }
}

#[doc = "Query result container."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AuthenticationInfoQueryResult {
    #[doc = "Gets or sets the items."]
    #[serde(
        rename = "Items",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<AuthenticationInfo>,
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

impl Default for AuthenticationInfoQueryResult {
    fn default() -> Self {
        Self {
            items: Default::default(),
            start_index: Default::default(),
            total_record_count: Default::default(),
        }
    }
}

#[doc = "A class representing an authentication result."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AuthenticationResult {
    #[doc = "Gets or sets the access token."]
    #[serde(
        rename = "AccessToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub access_token: Option<String>,
    #[doc = "Gets or sets the server id."]
    #[serde(
        rename = "ServerId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub server_id: Option<String>,
    #[doc = "Session info DTO."]
    #[serde(
        rename = "SessionInfo",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub session_info: Option<SessionInfoDto>,
    #[doc = "Class UserDto."]
    #[serde(
        rename = "User",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user: Option<UserDto>,
}

impl Default for AuthenticationResult {
    fn default() -> Self {
        Self {
            access_token: Default::default(),
            server_id: Default::default(),
            session_info: Default::default(),
            user: Default::default(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ForgotPasswordAction {
    ContactAdmin,
    PinCode,
    InNetworkRequired,
}

impl std::fmt::Display for ForgotPasswordAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::ContactAdmin => f.write_str("ContactAdmin"),
            Self::PinCode => f.write_str("PinCode"),
            Self::InNetworkRequired => f.write_str("InNetworkRequired"),
        }
    }
}

impl std::str::FromStr for ForgotPasswordAction {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "ContactAdmin" => Ok(Self::ContactAdmin),
            "PinCode" => Ok(Self::PinCode),
            "InNetworkRequired" => Ok(Self::InNetworkRequired),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for ForgotPasswordAction {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for ForgotPasswordAction {
    type Error = super::error::ConversionError;
    fn try_from(
        value: &String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for ForgotPasswordAction {
    type Error = super::error::ConversionError;
    fn try_from(
        value: String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "Forgot Password request body DTO."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ForgotPasswordDto {
    #[doc = "Gets or sets the entered username to have its password reset."]
    #[serde(rename = "EnteredUsername")]
    pub entered_username: String,
}

#[doc = "Forgot Password Pin enter request body DTO."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ForgotPasswordPinDto {
    #[doc = "Gets or sets the entered pin to have the password reset."]
    #[serde(rename = "Pin")]
    pub pin: String,
}

#[doc = "`ForgotPasswordResult`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ForgotPasswordResult {
    #[serde(
        rename = "Action",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action: Option<ForgotPasswordAction>,
    #[doc = "Gets or sets the pin expiration date."]
    #[serde(
        rename = "PinExpirationDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pin_expiration_date: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets the pin file."]
    #[serde(
        rename = "PinFile",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pin_file: Option<String>,
}

impl Default for ForgotPasswordResult {
    fn default() -> Self {
        Self {
            action: Default::default(),
            pin_expiration_date: Default::default(),
            pin_file: Default::default(),
        }
    }
}

#[doc = "The quick connect request body."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct QuickConnectDto {
    #[doc = "Gets or sets the quick connect secret."]
    #[serde(rename = "Secret")]
    pub secret: String,
}

#[doc = "Stores the state of an quick connect request."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct QuickConnectResult {
    #[doc = "Gets the requesting app name."]
    #[serde(
        rename = "AppName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub app_name: Option<String>,
    #[doc = "Gets the requesting app version."]
    #[serde(
        rename = "AppVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub app_version: Option<String>,
    #[doc = "Gets or sets a value indicating whether this request is authorized."]
    #[serde(
        rename = "Authenticated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub authenticated: Option<bool>,
    #[doc = "Gets the user facing code used so the user can quickly differentiate this request from others."]
    #[serde(
        rename = "Code",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub code: Option<String>,
    #[doc = "Gets or sets the DateTime that this request was created."]
    #[serde(
        rename = "DateAdded",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub date_added: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets the requesting device id."]
    #[serde(
        rename = "DeviceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub device_id: Option<String>,
    #[doc = "Gets the requesting device name."]
    #[serde(
        rename = "DeviceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub device_name: Option<String>,
    #[doc = "Gets the secret value used to uniquely identify this request. Can be used to retrieve authentication information."]
    #[serde(
        rename = "Secret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub secret: Option<String>,
}

impl Default for QuickConnectResult {
    fn default() -> Self {
        Self {
            app_name: Default::default(),
            app_version: Default::default(),
            authenticated: Default::default(),
            code: Default::default(),
            date_added: Default::default(),
            device_id: Default::default(),
            device_name: Default::default(),
            secret: Default::default(),
        }
    }
}

