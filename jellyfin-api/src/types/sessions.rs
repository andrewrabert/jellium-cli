use super::*;

#[doc = "Client capabilities dto."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ClientCapabilitiesDto {
    #[doc = "Gets or sets the app store url."]
    #[serde(
        rename = "AppStoreUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub app_store_url: Option<String>,
    #[doc = "Gets or sets the device profile."]
    #[serde(
        rename = "DeviceProfile",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub device_profile: Option<DeviceProfile>,
    #[doc = "Gets or sets the icon url."]
    #[serde(
        rename = "IconUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub icon_url: Option<String>,
    #[doc = "Gets or sets the list of playable media types."]
    #[serde(
        rename = "PlayableMediaTypes",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub playable_media_types: Vec<MediaType>,
    #[doc = "Gets or sets the list of supported commands."]
    #[serde(
        rename = "SupportedCommands",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_commands: Vec<GeneralCommandType>,
    #[doc = "Gets or sets a value indicating whether session supports media control."]
    #[serde(
        rename = "SupportsMediaControl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_media_control: Option<bool>,
    #[doc = "Gets or sets a value indicating whether session supports a persistent identifier."]
    #[serde(
        rename = "SupportsPersistentIdentifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_persistent_identifier: Option<bool>,
}

impl Default for ClientCapabilitiesDto {
    fn default() -> Self {
        Self {
            app_store_url: Default::default(),
            device_profile: Default::default(),
            icon_url: Default::default(),
            playable_media_types: Default::default(),
            supported_commands: Default::default(),
            supports_media_control: Default::default(),
            supports_persistent_identifier: Default::default(),
        }
    }
}

#[doc = "`GeneralCommand`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct GeneralCommand {
    #[serde(
        rename = "Arguments",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub arguments: std::collections::HashMap<
        String,
        Option<String>,
    >,
    #[serde(
        rename = "ControllingUserId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub controlling_user_id: Option<uuid::Uuid>,
    #[serde(
        rename = "Name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<GeneralCommandType>,
}

impl Default for GeneralCommand {
    fn default() -> Self {
        Self {
            arguments: Default::default(),
            controlling_user_id: Default::default(),
            name: Default::default(),
        }
    }
}

#[doc = "General command websocket message."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct GeneralCommandMessage {
    #[doc = "Gets or sets the data."]
    #[serde(
        rename = "Data",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data: Option<GeneralCommand>,
    #[doc = "Gets or sets the message id."]
    #[serde(
        rename = "MessageId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_id: Option<uuid::Uuid>,
    #[serde(
        rename = "MessageType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_type: Option<SessionMessageType>,
}

impl Default for GeneralCommandMessage {
    fn default() -> Self {
        Self {
            data: Default::default(),
            message_id: Default::default(),
            message_type: Default::default(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GeneralCommandType {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    PageUp,
    PageDown,
    PreviousLetter,
    NextLetter,
    ToggleOsd,
    ToggleContextMenu,
    Select,
    Back,
    TakeScreenshot,
    SendKey,
    SendString,
    GoHome,
    GoToSettings,
    VolumeUp,
    VolumeDown,
    Mute,
    Unmute,
    ToggleMute,
    SetVolume,
    SetAudioStreamIndex,
    SetSubtitleStreamIndex,
    ToggleFullscreen,
    DisplayContent,
    GoToSearch,
    DisplayMessage,
    SetRepeatMode,
    ChannelUp,
    ChannelDown,
    Guide,
    ToggleStats,
    PlayMediaSource,
    PlayTrailers,
    SetShuffleQueue,
    PlayState,
    PlayNext,
    ToggleOsdMenu,
    Play,
    SetMaxStreamingBitrate,
    SetPlaybackOrder,
}

impl std::fmt::Display for GeneralCommandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::MoveUp => f.write_str("MoveUp"),
            Self::MoveDown => f.write_str("MoveDown"),
            Self::MoveLeft => f.write_str("MoveLeft"),
            Self::MoveRight => f.write_str("MoveRight"),
            Self::PageUp => f.write_str("PageUp"),
            Self::PageDown => f.write_str("PageDown"),
            Self::PreviousLetter => f.write_str("PreviousLetter"),
            Self::NextLetter => f.write_str("NextLetter"),
            Self::ToggleOsd => f.write_str("ToggleOsd"),
            Self::ToggleContextMenu => f.write_str("ToggleContextMenu"),
            Self::Select => f.write_str("Select"),
            Self::Back => f.write_str("Back"),
            Self::TakeScreenshot => f.write_str("TakeScreenshot"),
            Self::SendKey => f.write_str("SendKey"),
            Self::SendString => f.write_str("SendString"),
            Self::GoHome => f.write_str("GoHome"),
            Self::GoToSettings => f.write_str("GoToSettings"),
            Self::VolumeUp => f.write_str("VolumeUp"),
            Self::VolumeDown => f.write_str("VolumeDown"),
            Self::Mute => f.write_str("Mute"),
            Self::Unmute => f.write_str("Unmute"),
            Self::ToggleMute => f.write_str("ToggleMute"),
            Self::SetVolume => f.write_str("SetVolume"),
            Self::SetAudioStreamIndex => f.write_str("SetAudioStreamIndex"),
            Self::SetSubtitleStreamIndex => f.write_str("SetSubtitleStreamIndex"),
            Self::ToggleFullscreen => f.write_str("ToggleFullscreen"),
            Self::DisplayContent => f.write_str("DisplayContent"),
            Self::GoToSearch => f.write_str("GoToSearch"),
            Self::DisplayMessage => f.write_str("DisplayMessage"),
            Self::SetRepeatMode => f.write_str("SetRepeatMode"),
            Self::ChannelUp => f.write_str("ChannelUp"),
            Self::ChannelDown => f.write_str("ChannelDown"),
            Self::Guide => f.write_str("Guide"),
            Self::ToggleStats => f.write_str("ToggleStats"),
            Self::PlayMediaSource => f.write_str("PlayMediaSource"),
            Self::PlayTrailers => f.write_str("PlayTrailers"),
            Self::SetShuffleQueue => f.write_str("SetShuffleQueue"),
            Self::PlayState => f.write_str("PlayState"),
            Self::PlayNext => f.write_str("PlayNext"),
            Self::ToggleOsdMenu => f.write_str("ToggleOsdMenu"),
            Self::Play => f.write_str("Play"),
            Self::SetMaxStreamingBitrate => f.write_str("SetMaxStreamingBitrate"),
            Self::SetPlaybackOrder => f.write_str("SetPlaybackOrder"),
        }
    }
}

impl std::str::FromStr for GeneralCommandType {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "MoveUp" => Ok(Self::MoveUp),
            "MoveDown" => Ok(Self::MoveDown),
            "MoveLeft" => Ok(Self::MoveLeft),
            "MoveRight" => Ok(Self::MoveRight),
            "PageUp" => Ok(Self::PageUp),
            "PageDown" => Ok(Self::PageDown),
            "PreviousLetter" => Ok(Self::PreviousLetter),
            "NextLetter" => Ok(Self::NextLetter),
            "ToggleOsd" => Ok(Self::ToggleOsd),
            "ToggleContextMenu" => Ok(Self::ToggleContextMenu),
            "Select" => Ok(Self::Select),
            "Back" => Ok(Self::Back),
            "TakeScreenshot" => Ok(Self::TakeScreenshot),
            "SendKey" => Ok(Self::SendKey),
            "SendString" => Ok(Self::SendString),
            "GoHome" => Ok(Self::GoHome),
            "GoToSettings" => Ok(Self::GoToSettings),
            "VolumeUp" => Ok(Self::VolumeUp),
            "VolumeDown" => Ok(Self::VolumeDown),
            "Mute" => Ok(Self::Mute),
            "Unmute" => Ok(Self::Unmute),
            "ToggleMute" => Ok(Self::ToggleMute),
            "SetVolume" => Ok(Self::SetVolume),
            "SetAudioStreamIndex" => Ok(Self::SetAudioStreamIndex),
            "SetSubtitleStreamIndex" => Ok(Self::SetSubtitleStreamIndex),
            "ToggleFullscreen" => Ok(Self::ToggleFullscreen),
            "DisplayContent" => Ok(Self::DisplayContent),
            "GoToSearch" => Ok(Self::GoToSearch),
            "DisplayMessage" => Ok(Self::DisplayMessage),
            "SetRepeatMode" => Ok(Self::SetRepeatMode),
            "ChannelUp" => Ok(Self::ChannelUp),
            "ChannelDown" => Ok(Self::ChannelDown),
            "Guide" => Ok(Self::Guide),
            "ToggleStats" => Ok(Self::ToggleStats),
            "PlayMediaSource" => Ok(Self::PlayMediaSource),
            "PlayTrailers" => Ok(Self::PlayTrailers),
            "SetShuffleQueue" => Ok(Self::SetShuffleQueue),
            "PlayState" => Ok(Self::PlayState),
            "PlayNext" => Ok(Self::PlayNext),
            "ToggleOsdMenu" => Ok(Self::ToggleOsdMenu),
            "Play" => Ok(Self::Play),
            "SetMaxStreamingBitrate" => Ok(Self::SetMaxStreamingBitrate),
            "SetPlaybackOrder" => Ok(Self::SetPlaybackOrder),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for GeneralCommandType {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GeneralCommandType {
    type Error = super::error::ConversionError;
    fn try_from(
        value: &String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GeneralCommandType {
    type Error = super::error::ConversionError;
    fn try_from(
        value: String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "`MessageCommand`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct MessageCommand {
    #[serde(
        rename = "Header",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub header: Option<String>,
    #[serde(rename = "Text")]
    pub text: String,
    #[serde(
        rename = "TimeoutMs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub timeout_ms: Option<i64>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PlayCommand {
    PlayNow,
    PlayNext,
    PlayLast,
    PlayInstantMix,
    PlayShuffle,
}

impl std::fmt::Display for PlayCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::PlayNow => f.write_str("PlayNow"),
            Self::PlayNext => f.write_str("PlayNext"),
            Self::PlayLast => f.write_str("PlayLast"),
            Self::PlayInstantMix => f.write_str("PlayInstantMix"),
            Self::PlayShuffle => f.write_str("PlayShuffle"),
        }
    }
}

impl std::str::FromStr for PlayCommand {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "PlayNow" => Ok(Self::PlayNow),
            "PlayNext" => Ok(Self::PlayNext),
            "PlayLast" => Ok(Self::PlayLast),
            "PlayInstantMix" => Ok(Self::PlayInstantMix),
            "PlayShuffle" => Ok(Self::PlayShuffle),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for PlayCommand {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for PlayCommand {
    type Error = super::error::ConversionError;
    fn try_from(
        value: &String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for PlayCommand {
    type Error = super::error::ConversionError;
    fn try_from(
        value: String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "`PlayerStateInfo`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct PlayerStateInfo {
    #[doc = "Gets or sets the index of the now playing audio stream."]
    #[serde(
        rename = "AudioStreamIndex",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub audio_stream_index: Option<i32>,
    #[doc = "Gets or sets a value indicating whether this instance can seek."]
    #[serde(
        rename = "CanSeek",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub can_seek: Option<bool>,
    #[doc = "Gets or sets a value indicating whether this instance is muted."]
    #[serde(
        rename = "IsMuted",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_muted: Option<bool>,
    #[doc = "Gets or sets a value indicating whether this instance is paused."]
    #[serde(
        rename = "IsPaused",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_paused: Option<bool>,
    #[doc = "Gets or sets the now playing live stream identifier."]
    #[serde(
        rename = "LiveStreamId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub live_stream_id: Option<String>,
    #[doc = "Gets or sets the now playing media version identifier."]
    #[serde(
        rename = "MediaSourceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub media_source_id: Option<String>,
    #[doc = "Gets or sets the play method."]
    #[serde(
        rename = "PlayMethod",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub play_method: Option<PlayMethod>,
    #[serde(
        rename = "PlaybackOrder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub playback_order: Option<PlaybackOrder>,
    #[doc = "Gets or sets the now playing position ticks."]
    #[serde(
        rename = "PositionTicks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub position_ticks: Option<i64>,
    #[serde(
        rename = "RepeatMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub repeat_mode: Option<RepeatMode>,
    #[doc = "Gets or sets the index of the now playing subtitle stream."]
    #[serde(
        rename = "SubtitleStreamIndex",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub subtitle_stream_index: Option<i32>,
    #[doc = "Gets or sets the volume level."]
    #[serde(
        rename = "VolumeLevel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub volume_level: Option<i32>,
}

impl Default for PlayerStateInfo {
    fn default() -> Self {
        Self {
            audio_stream_index: Default::default(),
            can_seek: Default::default(),
            is_muted: Default::default(),
            is_paused: Default::default(),
            live_stream_id: Default::default(),
            media_source_id: Default::default(),
            play_method: Default::default(),
            playback_order: Default::default(),
            position_ticks: Default::default(),
            repeat_mode: Default::default(),
            subtitle_stream_index: Default::default(),
            volume_level: Default::default(),
        }
    }
}

#[doc = "Session info DTO."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct SessionInfoDto {
    #[doc = "Gets or sets the additional users."]
    #[serde(
        rename = "AdditionalUsers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_users: Option<Vec<SessionUserInfo>>,
    #[doc = "Gets or sets the application version."]
    #[serde(
        rename = "ApplicationVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub application_version: Option<String>,
    #[doc = "Gets or sets the client capabilities."]
    #[serde(
        rename = "Capabilities",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub capabilities: Option<ClientCapabilitiesDto>,
    #[doc = "Gets or sets the type of the client."]
    #[serde(
        rename = "Client",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client: Option<String>,
    #[doc = "Gets or sets the device id."]
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
    #[doc = "Gets or sets the type of the device."]
    #[serde(
        rename = "DeviceType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub device_type: Option<String>,
    #[doc = "Gets or sets a value indicating whether the session has a custom device name."]
    #[serde(
        rename = "HasCustomDeviceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub has_custom_device_name: Option<bool>,
    #[doc = "Gets or sets the id."]
    #[serde(
        rename = "Id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<String>,
    #[doc = "Gets or sets a value indicating whether this session is active."]
    #[serde(
        rename = "IsActive",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_active: Option<bool>,
    #[doc = "Gets or sets the last activity date."]
    #[serde(
        rename = "LastActivityDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_activity_date: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets the last paused date."]
    #[serde(
        rename = "LastPausedDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_paused_date: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets the last playback check in."]
    #[serde(
        rename = "LastPlaybackCheckIn",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_playback_check_in:
        Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets the now playing item."]
    #[serde(
        rename = "NowPlayingItem",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub now_playing_item: Option<BaseItemDto>,
    #[doc = "Gets or sets the now playing queue."]
    #[serde(
        rename = "NowPlayingQueue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub now_playing_queue: Option<Vec<QueueItem>>,
    #[doc = "Gets or sets the now playing queue full items."]
    #[serde(
        rename = "NowPlayingQueueFullItems",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub now_playing_queue_full_items: Option<Vec<BaseItemDto>>,
    #[doc = "Gets or sets the now viewing item."]
    #[serde(
        rename = "NowViewingItem",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub now_viewing_item: Option<BaseItemDto>,
    #[doc = "Gets or sets the play state."]
    #[serde(
        rename = "PlayState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub play_state: Option<PlayerStateInfo>,
    #[doc = "Gets or sets the playable media types."]
    #[serde(
        rename = "PlayableMediaTypes",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub playable_media_types: Vec<MediaType>,
    #[doc = "Gets or sets the playlist item id."]
    #[serde(
        rename = "PlaylistItemId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub playlist_item_id: Option<String>,
    #[doc = "Gets or sets the remote end point."]
    #[serde(
        rename = "RemoteEndPoint",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub remote_end_point: Option<String>,
    #[doc = "Gets or sets the server id."]
    #[serde(
        rename = "ServerId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub server_id: Option<String>,
    #[doc = "Gets or sets the supported commands."]
    #[serde(
        rename = "SupportedCommands",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_commands: Vec<GeneralCommandType>,
    #[doc = "Gets or sets a value indicating whether the session supports media control."]
    #[serde(
        rename = "SupportsMediaControl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_media_control: Option<bool>,
    #[doc = "Gets or sets a value indicating whether the session supports remote control."]
    #[serde(
        rename = "SupportsRemoteControl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_remote_control: Option<bool>,
    #[doc = "Gets or sets the transcoding info."]
    #[serde(
        rename = "TranscodingInfo",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transcoding_info: Option<TranscodingInfo>,
    #[doc = "Gets or sets the user id."]
    #[serde(
        rename = "UserId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_id: Option<uuid::Uuid>,
    #[doc = "Gets or sets the username."]
    #[serde(
        rename = "UserName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_name: Option<String>,
    #[doc = "Gets or sets the user primary image tag."]
    #[serde(
        rename = "UserPrimaryImageTag",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_primary_image_tag: Option<String>,
}

impl Default for SessionInfoDto {
    fn default() -> Self {
        Self {
            additional_users: Default::default(),
            application_version: Default::default(),
            capabilities: Default::default(),
            client: Default::default(),
            device_id: Default::default(),
            device_name: Default::default(),
            device_type: Default::default(),
            has_custom_device_name: Default::default(),
            id: Default::default(),
            is_active: Default::default(),
            last_activity_date: Default::default(),
            last_paused_date: Default::default(),
            last_playback_check_in: Default::default(),
            now_playing_item: Default::default(),
            now_playing_queue: Default::default(),
            now_playing_queue_full_items: Default::default(),
            now_viewing_item: Default::default(),
            play_state: Default::default(),
            playable_media_types: Default::default(),
            playlist_item_id: Default::default(),
            remote_end_point: Default::default(),
            server_id: Default::default(),
            supported_commands: Default::default(),
            supports_media_control: Default::default(),
            supports_remote_control: Default::default(),
            transcoding_info: Default::default(),
            user_id: Default::default(),
            user_name: Default::default(),
            user_primary_image_tag: Default::default(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SessionMessageType {
    ForceKeepAlive,
    GeneralCommand,
    UserDataChanged,
    Sessions,
    Play,
    SyncPlayCommand,
    SyncPlayGroupUpdate,
    Playstate,
    RestartRequired,
    ServerShuttingDown,
    ServerRestarting,
    LibraryChanged,
    UserDeleted,
    UserUpdated,
    SeriesTimerCreated,
    TimerCreated,
    SeriesTimerCancelled,
    TimerCancelled,
    RefreshProgress,
    ScheduledTaskEnded,
    PackageInstallationCancelled,
    PackageInstallationFailed,
    PackageInstallationCompleted,
    PackageInstalling,
    PackageUninstalled,
    ActivityLogEntry,
    ScheduledTasksInfo,
    ActivityLogEntryStart,
    ActivityLogEntryStop,
    SessionsStart,
    SessionsStop,
    ScheduledTasksInfoStart,
    ScheduledTasksInfoStop,
    KeepAlive,
}

impl std::fmt::Display for SessionMessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::ForceKeepAlive => f.write_str("ForceKeepAlive"),
            Self::GeneralCommand => f.write_str("GeneralCommand"),
            Self::UserDataChanged => f.write_str("UserDataChanged"),
            Self::Sessions => f.write_str("Sessions"),
            Self::Play => f.write_str("Play"),
            Self::SyncPlayCommand => f.write_str("SyncPlayCommand"),
            Self::SyncPlayGroupUpdate => f.write_str("SyncPlayGroupUpdate"),
            Self::Playstate => f.write_str("Playstate"),
            Self::RestartRequired => f.write_str("RestartRequired"),
            Self::ServerShuttingDown => f.write_str("ServerShuttingDown"),
            Self::ServerRestarting => f.write_str("ServerRestarting"),
            Self::LibraryChanged => f.write_str("LibraryChanged"),
            Self::UserDeleted => f.write_str("UserDeleted"),
            Self::UserUpdated => f.write_str("UserUpdated"),
            Self::SeriesTimerCreated => f.write_str("SeriesTimerCreated"),
            Self::TimerCreated => f.write_str("TimerCreated"),
            Self::SeriesTimerCancelled => f.write_str("SeriesTimerCancelled"),
            Self::TimerCancelled => f.write_str("TimerCancelled"),
            Self::RefreshProgress => f.write_str("RefreshProgress"),
            Self::ScheduledTaskEnded => f.write_str("ScheduledTaskEnded"),
            Self::PackageInstallationCancelled => f.write_str("PackageInstallationCancelled"),
            Self::PackageInstallationFailed => f.write_str("PackageInstallationFailed"),
            Self::PackageInstallationCompleted => f.write_str("PackageInstallationCompleted"),
            Self::PackageInstalling => f.write_str("PackageInstalling"),
            Self::PackageUninstalled => f.write_str("PackageUninstalled"),
            Self::ActivityLogEntry => f.write_str("ActivityLogEntry"),
            Self::ScheduledTasksInfo => f.write_str("ScheduledTasksInfo"),
            Self::ActivityLogEntryStart => f.write_str("ActivityLogEntryStart"),
            Self::ActivityLogEntryStop => f.write_str("ActivityLogEntryStop"),
            Self::SessionsStart => f.write_str("SessionsStart"),
            Self::SessionsStop => f.write_str("SessionsStop"),
            Self::ScheduledTasksInfoStart => f.write_str("ScheduledTasksInfoStart"),
            Self::ScheduledTasksInfoStop => f.write_str("ScheduledTasksInfoStop"),
            Self::KeepAlive => f.write_str("KeepAlive"),
        }
    }
}

impl std::str::FromStr for SessionMessageType {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "ForceKeepAlive" => Ok(Self::ForceKeepAlive),
            "GeneralCommand" => Ok(Self::GeneralCommand),
            "UserDataChanged" => Ok(Self::UserDataChanged),
            "Sessions" => Ok(Self::Sessions),
            "Play" => Ok(Self::Play),
            "SyncPlayCommand" => Ok(Self::SyncPlayCommand),
            "SyncPlayGroupUpdate" => Ok(Self::SyncPlayGroupUpdate),
            "Playstate" => Ok(Self::Playstate),
            "RestartRequired" => Ok(Self::RestartRequired),
            "ServerShuttingDown" => Ok(Self::ServerShuttingDown),
            "ServerRestarting" => Ok(Self::ServerRestarting),
            "LibraryChanged" => Ok(Self::LibraryChanged),
            "UserDeleted" => Ok(Self::UserDeleted),
            "UserUpdated" => Ok(Self::UserUpdated),
            "SeriesTimerCreated" => Ok(Self::SeriesTimerCreated),
            "TimerCreated" => Ok(Self::TimerCreated),
            "SeriesTimerCancelled" => Ok(Self::SeriesTimerCancelled),
            "TimerCancelled" => Ok(Self::TimerCancelled),
            "RefreshProgress" => Ok(Self::RefreshProgress),
            "ScheduledTaskEnded" => Ok(Self::ScheduledTaskEnded),
            "PackageInstallationCancelled" => Ok(Self::PackageInstallationCancelled),
            "PackageInstallationFailed" => Ok(Self::PackageInstallationFailed),
            "PackageInstallationCompleted" => Ok(Self::PackageInstallationCompleted),
            "PackageInstalling" => Ok(Self::PackageInstalling),
            "PackageUninstalled" => Ok(Self::PackageUninstalled),
            "ActivityLogEntry" => Ok(Self::ActivityLogEntry),
            "ScheduledTasksInfo" => Ok(Self::ScheduledTasksInfo),
            "ActivityLogEntryStart" => Ok(Self::ActivityLogEntryStart),
            "ActivityLogEntryStop" => Ok(Self::ActivityLogEntryStop),
            "SessionsStart" => Ok(Self::SessionsStart),
            "SessionsStop" => Ok(Self::SessionsStop),
            "ScheduledTasksInfoStart" => Ok(Self::ScheduledTasksInfoStart),
            "ScheduledTasksInfoStop" => Ok(Self::ScheduledTasksInfoStop),
            "KeepAlive" => Ok(Self::KeepAlive),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for SessionMessageType {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for SessionMessageType {
    type Error = super::error::ConversionError;
    fn try_from(
        value: &String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for SessionMessageType {
    type Error = super::error::ConversionError;
    fn try_from(
        value: String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "Class SessionUserInfo."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct SessionUserInfo {
    #[doc = "Gets or sets the user identifier."]
    #[serde(
        rename = "UserId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_id: Option<uuid::Uuid>,
    #[doc = "Gets or sets the name of the user."]
    #[serde(
        rename = "UserName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_name: Option<String>,
}

impl Default for SessionUserInfo {
    fn default() -> Self {
        Self {
            user_id: Default::default(),
            user_name: Default::default(),
        }
    }
}

#[doc = "Sessions message."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct SessionsMessage {
    #[doc = "Gets or sets the data."]
    #[serde(
        rename = "Data",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data: Option<Vec<SessionInfoDto>>,
    #[doc = "Gets or sets the message id."]
    #[serde(
        rename = "MessageId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_id: Option<uuid::Uuid>,
    #[serde(
        rename = "MessageType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_type: Option<SessionMessageType>,
}

impl Default for SessionsMessage {
    fn default() -> Self {
        Self {
            data: Default::default(),
            message_id: Default::default(),
            message_type: Default::default(),
        }
    }
}

#[doc = "Sessions start message.\r\nData is the timing data encoded as \"$initialDelay,$interval\" in ms."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct SessionsStartMessage {
    #[doc = "Gets or sets the data."]
    #[serde(
        rename = "Data",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data: Option<String>,
    #[serde(
        rename = "MessageType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_type: Option<SessionMessageType>,
}

impl Default for SessionsStartMessage {
    fn default() -> Self {
        Self {
            data: Default::default(),
            message_type: Default::default(),
        }
    }
}

#[doc = "Sessions stop message."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct SessionsStopMessage {
    #[serde(
        rename = "MessageType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_type: Option<SessionMessageType>,
}

impl Default for SessionsStopMessage {
    fn default() -> Self {
        Self {
            message_type: Default::default(),
        }
    }
}

#[doc = "Sync play command."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct SyncPlayCommandMessage {
    #[doc = "Class SendCommand."]
    #[serde(
        rename = "Data",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data: Option<SendCommand>,
    #[doc = "Gets or sets the message id."]
    #[serde(
        rename = "MessageId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_id: Option<uuid::Uuid>,
    #[serde(
        rename = "MessageType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_type: Option<SessionMessageType>,
}

impl Default for SyncPlayCommandMessage {
    fn default() -> Self {
        Self {
            data: Default::default(),
            message_id: Default::default(),
            message_type: Default::default(),
        }
    }
}

#[doc = "Class holding information on a running transcode."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct TranscodingInfo {
    #[doc = "Gets or sets the audio channels."]
    #[serde(
        rename = "AudioChannels",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub audio_channels: Option<i32>,
    #[doc = "Gets or sets the thread count used for encoding."]
    #[serde(
        rename = "AudioCodec",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub audio_codec: Option<String>,
    #[doc = "Gets or sets the bitrate."]
    #[serde(
        rename = "Bitrate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bitrate: Option<i32>,
    #[doc = "Gets or sets the completion percentage."]
    #[serde(
        rename = "CompletionPercentage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub completion_percentage: Option<f64>,
    #[doc = "Gets or sets the thread count used for encoding."]
    #[serde(
        rename = "Container",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub container: Option<String>,
    #[doc = "Gets or sets the framerate."]
    #[serde(
        rename = "Framerate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub framerate: Option<f32>,
    #[doc = "Gets or sets the hardware acceleration type."]
    #[serde(
        rename = "HardwareAccelerationType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hardware_acceleration_type: Option<HardwareAccelerationType>,
    #[doc = "Gets or sets the video height."]
    #[serde(
        rename = "Height",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub height: Option<i32>,
    #[doc = "Gets or sets a value indicating whether the audio is passed through."]
    #[serde(
        rename = "IsAudioDirect",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_audio_direct: Option<bool>,
    #[doc = "Gets or sets a value indicating whether the video is passed through."]
    #[serde(
        rename = "IsVideoDirect",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_video_direct: Option<bool>,
    #[doc = "Gets or sets the transcode reasons."]
    #[serde(
        rename = "TranscodeReasons",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub transcode_reasons: Vec<TranscodeReason>,
    #[doc = "Gets or sets the thread count used for encoding."]
    #[serde(
        rename = "VideoCodec",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub video_codec: Option<String>,
    #[doc = "Gets or sets the video width."]
    #[serde(
        rename = "Width",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub width: Option<i32>,
}

impl Default for TranscodingInfo {
    fn default() -> Self {
        Self {
            audio_channels: Default::default(),
            audio_codec: Default::default(),
            bitrate: Default::default(),
            completion_percentage: Default::default(),
            container: Default::default(),
            framerate: Default::default(),
            hardware_acceleration_type: Default::default(),
            height: Default::default(),
            is_audio_direct: Default::default(),
            is_video_direct: Default::default(),
            transcode_reasons: Default::default(),
            video_codec: Default::default(),
            width: Default::default(),
        }
    }
}

