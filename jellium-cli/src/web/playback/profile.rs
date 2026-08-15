use jellium_protocol::{AudioCodec, Capabilities, Container, Decoding, VideoCodec};
use jellyfin_api::types::{
    DeviceProfile, DirectPlayProfile, DlnaProfileType, EncodingContext, MediaStreamProtocol,
    SubtitleDeliveryMethod, SubtitleProfile, TranscodeSeekInfo, TranscodingProfile,
};

/// The subtitle formats the Jellyfin server converts to WebVTT, delivered as
/// an external track.
const TEXT_SUBTITLES: &[&str] = &["srt", "subrip", "ass", "ssa", "vtt", "webvtt", "sub", "smi"];

/// The subtitle formats that are pictures, which only burning in delivers.
const BITMAP_SUBTITLES: &[&str] = &["pgs", "pgssub", "dvdsub", "dvbsub", "vobsub", "xsub"];

/// True when `codec` names a subtitle format that is a picture, which only
/// burning in delivers.
pub fn bitmap(codec: &str) -> bool {
    BITMAP_SUBTITLES
        .iter()
        .any(|format| codec.eq_ignore_ascii_case(format))
}

/// The containers the Jellyfin server is asked to transcode into.
const HLS_VIDEO_CONTAINER: &str = "mp4";
const HLS_AUDIO_CONTAINER: &str = "mp4";
const PROGRESSIVE_VIDEO_CONTAINER: &str = "mp4";
const PROGRESSIVE_AUDIO_CONTAINER: &str = "mp3";

impl Named for Container {
    fn named(&self) -> &'static str {
        match self {
            Container::Mp4 => "mp4,m4v",
            Container::WebM => "webm",
            Container::Mpegts => "ts,mpegts",
            Container::Mp3 => "mp3",
            Container::Aac => "aac",
            Container::Flac => "flac",
            Container::Ogg => "ogg,oga,opus",
            Container::Wav => "wav",
        }
    }
}

impl Named for VideoCodec {
    fn named(&self) -> &'static str {
        match self {
            VideoCodec::H264 => "h264",
            VideoCodec::Hevc => "hevc",
            VideoCodec::Vp8 => "vp8",
            VideoCodec::Vp9 => "vp9",
            VideoCodec::Av1 => "av1",
        }
    }
}

impl Named for AudioCodec {
    fn named(&self) -> &'static str {
        match self {
            AudioCodec::Aac => "aac",
            AudioCodec::Mp3 => "mp3",
            AudioCodec::Opus => "opus",
            AudioCodec::Vorbis => "vorbis",
            AudioCodec::Flac => "flac",
            AudioCodec::Ac3 => "ac3",
            AudioCodec::Eac3 => "eac3",
            AudioCodec::Alac => "alac",
            AudioCodec::Pcm => "pcm",
        }
    }
}

/// The spelling the Jellyfin server knows a probed capability by.
trait Named {
    fn named(&self) -> &'static str;
}

/// True for a container that carries video.
fn visual(container: &Container) -> bool {
    matches!(
        container,
        Container::Mp4 | Container::WebM | Container::Mpegts
    )
}

fn joined<T: Named>(items: &[T]) -> String {
    items.iter().map(Named::named).collect::<Vec<_>>().join(",")
}

fn direct_play(decoding: &Decoding) -> Vec<DirectPlayProfile> {
    let audio = joined(&decoding.audio_codecs);
    let mut profiles = Vec::new();
    for container in &decoding.containers {
        if visual(container) {
            for codec in &decoding.video_codecs {
                profiles.push(DirectPlayProfile {
                    container: Some(container.named().to_string()),
                    video_codec: Some(codec.named().to_string()),
                    audio_codec: (!audio.is_empty()).then(|| audio.clone()),
                    type_: Some(DlnaProfileType::Video),
                });
            }
        }
        for codec in &decoding.audio_codecs {
            profiles.push(DirectPlayProfile {
                container: Some(container.named().to_string()),
                video_codec: None,
                audio_codec: Some(codec.named().to_string()),
                type_: Some(DlnaProfileType::Audio),
            });
        }
    }
    profiles
}

fn transcoding(capabilities: &Capabilities) -> Vec<TranscodingProfile> {
    let decoding = if capabilities.media_source {
        &capabilities.adaptive
    } else {
        &capabilities.direct
    };
    let video = joined(&decoding.video_codecs);
    let audio = joined(&decoding.audio_codecs);
    if capabilities.media_source {
        vec![
            TranscodingProfile {
                container: Some(HLS_VIDEO_CONTAINER.to_string()),
                type_: Some(DlnaProfileType::Video),
                video_codec: Some(video),
                audio_codec: Some(audio.clone()),
                protocol: Some(MediaStreamProtocol::Hls),
                context: Some(EncodingContext::Streaming),
                max_audio_channels: Some("2".to_string()),
                min_segments: 1,
                break_on_non_key_frames: true,
                transcode_seek_info: Some(TranscodeSeekInfo::Auto),
                ..TranscodingProfile::default()
            },
            TranscodingProfile {
                container: Some(HLS_AUDIO_CONTAINER.to_string()),
                type_: Some(DlnaProfileType::Audio),
                audio_codec: Some(audio),
                protocol: Some(MediaStreamProtocol::Hls),
                context: Some(EncodingContext::Streaming),
                max_audio_channels: Some("2".to_string()),
                min_segments: 1,
                transcode_seek_info: Some(TranscodeSeekInfo::Auto),
                ..TranscodingProfile::default()
            },
        ]
    } else {
        vec![
            TranscodingProfile {
                container: Some(PROGRESSIVE_VIDEO_CONTAINER.to_string()),
                type_: Some(DlnaProfileType::Video),
                video_codec: Some(video),
                audio_codec: Some(audio),
                protocol: Some(MediaStreamProtocol::Http),
                context: Some(EncodingContext::Streaming),
                max_audio_channels: Some("2".to_string()),
                transcode_seek_info: Some(TranscodeSeekInfo::Auto),
                ..TranscodingProfile::default()
            },
            TranscodingProfile {
                container: Some(PROGRESSIVE_AUDIO_CONTAINER.to_string()),
                type_: Some(DlnaProfileType::Audio),
                audio_codec: Some(PROGRESSIVE_AUDIO_CONTAINER.to_string()),
                protocol: Some(MediaStreamProtocol::Http),
                context: Some(EncodingContext::Streaming),
                max_audio_channels: Some("2".to_string()),
                transcode_seek_info: Some(TranscodeSeekInfo::Auto),
                ..TranscodingProfile::default()
            },
        ]
    }
}

fn subtitles() -> Vec<SubtitleProfile> {
    TEXT_SUBTITLES
        .iter()
        .map(|format| SubtitleProfile {
            format: Some((*format).to_string()),
            method: Some(SubtitleDeliveryMethod::External),
            ..SubtitleProfile::default()
        })
        .chain(BITMAP_SUBTITLES.iter().map(|format| SubtitleProfile {
            format: Some((*format).to_string()),
            method: Some(SubtitleDeliveryMethod::Encode),
            ..SubtitleProfile::default()
        }))
        .collect()
}

/// The device profile a browser reporting `capabilities` gets: one direct-play
/// profile per container and codec pair the media element decodes, HLS
/// transcoding profiles built from what Media Source Extensions accept when
/// they are present and progressive ones built from the media element when
/// they are not, text subtitle profiles delivered as external WebVTT, and
/// bitmap subtitle profiles delivered by encoding.
/// `ceiling` caps the streaming and static bitrates when it is present.
pub fn build(capabilities: &Capabilities, ceiling: Option<i32>) -> DeviceProfile {
    DeviceProfile {
        name: Some("Jellium Web".to_string()),
        max_streaming_bitrate: ceiling,
        max_static_bitrate: ceiling,
        direct_play_profiles: direct_play(&capabilities.direct),
        transcoding_profiles: transcoding(capabilities),
        subtitle_profiles: subtitles(),
        ..DeviceProfile::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decoding() -> Decoding {
        Decoding {
            containers: vec![Container::Mp4, Container::Mp3],
            video_codecs: vec![VideoCodec::H264],
            audio_codecs: vec![AudioCodec::Aac, AudioCodec::Mp3],
        }
    }

    fn capabilities(media_source: bool) -> Capabilities {
        Capabilities {
            media_source,
            direct: decoding(),
            adaptive: if media_source {
                decoding()
            } else {
                Decoding::default()
            },
        }
    }

    #[test]
    fn a_codec_the_browser_did_not_report_is_absent_from_the_profile() {
        let profile = build(&capabilities(true), None);
        let named = serde_json::to_string(&profile).expect("serialized");
        assert!(named.contains("h264"));
        assert!(!named.contains("hevc"));
        assert!(!named.contains("av1"));
    }

    #[test]
    fn a_browser_with_media_source_extensions_transcodes_over_hls() {
        let profile = build(&capabilities(true), None);
        assert!(
            profile
                .transcoding_profiles
                .iter()
                .all(|transcoding| transcoding.protocol == Some(MediaStreamProtocol::Hls))
        );
    }

    #[test]
    fn a_browser_without_media_source_extensions_transcodes_progressively() {
        let profile = build(&capabilities(false), None);
        assert!(
            profile
                .transcoding_profiles
                .iter()
                .all(|transcoding| transcoding.protocol == Some(MediaStreamProtocol::Http))
        );
    }

    #[test]
    fn a_container_the_element_decodes_and_media_source_refuses_is_direct_played() {
        let capabilities = Capabilities {
            media_source: true,
            direct: Decoding {
                containers: vec![Container::Flac],
                video_codecs: Vec::new(),
                audio_codecs: vec![AudioCodec::Flac],
            },
            adaptive: Decoding {
                containers: vec![Container::Mp4],
                video_codecs: vec![VideoCodec::H264],
                audio_codecs: vec![AudioCodec::Aac],
            },
        };
        let profile = build(&capabilities, None);
        assert!(
            profile
                .direct_play_profiles
                .iter()
                .any(|direct| direct.container.as_deref() == Some("flac"))
        );
    }

    #[test]
    fn a_transcode_targets_only_what_the_delivery_path_accepts() {
        let capabilities = Capabilities {
            media_source: true,
            direct: Decoding {
                containers: vec![Container::Flac],
                video_codecs: Vec::new(),
                audio_codecs: vec![AudioCodec::Flac],
            },
            adaptive: Decoding {
                containers: vec![Container::Mp4],
                video_codecs: vec![VideoCodec::H264],
                audio_codecs: vec![AudioCodec::Aac],
            },
        };
        let profile = build(&capabilities, None);
        let named = serde_json::to_string(&profile.transcoding_profiles).expect("serialized");
        assert!(named.contains("aac"));
        assert!(!named.contains("flac"));
    }

    #[test]
    fn a_ceiling_caps_the_streaming_and_static_bitrates() {
        let profile = build(&capabilities(true), Some(4_000_000));
        assert_eq!(profile.max_streaming_bitrate, Some(4_000_000));
        assert_eq!(profile.max_static_bitrate, Some(4_000_000));
    }

    #[test]
    fn a_text_subtitle_is_external_and_a_bitmap_subtitle_is_encoded() {
        let profile = build(&capabilities(true), None);
        let method = |format: &str| {
            profile
                .subtitle_profiles
                .iter()
                .find(|subtitle| subtitle.format.as_deref() == Some(format))
                .and_then(|subtitle| subtitle.method)
        };
        assert_eq!(method("srt"), Some(SubtitleDeliveryMethod::External));
        assert_eq!(method("ass"), Some(SubtitleDeliveryMethod::External));
        assert_eq!(method("pgssub"), Some(SubtitleDeliveryMethod::Encode));
        assert_eq!(method("vobsub"), Some(SubtitleDeliveryMethod::Encode));
    }
}
