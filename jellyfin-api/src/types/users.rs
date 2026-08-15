use super::*;

#[doc = "An entity representing a user's access schedule."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AccessSchedule {
    #[serde(rename = "DayOfWeek", default, skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<DynamicDayOfWeek>,
    #[serde(rename = "EndHour", default, skip_serializing_if = "Option::is_none")]
    pub end_hour: Option<f64>,
    #[doc = "Gets the id of this instance."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "StartHour", default, skip_serializing_if = "Option::is_none")]
    pub start_hour: Option<f64>,
    #[doc = "Gets the id of the associated user."]
    #[serde(rename = "UserId", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<uuid::Uuid>,
}

impl Default for AccessSchedule {
    fn default() -> Self {
        Self {
            day_of_week: Default::default(),
            end_hour: Default::default(),
            id: Default::default(),
            start_hour: Default::default(),
            user_id: Default::default(),
        }
    }
}

#[doc = "The create user by name request body."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct CreateUserByName {
    #[doc = "Gets or sets the username."]
    #[serde(rename = "Name")]
    pub name: String,
    #[doc = "Gets or sets the password."]
    #[serde(rename = "Password", default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

#[doc = "`PinRedeemResult`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct PinRedeemResult {
    #[doc = "Gets or sets a value indicating whether this MediaBrowser.Model.Users.PinRedeemResult is success."]
    #[serde(rename = "Success", default, skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[doc = "Gets or sets the users reset."]
    #[serde(rename = "UsersReset", default, skip_serializing_if = "Vec::is_empty")]
    pub users_reset: Vec<String>,
}

impl Default for PinRedeemResult {
    fn default() -> Self {
        Self {
            success: Default::default(),
            users_reset: Default::default(),
        }
    }
}

#[doc = "The update user password request body."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct UpdateUserPassword {
    #[doc = "Gets or sets the current sha1-hashed password."]
    #[serde(
        rename = "CurrentPassword",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub current_password: Option<String>,
    #[doc = "Gets or sets the current plain text password."]
    #[serde(rename = "CurrentPw", default, skip_serializing_if = "Option::is_none")]
    pub current_pw: Option<String>,
    #[doc = "Gets or sets the new plain text password."]
    #[serde(rename = "NewPw", default, skip_serializing_if = "Option::is_none")]
    pub new_pw: Option<String>,
    #[doc = "Gets or sets a value indicating whether to reset the password."]
    #[serde(
        rename = "ResetPassword",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reset_password: Option<bool>,
}

impl Default for UpdateUserPassword {
    fn default() -> Self {
        Self {
            current_password: Default::default(),
            current_pw: Default::default(),
            new_pw: Default::default(),
            reset_password: Default::default(),
        }
    }
}

#[doc = "User deleted message."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct UserDeletedMessage {
    #[doc = "Gets or sets the data."]
    #[serde(rename = "Data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<uuid::Uuid>,
    #[doc = "Gets or sets the message id."]
    #[serde(rename = "MessageId", default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<uuid::Uuid>,
    #[serde(
        rename = "MessageType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_type: Option<SessionMessageType>,
}

impl Default for UserDeletedMessage {
    fn default() -> Self {
        Self {
            data: Default::default(),
            message_id: Default::default(),
            message_type: Default::default(),
        }
    }
}

#[doc = "Class UserDto."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct UserDto {
    #[doc = "Gets or sets the configuration."]
    #[serde(
        rename = "Configuration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub configuration: Option<UserConfiguration>,
    #[doc = "Gets or sets whether async login is enabled or not."]
    #[serde(
        rename = "EnableAutoLogin",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_auto_login: Option<bool>,
    #[doc = "Gets or sets a value indicating whether this instance has configured easy password."]
    #[serde(
        rename = "HasConfiguredEasyPassword",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub has_configured_easy_password: Option<bool>,
    #[doc = "Gets or sets a value indicating whether this instance has configured password."]
    #[serde(
        rename = "HasConfiguredPassword",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub has_configured_password: Option<bool>,
    #[doc = "Gets or sets a value indicating whether this instance has password."]
    #[serde(
        rename = "HasPassword",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub has_password: Option<bool>,
    #[doc = "Gets or sets the id."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[doc = "Gets or sets the last activity date."]
    #[serde(
        rename = "LastActivityDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_activity_date: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets the last login date."]
    #[serde(
        rename = "LastLoginDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_login_date: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets the name."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the policy."]
    #[serde(rename = "Policy", default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<UserPolicy>,
    #[doc = "Gets or sets the primary image aspect ratio."]
    #[serde(
        rename = "PrimaryImageAspectRatio",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_image_aspect_ratio: Option<f64>,
    #[doc = "Gets or sets the primary image tag."]
    #[serde(
        rename = "PrimaryImageTag",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_image_tag: Option<String>,
    #[doc = "Gets or sets the server identifier."]
    #[serde(rename = "ServerId", default, skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    #[doc = "Gets or sets the name of the server.\r\nThis is not used by the server and is for client-side usage only."]
    #[serde(
        rename = "ServerName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub server_name: Option<String>,
}

impl Default for UserDto {
    fn default() -> Self {
        Self {
            configuration: Default::default(),
            enable_auto_login: Default::default(),
            has_configured_easy_password: Default::default(),
            has_configured_password: Default::default(),
            has_password: Default::default(),
            id: Default::default(),
            last_activity_date: Default::default(),
            last_login_date: Default::default(),
            name: Default::default(),
            policy: Default::default(),
            primary_image_aspect_ratio: Default::default(),
            primary_image_tag: Default::default(),
            server_id: Default::default(),
            server_name: Default::default(),
        }
    }
}

#[doc = "`UserPolicy`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct UserPolicy {
    #[serde(
        rename = "AccessSchedules",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub access_schedules: Option<Vec<AccessSchedule>>,
    #[serde(
        rename = "AllowedTags",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allowed_tags: Option<Vec<String>>,
    #[serde(rename = "AuthenticationProviderId")]
    pub authentication_provider_id: String,
    #[serde(
        rename = "BlockUnratedItems",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub block_unrated_items: Option<Vec<UnratedItem>>,
    #[serde(
        rename = "BlockedChannels",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub blocked_channels: Option<Vec<uuid::Uuid>>,
    #[serde(
        rename = "BlockedMediaFolders",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub blocked_media_folders: Option<Vec<uuid::Uuid>>,
    #[serde(
        rename = "BlockedTags",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub blocked_tags: Option<Vec<String>>,
    #[serde(
        rename = "EnableAllChannels",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_all_channels: Option<bool>,
    #[serde(
        rename = "EnableAllDevices",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_all_devices: Option<bool>,
    #[serde(
        rename = "EnableAllFolders",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_all_folders: Option<bool>,
    #[serde(
        rename = "EnableAudioPlaybackTranscoding",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_audio_playback_transcoding: Option<bool>,
    #[doc = "Gets or sets a value indicating whether this instance can manage collections."]
    #[serde(rename = "EnableCollectionManagement", default)]
    pub enable_collection_management: bool,
    #[serde(
        rename = "EnableContentDeletion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_content_deletion: Option<bool>,
    #[serde(
        rename = "EnableContentDeletionFromFolders",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_content_deletion_from_folders: Option<Vec<String>>,
    #[serde(
        rename = "EnableContentDownloading",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_content_downloading: Option<bool>,
    #[serde(
        rename = "EnableLiveTvAccess",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_live_tv_access: Option<bool>,
    #[serde(
        rename = "EnableLiveTvManagement",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_live_tv_management: Option<bool>,
    #[doc = "Gets or sets a value indicating whether this user can manage lyrics."]
    #[serde(rename = "EnableLyricManagement", default)]
    pub enable_lyric_management: bool,
    #[serde(
        rename = "EnableMediaConversion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_media_conversion: Option<bool>,
    #[serde(
        rename = "EnableMediaPlayback",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_media_playback: Option<bool>,
    #[serde(
        rename = "EnablePlaybackRemuxing",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_playback_remuxing: Option<bool>,
    #[serde(
        rename = "EnablePublicSharing",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_public_sharing: Option<bool>,
    #[serde(
        rename = "EnableRemoteAccess",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_remote_access: Option<bool>,
    #[serde(
        rename = "EnableRemoteControlOfOtherUsers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_remote_control_of_other_users: Option<bool>,
    #[serde(
        rename = "EnableSharedDeviceControl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_shared_device_control: Option<bool>,
    #[doc = "Gets or sets a value indicating whether this instance can manage subtitles."]
    #[serde(rename = "EnableSubtitleManagement", default)]
    pub enable_subtitle_management: bool,
    #[doc = "Gets or sets a value indicating whether [enable synchronize]."]
    #[serde(
        rename = "EnableSyncTranscoding",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_sync_transcoding: Option<bool>,
    #[serde(
        rename = "EnableUserPreferenceAccess",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_user_preference_access: Option<bool>,
    #[serde(
        rename = "EnableVideoPlaybackTranscoding",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_video_playback_transcoding: Option<bool>,
    #[serde(
        rename = "EnabledChannels",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enabled_channels: Option<Vec<uuid::Uuid>>,
    #[serde(
        rename = "EnabledDevices",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enabled_devices: Option<Vec<String>>,
    #[serde(
        rename = "EnabledFolders",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enabled_folders: Option<Vec<uuid::Uuid>>,
    #[serde(
        rename = "ForceRemoteSourceTranscoding",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub force_remote_source_transcoding: Option<bool>,
    #[serde(
        rename = "InvalidLoginAttemptCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub invalid_login_attempt_count: Option<i32>,
    #[doc = "Gets or sets a value indicating whether this instance is administrator."]
    #[serde(
        rename = "IsAdministrator",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_administrator: Option<bool>,
    #[doc = "Gets or sets a value indicating whether this instance is disabled."]
    #[serde(
        rename = "IsDisabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_disabled: Option<bool>,
    #[doc = "Gets or sets a value indicating whether this instance is hidden."]
    #[serde(rename = "IsHidden", default, skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
    #[serde(
        rename = "LoginAttemptsBeforeLockout",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub login_attempts_before_lockout: Option<i32>,
    #[serde(
        rename = "MaxActiveSessions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_active_sessions: Option<i32>,
    #[doc = "Gets or sets the max parental rating."]
    #[serde(
        rename = "MaxParentalRating",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_parental_rating: Option<i32>,
    #[serde(
        rename = "MaxParentalSubRating",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_parental_sub_rating: Option<i32>,
    #[serde(rename = "PasswordResetProviderId")]
    pub password_reset_provider_id: String,
    #[serde(
        rename = "RemoteClientBitrateLimit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub remote_client_bitrate_limit: Option<i32>,
    #[serde(
        rename = "SyncPlayAccess",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sync_play_access: Option<SyncPlayUserAccessType>,
}

#[doc = "User updated message."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct UserUpdatedMessage {
    #[doc = "Class UserDto."]
    #[serde(rename = "Data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<UserDto>,
    #[doc = "Gets or sets the message id."]
    #[serde(rename = "MessageId", default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<uuid::Uuid>,
    #[serde(
        rename = "MessageType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_type: Option<SessionMessageType>,
}

impl Default for UserUpdatedMessage {
    fn default() -> Self {
        Self {
            data: Default::default(),
            message_id: Default::default(),
            message_type: Default::default(),
        }
    }
}
