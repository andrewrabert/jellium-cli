use crate::types;

/// What `/Audio/{itemId}/stream` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetAudioStream<'q> {
    /// Whether or not to allow copying of the audio stream url.
    pub allow_audio_stream_copy: Option<bool>,
    /// Whether or not to allow copying of the video stream url.
    pub allow_video_stream_copy: Option<bool>,
    /// Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults.
    pub audio_bit_rate: Option<i32>,
    /// Optional. Specify a specific number of audio channels to encode to, e.g. 2.
    pub audio_channels: Option<i32>,
    /// Optional. Specify an audio codec to encode to, e.g. mp3. If omitted the server will auto-select using the url's extension.
    pub audio_codec: Option<&'q types::GetAudioStreamAudioCodec>,
    /// Optional. Specify a specific audio sample rate, e.g. 44100.
    pub audio_sample_rate: Option<i32>,
    /// Optional. The index of the audio stream to use. If omitted the first audio stream will be used.
    pub audio_stream_index: Option<i32>,
    /// Optional. Whether to break on non key frames.
    pub break_on_non_key_frames: Option<bool>,
    /// The audio container.
    pub container: Option<&'q types::GetAudioStreamContainer>,
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
    pub level: Option<&'q types::GetAudioStreamLevel>,
    /// The live stream id.
    pub live_stream_id: Option<&'q str>,
    /// Optional. The maximum audio bit depth.
    pub max_audio_bit_depth: Option<i32>,
    /// Optional. Specify a maximum number of audio channels to encode to, e.g. 2.
    pub max_audio_channels: Option<i32>,
    /// Optional. A specific maximum video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.
    pub max_framerate: Option<f32>,
    /// Optional.
    pub max_ref_frames: Option<i32>,
    /// Optional. The maximum video bit depth.
    pub max_video_bit_depth: Option<i32>,
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
    pub segment_container: Option<&'q types::GetAudioStreamSegmentContainer>,
    /// The segment length.
    pub segment_length: Option<i32>,
    /// Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms.
    pub start_time_ticks: Option<i64>,
    /// Optional. If true, the original file will be streamed statically without any encoding. Use either no url extension or the original file extension. true/false.
    pub r#static: Option<bool>,
    /// Optional. The streaming options.
    pub stream_options: Option<&'q std::collections::HashMap<String, Option<String>>>,
    /// Optional. Specify a subtitle codec to encode to.
    pub subtitle_codec: Option<&'q types::GetAudioStreamSubtitleCodec>,
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
    pub video_codec: Option<&'q types::GetAudioStreamVideoCodec>,
    /// Optional. The index of the video stream to use. If omitted the first video stream will be used.
    pub video_stream_index: Option<i32>,
    /// Optional. The fixed horizontal resolution of the encoded video.
    pub width: Option<i32>,
}

/// What `/Audio/{itemId}/stream` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct HeadAudioStream<'q> {
    /// Whether or not to allow copying of the audio stream url.
    pub allow_audio_stream_copy: Option<bool>,
    /// Whether or not to allow copying of the video stream url.
    pub allow_video_stream_copy: Option<bool>,
    /// Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults.
    pub audio_bit_rate: Option<i32>,
    /// Optional. Specify a specific number of audio channels to encode to, e.g. 2.
    pub audio_channels: Option<i32>,
    /// Optional. Specify an audio codec to encode to, e.g. mp3. If omitted the server will auto-select using the url's extension.
    pub audio_codec: Option<&'q types::HeadAudioStreamAudioCodec>,
    /// Optional. Specify a specific audio sample rate, e.g. 44100.
    pub audio_sample_rate: Option<i32>,
    /// Optional. The index of the audio stream to use. If omitted the first audio stream will be used.
    pub audio_stream_index: Option<i32>,
    /// Optional. Whether to break on non key frames.
    pub break_on_non_key_frames: Option<bool>,
    /// The audio container.
    pub container: Option<&'q types::HeadAudioStreamContainer>,
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
    pub level: Option<&'q types::HeadAudioStreamLevel>,
    /// The live stream id.
    pub live_stream_id: Option<&'q str>,
    /// Optional. The maximum audio bit depth.
    pub max_audio_bit_depth: Option<i32>,
    /// Optional. Specify a maximum number of audio channels to encode to, e.g. 2.
    pub max_audio_channels: Option<i32>,
    /// Optional. A specific maximum video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.
    pub max_framerate: Option<f32>,
    /// Optional.
    pub max_ref_frames: Option<i32>,
    /// Optional. The maximum video bit depth.
    pub max_video_bit_depth: Option<i32>,
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
    pub segment_container: Option<&'q types::HeadAudioStreamSegmentContainer>,
    /// The segment length.
    pub segment_length: Option<i32>,
    /// Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms.
    pub start_time_ticks: Option<i64>,
    /// Optional. If true, the original file will be streamed statically without any encoding. Use either no url extension or the original file extension. true/false.
    pub r#static: Option<bool>,
    /// Optional. The streaming options.
    pub stream_options: Option<&'q std::collections::HashMap<String, Option<String>>>,
    /// Optional. Specify a subtitle codec to encode to.
    pub subtitle_codec: Option<&'q types::HeadAudioStreamSubtitleCodec>,
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
    pub video_codec: Option<&'q types::HeadAudioStreamVideoCodec>,
    /// Optional. The index of the video stream to use. If omitted the first video stream will be used.
    pub video_stream_index: Option<i32>,
    /// Optional. The fixed horizontal resolution of the encoded video.
    pub width: Option<i32>,
}

/// What `/Audio/{itemId}/stream.{container}` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetAudioStreamByContainer<'q> {
    /// Whether or not to allow copying of the audio stream url.
    pub allow_audio_stream_copy: Option<bool>,
    /// Whether or not to allow copying of the video stream url.
    pub allow_video_stream_copy: Option<bool>,
    /// Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults.
    pub audio_bit_rate: Option<i32>,
    /// Optional. Specify a specific number of audio channels to encode to, e.g. 2.
    pub audio_channels: Option<i32>,
    /// Optional. Specify an audio codec to encode to, e.g. mp3. If omitted the server will auto-select using the url's extension.
    pub audio_codec: Option<&'q types::GetAudioStreamByContainerAudioCodec>,
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
    pub level: Option<&'q types::GetAudioStreamByContainerLevel>,
    /// The live stream id.
    pub live_stream_id: Option<&'q str>,
    /// Optional. The maximum audio bit depth.
    pub max_audio_bit_depth: Option<i32>,
    /// Optional. Specify a maximum number of audio channels to encode to, e.g. 2.
    pub max_audio_channels: Option<i32>,
    /// Optional. A specific maximum video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.
    pub max_framerate: Option<f32>,
    /// Optional.
    pub max_ref_frames: Option<i32>,
    /// Optional. The maximum video bit depth.
    pub max_video_bit_depth: Option<i32>,
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
    pub segment_container: Option<&'q types::GetAudioStreamByContainerSegmentContainer>,
    /// The segment length.
    pub segment_length: Option<i32>,
    /// Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms.
    pub start_time_ticks: Option<i64>,
    /// Optional. If true, the original file will be streamed statically without any encoding. Use either no url extension or the original file extension. true/false.
    pub r#static: Option<bool>,
    /// Optional. The streaming options.
    pub stream_options: Option<&'q std::collections::HashMap<String, Option<String>>>,
    /// Optional. Specify a subtitle codec to encode to.
    pub subtitle_codec: Option<&'q types::GetAudioStreamByContainerSubtitleCodec>,
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
    pub video_codec: Option<&'q types::GetAudioStreamByContainerVideoCodec>,
    /// Optional. The index of the video stream to use. If omitted the first video stream will be used.
    pub video_stream_index: Option<i32>,
    /// Optional. The fixed horizontal resolution of the encoded video.
    pub width: Option<i32>,
}

/// What `/Audio/{itemId}/stream.{container}` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct HeadAudioStreamByContainer<'q> {
    /// Whether or not to allow copying of the audio stream url.
    pub allow_audio_stream_copy: Option<bool>,
    /// Whether or not to allow copying of the video stream url.
    pub allow_video_stream_copy: Option<bool>,
    /// Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults.
    pub audio_bit_rate: Option<i32>,
    /// Optional. Specify a specific number of audio channels to encode to, e.g. 2.
    pub audio_channels: Option<i32>,
    /// Optional. Specify an audio codec to encode to, e.g. mp3. If omitted the server will auto-select using the url's extension.
    pub audio_codec: Option<&'q types::HeadAudioStreamByContainerAudioCodec>,
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
    pub level: Option<&'q types::HeadAudioStreamByContainerLevel>,
    /// The live stream id.
    pub live_stream_id: Option<&'q str>,
    /// Optional. The maximum audio bit depth.
    pub max_audio_bit_depth: Option<i32>,
    /// Optional. Specify a maximum number of audio channels to encode to, e.g. 2.
    pub max_audio_channels: Option<i32>,
    /// Optional. A specific maximum video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.
    pub max_framerate: Option<f32>,
    /// Optional.
    pub max_ref_frames: Option<i32>,
    /// Optional. The maximum video bit depth.
    pub max_video_bit_depth: Option<i32>,
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
    pub segment_container: Option<&'q types::HeadAudioStreamByContainerSegmentContainer>,
    /// The segment length.
    pub segment_length: Option<i32>,
    /// Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms.
    pub start_time_ticks: Option<i64>,
    /// Optional. If true, the original file will be streamed statically without any encoding. Use either no url extension or the original file extension. true/false.
    pub r#static: Option<bool>,
    /// Optional. The streaming options.
    pub stream_options: Option<&'q std::collections::HashMap<String, Option<String>>>,
    /// Optional. Specify a subtitle codec to encode to.
    pub subtitle_codec: Option<&'q types::HeadAudioStreamByContainerSubtitleCodec>,
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
    pub video_codec: Option<&'q types::HeadAudioStreamByContainerVideoCodec>,
    /// Optional. The index of the video stream to use. If omitted the first video stream will be used.
    pub video_stream_index: Option<i32>,
    /// Optional. The fixed horizontal resolution of the encoded video.
    pub width: Option<i32>,
}

/// The whole of `/Audio/{itemId}/hls1/{playlistId}/{segmentId}.{container}`.
#[derive(Debug, Clone)]
pub struct GetHlsAudioSegment<'q> {
    pub item_id: &'q uuid::Uuid,
    pub playlist_id: &'q str,
    pub segment_id: i32,
    pub container: &'q types::GetHlsAudioSegmentContainer,
    pub actual_segment_length_ticks: i64,
    pub allow_audio_stream_copy: Option<bool>,
    pub allow_video_stream_copy: Option<bool>,
    pub audio_bit_rate: Option<i32>,
    pub audio_channels: Option<i32>,
    pub audio_codec: Option<&'q types::GetHlsAudioSegmentAudioCodec>,
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
    pub level: Option<&'q types::GetHlsAudioSegmentLevel>,
    pub live_stream_id: Option<&'q str>,
    pub max_audio_bit_depth: Option<i32>,
    pub max_audio_channels: Option<i32>,
    pub max_framerate: Option<f32>,
    pub max_ref_frames: Option<i32>,
    pub max_streaming_bitrate: Option<i32>,
    pub max_video_bit_depth: Option<i32>,
    pub media_source_id: Option<&'q str>,
    pub min_segments: Option<i32>,
    pub params: Option<&'q str>,
    pub play_session_id: Option<&'q str>,
    pub profile: Option<&'q str>,
    pub require_avc: Option<bool>,
    pub require_non_anamorphic: Option<bool>,
    pub runtime_ticks: i64,
    pub segment_container: Option<&'q types::GetHlsAudioSegmentSegmentContainer>,
    pub segment_length: Option<i32>,
    pub start_time_ticks: Option<i64>,
    pub r#static: Option<bool>,
    pub stream_options: Option<&'q std::collections::HashMap<String, Option<String>>>,
    pub subtitle_codec: Option<&'q types::GetHlsAudioSegmentSubtitleCodec>,
    pub subtitle_method: Option<types::SubtitleDeliveryMethod>,
    pub subtitle_stream_index: Option<i32>,
    pub tag: Option<&'q str>,
    pub transcode_reasons: Option<&'q str>,
    pub transcoding_max_audio_channels: Option<i32>,
    pub video_bit_rate: Option<i32>,
    pub video_codec: Option<&'q types::GetHlsAudioSegmentVideoCodec>,
    pub video_stream_index: Option<i32>,
    pub width: Option<i32>,
}

/// What `/Audio/{itemId}/main.m3u8` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetVariantHlsAudioPlaylist<'q> {
    /// Whether or not to allow copying of the audio stream url.
    pub allow_audio_stream_copy: Option<bool>,
    /// Whether or not to allow copying of the video stream url.
    pub allow_video_stream_copy: Option<bool>,
    /// Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults.
    pub audio_bit_rate: Option<i32>,
    /// Optional. Specify a specific number of audio channels to encode to, e.g. 2.
    pub audio_channels: Option<i32>,
    /// Optional. Specify an audio codec to encode to, e.g. mp3.
    pub audio_codec: Option<&'q types::GetVariantHlsAudioPlaylistAudioCodec>,
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
    pub level: Option<&'q types::GetVariantHlsAudioPlaylistLevel>,
    /// The live stream id.
    pub live_stream_id: Option<&'q str>,
    /// Optional. The maximum audio bit depth.
    pub max_audio_bit_depth: Option<i32>,
    /// Optional. Specify a maximum number of audio channels to encode to, e.g. 2.
    pub max_audio_channels: Option<i32>,
    /// Optional. A specific maximum video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.
    pub max_framerate: Option<f32>,
    /// Optional.
    pub max_ref_frames: Option<i32>,
    /// Optional. The maximum streaming bitrate.
    pub max_streaming_bitrate: Option<i32>,
    /// Optional. The maximum video bit depth.
    pub max_video_bit_depth: Option<i32>,
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
    pub segment_container: Option<&'q types::GetVariantHlsAudioPlaylistSegmentContainer>,
    /// The segment length.
    pub segment_length: Option<i32>,
    /// Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms.
    pub start_time_ticks: Option<i64>,
    /// Optional. If true, the original file will be streamed statically without any encoding. Use either no url extension or the original file extension. true/false.
    pub r#static: Option<bool>,
    /// Optional. The streaming options.
    pub stream_options: Option<&'q std::collections::HashMap<String, Option<String>>>,
    /// Optional. Specify a subtitle codec to encode to.
    pub subtitle_codec: Option<&'q types::GetVariantHlsAudioPlaylistSubtitleCodec>,
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
    pub video_codec: Option<&'q types::GetVariantHlsAudioPlaylistVideoCodec>,
    /// Optional. The index of the video stream to use. If omitted the first video stream will be used.
    pub video_stream_index: Option<i32>,
    /// Optional. The fixed horizontal resolution of the encoded video.
    pub width: Option<i32>,
}

/// What `/Audio/{itemId}/master.m3u8` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetMasterHlsAudioPlaylist<'q> {
    /// Whether or not to allow copying of the audio stream url.
    pub allow_audio_stream_copy: Option<bool>,
    /// Whether or not to allow copying of the video stream url.
    pub allow_video_stream_copy: Option<bool>,
    /// Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults.
    pub audio_bit_rate: Option<i32>,
    /// Optional. Specify a specific number of audio channels to encode to, e.g. 2.
    pub audio_channels: Option<i32>,
    /// Optional. Specify an audio codec to encode to, e.g. mp3.
    pub audio_codec: Option<&'q types::GetMasterHlsAudioPlaylistAudioCodec>,
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
    pub level: Option<&'q types::GetMasterHlsAudioPlaylistLevel>,
    /// The live stream id.
    pub live_stream_id: Option<&'q str>,
    /// Optional. The maximum audio bit depth.
    pub max_audio_bit_depth: Option<i32>,
    /// Optional. Specify a maximum number of audio channels to encode to, e.g. 2.
    pub max_audio_channels: Option<i32>,
    /// Optional. A specific maximum video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.
    pub max_framerate: Option<f32>,
    /// Optional.
    pub max_ref_frames: Option<i32>,
    /// Optional. The maximum streaming bitrate.
    pub max_streaming_bitrate: Option<i32>,
    /// Optional. The maximum video bit depth.
    pub max_video_bit_depth: Option<i32>,
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
    pub segment_container: Option<&'q types::GetMasterHlsAudioPlaylistSegmentContainer>,
    /// The segment length.
    pub segment_length: Option<i32>,
    /// Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms.
    pub start_time_ticks: Option<i64>,
    /// Optional. If true, the original file will be streamed statically without any encoding. Use either no url extension or the original file extension. true/false.
    pub r#static: Option<bool>,
    /// Optional. The streaming options.
    pub stream_options: Option<&'q std::collections::HashMap<String, Option<String>>>,
    /// Optional. Specify a subtitle codec to encode to.
    pub subtitle_codec: Option<&'q types::GetMasterHlsAudioPlaylistSubtitleCodec>,
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
    pub video_codec: Option<&'q types::GetMasterHlsAudioPlaylistVideoCodec>,
    /// Optional. The index of the video stream to use. If omitted the first video stream will be used.
    pub video_stream_index: Option<i32>,
    /// Optional. The fixed horizontal resolution of the encoded video.
    pub width: Option<i32>,
}

/// What `/Audio/{itemId}/master.m3u8` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct HeadMasterHlsAudioPlaylist<'q> {
    /// Whether or not to allow copying of the audio stream url.
    pub allow_audio_stream_copy: Option<bool>,
    /// Whether or not to allow copying of the video stream url.
    pub allow_video_stream_copy: Option<bool>,
    /// Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults.
    pub audio_bit_rate: Option<i32>,
    /// Optional. Specify a specific number of audio channels to encode to, e.g. 2.
    pub audio_channels: Option<i32>,
    /// Optional. Specify an audio codec to encode to, e.g. mp3.
    pub audio_codec: Option<&'q types::HeadMasterHlsAudioPlaylistAudioCodec>,
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
    pub level: Option<&'q types::HeadMasterHlsAudioPlaylistLevel>,
    /// The live stream id.
    pub live_stream_id: Option<&'q str>,
    /// Optional. The maximum audio bit depth.
    pub max_audio_bit_depth: Option<i32>,
    /// Optional. Specify a maximum number of audio channels to encode to, e.g. 2.
    pub max_audio_channels: Option<i32>,
    /// Optional. A specific maximum video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.
    pub max_framerate: Option<f32>,
    /// Optional.
    pub max_ref_frames: Option<i32>,
    /// Optional. The maximum streaming bitrate.
    pub max_streaming_bitrate: Option<i32>,
    /// Optional. The maximum video bit depth.
    pub max_video_bit_depth: Option<i32>,
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
    pub segment_container: Option<&'q types::HeadMasterHlsAudioPlaylistSegmentContainer>,
    /// The segment length.
    pub segment_length: Option<i32>,
    /// Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms.
    pub start_time_ticks: Option<i64>,
    /// Optional. If true, the original file will be streamed statically without any encoding. Use either no url extension or the original file extension. true/false.
    pub r#static: Option<bool>,
    /// Optional. The streaming options.
    pub stream_options: Option<&'q std::collections::HashMap<String, Option<String>>>,
    /// Optional. Specify a subtitle codec to encode to.
    pub subtitle_codec: Option<&'q types::HeadMasterHlsAudioPlaylistSubtitleCodec>,
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
    pub video_codec: Option<&'q types::HeadMasterHlsAudioPlaylistVideoCodec>,
    /// Optional. The index of the video stream to use. If omitted the first video stream will be used.
    pub video_stream_index: Option<i32>,
    /// Optional. The fixed horizontal resolution of the encoded video.
    pub width: Option<i32>,
}

/// What `/Audio/{itemId}/universal` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetUniversalAudioStream<'q> {
    /// Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults.
    pub audio_bit_rate: Option<i32>,
    /// Optional. The audio codec to transcode to.
    pub audio_codec: Option<&'q types::GetUniversalAudioStreamAudioCodec>,
    /// Optional. Whether to break on non key frames.
    pub break_on_non_key_frames: Option<bool>,
    /// Optional. The audio container.
    pub container: Option<&'q Vec<String>>,
    /// The device id of the client requesting. Used to stop encoding processes when needed.
    pub device_id: Option<&'q str>,
    /// Optional. Whether to enable Audio Encoding.
    pub enable_audio_vbr_encoding: Option<bool>,
    /// Whether to enable redirection. Defaults to true.
    pub enable_redirection: Option<bool>,
    /// Optional. Whether to enable remote media.
    pub enable_remote_media: Option<bool>,
    /// Optional. The maximum audio bit depth.
    pub max_audio_bit_depth: Option<i32>,
    /// Optional. The maximum number of audio channels.
    pub max_audio_channels: Option<i32>,
    /// Optional. The maximum audio sample rate.
    pub max_audio_sample_rate: Option<i32>,
    /// Optional. The maximum streaming bitrate.
    pub max_streaming_bitrate: Option<i32>,
    /// The media version id, if playing an alternate version.
    pub media_source_id: Option<&'q str>,
    /// Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms.
    pub start_time_ticks: Option<i64>,
    /// Optional. The number of how many audio channels to transcode to.
    pub transcoding_audio_channels: Option<i32>,
    /// Optional. The container to transcode to.
    pub transcoding_container: Option<&'q types::GetUniversalAudioStreamTranscodingContainer>,
    /// Optional. The transcoding protocol.
    pub transcoding_protocol: Option<types::MediaStreamProtocol>,
    /// Optional. The user id.
    pub user_id: Option<&'q uuid::Uuid>,
}

/// What `/Audio/{itemId}/universal` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct HeadUniversalAudioStream<'q> {
    /// Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults.
    pub audio_bit_rate: Option<i32>,
    /// Optional. The audio codec to transcode to.
    pub audio_codec: Option<&'q types::HeadUniversalAudioStreamAudioCodec>,
    /// Optional. Whether to break on non key frames.
    pub break_on_non_key_frames: Option<bool>,
    /// Optional. The audio container.
    pub container: Option<&'q Vec<String>>,
    /// The device id of the client requesting. Used to stop encoding processes when needed.
    pub device_id: Option<&'q str>,
    /// Optional. Whether to enable Audio Encoding.
    pub enable_audio_vbr_encoding: Option<bool>,
    /// Whether to enable redirection. Defaults to true.
    pub enable_redirection: Option<bool>,
    /// Optional. Whether to enable remote media.
    pub enable_remote_media: Option<bool>,
    /// Optional. The maximum audio bit depth.
    pub max_audio_bit_depth: Option<i32>,
    /// Optional. The maximum number of audio channels.
    pub max_audio_channels: Option<i32>,
    /// Optional. The maximum audio sample rate.
    pub max_audio_sample_rate: Option<i32>,
    /// Optional. The maximum streaming bitrate.
    pub max_streaming_bitrate: Option<i32>,
    /// The media version id, if playing an alternate version.
    pub media_source_id: Option<&'q str>,
    /// Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms.
    pub start_time_ticks: Option<i64>,
    /// Optional. The number of how many audio channels to transcode to.
    pub transcoding_audio_channels: Option<i32>,
    /// Optional. The container to transcode to.
    pub transcoding_container: Option<&'q types::HeadUniversalAudioStreamTranscodingContainer>,
    /// Optional. The transcoding protocol.
    pub transcoding_protocol: Option<types::MediaStreamProtocol>,
    /// Optional. The user id.
    pub user_id: Option<&'q uuid::Uuid>,
}
