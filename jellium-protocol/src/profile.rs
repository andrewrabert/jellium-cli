//! The device profile the browser builds and the relay posts, declared so that
//! serializing it emits jellyfin-web's key order and value types byte for byte.

use crate::Bitrate;
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

// reference: device-profile-object
// browserDeviceProfile.js:504-509, :834, :938, :953, :1568, :1598
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeviceProfile {
    pub max_streaming_bitrate: Bitrate,
    pub max_static_bitrate: Bitrate,
    pub music_streaming_transcoding_bitrate: Bitrate,
    pub direct_play_profiles: Vec<DirectPlayProfile>,
    pub transcoding_profiles: Vec<TranscodingProfile>,
    pub container_profiles: Vec<ContainerProfile>,
    pub codec_profiles: Vec<CodecProfile>,
    pub subtitle_profiles: Vec<SubtitleProfile>,
    pub response_profiles: Vec<ResponseProfile>,
}

impl Default for DeviceProfile {
    /// The profile a browser that probed nothing offers: every list empty and
    /// the three rates `browserDeviceProfile.js` starts from.
    fn default() -> DeviceProfile {
        DeviceProfile {
            max_streaming_bitrate: Bitrate::of(120_000_000),
            max_static_bitrate: Bitrate::of(100_000_000),
            music_streaming_transcoding_bitrate: Bitrate::of(384_000),
            direct_play_profiles: Vec::new(),
            transcoding_profiles: Vec::new(),
            container_profiles: Vec::new(),
            codec_profiles: Vec::new(),
            subtitle_profiles: Vec::new(),
            response_profiles: Vec::new(),
        }
    }
}

/// Video emits Container, Type, VideoCodec, AudioCodec; audio emits Container,
/// AudioCodec when present, Type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DirectPlayProfile {
    Video {
        container: String,
        video_codec: String,
        audio_codec: String,
    },
    Audio {
        container: String,
        audio_codec: Option<String>,
    },
}

/// Emits Container, Type, AudioCodec, VideoCodec, Context, Protocol,
/// MaxAudioChannels, MinSegments, BreakOnNonKeyFrames, EnableAudioVbrEncoding,
/// SegmentLength, ApplyConditions, Conditions, dropping every absent optional
/// and every empty condition list.
///
/// ApplyConditions precedes Conditions: browserDeviceProfile.js:1057 and :1099
/// append the first to a spread copy that carries neither, and apphost.js:81
/// appends the second after it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TranscodingProfile {
    pub container: String,
    #[serde(rename = "Type")]
    pub kind: MediaKind,
    pub audio_codec: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video_codec: Option<String>,
    pub context: Context,
    pub protocol: Protocol,
    pub max_audio_channels: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_segments: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub break_on_non_key_frames: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_audio_vbr_encoding: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segment_length: Option<i64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub apply_conditions: Vec<Condition>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<Condition>,
}

/// Emits Type, Conditions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerProfile {
    #[serde(rename = "Type")]
    pub kind: MediaKind,
    pub conditions: Vec<Condition>,
}

/// Any emits Type, Conditions; Codec emits Type, Codec, Conditions; Contained
/// emits Type, Codec, Container, Conditions; Barred emits Type, Container,
/// Codec, Conditions; SubContained emits Type, Container, SubContainer, Codec,
/// Conditions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodecProfile {
    Any {
        kind: CodecKind,
        conditions: Vec<Condition>,
    },
    Codec {
        kind: CodecKind,
        codec: String,
        conditions: Vec<Condition>,
    },
    Contained {
        kind: CodecKind,
        codec: String,
        container: String,
        conditions: Vec<Condition>,
    },
    /// `container` carries Jellyfin's leading `-` negation, which is how
    /// browserDeviceProfile.js:1507-1509 bars Dolby Vision outside ts and mp4.
    Barred {
        kind: CodecKind,
        container: String,
        codec: String,
        conditions: Vec<Condition>,
    },
    SubContained {
        kind: CodecKind,
        container: String,
        sub_container: String,
        codec: String,
        conditions: Vec<Condition>,
    },
}

/// Emits Condition, Property, Value, IsRequired when present.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Condition {
    #[serde(rename = "Condition")]
    pub comparison: Comparison,
    pub property: Property,
    pub value: String,
    #[serde(
        rename = "IsRequired",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub required: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Comparison {
    Equals,
    NotEquals,
    EqualsAny,
    LessThanEqual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Property {
    AudioChannels,
    AudioProfile,
    IsAnamorphic,
    IsInterlaced,
    IsSecondaryAudio,
    NumStreams,
    VideoBitrate,
    VideoCodecTag,
    VideoFramerate,
    VideoLevel,
    VideoProfile,
    VideoRangeType,
    Width,
}

/// Emits Format, Method.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SubtitleProfile {
    pub format: String,
    pub method: SubtitleMethod,
}

/// External is the only method this client sends.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SubtitleMethod {
    External,
}

/// Emits Type, Container, MimeType.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseProfile {
    #[serde(rename = "Type")]
    pub kind: MediaKind,
    pub container: String,
    pub mime_type: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MediaKind {
    Audio,
    Video,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum CodecKind {
    Audio,
    Video,
    VideoAudio,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Context {
    Streaming,
    Static,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    Hls,
    Http,
}

const CONTAINER: &str = "Container";
const TYPE: &str = "Type";
const VIDEO_CODEC: &str = "VideoCodec";
const AUDIO_CODEC: &str = "AudioCodec";
const CODEC: &str = "Codec";
const SUB_CONTAINER: &str = "SubContainer";
const CONDITIONS: &str = "Conditions";

impl Serialize for DirectPlayProfile {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            DirectPlayProfile::Video {
                container,
                video_codec,
                audio_codec,
            } => {
                let mut map = serializer.serialize_map(Some(4))?;
                map.serialize_entry(CONTAINER, container)?;
                map.serialize_entry(TYPE, &MediaKind::Video)?;
                map.serialize_entry(VIDEO_CODEC, video_codec)?;
                map.serialize_entry(AUDIO_CODEC, audio_codec)?;
                map.end()
            }
            DirectPlayProfile::Audio {
                container,
                audio_codec,
            } => {
                let mut map =
                    serializer.serialize_map(Some(2 + usize::from(audio_codec.is_some())))?;
                map.serialize_entry(CONTAINER, container)?;
                if let Some(audio_codec) = audio_codec {
                    map.serialize_entry(AUDIO_CODEC, audio_codec)?;
                }
                map.serialize_entry(TYPE, &MediaKind::Audio)?;
                map.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for DirectPlayProfile {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(DirectPlayProfileVisitor)
    }
}

struct DirectPlayProfileVisitor;

impl<'de> Visitor<'de> for DirectPlayProfileVisitor {
    type Value = DirectPlayProfile;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a direct play profile")
    }

    fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<DirectPlayProfile, M::Error> {
        let mut container = None;
        let mut kind = None;
        let mut video_codec = None;
        let mut audio_codec = None;
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                CONTAINER => container = Some(map.next_value::<String>()?),
                TYPE => kind = Some(map.next_value::<MediaKind>()?),
                VIDEO_CODEC => video_codec = Some(map.next_value::<String>()?),
                AUDIO_CODEC => audio_codec = Some(map.next_value::<String>()?),
                other => {
                    return Err(serde::de::Error::unknown_field(
                        other,
                        &[CONTAINER, TYPE, VIDEO_CODEC, AUDIO_CODEC],
                    ));
                }
            }
        }
        let container = container.ok_or_else(|| serde::de::Error::missing_field(CONTAINER))?;
        match kind.ok_or_else(|| serde::de::Error::missing_field(TYPE))? {
            MediaKind::Video => Ok(DirectPlayProfile::Video {
                container,
                video_codec: video_codec
                    .ok_or_else(|| serde::de::Error::missing_field(VIDEO_CODEC))?,
                audio_codec: audio_codec
                    .ok_or_else(|| serde::de::Error::missing_field(AUDIO_CODEC))?,
            }),
            MediaKind::Audio => match video_codec {
                Some(_) => Err(serde::de::Error::unknown_field(
                    VIDEO_CODEC,
                    &[CONTAINER, AUDIO_CODEC, TYPE],
                )),
                None => Ok(DirectPlayProfile::Audio {
                    container,
                    audio_codec,
                }),
            },
        }
    }
}

impl Serialize for CodecProfile {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            CodecProfile::Any { kind, conditions } => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry(TYPE, kind)?;
                map.serialize_entry(CONDITIONS, conditions)?;
                map.end()
            }
            CodecProfile::Codec {
                kind,
                codec,
                conditions,
            } => {
                let mut map = serializer.serialize_map(Some(3))?;
                map.serialize_entry(TYPE, kind)?;
                map.serialize_entry(CODEC, codec)?;
                map.serialize_entry(CONDITIONS, conditions)?;
                map.end()
            }
            CodecProfile::Contained {
                kind,
                codec,
                container,
                conditions,
            } => {
                let mut map = serializer.serialize_map(Some(4))?;
                map.serialize_entry(TYPE, kind)?;
                map.serialize_entry(CODEC, codec)?;
                map.serialize_entry(CONTAINER, container)?;
                map.serialize_entry(CONDITIONS, conditions)?;
                map.end()
            }
            CodecProfile::Barred {
                kind,
                container,
                codec,
                conditions,
            } => {
                let mut map = serializer.serialize_map(Some(4))?;
                map.serialize_entry(TYPE, kind)?;
                map.serialize_entry(CONTAINER, container)?;
                map.serialize_entry(CODEC, codec)?;
                map.serialize_entry(CONDITIONS, conditions)?;
                map.end()
            }
            CodecProfile::SubContained {
                kind,
                container,
                sub_container,
                codec,
                conditions,
            } => {
                let mut map = serializer.serialize_map(Some(5))?;
                map.serialize_entry(TYPE, kind)?;
                map.serialize_entry(CONTAINER, container)?;
                map.serialize_entry(SUB_CONTAINER, sub_container)?;
                map.serialize_entry(CODEC, codec)?;
                map.serialize_entry(CONDITIONS, conditions)?;
                map.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for CodecProfile {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(CodecProfileVisitor)
    }
}

struct CodecProfileVisitor;

/// A value and the position its key held in the map, which is what separates
/// `Contained` from `Barred`: they carry the same keys in opposite order.
struct Placed<T> {
    value: T,
    place: usize,
}

impl<'de> Visitor<'de> for CodecProfileVisitor {
    type Value = CodecProfile;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a codec profile")
    }

    fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<CodecProfile, M::Error> {
        const KNOWN: [&str; 5] = [TYPE, CODEC, CONTAINER, SUB_CONTAINER, CONDITIONS];
        let mut kind = None;
        let mut codec: Option<Placed<String>> = None;
        let mut container: Option<Placed<String>> = None;
        let mut sub_container = None;
        let mut conditions = None;
        let mut place = 0;
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                TYPE => kind = Some(map.next_value::<CodecKind>()?),
                CODEC => {
                    codec = Some(Placed {
                        value: map.next_value::<String>()?,
                        place,
                    });
                }
                CONTAINER => {
                    container = Some(Placed {
                        value: map.next_value::<String>()?,
                        place,
                    });
                }
                SUB_CONTAINER => sub_container = Some(map.next_value::<String>()?),
                CONDITIONS => conditions = Some(map.next_value::<Vec<Condition>>()?),
                other => return Err(serde::de::Error::unknown_field(other, &KNOWN)),
            }
            place += 1;
        }
        let kind = kind.ok_or_else(|| serde::de::Error::missing_field(TYPE))?;
        let conditions = conditions.ok_or_else(|| serde::de::Error::missing_field(CONDITIONS))?;
        match (codec, container, sub_container) {
            (Some(codec), Some(container), Some(sub_container)) => Ok(CodecProfile::SubContained {
                kind,
                container: container.value,
                sub_container,
                codec: codec.value,
                conditions,
            }),
            (Some(codec), Some(container), None) if codec.place < container.place => {
                Ok(CodecProfile::Contained {
                    kind,
                    codec: codec.value,
                    container: container.value,
                    conditions,
                })
            }
            (Some(codec), Some(container), None) => Ok(CodecProfile::Barred {
                kind,
                container: container.value,
                codec: codec.value,
                conditions,
            }),
            (Some(codec), None, None) => Ok(CodecProfile::Codec {
                kind,
                codec: codec.value,
                conditions,
            }),
            (None, None, None) => Ok(CodecProfile::Any { kind, conditions }),
            _ => Err(serde::de::Error::missing_field(CODEC)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn condition(property: Property, value: &str) -> Condition {
        Condition {
            comparison: Comparison::LessThanEqual,
            property,
            value: value.to_owned(),
            required: Some(false),
        }
    }

    fn profile() -> DeviceProfile {
        DeviceProfile {
            max_streaming_bitrate: Bitrate::of(120_000_000),
            max_static_bitrate: Bitrate::of(100_000_000),
            music_streaming_transcoding_bitrate: Bitrate::of(384_000),
            direct_play_profiles: vec![
                DirectPlayProfile::Video {
                    container: "mp4,m4v".to_owned(),
                    video_codec: "h264".to_owned(),
                    audio_codec: "aac".to_owned(),
                },
                DirectPlayProfile::Audio {
                    container: "webm".to_owned(),
                    audio_codec: Some("opus".to_owned()),
                },
                DirectPlayProfile::Audio {
                    container: "flac".to_owned(),
                    audio_codec: None,
                },
            ],
            transcoding_profiles: vec![
                TranscodingProfile {
                    container: "mp4".to_owned(),
                    kind: MediaKind::Video,
                    audio_codec: "flac".to_owned(),
                    video_codec: Some("h264".to_owned()),
                    context: Context::Streaming,
                    protocol: Protocol::Hls,
                    max_audio_channels: "2".to_owned(),
                    min_segments: Some("1".to_owned()),
                    break_on_non_key_frames: Some(true),
                    enable_audio_vbr_encoding: None,
                    segment_length: Some(1),
                    apply_conditions: vec![condition(Property::AudioChannels, "2")],
                    conditions: vec![condition(Property::Width, "1920")],
                },
                TranscodingProfile {
                    container: "aac".to_owned(),
                    kind: MediaKind::Audio,
                    audio_codec: "aac".to_owned(),
                    video_codec: None,
                    context: Context::Static,
                    protocol: Protocol::Http,
                    max_audio_channels: "6".to_owned(),
                    min_segments: None,
                    break_on_non_key_frames: None,
                    enable_audio_vbr_encoding: None,
                    segment_length: None,
                    apply_conditions: Vec::new(),
                    conditions: Vec::new(),
                },
            ],
            container_profiles: vec![ContainerProfile {
                kind: MediaKind::Video,
                conditions: vec![condition(Property::NumStreams, "32")],
            }],
            codec_profiles: vec![
                CodecProfile::Any {
                    kind: CodecKind::Video,
                    conditions: vec![condition(Property::VideoBitrate, "120000000")],
                },
                CodecProfile::Codec {
                    kind: CodecKind::VideoAudio,
                    codec: "flac".to_owned(),
                    conditions: vec![condition(Property::AudioChannels, "2")],
                },
                CodecProfile::Contained {
                    kind: CodecKind::Video,
                    codec: "h264".to_owned(),
                    container: "ts".to_owned(),
                    conditions: vec![condition(Property::VideoLevel, "42")],
                },
                CodecProfile::Barred {
                    kind: CodecKind::Video,
                    container: "-mp4,ts".to_owned(),
                    codec: "hevc".to_owned(),
                    conditions: vec![Condition {
                        comparison: Comparison::EqualsAny,
                        property: Property::VideoRangeType,
                        value: "SDR|HDR10".to_owned(),
                        required: Some(false),
                    }],
                },
                CodecProfile::SubContained {
                    kind: CodecKind::Video,
                    container: "hls".to_owned(),
                    sub_container: "mp4".to_owned(),
                    codec: "h264".to_owned(),
                    conditions: vec![Condition {
                        comparison: Comparison::EqualsAny,
                        property: Property::VideoProfile,
                        value: "high|high 10".to_owned(),
                        required: None,
                    }],
                },
            ],
            subtitle_profiles: vec![SubtitleProfile {
                format: "vtt".to_owned(),
                method: SubtitleMethod::External,
            }],
            response_profiles: vec![ResponseProfile {
                kind: MediaKind::Video,
                container: "m4v".to_owned(),
                mime_type: "video/mp4".to_owned(),
            }],
        }
    }

    #[test]
    fn a_device_profile_serializes_in_jellyfin_webs_key_order() {
        let rendered = serde_json::to_string(&profile()).expect("the profile renders");
        assert_eq!(
            rendered,
            concat!(
                r#"{"MaxStreamingBitrate":120000000,"MaxStaticBitrate":100000000,"#,
                r#""MusicStreamingTranscodingBitrate":384000,"DirectPlayProfiles":["#,
                r#"{"Container":"mp4,m4v","Type":"Video","VideoCodec":"h264","AudioCodec":"aac"},"#,
                r#"{"Container":"webm","AudioCodec":"opus","Type":"Audio"},"#,
                r#"{"Container":"flac","Type":"Audio"}],"#,
                r#""TranscodingProfiles":[{"Container":"mp4","Type":"Video","AudioCodec":"flac","#,
                r#""VideoCodec":"h264","Context":"Streaming","Protocol":"hls","#,
                r#""MaxAudioChannels":"2","MinSegments":"1","BreakOnNonKeyFrames":true,"#,
                r#""SegmentLength":1,"ApplyConditions":[{"Condition":"LessThanEqual","#,
                r#""Property":"AudioChannels","Value":"2","IsRequired":false}],"#,
                r#""Conditions":[{"Condition":"LessThanEqual","Property":"Width","#,
                r#""Value":"1920","IsRequired":false}]},"#,
                r#"{"Container":"aac","Type":"Audio","AudioCodec":"aac","Context":"Static","#,
                r#""Protocol":"http","MaxAudioChannels":"6"}],"#,
                r#""ContainerProfiles":[{"Type":"Video","Conditions":["#,
                r#"{"Condition":"LessThanEqual","Property":"NumStreams","Value":"32","#,
                r#""IsRequired":false}]}],"#,
                r#""CodecProfiles":[{"Type":"Video","Conditions":[{"Condition":"LessThanEqual","#,
                r#""Property":"VideoBitrate","Value":"120000000","IsRequired":false}]},"#,
                r#"{"Type":"VideoAudio","Codec":"flac","Conditions":["#,
                r#"{"Condition":"LessThanEqual","Property":"AudioChannels","Value":"2","#,
                r#""IsRequired":false}]},"#,
                r#"{"Type":"Video","Codec":"h264","Container":"ts","Conditions":["#,
                r#"{"Condition":"LessThanEqual","Property":"VideoLevel","Value":"42","#,
                r#""IsRequired":false}]},"#,
                r#"{"Type":"Video","Container":"-mp4,ts","Codec":"hevc","Conditions":["#,
                r#"{"Condition":"EqualsAny","Property":"VideoRangeType","Value":"SDR|HDR10","#,
                r#""IsRequired":false}]},"#,
                r#"{"Type":"Video","Container":"hls","SubContainer":"mp4","Codec":"h264","#,
                r#""Conditions":[{"Condition":"EqualsAny","Property":"VideoProfile","#,
                r#""Value":"high|high 10"}]}],"#,
                r#""SubtitleProfiles":[{"Format":"vtt","Method":"External"}],"#,
                r#""ResponseProfiles":[{"Type":"Video","Container":"m4v","#,
                r#""MimeType":"video/mp4"}]}"#,
            )
        );
    }

    #[test]
    fn a_transcoding_profile_carrying_both_lists_emits_apply_conditions_first() {
        let rendered = serde_json::to_string(&profile()).expect("the profile renders");
        let applied = rendered.find("\"ApplyConditions\"").expect("it applies");
        let held = rendered.find("\"Conditions\"").expect("it conditions");
        assert!(applied < held);
    }

    #[test]
    fn a_serialized_device_profile_reads_back_as_the_same_value() {
        let rendered = serde_json::to_string(&profile()).expect("the profile renders");
        let read: DeviceProfile = serde_json::from_str(&rendered).expect("the profile reads");
        assert_eq!(read, profile());
    }
}
