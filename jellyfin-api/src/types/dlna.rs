use super::*;

#[doc = "Defines the MediaBrowser.Model.Dlna.CodecProfile."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct CodecProfile {
    #[doc = "Gets or sets the list of MediaBrowser.Model.Dlna.ProfileCondition to apply if this profile is met."]
    #[serde(
        rename = "ApplyConditions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub apply_conditions: Vec<ProfileCondition>,
    #[doc = "Gets or sets the codec(s) that this profile applies to."]
    #[serde(
        rename = "Codec",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub codec: Option<String>,
    #[doc = "Gets or sets the list of MediaBrowser.Model.Dlna.ProfileCondition which this profile must meet."]
    #[serde(
        rename = "Conditions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub conditions: Vec<ProfileCondition>,
    #[doc = "Gets or sets the container(s) which this profile will be applied to."]
    #[serde(
        rename = "Container",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub container: Option<String>,
    #[doc = "Gets or sets the sub-container(s) which this profile will be applied to."]
    #[serde(
        rename = "SubContainer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sub_container: Option<String>,
    #[serde(
        rename = "Type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_: Option<CodecType>,
}

impl Default for CodecProfile {
    fn default() -> Self {
        Self {
            apply_conditions: Default::default(),
            codec: Default::default(),
            conditions: Default::default(),
            container: Default::default(),
            sub_container: Default::default(),
            type_: Default::default(),
        }
    }
}

#[doc = "Defines the MediaBrowser.Model.Dlna.ContainerProfile."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ContainerProfile {
    #[doc = "Gets or sets the list of MediaBrowser.Model.Dlna.ProfileCondition which this container will be applied to."]
    #[serde(
        rename = "Conditions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub conditions: Vec<ProfileCondition>,
    #[doc = "Gets or sets the container(s) which this container must meet."]
    #[serde(
        rename = "Container",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub container: Option<String>,
    #[doc = "Gets or sets the sub container(s) which this container must meet."]
    #[serde(
        rename = "SubContainer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sub_container: Option<String>,
    #[serde(
        rename = "Type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_: Option<DlnaProfileType>,
}

impl Default for ContainerProfile {
    fn default() -> Self {
        Self {
            conditions: Default::default(),
            container: Default::default(),
            sub_container: Default::default(),
            type_: Default::default(),
        }
    }
}

#[doc = "A MediaBrowser.Model.Dlna.DeviceProfile represents a set of metadata which determines which content a certain device is able to play.\r\n<br />\r\nSpecifically, it defines the supported <see cref=\"P:MediaBrowser.Model.Dlna.DeviceProfile.ContainerProfiles\">containers</see> and\r\n<see cref=\"P:MediaBrowser.Model.Dlna.DeviceProfile.CodecProfiles\">codecs</see> (video and/or audio, including codec profiles and levels)\r\nthe device is able to direct play (without transcoding or remuxing),\r\nas well as which <see cref=\"P:MediaBrowser.Model.Dlna.DeviceProfile.TranscodingProfiles\">containers/codecs to transcode to</see> in case it isn't."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct DeviceProfile {
    #[doc = "Gets or sets the codec profiles."]
    #[serde(
        rename = "CodecProfiles",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub codec_profiles: Vec<CodecProfile>,
    #[doc = "Gets or sets the container profiles. Failing to meet these optional conditions causes transcoding to occur."]
    #[serde(
        rename = "ContainerProfiles",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub container_profiles: Vec<ContainerProfile>,
    #[doc = "Gets or sets the direct play profiles."]
    #[serde(
        rename = "DirectPlayProfiles",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub direct_play_profiles: Vec<DirectPlayProfile>,
    #[doc = "Gets or sets the unique internal identifier."]
    #[serde(
        rename = "Id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<uuid::Uuid>,
    #[doc = "Gets or sets the maximum allowed bitrate for statically streamed content (= direct played files)."]
    #[serde(
        rename = "MaxStaticBitrate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_static_bitrate: Option<i32>,
    #[doc = "Gets or sets the maximum allowed bitrate for statically streamed (= direct played) music files."]
    #[serde(
        rename = "MaxStaticMusicBitrate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_static_music_bitrate: Option<i32>,
    #[doc = "Gets or sets the maximum allowed bitrate for all streamed content."]
    #[serde(
        rename = "MaxStreamingBitrate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_streaming_bitrate: Option<i32>,
    #[doc = "Gets or sets the maximum allowed bitrate for transcoded music streams."]
    #[serde(
        rename = "MusicStreamingTranscodingBitrate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub music_streaming_transcoding_bitrate: Option<i32>,
    #[doc = "Gets or sets the name of this device profile. User profiles must have a unique name."]
    #[serde(
        rename = "Name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,
    #[doc = "Gets or sets the subtitle profiles."]
    #[serde(
        rename = "SubtitleProfiles",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub subtitle_profiles: Vec<SubtitleProfile>,
    #[doc = "Gets or sets the transcoding profiles."]
    #[serde(
        rename = "TranscodingProfiles",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub transcoding_profiles: Vec<TranscodingProfile>,
}

impl Default for DeviceProfile {
    fn default() -> Self {
        Self {
            codec_profiles: Default::default(),
            container_profiles: Default::default(),
            direct_play_profiles: Default::default(),
            id: Default::default(),
            max_static_bitrate: Default::default(),
            max_static_music_bitrate: Default::default(),
            max_streaming_bitrate: Default::default(),
            music_streaming_transcoding_bitrate: Default::default(),
            name: Default::default(),
            subtitle_profiles: Default::default(),
            transcoding_profiles: Default::default(),
        }
    }
}

#[doc = "Defines the MediaBrowser.Model.Dlna.DirectPlayProfile."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct DirectPlayProfile {
    #[doc = "Gets or sets the audio codec."]
    #[serde(
        rename = "AudioCodec",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub audio_codec: Option<String>,
    #[doc = "Gets or sets the container."]
    #[serde(
        rename = "Container",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub container: Option<String>,
    #[serde(
        rename = "Type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_: Option<DlnaProfileType>,
    #[doc = "Gets or sets the video codec."]
    #[serde(
        rename = "VideoCodec",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub video_codec: Option<String>,
}

impl Default for DirectPlayProfile {
    fn default() -> Self {
        Self {
            audio_codec: Default::default(),
            container: Default::default(),
            type_: Default::default(),
            video_codec: Default::default(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DlnaProfileType {
    Audio,
    Video,
    Photo,
    Subtitle,
    Lyric,
}

impl std::fmt::Display for DlnaProfileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Audio => f.write_str("Audio"),
            Self::Video => f.write_str("Video"),
            Self::Photo => f.write_str("Photo"),
            Self::Subtitle => f.write_str("Subtitle"),
            Self::Lyric => f.write_str("Lyric"),
        }
    }
}

impl std::str::FromStr for DlnaProfileType {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Audio" => Ok(Self::Audio),
            "Video" => Ok(Self::Video),
            "Photo" => Ok(Self::Photo),
            "Subtitle" => Ok(Self::Subtitle),
            "Lyric" => Ok(Self::Lyric),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for DlnaProfileType {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for DlnaProfileType {
    type Error = super::error::ConversionError;
    fn try_from(
        value: &String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for DlnaProfileType {
    type Error = super::error::ConversionError;
    fn try_from(
        value: String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "`ProfileCondition`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ProfileCondition {
    #[serde(
        rename = "Condition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub condition: Option<ProfileConditionType>,
    #[serde(
        rename = "IsRequired",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_required: Option<bool>,
    #[serde(
        rename = "Property",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub property: Option<ProfileConditionValue>,
    #[serde(
        rename = "Value",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub value: Option<String>,
}

impl Default for ProfileCondition {
    fn default() -> Self {
        Self {
            condition: Default::default(),
            is_required: Default::default(),
            property: Default::default(),
            value: Default::default(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ProfileConditionType {
    Equals,
    NotEquals,
    LessThanEqual,
    GreaterThanEqual,
    EqualsAny,
}

impl std::fmt::Display for ProfileConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Equals => f.write_str("Equals"),
            Self::NotEquals => f.write_str("NotEquals"),
            Self::LessThanEqual => f.write_str("LessThanEqual"),
            Self::GreaterThanEqual => f.write_str("GreaterThanEqual"),
            Self::EqualsAny => f.write_str("EqualsAny"),
        }
    }
}

impl std::str::FromStr for ProfileConditionType {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Equals" => Ok(Self::Equals),
            "NotEquals" => Ok(Self::NotEquals),
            "LessThanEqual" => Ok(Self::LessThanEqual),
            "GreaterThanEqual" => Ok(Self::GreaterThanEqual),
            "EqualsAny" => Ok(Self::EqualsAny),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for ProfileConditionType {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for ProfileConditionType {
    type Error = super::error::ConversionError;
    fn try_from(
        value: &String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for ProfileConditionType {
    type Error = super::error::ConversionError;
    fn try_from(
        value: String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ProfileConditionValue {
    AudioChannels,
    AudioBitrate,
    AudioProfile,
    Width,
    Height,
    Has64BitOffsets,
    PacketLength,
    VideoBitDepth,
    VideoBitrate,
    VideoFramerate,
    VideoLevel,
    VideoProfile,
    VideoTimestamp,
    IsAnamorphic,
    RefFrames,
    NumAudioStreams,
    NumVideoStreams,
    IsSecondaryAudio,
    VideoCodecTag,
    IsAvc,
    IsInterlaced,
    AudioSampleRate,
    AudioBitDepth,
    VideoRangeType,
    NumStreams,
}

impl std::fmt::Display for ProfileConditionValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::AudioChannels => f.write_str("AudioChannels"),
            Self::AudioBitrate => f.write_str("AudioBitrate"),
            Self::AudioProfile => f.write_str("AudioProfile"),
            Self::Width => f.write_str("Width"),
            Self::Height => f.write_str("Height"),
            Self::Has64BitOffsets => f.write_str("Has64BitOffsets"),
            Self::PacketLength => f.write_str("PacketLength"),
            Self::VideoBitDepth => f.write_str("VideoBitDepth"),
            Self::VideoBitrate => f.write_str("VideoBitrate"),
            Self::VideoFramerate => f.write_str("VideoFramerate"),
            Self::VideoLevel => f.write_str("VideoLevel"),
            Self::VideoProfile => f.write_str("VideoProfile"),
            Self::VideoTimestamp => f.write_str("VideoTimestamp"),
            Self::IsAnamorphic => f.write_str("IsAnamorphic"),
            Self::RefFrames => f.write_str("RefFrames"),
            Self::NumAudioStreams => f.write_str("NumAudioStreams"),
            Self::NumVideoStreams => f.write_str("NumVideoStreams"),
            Self::IsSecondaryAudio => f.write_str("IsSecondaryAudio"),
            Self::VideoCodecTag => f.write_str("VideoCodecTag"),
            Self::IsAvc => f.write_str("IsAvc"),
            Self::IsInterlaced => f.write_str("IsInterlaced"),
            Self::AudioSampleRate => f.write_str("AudioSampleRate"),
            Self::AudioBitDepth => f.write_str("AudioBitDepth"),
            Self::VideoRangeType => f.write_str("VideoRangeType"),
            Self::NumStreams => f.write_str("NumStreams"),
        }
    }
}

impl std::str::FromStr for ProfileConditionValue {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "AudioChannels" => Ok(Self::AudioChannels),
            "AudioBitrate" => Ok(Self::AudioBitrate),
            "AudioProfile" => Ok(Self::AudioProfile),
            "Width" => Ok(Self::Width),
            "Height" => Ok(Self::Height),
            "Has64BitOffsets" => Ok(Self::Has64BitOffsets),
            "PacketLength" => Ok(Self::PacketLength),
            "VideoBitDepth" => Ok(Self::VideoBitDepth),
            "VideoBitrate" => Ok(Self::VideoBitrate),
            "VideoFramerate" => Ok(Self::VideoFramerate),
            "VideoLevel" => Ok(Self::VideoLevel),
            "VideoProfile" => Ok(Self::VideoProfile),
            "VideoTimestamp" => Ok(Self::VideoTimestamp),
            "IsAnamorphic" => Ok(Self::IsAnamorphic),
            "RefFrames" => Ok(Self::RefFrames),
            "NumAudioStreams" => Ok(Self::NumAudioStreams),
            "NumVideoStreams" => Ok(Self::NumVideoStreams),
            "IsSecondaryAudio" => Ok(Self::IsSecondaryAudio),
            "VideoCodecTag" => Ok(Self::VideoCodecTag),
            "IsAvc" => Ok(Self::IsAvc),
            "IsInterlaced" => Ok(Self::IsInterlaced),
            "AudioSampleRate" => Ok(Self::AudioSampleRate),
            "AudioBitDepth" => Ok(Self::AudioBitDepth),
            "VideoRangeType" => Ok(Self::VideoRangeType),
            "NumStreams" => Ok(Self::NumStreams),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for ProfileConditionValue {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for ProfileConditionValue {
    type Error = super::error::ConversionError;
    fn try_from(
        value: &String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for ProfileConditionValue {
    type Error = super::error::ConversionError;
    fn try_from(
        value: String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "A class for subtitle profile information."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct SubtitleProfile {
    #[doc = "Gets or sets the container."]
    #[serde(
        rename = "Container",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub container: Option<String>,
    #[doc = "Gets or sets the DIDL mode."]
    #[serde(
        rename = "DidlMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub didl_mode: Option<String>,
    #[doc = "Gets or sets the format."]
    #[serde(
        rename = "Format",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub format: Option<String>,
    #[doc = "Gets or sets the language."]
    #[serde(
        rename = "Language",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub language: Option<String>,
    #[serde(
        rename = "Method",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub method: Option<SubtitleDeliveryMethod>,
}

impl Default for SubtitleProfile {
    fn default() -> Self {
        Self {
            container: Default::default(),
            didl_mode: Default::default(),
            format: Default::default(),
            language: Default::default(),
            method: Default::default(),
        }
    }
}

#[doc = "A class for transcoding profile information.\r\nNote for client developers: Conditions defined in MediaBrowser.Model.Dlna.CodecProfile has higher priority and can override values defined here."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct TranscodingProfile {
    #[doc = "Gets or sets the audio codec."]
    #[serde(
        rename = "AudioCodec",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub audio_codec: Option<String>,
    #[doc = "Gets or sets a value indicating whether breaking the video stream on non-keyframes is supported."]
    #[serde(rename = "BreakOnNonKeyFrames", default)]
    pub break_on_non_key_frames: bool,
    #[doc = "Gets or sets the profile conditions."]
    #[serde(
        rename = "Conditions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub conditions: Vec<ProfileCondition>,
    #[doc = "Gets or sets the container."]
    #[serde(
        rename = "Container",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub container: Option<String>,
    #[serde(
        rename = "Context",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub context: Option<EncodingContext>,
    #[doc = "Gets or sets a value indicating whether timestamps should be copied."]
    #[serde(rename = "CopyTimestamps", default)]
    pub copy_timestamps: bool,
    #[doc = "Gets or sets a value indicating whether variable bitrate encoding is supported."]
    #[serde(
        rename = "EnableAudioVbrEncoding",
        default = "crate::types::defaults::default_bool::<true>"
    )]
    pub enable_audio_vbr_encoding: bool,
    #[doc = "Gets or sets a value indicating whether M2TS mode is enabled."]
    #[serde(rename = "EnableMpegtsM2TsMode", default)]
    pub enable_mpegts_m2_ts_mode: bool,
    #[doc = "Gets or sets a value indicating whether subtitles are allowed in the manifest."]
    #[serde(rename = "EnableSubtitlesInManifest", default)]
    pub enable_subtitles_in_manifest: bool,
    #[doc = "Gets or sets a value indicating whether the content length should be estimated."]
    #[serde(rename = "EstimateContentLength", default)]
    pub estimate_content_length: bool,
    #[doc = "Gets or sets the maximum audio channels."]
    #[serde(
        rename = "MaxAudioChannels",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_audio_channels: Option<String>,
    #[doc = "Gets or sets the minimum amount of segments."]
    #[serde(rename = "MinSegments", default)]
    pub min_segments: i32,
    #[serde(
        rename = "Protocol",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub protocol: Option<MediaStreamProtocol>,
    #[doc = "Gets or sets the segment length."]
    #[serde(rename = "SegmentLength", default)]
    pub segment_length: i32,
    #[serde(
        rename = "TranscodeSeekInfo",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transcode_seek_info: Option<TranscodeSeekInfo>,
    #[serde(
        rename = "Type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_: Option<DlnaProfileType>,
    #[doc = "Gets or sets the video codec."]
    #[serde(
        rename = "VideoCodec",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub video_codec: Option<String>,
}

impl Default for TranscodingProfile {
    fn default() -> Self {
        Self {
            audio_codec: Default::default(),
            break_on_non_key_frames: Default::default(),
            conditions: Default::default(),
            container: Default::default(),
            context: Default::default(),
            copy_timestamps: Default::default(),
            enable_audio_vbr_encoding: super::defaults::default_bool::<true>(),
            enable_mpegts_m2_ts_mode: Default::default(),
            enable_subtitles_in_manifest: Default::default(),
            estimate_content_length: Default::default(),
            max_audio_channels: Default::default(),
            min_segments: Default::default(),
            protocol: Default::default(),
            segment_length: Default::default(),
            transcode_seek_info: Default::default(),
            type_: Default::default(),
            video_codec: Default::default(),
        }
    }
}

