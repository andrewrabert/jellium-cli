use super::*;

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum GroupRepeatMode {
    RepeatOne,
    RepeatAll,
    RepeatNone,
}

impl std::fmt::Display for GroupRepeatMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::RepeatOne => f.write_str("RepeatOne"),
            Self::RepeatAll => f.write_str("RepeatAll"),
            Self::RepeatNone => f.write_str("RepeatNone"),
        }
    }
}

impl std::str::FromStr for GroupRepeatMode {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "RepeatOne" => Ok(Self::RepeatOne),
            "RepeatAll" => Ok(Self::RepeatAll),
            "RepeatNone" => Ok(Self::RepeatNone),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for GroupRepeatMode {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GroupRepeatMode {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GroupRepeatMode {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "`LiveStreamResponse`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct LiveStreamResponse {
    #[serde(
        rename = "MediaSource",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub media_source: Option<MediaSourceInfo>,
}

impl Default for LiveStreamResponse {
    fn default() -> Self {
        Self {
            media_source: Default::default(),
        }
    }
}

#[doc = "Open live stream dto."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct OpenLiveStreamDto {
    #[doc = "Gets or sets a value indicating whether always burn in subtitles when transcoding."]
    #[serde(
        rename = "AlwaysBurnInSubtitleWhenTranscoding",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub always_burn_in_subtitle_when_transcoding: Option<bool>,
    #[doc = "Gets or sets the audio stream index."]
    #[serde(
        rename = "AudioStreamIndex",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub audio_stream_index: Option<i32>,
    #[doc = "A MediaBrowser.Model.Dlna.DeviceProfile represents a set of metadata which determines which content a certain device is able to play.\r\n<br />\r\nSpecifically, it defines the supported <see cref=\"P:MediaBrowser.Model.Dlna.DeviceProfile.ContainerProfiles\">containers</see> and\r\n<see cref=\"P:MediaBrowser.Model.Dlna.DeviceProfile.CodecProfiles\">codecs</see> (video and/or audio, including codec profiles and levels)\r\nthe device is able to direct play (without transcoding or remuxing),\r\nas well as which <see cref=\"P:MediaBrowser.Model.Dlna.DeviceProfile.TranscodingProfiles\">containers/codecs to transcode to</see> in case it isn't."]
    #[serde(
        rename = "DeviceProfile",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub device_profile: Option<DeviceProfile>,
    #[doc = "Gets or sets the device play protocols."]
    #[serde(
        rename = "DirectPlayProtocols",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub direct_play_protocols: Vec<MediaProtocol>,
    #[doc = "Gets or sets a value indicating whether to enable direct play."]
    #[serde(
        rename = "EnableDirectPlay",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_direct_play: Option<bool>,
    #[doc = "Gets or sets a value indicating whether to enable direct stream."]
    #[serde(
        rename = "EnableDirectStream",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_direct_stream: Option<bool>,
    #[doc = "Gets or sets the item id."]
    #[serde(rename = "ItemId", default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<uuid::Uuid>,
    #[doc = "Gets or sets the max audio channels."]
    #[serde(
        rename = "MaxAudioChannels",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_audio_channels: Option<i32>,
    #[doc = "Gets or sets the max streaming bitrate."]
    #[serde(
        rename = "MaxStreamingBitrate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_streaming_bitrate: Option<i32>,
    #[doc = "Gets or sets the open token."]
    #[serde(rename = "OpenToken", default, skip_serializing_if = "Option::is_none")]
    pub open_token: Option<String>,
    #[doc = "Gets or sets the play session id."]
    #[serde(
        rename = "PlaySessionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub play_session_id: Option<String>,
    #[doc = "Gets or sets the start time in ticks."]
    #[serde(
        rename = "StartTimeTicks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_time_ticks: Option<i64>,
    #[doc = "Gets or sets the subtitle stream index."]
    #[serde(
        rename = "SubtitleStreamIndex",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub subtitle_stream_index: Option<i32>,
    #[doc = "Gets or sets the user id."]
    #[serde(rename = "UserId", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<uuid::Uuid>,
}

impl Default for OpenLiveStreamDto {
    fn default() -> Self {
        Self {
            always_burn_in_subtitle_when_transcoding: Default::default(),
            audio_stream_index: Default::default(),
            device_profile: Default::default(),
            direct_play_protocols: Default::default(),
            enable_direct_play: Default::default(),
            enable_direct_stream: Default::default(),
            item_id: Default::default(),
            max_audio_channels: Default::default(),
            max_streaming_bitrate: Default::default(),
            open_token: Default::default(),
            play_session_id: Default::default(),
            start_time_ticks: Default::default(),
            subtitle_stream_index: Default::default(),
            user_id: Default::default(),
        }
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum PlayAccess {
    Full,
    None,
}

impl std::fmt::Display for PlayAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Full => f.write_str("Full"),
            Self::None => f.write_str("None"),
        }
    }
}

impl std::str::FromStr for PlayAccess {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Full" => Ok(Self::Full),
            "None" => Ok(Self::None),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for PlayAccess {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for PlayAccess {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for PlayAccess {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "Play command websocket message."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct PlayMessage {
    #[doc = "Class PlayRequest."]
    #[serde(rename = "Data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<PlayRequest>,
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

impl Default for PlayMessage {
    fn default() -> Self {
        Self {
            data: Default::default(),
            message_id: Default::default(),
            message_type: Default::default(),
        }
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum PlayMethod {
    Transcode,
    DirectStream,
    DirectPlay,
}

impl std::fmt::Display for PlayMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Transcode => f.write_str("Transcode"),
            Self::DirectStream => f.write_str("DirectStream"),
            Self::DirectPlay => f.write_str("DirectPlay"),
        }
    }
}

impl std::str::FromStr for PlayMethod {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Transcode" => Ok(Self::Transcode),
            "DirectStream" => Ok(Self::DirectStream),
            "DirectPlay" => Ok(Self::DirectPlay),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for PlayMethod {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for PlayMethod {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for PlayMethod {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "Class PlayQueueUpdate."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct PlayQueueUpdate {
    #[doc = "Gets a value indicating whether the current item is playing."]
    #[serde(rename = "IsPlaying", default, skip_serializing_if = "Option::is_none")]
    pub is_playing: Option<bool>,
    #[doc = "Gets the UTC time of the last change to the playing queue."]
    #[serde(
        rename = "LastUpdate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_update: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets the playing item index in the playlist."]
    #[serde(
        rename = "PlayingItemIndex",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub playing_item_index: Option<i32>,
    #[doc = "Gets the playlist."]
    #[serde(rename = "Playlist", default, skip_serializing_if = "Vec::is_empty")]
    pub playlist: Vec<SyncPlayQueueItem>,
    #[serde(rename = "Reason", default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<PlayQueueUpdateReason>,
    #[serde(
        rename = "RepeatMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub repeat_mode: Option<GroupRepeatMode>,
    #[serde(
        rename = "ShuffleMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub shuffle_mode: Option<GroupShuffleMode>,
    #[doc = "Gets the start position ticks."]
    #[serde(
        rename = "StartPositionTicks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_position_ticks: Option<i64>,
}

impl Default for PlayQueueUpdate {
    fn default() -> Self {
        Self {
            is_playing: Default::default(),
            last_update: Default::default(),
            playing_item_index: Default::default(),
            playlist: Default::default(),
            reason: Default::default(),
            repeat_mode: Default::default(),
            shuffle_mode: Default::default(),
            start_position_ticks: Default::default(),
        }
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum PlayQueueUpdateReason {
    NewPlaylist,
    SetCurrentItem,
    RemoveItems,
    MoveItem,
    Queue,
    QueueNext,
    NextItem,
    PreviousItem,
    RepeatMode,
    ShuffleMode,
}

impl std::fmt::Display for PlayQueueUpdateReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::NewPlaylist => f.write_str("NewPlaylist"),
            Self::SetCurrentItem => f.write_str("SetCurrentItem"),
            Self::RemoveItems => f.write_str("RemoveItems"),
            Self::MoveItem => f.write_str("MoveItem"),
            Self::Queue => f.write_str("Queue"),
            Self::QueueNext => f.write_str("QueueNext"),
            Self::NextItem => f.write_str("NextItem"),
            Self::PreviousItem => f.write_str("PreviousItem"),
            Self::RepeatMode => f.write_str("RepeatMode"),
            Self::ShuffleMode => f.write_str("ShuffleMode"),
        }
    }
}

impl std::str::FromStr for PlayQueueUpdateReason {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "NewPlaylist" => Ok(Self::NewPlaylist),
            "SetCurrentItem" => Ok(Self::SetCurrentItem),
            "RemoveItems" => Ok(Self::RemoveItems),
            "MoveItem" => Ok(Self::MoveItem),
            "Queue" => Ok(Self::Queue),
            "QueueNext" => Ok(Self::QueueNext),
            "NextItem" => Ok(Self::NextItem),
            "PreviousItem" => Ok(Self::PreviousItem),
            "RepeatMode" => Ok(Self::RepeatMode),
            "ShuffleMode" => Ok(Self::ShuffleMode),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for PlayQueueUpdateReason {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for PlayQueueUpdateReason {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for PlayQueueUpdateReason {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "Class PlayRequest."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct PlayRequest {
    #[serde(
        rename = "AudioStreamIndex",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub audio_stream_index: Option<i32>,
    #[doc = "Gets or sets the controlling user identifier."]
    #[serde(
        rename = "ControllingUserId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub controlling_user_id: Option<uuid::Uuid>,
    #[doc = "Gets or sets the item ids."]
    #[serde(rename = "ItemIds", default, skip_serializing_if = "Option::is_none")]
    pub item_ids: Option<Vec<uuid::Uuid>>,
    #[serde(
        rename = "MediaSourceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub media_source_id: Option<String>,
    #[serde(
        rename = "PlayCommand",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub play_command: Option<PlayCommand>,
    #[serde(
        rename = "StartIndex",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_index: Option<i32>,
    #[doc = "Gets or sets the start position ticks that the first item should be played at."]
    #[serde(
        rename = "StartPositionTicks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_position_ticks: Option<i64>,
    #[serde(
        rename = "SubtitleStreamIndex",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub subtitle_stream_index: Option<i32>,
}

impl Default for PlayRequest {
    fn default() -> Self {
        Self {
            audio_stream_index: Default::default(),
            controlling_user_id: Default::default(),
            item_ids: Default::default(),
            media_source_id: Default::default(),
            play_command: Default::default(),
            start_index: Default::default(),
            start_position_ticks: Default::default(),
            subtitle_stream_index: Default::default(),
        }
    }
}

#[doc = "Class PlayRequestDto."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct PlayRequestDto {
    #[doc = "Gets or sets the position of the playing item in the queue."]
    #[serde(
        rename = "PlayingItemPosition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub playing_item_position: Option<i32>,
    #[doc = "Gets or sets the playing queue."]
    #[serde(
        rename = "PlayingQueue",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub playing_queue: Vec<uuid::Uuid>,
    #[doc = "Gets or sets the start position ticks."]
    #[serde(
        rename = "StartPositionTicks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_position_ticks: Option<i64>,
}

impl Default for PlayRequestDto {
    fn default() -> Self {
        Self {
            playing_item_position: Default::default(),
            playing_queue: Default::default(),
            start_position_ticks: Default::default(),
        }
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum PlaybackErrorCode {
    NotAllowed,
    NoCompatibleStream,
    RateLimitExceeded,
}

impl std::fmt::Display for PlaybackErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::NotAllowed => f.write_str("NotAllowed"),
            Self::NoCompatibleStream => f.write_str("NoCompatibleStream"),
            Self::RateLimitExceeded => f.write_str("RateLimitExceeded"),
        }
    }
}

impl std::str::FromStr for PlaybackErrorCode {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "NotAllowed" => Ok(Self::NotAllowed),
            "NoCompatibleStream" => Ok(Self::NoCompatibleStream),
            "RateLimitExceeded" => Ok(Self::RateLimitExceeded),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for PlaybackErrorCode {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for PlaybackErrorCode {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for PlaybackErrorCode {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "Playback info dto."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct PlaybackInfoDto {
    #[doc = "Gets or sets a value indicating whether to allow audio stream copy."]
    #[serde(
        rename = "AllowAudioStreamCopy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_audio_stream_copy: Option<bool>,
    #[doc = "Gets or sets a value indicating whether to enable video stream copy."]
    #[serde(
        rename = "AllowVideoStreamCopy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_video_stream_copy: Option<bool>,
    #[doc = "Gets or sets a value indicating whether always burn in subtitles when transcoding."]
    #[serde(
        rename = "AlwaysBurnInSubtitleWhenTranscoding",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub always_burn_in_subtitle_when_transcoding: Option<bool>,
    #[doc = "Gets or sets the audio stream index."]
    #[serde(
        rename = "AudioStreamIndex",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub audio_stream_index: Option<i32>,
    #[doc = "Gets or sets a value indicating whether to auto open the live stream."]
    #[serde(
        rename = "AutoOpenLiveStream",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_open_live_stream: Option<bool>,
    #[doc = "A MediaBrowser.Model.Dlna.DeviceProfile represents a set of metadata which determines which content a certain device is able to play.\r\n<br />\r\nSpecifically, it defines the supported <see cref=\"P:MediaBrowser.Model.Dlna.DeviceProfile.ContainerProfiles\">containers</see> and\r\n<see cref=\"P:MediaBrowser.Model.Dlna.DeviceProfile.CodecProfiles\">codecs</see> (video and/or audio, including codec profiles and levels)\r\nthe device is able to direct play (without transcoding or remuxing),\r\nas well as which <see cref=\"P:MediaBrowser.Model.Dlna.DeviceProfile.TranscodingProfiles\">containers/codecs to transcode to</see> in case it isn't."]
    #[serde(
        rename = "DeviceProfile",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub device_profile: Option<DeviceProfile>,
    #[doc = "Gets or sets a value indicating whether to enable direct play."]
    #[serde(
        rename = "EnableDirectPlay",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_direct_play: Option<bool>,
    #[doc = "Gets or sets a value indicating whether to enable direct stream."]
    #[serde(
        rename = "EnableDirectStream",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_direct_stream: Option<bool>,
    #[doc = "Gets or sets a value indicating whether to enable transcoding."]
    #[serde(
        rename = "EnableTranscoding",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_transcoding: Option<bool>,
    #[doc = "Gets or sets the live stream id."]
    #[serde(
        rename = "LiveStreamId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub live_stream_id: Option<String>,
    #[doc = "Gets or sets the max audio channels."]
    #[serde(
        rename = "MaxAudioChannels",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_audio_channels: Option<i32>,
    #[doc = "Gets or sets the max streaming bitrate."]
    #[serde(
        rename = "MaxStreamingBitrate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_streaming_bitrate: Option<i32>,
    #[doc = "Gets or sets the media source id."]
    #[serde(
        rename = "MediaSourceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub media_source_id: Option<String>,
    #[doc = "Gets or sets the start time in ticks."]
    #[serde(
        rename = "StartTimeTicks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_time_ticks: Option<i64>,
    #[doc = "Gets or sets the subtitle stream index."]
    #[serde(
        rename = "SubtitleStreamIndex",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub subtitle_stream_index: Option<i32>,
    #[doc = "Gets or sets the playback userId."]
    #[serde(rename = "UserId", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<uuid::Uuid>,
}

impl Default for PlaybackInfoDto {
    fn default() -> Self {
        Self {
            allow_audio_stream_copy: Default::default(),
            allow_video_stream_copy: Default::default(),
            always_burn_in_subtitle_when_transcoding: Default::default(),
            audio_stream_index: Default::default(),
            auto_open_live_stream: Default::default(),
            device_profile: Default::default(),
            enable_direct_play: Default::default(),
            enable_direct_stream: Default::default(),
            enable_transcoding: Default::default(),
            live_stream_id: Default::default(),
            max_audio_channels: Default::default(),
            max_streaming_bitrate: Default::default(),
            media_source_id: Default::default(),
            start_time_ticks: Default::default(),
            subtitle_stream_index: Default::default(),
            user_id: Default::default(),
        }
    }
}

#[doc = "Class PlaybackInfoResponse."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct PlaybackInfoResponse {
    #[doc = "Gets or sets the error code."]
    #[serde(rename = "ErrorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<PlaybackErrorCode>,
    #[doc = "Gets or sets the media sources."]
    #[serde(
        rename = "MediaSources",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub media_sources: Vec<MediaSourceInfo>,
    #[doc = "Gets or sets the play session identifier."]
    #[serde(
        rename = "PlaySessionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub play_session_id: Option<String>,
}

impl Default for PlaybackInfoResponse {
    fn default() -> Self {
        Self {
            error_code: Default::default(),
            media_sources: Default::default(),
            play_session_id: Default::default(),
        }
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum PlaybackOrder {
    Default,
    Shuffle,
}

impl std::fmt::Display for PlaybackOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Default => f.write_str("Default"),
            Self::Shuffle => f.write_str("Shuffle"),
        }
    }
}

impl std::str::FromStr for PlaybackOrder {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Default" => Ok(Self::Default),
            "Shuffle" => Ok(Self::Shuffle),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for PlaybackOrder {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for PlaybackOrder {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for PlaybackOrder {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "Class PlaybackProgressInfo."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct PlaybackProgressInfo {
    #[serde(
        rename = "AspectRatio",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub aspect_ratio: Option<String>,
    #[doc = "Gets or sets the index of the audio stream."]
    #[serde(
        rename = "AudioStreamIndex",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub audio_stream_index: Option<i32>,
    #[serde(
        rename = "Brightness",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub brightness: Option<i32>,
    #[doc = "Gets or sets a value indicating whether this instance can seek."]
    #[serde(rename = "CanSeek", default, skip_serializing_if = "Option::is_none")]
    pub can_seek: Option<bool>,
    #[doc = "Gets or sets a value indicating whether this instance is muted."]
    #[serde(rename = "IsMuted", default, skip_serializing_if = "Option::is_none")]
    pub is_muted: Option<bool>,
    #[doc = "Gets or sets a value indicating whether this instance is paused."]
    #[serde(rename = "IsPaused", default, skip_serializing_if = "Option::is_none")]
    pub is_paused: Option<bool>,
    #[doc = "Gets or sets the item."]
    #[serde(rename = "Item", default, skip_serializing_if = "Option::is_none")]
    pub item: Option<BaseItemDto>,
    #[doc = "Gets or sets the item identifier."]
    #[serde(rename = "ItemId", default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<uuid::Uuid>,
    #[doc = "Gets or sets the live stream identifier."]
    #[serde(
        rename = "LiveStreamId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub live_stream_id: Option<String>,
    #[doc = "Gets or sets the media version identifier."]
    #[serde(
        rename = "MediaSourceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub media_source_id: Option<String>,
    #[serde(
        rename = "NowPlayingQueue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub now_playing_queue: Option<Vec<QueueItem>>,
    #[serde(
        rename = "PlayMethod",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub play_method: Option<PlayMethod>,
    #[doc = "Gets or sets the play session identifier."]
    #[serde(
        rename = "PlaySessionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub play_session_id: Option<String>,
    #[serde(
        rename = "PlaybackOrder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub playback_order: Option<PlaybackOrder>,
    #[serde(
        rename = "PlaybackStartTimeTicks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub playback_start_time_ticks: Option<i64>,
    #[serde(
        rename = "PlaylistItemId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub playlist_item_id: Option<String>,
    #[doc = "Gets or sets the position ticks."]
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
    #[doc = "Gets or sets the session id."]
    #[serde(rename = "SessionId", default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[doc = "Gets or sets the index of the subtitle stream."]
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

impl Default for PlaybackProgressInfo {
    fn default() -> Self {
        Self {
            aspect_ratio: Default::default(),
            audio_stream_index: Default::default(),
            brightness: Default::default(),
            can_seek: Default::default(),
            is_muted: Default::default(),
            is_paused: Default::default(),
            item: Default::default(),
            item_id: Default::default(),
            live_stream_id: Default::default(),
            media_source_id: Default::default(),
            now_playing_queue: Default::default(),
            play_method: Default::default(),
            play_session_id: Default::default(),
            playback_order: Default::default(),
            playback_start_time_ticks: Default::default(),
            playlist_item_id: Default::default(),
            position_ticks: Default::default(),
            repeat_mode: Default::default(),
            session_id: Default::default(),
            subtitle_stream_index: Default::default(),
            volume_level: Default::default(),
        }
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum PlaybackRequestType {
    Play,
    SetPlaylistItem,
    RemoveFromPlaylist,
    MovePlaylistItem,
    Queue,
    Unpause,
    Pause,
    Stop,
    Seek,
    Buffer,
    Ready,
    NextItem,
    PreviousItem,
    SetRepeatMode,
    SetShuffleMode,
    Ping,
    IgnoreWait,
}

impl std::fmt::Display for PlaybackRequestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Play => f.write_str("Play"),
            Self::SetPlaylistItem => f.write_str("SetPlaylistItem"),
            Self::RemoveFromPlaylist => f.write_str("RemoveFromPlaylist"),
            Self::MovePlaylistItem => f.write_str("MovePlaylistItem"),
            Self::Queue => f.write_str("Queue"),
            Self::Unpause => f.write_str("Unpause"),
            Self::Pause => f.write_str("Pause"),
            Self::Stop => f.write_str("Stop"),
            Self::Seek => f.write_str("Seek"),
            Self::Buffer => f.write_str("Buffer"),
            Self::Ready => f.write_str("Ready"),
            Self::NextItem => f.write_str("NextItem"),
            Self::PreviousItem => f.write_str("PreviousItem"),
            Self::SetRepeatMode => f.write_str("SetRepeatMode"),
            Self::SetShuffleMode => f.write_str("SetShuffleMode"),
            Self::Ping => f.write_str("Ping"),
            Self::IgnoreWait => f.write_str("IgnoreWait"),
        }
    }
}

impl std::str::FromStr for PlaybackRequestType {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Play" => Ok(Self::Play),
            "SetPlaylistItem" => Ok(Self::SetPlaylistItem),
            "RemoveFromPlaylist" => Ok(Self::RemoveFromPlaylist),
            "MovePlaylistItem" => Ok(Self::MovePlaylistItem),
            "Queue" => Ok(Self::Queue),
            "Unpause" => Ok(Self::Unpause),
            "Pause" => Ok(Self::Pause),
            "Stop" => Ok(Self::Stop),
            "Seek" => Ok(Self::Seek),
            "Buffer" => Ok(Self::Buffer),
            "Ready" => Ok(Self::Ready),
            "NextItem" => Ok(Self::NextItem),
            "PreviousItem" => Ok(Self::PreviousItem),
            "SetRepeatMode" => Ok(Self::SetRepeatMode),
            "SetShuffleMode" => Ok(Self::SetShuffleMode),
            "Ping" => Ok(Self::Ping),
            "IgnoreWait" => Ok(Self::IgnoreWait),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for PlaybackRequestType {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for PlaybackRequestType {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for PlaybackRequestType {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "Class PlaybackStartInfo."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct PlaybackStartInfo {
    #[serde(
        rename = "AspectRatio",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub aspect_ratio: Option<String>,
    #[doc = "Gets or sets the index of the audio stream."]
    #[serde(
        rename = "AudioStreamIndex",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub audio_stream_index: Option<i32>,
    #[serde(
        rename = "Brightness",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub brightness: Option<i32>,
    #[doc = "Gets or sets a value indicating whether this instance can seek."]
    #[serde(rename = "CanSeek", default, skip_serializing_if = "Option::is_none")]
    pub can_seek: Option<bool>,
    #[doc = "Gets or sets a value indicating whether this instance is muted."]
    #[serde(rename = "IsMuted", default, skip_serializing_if = "Option::is_none")]
    pub is_muted: Option<bool>,
    #[doc = "Gets or sets a value indicating whether this instance is paused."]
    #[serde(rename = "IsPaused", default, skip_serializing_if = "Option::is_none")]
    pub is_paused: Option<bool>,
    #[doc = "Gets or sets the item."]
    #[serde(rename = "Item", default, skip_serializing_if = "Option::is_none")]
    pub item: Option<BaseItemDto>,
    #[doc = "Gets or sets the item identifier."]
    #[serde(rename = "ItemId", default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<uuid::Uuid>,
    #[doc = "Gets or sets the live stream identifier."]
    #[serde(
        rename = "LiveStreamId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub live_stream_id: Option<String>,
    #[doc = "Gets or sets the media version identifier."]
    #[serde(
        rename = "MediaSourceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub media_source_id: Option<String>,
    #[serde(
        rename = "NowPlayingQueue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub now_playing_queue: Option<Vec<QueueItem>>,
    #[serde(
        rename = "PlayMethod",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub play_method: Option<PlayMethod>,
    #[doc = "Gets or sets the play session identifier."]
    #[serde(
        rename = "PlaySessionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub play_session_id: Option<String>,
    #[serde(
        rename = "PlaybackOrder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub playback_order: Option<PlaybackOrder>,
    #[serde(
        rename = "PlaybackStartTimeTicks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub playback_start_time_ticks: Option<i64>,
    #[serde(
        rename = "PlaylistItemId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub playlist_item_id: Option<String>,
    #[doc = "Gets or sets the position ticks."]
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
    #[doc = "Gets or sets the session id."]
    #[serde(rename = "SessionId", default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[doc = "Gets or sets the index of the subtitle stream."]
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

impl Default for PlaybackStartInfo {
    fn default() -> Self {
        Self {
            aspect_ratio: Default::default(),
            audio_stream_index: Default::default(),
            brightness: Default::default(),
            can_seek: Default::default(),
            is_muted: Default::default(),
            is_paused: Default::default(),
            item: Default::default(),
            item_id: Default::default(),
            live_stream_id: Default::default(),
            media_source_id: Default::default(),
            now_playing_queue: Default::default(),
            play_method: Default::default(),
            play_session_id: Default::default(),
            playback_order: Default::default(),
            playback_start_time_ticks: Default::default(),
            playlist_item_id: Default::default(),
            position_ticks: Default::default(),
            repeat_mode: Default::default(),
            session_id: Default::default(),
            subtitle_stream_index: Default::default(),
            volume_level: Default::default(),
        }
    }
}

#[doc = "Class PlaybackStopInfo."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct PlaybackStopInfo {
    #[doc = "Gets or sets a value indicating whether this MediaBrowser.Model.Session.PlaybackStopInfo is failed."]
    #[serde(rename = "Failed", default, skip_serializing_if = "Option::is_none")]
    pub failed: Option<bool>,
    #[doc = "Gets or sets the item."]
    #[serde(rename = "Item", default, skip_serializing_if = "Option::is_none")]
    pub item: Option<BaseItemDto>,
    #[doc = "Gets or sets the item identifier."]
    #[serde(rename = "ItemId", default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<uuid::Uuid>,
    #[doc = "Gets or sets the live stream identifier."]
    #[serde(
        rename = "LiveStreamId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub live_stream_id: Option<String>,
    #[doc = "Gets or sets the media version identifier."]
    #[serde(
        rename = "MediaSourceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub media_source_id: Option<String>,
    #[serde(
        rename = "NextMediaType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub next_media_type: Option<String>,
    #[serde(
        rename = "NowPlayingQueue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub now_playing_queue: Option<Vec<QueueItem>>,
    #[doc = "Gets or sets the play session identifier."]
    #[serde(
        rename = "PlaySessionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub play_session_id: Option<String>,
    #[serde(
        rename = "PlaylistItemId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub playlist_item_id: Option<String>,
    #[doc = "Gets or sets the position ticks."]
    #[serde(
        rename = "PositionTicks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub position_ticks: Option<i64>,
    #[doc = "Gets or sets the session id."]
    #[serde(rename = "SessionId", default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

impl Default for PlaybackStopInfo {
    fn default() -> Self {
        Self {
            failed: Default::default(),
            item: Default::default(),
            item_id: Default::default(),
            live_stream_id: Default::default(),
            media_source_id: Default::default(),
            next_media_type: Default::default(),
            now_playing_queue: Default::default(),
            play_session_id: Default::default(),
            playlist_item_id: Default::default(),
            position_ticks: Default::default(),
            session_id: Default::default(),
        }
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum PlaystateCommand {
    Stop,
    Pause,
    Unpause,
    NextTrack,
    PreviousTrack,
    Seek,
    Rewind,
    FastForward,
    PlayPause,
}

impl std::fmt::Display for PlaystateCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Stop => f.write_str("Stop"),
            Self::Pause => f.write_str("Pause"),
            Self::Unpause => f.write_str("Unpause"),
            Self::NextTrack => f.write_str("NextTrack"),
            Self::PreviousTrack => f.write_str("PreviousTrack"),
            Self::Seek => f.write_str("Seek"),
            Self::Rewind => f.write_str("Rewind"),
            Self::FastForward => f.write_str("FastForward"),
            Self::PlayPause => f.write_str("PlayPause"),
        }
    }
}

impl std::str::FromStr for PlaystateCommand {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Stop" => Ok(Self::Stop),
            "Pause" => Ok(Self::Pause),
            "Unpause" => Ok(Self::Unpause),
            "NextTrack" => Ok(Self::NextTrack),
            "PreviousTrack" => Ok(Self::PreviousTrack),
            "Seek" => Ok(Self::Seek),
            "Rewind" => Ok(Self::Rewind),
            "FastForward" => Ok(Self::FastForward),
            "PlayPause" => Ok(Self::PlayPause),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for PlaystateCommand {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for PlaystateCommand {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for PlaystateCommand {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "Playstate message."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct PlaystateMessage {
    #[doc = "Gets or sets the data."]
    #[serde(rename = "Data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<PlaystateRequest>,
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

impl Default for PlaystateMessage {
    fn default() -> Self {
        Self {
            data: Default::default(),
            message_id: Default::default(),
            message_type: Default::default(),
        }
    }
}

#[doc = "`PlaystateRequest`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct PlaystateRequest {
    #[serde(rename = "Command", default, skip_serializing_if = "Option::is_none")]
    pub command: Option<PlaystateCommand>,
    #[doc = "Gets or sets the controlling user identifier."]
    #[serde(
        rename = "ControllingUserId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub controlling_user_id: Option<String>,
    #[serde(
        rename = "SeekPositionTicks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub seek_position_ticks: Option<i64>,
}

impl Default for PlaystateRequest {
    fn default() -> Self {
        Self {
            command: Default::default(),
            controlling_user_id: Default::default(),
            seek_position_ticks: Default::default(),
        }
    }
}

#[doc = "`QueueItem`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct QueueItem {
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(
        rename = "PlaylistItemId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub playlist_item_id: Option<String>,
}

impl Default for QueueItem {
    fn default() -> Self {
        Self {
            id: Default::default(),
            playlist_item_id: Default::default(),
        }
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum RepeatMode {
    RepeatNone,
    RepeatAll,
    RepeatOne,
}

impl std::fmt::Display for RepeatMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::RepeatNone => f.write_str("RepeatNone"),
            Self::RepeatAll => f.write_str("RepeatAll"),
            Self::RepeatOne => f.write_str("RepeatOne"),
        }
    }
}

impl std::str::FromStr for RepeatMode {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "RepeatNone" => Ok(Self::RepeatNone),
            "RepeatAll" => Ok(Self::RepeatAll),
            "RepeatOne" => Ok(Self::RepeatOne),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for RepeatMode {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for RepeatMode {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for RepeatMode {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "Class SendCommand."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct SendCommand {
    #[serde(rename = "Command", default, skip_serializing_if = "Option::is_none")]
    pub command: Option<SendCommandType>,
    #[doc = "Gets the UTC time when this command has been emitted."]
    #[serde(rename = "EmittedAt", default, skip_serializing_if = "Option::is_none")]
    pub emitted_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets the group identifier."]
    #[serde(rename = "GroupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<uuid::Uuid>,
    #[doc = "Gets the playlist identifier of the playing item."]
    #[serde(
        rename = "PlaylistItemId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub playlist_item_id: Option<uuid::Uuid>,
    #[doc = "Gets the position ticks."]
    #[serde(
        rename = "PositionTicks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub position_ticks: Option<i64>,
    #[doc = "Gets or sets the UTC time when to execute the command."]
    #[serde(rename = "When", default, skip_serializing_if = "Option::is_none")]
    pub when: Option<chrono::DateTime<chrono::Utc>>,
}

impl Default for SendCommand {
    fn default() -> Self {
        Self {
            command: Default::default(),
            emitted_at: Default::default(),
            group_id: Default::default(),
            playlist_item_id: Default::default(),
            position_ticks: Default::default(),
            when: Default::default(),
        }
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum SendCommandType {
    Unpause,
    Pause,
    Stop,
    Seek,
}

impl std::fmt::Display for SendCommandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Unpause => f.write_str("Unpause"),
            Self::Pause => f.write_str("Pause"),
            Self::Stop => f.write_str("Stop"),
            Self::Seek => f.write_str("Seek"),
        }
    }
}

impl std::str::FromStr for SendCommandType {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Unpause" => Ok(Self::Unpause),
            "Pause" => Ok(Self::Pause),
            "Stop" => Ok(Self::Stop),
            "Seek" => Ok(Self::Seek),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for SendCommandType {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for SendCommandType {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for SendCommandType {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "Class SetRepeatModeRequestDto."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct SetRepeatModeRequestDto {
    #[serde(rename = "Mode", default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<GroupRepeatMode>,
}

impl Default for SetRepeatModeRequestDto {
    fn default() -> Self {
        Self {
            mode: Default::default(),
        }
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum SubtitlePlaybackMode {
    Default,
    Always,
    OnlyForced,
    None,
    Smart,
}

impl std::fmt::Display for SubtitlePlaybackMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Default => f.write_str("Default"),
            Self::Always => f.write_str("Always"),
            Self::OnlyForced => f.write_str("OnlyForced"),
            Self::None => f.write_str("None"),
            Self::Smart => f.write_str("Smart"),
        }
    }
}

impl std::str::FromStr for SubtitlePlaybackMode {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Default" => Ok(Self::Default),
            "Always" => Ok(Self::Always),
            "OnlyForced" => Ok(Self::OnlyForced),
            "None" => Ok(Self::None),
            "Smart" => Ok(Self::Smart),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for SubtitlePlaybackMode {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for SubtitlePlaybackMode {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for SubtitlePlaybackMode {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "`SyncPlayPlayQueueUpdate`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct SyncPlayPlayQueueUpdate {
    #[doc = "Gets the update data."]
    #[serde(rename = "Data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<PlayQueueUpdate>,
    #[doc = "Gets the group identifier."]
    #[serde(rename = "GroupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<uuid::Uuid>,
    #[serde(rename = "Type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<GroupUpdateType>,
}

impl Default for SyncPlayPlayQueueUpdate {
    fn default() -> Self {
        Self {
            data: Default::default(),
            group_id: Default::default(),
            type_: Default::default(),
        }
    }
}

#[doc = "Class QueueItem."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct SyncPlayQueueItem {
    #[doc = "Gets the item identifier."]
    #[serde(rename = "ItemId", default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<uuid::Uuid>,
    #[doc = "Gets the playlist identifier of the item."]
    #[serde(
        rename = "PlaylistItemId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub playlist_item_id: Option<uuid::Uuid>,
}

impl Default for SyncPlayQueueItem {
    fn default() -> Self {
        Self {
            item_id: Default::default(),
            playlist_item_id: Default::default(),
        }
    }
}

#[doc = "`SyncPlayStateUpdate`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct SyncPlayStateUpdate {
    #[doc = "Gets the update data."]
    #[serde(rename = "Data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<GroupStateUpdate>,
    #[doc = "Gets the group identifier."]
    #[serde(rename = "GroupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<uuid::Uuid>,
    #[serde(rename = "Type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<GroupUpdateType>,
}

impl Default for SyncPlayStateUpdate {
    fn default() -> Self {
        Self {
            data: Default::default(),
            group_id: Default::default(),
            type_: Default::default(),
        }
    }
}
