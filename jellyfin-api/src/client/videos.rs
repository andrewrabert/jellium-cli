use crate::Client;
use crate::error::Error;
use crate::query;
use crate::types;
use crate::util::encode_path;

impl Client {
    #[doc = "Gets a video stream using HTTP live streaming\n\nSends a `GET` request to `/Videos/{itemId}/hls1/{playlistId}/{segmentId}.{container}`\n\nArguments:\n- `item_id`: The item id.\n- `playlist_id`: The playlist id.\n- `segment_id`: The segment id.\n- `container`: The video container. Possible values are: ts, webm, asf, wmv, ogv, mp4, m4v, mkv, mpeg, mpg, avi, 3gp, wmv, wtv, m2ts, mov, iso, flv.\n- `actual_segment_length_ticks`: The length of the requested segment in ticks.\n- `allow_audio_stream_copy`: Whether or not to allow copying of the audio stream url.\n- `allow_video_stream_copy`: Whether or not to allow copying of the video stream url.\n- `always_burn_in_subtitle_when_transcoding`: Whether to always burn in subtitles when transcoding.\n- `audio_bit_rate`: Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults.\n- `audio_channels`: Optional. Specify a specific number of audio channels to encode to, e.g. 2.\n- `audio_codec`: Optional. Specify an audio codec to encode to, e.g. mp3.\n- `audio_sample_rate`: Optional. Specify a specific audio sample rate, e.g. 44100.\n- `audio_stream_index`: Optional. The index of the audio stream to use. If omitted the first audio stream will be used.\n- `break_on_non_key_frames`: Optional. Whether to break on non key frames.\n- `context`: Optional. The MediaBrowser.Model.Dlna.EncodingContext.\n- `copy_timestamps`: Whether or not to copy timestamps when transcoding with an offset. Defaults to false.\n- `cpu_core_limit`: Optional. The limit of how many cpu cores to use.\n- `de_interlace`: Optional. Whether to deinterlace the video.\n- `device_id`: The device id of the client requesting. Used to stop encoding processes when needed.\n- `device_profile_id`: Optional. The dlna device profile id to utilize.\n- `enable_audio_vbr_encoding`: Optional. Whether to enable Audio Encoding.\n- `enable_auto_stream_copy`: Whether or not to allow automatic stream copy if requested values match the original source. Defaults to true.\n- `enable_mpegts_m2_ts_mode`: Optional. Whether to enable the MpegtsM2Ts mode.\n- `framerate`: Optional. A specific video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.\n- `height`: Optional. The fixed vertical resolution of the encoded video.\n- `level`: Optional. Specify a level for the encoder profile (varies by encoder), e.g. 3, 3.1.\n- `live_stream_id`: The live stream id.\n- `max_audio_bit_depth`: Optional. The maximum audio bit depth.\n- `max_audio_channels`: Optional. Specify a maximum number of audio channels to encode to, e.g. 2.\n- `max_framerate`: Optional. A specific maximum video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.\n- `max_height`: Optional. The maximum vertical resolution of the encoded video.\n- `max_ref_frames`: Optional.\n- `max_video_bit_depth`: Optional. The maximum video bit depth.\n- `max_width`: Optional. The maximum horizontal resolution of the encoded video.\n- `media_source_id`: The media version id, if playing an alternate version.\n- `min_segments`: The minimum number of segments.\n- `params`: The streaming parameters.\n- `play_session_id`: The play session id.\n- `profile`: Optional. Specify a specific an encoder profile (varies by encoder), e.g. main, baseline, high.\n- `require_avc`: Optional. Whether to require avc.\n- `require_non_anamorphic`: Optional. Whether to require a non anamorphic stream.\n- `runtime_ticks`: The position of the requested segment in ticks.\n- `segment_container`: The segment container.\n- `segment_length`: The desired segment length.\n- `start_time_ticks`: Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms.\n- `static_`: Optional. If true, the original file will be streamed statically without any encoding. Use either no url extension or the original file extension. true/false.\n- `stream_options`: Optional. The streaming options.\n- `subtitle_codec`: Optional. Specify a subtitle codec to encode to.\n- `subtitle_method`: Optional. Specify the subtitle delivery method.\n- `subtitle_stream_index`: Optional. The index of the subtitle stream to use. If omitted no subtitles will be used.\n- `tag`: The tag.\n- `transcode_reasons`: Optional. The transcoding reason.\n- `transcoding_max_audio_channels`: Optional. The maximum number of audio channels to transcode.\n- `video_bit_rate`: Optional. Specify a video bitrate to encode to, e.g. 500000. If omitted this will be left to encoder defaults.\n- `video_codec`: Optional. Specify a video codec to encode to, e.g. h264.\n- `video_stream_index`: Optional. The index of the video stream to use. If omitted the first video stream will be used.\n- `width`: Optional. The fixed horizontal resolution of the encoded video.\n"]
    pub async fn get_hls_video_segment(
        &self,
        asked: &query::GetHlsVideoSegment<'_>,
    ) -> Result<reqwest::Response, Error> {
        self.request(
            reqwest::Method::GET,
            format!(
                "/Videos/{}/hls1/{}/{}.{}",
                encode_path(&asked.item_id.to_string()),
                encode_path(asked.playlist_id),
                encode_path(&asked.segment_id.to_string()),
                encode_path(&asked.container.to_string())
            ),
        )
        .query(
            "actualSegmentLengthTicks",
            asked.actual_segment_length_ticks,
        )
        .query_opt("allowAudioStreamCopy", asked.allow_audio_stream_copy)
        .query_opt("allowVideoStreamCopy", asked.allow_video_stream_copy)
        .query_opt(
            "alwaysBurnInSubtitleWhenTranscoding",
            asked.always_burn_in_subtitle_when_transcoding,
        )
        .query_opt("audioBitRate", asked.audio_bit_rate)
        .query_opt("audioChannels", asked.audio_channels)
        .query_opt("audioCodec", asked.audio_codec)
        .query_opt("audioSampleRate", asked.audio_sample_rate)
        .query_opt("audioStreamIndex", asked.audio_stream_index)
        .query_opt("breakOnNonKeyFrames", asked.break_on_non_key_frames)
        .query_opt("context", asked.context)
        .query_opt("copyTimestamps", asked.copy_timestamps)
        .query_opt("cpuCoreLimit", asked.cpu_core_limit)
        .query_opt("deInterlace", asked.de_interlace)
        .query_opt("deviceId", asked.device_id)
        .query_opt("deviceProfileId", asked.device_profile_id)
        .query_opt("enableAudioVbrEncoding", asked.enable_audio_vbr_encoding)
        .query_opt("enableAutoStreamCopy", asked.enable_auto_stream_copy)
        .query_opt("enableMpegtsM2TsMode", asked.enable_mpegts_m2_ts_mode)
        .query_opt("framerate", asked.framerate)
        .query_opt("height", asked.height)
        .query_opt("level", asked.level)
        .query_opt("liveStreamId", asked.live_stream_id)
        .query_opt("maxAudioBitDepth", asked.max_audio_bit_depth)
        .query_opt("maxAudioChannels", asked.max_audio_channels)
        .query_opt("maxFramerate", asked.max_framerate)
        .query_opt("maxHeight", asked.max_height)
        .query_opt("maxRefFrames", asked.max_ref_frames)
        .query_opt("maxVideoBitDepth", asked.max_video_bit_depth)
        .query_opt("maxWidth", asked.max_width)
        .query_opt("mediaSourceId", asked.media_source_id)
        .query_opt("minSegments", asked.min_segments)
        .query_opt("params", asked.params)
        .query_opt("playSessionId", asked.play_session_id)
        .query_opt("profile", asked.profile)
        .query_opt("requireAvc", asked.require_avc)
        .query_opt("requireNonAnamorphic", asked.require_non_anamorphic)
        .query("runtimeTicks", asked.runtime_ticks)
        .query_opt("segmentContainer", asked.segment_container)
        .query_opt("segmentLength", asked.segment_length)
        .query_opt("startTimeTicks", asked.start_time_ticks)
        .query_opt("static", asked.r#static)
        .query_opt(
            "streamOptions",
            asked
                .stream_options
                .map(|v| serde_json::to_string(v).unwrap_or_default()),
        )
        .query_opt("subtitleCodec", asked.subtitle_codec)
        .query_opt("subtitleMethod", asked.subtitle_method)
        .query_opt("subtitleStreamIndex", asked.subtitle_stream_index)
        .query_opt("tag", asked.tag)
        .query_opt("transcodeReasons", asked.transcode_reasons)
        .query_opt(
            "transcodingMaxAudioChannels",
            asked.transcoding_max_audio_channels,
        )
        .query_opt("videoBitRate", asked.video_bit_rate)
        .query_opt("videoCodec", asked.video_codec)
        .query_opt("videoStreamIndex", asked.video_stream_index)
        .query_opt("width", asked.width)
        .send_response()
        .await
    }

    #[doc = "Gets a hls live stream\n\nSends a `GET` request to `/Videos/{itemId}/live.m3u8`\n\nArguments:\n- `item_id`: The item id.\n- `allow_audio_stream_copy`: Whether or not to allow copying of the audio stream url.\n- `allow_video_stream_copy`: Whether or not to allow copying of the video stream url.\n- `always_burn_in_subtitle_when_transcoding`: Whether to always burn in subtitles when transcoding.\n- `audio_bit_rate`: Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults.\n- `audio_channels`: Optional. Specify a specific number of audio channels to encode to, e.g. 2.\n- `audio_codec`: Optional. Specify an audio codec to encode to, e.g. mp3.\n- `audio_sample_rate`: Optional. Specify a specific audio sample rate, e.g. 44100.\n- `audio_stream_index`: Optional. The index of the audio stream to use. If omitted the first audio stream will be used.\n- `break_on_non_key_frames`: Optional. Whether to break on non key frames.\n- `container`: The audio container.\n- `context`: Optional. The MediaBrowser.Model.Dlna.EncodingContext.\n- `copy_timestamps`: Whether or not to copy timestamps when transcoding with an offset. Defaults to false.\n- `cpu_core_limit`: Optional. The limit of how many cpu cores to use.\n- `de_interlace`: Optional. Whether to deinterlace the video.\n- `device_id`: The device id of the client requesting. Used to stop encoding processes when needed.\n- `device_profile_id`: Optional. The dlna device profile id to utilize.\n- `enable_audio_vbr_encoding`: Optional. Whether to enable Audio Encoding.\n- `enable_auto_stream_copy`: Whether or not to allow automatic stream copy if requested values match the original source. Defaults to true.\n- `enable_mpegts_m2_ts_mode`: Optional. Whether to enable the MpegtsM2Ts mode.\n- `enable_subtitles_in_manifest`: Optional. Whether to enable subtitles in the manifest.\n- `framerate`: Optional. A specific video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.\n- `height`: Optional. The fixed vertical resolution of the encoded video.\n- `level`: Optional. Specify a level for the encoder profile (varies by encoder), e.g. 3, 3.1.\n- `live_stream_id`: The live stream id.\n- `max_audio_bit_depth`: Optional. The maximum audio bit depth.\n- `max_audio_channels`: Optional. Specify a maximum number of audio channels to encode to, e.g. 2.\n- `max_framerate`: Optional. A specific maximum video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.\n- `max_height`: Optional. The max height.\n- `max_ref_frames`: Optional.\n- `max_video_bit_depth`: Optional. The maximum video bit depth.\n- `max_width`: Optional. The max width.\n- `media_source_id`: The media version id, if playing an alternate version.\n- `min_segments`: The minimum number of segments.\n- `params`: The streaming parameters.\n- `play_session_id`: The play session id.\n- `profile`: Optional. Specify a specific an encoder profile (varies by encoder), e.g. main, baseline, high.\n- `require_avc`: Optional. Whether to require avc.\n- `require_non_anamorphic`: Optional. Whether to require a non anamorphic stream.\n- `segment_container`: The segment container.\n- `segment_length`: The segment length.\n- `start_time_ticks`: Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms.\n- `static_`: Optional. If true, the original file will be streamed statically without any encoding. Use either no url extension or the original file extension. true/false.\n- `stream_options`: Optional. The streaming options.\n- `subtitle_codec`: Optional. Specify a subtitle codec to encode to.\n- `subtitle_method`: Optional. Specify the subtitle delivery method.\n- `subtitle_stream_index`: Optional. The index of the subtitle stream to use. If omitted no subtitles will be used.\n- `tag`: The tag.\n- `transcode_reasons`: Optional. The transcoding reason.\n- `transcoding_max_audio_channels`: Optional. The maximum number of audio channels to transcode.\n- `video_bit_rate`: Optional. Specify a video bitrate to encode to, e.g. 500000. If omitted this will be left to encoder defaults.\n- `video_codec`: Optional. Specify a video codec to encode to, e.g. h264.\n- `video_stream_index`: Optional. The index of the video stream to use. If omitted the first video stream will be used.\n- `width`: Optional. The fixed horizontal resolution of the encoded video.\n"]
    pub async fn get_live_hls_stream(
        &self,
        item_id: &uuid::Uuid,
        query: &query::GetLiveHlsStream<'_>,
    ) -> Result<reqwest::Response, Error> {
        self.request(
            reqwest::Method::GET,
            format!("/Videos/{}/live.m3u8", encode_path(&item_id.to_string())),
        )
        .query_opt("allowAudioStreamCopy", query.allow_audio_stream_copy)
        .query_opt("allowVideoStreamCopy", query.allow_video_stream_copy)
        .query_opt(
            "alwaysBurnInSubtitleWhenTranscoding",
            query.always_burn_in_subtitle_when_transcoding,
        )
        .query_opt("audioBitRate", query.audio_bit_rate)
        .query_opt("audioChannels", query.audio_channels)
        .query_opt("audioCodec", query.audio_codec)
        .query_opt("audioSampleRate", query.audio_sample_rate)
        .query_opt("audioStreamIndex", query.audio_stream_index)
        .query_opt("breakOnNonKeyFrames", query.break_on_non_key_frames)
        .query_opt("container", query.container)
        .query_opt("context", query.context)
        .query_opt("copyTimestamps", query.copy_timestamps)
        .query_opt("cpuCoreLimit", query.cpu_core_limit)
        .query_opt("deInterlace", query.de_interlace)
        .query_opt("deviceId", query.device_id)
        .query_opt("deviceProfileId", query.device_profile_id)
        .query_opt("enableAudioVbrEncoding", query.enable_audio_vbr_encoding)
        .query_opt("enableAutoStreamCopy", query.enable_auto_stream_copy)
        .query_opt("enableMpegtsM2TsMode", query.enable_mpegts_m2_ts_mode)
        .query_opt(
            "enableSubtitlesInManifest",
            query.enable_subtitles_in_manifest,
        )
        .query_opt("framerate", query.framerate)
        .query_opt("height", query.height)
        .query_opt("level", query.level)
        .query_opt("liveStreamId", query.live_stream_id)
        .query_opt("maxAudioBitDepth", query.max_audio_bit_depth)
        .query_opt("maxAudioChannels", query.max_audio_channels)
        .query_opt("maxFramerate", query.max_framerate)
        .query_opt("maxHeight", query.max_height)
        .query_opt("maxRefFrames", query.max_ref_frames)
        .query_opt("maxVideoBitDepth", query.max_video_bit_depth)
        .query_opt("maxWidth", query.max_width)
        .query_opt("mediaSourceId", query.media_source_id)
        .query_opt("minSegments", query.min_segments)
        .query_opt("params", query.params)
        .query_opt("playSessionId", query.play_session_id)
        .query_opt("profile", query.profile)
        .query_opt("requireAvc", query.require_avc)
        .query_opt("requireNonAnamorphic", query.require_non_anamorphic)
        .query_opt("segmentContainer", query.segment_container)
        .query_opt("segmentLength", query.segment_length)
        .query_opt("startTimeTicks", query.start_time_ticks)
        .query_opt("static", query.r#static)
        .query_opt(
            "streamOptions",
            query
                .stream_options
                .map(|v| serde_json::to_string(v).unwrap_or_default()),
        )
        .query_opt("subtitleCodec", query.subtitle_codec)
        .query_opt("subtitleMethod", query.subtitle_method)
        .query_opt("subtitleStreamIndex", query.subtitle_stream_index)
        .query_opt("tag", query.tag)
        .query_opt("transcodeReasons", query.transcode_reasons)
        .query_opt(
            "transcodingMaxAudioChannels",
            query.transcoding_max_audio_channels,
        )
        .query_opt("videoBitRate", query.video_bit_rate)
        .query_opt("videoCodec", query.video_codec)
        .query_opt("videoStreamIndex", query.video_stream_index)
        .query_opt("width", query.width)
        .send_response()
        .await
    }

    #[doc = "Gets a video stream using HTTP live streaming\n\nSends a `GET` request to `/Videos/{itemId}/main.m3u8`\n\nArguments:\n- `item_id`: The item id.\n- `allow_audio_stream_copy`: Whether or not to allow copying of the audio stream url.\n- `allow_video_stream_copy`: Whether or not to allow copying of the video stream url.\n- `always_burn_in_subtitle_when_transcoding`: Whether to always burn in subtitles when transcoding.\n- `audio_bit_rate`: Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults.\n- `audio_channels`: Optional. Specify a specific number of audio channels to encode to, e.g. 2.\n- `audio_codec`: Optional. Specify an audio codec to encode to, e.g. mp3.\n- `audio_sample_rate`: Optional. Specify a specific audio sample rate, e.g. 44100.\n- `audio_stream_index`: Optional. The index of the audio stream to use. If omitted the first audio stream will be used.\n- `break_on_non_key_frames`: Optional. Whether to break on non key frames.\n- `context`: Optional. The MediaBrowser.Model.Dlna.EncodingContext.\n- `copy_timestamps`: Whether or not to copy timestamps when transcoding with an offset. Defaults to false.\n- `cpu_core_limit`: Optional. The limit of how many cpu cores to use.\n- `de_interlace`: Optional. Whether to deinterlace the video.\n- `device_id`: The device id of the client requesting. Used to stop encoding processes when needed.\n- `device_profile_id`: Optional. The dlna device profile id to utilize.\n- `enable_audio_vbr_encoding`: Optional. Whether to enable Audio Encoding.\n- `enable_auto_stream_copy`: Whether or not to allow automatic stream copy if requested values match the original source. Defaults to true.\n- `enable_mpegts_m2_ts_mode`: Optional. Whether to enable the MpegtsM2Ts mode.\n- `framerate`: Optional. A specific video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.\n- `height`: Optional. The fixed vertical resolution of the encoded video.\n- `level`: Optional. Specify a level for the encoder profile (varies by encoder), e.g. 3, 3.1.\n- `live_stream_id`: The live stream id.\n- `max_audio_bit_depth`: Optional. The maximum audio bit depth.\n- `max_audio_channels`: Optional. Specify a maximum number of audio channels to encode to, e.g. 2.\n- `max_framerate`: Optional. A specific maximum video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.\n- `max_height`: Optional. The maximum vertical resolution of the encoded video.\n- `max_ref_frames`: Optional.\n- `max_video_bit_depth`: Optional. The maximum video bit depth.\n- `max_width`: Optional. The maximum horizontal resolution of the encoded video.\n- `media_source_id`: The media version id, if playing an alternate version.\n- `min_segments`: The minimum number of segments.\n- `params`: The streaming parameters.\n- `play_session_id`: The play session id.\n- `profile`: Optional. Specify a specific an encoder profile (varies by encoder), e.g. main, baseline, high.\n- `require_avc`: Optional. Whether to require avc.\n- `require_non_anamorphic`: Optional. Whether to require a non anamorphic stream.\n- `segment_container`: The segment container.\n- `segment_length`: The segment length.\n- `start_time_ticks`: Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms.\n- `static_`: Optional. If true, the original file will be streamed statically without any encoding. Use either no url extension or the original file extension. true/false.\n- `stream_options`: Optional. The streaming options.\n- `subtitle_codec`: Optional. Specify a subtitle codec to encode to.\n- `subtitle_method`: Optional. Specify the subtitle delivery method.\n- `subtitle_stream_index`: Optional. The index of the subtitle stream to use. If omitted no subtitles will be used.\n- `tag`: The tag.\n- `transcode_reasons`: Optional. The transcoding reason.\n- `transcoding_max_audio_channels`: Optional. The maximum number of audio channels to transcode.\n- `video_bit_rate`: Optional. Specify a video bitrate to encode to, e.g. 500000. If omitted this will be left to encoder defaults.\n- `video_codec`: Optional. Specify a video codec to encode to, e.g. h264.\n- `video_stream_index`: Optional. The index of the video stream to use. If omitted the first video stream will be used.\n- `width`: Optional. The fixed horizontal resolution of the encoded video.\n"]
    pub async fn get_variant_hls_video_playlist(
        &self,
        item_id: &uuid::Uuid,
        query: &query::GetVariantHlsVideoPlaylist<'_>,
    ) -> Result<reqwest::Response, Error> {
        self.request(
            reqwest::Method::GET,
            format!("/Videos/{}/main.m3u8", encode_path(&item_id.to_string())),
        )
        .query_opt("allowAudioStreamCopy", query.allow_audio_stream_copy)
        .query_opt("allowVideoStreamCopy", query.allow_video_stream_copy)
        .query_opt(
            "alwaysBurnInSubtitleWhenTranscoding",
            query.always_burn_in_subtitle_when_transcoding,
        )
        .query_opt("audioBitRate", query.audio_bit_rate)
        .query_opt("audioChannels", query.audio_channels)
        .query_opt("audioCodec", query.audio_codec)
        .query_opt("audioSampleRate", query.audio_sample_rate)
        .query_opt("audioStreamIndex", query.audio_stream_index)
        .query_opt("breakOnNonKeyFrames", query.break_on_non_key_frames)
        .query_opt("context", query.context)
        .query_opt("copyTimestamps", query.copy_timestamps)
        .query_opt("cpuCoreLimit", query.cpu_core_limit)
        .query_opt("deInterlace", query.de_interlace)
        .query_opt("deviceId", query.device_id)
        .query_opt("deviceProfileId", query.device_profile_id)
        .query_opt("enableAudioVbrEncoding", query.enable_audio_vbr_encoding)
        .query_opt("enableAutoStreamCopy", query.enable_auto_stream_copy)
        .query_opt("enableMpegtsM2TsMode", query.enable_mpegts_m2_ts_mode)
        .query_opt("framerate", query.framerate)
        .query_opt("height", query.height)
        .query_opt("level", query.level)
        .query_opt("liveStreamId", query.live_stream_id)
        .query_opt("maxAudioBitDepth", query.max_audio_bit_depth)
        .query_opt("maxAudioChannels", query.max_audio_channels)
        .query_opt("maxFramerate", query.max_framerate)
        .query_opt("maxHeight", query.max_height)
        .query_opt("maxRefFrames", query.max_ref_frames)
        .query_opt("maxVideoBitDepth", query.max_video_bit_depth)
        .query_opt("maxWidth", query.max_width)
        .query_opt("mediaSourceId", query.media_source_id)
        .query_opt("minSegments", query.min_segments)
        .query_opt("params", query.params)
        .query_opt("playSessionId", query.play_session_id)
        .query_opt("profile", query.profile)
        .query_opt("requireAvc", query.require_avc)
        .query_opt("requireNonAnamorphic", query.require_non_anamorphic)
        .query_opt("segmentContainer", query.segment_container)
        .query_opt("segmentLength", query.segment_length)
        .query_opt("startTimeTicks", query.start_time_ticks)
        .query_opt("static", query.r#static)
        .query_opt(
            "streamOptions",
            query
                .stream_options
                .map(|v| serde_json::to_string(v).unwrap_or_default()),
        )
        .query_opt("subtitleCodec", query.subtitle_codec)
        .query_opt("subtitleMethod", query.subtitle_method)
        .query_opt("subtitleStreamIndex", query.subtitle_stream_index)
        .query_opt("tag", query.tag)
        .query_opt("transcodeReasons", query.transcode_reasons)
        .query_opt(
            "transcodingMaxAudioChannels",
            query.transcoding_max_audio_channels,
        )
        .query_opt("videoBitRate", query.video_bit_rate)
        .query_opt("videoCodec", query.video_codec)
        .query_opt("videoStreamIndex", query.video_stream_index)
        .query_opt("width", query.width)
        .send_response()
        .await
    }

    #[doc = "Gets a video hls playlist stream\n\nSends a `GET` request to `/Videos/{itemId}/master.m3u8`\n\nArguments:\n- `item_id`: The item id.\n- `allow_audio_stream_copy`: Whether or not to allow copying of the audio stream url.\n- `allow_video_stream_copy`: Whether or not to allow copying of the video stream url.\n- `always_burn_in_subtitle_when_transcoding`: Whether to always burn in subtitles when transcoding.\n- `audio_bit_rate`: Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults.\n- `audio_channels`: Optional. Specify a specific number of audio channels to encode to, e.g. 2.\n- `audio_codec`: Optional. Specify an audio codec to encode to, e.g. mp3.\n- `audio_sample_rate`: Optional. Specify a specific audio sample rate, e.g. 44100.\n- `audio_stream_index`: Optional. The index of the audio stream to use. If omitted the first audio stream will be used.\n- `break_on_non_key_frames`: Optional. Whether to break on non key frames.\n- `context`: Optional. The MediaBrowser.Model.Dlna.EncodingContext.\n- `copy_timestamps`: Whether or not to copy timestamps when transcoding with an offset. Defaults to false.\n- `cpu_core_limit`: Optional. The limit of how many cpu cores to use.\n- `de_interlace`: Optional. Whether to deinterlace the video.\n- `device_id`: The device id of the client requesting. Used to stop encoding processes when needed.\n- `device_profile_id`: Optional. The dlna device profile id to utilize.\n- `enable_adaptive_bitrate_streaming`: Enable adaptive bitrate streaming.\n- `enable_audio_vbr_encoding`: Whether to enable Audio Encoding.\n- `enable_auto_stream_copy`: Whether or not to allow automatic stream copy if requested values match the original source. Defaults to true.\n- `enable_mpegts_m2_ts_mode`: Optional. Whether to enable the MpegtsM2Ts mode.\n- `enable_trickplay`: Enable trickplay image playlists being added to master playlist.\n- `framerate`: Optional. A specific video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.\n- `height`: Optional. The fixed vertical resolution of the encoded video.\n- `level`: Optional. Specify a level for the encoder profile (varies by encoder), e.g. 3, 3.1.\n- `live_stream_id`: The live stream id.\n- `max_audio_bit_depth`: Optional. The maximum audio bit depth.\n- `max_audio_channels`: Optional. Specify a maximum number of audio channels to encode to, e.g. 2.\n- `max_framerate`: Optional. A specific maximum video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.\n- `max_height`: Optional. The maximum vertical resolution of the encoded video.\n- `max_ref_frames`: Optional.\n- `max_video_bit_depth`: Optional. The maximum video bit depth.\n- `max_width`: Optional. The maximum horizontal resolution of the encoded video.\n- `media_source_id`: The media version id, if playing an alternate version.\n- `min_segments`: The minimum number of segments.\n- `params`: The streaming parameters.\n- `play_session_id`: The play session id.\n- `profile`: Optional. Specify a specific an encoder profile (varies by encoder), e.g. main, baseline, high.\n- `require_avc`: Optional. Whether to require avc.\n- `require_non_anamorphic`: Optional. Whether to require a non anamorphic stream.\n- `segment_container`: The segment container.\n- `segment_length`: The segment length.\n- `start_time_ticks`: Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms.\n- `static_`: Optional. If true, the original file will be streamed statically without any encoding. Use either no url extension or the original file extension. true/false.\n- `stream_options`: Optional. The streaming options.\n- `subtitle_codec`: Optional. Specify a subtitle codec to encode to.\n- `subtitle_method`: Optional. Specify the subtitle delivery method.\n- `subtitle_stream_index`: Optional. The index of the subtitle stream to use. If omitted no subtitles will be used.\n- `tag`: The tag.\n- `transcode_reasons`: Optional. The transcoding reason.\n- `transcoding_max_audio_channels`: Optional. The maximum number of audio channels to transcode.\n- `video_bit_rate`: Optional. Specify a video bitrate to encode to, e.g. 500000. If omitted this will be left to encoder defaults.\n- `video_codec`: Optional. Specify a video codec to encode to, e.g. h264.\n- `video_stream_index`: Optional. The index of the video stream to use. If omitted the first video stream will be used.\n- `width`: Optional. The fixed horizontal resolution of the encoded video.\n"]
    pub async fn get_master_hls_video_playlist(
        &self,
        item_id: &uuid::Uuid,
        media_source_id: &str,
        query: &query::GetMasterHlsVideoPlaylist<'_>,
    ) -> Result<reqwest::Response, Error> {
        self.request(
            reqwest::Method::GET,
            format!("/Videos/{}/master.m3u8", encode_path(&item_id.to_string())),
        )
        .query_opt("allowAudioStreamCopy", query.allow_audio_stream_copy)
        .query_opt("allowVideoStreamCopy", query.allow_video_stream_copy)
        .query_opt(
            "alwaysBurnInSubtitleWhenTranscoding",
            query.always_burn_in_subtitle_when_transcoding,
        )
        .query_opt("audioBitRate", query.audio_bit_rate)
        .query_opt("audioChannels", query.audio_channels)
        .query_opt("audioCodec", query.audio_codec)
        .query_opt("audioSampleRate", query.audio_sample_rate)
        .query_opt("audioStreamIndex", query.audio_stream_index)
        .query_opt("breakOnNonKeyFrames", query.break_on_non_key_frames)
        .query_opt("context", query.context)
        .query_opt("copyTimestamps", query.copy_timestamps)
        .query_opt("cpuCoreLimit", query.cpu_core_limit)
        .query_opt("deInterlace", query.de_interlace)
        .query_opt("deviceId", query.device_id)
        .query_opt("deviceProfileId", query.device_profile_id)
        .query_opt(
            "enableAdaptiveBitrateStreaming",
            query.enable_adaptive_bitrate_streaming,
        )
        .query_opt("enableAudioVbrEncoding", query.enable_audio_vbr_encoding)
        .query_opt("enableAutoStreamCopy", query.enable_auto_stream_copy)
        .query_opt("enableMpegtsM2TsMode", query.enable_mpegts_m2_ts_mode)
        .query_opt("enableTrickplay", query.enable_trickplay)
        .query_opt("framerate", query.framerate)
        .query_opt("height", query.height)
        .query_opt("level", query.level)
        .query_opt("liveStreamId", query.live_stream_id)
        .query_opt("maxAudioBitDepth", query.max_audio_bit_depth)
        .query_opt("maxAudioChannels", query.max_audio_channels)
        .query_opt("maxFramerate", query.max_framerate)
        .query_opt("maxHeight", query.max_height)
        .query_opt("maxRefFrames", query.max_ref_frames)
        .query_opt("maxVideoBitDepth", query.max_video_bit_depth)
        .query_opt("maxWidth", query.max_width)
        .query("mediaSourceId", media_source_id)
        .query_opt("minSegments", query.min_segments)
        .query_opt("params", query.params)
        .query_opt("playSessionId", query.play_session_id)
        .query_opt("profile", query.profile)
        .query_opt("requireAvc", query.require_avc)
        .query_opt("requireNonAnamorphic", query.require_non_anamorphic)
        .query_opt("segmentContainer", query.segment_container)
        .query_opt("segmentLength", query.segment_length)
        .query_opt("startTimeTicks", query.start_time_ticks)
        .query_opt("static", query.r#static)
        .query_opt(
            "streamOptions",
            query
                .stream_options
                .map(|v| serde_json::to_string(v).unwrap_or_default()),
        )
        .query_opt("subtitleCodec", query.subtitle_codec)
        .query_opt("subtitleMethod", query.subtitle_method)
        .query_opt("subtitleStreamIndex", query.subtitle_stream_index)
        .query_opt("tag", query.tag)
        .query_opt("transcodeReasons", query.transcode_reasons)
        .query_opt(
            "transcodingMaxAudioChannels",
            query.transcoding_max_audio_channels,
        )
        .query_opt("videoBitRate", query.video_bit_rate)
        .query_opt("videoCodec", query.video_codec)
        .query_opt("videoStreamIndex", query.video_stream_index)
        .query_opt("width", query.width)
        .send_response()
        .await
    }

    #[doc = "Gets a video hls playlist stream\n\nSends a `HEAD` request to `/Videos/{itemId}/master.m3u8`\n\nArguments:\n- `item_id`: The item id.\n- `allow_audio_stream_copy`: Whether or not to allow copying of the audio stream url.\n- `allow_video_stream_copy`: Whether or not to allow copying of the video stream url.\n- `always_burn_in_subtitle_when_transcoding`: Whether to always burn in subtitles when transcoding.\n- `audio_bit_rate`: Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults.\n- `audio_channels`: Optional. Specify a specific number of audio channels to encode to, e.g. 2.\n- `audio_codec`: Optional. Specify an audio codec to encode to, e.g. mp3.\n- `audio_sample_rate`: Optional. Specify a specific audio sample rate, e.g. 44100.\n- `audio_stream_index`: Optional. The index of the audio stream to use. If omitted the first audio stream will be used.\n- `break_on_non_key_frames`: Optional. Whether to break on non key frames.\n- `context`: Optional. The MediaBrowser.Model.Dlna.EncodingContext.\n- `copy_timestamps`: Whether or not to copy timestamps when transcoding with an offset. Defaults to false.\n- `cpu_core_limit`: Optional. The limit of how many cpu cores to use.\n- `de_interlace`: Optional. Whether to deinterlace the video.\n- `device_id`: The device id of the client requesting. Used to stop encoding processes when needed.\n- `device_profile_id`: Optional. The dlna device profile id to utilize.\n- `enable_adaptive_bitrate_streaming`: Enable adaptive bitrate streaming.\n- `enable_audio_vbr_encoding`: Whether to enable Audio Encoding.\n- `enable_auto_stream_copy`: Whether or not to allow automatic stream copy if requested values match the original source. Defaults to true.\n- `enable_mpegts_m2_ts_mode`: Optional. Whether to enable the MpegtsM2Ts mode.\n- `enable_trickplay`: Enable trickplay image playlists being added to master playlist.\n- `framerate`: Optional. A specific video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.\n- `height`: Optional. The fixed vertical resolution of the encoded video.\n- `level`: Optional. Specify a level for the encoder profile (varies by encoder), e.g. 3, 3.1.\n- `live_stream_id`: The live stream id.\n- `max_audio_bit_depth`: Optional. The maximum audio bit depth.\n- `max_audio_channels`: Optional. Specify a maximum number of audio channels to encode to, e.g. 2.\n- `max_framerate`: Optional. A specific maximum video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.\n- `max_height`: Optional. The maximum vertical resolution of the encoded video.\n- `max_ref_frames`: Optional.\n- `max_video_bit_depth`: Optional. The maximum video bit depth.\n- `max_width`: Optional. The maximum horizontal resolution of the encoded video.\n- `media_source_id`: The media version id, if playing an alternate version.\n- `min_segments`: The minimum number of segments.\n- `params`: The streaming parameters.\n- `play_session_id`: The play session id.\n- `profile`: Optional. Specify a specific an encoder profile (varies by encoder), e.g. main, baseline, high.\n- `require_avc`: Optional. Whether to require avc.\n- `require_non_anamorphic`: Optional. Whether to require a non anamorphic stream.\n- `segment_container`: The segment container.\n- `segment_length`: The segment length.\n- `start_time_ticks`: Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms.\n- `static_`: Optional. If true, the original file will be streamed statically without any encoding. Use either no url extension or the original file extension. true/false.\n- `stream_options`: Optional. The streaming options.\n- `subtitle_codec`: Optional. Specify a subtitle codec to encode to.\n- `subtitle_method`: Optional. Specify the subtitle delivery method.\n- `subtitle_stream_index`: Optional. The index of the subtitle stream to use. If omitted no subtitles will be used.\n- `tag`: The tag.\n- `transcode_reasons`: Optional. The transcoding reason.\n- `transcoding_max_audio_channels`: Optional. The maximum number of audio channels to transcode.\n- `video_bit_rate`: Optional. Specify a video bitrate to encode to, e.g. 500000. If omitted this will be left to encoder defaults.\n- `video_codec`: Optional. Specify a video codec to encode to, e.g. h264.\n- `video_stream_index`: Optional. The index of the video stream to use. If omitted the first video stream will be used.\n- `width`: Optional. The fixed horizontal resolution of the encoded video.\n"]
    pub async fn head_master_hls_video_playlist(
        &self,
        item_id: &uuid::Uuid,
        media_source_id: &str,
        query: &query::HeadMasterHlsVideoPlaylist<'_>,
    ) -> Result<reqwest::Response, Error> {
        self.request(
            reqwest::Method::HEAD,
            format!("/Videos/{}/master.m3u8", encode_path(&item_id.to_string())),
        )
        .query_opt("allowAudioStreamCopy", query.allow_audio_stream_copy)
        .query_opt("allowVideoStreamCopy", query.allow_video_stream_copy)
        .query_opt(
            "alwaysBurnInSubtitleWhenTranscoding",
            query.always_burn_in_subtitle_when_transcoding,
        )
        .query_opt("audioBitRate", query.audio_bit_rate)
        .query_opt("audioChannels", query.audio_channels)
        .query_opt("audioCodec", query.audio_codec)
        .query_opt("audioSampleRate", query.audio_sample_rate)
        .query_opt("audioStreamIndex", query.audio_stream_index)
        .query_opt("breakOnNonKeyFrames", query.break_on_non_key_frames)
        .query_opt("context", query.context)
        .query_opt("copyTimestamps", query.copy_timestamps)
        .query_opt("cpuCoreLimit", query.cpu_core_limit)
        .query_opt("deInterlace", query.de_interlace)
        .query_opt("deviceId", query.device_id)
        .query_opt("deviceProfileId", query.device_profile_id)
        .query_opt(
            "enableAdaptiveBitrateStreaming",
            query.enable_adaptive_bitrate_streaming,
        )
        .query_opt("enableAudioVbrEncoding", query.enable_audio_vbr_encoding)
        .query_opt("enableAutoStreamCopy", query.enable_auto_stream_copy)
        .query_opt("enableMpegtsM2TsMode", query.enable_mpegts_m2_ts_mode)
        .query_opt("enableTrickplay", query.enable_trickplay)
        .query_opt("framerate", query.framerate)
        .query_opt("height", query.height)
        .query_opt("level", query.level)
        .query_opt("liveStreamId", query.live_stream_id)
        .query_opt("maxAudioBitDepth", query.max_audio_bit_depth)
        .query_opt("maxAudioChannels", query.max_audio_channels)
        .query_opt("maxFramerate", query.max_framerate)
        .query_opt("maxHeight", query.max_height)
        .query_opt("maxRefFrames", query.max_ref_frames)
        .query_opt("maxVideoBitDepth", query.max_video_bit_depth)
        .query_opt("maxWidth", query.max_width)
        .query("mediaSourceId", media_source_id)
        .query_opt("minSegments", query.min_segments)
        .query_opt("params", query.params)
        .query_opt("playSessionId", query.play_session_id)
        .query_opt("profile", query.profile)
        .query_opt("requireAvc", query.require_avc)
        .query_opt("requireNonAnamorphic", query.require_non_anamorphic)
        .query_opt("segmentContainer", query.segment_container)
        .query_opt("segmentLength", query.segment_length)
        .query_opt("startTimeTicks", query.start_time_ticks)
        .query_opt("static", query.r#static)
        .query_opt(
            "streamOptions",
            query
                .stream_options
                .map(|v| serde_json::to_string(v).unwrap_or_default()),
        )
        .query_opt("subtitleCodec", query.subtitle_codec)
        .query_opt("subtitleMethod", query.subtitle_method)
        .query_opt("subtitleStreamIndex", query.subtitle_stream_index)
        .query_opt("tag", query.tag)
        .query_opt("transcodeReasons", query.transcode_reasons)
        .query_opt(
            "transcodingMaxAudioChannels",
            query.transcoding_max_audio_channels,
        )
        .query_opt("videoBitRate", query.video_bit_rate)
        .query_opt("videoCodec", query.video_codec)
        .query_opt("videoStreamIndex", query.video_stream_index)
        .query_opt("width", query.width)
        .send_response()
        .await
    }

    #[doc = "Gets a hls video segment\n\nSends a `GET` request to `/Videos/{itemId}/hls/{playlistId}/{segmentId}.{segmentContainer}`\n\nArguments:\n- `item_id`: The item id.\n- `playlist_id`: The playlist id.\n- `segment_id`: The segment id.\n- `segment_container`: The segment container.\n"]
    pub async fn get_hls_video_segment_legacy(
        &self,
        item_id: &str,
        playlist_id: &str,
        segment_id: &str,
        segment_container: &str,
    ) -> Result<reqwest::Response, Error> {
        self.request(
            reqwest::Method::GET,
            format!(
                "/Videos/{}/hls/{}/{}.{}",
                encode_path(item_id),
                encode_path(playlist_id),
                encode_path(segment_id),
                encode_path(segment_container)
            ),
        )
        .send_response()
        .await
    }

    #[doc = "Gets a hls video playlist\n\nSends a `GET` request to `/Videos/{itemId}/hls/{playlistId}/stream.m3u8`\n\nArguments:\n- `item_id`: The video id.\n- `playlist_id`: The playlist id.\n"]
    pub async fn get_hls_playlist_legacy(
        &self,
        item_id: &str,
        playlist_id: &str,
    ) -> Result<reqwest::Response, Error> {
        self.request(
            reqwest::Method::GET,
            format!(
                "/Videos/{}/hls/{}/stream.m3u8",
                encode_path(item_id),
                encode_path(playlist_id)
            ),
        )
        .send_response()
        .await
    }

    #[doc = "Stops an active encoding\n\nSends a `DELETE` request to `/Videos/ActiveEncodings`\n\nArguments:\n- `device_id`: The device id of the client requesting. Used to stop encoding processes when needed.\n- `play_session_id`: The play session id.\n"]
    pub async fn stop_encoding_process(
        &self,
        device_id: &str,
        play_session_id: &str,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::DELETE, "/Videos/ActiveEncodings".into())
            .query("deviceId", device_id)
            .query("playSessionId", play_session_id)
            .send_no_content()
            .await
    }

    #[doc = "Gets an HLS subtitle playlist\n\nSends a `GET` request to `/Videos/{itemId}/{mediaSourceId}/Subtitles/{index}/subtitles.m3u8`\n\nArguments:\n- `item_id`: The item id.\n- `media_source_id`: The media source id.\n- `index`: The subtitle stream index.\n- `segment_length`: The subtitle segment length.\n"]
    pub async fn get_subtitle_playlist(
        &self,
        item_id: &uuid::Uuid,
        media_source_id: &str,
        index: i32,
        segment_length: i32,
    ) -> Result<reqwest::Response, Error> {
        self.request(
            reqwest::Method::GET,
            format!(
                "/Videos/{}/{}/Subtitles/{}/subtitles.m3u8",
                encode_path(&item_id.to_string()),
                encode_path(media_source_id),
                encode_path(&index.to_string())
            ),
        )
        .query("segmentLength", segment_length)
        .send_response()
        .await
    }

    #[doc = "Upload an external subtitle file\n\nSends a `POST` request to `/Videos/{itemId}/Subtitles`\n\nArguments:\n- `item_id`: The item the subtitle belongs to.\n- `body`: The request body.\n"]
    pub async fn upload_subtitle(
        &self,
        item_id: &uuid::Uuid,
        body: &types::UploadSubtitleDto,
    ) -> Result<(), Error> {
        self.request(
            reqwest::Method::POST,
            format!("/Videos/{}/Subtitles", encode_path(&item_id.to_string())),
        )
        .json_body(body)
        .send_no_content()
        .await
    }

    #[doc = "Deletes an external subtitle file\n\nSends a `DELETE` request to `/Videos/{itemId}/Subtitles/{index}`\n\nArguments:\n- `item_id`: The item id.\n- `index`: The index of the subtitle file.\n"]
    pub async fn delete_subtitle(&self, item_id: &uuid::Uuid, index: i32) -> Result<(), Error> {
        self.request(
            reqwest::Method::DELETE,
            format!(
                "/Videos/{}/Subtitles/{}",
                encode_path(&item_id.to_string()),
                encode_path(&index.to_string())
            ),
        )
        .send_no_content()
        .await
    }

    #[doc = "Gets subtitles in a specified format\n\nSends a `GET` request to `/Videos/{routeItemId}/{routeMediaSourceId}/Subtitles/{routeIndex}/{routeStartPositionTicks}/Stream.{routeFormat}`\n\nArguments:\n- `route_item_id`: The (route) item id.\n- `route_media_source_id`: The (route) media source id.\n- `route_index`: The (route) subtitle stream index.\n- `route_start_position_ticks`: The (route) start position of the subtitle in ticks.\n- `route_format`: The (route) format of the returned subtitle.\n- `add_vtt_time_map`: Optional. Whether to add a VTT time map.\n- `copy_timestamps`: Optional. Whether to copy the timestamps.\n- `end_position_ticks`: Optional. The end position of the subtitle in ticks.\n- `format`: The format of the returned subtitle.\n- `index`: The subtitle stream index.\n- `item_id`: The item id.\n- `media_source_id`: The media source id.\n- `start_position_ticks`: The start position of the subtitle in ticks.\n"]
    pub async fn get_subtitle_with_ticks(
        &self,
        route_item_id: &uuid::Uuid,
        route_media_source_id: &str,
        route_index: i32,
        route_start_position_ticks: i64,
        route_format: &str,
        query: &query::GetSubtitleWithTicks<'_>,
    ) -> Result<reqwest::Response, Error> {
        self.request(
            reqwest::Method::GET,
            format!(
                "/Videos/{}/{}/Subtitles/{}/{}/Stream.{}",
                encode_path(&route_item_id.to_string()),
                encode_path(route_media_source_id),
                encode_path(&route_index.to_string()),
                encode_path(&route_start_position_ticks.to_string()),
                encode_path(route_format)
            ),
        )
        .query_opt("addVttTimeMap", query.add_vtt_time_map)
        .query_opt("copyTimestamps", query.copy_timestamps)
        .query_opt("endPositionTicks", query.end_position_ticks)
        .query_opt("format", query.format)
        .query_opt("index", query.index)
        .query_opt("itemId", query.item_id)
        .query_opt("mediaSourceId", query.media_source_id)
        .query_opt("startPositionTicks", query.start_position_ticks)
        .send_response()
        .await
    }

    #[doc = "Gets subtitles in a specified format\n\nSends a `GET` request to `/Videos/{routeItemId}/{routeMediaSourceId}/Subtitles/{routeIndex}/Stream.{routeFormat}`\n\nArguments:\n- `route_item_id`: The (route) item id.\n- `route_media_source_id`: The (route) media source id.\n- `route_index`: The (route) subtitle stream index.\n- `route_format`: The (route) format of the returned subtitle.\n- `add_vtt_time_map`: Optional. Whether to add a VTT time map.\n- `copy_timestamps`: Optional. Whether to copy the timestamps.\n- `end_position_ticks`: Optional. The end position of the subtitle in ticks.\n- `format`: The format of the returned subtitle.\n- `index`: The subtitle stream index.\n- `item_id`: The item id.\n- `media_source_id`: The media source id.\n- `start_position_ticks`: The start position of the subtitle in ticks.\n"]
    pub async fn get_subtitle(
        &self,
        route_item_id: &uuid::Uuid,
        route_media_source_id: &str,
        route_index: i32,
        route_format: &str,
        query: &query::GetSubtitle<'_>,
    ) -> Result<reqwest::Response, Error> {
        self.request(
            reqwest::Method::GET,
            format!(
                "/Videos/{}/{}/Subtitles/{}/Stream.{}",
                encode_path(&route_item_id.to_string()),
                encode_path(route_media_source_id),
                encode_path(&route_index.to_string()),
                encode_path(route_format)
            ),
        )
        .query_opt("addVttTimeMap", query.add_vtt_time_map)
        .query_opt("copyTimestamps", query.copy_timestamps)
        .query_opt("endPositionTicks", query.end_position_ticks)
        .query_opt("format", query.format)
        .query_opt("index", query.index)
        .query_opt("itemId", query.item_id)
        .query_opt("mediaSourceId", query.media_source_id)
        .query_opt("startPositionTicks", query.start_position_ticks)
        .send_response()
        .await
    }

    #[doc = "Gets a trickplay tile image\n\nSends a `GET` request to `/Videos/{itemId}/Trickplay/{width}/{index}.jpg`\n\nArguments:\n- `item_id`: The item id.\n- `width`: The width of a single tile.\n- `index`: The index of the desired tile.\n- `media_source_id`: The media version id, if using an alternate version.\n"]
    pub async fn get_trickplay_tile_image(
        &self,
        item_id: &uuid::Uuid,
        width: i32,
        index: i32,
        media_source_id: Option<&uuid::Uuid>,
    ) -> Result<reqwest::Response, Error> {
        self.request(
            reqwest::Method::GET,
            format!(
                "/Videos/{}/Trickplay/{}/{}.jpg",
                encode_path(&item_id.to_string()),
                encode_path(&width.to_string()),
                encode_path(&index.to_string())
            ),
        )
        .query_opt("mediaSourceId", media_source_id)
        .send_response()
        .await
    }

    #[doc = "Gets an image tiles playlist for trickplay\n\nSends a `GET` request to `/Videos/{itemId}/Trickplay/{width}/tiles.m3u8`\n\nArguments:\n- `item_id`: The item id.\n- `width`: The width of a single tile.\n- `media_source_id`: The media version id, if using an alternate version.\n"]
    pub async fn get_trickplay_hls_playlist(
        &self,
        item_id: &uuid::Uuid,
        width: i32,
        media_source_id: Option<&uuid::Uuid>,
    ) -> Result<reqwest::Response, Error> {
        self.request(
            reqwest::Method::GET,
            format!(
                "/Videos/{}/Trickplay/{}/tiles.m3u8",
                encode_path(&item_id.to_string()),
                encode_path(&width.to_string())
            ),
        )
        .query_opt("mediaSourceId", media_source_id)
        .send_response()
        .await
    }

    #[doc = "Get video attachment\n\nSends a `GET` request to `/Videos/{videoId}/{mediaSourceId}/Attachments/{index}`\n\nArguments:\n- `video_id`: Video ID.\n- `media_source_id`: Media Source ID.\n- `index`: Attachment Index.\n"]
    pub async fn get_attachment(
        &self,
        video_id: &uuid::Uuid,
        media_source_id: &str,
        index: i32,
    ) -> Result<reqwest::Response, Error> {
        self.request(
            reqwest::Method::GET,
            format!(
                "/Videos/{}/{}/Attachments/{}",
                encode_path(&video_id.to_string()),
                encode_path(media_source_id),
                encode_path(&index.to_string())
            ),
        )
        .send_response()
        .await
    }

    #[doc = "Gets additional parts for a video\n\nSends a `GET` request to `/Videos/{itemId}/AdditionalParts`\n\nArguments:\n- `item_id`: The item id.\n- `user_id`: Optional. Filter by user id, and attach user data.\n"]
    pub async fn get_additional_part(
        &self,
        item_id: &uuid::Uuid,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(
            reqwest::Method::GET,
            format!(
                "/Videos/{}/AdditionalParts",
                encode_path(&item_id.to_string())
            ),
        )
        .query_opt("userId", user_id)
        .send()
        .await
    }

    #[doc = "Removes alternate video sources\n\nSends a `DELETE` request to `/Videos/{itemId}/AlternateSources`\n\nArguments:\n- `item_id`: The item id.\n"]
    pub async fn delete_alternate_sources(&self, item_id: &uuid::Uuid) -> Result<(), Error> {
        self.request(
            reqwest::Method::DELETE,
            format!(
                "/Videos/{}/AlternateSources",
                encode_path(&item_id.to_string())
            ),
        )
        .send_no_content()
        .await
    }

    #[doc = "Gets a video stream\n\nSends a `GET` request to `/Videos/{itemId}/stream`\n\nArguments:\n- `item_id`: The item id.\n- `allow_audio_stream_copy`: Whether or not to allow copying of the audio stream url.\n- `allow_video_stream_copy`: Whether or not to allow copying of the video stream url.\n- `audio_bit_rate`: Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults.\n- `audio_channels`: Optional. Specify a specific number of audio channels to encode to, e.g. 2.\n- `audio_codec`: Optional. Specify an audio codec to encode to, e.g. mp3. If omitted the server will auto-select using the url's extension.\n- `audio_sample_rate`: Optional. Specify a specific audio sample rate, e.g. 44100.\n- `audio_stream_index`: Optional. The index of the audio stream to use. If omitted the first audio stream will be used.\n- `break_on_non_key_frames`: Optional. Whether to break on non key frames.\n- `container`: The video container. Possible values are: ts, webm, asf, wmv, ogv, mp4, m4v, mkv, mpeg, mpg, avi, 3gp, wmv, wtv, m2ts, mov, iso, flv.\n- `context`: Optional. The MediaBrowser.Model.Dlna.EncodingContext.\n- `copy_timestamps`: Whether or not to copy timestamps when transcoding with an offset. Defaults to false.\n- `cpu_core_limit`: Optional. The limit of how many cpu cores to use.\n- `de_interlace`: Optional. Whether to deinterlace the video.\n- `device_id`: The device id of the client requesting. Used to stop encoding processes when needed.\n- `device_profile_id`: Optional. The dlna device profile id to utilize.\n- `enable_audio_vbr_encoding`: Optional. Whether to enable Audio Encoding.\n- `enable_auto_stream_copy`: Whether or not to allow automatic stream copy if requested values match the original source. Defaults to true.\n- `enable_mpegts_m2_ts_mode`: Optional. Whether to enable the MpegtsM2Ts mode.\n- `framerate`: Optional. A specific video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.\n- `height`: Optional. The fixed vertical resolution of the encoded video.\n- `level`: Optional. Specify a level for the encoder profile (varies by encoder), e.g. 3, 3.1.\n- `live_stream_id`: The live stream id.\n- `max_audio_bit_depth`: Optional. The maximum audio bit depth.\n- `max_audio_channels`: Optional. Specify a maximum number of audio channels to encode to, e.g. 2.\n- `max_framerate`: Optional. A specific maximum video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.\n- `max_height`: Optional. The maximum vertical resolution of the encoded video.\n- `max_ref_frames`: Optional.\n- `max_video_bit_depth`: Optional. The maximum video bit depth.\n- `max_width`: Optional. The maximum horizontal resolution of the encoded video.\n- `media_source_id`: The media version id, if playing an alternate version.\n- `min_segments`: The minimum number of segments.\n- `params`: The streaming parameters.\n- `play_session_id`: The play session id.\n- `profile`: Optional. Specify a specific an encoder profile (varies by encoder), e.g. main, baseline, high.\n- `require_avc`: Optional. Whether to require avc.\n- `require_non_anamorphic`: Optional. Whether to require a non anamorphic stream.\n- `segment_container`: The segment container.\n- `segment_length`: The segment length.\n- `start_time_ticks`: Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms.\n- `static_`: Optional. If true, the original file will be streamed statically without any encoding. Use either no url extension or the original file extension. true/false.\n- `stream_options`: Optional. The streaming options.\n- `subtitle_codec`: Optional. Specify a subtitle codec to encode to.\n- `subtitle_method`: Optional. Specify the subtitle delivery method.\n- `subtitle_stream_index`: Optional. The index of the subtitle stream to use. If omitted no subtitles will be used.\n- `tag`: The tag.\n- `transcode_reasons`: Optional. The transcoding reason.\n- `transcoding_max_audio_channels`: Optional. The maximum number of audio channels to transcode.\n- `video_bit_rate`: Optional. Specify a video bitrate to encode to, e.g. 500000. If omitted this will be left to encoder defaults.\n- `video_codec`: Optional. Specify a video codec to encode to, e.g. h264. If omitted the server will auto-select using the url's extension.\n- `video_stream_index`: Optional. The index of the video stream to use. If omitted the first video stream will be used.\n- `width`: Optional. The fixed horizontal resolution of the encoded video.\n"]
    pub async fn get_video_stream(
        &self,
        item_id: &uuid::Uuid,
        query: &query::GetVideoStream<'_>,
    ) -> Result<reqwest::Response, Error> {
        self.request(
            reqwest::Method::GET,
            format!("/Videos/{}/stream", encode_path(&item_id.to_string())),
        )
        .query_opt("allowAudioStreamCopy", query.allow_audio_stream_copy)
        .query_opt("allowVideoStreamCopy", query.allow_video_stream_copy)
        .query_opt("audioBitRate", query.audio_bit_rate)
        .query_opt("audioChannels", query.audio_channels)
        .query_opt("audioCodec", query.audio_codec)
        .query_opt("audioSampleRate", query.audio_sample_rate)
        .query_opt("audioStreamIndex", query.audio_stream_index)
        .query_opt("breakOnNonKeyFrames", query.break_on_non_key_frames)
        .query_opt("container", query.container)
        .query_opt("context", query.context)
        .query_opt("copyTimestamps", query.copy_timestamps)
        .query_opt("cpuCoreLimit", query.cpu_core_limit)
        .query_opt("deInterlace", query.de_interlace)
        .query_opt("deviceId", query.device_id)
        .query_opt("deviceProfileId", query.device_profile_id)
        .query_opt("enableAudioVbrEncoding", query.enable_audio_vbr_encoding)
        .query_opt("enableAutoStreamCopy", query.enable_auto_stream_copy)
        .query_opt("enableMpegtsM2TsMode", query.enable_mpegts_m2_ts_mode)
        .query_opt("framerate", query.framerate)
        .query_opt("height", query.height)
        .query_opt("level", query.level)
        .query_opt("liveStreamId", query.live_stream_id)
        .query_opt("maxAudioBitDepth", query.max_audio_bit_depth)
        .query_opt("maxAudioChannels", query.max_audio_channels)
        .query_opt("maxFramerate", query.max_framerate)
        .query_opt("maxHeight", query.max_height)
        .query_opt("maxRefFrames", query.max_ref_frames)
        .query_opt("maxVideoBitDepth", query.max_video_bit_depth)
        .query_opt("maxWidth", query.max_width)
        .query_opt("mediaSourceId", query.media_source_id)
        .query_opt("minSegments", query.min_segments)
        .query_opt("params", query.params)
        .query_opt("playSessionId", query.play_session_id)
        .query_opt("profile", query.profile)
        .query_opt("requireAvc", query.require_avc)
        .query_opt("requireNonAnamorphic", query.require_non_anamorphic)
        .query_opt("segmentContainer", query.segment_container)
        .query_opt("segmentLength", query.segment_length)
        .query_opt("startTimeTicks", query.start_time_ticks)
        .query_opt("static", query.r#static)
        .query_opt(
            "streamOptions",
            query
                .stream_options
                .map(|v| serde_json::to_string(v).unwrap_or_default()),
        )
        .query_opt("subtitleCodec", query.subtitle_codec)
        .query_opt("subtitleMethod", query.subtitle_method)
        .query_opt("subtitleStreamIndex", query.subtitle_stream_index)
        .query_opt("tag", query.tag)
        .query_opt("transcodeReasons", query.transcode_reasons)
        .query_opt(
            "transcodingMaxAudioChannels",
            query.transcoding_max_audio_channels,
        )
        .query_opt("videoBitRate", query.video_bit_rate)
        .query_opt("videoCodec", query.video_codec)
        .query_opt("videoStreamIndex", query.video_stream_index)
        .query_opt("width", query.width)
        .send_response()
        .await
    }

    #[doc = "Gets a video stream\n\nSends a `HEAD` request to `/Videos/{itemId}/stream`\n\nArguments:\n- `item_id`: The item id.\n- `allow_audio_stream_copy`: Whether or not to allow copying of the audio stream url.\n- `allow_video_stream_copy`: Whether or not to allow copying of the video stream url.\n- `audio_bit_rate`: Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults.\n- `audio_channels`: Optional. Specify a specific number of audio channels to encode to, e.g. 2.\n- `audio_codec`: Optional. Specify an audio codec to encode to, e.g. mp3. If omitted the server will auto-select using the url's extension.\n- `audio_sample_rate`: Optional. Specify a specific audio sample rate, e.g. 44100.\n- `audio_stream_index`: Optional. The index of the audio stream to use. If omitted the first audio stream will be used.\n- `break_on_non_key_frames`: Optional. Whether to break on non key frames.\n- `container`: The video container. Possible values are: ts, webm, asf, wmv, ogv, mp4, m4v, mkv, mpeg, mpg, avi, 3gp, wmv, wtv, m2ts, mov, iso, flv.\n- `context`: Optional. The MediaBrowser.Model.Dlna.EncodingContext.\n- `copy_timestamps`: Whether or not to copy timestamps when transcoding with an offset. Defaults to false.\n- `cpu_core_limit`: Optional. The limit of how many cpu cores to use.\n- `de_interlace`: Optional. Whether to deinterlace the video.\n- `device_id`: The device id of the client requesting. Used to stop encoding processes when needed.\n- `device_profile_id`: Optional. The dlna device profile id to utilize.\n- `enable_audio_vbr_encoding`: Optional. Whether to enable Audio Encoding.\n- `enable_auto_stream_copy`: Whether or not to allow automatic stream copy if requested values match the original source. Defaults to true.\n- `enable_mpegts_m2_ts_mode`: Optional. Whether to enable the MpegtsM2Ts mode.\n- `framerate`: Optional. A specific video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.\n- `height`: Optional. The fixed vertical resolution of the encoded video.\n- `level`: Optional. Specify a level for the encoder profile (varies by encoder), e.g. 3, 3.1.\n- `live_stream_id`: The live stream id.\n- `max_audio_bit_depth`: Optional. The maximum audio bit depth.\n- `max_audio_channels`: Optional. Specify a maximum number of audio channels to encode to, e.g. 2.\n- `max_framerate`: Optional. A specific maximum video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.\n- `max_height`: Optional. The maximum vertical resolution of the encoded video.\n- `max_ref_frames`: Optional.\n- `max_video_bit_depth`: Optional. The maximum video bit depth.\n- `max_width`: Optional. The maximum horizontal resolution of the encoded video.\n- `media_source_id`: The media version id, if playing an alternate version.\n- `min_segments`: The minimum number of segments.\n- `params`: The streaming parameters.\n- `play_session_id`: The play session id.\n- `profile`: Optional. Specify a specific an encoder profile (varies by encoder), e.g. main, baseline, high.\n- `require_avc`: Optional. Whether to require avc.\n- `require_non_anamorphic`: Optional. Whether to require a non anamorphic stream.\n- `segment_container`: The segment container.\n- `segment_length`: The segment length.\n- `start_time_ticks`: Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms.\n- `static_`: Optional. If true, the original file will be streamed statically without any encoding. Use either no url extension or the original file extension. true/false.\n- `stream_options`: Optional. The streaming options.\n- `subtitle_codec`: Optional. Specify a subtitle codec to encode to.\n- `subtitle_method`: Optional. Specify the subtitle delivery method.\n- `subtitle_stream_index`: Optional. The index of the subtitle stream to use. If omitted no subtitles will be used.\n- `tag`: The tag.\n- `transcode_reasons`: Optional. The transcoding reason.\n- `transcoding_max_audio_channels`: Optional. The maximum number of audio channels to transcode.\n- `video_bit_rate`: Optional. Specify a video bitrate to encode to, e.g. 500000. If omitted this will be left to encoder defaults.\n- `video_codec`: Optional. Specify a video codec to encode to, e.g. h264. If omitted the server will auto-select using the url's extension.\n- `video_stream_index`: Optional. The index of the video stream to use. If omitted the first video stream will be used.\n- `width`: Optional. The fixed horizontal resolution of the encoded video.\n"]
    pub async fn head_video_stream(
        &self,
        item_id: &uuid::Uuid,
        query: &query::HeadVideoStream<'_>,
    ) -> Result<reqwest::Response, Error> {
        self.request(
            reqwest::Method::HEAD,
            format!("/Videos/{}/stream", encode_path(&item_id.to_string())),
        )
        .query_opt("allowAudioStreamCopy", query.allow_audio_stream_copy)
        .query_opt("allowVideoStreamCopy", query.allow_video_stream_copy)
        .query_opt("audioBitRate", query.audio_bit_rate)
        .query_opt("audioChannels", query.audio_channels)
        .query_opt("audioCodec", query.audio_codec)
        .query_opt("audioSampleRate", query.audio_sample_rate)
        .query_opt("audioStreamIndex", query.audio_stream_index)
        .query_opt("breakOnNonKeyFrames", query.break_on_non_key_frames)
        .query_opt("container", query.container)
        .query_opt("context", query.context)
        .query_opt("copyTimestamps", query.copy_timestamps)
        .query_opt("cpuCoreLimit", query.cpu_core_limit)
        .query_opt("deInterlace", query.de_interlace)
        .query_opt("deviceId", query.device_id)
        .query_opt("deviceProfileId", query.device_profile_id)
        .query_opt("enableAudioVbrEncoding", query.enable_audio_vbr_encoding)
        .query_opt("enableAutoStreamCopy", query.enable_auto_stream_copy)
        .query_opt("enableMpegtsM2TsMode", query.enable_mpegts_m2_ts_mode)
        .query_opt("framerate", query.framerate)
        .query_opt("height", query.height)
        .query_opt("level", query.level)
        .query_opt("liveStreamId", query.live_stream_id)
        .query_opt("maxAudioBitDepth", query.max_audio_bit_depth)
        .query_opt("maxAudioChannels", query.max_audio_channels)
        .query_opt("maxFramerate", query.max_framerate)
        .query_opt("maxHeight", query.max_height)
        .query_opt("maxRefFrames", query.max_ref_frames)
        .query_opt("maxVideoBitDepth", query.max_video_bit_depth)
        .query_opt("maxWidth", query.max_width)
        .query_opt("mediaSourceId", query.media_source_id)
        .query_opt("minSegments", query.min_segments)
        .query_opt("params", query.params)
        .query_opt("playSessionId", query.play_session_id)
        .query_opt("profile", query.profile)
        .query_opt("requireAvc", query.require_avc)
        .query_opt("requireNonAnamorphic", query.require_non_anamorphic)
        .query_opt("segmentContainer", query.segment_container)
        .query_opt("segmentLength", query.segment_length)
        .query_opt("startTimeTicks", query.start_time_ticks)
        .query_opt("static", query.r#static)
        .query_opt(
            "streamOptions",
            query
                .stream_options
                .map(|v| serde_json::to_string(v).unwrap_or_default()),
        )
        .query_opt("subtitleCodec", query.subtitle_codec)
        .query_opt("subtitleMethod", query.subtitle_method)
        .query_opt("subtitleStreamIndex", query.subtitle_stream_index)
        .query_opt("tag", query.tag)
        .query_opt("transcodeReasons", query.transcode_reasons)
        .query_opt(
            "transcodingMaxAudioChannels",
            query.transcoding_max_audio_channels,
        )
        .query_opt("videoBitRate", query.video_bit_rate)
        .query_opt("videoCodec", query.video_codec)
        .query_opt("videoStreamIndex", query.video_stream_index)
        .query_opt("width", query.width)
        .send_response()
        .await
    }

    #[doc = "Gets a video stream\n\nSends a `GET` request to `/Videos/{itemId}/stream.{container}`\n\nArguments:\n- `item_id`: The item id.\n- `container`: The video container. Possible values are: ts, webm, asf, wmv, ogv, mp4, m4v, mkv, mpeg, mpg, avi, 3gp, wmv, wtv, m2ts, mov, iso, flv.\n- `allow_audio_stream_copy`: Whether or not to allow copying of the audio stream url.\n- `allow_video_stream_copy`: Whether or not to allow copying of the video stream url.\n- `audio_bit_rate`: Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults.\n- `audio_channels`: Optional. Specify a specific number of audio channels to encode to, e.g. 2.\n- `audio_codec`: Optional. Specify an audio codec to encode to, e.g. mp3. If omitted the server will auto-select using the url's extension.\n- `audio_sample_rate`: Optional. Specify a specific audio sample rate, e.g. 44100.\n- `audio_stream_index`: Optional. The index of the audio stream to use. If omitted the first audio stream will be used.\n- `break_on_non_key_frames`: Optional. Whether to break on non key frames.\n- `context`: Optional. The MediaBrowser.Model.Dlna.EncodingContext.\n- `copy_timestamps`: Whether or not to copy timestamps when transcoding with an offset. Defaults to false.\n- `cpu_core_limit`: Optional. The limit of how many cpu cores to use.\n- `de_interlace`: Optional. Whether to deinterlace the video.\n- `device_id`: The device id of the client requesting. Used to stop encoding processes when needed.\n- `device_profile_id`: Optional. The dlna device profile id to utilize.\n- `enable_audio_vbr_encoding`: Optional. Whether to enable Audio Encoding.\n- `enable_auto_stream_copy`: Whether or not to allow automatic stream copy if requested values match the original source. Defaults to true.\n- `enable_mpegts_m2_ts_mode`: Optional. Whether to enable the MpegtsM2Ts mode.\n- `framerate`: Optional. A specific video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.\n- `height`: Optional. The fixed vertical resolution of the encoded video.\n- `level`: Optional. Specify a level for the encoder profile (varies by encoder), e.g. 3, 3.1.\n- `live_stream_id`: The live stream id.\n- `max_audio_bit_depth`: Optional. The maximum audio bit depth.\n- `max_audio_channels`: Optional. Specify a maximum number of audio channels to encode to, e.g. 2.\n- `max_framerate`: Optional. A specific maximum video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.\n- `max_height`: Optional. The maximum vertical resolution of the encoded video.\n- `max_ref_frames`: Optional.\n- `max_video_bit_depth`: Optional. The maximum video bit depth.\n- `max_width`: Optional. The maximum horizontal resolution of the encoded video.\n- `media_source_id`: The media version id, if playing an alternate version.\n- `min_segments`: The minimum number of segments.\n- `params`: The streaming parameters.\n- `play_session_id`: The play session id.\n- `profile`: Optional. Specify a specific an encoder profile (varies by encoder), e.g. main, baseline, high.\n- `require_avc`: Optional. Whether to require avc.\n- `require_non_anamorphic`: Optional. Whether to require a non anamorphic stream.\n- `segment_container`: The segment container.\n- `segment_length`: The segment length.\n- `start_time_ticks`: Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms.\n- `static_`: Optional. If true, the original file will be streamed statically without any encoding. Use either no url extension or the original file extension. true/false.\n- `stream_options`: Optional. The streaming options.\n- `subtitle_codec`: Optional. Specify a subtitle codec to encode to.\n- `subtitle_method`: Optional. Specify the subtitle delivery method.\n- `subtitle_stream_index`: Optional. The index of the subtitle stream to use. If omitted no subtitles will be used.\n- `tag`: The tag.\n- `transcode_reasons`: Optional. The transcoding reason.\n- `transcoding_max_audio_channels`: Optional. The maximum number of audio channels to transcode.\n- `video_bit_rate`: Optional. Specify a video bitrate to encode to, e.g. 500000. If omitted this will be left to encoder defaults.\n- `video_codec`: Optional. Specify a video codec to encode to, e.g. h264. If omitted the server will auto-select using the url's extension.\n- `video_stream_index`: Optional. The index of the video stream to use. If omitted the first video stream will be used.\n- `width`: Optional. The fixed horizontal resolution of the encoded video.\n"]
    pub async fn get_video_stream_by_container(
        &self,
        item_id: &uuid::Uuid,
        container: &types::GetVideoStreamByContainerContainer,
        query: &query::GetVideoStreamByContainer<'_>,
    ) -> Result<reqwest::Response, Error> {
        self.request(
            reqwest::Method::GET,
            format!(
                "/Videos/{}/stream.{}",
                encode_path(&item_id.to_string()),
                encode_path(&container.to_string())
            ),
        )
        .query_opt("allowAudioStreamCopy", query.allow_audio_stream_copy)
        .query_opt("allowVideoStreamCopy", query.allow_video_stream_copy)
        .query_opt("audioBitRate", query.audio_bit_rate)
        .query_opt("audioChannels", query.audio_channels)
        .query_opt("audioCodec", query.audio_codec)
        .query_opt("audioSampleRate", query.audio_sample_rate)
        .query_opt("audioStreamIndex", query.audio_stream_index)
        .query_opt("breakOnNonKeyFrames", query.break_on_non_key_frames)
        .query_opt("context", query.context)
        .query_opt("copyTimestamps", query.copy_timestamps)
        .query_opt("cpuCoreLimit", query.cpu_core_limit)
        .query_opt("deInterlace", query.de_interlace)
        .query_opt("deviceId", query.device_id)
        .query_opt("deviceProfileId", query.device_profile_id)
        .query_opt("enableAudioVbrEncoding", query.enable_audio_vbr_encoding)
        .query_opt("enableAutoStreamCopy", query.enable_auto_stream_copy)
        .query_opt("enableMpegtsM2TsMode", query.enable_mpegts_m2_ts_mode)
        .query_opt("framerate", query.framerate)
        .query_opt("height", query.height)
        .query_opt("level", query.level)
        .query_opt("liveStreamId", query.live_stream_id)
        .query_opt("maxAudioBitDepth", query.max_audio_bit_depth)
        .query_opt("maxAudioChannels", query.max_audio_channels)
        .query_opt("maxFramerate", query.max_framerate)
        .query_opt("maxHeight", query.max_height)
        .query_opt("maxRefFrames", query.max_ref_frames)
        .query_opt("maxVideoBitDepth", query.max_video_bit_depth)
        .query_opt("maxWidth", query.max_width)
        .query_opt("mediaSourceId", query.media_source_id)
        .query_opt("minSegments", query.min_segments)
        .query_opt("params", query.params)
        .query_opt("playSessionId", query.play_session_id)
        .query_opt("profile", query.profile)
        .query_opt("requireAvc", query.require_avc)
        .query_opt("requireNonAnamorphic", query.require_non_anamorphic)
        .query_opt("segmentContainer", query.segment_container)
        .query_opt("segmentLength", query.segment_length)
        .query_opt("startTimeTicks", query.start_time_ticks)
        .query_opt("static", query.r#static)
        .query_opt(
            "streamOptions",
            query
                .stream_options
                .map(|v| serde_json::to_string(v).unwrap_or_default()),
        )
        .query_opt("subtitleCodec", query.subtitle_codec)
        .query_opt("subtitleMethod", query.subtitle_method)
        .query_opt("subtitleStreamIndex", query.subtitle_stream_index)
        .query_opt("tag", query.tag)
        .query_opt("transcodeReasons", query.transcode_reasons)
        .query_opt(
            "transcodingMaxAudioChannels",
            query.transcoding_max_audio_channels,
        )
        .query_opt("videoBitRate", query.video_bit_rate)
        .query_opt("videoCodec", query.video_codec)
        .query_opt("videoStreamIndex", query.video_stream_index)
        .query_opt("width", query.width)
        .send_response()
        .await
    }

    #[doc = "Gets a video stream\n\nSends a `HEAD` request to `/Videos/{itemId}/stream.{container}`\n\nArguments:\n- `item_id`: The item id.\n- `container`: The video container. Possible values are: ts, webm, asf, wmv, ogv, mp4, m4v, mkv, mpeg, mpg, avi, 3gp, wmv, wtv, m2ts, mov, iso, flv.\n- `allow_audio_stream_copy`: Whether or not to allow copying of the audio stream url.\n- `allow_video_stream_copy`: Whether or not to allow copying of the video stream url.\n- `audio_bit_rate`: Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults.\n- `audio_channels`: Optional. Specify a specific number of audio channels to encode to, e.g. 2.\n- `audio_codec`: Optional. Specify an audio codec to encode to, e.g. mp3. If omitted the server will auto-select using the url's extension.\n- `audio_sample_rate`: Optional. Specify a specific audio sample rate, e.g. 44100.\n- `audio_stream_index`: Optional. The index of the audio stream to use. If omitted the first audio stream will be used.\n- `break_on_non_key_frames`: Optional. Whether to break on non key frames.\n- `context`: Optional. The MediaBrowser.Model.Dlna.EncodingContext.\n- `copy_timestamps`: Whether or not to copy timestamps when transcoding with an offset. Defaults to false.\n- `cpu_core_limit`: Optional. The limit of how many cpu cores to use.\n- `de_interlace`: Optional. Whether to deinterlace the video.\n- `device_id`: The device id of the client requesting. Used to stop encoding processes when needed.\n- `device_profile_id`: Optional. The dlna device profile id to utilize.\n- `enable_audio_vbr_encoding`: Optional. Whether to enable Audio Encoding.\n- `enable_auto_stream_copy`: Whether or not to allow automatic stream copy if requested values match the original source. Defaults to true.\n- `enable_mpegts_m2_ts_mode`: Optional. Whether to enable the MpegtsM2Ts mode.\n- `framerate`: Optional. A specific video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.\n- `height`: Optional. The fixed vertical resolution of the encoded video.\n- `level`: Optional. Specify a level for the encoder profile (varies by encoder), e.g. 3, 3.1.\n- `live_stream_id`: The live stream id.\n- `max_audio_bit_depth`: Optional. The maximum audio bit depth.\n- `max_audio_channels`: Optional. Specify a maximum number of audio channels to encode to, e.g. 2.\n- `max_framerate`: Optional. A specific maximum video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements.\n- `max_height`: Optional. The maximum vertical resolution of the encoded video.\n- `max_ref_frames`: Optional.\n- `max_video_bit_depth`: Optional. The maximum video bit depth.\n- `max_width`: Optional. The maximum horizontal resolution of the encoded video.\n- `media_source_id`: The media version id, if playing an alternate version.\n- `min_segments`: The minimum number of segments.\n- `params`: The streaming parameters.\n- `play_session_id`: The play session id.\n- `profile`: Optional. Specify a specific an encoder profile (varies by encoder), e.g. main, baseline, high.\n- `require_avc`: Optional. Whether to require avc.\n- `require_non_anamorphic`: Optional. Whether to require a non anamorphic stream.\n- `segment_container`: The segment container.\n- `segment_length`: The segment length.\n- `start_time_ticks`: Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms.\n- `static_`: Optional. If true, the original file will be streamed statically without any encoding. Use either no url extension or the original file extension. true/false.\n- `stream_options`: Optional. The streaming options.\n- `subtitle_codec`: Optional. Specify a subtitle codec to encode to.\n- `subtitle_method`: Optional. Specify the subtitle delivery method.\n- `subtitle_stream_index`: Optional. The index of the subtitle stream to use. If omitted no subtitles will be used.\n- `tag`: The tag.\n- `transcode_reasons`: Optional. The transcoding reason.\n- `transcoding_max_audio_channels`: Optional. The maximum number of audio channels to transcode.\n- `video_bit_rate`: Optional. Specify a video bitrate to encode to, e.g. 500000. If omitted this will be left to encoder defaults.\n- `video_codec`: Optional. Specify a video codec to encode to, e.g. h264. If omitted the server will auto-select using the url's extension.\n- `video_stream_index`: Optional. The index of the video stream to use. If omitted the first video stream will be used.\n- `width`: Optional. The fixed horizontal resolution of the encoded video.\n"]
    pub async fn head_video_stream_by_container(
        &self,
        item_id: &uuid::Uuid,
        container: &types::HeadVideoStreamByContainerContainer,
        query: &query::HeadVideoStreamByContainer<'_>,
    ) -> Result<reqwest::Response, Error> {
        self.request(
            reqwest::Method::HEAD,
            format!(
                "/Videos/{}/stream.{}",
                encode_path(&item_id.to_string()),
                encode_path(&container.to_string())
            ),
        )
        .query_opt("allowAudioStreamCopy", query.allow_audio_stream_copy)
        .query_opt("allowVideoStreamCopy", query.allow_video_stream_copy)
        .query_opt("audioBitRate", query.audio_bit_rate)
        .query_opt("audioChannels", query.audio_channels)
        .query_opt("audioCodec", query.audio_codec)
        .query_opt("audioSampleRate", query.audio_sample_rate)
        .query_opt("audioStreamIndex", query.audio_stream_index)
        .query_opt("breakOnNonKeyFrames", query.break_on_non_key_frames)
        .query_opt("context", query.context)
        .query_opt("copyTimestamps", query.copy_timestamps)
        .query_opt("cpuCoreLimit", query.cpu_core_limit)
        .query_opt("deInterlace", query.de_interlace)
        .query_opt("deviceId", query.device_id)
        .query_opt("deviceProfileId", query.device_profile_id)
        .query_opt("enableAudioVbrEncoding", query.enable_audio_vbr_encoding)
        .query_opt("enableAutoStreamCopy", query.enable_auto_stream_copy)
        .query_opt("enableMpegtsM2TsMode", query.enable_mpegts_m2_ts_mode)
        .query_opt("framerate", query.framerate)
        .query_opt("height", query.height)
        .query_opt("level", query.level)
        .query_opt("liveStreamId", query.live_stream_id)
        .query_opt("maxAudioBitDepth", query.max_audio_bit_depth)
        .query_opt("maxAudioChannels", query.max_audio_channels)
        .query_opt("maxFramerate", query.max_framerate)
        .query_opt("maxHeight", query.max_height)
        .query_opt("maxRefFrames", query.max_ref_frames)
        .query_opt("maxVideoBitDepth", query.max_video_bit_depth)
        .query_opt("maxWidth", query.max_width)
        .query_opt("mediaSourceId", query.media_source_id)
        .query_opt("minSegments", query.min_segments)
        .query_opt("params", query.params)
        .query_opt("playSessionId", query.play_session_id)
        .query_opt("profile", query.profile)
        .query_opt("requireAvc", query.require_avc)
        .query_opt("requireNonAnamorphic", query.require_non_anamorphic)
        .query_opt("segmentContainer", query.segment_container)
        .query_opt("segmentLength", query.segment_length)
        .query_opt("startTimeTicks", query.start_time_ticks)
        .query_opt("static", query.r#static)
        .query_opt(
            "streamOptions",
            query
                .stream_options
                .map(|v| serde_json::to_string(v).unwrap_or_default()),
        )
        .query_opt("subtitleCodec", query.subtitle_codec)
        .query_opt("subtitleMethod", query.subtitle_method)
        .query_opt("subtitleStreamIndex", query.subtitle_stream_index)
        .query_opt("tag", query.tag)
        .query_opt("transcodeReasons", query.transcode_reasons)
        .query_opt(
            "transcodingMaxAudioChannels",
            query.transcoding_max_audio_channels,
        )
        .query_opt("videoBitRate", query.video_bit_rate)
        .query_opt("videoCodec", query.video_codec)
        .query_opt("videoStreamIndex", query.video_stream_index)
        .query_opt("width", query.width)
        .send_response()
        .await
    }

    #[doc = "Merges videos into a single record\n\nSends a `POST` request to `/Videos/MergeVersions`\n\nArguments:\n- `ids`: Item id list. This allows multiple, comma delimited.\n"]
    pub async fn merge_versions(&self, ids: &[uuid::Uuid]) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/Videos/MergeVersions".into())
            .query_list("ids", ids)
            .send_no_content()
            .await
    }
}
