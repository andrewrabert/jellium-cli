use crate::types;

/// The whole of `/Videos/{itemId}/hls1/{playlistId}/{segmentId}.{container}`.
#[derive(Debug, Clone)]
pub struct GetHlsVideoSegment<'q> {
    pub item_id: &'q uuid::Uuid,
    pub playlist_id: &'q str,
    pub segment_id: i32,
    pub container: &'q types::GetHlsVideoSegmentContainer,
    pub actual_segment_length_ticks: i64,
    pub allow_audio_stream_copy: Option<bool>,
    pub allow_video_stream_copy: Option<bool>,
    pub always_burn_in_subtitle_when_transcoding: Option<bool>,
    pub audio_bit_rate: Option<i32>,
    pub audio_channels: Option<i32>,
    pub audio_codec: Option<&'q types::GetHlsVideoSegmentAudioCodec>,
    pub audio_sample_rate: Option<i32>,
    pub audio_stream_index: Option<i32>,
    pub break_on_non_key_frames: Option<bool>,
    pub context: Option<types::EncodingContext>,
    pub copy_timestamps: Option<bool>,
    pub cpu_core_limit: Option<i32>,
    pub de_interlace: Option<bool>,
    pub device_id: Option<&'q str>,
    pub device_profile_id: Option<&'q str>,
    pub enable_audio_vbr_encoding: Option<bool>,
    pub enable_auto_stream_copy: Option<bool>,
    pub enable_mpegts_m2_ts_mode: Option<bool>,
    pub framerate: Option<f32>,
    pub height: Option<i32>,
    pub level: Option<&'q types::GetHlsVideoSegmentLevel>,
    pub live_stream_id: Option<&'q str>,
    pub max_audio_bit_depth: Option<i32>,
    pub max_audio_channels: Option<i32>,
    pub max_framerate: Option<f32>,
    pub max_height: Option<i32>,
    pub max_ref_frames: Option<i32>,
    pub max_video_bit_depth: Option<i32>,
    pub max_width: Option<i32>,
    pub media_source_id: Option<&'q str>,
    pub min_segments: Option<i32>,
    pub params: Option<&'q str>,
    pub play_session_id: Option<&'q str>,
    pub profile: Option<&'q str>,
    pub require_avc: Option<bool>,
    pub require_non_anamorphic: Option<bool>,
    pub runtime_ticks: i64,
    pub segment_container: Option<&'q types::GetHlsVideoSegmentSegmentContainer>,
    pub segment_length: Option<i32>,
    pub start_time_ticks: Option<i64>,
    pub r#static: Option<bool>,
    pub stream_options: Option<&'q std::collections::HashMap<String, Option<String>>>,
    pub subtitle_codec: Option<&'q types::GetHlsVideoSegmentSubtitleCodec>,
    pub subtitle_method: Option<types::SubtitleDeliveryMethod>,
    pub subtitle_stream_index: Option<i32>,
    pub tag: Option<&'q str>,
    pub transcode_reasons: Option<&'q str>,
    pub transcoding_max_audio_channels: Option<i32>,
    pub video_bit_rate: Option<i32>,
    pub video_codec: Option<&'q types::GetHlsVideoSegmentVideoCodec>,
    pub video_stream_index: Option<i32>,
    pub width: Option<i32>,
}

/// What `/Videos/{itemId}/live.m3u8` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetLiveHlsStream<'q> {
    /// Whether or not to allow copying of the audio stream url.
    pub allow_audio_stream_copy: Option<bool>,
    /// Whether or not to allow copying of the video stream url.
    pub allow_video_stream_copy: Option<bool>,
    /// Whether to always burn in subtitles when transcoding.
    pub always_burn_in_subtitle_when_transcoding: Option<bool>,
    /// Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults.
    pub audio_bit_rate: Option<i32>,
    /// Optional. Specify a specific number of audio channels to encode to, e.g. 2.
    pub audio_channels: Option<i32>,
    /// Optional. Specify an audio codec to encode to, e.g. mp3.
    pub audio_codec: Option<&'q types::GetLiveHlsStreamAudioCodec>,
    /// Optional. Specify a specific audio sample rate, e.g. 44100.
    pub audio_sample_rate: Option<i32>,
    /// Optional. The index of the audio stream to use. If omitted the first audio stream will be used.
    pub audio_stream_index: Option<i32>,
    /// Optional. Whether to break on non key frames.
    pub break_on_non_key_frames: Option<bool>,
    /// The audio container.
    pub container: Option<&'q types::GetLiveHlsStreamContainer>,
    /// Optional. The MediaBrowser.Model.Dlna.EncodingContext.
    pub context: Option<types::EncodingContext>,
    /// Whether or not to copy timestamps when transcoding with an offset. Defaults to false.
    pub copy_timestamps: Option<bool>,
    /// Optional. The limit of how many cpu cores to use.
    pub cpu_core_limit: Option<i32>,
    /// Optional. Whether to deinterlace the video.
    pub de_interlace: Option<bool>,
    /// The device id of the client requesting. Used to stop encoding processes when needed.
    pub device_id: Option<&'q str>,
    /// Optional. The dlna device profile id to utilize.
    pub device_profile_id: Option<&'q str>,
    /// Optional. Whether to enable Audio Encoding.
    pub enable_audio_vbr_encoding: Option<bool>,
    /// Whether or not to allow automatic stream copy if requested values match the original source. Defaults to true.
    pub enable_auto_stream_copy: Option<bool>,
    /// Optional. Whether to enable the MpegtsM2Ts mode.
    pub enable_mpegts_m2_ts_mode: Option<bool>,
    /// Optional. Whether to enable subtitles in the manifest.
    pub enable_subtitles_in_manifest: Option<bool>,
    /// Optional. A specific video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.
    pub framerate: Option<f32>,
    /// Optional. The fixed vertical resolution of the encoded video.
    pub height: Option<i32>,
    /// Optional. Specify a level for the encoder profile (varies by encoder), e.g. 3, 3.1.
    pub level: Option<&'q types::GetLiveHlsStreamLevel>,
    /// The live stream id.
    pub live_stream_id: Option<&'q str>,
    /// Optional. The maximum audio bit depth.
    pub max_audio_bit_depth: Option<i32>,
    /// Optional. Specify a maximum number of audio channels to encode to, e.g. 2.
    pub max_audio_channels: Option<i32>,
    /// Optional. A specific maximum video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.
    pub max_framerate: Option<f32>,
    /// Optional. The max height.
    pub max_height: Option<i32>,
    /// Optional.
    pub max_ref_frames: Option<i32>,
    /// Optional. The maximum video bit depth.
    pub max_video_bit_depth: Option<i32>,
    /// Optional. The max width.
    pub max_width: Option<i32>,
    /// The media version id, if playing an alternate version.
    pub media_source_id: Option<&'q str>,
    /// The minimum number of segments.
    pub min_segments: Option<i32>,
    /// The streaming parameters.
    pub params: Option<&'q str>,
    /// The play session id.
    pub play_session_id: Option<&'q str>,
    /// Optional. Specify a specific an encoder profile (varies by encoder), e.g. main, baseline, high.
    pub profile: Option<&'q str>,
    /// Optional. Whether to require avc.
    pub require_avc: Option<bool>,
    /// Optional. Whether to require a non anamorphic stream.
    pub require_non_anamorphic: Option<bool>,
    /// The segment container.
    pub segment_container: Option<&'q types::GetLiveHlsStreamSegmentContainer>,
    /// The segment length.
    pub segment_length: Option<i32>,
    /// Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms.
    pub start_time_ticks: Option<i64>,
    /// Optional. If true, the original file will be streamed statically without any encoding. Use either no url extension or the original file extension. true/false.
    pub r#static: Option<bool>,
    /// Optional. The streaming options.
    pub stream_options: Option<&'q std::collections::HashMap<String, Option<String>>>,
    /// Optional. Specify a subtitle codec to encode to.
    pub subtitle_codec: Option<&'q types::GetLiveHlsStreamSubtitleCodec>,
    /// Optional. Specify the subtitle delivery method.
    pub subtitle_method: Option<types::SubtitleDeliveryMethod>,
    /// Optional. The index of the subtitle stream to use. If omitted no subtitles will be used.
    pub subtitle_stream_index: Option<i32>,
    /// The tag.
    pub tag: Option<&'q str>,
    /// Optional. The transcoding reason.
    pub transcode_reasons: Option<&'q str>,
    /// Optional. The maximum number of audio channels to transcode.
    pub transcoding_max_audio_channels: Option<i32>,
    /// Optional. Specify a video bitrate to encode to, e.g. 500000. If omitted this will be left to encoder defaults.
    pub video_bit_rate: Option<i32>,
    /// Optional. Specify a video codec to encode to, e.g. h264.
    pub video_codec: Option<&'q types::GetLiveHlsStreamVideoCodec>,
    /// Optional. The index of the video stream to use. If omitted the first video stream will be used.
    pub video_stream_index: Option<i32>,
    /// Optional. The fixed horizontal resolution of the encoded video.
    pub width: Option<i32>,
}

/// What `/Videos/{itemId}/main.m3u8` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetVariantHlsVideoPlaylist<'q> {
    /// Whether or not to allow copying of the audio stream url.
    pub allow_audio_stream_copy: Option<bool>,
    /// Whether or not to allow copying of the video stream url.
    pub allow_video_stream_copy: Option<bool>,
    /// Whether to always burn in subtitles when transcoding.
    pub always_burn_in_subtitle_when_transcoding: Option<bool>,
    /// Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults.
    pub audio_bit_rate: Option<i32>,
    /// Optional. Specify a specific number of audio channels to encode to, e.g. 2.
    pub audio_channels: Option<i32>,
    /// Optional. Specify an audio codec to encode to, e.g. mp3.
    pub audio_codec: Option<&'q types::GetVariantHlsVideoPlaylistAudioCodec>,
    /// Optional. Specify a specific audio sample rate, e.g. 44100.
    pub audio_sample_rate: Option<i32>,
    /// Optional. The index of the audio stream to use. If omitted the first audio stream will be used.
    pub audio_stream_index: Option<i32>,
    /// Optional. Whether to break on non key frames.
    pub break_on_non_key_frames: Option<bool>,
    /// Optional. The MediaBrowser.Model.Dlna.EncodingContext.
    pub context: Option<types::EncodingContext>,
    /// Whether or not to copy timestamps when transcoding with an offset. Defaults to false.
    pub copy_timestamps: Option<bool>,
    /// Optional. The limit of how many cpu cores to use.
    pub cpu_core_limit: Option<i32>,
    /// Optional. Whether to deinterlace the video.
    pub de_interlace: Option<bool>,
    /// The device id of the client requesting. Used to stop encoding processes when needed.
    pub device_id: Option<&'q str>,
    /// Optional. The dlna device profile id to utilize.
    pub device_profile_id: Option<&'q str>,
    /// Optional. Whether to enable Audio Encoding.
    pub enable_audio_vbr_encoding: Option<bool>,
    /// Whether or not to allow automatic stream copy if requested values match the original source. Defaults to true.
    pub enable_auto_stream_copy: Option<bool>,
    /// Optional. Whether to enable the MpegtsM2Ts mode.
    pub enable_mpegts_m2_ts_mode: Option<bool>,
    /// Optional. A specific video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.
    pub framerate: Option<f32>,
    /// Optional. The fixed vertical resolution of the encoded video.
    pub height: Option<i32>,
    /// Optional. Specify a level for the encoder profile (varies by encoder), e.g. 3, 3.1.
    pub level: Option<&'q types::GetVariantHlsVideoPlaylistLevel>,
    /// The live stream id.
    pub live_stream_id: Option<&'q str>,
    /// Optional. The maximum audio bit depth.
    pub max_audio_bit_depth: Option<i32>,
    /// Optional. Specify a maximum number of audio channels to encode to, e.g. 2.
    pub max_audio_channels: Option<i32>,
    /// Optional. A specific maximum video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.
    pub max_framerate: Option<f32>,
    /// Optional. The maximum vertical resolution of the encoded video.
    pub max_height: Option<i32>,
    /// Optional.
    pub max_ref_frames: Option<i32>,
    /// Optional. The maximum video bit depth.
    pub max_video_bit_depth: Option<i32>,
    /// Optional. The maximum horizontal resolution of the encoded video.
    pub max_width: Option<i32>,
    /// The media version id, if playing an alternate version.
    pub media_source_id: Option<&'q str>,
    /// The minimum number of segments.
    pub min_segments: Option<i32>,
    /// The streaming parameters.
    pub params: Option<&'q str>,
    /// The play session id.
    pub play_session_id: Option<&'q str>,
    /// Optional. Specify a specific an encoder profile (varies by encoder), e.g. main, baseline, high.
    pub profile: Option<&'q str>,
    /// Optional. Whether to require avc.
    pub require_avc: Option<bool>,
    /// Optional. Whether to require a non anamorphic stream.
    pub require_non_anamorphic: Option<bool>,
    /// The segment container.
    pub segment_container: Option<&'q types::GetVariantHlsVideoPlaylistSegmentContainer>,
    /// The segment length.
    pub segment_length: Option<i32>,
    /// Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms.
    pub start_time_ticks: Option<i64>,
    /// Optional. If true, the original file will be streamed statically without any encoding. Use either no url extension or the original file extension. true/false.
    pub r#static: Option<bool>,
    /// Optional. The streaming options.
    pub stream_options: Option<&'q std::collections::HashMap<String, Option<String>>>,
    /// Optional. Specify a subtitle codec to encode to.
    pub subtitle_codec: Option<&'q types::GetVariantHlsVideoPlaylistSubtitleCodec>,
    /// Optional. Specify the subtitle delivery method.
    pub subtitle_method: Option<types::SubtitleDeliveryMethod>,
    /// Optional. The index of the subtitle stream to use. If omitted no subtitles will be used.
    pub subtitle_stream_index: Option<i32>,
    /// The tag.
    pub tag: Option<&'q str>,
    /// Optional. The transcoding reason.
    pub transcode_reasons: Option<&'q str>,
    /// Optional. The maximum number of audio channels to transcode.
    pub transcoding_max_audio_channels: Option<i32>,
    /// Optional. Specify a video bitrate to encode to, e.g. 500000. If omitted this will be left to encoder defaults.
    pub video_bit_rate: Option<i32>,
    /// Optional. Specify a video codec to encode to, e.g. h264.
    pub video_codec: Option<&'q types::GetVariantHlsVideoPlaylistVideoCodec>,
    /// Optional. The index of the video stream to use. If omitted the first video stream will be used.
    pub video_stream_index: Option<i32>,
    /// Optional. The fixed horizontal resolution of the encoded video.
    pub width: Option<i32>,
}

/// What `/Videos/{itemId}/master.m3u8` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetMasterHlsVideoPlaylist<'q> {
    /// Whether or not to allow copying of the audio stream url.
    pub allow_audio_stream_copy: Option<bool>,
    /// Whether or not to allow copying of the video stream url.
    pub allow_video_stream_copy: Option<bool>,
    /// Whether to always burn in subtitles when transcoding.
    pub always_burn_in_subtitle_when_transcoding: Option<bool>,
    /// Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults.
    pub audio_bit_rate: Option<i32>,
    /// Optional. Specify a specific number of audio channels to encode to, e.g. 2.
    pub audio_channels: Option<i32>,
    /// Optional. Specify an audio codec to encode to, e.g. mp3.
    pub audio_codec: Option<&'q types::GetMasterHlsVideoPlaylistAudioCodec>,
    /// Optional. Specify a specific audio sample rate, e.g. 44100.
    pub audio_sample_rate: Option<i32>,
    /// Optional. The index of the audio stream to use. If omitted the first audio stream will be used.
    pub audio_stream_index: Option<i32>,
    /// Optional. Whether to break on non key frames.
    pub break_on_non_key_frames: Option<bool>,
    /// Optional. The MediaBrowser.Model.Dlna.EncodingContext.
    pub context: Option<types::EncodingContext>,
    /// Whether or not to copy timestamps when transcoding with an offset. Defaults to false.
    pub copy_timestamps: Option<bool>,
    /// Optional. The limit of how many cpu cores to use.
    pub cpu_core_limit: Option<i32>,
    /// Optional. Whether to deinterlace the video.
    pub de_interlace: Option<bool>,
    /// The device id of the client requesting. Used to stop encoding processes when needed.
    pub device_id: Option<&'q str>,
    /// Optional. The dlna device profile id to utilize.
    pub device_profile_id: Option<&'q str>,
    /// Enable adaptive bitrate streaming.
    pub enable_adaptive_bitrate_streaming: Option<bool>,
    /// Whether to enable Audio Encoding.
    pub enable_audio_vbr_encoding: Option<bool>,
    /// Whether or not to allow automatic stream copy if requested values match the original source. Defaults to true.
    pub enable_auto_stream_copy: Option<bool>,
    /// Optional. Whether to enable the MpegtsM2Ts mode.
    pub enable_mpegts_m2_ts_mode: Option<bool>,
    /// Enable trickplay image playlists being added to master playlist.
    pub enable_trickplay: Option<bool>,
    /// Optional. A specific video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.
    pub framerate: Option<f32>,
    /// Optional. The fixed vertical resolution of the encoded video.
    pub height: Option<i32>,
    /// Optional. Specify a level for the encoder profile (varies by encoder), e.g. 3, 3.1.
    pub level: Option<&'q types::GetMasterHlsVideoPlaylistLevel>,
    /// The live stream id.
    pub live_stream_id: Option<&'q str>,
    /// Optional. The maximum audio bit depth.
    pub max_audio_bit_depth: Option<i32>,
    /// Optional. Specify a maximum number of audio channels to encode to, e.g. 2.
    pub max_audio_channels: Option<i32>,
    /// Optional. A specific maximum video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.
    pub max_framerate: Option<f32>,
    /// Optional. The maximum vertical resolution of the encoded video.
    pub max_height: Option<i32>,
    /// Optional.
    pub max_ref_frames: Option<i32>,
    /// Optional. The maximum video bit depth.
    pub max_video_bit_depth: Option<i32>,
    /// Optional. The maximum horizontal resolution of the encoded video.
    pub max_width: Option<i32>,
    /// The minimum number of segments.
    pub min_segments: Option<i32>,
    /// The streaming parameters.
    pub params: Option<&'q str>,
    /// The play session id.
    pub play_session_id: Option<&'q str>,
    /// Optional. Specify a specific an encoder profile (varies by encoder), e.g. main, baseline, high.
    pub profile: Option<&'q str>,
    /// Optional. Whether to require avc.
    pub require_avc: Option<bool>,
    /// Optional. Whether to require a non anamorphic stream.
    pub require_non_anamorphic: Option<bool>,
    /// The segment container.
    pub segment_container: Option<&'q types::GetMasterHlsVideoPlaylistSegmentContainer>,
    /// The segment length.
    pub segment_length: Option<i32>,
    /// Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms.
    pub start_time_ticks: Option<i64>,
    /// Optional. If true, the original file will be streamed statically without any encoding. Use either no url extension or the original file extension. true/false.
    pub r#static: Option<bool>,
    /// Optional. The streaming options.
    pub stream_options: Option<&'q std::collections::HashMap<String, Option<String>>>,
    /// Optional. Specify a subtitle codec to encode to.
    pub subtitle_codec: Option<&'q types::GetMasterHlsVideoPlaylistSubtitleCodec>,
    /// Optional. Specify the subtitle delivery method.
    pub subtitle_method: Option<types::SubtitleDeliveryMethod>,
    /// Optional. The index of the subtitle stream to use. If omitted no subtitles will be used.
    pub subtitle_stream_index: Option<i32>,
    /// The tag.
    pub tag: Option<&'q str>,
    /// Optional. The transcoding reason.
    pub transcode_reasons: Option<&'q str>,
    /// Optional. The maximum number of audio channels to transcode.
    pub transcoding_max_audio_channels: Option<i32>,
    /// Optional. Specify a video bitrate to encode to, e.g. 500000. If omitted this will be left to encoder defaults.
    pub video_bit_rate: Option<i32>,
    /// Optional. Specify a video codec to encode to, e.g. h264.
    pub video_codec: Option<&'q types::GetMasterHlsVideoPlaylistVideoCodec>,
    /// Optional. The index of the video stream to use. If omitted the first video stream will be used.
    pub video_stream_index: Option<i32>,
    /// Optional. The fixed horizontal resolution of the encoded video.
    pub width: Option<i32>,
}

/// What `/Videos/{itemId}/master.m3u8` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct HeadMasterHlsVideoPlaylist<'q> {
    /// Whether or not to allow copying of the audio stream url.
    pub allow_audio_stream_copy: Option<bool>,
    /// Whether or not to allow copying of the video stream url.
    pub allow_video_stream_copy: Option<bool>,
    /// Whether to always burn in subtitles when transcoding.
    pub always_burn_in_subtitle_when_transcoding: Option<bool>,
    /// Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults.
    pub audio_bit_rate: Option<i32>,
    /// Optional. Specify a specific number of audio channels to encode to, e.g. 2.
    pub audio_channels: Option<i32>,
    /// Optional. Specify an audio codec to encode to, e.g. mp3.
    pub audio_codec: Option<&'q types::HeadMasterHlsVideoPlaylistAudioCodec>,
    /// Optional. Specify a specific audio sample rate, e.g. 44100.
    pub audio_sample_rate: Option<i32>,
    /// Optional. The index of the audio stream to use. If omitted the first audio stream will be used.
    pub audio_stream_index: Option<i32>,
    /// Optional. Whether to break on non key frames.
    pub break_on_non_key_frames: Option<bool>,
    /// Optional. The MediaBrowser.Model.Dlna.EncodingContext.
    pub context: Option<types::EncodingContext>,
    /// Whether or not to copy timestamps when transcoding with an offset. Defaults to false.
    pub copy_timestamps: Option<bool>,
    /// Optional. The limit of how many cpu cores to use.
    pub cpu_core_limit: Option<i32>,
    /// Optional. Whether to deinterlace the video.
    pub de_interlace: Option<bool>,
    /// The device id of the client requesting. Used to stop encoding processes when needed.
    pub device_id: Option<&'q str>,
    /// Optional. The dlna device profile id to utilize.
    pub device_profile_id: Option<&'q str>,
    /// Enable adaptive bitrate streaming.
    pub enable_adaptive_bitrate_streaming: Option<bool>,
    /// Whether to enable Audio Encoding.
    pub enable_audio_vbr_encoding: Option<bool>,
    /// Whether or not to allow automatic stream copy if requested values match the original source. Defaults to true.
    pub enable_auto_stream_copy: Option<bool>,
    /// Optional. Whether to enable the MpegtsM2Ts mode.
    pub enable_mpegts_m2_ts_mode: Option<bool>,
    /// Enable trickplay image playlists being added to master playlist.
    pub enable_trickplay: Option<bool>,
    /// Optional. A specific video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.
    pub framerate: Option<f32>,
    /// Optional. The fixed vertical resolution of the encoded video.
    pub height: Option<i32>,
    /// Optional. Specify a level for the encoder profile (varies by encoder), e.g. 3, 3.1.
    pub level: Option<&'q types::HeadMasterHlsVideoPlaylistLevel>,
    /// The live stream id.
    pub live_stream_id: Option<&'q str>,
    /// Optional. The maximum audio bit depth.
    pub max_audio_bit_depth: Option<i32>,
    /// Optional. Specify a maximum number of audio channels to encode to, e.g. 2.
    pub max_audio_channels: Option<i32>,
    /// Optional. A specific maximum video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.
    pub max_framerate: Option<f32>,
    /// Optional. The maximum vertical resolution of the encoded video.
    pub max_height: Option<i32>,
    /// Optional.
    pub max_ref_frames: Option<i32>,
    /// Optional. The maximum video bit depth.
    pub max_video_bit_depth: Option<i32>,
    /// Optional. The maximum horizontal resolution of the encoded video.
    pub max_width: Option<i32>,
    /// The minimum number of segments.
    pub min_segments: Option<i32>,
    /// The streaming parameters.
    pub params: Option<&'q str>,
    /// The play session id.
    pub play_session_id: Option<&'q str>,
    /// Optional. Specify a specific an encoder profile (varies by encoder), e.g. main, baseline, high.
    pub profile: Option<&'q str>,
    /// Optional. Whether to require avc.
    pub require_avc: Option<bool>,
    /// Optional. Whether to require a non anamorphic stream.
    pub require_non_anamorphic: Option<bool>,
    /// The segment container.
    pub segment_container: Option<&'q types::HeadMasterHlsVideoPlaylistSegmentContainer>,
    /// The segment length.
    pub segment_length: Option<i32>,
    /// Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms.
    pub start_time_ticks: Option<i64>,
    /// Optional. If true, the original file will be streamed statically without any encoding. Use either no url extension or the original file extension. true/false.
    pub r#static: Option<bool>,
    /// Optional. The streaming options.
    pub stream_options: Option<&'q std::collections::HashMap<String, Option<String>>>,
    /// Optional. Specify a subtitle codec to encode to.
    pub subtitle_codec: Option<&'q types::HeadMasterHlsVideoPlaylistSubtitleCodec>,
    /// Optional. Specify the subtitle delivery method.
    pub subtitle_method: Option<types::SubtitleDeliveryMethod>,
    /// Optional. The index of the subtitle stream to use. If omitted no subtitles will be used.
    pub subtitle_stream_index: Option<i32>,
    /// The tag.
    pub tag: Option<&'q str>,
    /// Optional. The transcoding reason.
    pub transcode_reasons: Option<&'q str>,
    /// Optional. The maximum number of audio channels to transcode.
    pub transcoding_max_audio_channels: Option<i32>,
    /// Optional. Specify a video bitrate to encode to, e.g. 500000. If omitted this will be left to encoder defaults.
    pub video_bit_rate: Option<i32>,
    /// Optional. Specify a video codec to encode to, e.g. h264.
    pub video_codec: Option<&'q types::HeadMasterHlsVideoPlaylistVideoCodec>,
    /// Optional. The index of the video stream to use. If omitted the first video stream will be used.
    pub video_stream_index: Option<i32>,
    /// Optional. The fixed horizontal resolution of the encoded video.
    pub width: Option<i32>,
}

/// What `/Videos/{routeItemId}/{routeMediaSourceId}/Subtitles/{routeIndex}/{routeStartPositionTicks}/Stream.{routeFormat}` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetSubtitleWithTicks<'q> {
    /// Optional. Whether to add a VTT time map.
    pub add_vtt_time_map: Option<bool>,
    /// Optional. Whether to copy the timestamps.
    pub copy_timestamps: Option<bool>,
    /// Optional. The end position of the subtitle in ticks.
    pub end_position_ticks: Option<i64>,
    /// The format of the returned subtitle.
    pub format: Option<&'q str>,
    /// The subtitle stream index.
    pub index: Option<i32>,
    /// The item id.
    pub item_id: Option<&'q uuid::Uuid>,
    /// The media source id.
    pub media_source_id: Option<&'q str>,
    /// The start position of the subtitle in ticks.
    pub start_position_ticks: Option<i64>,
}

/// What `/Videos/{routeItemId}/{routeMediaSourceId}/Subtitles/{routeIndex}/Stream.{routeFormat}` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetSubtitle<'q> {
    /// Optional. Whether to add a VTT time map.
    pub add_vtt_time_map: Option<bool>,
    /// Optional. Whether to copy the timestamps.
    pub copy_timestamps: Option<bool>,
    /// Optional. The end position of the subtitle in ticks.
    pub end_position_ticks: Option<i64>,
    /// The format of the returned subtitle.
    pub format: Option<&'q str>,
    /// The subtitle stream index.
    pub index: Option<i32>,
    /// The item id.
    pub item_id: Option<&'q uuid::Uuid>,
    /// The media source id.
    pub media_source_id: Option<&'q str>,
    /// The start position of the subtitle in ticks.
    pub start_position_ticks: Option<i64>,
}

/// What `/Videos/{itemId}/stream` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetVideoStream<'q> {
    /// Whether or not to allow copying of the audio stream url.
    pub allow_audio_stream_copy: Option<bool>,
    /// Whether or not to allow copying of the video stream url.
    pub allow_video_stream_copy: Option<bool>,
    /// Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults.
    pub audio_bit_rate: Option<i32>,
    /// Optional. Specify a specific number of audio channels to encode to, e.g. 2.
    pub audio_channels: Option<i32>,
    /// Optional. Specify an audio codec to encode to, e.g. mp3. If omitted the server will auto-select using the url's extension.
    pub audio_codec: Option<&'q types::GetVideoStreamAudioCodec>,
    /// Optional. Specify a specific audio sample rate, e.g. 44100.
    pub audio_sample_rate: Option<i32>,
    /// Optional. The index of the audio stream to use. If omitted the first audio stream will be used.
    pub audio_stream_index: Option<i32>,
    /// Optional. Whether to break on non key frames.
    pub break_on_non_key_frames: Option<bool>,
    /// The video container. Possible values are: ts, webm, asf, wmv, ogv, mp4, m4v, mkv, mpeg, mpg, avi, 3gp, wmv, wtv, m2ts, mov, iso, flv.
    pub container: Option<&'q types::GetVideoStreamContainer>,
    /// Optional. The MediaBrowser.Model.Dlna.EncodingContext.
    pub context: Option<types::EncodingContext>,
    /// Whether or not to copy timestamps when transcoding with an offset. Defaults to false.
    pub copy_timestamps: Option<bool>,
    /// Optional. The limit of how many cpu cores to use.
    pub cpu_core_limit: Option<i32>,
    /// Optional. Whether to deinterlace the video.
    pub de_interlace: Option<bool>,
    /// The device id of the client requesting. Used to stop encoding processes when needed.
    pub device_id: Option<&'q str>,
    /// Optional. The dlna device profile id to utilize.
    pub device_profile_id: Option<&'q str>,
    /// Optional. Whether to enable Audio Encoding.
    pub enable_audio_vbr_encoding: Option<bool>,
    /// Whether or not to allow automatic stream copy if requested values match the original source. Defaults to true.
    pub enable_auto_stream_copy: Option<bool>,
    /// Optional. Whether to enable the MpegtsM2Ts mode.
    pub enable_mpegts_m2_ts_mode: Option<bool>,
    /// Optional. A specific video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.
    pub framerate: Option<f32>,
    /// Optional. The fixed vertical resolution of the encoded video.
    pub height: Option<i32>,
    /// Optional. Specify a level for the encoder profile (varies by encoder), e.g. 3, 3.1.
    pub level: Option<&'q types::GetVideoStreamLevel>,
    /// The live stream id.
    pub live_stream_id: Option<&'q str>,
    /// Optional. The maximum audio bit depth.
    pub max_audio_bit_depth: Option<i32>,
    /// Optional. Specify a maximum number of audio channels to encode to, e.g. 2.
    pub max_audio_channels: Option<i32>,
    /// Optional. A specific maximum video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.
    pub max_framerate: Option<f32>,
    /// Optional. The maximum vertical resolution of the encoded video.
    pub max_height: Option<i32>,
    /// Optional.
    pub max_ref_frames: Option<i32>,
    /// Optional. The maximum video bit depth.
    pub max_video_bit_depth: Option<i32>,
    /// Optional. The maximum horizontal resolution of the encoded video.
    pub max_width: Option<i32>,
    /// The media version id, if playing an alternate version.
    pub media_source_id: Option<&'q str>,
    /// The minimum number of segments.
    pub min_segments: Option<i32>,
    /// The streaming parameters.
    pub params: Option<&'q str>,
    /// The play session id.
    pub play_session_id: Option<&'q str>,
    /// Optional. Specify a specific an encoder profile (varies by encoder), e.g. main, baseline, high.
    pub profile: Option<&'q str>,
    /// Optional. Whether to require avc.
    pub require_avc: Option<bool>,
    /// Optional. Whether to require a non anamorphic stream.
    pub require_non_anamorphic: Option<bool>,
    /// The segment container.
    pub segment_container: Option<&'q types::GetVideoStreamSegmentContainer>,
    /// The segment length.
    pub segment_length: Option<i32>,
    /// Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms.
    pub start_time_ticks: Option<i64>,
    /// Optional. If true, the original file will be streamed statically without any encoding. Use either no url extension or the original file extension. true/false.
    pub r#static: Option<bool>,
    /// Optional. The streaming options.
    pub stream_options: Option<&'q std::collections::HashMap<String, Option<String>>>,
    /// Optional. Specify a subtitle codec to encode to.
    pub subtitle_codec: Option<&'q types::GetVideoStreamSubtitleCodec>,
    /// Optional. Specify the subtitle delivery method.
    pub subtitle_method: Option<types::SubtitleDeliveryMethod>,
    /// Optional. The index of the subtitle stream to use. If omitted no subtitles will be used.
    pub subtitle_stream_index: Option<i32>,
    /// The tag.
    pub tag: Option<&'q str>,
    /// Optional. The transcoding reason.
    pub transcode_reasons: Option<&'q str>,
    /// Optional. The maximum number of audio channels to transcode.
    pub transcoding_max_audio_channels: Option<i32>,
    /// Optional. Specify a video bitrate to encode to, e.g. 500000. If omitted this will be left to encoder defaults.
    pub video_bit_rate: Option<i32>,
    /// Optional. Specify a video codec to encode to, e.g. h264. If omitted the server will auto-select using the url's extension.
    pub video_codec: Option<&'q types::GetVideoStreamVideoCodec>,
    /// Optional. The index of the video stream to use. If omitted the first video stream will be used.
    pub video_stream_index: Option<i32>,
    /// Optional. The fixed horizontal resolution of the encoded video.
    pub width: Option<i32>,
}

/// What `/Videos/{itemId}/stream` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct HeadVideoStream<'q> {
    /// Whether or not to allow copying of the audio stream url.
    pub allow_audio_stream_copy: Option<bool>,
    /// Whether or not to allow copying of the video stream url.
    pub allow_video_stream_copy: Option<bool>,
    /// Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults.
    pub audio_bit_rate: Option<i32>,
    /// Optional. Specify a specific number of audio channels to encode to, e.g. 2.
    pub audio_channels: Option<i32>,
    /// Optional. Specify an audio codec to encode to, e.g. mp3. If omitted the server will auto-select using the url's extension.
    pub audio_codec: Option<&'q types::HeadVideoStreamAudioCodec>,
    /// Optional. Specify a specific audio sample rate, e.g. 44100.
    pub audio_sample_rate: Option<i32>,
    /// Optional. The index of the audio stream to use. If omitted the first audio stream will be used.
    pub audio_stream_index: Option<i32>,
    /// Optional. Whether to break on non key frames.
    pub break_on_non_key_frames: Option<bool>,
    /// The video container. Possible values are: ts, webm, asf, wmv, ogv, mp4, m4v, mkv, mpeg, mpg, avi, 3gp, wmv, wtv, m2ts, mov, iso, flv.
    pub container: Option<&'q types::HeadVideoStreamContainer>,
    /// Optional. The MediaBrowser.Model.Dlna.EncodingContext.
    pub context: Option<types::EncodingContext>,
    /// Whether or not to copy timestamps when transcoding with an offset. Defaults to false.
    pub copy_timestamps: Option<bool>,
    /// Optional. The limit of how many cpu cores to use.
    pub cpu_core_limit: Option<i32>,
    /// Optional. Whether to deinterlace the video.
    pub de_interlace: Option<bool>,
    /// The device id of the client requesting. Used to stop encoding processes when needed.
    pub device_id: Option<&'q str>,
    /// Optional. The dlna device profile id to utilize.
    pub device_profile_id: Option<&'q str>,
    /// Optional. Whether to enable Audio Encoding.
    pub enable_audio_vbr_encoding: Option<bool>,
    /// Whether or not to allow automatic stream copy if requested values match the original source. Defaults to true.
    pub enable_auto_stream_copy: Option<bool>,
    /// Optional. Whether to enable the MpegtsM2Ts mode.
    pub enable_mpegts_m2_ts_mode: Option<bool>,
    /// Optional. A specific video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.
    pub framerate: Option<f32>,
    /// Optional. The fixed vertical resolution of the encoded video.
    pub height: Option<i32>,
    /// Optional. Specify a level for the encoder profile (varies by encoder), e.g. 3, 3.1.
    pub level: Option<&'q types::HeadVideoStreamLevel>,
    /// The live stream id.
    pub live_stream_id: Option<&'q str>,
    /// Optional. The maximum audio bit depth.
    pub max_audio_bit_depth: Option<i32>,
    /// Optional. Specify a maximum number of audio channels to encode to, e.g. 2.
    pub max_audio_channels: Option<i32>,
    /// Optional. A specific maximum video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.
    pub max_framerate: Option<f32>,
    /// Optional. The maximum vertical resolution of the encoded video.
    pub max_height: Option<i32>,
    /// Optional.
    pub max_ref_frames: Option<i32>,
    /// Optional. The maximum video bit depth.
    pub max_video_bit_depth: Option<i32>,
    /// Optional. The maximum horizontal resolution of the encoded video.
    pub max_width: Option<i32>,
    /// The media version id, if playing an alternate version.
    pub media_source_id: Option<&'q str>,
    /// The minimum number of segments.
    pub min_segments: Option<i32>,
    /// The streaming parameters.
    pub params: Option<&'q str>,
    /// The play session id.
    pub play_session_id: Option<&'q str>,
    /// Optional. Specify a specific an encoder profile (varies by encoder), e.g. main, baseline, high.
    pub profile: Option<&'q str>,
    /// Optional. Whether to require avc.
    pub require_avc: Option<bool>,
    /// Optional. Whether to require a non anamorphic stream.
    pub require_non_anamorphic: Option<bool>,
    /// The segment container.
    pub segment_container: Option<&'q types::HeadVideoStreamSegmentContainer>,
    /// The segment length.
    pub segment_length: Option<i32>,
    /// Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms.
    pub start_time_ticks: Option<i64>,
    /// Optional. If true, the original file will be streamed statically without any encoding. Use either no url extension or the original file extension. true/false.
    pub r#static: Option<bool>,
    /// Optional. The streaming options.
    pub stream_options: Option<&'q std::collections::HashMap<String, Option<String>>>,
    /// Optional. Specify a subtitle codec to encode to.
    pub subtitle_codec: Option<&'q types::HeadVideoStreamSubtitleCodec>,
    /// Optional. Specify the subtitle delivery method.
    pub subtitle_method: Option<types::SubtitleDeliveryMethod>,
    /// Optional. The index of the subtitle stream to use. If omitted no subtitles will be used.
    pub subtitle_stream_index: Option<i32>,
    /// The tag.
    pub tag: Option<&'q str>,
    /// Optional. The transcoding reason.
    pub transcode_reasons: Option<&'q str>,
    /// Optional. The maximum number of audio channels to transcode.
    pub transcoding_max_audio_channels: Option<i32>,
    /// Optional. Specify a video bitrate to encode to, e.g. 500000. If omitted this will be left to encoder defaults.
    pub video_bit_rate: Option<i32>,
    /// Optional. Specify a video codec to encode to, e.g. h264. If omitted the server will auto-select using the url's extension.
    pub video_codec: Option<&'q types::HeadVideoStreamVideoCodec>,
    /// Optional. The index of the video stream to use. If omitted the first video stream will be used.
    pub video_stream_index: Option<i32>,
    /// Optional. The fixed horizontal resolution of the encoded video.
    pub width: Option<i32>,
}

/// What `/Videos/{itemId}/stream.{container}` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetVideoStreamByContainer<'q> {
    /// Whether or not to allow copying of the audio stream url.
    pub allow_audio_stream_copy: Option<bool>,
    /// Whether or not to allow copying of the video stream url.
    pub allow_video_stream_copy: Option<bool>,
    /// Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults.
    pub audio_bit_rate: Option<i32>,
    /// Optional. Specify a specific number of audio channels to encode to, e.g. 2.
    pub audio_channels: Option<i32>,
    /// Optional. Specify an audio codec to encode to, e.g. mp3. If omitted the server will auto-select using the url's extension.
    pub audio_codec: Option<&'q types::GetVideoStreamByContainerAudioCodec>,
    /// Optional. Specify a specific audio sample rate, e.g. 44100.
    pub audio_sample_rate: Option<i32>,
    /// Optional. The index of the audio stream to use. If omitted the first audio stream will be used.
    pub audio_stream_index: Option<i32>,
    /// Optional. Whether to break on non key frames.
    pub break_on_non_key_frames: Option<bool>,
    /// Optional. The MediaBrowser.Model.Dlna.EncodingContext.
    pub context: Option<types::EncodingContext>,
    /// Whether or not to copy timestamps when transcoding with an offset. Defaults to false.
    pub copy_timestamps: Option<bool>,
    /// Optional. The limit of how many cpu cores to use.
    pub cpu_core_limit: Option<i32>,
    /// Optional. Whether to deinterlace the video.
    pub de_interlace: Option<bool>,
    /// The device id of the client requesting. Used to stop encoding processes when needed.
    pub device_id: Option<&'q str>,
    /// Optional. The dlna device profile id to utilize.
    pub device_profile_id: Option<&'q str>,
    /// Optional. Whether to enable Audio Encoding.
    pub enable_audio_vbr_encoding: Option<bool>,
    /// Whether or not to allow automatic stream copy if requested values match the original source. Defaults to true.
    pub enable_auto_stream_copy: Option<bool>,
    /// Optional. Whether to enable the MpegtsM2Ts mode.
    pub enable_mpegts_m2_ts_mode: Option<bool>,
    /// Optional. A specific video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.
    pub framerate: Option<f32>,
    /// Optional. The fixed vertical resolution of the encoded video.
    pub height: Option<i32>,
    /// Optional. Specify a level for the encoder profile (varies by encoder), e.g. 3, 3.1.
    pub level: Option<&'q types::GetVideoStreamByContainerLevel>,
    /// The live stream id.
    pub live_stream_id: Option<&'q str>,
    /// Optional. The maximum audio bit depth.
    pub max_audio_bit_depth: Option<i32>,
    /// Optional. Specify a maximum number of audio channels to encode to, e.g. 2.
    pub max_audio_channels: Option<i32>,
    /// Optional. A specific maximum video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.
    pub max_framerate: Option<f32>,
    /// Optional. The maximum vertical resolution of the encoded video.
    pub max_height: Option<i32>,
    /// Optional.
    pub max_ref_frames: Option<i32>,
    /// Optional. The maximum video bit depth.
    pub max_video_bit_depth: Option<i32>,
    /// Optional. The maximum horizontal resolution of the encoded video.
    pub max_width: Option<i32>,
    /// The media version id, if playing an alternate version.
    pub media_source_id: Option<&'q str>,
    /// The minimum number of segments.
    pub min_segments: Option<i32>,
    /// The streaming parameters.
    pub params: Option<&'q str>,
    /// The play session id.
    pub play_session_id: Option<&'q str>,
    /// Optional. Specify a specific an encoder profile (varies by encoder), e.g. main, baseline, high.
    pub profile: Option<&'q str>,
    /// Optional. Whether to require avc.
    pub require_avc: Option<bool>,
    /// Optional. Whether to require a non anamorphic stream.
    pub require_non_anamorphic: Option<bool>,
    /// The segment container.
    pub segment_container: Option<&'q types::GetVideoStreamByContainerSegmentContainer>,
    /// The segment length.
    pub segment_length: Option<i32>,
    /// Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms.
    pub start_time_ticks: Option<i64>,
    /// Optional. If true, the original file will be streamed statically without any encoding. Use either no url extension or the original file extension. true/false.
    pub r#static: Option<bool>,
    /// Optional. The streaming options.
    pub stream_options: Option<&'q std::collections::HashMap<String, Option<String>>>,
    /// Optional. Specify a subtitle codec to encode to.
    pub subtitle_codec: Option<&'q types::GetVideoStreamByContainerSubtitleCodec>,
    /// Optional. Specify the subtitle delivery method.
    pub subtitle_method: Option<types::SubtitleDeliveryMethod>,
    /// Optional. The index of the subtitle stream to use. If omitted no subtitles will be used.
    pub subtitle_stream_index: Option<i32>,
    /// The tag.
    pub tag: Option<&'q str>,
    /// Optional. The transcoding reason.
    pub transcode_reasons: Option<&'q str>,
    /// Optional. The maximum number of audio channels to transcode.
    pub transcoding_max_audio_channels: Option<i32>,
    /// Optional. Specify a video bitrate to encode to, e.g. 500000. If omitted this will be left to encoder defaults.
    pub video_bit_rate: Option<i32>,
    /// Optional. Specify a video codec to encode to, e.g. h264. If omitted the server will auto-select using the url's extension.
    pub video_codec: Option<&'q types::GetVideoStreamByContainerVideoCodec>,
    /// Optional. The index of the video stream to use. If omitted the first video stream will be used.
    pub video_stream_index: Option<i32>,
    /// Optional. The fixed horizontal resolution of the encoded video.
    pub width: Option<i32>,
}

/// What `/Videos/{itemId}/stream.{container}` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct HeadVideoStreamByContainer<'q> {
    /// Whether or not to allow copying of the audio stream url.
    pub allow_audio_stream_copy: Option<bool>,
    /// Whether or not to allow copying of the video stream url.
    pub allow_video_stream_copy: Option<bool>,
    /// Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults.
    pub audio_bit_rate: Option<i32>,
    /// Optional. Specify a specific number of audio channels to encode to, e.g. 2.
    pub audio_channels: Option<i32>,
    /// Optional. Specify an audio codec to encode to, e.g. mp3. If omitted the server will auto-select using the url's extension.
    pub audio_codec: Option<&'q types::HeadVideoStreamByContainerAudioCodec>,
    /// Optional. Specify a specific audio sample rate, e.g. 44100.
    pub audio_sample_rate: Option<i32>,
    /// Optional. The index of the audio stream to use. If omitted the first audio stream will be used.
    pub audio_stream_index: Option<i32>,
    /// Optional. Whether to break on non key frames.
    pub break_on_non_key_frames: Option<bool>,
    /// Optional. The MediaBrowser.Model.Dlna.EncodingContext.
    pub context: Option<types::EncodingContext>,
    /// Whether or not to copy timestamps when transcoding with an offset. Defaults to false.
    pub copy_timestamps: Option<bool>,
    /// Optional. The limit of how many cpu cores to use.
    pub cpu_core_limit: Option<i32>,
    /// Optional. Whether to deinterlace the video.
    pub de_interlace: Option<bool>,
    /// The device id of the client requesting. Used to stop encoding processes when needed.
    pub device_id: Option<&'q str>,
    /// Optional. The dlna device profile id to utilize.
    pub device_profile_id: Option<&'q str>,
    /// Optional. Whether to enable Audio Encoding.
    pub enable_audio_vbr_encoding: Option<bool>,
    /// Whether or not to allow automatic stream copy if requested values match the original source. Defaults to true.
    pub enable_auto_stream_copy: Option<bool>,
    /// Optional. Whether to enable the MpegtsM2Ts mode.
    pub enable_mpegts_m2_ts_mode: Option<bool>,
    /// Optional. A specific video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.
    pub framerate: Option<f32>,
    /// Optional. The fixed vertical resolution of the encoded video.
    pub height: Option<i32>,
    /// Optional. Specify a level for the encoder profile (varies by encoder), e.g. 3, 3.1.
    pub level: Option<&'q types::HeadVideoStreamByContainerLevel>,
    /// The live stream id.
    pub live_stream_id: Option<&'q str>,
    /// Optional. The maximum audio bit depth.
    pub max_audio_bit_depth: Option<i32>,
    /// Optional. Specify a maximum number of audio channels to encode to, e.g. 2.
    pub max_audio_channels: Option<i32>,
    /// Optional. A specific maximum video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.
    pub max_framerate: Option<f32>,
    /// Optional. The maximum vertical resolution of the encoded video.
    pub max_height: Option<i32>,
    /// Optional.
    pub max_ref_frames: Option<i32>,
    /// Optional. The maximum video bit depth.
    pub max_video_bit_depth: Option<i32>,
    /// Optional. The maximum horizontal resolution of the encoded video.
    pub max_width: Option<i32>,
    /// The media version id, if playing an alternate version.
    pub media_source_id: Option<&'q str>,
    /// The minimum number of segments.
    pub min_segments: Option<i32>,
    /// The streaming parameters.
    pub params: Option<&'q str>,
    /// The play session id.
    pub play_session_id: Option<&'q str>,
    /// Optional. Specify a specific an encoder profile (varies by encoder), e.g. main, baseline, high.
    pub profile: Option<&'q str>,
    /// Optional. Whether to require avc.
    pub require_avc: Option<bool>,
    /// Optional. Whether to require a non anamorphic stream.
    pub require_non_anamorphic: Option<bool>,
    /// The segment container.
    pub segment_container: Option<&'q types::HeadVideoStreamByContainerSegmentContainer>,
    /// The segment length.
    pub segment_length: Option<i32>,
    /// Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms.
    pub start_time_ticks: Option<i64>,
    /// Optional. If true, the original file will be streamed statically without any encoding. Use either no url extension or the original file extension. true/false.
    pub r#static: Option<bool>,
    /// Optional. The streaming options.
    pub stream_options: Option<&'q std::collections::HashMap<String, Option<String>>>,
    /// Optional. Specify a subtitle codec to encode to.
    pub subtitle_codec: Option<&'q types::HeadVideoStreamByContainerSubtitleCodec>,
    /// Optional. Specify the subtitle delivery method.
    pub subtitle_method: Option<types::SubtitleDeliveryMethod>,
    /// Optional. The index of the subtitle stream to use. If omitted no subtitles will be used.
    pub subtitle_stream_index: Option<i32>,
    /// The tag.
    pub tag: Option<&'q str>,
    /// Optional. The transcoding reason.
    pub transcode_reasons: Option<&'q str>,
    /// Optional. The maximum number of audio channels to transcode.
    pub transcoding_max_audio_channels: Option<i32>,
    /// Optional. Specify a video bitrate to encode to, e.g. 500000. If omitted this will be left to encoder defaults.
    pub video_bit_rate: Option<i32>,
    /// Optional. Specify a video codec to encode to, e.g. h264. If omitted the server will auto-select using the url's extension.
    pub video_codec: Option<&'q types::HeadVideoStreamByContainerVideoCodec>,
    /// Optional. The index of the video stream to use. If omitted the first video stream will be used.
    pub video_stream_index: Option<i32>,
    /// Optional. The fixed horizontal resolution of the encoded video.
    pub width: Option<i32>,
}
