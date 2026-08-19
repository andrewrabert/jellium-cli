use crate::Client;
use crate::error::Error;
use crate::query;
use crate::types;
use crate::util::encode_path;

impl Client {
    #[doc = "Closes a media source\n\nSends a `POST` request to `/LiveStreams/Close`\n\nArguments:\n- `live_stream_id`: The livestream id.\n"]
    pub async fn close_live_stream(&self, live_stream_id: &str) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/LiveStreams/Close".into())
            .query("liveStreamId", live_stream_id)
            .send_no_content()
            .await
    }

    #[doc = "Opens a media source\n\nSends a `POST` request to `/LiveStreams/Open`\n\nArguments:\n- `always_burn_in_subtitle_when_transcoding`: Always burn-in subtitle when transcoding.\n- `audio_stream_index`: The audio stream index.\n- `enable_direct_play`: Whether to enable direct play. Default: true.\n- `enable_direct_stream`: Whether to enable direct stream. Default: true.\n- `item_id`: The item id.\n- `max_audio_channels`: The maximum number of audio channels.\n- `max_streaming_bitrate`: The maximum streaming bitrate.\n- `open_token`: The open token.\n- `play_session_id`: The play session id.\n- `start_time_ticks`: The start time in ticks.\n- `subtitle_stream_index`: The subtitle stream index.\n- `user_id`: The user id.\n- `body`: The open live stream dto.\n"]
    pub async fn open_live_stream(
        &self,
        query: &query::OpenLiveStream<'_>,
        body: &types::OpenLiveStreamDto,
    ) -> Result<types::LiveStreamResponse, Error> {
        self.request(reqwest::Method::POST, "/LiveStreams/Open".into())
            .query_opt(
                "alwaysBurnInSubtitleWhenTranscoding",
                query.always_burn_in_subtitle_when_transcoding,
            )
            .query_opt("audioStreamIndex", query.audio_stream_index)
            .query_opt("enableDirectPlay", query.enable_direct_play)
            .query_opt("enableDirectStream", query.enable_direct_stream)
            .query_opt("itemId", query.item_id)
            .query_opt("maxAudioChannels", query.max_audio_channels)
            .query_opt("maxStreamingBitrate", query.max_streaming_bitrate)
            .query_opt("openToken", query.open_token)
            .query_opt("playSessionId", query.play_session_id)
            .query_opt("startTimeTicks", query.start_time_ticks)
            .query_opt("subtitleStreamIndex", query.subtitle_stream_index)
            .query_opt("userId", query.user_id)
            .json_body(body)
            .send()
            .await
    }

    #[doc = "Tests the network with a request with the size of the bitrate\n\nSends a `GET` request to `/Playback/BitrateTest`\n\nArguments:\n- `size`: The bitrate. Defaults to 102400.\n"]
    pub async fn get_bitrate_test_bytes(
        &self,
        size: Option<std::num::NonZeroU32>,
    ) -> Result<reqwest::Response, Error> {
        self.request(reqwest::Method::GET, "/Playback/BitrateTest".into())
            .query_opt("size", size)
            .send_response()
            .await
    }

    #[doc = "Reports that a session has begun playing an item\n\nSends a `POST` request to `/PlayingItems/{itemId}`\n\nArguments:\n- `item_id`: Item id.\n- `audio_stream_index`: The audio stream index.\n- `can_seek`: Indicates if the client can seek.\n- `live_stream_id`: The live stream id.\n- `media_source_id`: The id of the MediaSource.\n- `play_method`: The play method.\n- `play_session_id`: The play session id.\n- `subtitle_stream_index`: The subtitle stream index.\n"]
    pub async fn on_playback_start(
        &self,
        item_id: &uuid::Uuid,
        query: &query::OnPlaybackStart<'_>,
    ) -> Result<(), Error> {
        self.request(
            reqwest::Method::POST,
            format!("/PlayingItems/{}", encode_path(&item_id.to_string())),
        )
        .query_opt("audioStreamIndex", query.audio_stream_index)
        .query_opt("canSeek", query.can_seek)
        .query_opt("liveStreamId", query.live_stream_id)
        .query_opt("mediaSourceId", query.media_source_id)
        .query_opt("playMethod", query.play_method)
        .query_opt("playSessionId", query.play_session_id)
        .query_opt("subtitleStreamIndex", query.subtitle_stream_index)
        .send_no_content()
        .await
    }

    #[doc = "Reports that a session has stopped playing an item\n\nSends a `DELETE` request to `/PlayingItems/{itemId}`\n\nArguments:\n- `item_id`: Item id.\n- `live_stream_id`: The live stream id.\n- `media_source_id`: The id of the MediaSource.\n- `next_media_type`: The next media type that will play.\n- `play_session_id`: The play session id.\n- `position_ticks`: Optional. The position, in ticks, where playback stopped. 1 tick = 10000 ms.\n"]
    pub async fn on_playback_stopped(
        &self,
        item_id: &uuid::Uuid,
        live_stream_id: Option<&str>,
        media_source_id: Option<&str>,
        next_media_type: Option<&str>,
        play_session_id: Option<&str>,
        position_ticks: Option<i64>,
    ) -> Result<(), Error> {
        self.request(
            reqwest::Method::DELETE,
            format!("/PlayingItems/{}", encode_path(&item_id.to_string())),
        )
        .query_opt("liveStreamId", live_stream_id)
        .query_opt("mediaSourceId", media_source_id)
        .query_opt("nextMediaType", next_media_type)
        .query_opt("playSessionId", play_session_id)
        .query_opt("positionTicks", position_ticks)
        .send_no_content()
        .await
    }

    #[doc = "Reports a session's playback progress\n\nSends a `POST` request to `/PlayingItems/{itemId}/Progress`\n\nArguments:\n- `item_id`: Item id.\n- `audio_stream_index`: The audio stream index.\n- `is_muted`: Indicates if the player is muted.\n- `is_paused`: Indicates if the player is paused.\n- `live_stream_id`: The live stream id.\n- `media_source_id`: The id of the MediaSource.\n- `play_method`: The play method.\n- `play_session_id`: The play session id.\n- `position_ticks`: Optional. The current position, in ticks. 1 tick = 10000 ms.\n- `repeat_mode`: The repeat mode.\n- `subtitle_stream_index`: The subtitle stream index.\n- `volume_level`: Scale of 0-100.\n"]
    pub async fn on_playback_progress(
        &self,
        item_id: &uuid::Uuid,
        query: &query::OnPlaybackProgress<'_>,
    ) -> Result<(), Error> {
        self.request(
            reqwest::Method::POST,
            format!(
                "/PlayingItems/{}/Progress",
                encode_path(&item_id.to_string())
            ),
        )
        .query_opt("audioStreamIndex", query.audio_stream_index)
        .query_opt("isMuted", query.is_muted)
        .query_opt("isPaused", query.is_paused)
        .query_opt("liveStreamId", query.live_stream_id)
        .query_opt("mediaSourceId", query.media_source_id)
        .query_opt("playMethod", query.play_method)
        .query_opt("playSessionId", query.play_session_id)
        .query_opt("positionTicks", query.position_ticks)
        .query_opt("repeatMode", query.repeat_mode)
        .query_opt("subtitleStreamIndex", query.subtitle_stream_index)
        .query_opt("volumeLevel", query.volume_level)
        .send_no_content()
        .await
    }
}
