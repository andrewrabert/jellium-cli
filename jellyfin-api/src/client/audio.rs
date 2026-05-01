use crate::types;
use crate::error::Error;
use crate::util::encode_path;
use crate::Client;

impl Client {
    #[doc = "Gets an audio stream\n\nSends a `GET` request to `/Audio/{itemId}/stream`\n\nArguments:\n- `item_id`: The item id.\n- `allow_audio_stream_copy`: Whether or not to allow copying of the audio stream url.\n- `allow_video_stream_copy`: Whether or not to allow copying of the video stream url.\n- `audio_bit_rate`: Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults.\n- `audio_channels`: Optional. Specify a specific number of audio channels to encode to, e.g. 2.\n- `audio_codec`: Optional. Specify an audio codec to encode to, e.g. mp3. If omitted the server will auto-select using the url's extension.\n- `audio_sample_rate`: Optional. Specify a specific audio sample rate, e.g. 44100.\n- `audio_stream_index`: Optional. The index of the audio stream to use. If omitted the first audio stream will be used.\n- `break_on_non_key_frames`: Optional. Whether to break on non key frames.\n- `container`: The audio container.\n- `context`: Optional. The MediaBrowser.Model.Dlna.EncodingContext.\n- `copy_timestamps`: Whether or not to copy timestamps when transcoding with an offset. Defaults to false.\n- `cpu_core_limit`: Optional. The limit of how many cpu cores to use.\n- `de_interlace`: Optional. Whether to deinterlace the video.\n- `device_id`: The device id of the client requesting. Used to stop encoding processes when needed.\n- `device_profile_id`: Optional. The dlna device profile id to utilize.\n- `enable_audio_vbr_encoding`: Optional. Whether to enable Audio Encoding.\n- `enable_auto_stream_copy`: Whether or not to allow automatic stream copy if requested values match the original source. Defaults to true.\n- `enable_mpegts_m2_ts_mode`: Optional. Whether to enable the MpegtsM2Ts mode.\n- `framerate`: Optional. A specific video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.\n- `height`: Optional. The fixed vertical resolution of the encoded video.\n- `level`: Optional. Specify a level for the encoder profile (varies by encoder), e.g. 3, 3.1.\n- `live_stream_id`: The live stream id.\n- `max_audio_bit_depth`: Optional. The maximum audio bit depth.\n- `max_audio_channels`: Optional. Specify a maximum number of audio channels to encode to, e.g. 2.\n- `max_framerate`: Optional. A specific maximum video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.\n- `max_ref_frames`: Optional.\n- `max_video_bit_depth`: Optional. The maximum video bit depth.\n- `media_source_id`: The media version id, if playing an alternate version.\n- `min_segments`: The minimum number of segments.\n- `params`: The streaming parameters.\n- `play_session_id`: The play session id.\n- `profile`: Optional. Specify a specific an encoder profile (varies by encoder), e.g. main, baseline, high.\n- `require_avc`: Optional. Whether to require avc.\n- `require_non_anamorphic`: Optional. Whether to require a non anamorphic stream.\n- `segment_container`: The segment container.\n- `segment_length`: The segment length.\n- `start_time_ticks`: Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms.\n- `static_`: Optional. If true, the original file will be streamed statically without any encoding. Use either no url extension or the original file extension. true/false.\n- `stream_options`: Optional. The streaming options.\n- `subtitle_codec`: Optional. Specify a subtitle codec to encode to.\n- `subtitle_method`: Optional. Specify the subtitle delivery method.\n- `subtitle_stream_index`: Optional. The index of the subtitle stream to use. If omitted no subtitles will be used.\n- `tag`: The tag.\n- `transcode_reasons`: Optional. The transcoding reason.\n- `transcoding_max_audio_channels`: Optional. The maximum number of audio channels to transcode.\n- `video_bit_rate`: Optional. Specify a video bitrate to encode to, e.g. 500000. If omitted this will be left to encoder defaults.\n- `video_codec`: Optional. Specify a video codec to encode to, e.g. h264. If omitted the server will auto-select using the url's extension.\n- `video_stream_index`: Optional. The index of the video stream to use. If omitted the first video stream will be used.\n- `width`: Optional. The fixed horizontal resolution of the encoded video.\n"]
    pub async fn get_audio_stream(
        &self,
        item_id: &uuid::Uuid,
        allow_audio_stream_copy: Option<bool>,
        allow_video_stream_copy: Option<bool>,
        audio_bit_rate: Option<i32>,
        audio_channels: Option<i32>,
        audio_codec: Option<&types::GetAudioStreamAudioCodec>,
        audio_sample_rate: Option<i32>,
        audio_stream_index: Option<i32>,
        break_on_non_key_frames: Option<bool>,
        container: Option<&types::GetAudioStreamContainer>,
        context: Option<types::EncodingContext>,
        copy_timestamps: Option<bool>,
        cpu_core_limit: Option<i32>,
        de_interlace: Option<bool>,
        device_id: Option<&str>,
        device_profile_id: Option<&str>,
        enable_audio_vbr_encoding: Option<bool>,
        enable_auto_stream_copy: Option<bool>,
        enable_mpegts_m2_ts_mode: Option<bool>,
        framerate: Option<f32>,
        height: Option<i32>,
        level: Option<&types::GetAudioStreamLevel>,
        live_stream_id: Option<&str>,
        max_audio_bit_depth: Option<i32>,
        max_audio_channels: Option<i32>,
        max_framerate: Option<f32>,
        max_ref_frames: Option<i32>,
        max_video_bit_depth: Option<i32>,
        media_source_id: Option<&str>,
        min_segments: Option<i32>,
        params: Option<&str>,
        play_session_id: Option<&str>,
        profile: Option<&str>,
        require_avc: Option<bool>,
        require_non_anamorphic: Option<bool>,
        segment_container: Option<&types::GetAudioStreamSegmentContainer>,
        segment_length: Option<i32>,
        start_time_ticks: Option<i64>,
        static_: Option<bool>,
        stream_options: Option<
            &std::collections::HashMap<
                String,
                Option<String>,
            >,
        >,
        subtitle_codec: Option<&types::GetAudioStreamSubtitleCodec>,
        subtitle_method: Option<types::SubtitleDeliveryMethod>,
        subtitle_stream_index: Option<i32>,
        tag: Option<&str>,
        transcode_reasons: Option<&str>,
        transcoding_max_audio_channels: Option<i32>,
        video_bit_rate: Option<i32>,
        video_codec: Option<&types::GetAudioStreamVideoCodec>,
        video_stream_index: Option<i32>,
        width: Option<i32>,
    ) -> Result<reqwest::Response, Error> {
        self.request(reqwest::Method::GET, format!("/Audio/{}/stream", encode_path(&item_id.to_string())))
            .query_opt("allowAudioStreamCopy", allow_audio_stream_copy)
            .query_opt("allowVideoStreamCopy", allow_video_stream_copy)
            .query_opt("audioBitRate", audio_bit_rate)
            .query_opt("audioChannels", audio_channels)
            .query_opt("audioCodec", audio_codec)
            .query_opt("audioSampleRate", audio_sample_rate)
            .query_opt("audioStreamIndex", audio_stream_index)
            .query_opt("breakOnNonKeyFrames", break_on_non_key_frames)
            .query_opt("container", container)
            .query_opt("context", context)
            .query_opt("copyTimestamps", copy_timestamps)
            .query_opt("cpuCoreLimit", cpu_core_limit)
            .query_opt("deInterlace", de_interlace)
            .query_opt("deviceId", device_id)
            .query_opt("deviceProfileId", device_profile_id)
            .query_opt("enableAudioVbrEncoding", enable_audio_vbr_encoding)
            .query_opt("enableAutoStreamCopy", enable_auto_stream_copy)
            .query_opt("enableMpegtsM2TsMode", enable_mpegts_m2_ts_mode)
            .query_opt("framerate", framerate)
            .query_opt("height", height)
            .query_opt("level", level)
            .query_opt("liveStreamId", live_stream_id)
            .query_opt("maxAudioBitDepth", max_audio_bit_depth)
            .query_opt("maxAudioChannels", max_audio_channels)
            .query_opt("maxFramerate", max_framerate)
            .query_opt("maxRefFrames", max_ref_frames)
            .query_opt("maxVideoBitDepth", max_video_bit_depth)
            .query_opt("mediaSourceId", media_source_id)
            .query_opt("minSegments", min_segments)
            .query_opt("params", params)
            .query_opt("playSessionId", play_session_id)
            .query_opt("profile", profile)
            .query_opt("requireAvc", require_avc)
            .query_opt("requireNonAnamorphic", require_non_anamorphic)
            .query_opt("segmentContainer", segment_container)
            .query_opt("segmentLength", segment_length)
            .query_opt("startTimeTicks", start_time_ticks)
            .query_opt("static", static_)
            .query_opt("streamOptions", stream_options.map(|v| serde_json::to_string(v).unwrap_or_default()))
            .query_opt("subtitleCodec", subtitle_codec)
            .query_opt("subtitleMethod", subtitle_method)
            .query_opt("subtitleStreamIndex", subtitle_stream_index)
            .query_opt("tag", tag)
            .query_opt("transcodeReasons", transcode_reasons)
            .query_opt("transcodingMaxAudioChannels", transcoding_max_audio_channels)
            .query_opt("videoBitRate", video_bit_rate)
            .query_opt("videoCodec", video_codec)
            .query_opt("videoStreamIndex", video_stream_index)
            .query_opt("width", width)
            .send_response()
            .await
    }

    #[doc = "Gets an audio stream\n\nSends a `HEAD` request to `/Audio/{itemId}/stream`\n\nArguments:\n- `item_id`: The item id.\n- `allow_audio_stream_copy`: Whether or not to allow copying of the audio stream url.\n- `allow_video_stream_copy`: Whether or not to allow copying of the video stream url.\n- `audio_bit_rate`: Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults.\n- `audio_channels`: Optional. Specify a specific number of audio channels to encode to, e.g. 2.\n- `audio_codec`: Optional. Specify an audio codec to encode to, e.g. mp3. If omitted the server will auto-select using the url's extension.\n- `audio_sample_rate`: Optional. Specify a specific audio sample rate, e.g. 44100.\n- `audio_stream_index`: Optional. The index of the audio stream to use. If omitted the first audio stream will be used.\n- `break_on_non_key_frames`: Optional. Whether to break on non key frames.\n- `container`: The audio container.\n- `context`: Optional. The MediaBrowser.Model.Dlna.EncodingContext.\n- `copy_timestamps`: Whether or not to copy timestamps when transcoding with an offset. Defaults to false.\n- `cpu_core_limit`: Optional. The limit of how many cpu cores to use.\n- `de_interlace`: Optional. Whether to deinterlace the video.\n- `device_id`: The device id of the client requesting. Used to stop encoding processes when needed.\n- `device_profile_id`: Optional. The dlna device profile id to utilize.\n- `enable_audio_vbr_encoding`: Optional. Whether to enable Audio Encoding.\n- `enable_auto_stream_copy`: Whether or not to allow automatic stream copy if requested values match the original source. Defaults to true.\n- `enable_mpegts_m2_ts_mode`: Optional. Whether to enable the MpegtsM2Ts mode.\n- `framerate`: Optional. A specific video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.\n- `height`: Optional. The fixed vertical resolution of the encoded video.\n- `level`: Optional. Specify a level for the encoder profile (varies by encoder), e.g. 3, 3.1.\n- `live_stream_id`: The live stream id.\n- `max_audio_bit_depth`: Optional. The maximum audio bit depth.\n- `max_audio_channels`: Optional. Specify a maximum number of audio channels to encode to, e.g. 2.\n- `max_framerate`: Optional. A specific maximum video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.\n- `max_ref_frames`: Optional.\n- `max_video_bit_depth`: Optional. The maximum video bit depth.\n- `media_source_id`: The media version id, if playing an alternate version.\n- `min_segments`: The minimum number of segments.\n- `params`: The streaming parameters.\n- `play_session_id`: The play session id.\n- `profile`: Optional. Specify a specific an encoder profile (varies by encoder), e.g. main, baseline, high.\n- `require_avc`: Optional. Whether to require avc.\n- `require_non_anamorphic`: Optional. Whether to require a non anamorphic stream.\n- `segment_container`: The segment container.\n- `segment_length`: The segment length.\n- `start_time_ticks`: Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms.\n- `static_`: Optional. If true, the original file will be streamed statically without any encoding. Use either no url extension or the original file extension. true/false.\n- `stream_options`: Optional. The streaming options.\n- `subtitle_codec`: Optional. Specify a subtitle codec to encode to.\n- `subtitle_method`: Optional. Specify the subtitle delivery method.\n- `subtitle_stream_index`: Optional. The index of the subtitle stream to use. If omitted no subtitles will be used.\n- `tag`: The tag.\n- `transcode_reasons`: Optional. The transcoding reason.\n- `transcoding_max_audio_channels`: Optional. The maximum number of audio channels to transcode.\n- `video_bit_rate`: Optional. Specify a video bitrate to encode to, e.g. 500000. If omitted this will be left to encoder defaults.\n- `video_codec`: Optional. Specify a video codec to encode to, e.g. h264. If omitted the server will auto-select using the url's extension.\n- `video_stream_index`: Optional. The index of the video stream to use. If omitted the first video stream will be used.\n- `width`: Optional. The fixed horizontal resolution of the encoded video.\n"]
    pub async fn head_audio_stream(
        &self,
        item_id: &uuid::Uuid,
        allow_audio_stream_copy: Option<bool>,
        allow_video_stream_copy: Option<bool>,
        audio_bit_rate: Option<i32>,
        audio_channels: Option<i32>,
        audio_codec: Option<&types::HeadAudioStreamAudioCodec>,
        audio_sample_rate: Option<i32>,
        audio_stream_index: Option<i32>,
        break_on_non_key_frames: Option<bool>,
        container: Option<&types::HeadAudioStreamContainer>,
        context: Option<types::EncodingContext>,
        copy_timestamps: Option<bool>,
        cpu_core_limit: Option<i32>,
        de_interlace: Option<bool>,
        device_id: Option<&str>,
        device_profile_id: Option<&str>,
        enable_audio_vbr_encoding: Option<bool>,
        enable_auto_stream_copy: Option<bool>,
        enable_mpegts_m2_ts_mode: Option<bool>,
        framerate: Option<f32>,
        height: Option<i32>,
        level: Option<&types::HeadAudioStreamLevel>,
        live_stream_id: Option<&str>,
        max_audio_bit_depth: Option<i32>,
        max_audio_channels: Option<i32>,
        max_framerate: Option<f32>,
        max_ref_frames: Option<i32>,
        max_video_bit_depth: Option<i32>,
        media_source_id: Option<&str>,
        min_segments: Option<i32>,
        params: Option<&str>,
        play_session_id: Option<&str>,
        profile: Option<&str>,
        require_avc: Option<bool>,
        require_non_anamorphic: Option<bool>,
        segment_container: Option<&types::HeadAudioStreamSegmentContainer>,
        segment_length: Option<i32>,
        start_time_ticks: Option<i64>,
        static_: Option<bool>,
        stream_options: Option<
            &std::collections::HashMap<
                String,
                Option<String>,
            >,
        >,
        subtitle_codec: Option<&types::HeadAudioStreamSubtitleCodec>,
        subtitle_method: Option<types::SubtitleDeliveryMethod>,
        subtitle_stream_index: Option<i32>,
        tag: Option<&str>,
        transcode_reasons: Option<&str>,
        transcoding_max_audio_channels: Option<i32>,
        video_bit_rate: Option<i32>,
        video_codec: Option<&types::HeadAudioStreamVideoCodec>,
        video_stream_index: Option<i32>,
        width: Option<i32>,
    ) -> Result<reqwest::Response, Error> {
        self.request(reqwest::Method::HEAD, format!("/Audio/{}/stream", encode_path(&item_id.to_string())))
            .query_opt("allowAudioStreamCopy", allow_audio_stream_copy)
            .query_opt("allowVideoStreamCopy", allow_video_stream_copy)
            .query_opt("audioBitRate", audio_bit_rate)
            .query_opt("audioChannels", audio_channels)
            .query_opt("audioCodec", audio_codec)
            .query_opt("audioSampleRate", audio_sample_rate)
            .query_opt("audioStreamIndex", audio_stream_index)
            .query_opt("breakOnNonKeyFrames", break_on_non_key_frames)
            .query_opt("container", container)
            .query_opt("context", context)
            .query_opt("copyTimestamps", copy_timestamps)
            .query_opt("cpuCoreLimit", cpu_core_limit)
            .query_opt("deInterlace", de_interlace)
            .query_opt("deviceId", device_id)
            .query_opt("deviceProfileId", device_profile_id)
            .query_opt("enableAudioVbrEncoding", enable_audio_vbr_encoding)
            .query_opt("enableAutoStreamCopy", enable_auto_stream_copy)
            .query_opt("enableMpegtsM2TsMode", enable_mpegts_m2_ts_mode)
            .query_opt("framerate", framerate)
            .query_opt("height", height)
            .query_opt("level", level)
            .query_opt("liveStreamId", live_stream_id)
            .query_opt("maxAudioBitDepth", max_audio_bit_depth)
            .query_opt("maxAudioChannels", max_audio_channels)
            .query_opt("maxFramerate", max_framerate)
            .query_opt("maxRefFrames", max_ref_frames)
            .query_opt("maxVideoBitDepth", max_video_bit_depth)
            .query_opt("mediaSourceId", media_source_id)
            .query_opt("minSegments", min_segments)
            .query_opt("params", params)
            .query_opt("playSessionId", play_session_id)
            .query_opt("profile", profile)
            .query_opt("requireAvc", require_avc)
            .query_opt("requireNonAnamorphic", require_non_anamorphic)
            .query_opt("segmentContainer", segment_container)
            .query_opt("segmentLength", segment_length)
            .query_opt("startTimeTicks", start_time_ticks)
            .query_opt("static", static_)
            .query_opt("streamOptions", stream_options.map(|v| serde_json::to_string(v).unwrap_or_default()))
            .query_opt("subtitleCodec", subtitle_codec)
            .query_opt("subtitleMethod", subtitle_method)
            .query_opt("subtitleStreamIndex", subtitle_stream_index)
            .query_opt("tag", tag)
            .query_opt("transcodeReasons", transcode_reasons)
            .query_opt("transcodingMaxAudioChannels", transcoding_max_audio_channels)
            .query_opt("videoBitRate", video_bit_rate)
            .query_opt("videoCodec", video_codec)
            .query_opt("videoStreamIndex", video_stream_index)
            .query_opt("width", width)
            .send_response()
            .await
    }

    #[doc = "Gets an audio stream\n\nSends a `GET` request to `/Audio/{itemId}/stream.{container}`\n\nArguments:\n- `item_id`: The item id.\n- `container`: The audio container.\n- `allow_audio_stream_copy`: Whether or not to allow copying of the audio stream url.\n- `allow_video_stream_copy`: Whether or not to allow copying of the video stream url.\n- `audio_bit_rate`: Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults.\n- `audio_channels`: Optional. Specify a specific number of audio channels to encode to, e.g. 2.\n- `audio_codec`: Optional. Specify an audio codec to encode to, e.g. mp3. If omitted the server will auto-select using the url's extension.\n- `audio_sample_rate`: Optional. Specify a specific audio sample rate, e.g. 44100.\n- `audio_stream_index`: Optional. The index of the audio stream to use. If omitted the first audio stream will be used.\n- `break_on_non_key_frames`: Optional. Whether to break on non key frames.\n- `context`: Optional. The MediaBrowser.Model.Dlna.EncodingContext.\n- `copy_timestamps`: Whether or not to copy timestamps when transcoding with an offset. Defaults to false.\n- `cpu_core_limit`: Optional. The limit of how many cpu cores to use.\n- `de_interlace`: Optional. Whether to deinterlace the video.\n- `device_id`: The device id of the client requesting. Used to stop encoding processes when needed.\n- `device_profile_id`: Optional. The dlna device profile id to utilize.\n- `enable_audio_vbr_encoding`: Optional. Whether to enable Audio Encoding.\n- `enable_auto_stream_copy`: Whether or not to allow automatic stream copy if requested values match the original source. Defaults to true.\n- `enable_mpegts_m2_ts_mode`: Optional. Whether to enable the MpegtsM2Ts mode.\n- `framerate`: Optional. A specific video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.\n- `height`: Optional. The fixed vertical resolution of the encoded video.\n- `level`: Optional. Specify a level for the encoder profile (varies by encoder), e.g. 3, 3.1.\n- `live_stream_id`: The live stream id.\n- `max_audio_bit_depth`: Optional. The maximum audio bit depth.\n- `max_audio_channels`: Optional. Specify a maximum number of audio channels to encode to, e.g. 2.\n- `max_framerate`: Optional. A specific maximum video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.\n- `max_ref_frames`: Optional.\n- `max_video_bit_depth`: Optional. The maximum video bit depth.\n- `media_source_id`: The media version id, if playing an alternate version.\n- `min_segments`: The minimum number of segments.\n- `params`: The streaming parameters.\n- `play_session_id`: The play session id.\n- `profile`: Optional. Specify a specific an encoder profile (varies by encoder), e.g. main, baseline, high.\n- `require_avc`: Optional. Whether to require avc.\n- `require_non_anamorphic`: Optional. Whether to require a non anamorphic stream.\n- `segment_container`: The segment container.\n- `segment_length`: The segment length.\n- `start_time_ticks`: Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms.\n- `static_`: Optional. If true, the original file will be streamed statically without any encoding. Use either no url extension or the original file extension. true/false.\n- `stream_options`: Optional. The streaming options.\n- `subtitle_codec`: Optional. Specify a subtitle codec to encode to.\n- `subtitle_method`: Optional. Specify the subtitle delivery method.\n- `subtitle_stream_index`: Optional. The index of the subtitle stream to use. If omitted no subtitles will be used.\n- `tag`: The tag.\n- `transcode_reasons`: Optional. The transcoding reason.\n- `transcoding_max_audio_channels`: Optional. The maximum number of audio channels to transcode.\n- `video_bit_rate`: Optional. Specify a video bitrate to encode to, e.g. 500000. If omitted this will be left to encoder defaults.\n- `video_codec`: Optional. Specify a video codec to encode to, e.g. h264. If omitted the server will auto-select using the url's extension.\n- `video_stream_index`: Optional. The index of the video stream to use. If omitted the first video stream will be used.\n- `width`: Optional. The fixed horizontal resolution of the encoded video.\n"]
    pub async fn get_audio_stream_by_container(
        &self,
        item_id: &uuid::Uuid,
        container: &types::GetAudioStreamByContainerContainer,
        allow_audio_stream_copy: Option<bool>,
        allow_video_stream_copy: Option<bool>,
        audio_bit_rate: Option<i32>,
        audio_channels: Option<i32>,
        audio_codec: Option<&types::GetAudioStreamByContainerAudioCodec>,
        audio_sample_rate: Option<i32>,
        audio_stream_index: Option<i32>,
        break_on_non_key_frames: Option<bool>,
        context: Option<types::EncodingContext>,
        copy_timestamps: Option<bool>,
        cpu_core_limit: Option<i32>,
        de_interlace: Option<bool>,
        device_id: Option<&str>,
        device_profile_id: Option<&str>,
        enable_audio_vbr_encoding: Option<bool>,
        enable_auto_stream_copy: Option<bool>,
        enable_mpegts_m2_ts_mode: Option<bool>,
        framerate: Option<f32>,
        height: Option<i32>,
        level: Option<&types::GetAudioStreamByContainerLevel>,
        live_stream_id: Option<&str>,
        max_audio_bit_depth: Option<i32>,
        max_audio_channels: Option<i32>,
        max_framerate: Option<f32>,
        max_ref_frames: Option<i32>,
        max_video_bit_depth: Option<i32>,
        media_source_id: Option<&str>,
        min_segments: Option<i32>,
        params: Option<&str>,
        play_session_id: Option<&str>,
        profile: Option<&str>,
        require_avc: Option<bool>,
        require_non_anamorphic: Option<bool>,
        segment_container: Option<&types::GetAudioStreamByContainerSegmentContainer>,
        segment_length: Option<i32>,
        start_time_ticks: Option<i64>,
        static_: Option<bool>,
        stream_options: Option<
            &std::collections::HashMap<
                String,
                Option<String>,
            >,
        >,
        subtitle_codec: Option<&types::GetAudioStreamByContainerSubtitleCodec>,
        subtitle_method: Option<types::SubtitleDeliveryMethod>,
        subtitle_stream_index: Option<i32>,
        tag: Option<&str>,
        transcode_reasons: Option<&str>,
        transcoding_max_audio_channels: Option<i32>,
        video_bit_rate: Option<i32>,
        video_codec: Option<&types::GetAudioStreamByContainerVideoCodec>,
        video_stream_index: Option<i32>,
        width: Option<i32>,
    ) -> Result<reqwest::Response, Error> {
        self.request(reqwest::Method::GET, format!("/Audio/{}/stream.{}", encode_path(&item_id.to_string()), encode_path(&container.to_string())))
            .query_opt("allowAudioStreamCopy", allow_audio_stream_copy)
            .query_opt("allowVideoStreamCopy", allow_video_stream_copy)
            .query_opt("audioBitRate", audio_bit_rate)
            .query_opt("audioChannels", audio_channels)
            .query_opt("audioCodec", audio_codec)
            .query_opt("audioSampleRate", audio_sample_rate)
            .query_opt("audioStreamIndex", audio_stream_index)
            .query_opt("breakOnNonKeyFrames", break_on_non_key_frames)
            .query_opt("context", context)
            .query_opt("copyTimestamps", copy_timestamps)
            .query_opt("cpuCoreLimit", cpu_core_limit)
            .query_opt("deInterlace", de_interlace)
            .query_opt("deviceId", device_id)
            .query_opt("deviceProfileId", device_profile_id)
            .query_opt("enableAudioVbrEncoding", enable_audio_vbr_encoding)
            .query_opt("enableAutoStreamCopy", enable_auto_stream_copy)
            .query_opt("enableMpegtsM2TsMode", enable_mpegts_m2_ts_mode)
            .query_opt("framerate", framerate)
            .query_opt("height", height)
            .query_opt("level", level)
            .query_opt("liveStreamId", live_stream_id)
            .query_opt("maxAudioBitDepth", max_audio_bit_depth)
            .query_opt("maxAudioChannels", max_audio_channels)
            .query_opt("maxFramerate", max_framerate)
            .query_opt("maxRefFrames", max_ref_frames)
            .query_opt("maxVideoBitDepth", max_video_bit_depth)
            .query_opt("mediaSourceId", media_source_id)
            .query_opt("minSegments", min_segments)
            .query_opt("params", params)
            .query_opt("playSessionId", play_session_id)
            .query_opt("profile", profile)
            .query_opt("requireAvc", require_avc)
            .query_opt("requireNonAnamorphic", require_non_anamorphic)
            .query_opt("segmentContainer", segment_container)
            .query_opt("segmentLength", segment_length)
            .query_opt("startTimeTicks", start_time_ticks)
            .query_opt("static", static_)
            .query_opt("streamOptions", stream_options.map(|v| serde_json::to_string(v).unwrap_or_default()))
            .query_opt("subtitleCodec", subtitle_codec)
            .query_opt("subtitleMethod", subtitle_method)
            .query_opt("subtitleStreamIndex", subtitle_stream_index)
            .query_opt("tag", tag)
            .query_opt("transcodeReasons", transcode_reasons)
            .query_opt("transcodingMaxAudioChannels", transcoding_max_audio_channels)
            .query_opt("videoBitRate", video_bit_rate)
            .query_opt("videoCodec", video_codec)
            .query_opt("videoStreamIndex", video_stream_index)
            .query_opt("width", width)
            .send_response()
            .await
    }

    #[doc = "Gets an audio stream\n\nSends a `HEAD` request to `/Audio/{itemId}/stream.{container}`\n\nArguments:\n- `item_id`: The item id.\n- `container`: The audio container.\n- `allow_audio_stream_copy`: Whether or not to allow copying of the audio stream url.\n- `allow_video_stream_copy`: Whether or not to allow copying of the video stream url.\n- `audio_bit_rate`: Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults.\n- `audio_channels`: Optional. Specify a specific number of audio channels to encode to, e.g. 2.\n- `audio_codec`: Optional. Specify an audio codec to encode to, e.g. mp3. If omitted the server will auto-select using the url's extension.\n- `audio_sample_rate`: Optional. Specify a specific audio sample rate, e.g. 44100.\n- `audio_stream_index`: Optional. The index of the audio stream to use. If omitted the first audio stream will be used.\n- `break_on_non_key_frames`: Optional. Whether to break on non key frames.\n- `context`: Optional. The MediaBrowser.Model.Dlna.EncodingContext.\n- `copy_timestamps`: Whether or not to copy timestamps when transcoding with an offset. Defaults to false.\n- `cpu_core_limit`: Optional. The limit of how many cpu cores to use.\n- `de_interlace`: Optional. Whether to deinterlace the video.\n- `device_id`: The device id of the client requesting. Used to stop encoding processes when needed.\n- `device_profile_id`: Optional. The dlna device profile id to utilize.\n- `enable_audio_vbr_encoding`: Optional. Whether to enable Audio Encoding.\n- `enable_auto_stream_copy`: Whether or not to allow automatic stream copy if requested values match the original source. Defaults to true.\n- `enable_mpegts_m2_ts_mode`: Optional. Whether to enable the MpegtsM2Ts mode.\n- `framerate`: Optional. A specific video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.\n- `height`: Optional. The fixed vertical resolution of the encoded video.\n- `level`: Optional. Specify a level for the encoder profile (varies by encoder), e.g. 3, 3.1.\n- `live_stream_id`: The live stream id.\n- `max_audio_bit_depth`: Optional. The maximum audio bit depth.\n- `max_audio_channels`: Optional. Specify a maximum number of audio channels to encode to, e.g. 2.\n- `max_framerate`: Optional. A specific maximum video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.\n- `max_ref_frames`: Optional.\n- `max_video_bit_depth`: Optional. The maximum video bit depth.\n- `media_source_id`: The media version id, if playing an alternate version.\n- `min_segments`: The minimum number of segments.\n- `params`: The streaming parameters.\n- `play_session_id`: The play session id.\n- `profile`: Optional. Specify a specific an encoder profile (varies by encoder), e.g. main, baseline, high.\n- `require_avc`: Optional. Whether to require avc.\n- `require_non_anamorphic`: Optional. Whether to require a non anamorphic stream.\n- `segment_container`: The segment container.\n- `segment_length`: The segment length.\n- `start_time_ticks`: Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms.\n- `static_`: Optional. If true, the original file will be streamed statically without any encoding. Use either no url extension or the original file extension. true/false.\n- `stream_options`: Optional. The streaming options.\n- `subtitle_codec`: Optional. Specify a subtitle codec to encode to.\n- `subtitle_method`: Optional. Specify the subtitle delivery method.\n- `subtitle_stream_index`: Optional. The index of the subtitle stream to use. If omitted no subtitles will be used.\n- `tag`: The tag.\n- `transcode_reasons`: Optional. The transcoding reason.\n- `transcoding_max_audio_channels`: Optional. The maximum number of audio channels to transcode.\n- `video_bit_rate`: Optional. Specify a video bitrate to encode to, e.g. 500000. If omitted this will be left to encoder defaults.\n- `video_codec`: Optional. Specify a video codec to encode to, e.g. h264. If omitted the server will auto-select using the url's extension.\n- `video_stream_index`: Optional. The index of the video stream to use. If omitted the first video stream will be used.\n- `width`: Optional. The fixed horizontal resolution of the encoded video.\n"]
    pub async fn head_audio_stream_by_container(
        &self,
        item_id: &uuid::Uuid,
        container: &types::HeadAudioStreamByContainerContainer,
        allow_audio_stream_copy: Option<bool>,
        allow_video_stream_copy: Option<bool>,
        audio_bit_rate: Option<i32>,
        audio_channels: Option<i32>,
        audio_codec: Option<&types::HeadAudioStreamByContainerAudioCodec>,
        audio_sample_rate: Option<i32>,
        audio_stream_index: Option<i32>,
        break_on_non_key_frames: Option<bool>,
        context: Option<types::EncodingContext>,
        copy_timestamps: Option<bool>,
        cpu_core_limit: Option<i32>,
        de_interlace: Option<bool>,
        device_id: Option<&str>,
        device_profile_id: Option<&str>,
        enable_audio_vbr_encoding: Option<bool>,
        enable_auto_stream_copy: Option<bool>,
        enable_mpegts_m2_ts_mode: Option<bool>,
        framerate: Option<f32>,
        height: Option<i32>,
        level: Option<&types::HeadAudioStreamByContainerLevel>,
        live_stream_id: Option<&str>,
        max_audio_bit_depth: Option<i32>,
        max_audio_channels: Option<i32>,
        max_framerate: Option<f32>,
        max_ref_frames: Option<i32>,
        max_video_bit_depth: Option<i32>,
        media_source_id: Option<&str>,
        min_segments: Option<i32>,
        params: Option<&str>,
        play_session_id: Option<&str>,
        profile: Option<&str>,
        require_avc: Option<bool>,
        require_non_anamorphic: Option<bool>,
        segment_container: Option<&types::HeadAudioStreamByContainerSegmentContainer>,
        segment_length: Option<i32>,
        start_time_ticks: Option<i64>,
        static_: Option<bool>,
        stream_options: Option<
            &std::collections::HashMap<
                String,
                Option<String>,
            >,
        >,
        subtitle_codec: Option<&types::HeadAudioStreamByContainerSubtitleCodec>,
        subtitle_method: Option<types::SubtitleDeliveryMethod>,
        subtitle_stream_index: Option<i32>,
        tag: Option<&str>,
        transcode_reasons: Option<&str>,
        transcoding_max_audio_channels: Option<i32>,
        video_bit_rate: Option<i32>,
        video_codec: Option<&types::HeadAudioStreamByContainerVideoCodec>,
        video_stream_index: Option<i32>,
        width: Option<i32>,
    ) -> Result<reqwest::Response, Error> {
        self.request(reqwest::Method::HEAD, format!("/Audio/{}/stream.{}", encode_path(&item_id.to_string()), encode_path(&container.to_string())))
            .query_opt("allowAudioStreamCopy", allow_audio_stream_copy)
            .query_opt("allowVideoStreamCopy", allow_video_stream_copy)
            .query_opt("audioBitRate", audio_bit_rate)
            .query_opt("audioChannels", audio_channels)
            .query_opt("audioCodec", audio_codec)
            .query_opt("audioSampleRate", audio_sample_rate)
            .query_opt("audioStreamIndex", audio_stream_index)
            .query_opt("breakOnNonKeyFrames", break_on_non_key_frames)
            .query_opt("context", context)
            .query_opt("copyTimestamps", copy_timestamps)
            .query_opt("cpuCoreLimit", cpu_core_limit)
            .query_opt("deInterlace", de_interlace)
            .query_opt("deviceId", device_id)
            .query_opt("deviceProfileId", device_profile_id)
            .query_opt("enableAudioVbrEncoding", enable_audio_vbr_encoding)
            .query_opt("enableAutoStreamCopy", enable_auto_stream_copy)
            .query_opt("enableMpegtsM2TsMode", enable_mpegts_m2_ts_mode)
            .query_opt("framerate", framerate)
            .query_opt("height", height)
            .query_opt("level", level)
            .query_opt("liveStreamId", live_stream_id)
            .query_opt("maxAudioBitDepth", max_audio_bit_depth)
            .query_opt("maxAudioChannels", max_audio_channels)
            .query_opt("maxFramerate", max_framerate)
            .query_opt("maxRefFrames", max_ref_frames)
            .query_opt("maxVideoBitDepth", max_video_bit_depth)
            .query_opt("mediaSourceId", media_source_id)
            .query_opt("minSegments", min_segments)
            .query_opt("params", params)
            .query_opt("playSessionId", play_session_id)
            .query_opt("profile", profile)
            .query_opt("requireAvc", require_avc)
            .query_opt("requireNonAnamorphic", require_non_anamorphic)
            .query_opt("segmentContainer", segment_container)
            .query_opt("segmentLength", segment_length)
            .query_opt("startTimeTicks", start_time_ticks)
            .query_opt("static", static_)
            .query_opt("streamOptions", stream_options.map(|v| serde_json::to_string(v).unwrap_or_default()))
            .query_opt("subtitleCodec", subtitle_codec)
            .query_opt("subtitleMethod", subtitle_method)
            .query_opt("subtitleStreamIndex", subtitle_stream_index)
            .query_opt("tag", tag)
            .query_opt("transcodeReasons", transcode_reasons)
            .query_opt("transcodingMaxAudioChannels", transcoding_max_audio_channels)
            .query_opt("videoBitRate", video_bit_rate)
            .query_opt("videoCodec", video_codec)
            .query_opt("videoStreamIndex", video_stream_index)
            .query_opt("width", width)
            .send_response()
            .await
    }

    #[doc = "Gets a video stream using HTTP live streaming\n\nSends a `GET` request to `/Audio/{itemId}/hls1/{playlistId}/{segmentId}.{container}`\n\nArguments:\n- `item_id`: The item id.\n- `playlist_id`: The playlist id.\n- `segment_id`: The segment id.\n- `container`: The video container. Possible values are: ts, webm, asf, wmv, ogv, mp4, m4v, mkv, mpeg, mpg, avi, 3gp, wmv, wtv, m2ts, mov, iso, flv.\n- `actual_segment_length_ticks`: The length of the requested segment in ticks.\n- `allow_audio_stream_copy`: Whether or not to allow copying of the audio stream url.\n- `allow_video_stream_copy`: Whether or not to allow copying of the video stream url.\n- `audio_bit_rate`: Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults.\n- `audio_channels`: Optional. Specify a specific number of audio channels to encode to, e.g. 2.\n- `audio_codec`: Optional. Specify an audio codec to encode to, e.g. mp3.\n- `audio_sample_rate`: Optional. Specify a specific audio sample rate, e.g. 44100.\n- `audio_stream_index`: Optional. The index of the audio stream to use. If omitted the first audio stream will be used.\n- `break_on_non_key_frames`: Optional. Whether to break on non key frames.\n- `context`: Optional. The MediaBrowser.Model.Dlna.EncodingContext.\n- `copy_timestamps`: Whether or not to copy timestamps when transcoding with an offset. Defaults to false.\n- `cpu_core_limit`: Optional. The limit of how many cpu cores to use.\n- `de_interlace`: Optional. Whether to deinterlace the video.\n- `device_id`: The device id of the client requesting. Used to stop encoding processes when needed.\n- `device_profile_id`: Optional. The dlna device profile id to utilize.\n- `enable_audio_vbr_encoding`: Optional. Whether to enable Audio Encoding.\n- `enable_auto_stream_copy`: Whether or not to allow automatic stream copy if requested values match the original source. Defaults to true.\n- `enable_mpegts_m2_ts_mode`: Optional. Whether to enable the MpegtsM2Ts mode.\n- `framerate`: Optional. A specific video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.\n- `height`: Optional. The fixed vertical resolution of the encoded video.\n- `level`: Optional. Specify a level for the encoder profile (varies by encoder), e.g. 3, 3.1.\n- `live_stream_id`: The live stream id.\n- `max_audio_bit_depth`: Optional. The maximum audio bit depth.\n- `max_audio_channels`: Optional. Specify a maximum number of audio channels to encode to, e.g. 2.\n- `max_framerate`: Optional. A specific maximum video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.\n- `max_ref_frames`: Optional.\n- `max_streaming_bitrate`: Optional. The maximum streaming bitrate.\n- `max_video_bit_depth`: Optional. The maximum video bit depth.\n- `media_source_id`: The media version id, if playing an alternate version.\n- `min_segments`: The minimum number of segments.\n- `params`: The streaming parameters.\n- `play_session_id`: The play session id.\n- `profile`: Optional. Specify a specific an encoder profile (varies by encoder), e.g. main, baseline, high.\n- `require_avc`: Optional. Whether to require avc.\n- `require_non_anamorphic`: Optional. Whether to require a non anamorphic stream.\n- `runtime_ticks`: The position of the requested segment in ticks.\n- `segment_container`: The segment container.\n- `segment_length`: The segment length.\n- `start_time_ticks`: Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms.\n- `static_`: Optional. If true, the original file will be streamed statically without any encoding. Use either no url extension or the original file extension. true/false.\n- `stream_options`: Optional. The streaming options.\n- `subtitle_codec`: Optional. Specify a subtitle codec to encode to.\n- `subtitle_method`: Optional. Specify the subtitle delivery method.\n- `subtitle_stream_index`: Optional. The index of the subtitle stream to use. If omitted no subtitles will be used.\n- `tag`: The tag.\n- `transcode_reasons`: Optional. The transcoding reason.\n- `transcoding_max_audio_channels`: Optional. The maximum number of audio channels to transcode.\n- `video_bit_rate`: Optional. Specify a video bitrate to encode to, e.g. 500000. If omitted this will be left to encoder defaults.\n- `video_codec`: Optional. Specify a video codec to encode to, e.g. h264.\n- `video_stream_index`: Optional. The index of the video stream to use. If omitted the first video stream will be used.\n- `width`: Optional. The fixed horizontal resolution of the encoded video.\n"]
    pub async fn get_hls_audio_segment(
        &self,
        item_id: &uuid::Uuid,
        playlist_id: &str,
        segment_id: i32,
        container: &types::GetHlsAudioSegmentContainer,
        actual_segment_length_ticks: i64,
        allow_audio_stream_copy: Option<bool>,
        allow_video_stream_copy: Option<bool>,
        audio_bit_rate: Option<i32>,
        audio_channels: Option<i32>,
        audio_codec: Option<&types::GetHlsAudioSegmentAudioCodec>,
        audio_sample_rate: Option<i32>,
        audio_stream_index: Option<i32>,
        break_on_non_key_frames: Option<bool>,
        context: Option<types::EncodingContext>,
        copy_timestamps: Option<bool>,
        cpu_core_limit: Option<i32>,
        de_interlace: Option<bool>,
        device_id: Option<&str>,
        device_profile_id: Option<&str>,
        enable_audio_vbr_encoding: Option<bool>,
        enable_auto_stream_copy: Option<bool>,
        enable_mpegts_m2_ts_mode: Option<bool>,
        framerate: Option<f32>,
        height: Option<i32>,
        level: Option<&types::GetHlsAudioSegmentLevel>,
        live_stream_id: Option<&str>,
        max_audio_bit_depth: Option<i32>,
        max_audio_channels: Option<i32>,
        max_framerate: Option<f32>,
        max_ref_frames: Option<i32>,
        max_streaming_bitrate: Option<i32>,
        max_video_bit_depth: Option<i32>,
        media_source_id: Option<&str>,
        min_segments: Option<i32>,
        params: Option<&str>,
        play_session_id: Option<&str>,
        profile: Option<&str>,
        require_avc: Option<bool>,
        require_non_anamorphic: Option<bool>,
        runtime_ticks: i64,
        segment_container: Option<&types::GetHlsAudioSegmentSegmentContainer>,
        segment_length: Option<i32>,
        start_time_ticks: Option<i64>,
        static_: Option<bool>,
        stream_options: Option<
            &std::collections::HashMap<
                String,
                Option<String>,
            >,
        >,
        subtitle_codec: Option<&types::GetHlsAudioSegmentSubtitleCodec>,
        subtitle_method: Option<types::SubtitleDeliveryMethod>,
        subtitle_stream_index: Option<i32>,
        tag: Option<&str>,
        transcode_reasons: Option<&str>,
        transcoding_max_audio_channels: Option<i32>,
        video_bit_rate: Option<i32>,
        video_codec: Option<&types::GetHlsAudioSegmentVideoCodec>,
        video_stream_index: Option<i32>,
        width: Option<i32>,
    ) -> Result<reqwest::Response, Error> {
        self.request(reqwest::Method::GET, format!("/Audio/{}/hls1/{}/{}.{}", encode_path(&item_id.to_string()), encode_path(playlist_id), encode_path(&segment_id.to_string()), encode_path(&container.to_string())))
            .query("actualSegmentLengthTicks", actual_segment_length_ticks)
            .query_opt("allowAudioStreamCopy", allow_audio_stream_copy)
            .query_opt("allowVideoStreamCopy", allow_video_stream_copy)
            .query_opt("audioBitRate", audio_bit_rate)
            .query_opt("audioChannels", audio_channels)
            .query_opt("audioCodec", audio_codec)
            .query_opt("audioSampleRate", audio_sample_rate)
            .query_opt("audioStreamIndex", audio_stream_index)
            .query_opt("breakOnNonKeyFrames", break_on_non_key_frames)
            .query_opt("context", context)
            .query_opt("copyTimestamps", copy_timestamps)
            .query_opt("cpuCoreLimit", cpu_core_limit)
            .query_opt("deInterlace", de_interlace)
            .query_opt("deviceId", device_id)
            .query_opt("deviceProfileId", device_profile_id)
            .query_opt("enableAudioVbrEncoding", enable_audio_vbr_encoding)
            .query_opt("enableAutoStreamCopy", enable_auto_stream_copy)
            .query_opt("enableMpegtsM2TsMode", enable_mpegts_m2_ts_mode)
            .query_opt("framerate", framerate)
            .query_opt("height", height)
            .query_opt("level", level)
            .query_opt("liveStreamId", live_stream_id)
            .query_opt("maxAudioBitDepth", max_audio_bit_depth)
            .query_opt("maxAudioChannels", max_audio_channels)
            .query_opt("maxFramerate", max_framerate)
            .query_opt("maxRefFrames", max_ref_frames)
            .query_opt("maxStreamingBitrate", max_streaming_bitrate)
            .query_opt("maxVideoBitDepth", max_video_bit_depth)
            .query_opt("mediaSourceId", media_source_id)
            .query_opt("minSegments", min_segments)
            .query_opt("params", params)
            .query_opt("playSessionId", play_session_id)
            .query_opt("profile", profile)
            .query_opt("requireAvc", require_avc)
            .query_opt("requireNonAnamorphic", require_non_anamorphic)
            .query("runtimeTicks", runtime_ticks)
            .query_opt("segmentContainer", segment_container)
            .query_opt("segmentLength", segment_length)
            .query_opt("startTimeTicks", start_time_ticks)
            .query_opt("static", static_)
            .query_opt("streamOptions", stream_options.map(|v| serde_json::to_string(v).unwrap_or_default()))
            .query_opt("subtitleCodec", subtitle_codec)
            .query_opt("subtitleMethod", subtitle_method)
            .query_opt("subtitleStreamIndex", subtitle_stream_index)
            .query_opt("tag", tag)
            .query_opt("transcodeReasons", transcode_reasons)
            .query_opt("transcodingMaxAudioChannels", transcoding_max_audio_channels)
            .query_opt("videoBitRate", video_bit_rate)
            .query_opt("videoCodec", video_codec)
            .query_opt("videoStreamIndex", video_stream_index)
            .query_opt("width", width)
            .send_response()
            .await
    }

    #[doc = "Gets an audio stream using HTTP live streaming\n\nSends a `GET` request to `/Audio/{itemId}/main.m3u8`\n\nArguments:\n- `item_id`: The item id.\n- `allow_audio_stream_copy`: Whether or not to allow copying of the audio stream url.\n- `allow_video_stream_copy`: Whether or not to allow copying of the video stream url.\n- `audio_bit_rate`: Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults.\n- `audio_channels`: Optional. Specify a specific number of audio channels to encode to, e.g. 2.\n- `audio_codec`: Optional. Specify an audio codec to encode to, e.g. mp3.\n- `audio_sample_rate`: Optional. Specify a specific audio sample rate, e.g. 44100.\n- `audio_stream_index`: Optional. The index of the audio stream to use. If omitted the first audio stream will be used.\n- `break_on_non_key_frames`: Optional. Whether to break on non key frames.\n- `context`: Optional. The MediaBrowser.Model.Dlna.EncodingContext.\n- `copy_timestamps`: Whether or not to copy timestamps when transcoding with an offset. Defaults to false.\n- `cpu_core_limit`: Optional. The limit of how many cpu cores to use.\n- `de_interlace`: Optional. Whether to deinterlace the video.\n- `device_id`: The device id of the client requesting. Used to stop encoding processes when needed.\n- `device_profile_id`: Optional. The dlna device profile id to utilize.\n- `enable_audio_vbr_encoding`: Optional. Whether to enable Audio Encoding.\n- `enable_auto_stream_copy`: Whether or not to allow automatic stream copy if requested values match the original source. Defaults to true.\n- `enable_mpegts_m2_ts_mode`: Optional. Whether to enable the MpegtsM2Ts mode.\n- `framerate`: Optional. A specific video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.\n- `height`: Optional. The fixed vertical resolution of the encoded video.\n- `level`: Optional. Specify a level for the encoder profile (varies by encoder), e.g. 3, 3.1.\n- `live_stream_id`: The live stream id.\n- `max_audio_bit_depth`: Optional. The maximum audio bit depth.\n- `max_audio_channels`: Optional. Specify a maximum number of audio channels to encode to, e.g. 2.\n- `max_framerate`: Optional. A specific maximum video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.\n- `max_ref_frames`: Optional.\n- `max_streaming_bitrate`: Optional. The maximum streaming bitrate.\n- `max_video_bit_depth`: Optional. The maximum video bit depth.\n- `media_source_id`: The media version id, if playing an alternate version.\n- `min_segments`: The minimum number of segments.\n- `params`: The streaming parameters.\n- `play_session_id`: The play session id.\n- `profile`: Optional. Specify a specific an encoder profile (varies by encoder), e.g. main, baseline, high.\n- `require_avc`: Optional. Whether to require avc.\n- `require_non_anamorphic`: Optional. Whether to require a non anamorphic stream.\n- `segment_container`: The segment container.\n- `segment_length`: The segment length.\n- `start_time_ticks`: Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms.\n- `static_`: Optional. If true, the original file will be streamed statically without any encoding. Use either no url extension or the original file extension. true/false.\n- `stream_options`: Optional. The streaming options.\n- `subtitle_codec`: Optional. Specify a subtitle codec to encode to.\n- `subtitle_method`: Optional. Specify the subtitle delivery method.\n- `subtitle_stream_index`: Optional. The index of the subtitle stream to use. If omitted no subtitles will be used.\n- `tag`: The tag.\n- `transcode_reasons`: Optional. The transcoding reason.\n- `transcoding_max_audio_channels`: Optional. The maximum number of audio channels to transcode.\n- `video_bit_rate`: Optional. Specify a video bitrate to encode to, e.g. 500000. If omitted this will be left to encoder defaults.\n- `video_codec`: Optional. Specify a video codec to encode to, e.g. h264.\n- `video_stream_index`: Optional. The index of the video stream to use. If omitted the first video stream will be used.\n- `width`: Optional. The fixed horizontal resolution of the encoded video.\n"]
    pub async fn get_variant_hls_audio_playlist(
        &self,
        item_id: &uuid::Uuid,
        allow_audio_stream_copy: Option<bool>,
        allow_video_stream_copy: Option<bool>,
        audio_bit_rate: Option<i32>,
        audio_channels: Option<i32>,
        audio_codec: Option<&types::GetVariantHlsAudioPlaylistAudioCodec>,
        audio_sample_rate: Option<i32>,
        audio_stream_index: Option<i32>,
        break_on_non_key_frames: Option<bool>,
        context: Option<types::EncodingContext>,
        copy_timestamps: Option<bool>,
        cpu_core_limit: Option<i32>,
        de_interlace: Option<bool>,
        device_id: Option<&str>,
        device_profile_id: Option<&str>,
        enable_audio_vbr_encoding: Option<bool>,
        enable_auto_stream_copy: Option<bool>,
        enable_mpegts_m2_ts_mode: Option<bool>,
        framerate: Option<f32>,
        height: Option<i32>,
        level: Option<&types::GetVariantHlsAudioPlaylistLevel>,
        live_stream_id: Option<&str>,
        max_audio_bit_depth: Option<i32>,
        max_audio_channels: Option<i32>,
        max_framerate: Option<f32>,
        max_ref_frames: Option<i32>,
        max_streaming_bitrate: Option<i32>,
        max_video_bit_depth: Option<i32>,
        media_source_id: Option<&str>,
        min_segments: Option<i32>,
        params: Option<&str>,
        play_session_id: Option<&str>,
        profile: Option<&str>,
        require_avc: Option<bool>,
        require_non_anamorphic: Option<bool>,
        segment_container: Option<&types::GetVariantHlsAudioPlaylistSegmentContainer>,
        segment_length: Option<i32>,
        start_time_ticks: Option<i64>,
        static_: Option<bool>,
        stream_options: Option<
            &std::collections::HashMap<
                String,
                Option<String>,
            >,
        >,
        subtitle_codec: Option<&types::GetVariantHlsAudioPlaylistSubtitleCodec>,
        subtitle_method: Option<types::SubtitleDeliveryMethod>,
        subtitle_stream_index: Option<i32>,
        tag: Option<&str>,
        transcode_reasons: Option<&str>,
        transcoding_max_audio_channels: Option<i32>,
        video_bit_rate: Option<i32>,
        video_codec: Option<&types::GetVariantHlsAudioPlaylistVideoCodec>,
        video_stream_index: Option<i32>,
        width: Option<i32>,
    ) -> Result<reqwest::Response, Error> {
        self.request(reqwest::Method::GET, format!("/Audio/{}/main.m3u8", encode_path(&item_id.to_string())))
            .query_opt("allowAudioStreamCopy", allow_audio_stream_copy)
            .query_opt("allowVideoStreamCopy", allow_video_stream_copy)
            .query_opt("audioBitRate", audio_bit_rate)
            .query_opt("audioChannels", audio_channels)
            .query_opt("audioCodec", audio_codec)
            .query_opt("audioSampleRate", audio_sample_rate)
            .query_opt("audioStreamIndex", audio_stream_index)
            .query_opt("breakOnNonKeyFrames", break_on_non_key_frames)
            .query_opt("context", context)
            .query_opt("copyTimestamps", copy_timestamps)
            .query_opt("cpuCoreLimit", cpu_core_limit)
            .query_opt("deInterlace", de_interlace)
            .query_opt("deviceId", device_id)
            .query_opt("deviceProfileId", device_profile_id)
            .query_opt("enableAudioVbrEncoding", enable_audio_vbr_encoding)
            .query_opt("enableAutoStreamCopy", enable_auto_stream_copy)
            .query_opt("enableMpegtsM2TsMode", enable_mpegts_m2_ts_mode)
            .query_opt("framerate", framerate)
            .query_opt("height", height)
            .query_opt("level", level)
            .query_opt("liveStreamId", live_stream_id)
            .query_opt("maxAudioBitDepth", max_audio_bit_depth)
            .query_opt("maxAudioChannels", max_audio_channels)
            .query_opt("maxFramerate", max_framerate)
            .query_opt("maxRefFrames", max_ref_frames)
            .query_opt("maxStreamingBitrate", max_streaming_bitrate)
            .query_opt("maxVideoBitDepth", max_video_bit_depth)
            .query_opt("mediaSourceId", media_source_id)
            .query_opt("minSegments", min_segments)
            .query_opt("params", params)
            .query_opt("playSessionId", play_session_id)
            .query_opt("profile", profile)
            .query_opt("requireAvc", require_avc)
            .query_opt("requireNonAnamorphic", require_non_anamorphic)
            .query_opt("segmentContainer", segment_container)
            .query_opt("segmentLength", segment_length)
            .query_opt("startTimeTicks", start_time_ticks)
            .query_opt("static", static_)
            .query_opt("streamOptions", stream_options.map(|v| serde_json::to_string(v).unwrap_or_default()))
            .query_opt("subtitleCodec", subtitle_codec)
            .query_opt("subtitleMethod", subtitle_method)
            .query_opt("subtitleStreamIndex", subtitle_stream_index)
            .query_opt("tag", tag)
            .query_opt("transcodeReasons", transcode_reasons)
            .query_opt("transcodingMaxAudioChannels", transcoding_max_audio_channels)
            .query_opt("videoBitRate", video_bit_rate)
            .query_opt("videoCodec", video_codec)
            .query_opt("videoStreamIndex", video_stream_index)
            .query_opt("width", width)
            .send_response()
            .await
    }

    #[doc = "Gets an audio hls playlist stream\n\nSends a `GET` request to `/Audio/{itemId}/master.m3u8`\n\nArguments:\n- `item_id`: The item id.\n- `allow_audio_stream_copy`: Whether or not to allow copying of the audio stream url.\n- `allow_video_stream_copy`: Whether or not to allow copying of the video stream url.\n- `audio_bit_rate`: Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults.\n- `audio_channels`: Optional. Specify a specific number of audio channels to encode to, e.g. 2.\n- `audio_codec`: Optional. Specify an audio codec to encode to, e.g. mp3.\n- `audio_sample_rate`: Optional. Specify a specific audio sample rate, e.g. 44100.\n- `audio_stream_index`: Optional. The index of the audio stream to use. If omitted the first audio stream will be used.\n- `break_on_non_key_frames`: Optional. Whether to break on non key frames.\n- `context`: Optional. The MediaBrowser.Model.Dlna.EncodingContext.\n- `copy_timestamps`: Whether or not to copy timestamps when transcoding with an offset. Defaults to false.\n- `cpu_core_limit`: Optional. The limit of how many cpu cores to use.\n- `de_interlace`: Optional. Whether to deinterlace the video.\n- `device_id`: The device id of the client requesting. Used to stop encoding processes when needed.\n- `device_profile_id`: Optional. The dlna device profile id to utilize.\n- `enable_adaptive_bitrate_streaming`: Enable adaptive bitrate streaming.\n- `enable_audio_vbr_encoding`: Optional. Whether to enable Audio Encoding.\n- `enable_auto_stream_copy`: Whether or not to allow automatic stream copy if requested values match the original source. Defaults to true.\n- `enable_mpegts_m2_ts_mode`: Optional. Whether to enable the MpegtsM2Ts mode.\n- `framerate`: Optional. A specific video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.\n- `height`: Optional. The fixed vertical resolution of the encoded video.\n- `level`: Optional. Specify a level for the encoder profile (varies by encoder), e.g. 3, 3.1.\n- `live_stream_id`: The live stream id.\n- `max_audio_bit_depth`: Optional. The maximum audio bit depth.\n- `max_audio_channels`: Optional. Specify a maximum number of audio channels to encode to, e.g. 2.\n- `max_framerate`: Optional. A specific maximum video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.\n- `max_ref_frames`: Optional.\n- `max_streaming_bitrate`: Optional. The maximum streaming bitrate.\n- `max_video_bit_depth`: Optional. The maximum video bit depth.\n- `media_source_id`: The media version id, if playing an alternate version.\n- `min_segments`: The minimum number of segments.\n- `params`: The streaming parameters.\n- `play_session_id`: The play session id.\n- `profile`: Optional. Specify a specific an encoder profile (varies by encoder), e.g. main, baseline, high.\n- `require_avc`: Optional. Whether to require avc.\n- `require_non_anamorphic`: Optional. Whether to require a non anamorphic stream.\n- `segment_container`: The segment container.\n- `segment_length`: The segment length.\n- `start_time_ticks`: Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms.\n- `static_`: Optional. If true, the original file will be streamed statically without any encoding. Use either no url extension or the original file extension. true/false.\n- `stream_options`: Optional. The streaming options.\n- `subtitle_codec`: Optional. Specify a subtitle codec to encode to.\n- `subtitle_method`: Optional. Specify the subtitle delivery method.\n- `subtitle_stream_index`: Optional. The index of the subtitle stream to use. If omitted no subtitles will be used.\n- `tag`: The tag.\n- `transcode_reasons`: Optional. The transcoding reason.\n- `transcoding_max_audio_channels`: Optional. The maximum number of audio channels to transcode.\n- `video_bit_rate`: Optional. Specify a video bitrate to encode to, e.g. 500000. If omitted this will be left to encoder defaults.\n- `video_codec`: Optional. Specify a video codec to encode to, e.g. h264.\n- `video_stream_index`: Optional. The index of the video stream to use. If omitted the first video stream will be used.\n- `width`: Optional. The fixed horizontal resolution of the encoded video.\n"]
    pub async fn get_master_hls_audio_playlist(
        &self,
        item_id: &uuid::Uuid,
        allow_audio_stream_copy: Option<bool>,
        allow_video_stream_copy: Option<bool>,
        audio_bit_rate: Option<i32>,
        audio_channels: Option<i32>,
        audio_codec: Option<&types::GetMasterHlsAudioPlaylistAudioCodec>,
        audio_sample_rate: Option<i32>,
        audio_stream_index: Option<i32>,
        break_on_non_key_frames: Option<bool>,
        context: Option<types::EncodingContext>,
        copy_timestamps: Option<bool>,
        cpu_core_limit: Option<i32>,
        de_interlace: Option<bool>,
        device_id: Option<&str>,
        device_profile_id: Option<&str>,
        enable_adaptive_bitrate_streaming: Option<bool>,
        enable_audio_vbr_encoding: Option<bool>,
        enable_auto_stream_copy: Option<bool>,
        enable_mpegts_m2_ts_mode: Option<bool>,
        framerate: Option<f32>,
        height: Option<i32>,
        level: Option<&types::GetMasterHlsAudioPlaylistLevel>,
        live_stream_id: Option<&str>,
        max_audio_bit_depth: Option<i32>,
        max_audio_channels: Option<i32>,
        max_framerate: Option<f32>,
        max_ref_frames: Option<i32>,
        max_streaming_bitrate: Option<i32>,
        max_video_bit_depth: Option<i32>,
        media_source_id: &str,
        min_segments: Option<i32>,
        params: Option<&str>,
        play_session_id: Option<&str>,
        profile: Option<&str>,
        require_avc: Option<bool>,
        require_non_anamorphic: Option<bool>,
        segment_container: Option<&types::GetMasterHlsAudioPlaylistSegmentContainer>,
        segment_length: Option<i32>,
        start_time_ticks: Option<i64>,
        static_: Option<bool>,
        stream_options: Option<
            &std::collections::HashMap<
                String,
                Option<String>,
            >,
        >,
        subtitle_codec: Option<&types::GetMasterHlsAudioPlaylistSubtitleCodec>,
        subtitle_method: Option<types::SubtitleDeliveryMethod>,
        subtitle_stream_index: Option<i32>,
        tag: Option<&str>,
        transcode_reasons: Option<&str>,
        transcoding_max_audio_channels: Option<i32>,
        video_bit_rate: Option<i32>,
        video_codec: Option<&types::GetMasterHlsAudioPlaylistVideoCodec>,
        video_stream_index: Option<i32>,
        width: Option<i32>,
    ) -> Result<reqwest::Response, Error> {
        self.request(reqwest::Method::GET, format!("/Audio/{}/master.m3u8", encode_path(&item_id.to_string())))
            .query_opt("allowAudioStreamCopy", allow_audio_stream_copy)
            .query_opt("allowVideoStreamCopy", allow_video_stream_copy)
            .query_opt("audioBitRate", audio_bit_rate)
            .query_opt("audioChannels", audio_channels)
            .query_opt("audioCodec", audio_codec)
            .query_opt("audioSampleRate", audio_sample_rate)
            .query_opt("audioStreamIndex", audio_stream_index)
            .query_opt("breakOnNonKeyFrames", break_on_non_key_frames)
            .query_opt("context", context)
            .query_opt("copyTimestamps", copy_timestamps)
            .query_opt("cpuCoreLimit", cpu_core_limit)
            .query_opt("deInterlace", de_interlace)
            .query_opt("deviceId", device_id)
            .query_opt("deviceProfileId", device_profile_id)
            .query_opt("enableAdaptiveBitrateStreaming", enable_adaptive_bitrate_streaming)
            .query_opt("enableAudioVbrEncoding", enable_audio_vbr_encoding)
            .query_opt("enableAutoStreamCopy", enable_auto_stream_copy)
            .query_opt("enableMpegtsM2TsMode", enable_mpegts_m2_ts_mode)
            .query_opt("framerate", framerate)
            .query_opt("height", height)
            .query_opt("level", level)
            .query_opt("liveStreamId", live_stream_id)
            .query_opt("maxAudioBitDepth", max_audio_bit_depth)
            .query_opt("maxAudioChannels", max_audio_channels)
            .query_opt("maxFramerate", max_framerate)
            .query_opt("maxRefFrames", max_ref_frames)
            .query_opt("maxStreamingBitrate", max_streaming_bitrate)
            .query_opt("maxVideoBitDepth", max_video_bit_depth)
            .query("mediaSourceId", media_source_id)
            .query_opt("minSegments", min_segments)
            .query_opt("params", params)
            .query_opt("playSessionId", play_session_id)
            .query_opt("profile", profile)
            .query_opt("requireAvc", require_avc)
            .query_opt("requireNonAnamorphic", require_non_anamorphic)
            .query_opt("segmentContainer", segment_container)
            .query_opt("segmentLength", segment_length)
            .query_opt("startTimeTicks", start_time_ticks)
            .query_opt("static", static_)
            .query_opt("streamOptions", stream_options.map(|v| serde_json::to_string(v).unwrap_or_default()))
            .query_opt("subtitleCodec", subtitle_codec)
            .query_opt("subtitleMethod", subtitle_method)
            .query_opt("subtitleStreamIndex", subtitle_stream_index)
            .query_opt("tag", tag)
            .query_opt("transcodeReasons", transcode_reasons)
            .query_opt("transcodingMaxAudioChannels", transcoding_max_audio_channels)
            .query_opt("videoBitRate", video_bit_rate)
            .query_opt("videoCodec", video_codec)
            .query_opt("videoStreamIndex", video_stream_index)
            .query_opt("width", width)
            .send_response()
            .await
    }

    #[doc = "Gets an audio hls playlist stream\n\nSends a `HEAD` request to `/Audio/{itemId}/master.m3u8`\n\nArguments:\n- `item_id`: The item id.\n- `allow_audio_stream_copy`: Whether or not to allow copying of the audio stream url.\n- `allow_video_stream_copy`: Whether or not to allow copying of the video stream url.\n- `audio_bit_rate`: Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults.\n- `audio_channels`: Optional. Specify a specific number of audio channels to encode to, e.g. 2.\n- `audio_codec`: Optional. Specify an audio codec to encode to, e.g. mp3.\n- `audio_sample_rate`: Optional. Specify a specific audio sample rate, e.g. 44100.\n- `audio_stream_index`: Optional. The index of the audio stream to use. If omitted the first audio stream will be used.\n- `break_on_non_key_frames`: Optional. Whether to break on non key frames.\n- `context`: Optional. The MediaBrowser.Model.Dlna.EncodingContext.\n- `copy_timestamps`: Whether or not to copy timestamps when transcoding with an offset. Defaults to false.\n- `cpu_core_limit`: Optional. The limit of how many cpu cores to use.\n- `de_interlace`: Optional. Whether to deinterlace the video.\n- `device_id`: The device id of the client requesting. Used to stop encoding processes when needed.\n- `device_profile_id`: Optional. The dlna device profile id to utilize.\n- `enable_adaptive_bitrate_streaming`: Enable adaptive bitrate streaming.\n- `enable_audio_vbr_encoding`: Optional. Whether to enable Audio Encoding.\n- `enable_auto_stream_copy`: Whether or not to allow automatic stream copy if requested values match the original source. Defaults to true.\n- `enable_mpegts_m2_ts_mode`: Optional. Whether to enable the MpegtsM2Ts mode.\n- `framerate`: Optional. A specific video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.\n- `height`: Optional. The fixed vertical resolution of the encoded video.\n- `level`: Optional. Specify a level for the encoder profile (varies by encoder), e.g. 3, 3.1.\n- `live_stream_id`: The live stream id.\n- `max_audio_bit_depth`: Optional. The maximum audio bit depth.\n- `max_audio_channels`: Optional. Specify a maximum number of audio channels to encode to, e.g. 2.\n- `max_framerate`: Optional. A specific maximum video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.\n- `max_ref_frames`: Optional.\n- `max_streaming_bitrate`: Optional. The maximum streaming bitrate.\n- `max_video_bit_depth`: Optional. The maximum video bit depth.\n- `media_source_id`: The media version id, if playing an alternate version.\n- `min_segments`: The minimum number of segments.\n- `params`: The streaming parameters.\n- `play_session_id`: The play session id.\n- `profile`: Optional. Specify a specific an encoder profile (varies by encoder), e.g. main, baseline, high.\n- `require_avc`: Optional. Whether to require avc.\n- `require_non_anamorphic`: Optional. Whether to require a non anamorphic stream.\n- `segment_container`: The segment container.\n- `segment_length`: The segment length.\n- `start_time_ticks`: Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms.\n- `static_`: Optional. If true, the original file will be streamed statically without any encoding. Use either no url extension or the original file extension. true/false.\n- `stream_options`: Optional. The streaming options.\n- `subtitle_codec`: Optional. Specify a subtitle codec to encode to.\n- `subtitle_method`: Optional. Specify the subtitle delivery method.\n- `subtitle_stream_index`: Optional. The index of the subtitle stream to use. If omitted no subtitles will be used.\n- `tag`: The tag.\n- `transcode_reasons`: Optional. The transcoding reason.\n- `transcoding_max_audio_channels`: Optional. The maximum number of audio channels to transcode.\n- `video_bit_rate`: Optional. Specify a video bitrate to encode to, e.g. 500000. If omitted this will be left to encoder defaults.\n- `video_codec`: Optional. Specify a video codec to encode to, e.g. h264.\n- `video_stream_index`: Optional. The index of the video stream to use. If omitted the first video stream will be used.\n- `width`: Optional. The fixed horizontal resolution of the encoded video.\n"]
    pub async fn head_master_hls_audio_playlist(
        &self,
        item_id: &uuid::Uuid,
        allow_audio_stream_copy: Option<bool>,
        allow_video_stream_copy: Option<bool>,
        audio_bit_rate: Option<i32>,
        audio_channels: Option<i32>,
        audio_codec: Option<&types::HeadMasterHlsAudioPlaylistAudioCodec>,
        audio_sample_rate: Option<i32>,
        audio_stream_index: Option<i32>,
        break_on_non_key_frames: Option<bool>,
        context: Option<types::EncodingContext>,
        copy_timestamps: Option<bool>,
        cpu_core_limit: Option<i32>,
        de_interlace: Option<bool>,
        device_id: Option<&str>,
        device_profile_id: Option<&str>,
        enable_adaptive_bitrate_streaming: Option<bool>,
        enable_audio_vbr_encoding: Option<bool>,
        enable_auto_stream_copy: Option<bool>,
        enable_mpegts_m2_ts_mode: Option<bool>,
        framerate: Option<f32>,
        height: Option<i32>,
        level: Option<&types::HeadMasterHlsAudioPlaylistLevel>,
        live_stream_id: Option<&str>,
        max_audio_bit_depth: Option<i32>,
        max_audio_channels: Option<i32>,
        max_framerate: Option<f32>,
        max_ref_frames: Option<i32>,
        max_streaming_bitrate: Option<i32>,
        max_video_bit_depth: Option<i32>,
        media_source_id: &str,
        min_segments: Option<i32>,
        params: Option<&str>,
        play_session_id: Option<&str>,
        profile: Option<&str>,
        require_avc: Option<bool>,
        require_non_anamorphic: Option<bool>,
        segment_container: Option<&types::HeadMasterHlsAudioPlaylistSegmentContainer>,
        segment_length: Option<i32>,
        start_time_ticks: Option<i64>,
        static_: Option<bool>,
        stream_options: Option<
            &std::collections::HashMap<
                String,
                Option<String>,
            >,
        >,
        subtitle_codec: Option<&types::HeadMasterHlsAudioPlaylistSubtitleCodec>,
        subtitle_method: Option<types::SubtitleDeliveryMethod>,
        subtitle_stream_index: Option<i32>,
        tag: Option<&str>,
        transcode_reasons: Option<&str>,
        transcoding_max_audio_channels: Option<i32>,
        video_bit_rate: Option<i32>,
        video_codec: Option<&types::HeadMasterHlsAudioPlaylistVideoCodec>,
        video_stream_index: Option<i32>,
        width: Option<i32>,
    ) -> Result<reqwest::Response, Error> {
        self.request(reqwest::Method::HEAD, format!("/Audio/{}/master.m3u8", encode_path(&item_id.to_string())))
            .query_opt("allowAudioStreamCopy", allow_audio_stream_copy)
            .query_opt("allowVideoStreamCopy", allow_video_stream_copy)
            .query_opt("audioBitRate", audio_bit_rate)
            .query_opt("audioChannels", audio_channels)
            .query_opt("audioCodec", audio_codec)
            .query_opt("audioSampleRate", audio_sample_rate)
            .query_opt("audioStreamIndex", audio_stream_index)
            .query_opt("breakOnNonKeyFrames", break_on_non_key_frames)
            .query_opt("context", context)
            .query_opt("copyTimestamps", copy_timestamps)
            .query_opt("cpuCoreLimit", cpu_core_limit)
            .query_opt("deInterlace", de_interlace)
            .query_opt("deviceId", device_id)
            .query_opt("deviceProfileId", device_profile_id)
            .query_opt("enableAdaptiveBitrateStreaming", enable_adaptive_bitrate_streaming)
            .query_opt("enableAudioVbrEncoding", enable_audio_vbr_encoding)
            .query_opt("enableAutoStreamCopy", enable_auto_stream_copy)
            .query_opt("enableMpegtsM2TsMode", enable_mpegts_m2_ts_mode)
            .query_opt("framerate", framerate)
            .query_opt("height", height)
            .query_opt("level", level)
            .query_opt("liveStreamId", live_stream_id)
            .query_opt("maxAudioBitDepth", max_audio_bit_depth)
            .query_opt("maxAudioChannels", max_audio_channels)
            .query_opt("maxFramerate", max_framerate)
            .query_opt("maxRefFrames", max_ref_frames)
            .query_opt("maxStreamingBitrate", max_streaming_bitrate)
            .query_opt("maxVideoBitDepth", max_video_bit_depth)
            .query("mediaSourceId", media_source_id)
            .query_opt("minSegments", min_segments)
            .query_opt("params", params)
            .query_opt("playSessionId", play_session_id)
            .query_opt("profile", profile)
            .query_opt("requireAvc", require_avc)
            .query_opt("requireNonAnamorphic", require_non_anamorphic)
            .query_opt("segmentContainer", segment_container)
            .query_opt("segmentLength", segment_length)
            .query_opt("startTimeTicks", start_time_ticks)
            .query_opt("static", static_)
            .query_opt("streamOptions", stream_options.map(|v| serde_json::to_string(v).unwrap_or_default()))
            .query_opt("subtitleCodec", subtitle_codec)
            .query_opt("subtitleMethod", subtitle_method)
            .query_opt("subtitleStreamIndex", subtitle_stream_index)
            .query_opt("tag", tag)
            .query_opt("transcodeReasons", transcode_reasons)
            .query_opt("transcodingMaxAudioChannels", transcoding_max_audio_channels)
            .query_opt("videoBitRate", video_bit_rate)
            .query_opt("videoCodec", video_codec)
            .query_opt("videoStreamIndex", video_stream_index)
            .query_opt("width", width)
            .send_response()
            .await
    }

    #[doc = "Gets the specified audio segment for an audio item\n\nSends a `GET` request to `/Audio/{itemId}/hls/{segmentId}/stream.aac`\n\nArguments:\n- `item_id`: The item id.\n- `segment_id`: The segment id.\n"]
    pub async fn get_hls_audio_segment_legacy_aac(
        &self,
        item_id: &str,
        segment_id: &str,
    ) -> Result<reqwest::Response, Error> {
        self.request(reqwest::Method::GET, format!("/Audio/{}/hls/{}/stream.aac", encode_path(item_id), encode_path(segment_id)))
            .send_response()
            .await
    }

    #[doc = "Gets the specified audio segment for an audio item\n\nSends a `GET` request to `/Audio/{itemId}/hls/{segmentId}/stream.mp3`\n\nArguments:\n- `item_id`: The item id.\n- `segment_id`: The segment id.\n"]
    pub async fn get_hls_audio_segment_legacy_mp3(
        &self,
        item_id: &str,
        segment_id: &str,
    ) -> Result<reqwest::Response, Error> {
        self.request(reqwest::Method::GET, format!("/Audio/{}/hls/{}/stream.mp3", encode_path(item_id), encode_path(segment_id)))
            .send_response()
            .await
    }

    #[doc = "Gets an item's lyrics\n\nSends a `GET` request to `/Audio/{itemId}/Lyrics`\n\nArguments:\n- `item_id`: Item id.\n"]
    pub async fn get_lyrics(
        &self,
        item_id: &uuid::Uuid,
    ) -> Result<types::LyricDto, Error> {
        self.request(reqwest::Method::GET, format!("/Audio/{}/Lyrics", encode_path(&item_id.to_string())))
            .send()
            .await
    }

    #[doc = "Upload an external lyric file\n\nSends a `POST` request to `/Audio/{itemId}/Lyrics`\n\nArguments:\n- `item_id`: The item the lyric belongs to.\n- `file_name`: Name of the file being uploaded.\n- `body`\n"]
    pub async fn upload_lyrics<B: Into<reqwest::Body>>(
        &self,
        item_id: &uuid::Uuid,
        file_name: &str,
        body: B,
    ) -> Result<types::LyricDto, Error> {
        self.request(reqwest::Method::POST, format!("/Audio/{}/Lyrics", encode_path(&item_id.to_string())))
            .query("fileName", file_name)
            .raw_body(body, "application/octet-stream")
            .send()
            .await
    }

    #[doc = "Deletes an external lyric file\n\nSends a `DELETE` request to `/Audio/{itemId}/Lyrics`\n\nArguments:\n- `item_id`: The item id.\n"]
    pub async fn delete_lyrics(
        &self,
        item_id: &uuid::Uuid,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::DELETE, format!("/Audio/{}/Lyrics", encode_path(&item_id.to_string())))
            .send_no_content()
            .await
    }

    #[doc = "Search remote lyrics\n\nSends a `GET` request to `/Audio/{itemId}/RemoteSearch/Lyrics`\n\nArguments:\n- `item_id`: The item id.\n"]
    pub async fn search_remote_lyrics(
        &self,
        item_id: &uuid::Uuid,
    ) -> Result<Vec<types::RemoteLyricInfoDto>, Error> {
        self.request(reqwest::Method::GET, format!("/Audio/{}/RemoteSearch/Lyrics", encode_path(&item_id.to_string())))
            .send()
            .await
    }

    #[doc = "Downloads a remote lyric\n\nSends a `POST` request to `/Audio/{itemId}/RemoteSearch/Lyrics/{lyricId}`\n\nArguments:\n- `item_id`: The item id.\n- `lyric_id`: The lyric id.\n"]
    pub async fn download_remote_lyrics(
        &self,
        item_id: &uuid::Uuid,
        lyric_id: &str,
    ) -> Result<types::LyricDto, Error> {
        self.request(reqwest::Method::POST, format!("/Audio/{}/RemoteSearch/Lyrics/{}", encode_path(&item_id.to_string()), encode_path(lyric_id)))
            .send()
            .await
    }

    #[doc = "Gets an audio stream\n\nSends a `GET` request to `/Audio/{itemId}/universal`\n\nArguments:\n- `item_id`: The item id.\n- `audio_bit_rate`: Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults.\n- `audio_codec`: Optional. The audio codec to transcode to.\n- `break_on_non_key_frames`: Optional. Whether to break on non key frames.\n- `container`: Optional. The audio container.\n- `device_id`: The device id of the client requesting. Used to stop encoding processes when needed.\n- `enable_audio_vbr_encoding`: Optional. Whether to enable Audio Encoding.\n- `enable_redirection`: Whether to enable redirection. Defaults to true.\n- `enable_remote_media`: Optional. Whether to enable remote media.\n- `max_audio_bit_depth`: Optional. The maximum audio bit depth.\n- `max_audio_channels`: Optional. The maximum number of audio channels.\n- `max_audio_sample_rate`: Optional. The maximum audio sample rate.\n- `max_streaming_bitrate`: Optional. The maximum streaming bitrate.\n- `media_source_id`: The media version id, if playing an alternate version.\n- `start_time_ticks`: Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms.\n- `transcoding_audio_channels`: Optional. The number of how many audio channels to transcode to.\n- `transcoding_container`: Optional. The container to transcode to.\n- `transcoding_protocol`: Optional. The transcoding protocol.\n- `user_id`: Optional. The user id.\n"]
    pub async fn get_universal_audio_stream(
        &self,
        item_id: &uuid::Uuid,
        audio_bit_rate: Option<i32>,
        audio_codec: Option<&types::GetUniversalAudioStreamAudioCodec>,
        break_on_non_key_frames: Option<bool>,
        container: Option<&Vec<String>>,
        device_id: Option<&str>,
        enable_audio_vbr_encoding: Option<bool>,
        enable_redirection: Option<bool>,
        enable_remote_media: Option<bool>,
        max_audio_bit_depth: Option<i32>,
        max_audio_channels: Option<i32>,
        max_audio_sample_rate: Option<i32>,
        max_streaming_bitrate: Option<i32>,
        media_source_id: Option<&str>,
        start_time_ticks: Option<i64>,
        transcoding_audio_channels: Option<i32>,
        transcoding_container: Option<&types::GetUniversalAudioStreamTranscodingContainer>,
        transcoding_protocol: Option<types::MediaStreamProtocol>,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<reqwest::Response, Error> {
        self.request(reqwest::Method::GET, format!("/Audio/{}/universal", encode_path(&item_id.to_string())))
            .query_opt("audioBitRate", audio_bit_rate)
            .query_opt("audioCodec", audio_codec)
            .query_opt("breakOnNonKeyFrames", break_on_non_key_frames)
            .query_list_opt("container", container)
            .query_opt("deviceId", device_id)
            .query_opt("enableAudioVbrEncoding", enable_audio_vbr_encoding)
            .query_opt("enableRedirection", enable_redirection)
            .query_opt("enableRemoteMedia", enable_remote_media)
            .query_opt("maxAudioBitDepth", max_audio_bit_depth)
            .query_opt("maxAudioChannels", max_audio_channels)
            .query_opt("maxAudioSampleRate", max_audio_sample_rate)
            .query_opt("maxStreamingBitrate", max_streaming_bitrate)
            .query_opt("mediaSourceId", media_source_id)
            .query_opt("startTimeTicks", start_time_ticks)
            .query_opt("transcodingAudioChannels", transcoding_audio_channels)
            .query_opt("transcodingContainer", transcoding_container)
            .query_opt("transcodingProtocol", transcoding_protocol)
            .query_opt("userId", user_id)
            .send_response()
            .await
    }

    #[doc = "Gets an audio stream\n\nSends a `HEAD` request to `/Audio/{itemId}/universal`\n\nArguments:\n- `item_id`: The item id.\n- `audio_bit_rate`: Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults.\n- `audio_codec`: Optional. The audio codec to transcode to.\n- `break_on_non_key_frames`: Optional. Whether to break on non key frames.\n- `container`: Optional. The audio container.\n- `device_id`: The device id of the client requesting. Used to stop encoding processes when needed.\n- `enable_audio_vbr_encoding`: Optional. Whether to enable Audio Encoding.\n- `enable_redirection`: Whether to enable redirection. Defaults to true.\n- `enable_remote_media`: Optional. Whether to enable remote media.\n- `max_audio_bit_depth`: Optional. The maximum audio bit depth.\n- `max_audio_channels`: Optional. The maximum number of audio channels.\n- `max_audio_sample_rate`: Optional. The maximum audio sample rate.\n- `max_streaming_bitrate`: Optional. The maximum streaming bitrate.\n- `media_source_id`: The media version id, if playing an alternate version.\n- `start_time_ticks`: Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms.\n- `transcoding_audio_channels`: Optional. The number of how many audio channels to transcode to.\n- `transcoding_container`: Optional. The container to transcode to.\n- `transcoding_protocol`: Optional. The transcoding protocol.\n- `user_id`: Optional. The user id.\n"]
    pub async fn head_universal_audio_stream(
        &self,
        item_id: &uuid::Uuid,
        audio_bit_rate: Option<i32>,
        audio_codec: Option<&types::HeadUniversalAudioStreamAudioCodec>,
        break_on_non_key_frames: Option<bool>,
        container: Option<&Vec<String>>,
        device_id: Option<&str>,
        enable_audio_vbr_encoding: Option<bool>,
        enable_redirection: Option<bool>,
        enable_remote_media: Option<bool>,
        max_audio_bit_depth: Option<i32>,
        max_audio_channels: Option<i32>,
        max_audio_sample_rate: Option<i32>,
        max_streaming_bitrate: Option<i32>,
        media_source_id: Option<&str>,
        start_time_ticks: Option<i64>,
        transcoding_audio_channels: Option<i32>,
        transcoding_container: Option<&types::HeadUniversalAudioStreamTranscodingContainer>,
        transcoding_protocol: Option<types::MediaStreamProtocol>,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<reqwest::Response, Error> {
        self.request(reqwest::Method::HEAD, format!("/Audio/{}/universal", encode_path(&item_id.to_string())))
            .query_opt("audioBitRate", audio_bit_rate)
            .query_opt("audioCodec", audio_codec)
            .query_opt("breakOnNonKeyFrames", break_on_non_key_frames)
            .query_list_opt("container", container)
            .query_opt("deviceId", device_id)
            .query_opt("enableAudioVbrEncoding", enable_audio_vbr_encoding)
            .query_opt("enableRedirection", enable_redirection)
            .query_opt("enableRemoteMedia", enable_remote_media)
            .query_opt("maxAudioBitDepth", max_audio_bit_depth)
            .query_opt("maxAudioChannels", max_audio_channels)
            .query_opt("maxAudioSampleRate", max_audio_sample_rate)
            .query_opt("maxStreamingBitrate", max_streaming_bitrate)
            .query_opt("mediaSourceId", media_source_id)
            .query_opt("startTimeTicks", start_time_ticks)
            .query_opt("transcodingAudioChannels", transcoding_audio_channels)
            .query_opt("transcodingContainer", transcoding_container)
            .query_opt("transcodingProtocol", transcoding_protocol)
            .query_opt("userId", user_id)
            .send_response()
            .await
    }
}
