use super::*;

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DeinterlaceMethod {
    #[serde(rename = "yadif")]
    Yadif,
    #[serde(rename = "bwdif")]
    Bwdif,
}

impl std::fmt::Display for DeinterlaceMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Yadif => f.write_str("yadif"),
            Self::Bwdif => f.write_str("bwdif"),
        }
    }
}

impl std::str::FromStr for DeinterlaceMethod {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "yadif" => Ok(Self::Yadif),
            "bwdif" => Ok(Self::Bwdif),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for DeinterlaceMethod {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for DeinterlaceMethod {
    type Error = super::error::ConversionError;
    fn try_from(
        value: &String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for DeinterlaceMethod {
    type Error = super::error::ConversionError;
    fn try_from(
        value: String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DownMixStereoAlgorithms {
    None,
    Dave750,
    NightmodeDialogue,
    Rfc7845,
    Ac4,
}

impl std::fmt::Display for DownMixStereoAlgorithms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::None => f.write_str("None"),
            Self::Dave750 => f.write_str("Dave750"),
            Self::NightmodeDialogue => f.write_str("NightmodeDialogue"),
            Self::Rfc7845 => f.write_str("Rfc7845"),
            Self::Ac4 => f.write_str("Ac4"),
        }
    }
}

impl std::str::FromStr for DownMixStereoAlgorithms {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "None" => Ok(Self::None),
            "Dave750" => Ok(Self::Dave750),
            "NightmodeDialogue" => Ok(Self::NightmodeDialogue),
            "Rfc7845" => Ok(Self::Rfc7845),
            "Ac4" => Ok(Self::Ac4),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for DownMixStereoAlgorithms {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for DownMixStereoAlgorithms {
    type Error = super::error::ConversionError;
    fn try_from(
        value: &String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for DownMixStereoAlgorithms {
    type Error = super::error::ConversionError;
    fn try_from(
        value: String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EncoderPreset {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "placebo")]
    Placebo,
    #[serde(rename = "veryslow")]
    Veryslow,
    #[serde(rename = "slower")]
    Slower,
    #[serde(rename = "slow")]
    Slow,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "fast")]
    Fast,
    #[serde(rename = "faster")]
    Faster,
    #[serde(rename = "veryfast")]
    Veryfast,
    #[serde(rename = "superfast")]
    Superfast,
    #[serde(rename = "ultrafast")]
    Ultrafast,
}

impl std::fmt::Display for EncoderPreset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::Placebo => f.write_str("placebo"),
            Self::Veryslow => f.write_str("veryslow"),
            Self::Slower => f.write_str("slower"),
            Self::Slow => f.write_str("slow"),
            Self::Medium => f.write_str("medium"),
            Self::Fast => f.write_str("fast"),
            Self::Faster => f.write_str("faster"),
            Self::Veryfast => f.write_str("veryfast"),
            Self::Superfast => f.write_str("superfast"),
            Self::Ultrafast => f.write_str("ultrafast"),
        }
    }
}

impl std::str::FromStr for EncoderPreset {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "placebo" => Ok(Self::Placebo),
            "veryslow" => Ok(Self::Veryslow),
            "slower" => Ok(Self::Slower),
            "slow" => Ok(Self::Slow),
            "medium" => Ok(Self::Medium),
            "fast" => Ok(Self::Fast),
            "faster" => Ok(Self::Faster),
            "veryfast" => Ok(Self::Veryfast),
            "superfast" => Ok(Self::Superfast),
            "ultrafast" => Ok(Self::Ultrafast),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for EncoderPreset {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for EncoderPreset {
    type Error = super::error::ConversionError;
    fn try_from(
        value: &String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for EncoderPreset {
    type Error = super::error::ConversionError;
    fn try_from(
        value: String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EncodingContext {
    Streaming,
    Static,
}

impl std::fmt::Display for EncodingContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Streaming => f.write_str("Streaming"),
            Self::Static => f.write_str("Static"),
        }
    }
}

impl std::str::FromStr for EncodingContext {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Streaming" => Ok(Self::Streaming),
            "Static" => Ok(Self::Static),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for EncodingContext {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for EncodingContext {
    type Error = super::error::ConversionError;
    fn try_from(
        value: &String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for EncodingContext {
    type Error = super::error::ConversionError;
    fn try_from(
        value: String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "Class EncodingOptions."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct EncodingOptions {
    #[doc = "Gets or sets a value indicating whether AV1 encoding is enabled."]
    #[serde(
        rename = "AllowAv1Encoding",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_av1_encoding: Option<bool>,
    #[doc = "Gets or sets a value indicating whether HEVC encoding is enabled."]
    #[serde(
        rename = "AllowHevcEncoding",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_hevc_encoding: Option<bool>,
    #[doc = "Gets or sets the file extensions on-demand metadata based keyframe extraction is enabled for."]
    #[serde(
        rename = "AllowOnDemandMetadataBasedKeyframeExtractionForExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_on_demand_metadata_based_keyframe_extraction_for_extensions:
        Option<Vec<String>>,
    #[doc = "Gets or sets a value indicating whether the framerate is doubled when deinterlacing."]
    #[serde(
        rename = "DeinterlaceDoubleRate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deinterlace_double_rate: Option<bool>,
    #[serde(
        rename = "DeinterlaceMethod",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deinterlace_method: Option<DeinterlaceMethod>,
    #[serde(
        rename = "DownMixAudioBoost",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub down_mix_audio_boost: Option<f64>,
    #[serde(
        rename = "DownMixStereoAlgorithm",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub down_mix_stereo_algorithm: Option<DownMixStereoAlgorithms>,
    #[doc = "Gets or sets a value indicating whether audio VBR is enabled."]
    #[serde(
        rename = "EnableAudioVbr",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_audio_vbr: Option<bool>,
    #[doc = "Gets or sets a value indicating whether 10bit HEVC decoding is enabled."]
    #[serde(
        rename = "EnableDecodingColorDepth10Hevc",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_decoding_color_depth10_hevc: Option<bool>,
    #[doc = "Gets or sets a value indicating whether 8/10bit HEVC RExt decoding is enabled."]
    #[serde(
        rename = "EnableDecodingColorDepth10HevcRext",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_decoding_color_depth10_hevc_rext: Option<bool>,
    #[doc = "Gets or sets a value indicating whether 10bit VP9 decoding is enabled."]
    #[serde(
        rename = "EnableDecodingColorDepth10Vp9",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_decoding_color_depth10_vp9: Option<bool>,
    #[doc = "Gets or sets a value indicating whether 12bit HEVC RExt decoding is enabled."]
    #[serde(
        rename = "EnableDecodingColorDepth12HevcRext",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_decoding_color_depth12_hevc_rext: Option<bool>,
    #[doc = "Gets or sets a value indicating whether the enhanced NVDEC is enabled."]
    #[serde(
        rename = "EnableEnhancedNvdecDecoder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_enhanced_nvdec_decoder: Option<bool>,
    #[doc = "Gets or sets a value indicating whether to use the fallback font."]
    #[serde(
        rename = "EnableFallbackFont",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_fallback_font: Option<bool>,
    #[doc = "Gets or sets a value indicating whether hardware encoding is enabled."]
    #[serde(
        rename = "EnableHardwareEncoding",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_hardware_encoding: Option<bool>,
    #[doc = "Gets or sets a value indicating whether the Intel H264 low-power hardware encoder should be used."]
    #[serde(
        rename = "EnableIntelLowPowerH264HwEncoder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_intel_low_power_h264_hw_encoder: Option<bool>,
    #[doc = "Gets or sets a value indicating whether the Intel HEVC low-power hardware encoder should be used."]
    #[serde(
        rename = "EnableIntelLowPowerHevcHwEncoder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_intel_low_power_hevc_hw_encoder: Option<bool>,
    #[doc = "Gets or sets a value indicating whether segment deletion is enabled."]
    #[serde(
        rename = "EnableSegmentDeletion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_segment_deletion: Option<bool>,
    #[doc = "Gets or sets a value indicating whether subtitle extraction is enabled."]
    #[serde(
        rename = "EnableSubtitleExtraction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_subtitle_extraction: Option<bool>,
    #[doc = "Gets or sets a value indicating whether throttling is enabled."]
    #[serde(
        rename = "EnableThrottling",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_throttling: Option<bool>,
    #[doc = "Gets or sets a value indicating whether tonemapping is enabled."]
    #[serde(
        rename = "EnableTonemapping",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_tonemapping: Option<bool>,
    #[doc = "Gets or sets a value indicating whether videotoolbox tonemapping is enabled."]
    #[serde(
        rename = "EnableVideoToolboxTonemapping",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_video_toolbox_tonemapping: Option<bool>,
    #[doc = "Gets or sets a value indicating whether VPP tonemapping is enabled."]
    #[serde(
        rename = "EnableVppTonemapping",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_vpp_tonemapping: Option<bool>,
    #[doc = "Gets or sets the FFmpeg path as set by the user via the UI."]
    #[serde(
        rename = "EncoderAppPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub encoder_app_path: Option<String>,
    #[doc = "Gets or sets the current FFmpeg path being used by the system and displayed on the transcode page."]
    #[serde(
        rename = "EncoderAppPathDisplay",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub encoder_app_path_display: Option<String>,
    #[doc = "Gets or sets the encoder preset."]
    #[serde(
        rename = "EncoderPreset",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub encoder_preset: Option<EncoderPreset>,
    #[doc = "Gets or sets the thread count used for encoding."]
    #[serde(
        rename = "EncodingThreadCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub encoding_thread_count: Option<i32>,
    #[doc = "Gets or sets the path to the fallback font."]
    #[serde(
        rename = "FallbackFontPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fallback_font_path: Option<String>,
    #[doc = "Gets or sets the H264 CRF."]
    #[serde(
        rename = "H264Crf",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub h264_crf: Option<i32>,
    #[doc = "Gets or sets the H265 CRF."]
    #[serde(
        rename = "H265Crf",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub h265_crf: Option<i32>,
    #[serde(
        rename = "HardwareAccelerationType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hardware_acceleration_type: Option<HardwareAccelerationType>,
    #[doc = "Gets or sets the codecs hardware encoding is used for."]
    #[serde(
        rename = "HardwareDecodingCodecs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hardware_decoding_codecs: Option<Vec<String>>,
    #[doc = "Gets or sets the maximum size of the muxing queue."]
    #[serde(
        rename = "MaxMuxingQueueSize",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_muxing_queue_size: Option<i32>,
    #[doc = "Gets or sets a value indicating whether the system native hardware decoder should be used."]
    #[serde(
        rename = "PreferSystemNativeHwDecoder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prefer_system_native_hw_decoder: Option<bool>,
    #[doc = "Gets or sets the QSV device."]
    #[serde(
        rename = "QsvDevice",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub qsv_device: Option<String>,
    #[doc = "Gets or sets seconds for which segments should be kept before being deleted."]
    #[serde(
        rename = "SegmentKeepSeconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub segment_keep_seconds: Option<i32>,
    #[doc = "Gets or sets the delay after which throttling happens."]
    #[serde(
        rename = "ThrottleDelaySeconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub throttle_delay_seconds: Option<i32>,
    #[serde(
        rename = "TonemappingAlgorithm",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tonemapping_algorithm: Option<TonemappingAlgorithm>,
    #[serde(
        rename = "TonemappingDesat",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tonemapping_desat: Option<f64>,
    #[serde(
        rename = "TonemappingMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tonemapping_mode: Option<TonemappingMode>,
    #[serde(
        rename = "TonemappingParam",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tonemapping_param: Option<f64>,
    #[serde(
        rename = "TonemappingPeak",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tonemapping_peak: Option<f64>,
    #[serde(
        rename = "TonemappingRange",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tonemapping_range: Option<TonemappingRange>,
    #[doc = "Gets or sets the temporary transcoding path."]
    #[serde(
        rename = "TranscodingTempPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transcoding_temp_path: Option<String>,
    #[doc = "Gets or sets the VA-API device."]
    #[serde(
        rename = "VaapiDevice",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vaapi_device: Option<String>,
    #[serde(
        rename = "VppTonemappingBrightness",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vpp_tonemapping_brightness: Option<f64>,
    #[serde(
        rename = "VppTonemappingContrast",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vpp_tonemapping_contrast: Option<f64>,
}

impl Default for EncodingOptions {
    fn default() -> Self {
        Self {
            allow_av1_encoding: Default::default(),
            allow_hevc_encoding: Default::default(),
            allow_on_demand_metadata_based_keyframe_extraction_for_extensions: Default::default(
            ),
            deinterlace_double_rate: Default::default(),
            deinterlace_method: Default::default(),
            down_mix_audio_boost: Default::default(),
            down_mix_stereo_algorithm: Default::default(),
            enable_audio_vbr: Default::default(),
            enable_decoding_color_depth10_hevc: Default::default(),
            enable_decoding_color_depth10_hevc_rext: Default::default(),
            enable_decoding_color_depth10_vp9: Default::default(),
            enable_decoding_color_depth12_hevc_rext: Default::default(),
            enable_enhanced_nvdec_decoder: Default::default(),
            enable_fallback_font: Default::default(),
            enable_hardware_encoding: Default::default(),
            enable_intel_low_power_h264_hw_encoder: Default::default(),
            enable_intel_low_power_hevc_hw_encoder: Default::default(),
            enable_segment_deletion: Default::default(),
            enable_subtitle_extraction: Default::default(),
            enable_throttling: Default::default(),
            enable_tonemapping: Default::default(),
            enable_video_toolbox_tonemapping: Default::default(),
            enable_vpp_tonemapping: Default::default(),
            encoder_app_path: Default::default(),
            encoder_app_path_display: Default::default(),
            encoder_preset: Default::default(),
            encoding_thread_count: Default::default(),
            fallback_font_path: Default::default(),
            h264_crf: Default::default(),
            h265_crf: Default::default(),
            hardware_acceleration_type: Default::default(),
            hardware_decoding_codecs: Default::default(),
            max_muxing_queue_size: Default::default(),
            prefer_system_native_hw_decoder: Default::default(),
            qsv_device: Default::default(),
            segment_keep_seconds: Default::default(),
            throttle_delay_seconds: Default::default(),
            tonemapping_algorithm: Default::default(),
            tonemapping_desat: Default::default(),
            tonemapping_mode: Default::default(),
            tonemapping_param: Default::default(),
            tonemapping_peak: Default::default(),
            tonemapping_range: Default::default(),
            transcoding_temp_path: Default::default(),
            vaapi_device: Default::default(),
            vpp_tonemapping_brightness: Default::default(),
            vpp_tonemapping_contrast: Default::default(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TonemappingAlgorithm {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "clip")]
    Clip,
    #[serde(rename = "linear")]
    Linear,
    #[serde(rename = "gamma")]
    Gamma,
    #[serde(rename = "reinhard")]
    Reinhard,
    #[serde(rename = "hable")]
    Hable,
    #[serde(rename = "mobius")]
    Mobius,
    #[serde(rename = "bt2390")]
    Bt2390,
}

impl std::fmt::Display for TonemappingAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::None => f.write_str("none"),
            Self::Clip => f.write_str("clip"),
            Self::Linear => f.write_str("linear"),
            Self::Gamma => f.write_str("gamma"),
            Self::Reinhard => f.write_str("reinhard"),
            Self::Hable => f.write_str("hable"),
            Self::Mobius => f.write_str("mobius"),
            Self::Bt2390 => f.write_str("bt2390"),
        }
    }
}

impl std::str::FromStr for TonemappingAlgorithm {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "none" => Ok(Self::None),
            "clip" => Ok(Self::Clip),
            "linear" => Ok(Self::Linear),
            "gamma" => Ok(Self::Gamma),
            "reinhard" => Ok(Self::Reinhard),
            "hable" => Ok(Self::Hable),
            "mobius" => Ok(Self::Mobius),
            "bt2390" => Ok(Self::Bt2390),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for TonemappingAlgorithm {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for TonemappingAlgorithm {
    type Error = super::error::ConversionError;
    fn try_from(
        value: &String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for TonemappingAlgorithm {
    type Error = super::error::ConversionError;
    fn try_from(
        value: String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TonemappingMode {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "max")]
    Max,
    #[serde(rename = "rgb")]
    Rgb,
    #[serde(rename = "lum")]
    Lum,
    #[serde(rename = "itp")]
    Itp,
}

impl std::fmt::Display for TonemappingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::Max => f.write_str("max"),
            Self::Rgb => f.write_str("rgb"),
            Self::Lum => f.write_str("lum"),
            Self::Itp => f.write_str("itp"),
        }
    }
}

impl std::str::FromStr for TonemappingMode {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "max" => Ok(Self::Max),
            "rgb" => Ok(Self::Rgb),
            "lum" => Ok(Self::Lum),
            "itp" => Ok(Self::Itp),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for TonemappingMode {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for TonemappingMode {
    type Error = super::error::ConversionError;
    fn try_from(
        value: &String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for TonemappingMode {
    type Error = super::error::ConversionError;
    fn try_from(
        value: String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TonemappingRange {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "tv")]
    Tv,
    #[serde(rename = "pc")]
    Pc,
}

impl std::fmt::Display for TonemappingRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::Tv => f.write_str("tv"),
            Self::Pc => f.write_str("pc"),
        }
    }
}

impl std::str::FromStr for TonemappingRange {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "tv" => Ok(Self::Tv),
            "pc" => Ok(Self::Pc),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for TonemappingRange {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for TonemappingRange {
    type Error = super::error::ConversionError;
    fn try_from(
        value: &String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for TonemappingRange {
    type Error = super::error::ConversionError;
    fn try_from(
        value: String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TranscodeReason {
    ContainerNotSupported,
    VideoCodecNotSupported,
    AudioCodecNotSupported,
    SubtitleCodecNotSupported,
    AudioIsExternal,
    SecondaryAudioNotSupported,
    VideoProfileNotSupported,
    VideoLevelNotSupported,
    VideoResolutionNotSupported,
    VideoBitDepthNotSupported,
    VideoFramerateNotSupported,
    RefFramesNotSupported,
    AnamorphicVideoNotSupported,
    InterlacedVideoNotSupported,
    AudioChannelsNotSupported,
    AudioProfileNotSupported,
    AudioSampleRateNotSupported,
    AudioBitDepthNotSupported,
    ContainerBitrateExceedsLimit,
    VideoBitrateNotSupported,
    AudioBitrateNotSupported,
    UnknownVideoStreamInfo,
    UnknownAudioStreamInfo,
    DirectPlayError,
    VideoRangeTypeNotSupported,
    VideoCodecTagNotSupported,
    StreamCountExceedsLimit,
}

impl std::fmt::Display for TranscodeReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::ContainerNotSupported => f.write_str("ContainerNotSupported"),
            Self::VideoCodecNotSupported => f.write_str("VideoCodecNotSupported"),
            Self::AudioCodecNotSupported => f.write_str("AudioCodecNotSupported"),
            Self::SubtitleCodecNotSupported => f.write_str("SubtitleCodecNotSupported"),
            Self::AudioIsExternal => f.write_str("AudioIsExternal"),
            Self::SecondaryAudioNotSupported => f.write_str("SecondaryAudioNotSupported"),
            Self::VideoProfileNotSupported => f.write_str("VideoProfileNotSupported"),
            Self::VideoLevelNotSupported => f.write_str("VideoLevelNotSupported"),
            Self::VideoResolutionNotSupported => f.write_str("VideoResolutionNotSupported"),
            Self::VideoBitDepthNotSupported => f.write_str("VideoBitDepthNotSupported"),
            Self::VideoFramerateNotSupported => f.write_str("VideoFramerateNotSupported"),
            Self::RefFramesNotSupported => f.write_str("RefFramesNotSupported"),
            Self::AnamorphicVideoNotSupported => f.write_str("AnamorphicVideoNotSupported"),
            Self::InterlacedVideoNotSupported => f.write_str("InterlacedVideoNotSupported"),
            Self::AudioChannelsNotSupported => f.write_str("AudioChannelsNotSupported"),
            Self::AudioProfileNotSupported => f.write_str("AudioProfileNotSupported"),
            Self::AudioSampleRateNotSupported => f.write_str("AudioSampleRateNotSupported"),
            Self::AudioBitDepthNotSupported => f.write_str("AudioBitDepthNotSupported"),
            Self::ContainerBitrateExceedsLimit => f.write_str("ContainerBitrateExceedsLimit"),
            Self::VideoBitrateNotSupported => f.write_str("VideoBitrateNotSupported"),
            Self::AudioBitrateNotSupported => f.write_str("AudioBitrateNotSupported"),
            Self::UnknownVideoStreamInfo => f.write_str("UnknownVideoStreamInfo"),
            Self::UnknownAudioStreamInfo => f.write_str("UnknownAudioStreamInfo"),
            Self::DirectPlayError => f.write_str("DirectPlayError"),
            Self::VideoRangeTypeNotSupported => f.write_str("VideoRangeTypeNotSupported"),
            Self::VideoCodecTagNotSupported => f.write_str("VideoCodecTagNotSupported"),
            Self::StreamCountExceedsLimit => f.write_str("StreamCountExceedsLimit"),
        }
    }
}

impl std::str::FromStr for TranscodeReason {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "ContainerNotSupported" => Ok(Self::ContainerNotSupported),
            "VideoCodecNotSupported" => Ok(Self::VideoCodecNotSupported),
            "AudioCodecNotSupported" => Ok(Self::AudioCodecNotSupported),
            "SubtitleCodecNotSupported" => Ok(Self::SubtitleCodecNotSupported),
            "AudioIsExternal" => Ok(Self::AudioIsExternal),
            "SecondaryAudioNotSupported" => Ok(Self::SecondaryAudioNotSupported),
            "VideoProfileNotSupported" => Ok(Self::VideoProfileNotSupported),
            "VideoLevelNotSupported" => Ok(Self::VideoLevelNotSupported),
            "VideoResolutionNotSupported" => Ok(Self::VideoResolutionNotSupported),
            "VideoBitDepthNotSupported" => Ok(Self::VideoBitDepthNotSupported),
            "VideoFramerateNotSupported" => Ok(Self::VideoFramerateNotSupported),
            "RefFramesNotSupported" => Ok(Self::RefFramesNotSupported),
            "AnamorphicVideoNotSupported" => Ok(Self::AnamorphicVideoNotSupported),
            "InterlacedVideoNotSupported" => Ok(Self::InterlacedVideoNotSupported),
            "AudioChannelsNotSupported" => Ok(Self::AudioChannelsNotSupported),
            "AudioProfileNotSupported" => Ok(Self::AudioProfileNotSupported),
            "AudioSampleRateNotSupported" => Ok(Self::AudioSampleRateNotSupported),
            "AudioBitDepthNotSupported" => Ok(Self::AudioBitDepthNotSupported),
            "ContainerBitrateExceedsLimit" => Ok(Self::ContainerBitrateExceedsLimit),
            "VideoBitrateNotSupported" => Ok(Self::VideoBitrateNotSupported),
            "AudioBitrateNotSupported" => Ok(Self::AudioBitrateNotSupported),
            "UnknownVideoStreamInfo" => Ok(Self::UnknownVideoStreamInfo),
            "UnknownAudioStreamInfo" => Ok(Self::UnknownAudioStreamInfo),
            "DirectPlayError" => Ok(Self::DirectPlayError),
            "VideoRangeTypeNotSupported" => Ok(Self::VideoRangeTypeNotSupported),
            "VideoCodecTagNotSupported" => Ok(Self::VideoCodecTagNotSupported),
            "StreamCountExceedsLimit" => Ok(Self::StreamCountExceedsLimit),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for TranscodeReason {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for TranscodeReason {
    type Error = super::error::ConversionError;
    fn try_from(
        value: &String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for TranscodeReason {
    type Error = super::error::ConversionError;
    fn try_from(
        value: String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TranscodeSeekInfo {
    Auto,
    Bytes,
}

impl std::fmt::Display for TranscodeSeekInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("Auto"),
            Self::Bytes => f.write_str("Bytes"),
        }
    }
}

impl std::str::FromStr for TranscodeSeekInfo {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Auto" => Ok(Self::Auto),
            "Bytes" => Ok(Self::Bytes),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for TranscodeSeekInfo {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for TranscodeSeekInfo {
    type Error = super::error::ConversionError;
    fn try_from(
        value: &String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for TranscodeSeekInfo {
    type Error = super::error::ConversionError;
    fn try_from(
        value: String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

