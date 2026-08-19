use crate::types;

/// What `/LiveStreams/Open` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct OpenLiveStream<'q> {
    /// Always burn-in subtitle when transcoding.
    pub always_burn_in_subtitle_when_transcoding: Option<bool>,
    /// The audio stream index.
    pub audio_stream_index: Option<i32>,
    /// Whether to enable direct play. Default: true.
    pub enable_direct_play: Option<bool>,
    /// Whether to enable direct stream. Default: true.
    pub enable_direct_stream: Option<bool>,
    /// The item id.
    pub item_id: Option<&'q uuid::Uuid>,
    /// The maximum number of audio channels.
    pub max_audio_channels: Option<i32>,
    /// The maximum streaming bitrate.
    pub max_streaming_bitrate: Option<i32>,
    /// The open token.
    pub open_token: Option<&'q str>,
    /// The play session id.
    pub play_session_id: Option<&'q str>,
    /// The start time in ticks.
    pub start_time_ticks: Option<i64>,
    /// The subtitle stream index.
    pub subtitle_stream_index: Option<i32>,
    /// The user id.
    pub user_id: Option<&'q uuid::Uuid>,
}

/// What `/PlayingItems/{itemId}` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct OnPlaybackStart<'q> {
    /// The audio stream index.
    pub audio_stream_index: Option<i32>,
    /// Indicates if the client can seek.
    pub can_seek: Option<bool>,
    /// The live stream id.
    pub live_stream_id: Option<&'q str>,
    /// The id of the MediaSource.
    pub media_source_id: Option<&'q str>,
    /// The play method.
    pub play_method: Option<types::PlayMethod>,
    /// The play session id.
    pub play_session_id: Option<&'q str>,
    /// The subtitle stream index.
    pub subtitle_stream_index: Option<i32>,
}

/// What `/PlayingItems/{itemId}/Progress` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct OnPlaybackProgress<'q> {
    /// The audio stream index.
    pub audio_stream_index: Option<i32>,
    /// Indicates if the player is muted.
    pub is_muted: Option<bool>,
    /// Indicates if the player is paused.
    pub is_paused: Option<bool>,
    /// The live stream id.
    pub live_stream_id: Option<&'q str>,
    /// The id of the MediaSource.
    pub media_source_id: Option<&'q str>,
    /// The play method.
    pub play_method: Option<types::PlayMethod>,
    /// The play session id.
    pub play_session_id: Option<&'q str>,
    /// Optional. The current position, in ticks. 1 tick = 10000 ms.
    pub position_ticks: Option<i64>,
    /// The repeat mode.
    pub repeat_mode: Option<types::RepeatMode>,
    /// The subtitle stream index.
    pub subtitle_stream_index: Option<i32>,
    /// Scale of 0-100.
    pub volume_level: Option<i32>,
}
