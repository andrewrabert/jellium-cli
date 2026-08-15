use clap::Subcommand;
use jellyfin_api::types::{
    OpenLiveStreamDto, PlayMethod, PlaybackInfoDto, PlaybackOrder, PlaybackProgressInfo,
    PlaybackStartInfo, PlaybackStopInfo, RepeatMode,
};
use uuid::Uuid;

#[derive(Subcommand)]
pub enum PlaybackCommand {
    /// Report playback has started (via body)
    ReportStart {
        /// The item ID
        #[arg(long)]
        item_id: Option<Uuid>,

        /// Audio stream index
        #[arg(long)]
        audio_stream_index: Option<i32>,

        /// Whether the client can seek
        #[arg(long)]
        can_seek: Option<bool>,

        /// Whether the player is muted
        #[arg(long)]
        is_muted: Option<bool>,

        /// Whether the player is paused
        #[arg(long)]
        is_paused: Option<bool>,

        /// Live stream ID
        #[arg(long)]
        live_stream_id: Option<String>,

        /// Media source ID
        #[arg(long)]
        media_source_id: Option<String>,

        /// Play method (Transcode, DirectStream, DirectPlay)
        #[arg(long)]
        play_method: Option<PlayMethod>,

        /// Play session ID
        #[arg(long)]
        play_session_id: Option<String>,

        /// Playback order (Default, Shuffle)
        #[arg(long)]
        playback_order: Option<PlaybackOrder>,

        /// Position ticks
        #[arg(long)]
        position_ticks: Option<i64>,

        /// Repeat mode (RepeatNone, RepeatAll, RepeatOne)
        #[arg(long)]
        repeat_mode: Option<RepeatMode>,

        /// Subtitle stream index
        #[arg(long)]
        subtitle_stream_index: Option<i32>,

        /// Volume level (0-100)
        #[arg(long)]
        volume_level: Option<i32>,
    },

    /// Report playback progress (via body)
    ReportProgress {
        /// The item ID
        #[arg(long)]
        item_id: Option<Uuid>,

        /// Audio stream index
        #[arg(long)]
        audio_stream_index: Option<i32>,

        /// Whether the client can seek
        #[arg(long)]
        can_seek: Option<bool>,

        /// Whether the player is muted
        #[arg(long)]
        is_muted: Option<bool>,

        /// Whether the player is paused
        #[arg(long)]
        is_paused: Option<bool>,

        /// Live stream ID
        #[arg(long)]
        live_stream_id: Option<String>,

        /// Media source ID
        #[arg(long)]
        media_source_id: Option<String>,

        /// Play method (Transcode, DirectStream, DirectPlay)
        #[arg(long)]
        play_method: Option<PlayMethod>,

        /// Play session ID
        #[arg(long)]
        play_session_id: Option<String>,

        /// Playback order (Default, Shuffle)
        #[arg(long)]
        playback_order: Option<PlaybackOrder>,

        /// Current position in ticks (1 tick = 10000 ms)
        #[arg(long)]
        position_ticks: Option<i64>,

        /// Repeat mode (RepeatNone, RepeatAll, RepeatOne)
        #[arg(long)]
        repeat_mode: Option<RepeatMode>,

        /// Subtitle stream index
        #[arg(long)]
        subtitle_stream_index: Option<i32>,

        /// Volume level (0-100)
        #[arg(long)]
        volume_level: Option<i32>,
    },

    /// Report playback has stopped (via body)
    ReportStopped {
        /// The item ID
        #[arg(long)]
        item_id: Option<Uuid>,

        /// Whether playback failed
        #[arg(long)]
        failed: Option<bool>,

        /// Live stream ID
        #[arg(long)]
        live_stream_id: Option<String>,

        /// Media source ID
        #[arg(long)]
        media_source_id: Option<String>,

        /// Next media type that will play
        #[arg(long)]
        next_media_type: Option<String>,

        /// Play session ID
        #[arg(long)]
        play_session_id: Option<String>,

        /// Position in ticks where playback stopped
        #[arg(long)]
        position_ticks: Option<i64>,
    },

    /// Open a live stream
    OpenLiveStream {
        /// Always burn in subtitle when transcoding
        #[arg(long)]
        always_burn_in_subtitle_when_transcoding: Option<bool>,

        /// Audio stream index
        #[arg(long)]
        audio_stream_index: Option<i32>,

        /// Enable direct play
        #[arg(long)]
        enable_direct_play: Option<bool>,

        /// Enable direct stream
        #[arg(long)]
        enable_direct_stream: Option<bool>,

        /// Item ID
        #[arg(long)]
        item_id: Option<Uuid>,

        /// Maximum number of audio channels
        #[arg(long)]
        max_audio_channels: Option<i32>,

        /// Maximum streaming bitrate
        #[arg(long)]
        max_streaming_bitrate: Option<i32>,

        /// Open token
        #[arg(long)]
        open_token: Option<String>,

        /// Play session ID
        #[arg(long)]
        play_session_id: Option<String>,

        /// Start time in ticks
        #[arg(long)]
        start_time_ticks: Option<i64>,

        /// Subtitle stream index
        #[arg(long)]
        subtitle_stream_index: Option<i32>,

        /// User ID
        #[arg(long)]
        user_id: Option<Uuid>,
    },

    /// Close a live stream
    CloseLiveStream {
        /// The live stream ID
        live_stream_id: String,
    },

    /// Get playback info for an item
    PlaybackInfo {
        /// The item ID
        item_id: Uuid,

        /// The user ID
        #[arg(long)]
        user_id: Option<Uuid>,
    },

    /// Post playback info for an item (advanced)
    PostPlaybackInfo {
        /// The item ID
        item_id: Uuid,

        /// Allow audio stream copy
        #[arg(long)]
        allow_audio_stream_copy: Option<bool>,

        /// Allow video stream copy
        #[arg(long)]
        allow_video_stream_copy: Option<bool>,

        /// Audio stream index
        #[arg(long)]
        audio_stream_index: Option<i32>,

        /// Auto open live stream
        #[arg(long)]
        auto_open_live_stream: Option<bool>,

        /// Enable direct play
        #[arg(long)]
        enable_direct_play: Option<bool>,

        /// Enable direct stream
        #[arg(long)]
        enable_direct_stream: Option<bool>,

        /// Enable transcoding
        #[arg(long)]
        enable_transcoding: Option<bool>,

        /// Live stream ID
        #[arg(long)]
        live_stream_id: Option<String>,

        /// Maximum number of audio channels
        #[arg(long)]
        max_audio_channels: Option<i32>,

        /// Maximum streaming bitrate
        #[arg(long)]
        max_streaming_bitrate: Option<i32>,

        /// Media source ID
        #[arg(long)]
        media_source_id: Option<String>,

        /// Start time in ticks
        #[arg(long)]
        start_time_ticks: Option<i64>,

        /// Subtitle stream index
        #[arg(long)]
        subtitle_stream_index: Option<i32>,

        /// User ID
        #[arg(long)]
        user_id: Option<Uuid>,
    },
}

pub async fn execute(
    client: &jellyfin_api::Client,
    _user_id: &Uuid,
    command: &PlaybackCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        PlaybackCommand::ReportStart {
            item_id,
            audio_stream_index,
            can_seek,
            is_muted,
            is_paused,
            live_stream_id,
            media_source_id,
            play_method,
            play_session_id,
            playback_order,
            position_ticks,
            repeat_mode,
            subtitle_stream_index,
            volume_level,
        } => {
            let body = PlaybackStartInfo {
                item_id: *item_id,
                audio_stream_index: *audio_stream_index,
                can_seek: *can_seek,
                is_muted: *is_muted,
                is_paused: *is_paused,
                live_stream_id: live_stream_id.clone(),
                media_source_id: media_source_id.clone(),
                play_method: *play_method,
                play_session_id: play_session_id.clone(),
                playback_order: *playback_order,
                position_ticks: *position_ticks,
                repeat_mode: *repeat_mode,
                subtitle_stream_index: *subtitle_stream_index,
                volume_level: *volume_level,
                ..Default::default()
            };
            client.report_playback_start(&body).await?;
        }
        PlaybackCommand::ReportProgress {
            item_id,
            audio_stream_index,
            can_seek,
            is_muted,
            is_paused,
            live_stream_id,
            media_source_id,
            play_method,
            play_session_id,
            playback_order,
            position_ticks,
            repeat_mode,
            subtitle_stream_index,
            volume_level,
        } => {
            let body = PlaybackProgressInfo {
                item_id: *item_id,
                audio_stream_index: *audio_stream_index,
                can_seek: *can_seek,
                is_muted: *is_muted,
                is_paused: *is_paused,
                live_stream_id: live_stream_id.clone(),
                media_source_id: media_source_id.clone(),
                play_method: *play_method,
                play_session_id: play_session_id.clone(),
                playback_order: *playback_order,
                position_ticks: *position_ticks,
                repeat_mode: *repeat_mode,
                subtitle_stream_index: *subtitle_stream_index,
                volume_level: *volume_level,
                ..Default::default()
            };
            client.report_playback_progress(&body).await?;
        }
        PlaybackCommand::ReportStopped {
            item_id,
            failed,
            live_stream_id,
            media_source_id,
            next_media_type,
            play_session_id,
            position_ticks,
        } => {
            let body = PlaybackStopInfo {
                item_id: *item_id,
                failed: *failed,
                live_stream_id: live_stream_id.clone(),
                media_source_id: media_source_id.clone(),
                next_media_type: next_media_type.clone(),
                play_session_id: play_session_id.clone(),
                position_ticks: *position_ticks,
                ..Default::default()
            };
            client.report_playback_stopped(&body).await?;
        }
        PlaybackCommand::OpenLiveStream {
            always_burn_in_subtitle_when_transcoding,
            audio_stream_index,
            enable_direct_play,
            enable_direct_stream,
            item_id,
            max_audio_channels,
            max_streaming_bitrate,
            open_token,
            play_session_id,
            start_time_ticks,
            subtitle_stream_index,
            user_id,
        } => {
            let body = OpenLiveStreamDto {
                ..Default::default()
            };
            let result = client
                .open_live_stream(
                    *always_burn_in_subtitle_when_transcoding,
                    *audio_stream_index,
                    *enable_direct_play,
                    *enable_direct_stream,
                    item_id.as_ref(),
                    *max_audio_channels,
                    *max_streaming_bitrate,
                    open_token.as_deref(),
                    play_session_id.as_deref(),
                    *start_time_ticks,
                    *subtitle_stream_index,
                    user_id.as_ref(),
                    &body,
                )
                .await?;
            crate::output::print_json(&result)?;
        }
        PlaybackCommand::CloseLiveStream { live_stream_id } => {
            client.close_live_stream(live_stream_id).await?;
        }
        PlaybackCommand::PlaybackInfo { item_id, user_id } => {
            let result = client.get_playback_info(item_id, user_id.as_ref()).await?;
            crate::output::print_json(&result)?;
        }
        PlaybackCommand::PostPlaybackInfo {
            item_id,
            allow_audio_stream_copy,
            allow_video_stream_copy,
            audio_stream_index,
            auto_open_live_stream,
            enable_direct_play,
            enable_direct_stream,
            enable_transcoding,
            live_stream_id,
            max_audio_channels,
            max_streaming_bitrate,
            media_source_id,
            start_time_ticks,
            subtitle_stream_index,
            user_id,
        } => {
            let body = PlaybackInfoDto {
                allow_audio_stream_copy: *allow_audio_stream_copy,
                allow_video_stream_copy: *allow_video_stream_copy,
                audio_stream_index: *audio_stream_index,
                auto_open_live_stream: *auto_open_live_stream,
                enable_direct_play: *enable_direct_play,
                enable_direct_stream: *enable_direct_stream,
                enable_transcoding: *enable_transcoding,
                live_stream_id: live_stream_id.clone(),
                max_audio_channels: *max_audio_channels,
                max_streaming_bitrate: *max_streaming_bitrate,
                media_source_id: media_source_id.clone(),
                start_time_ticks: *start_time_ticks,
                subtitle_stream_index: *subtitle_stream_index,
                user_id: *user_id,
                ..Default::default()
            };
            let result = client
                .get_posted_playback_info(
                    item_id,
                    *allow_audio_stream_copy,
                    *allow_video_stream_copy,
                    *audio_stream_index,
                    *auto_open_live_stream,
                    *enable_direct_play,
                    *enable_direct_stream,
                    *enable_transcoding,
                    live_stream_id.as_deref(),
                    *max_audio_channels,
                    *max_streaming_bitrate,
                    media_source_id.as_deref(),
                    *start_time_ticks,
                    *subtitle_stream_index,
                    user_id.as_ref(),
                    &body,
                )
                .await?;
            crate::output::print_json(&result)?;
        }
    }
    Ok(())
}
