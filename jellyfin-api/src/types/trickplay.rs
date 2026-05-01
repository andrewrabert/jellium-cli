use super::*;

#[doc = "The trickplay api model."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct TrickplayInfoDto {
    #[doc = "Gets the peak bandwidth usage in bits per second."]
    #[serde(
        rename = "Bandwidth",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bandwidth: Option<i32>,
    #[doc = "Gets the height of an individual thumbnail."]
    #[serde(
        rename = "Height",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub height: Option<i32>,
    #[doc = "Gets the interval in milliseconds between each trickplay thumbnail."]
    #[serde(
        rename = "Interval",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub interval: Option<i32>,
    #[doc = "Gets the total amount of non-black thumbnails."]
    #[serde(
        rename = "ThumbnailCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub thumbnail_count: Option<i32>,
    #[doc = "Gets the amount of thumbnails per column."]
    #[serde(
        rename = "TileHeight",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tile_height: Option<i32>,
    #[doc = "Gets the amount of thumbnails per row."]
    #[serde(
        rename = "TileWidth",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tile_width: Option<i32>,
    #[doc = "Gets the width of an individual thumbnail."]
    #[serde(
        rename = "Width",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub width: Option<i32>,
}

impl Default for TrickplayInfoDto {
    fn default() -> Self {
        Self {
            bandwidth: Default::default(),
            height: Default::default(),
            interval: Default::default(),
            thumbnail_count: Default::default(),
            tile_height: Default::default(),
            tile_width: Default::default(),
            width: Default::default(),
        }
    }
}

#[doc = "Class TrickplayOptions."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct TrickplayOptions {
    #[doc = "Gets or sets a value indicating whether or not to use HW acceleration."]
    #[serde(
        rename = "EnableHwAcceleration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_hw_acceleration: Option<bool>,
    #[doc = "Gets or sets a value indicating whether or not to use HW accelerated MJPEG encoding."]
    #[serde(
        rename = "EnableHwEncoding",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_hw_encoding: Option<bool>,
    #[doc = "Gets or sets a value indicating whether to only extract key frames.\r\nSignificantly faster, but is not compatible with all decoders and/or video files."]
    #[serde(
        rename = "EnableKeyFrameOnlyExtraction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_key_frame_only_extraction: Option<bool>,
    #[doc = "Gets or sets the interval, in ms, between each new trickplay image."]
    #[serde(
        rename = "Interval",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub interval: Option<i32>,
    #[doc = "Gets or sets the jpeg quality to use for image tiles."]
    #[serde(
        rename = "JpegQuality",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub jpeg_quality: Option<i32>,
    #[serde(
        rename = "ProcessPriority",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub process_priority: Option<ProcessPriorityClass>,
    #[doc = "Gets or sets the number of threads to be used by ffmpeg."]
    #[serde(
        rename = "ProcessThreads",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub process_threads: Option<i32>,
    #[doc = "Gets or sets the ffmpeg output quality level."]
    #[serde(
        rename = "Qscale",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub qscale: Option<i32>,
    #[serde(
        rename = "ScanBehavior",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scan_behavior: Option<TrickplayScanBehavior>,
    #[doc = "Gets or sets number of tile images to allow in Y dimension."]
    #[serde(
        rename = "TileHeight",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tile_height: Option<i32>,
    #[doc = "Gets or sets number of tile images to allow in X dimension."]
    #[serde(
        rename = "TileWidth",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tile_width: Option<i32>,
    #[doc = "Gets or sets the target width resolutions, in px, to generates preview images for."]
    #[serde(
        rename = "WidthResolutions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub width_resolutions: Vec<i32>,
}

impl Default for TrickplayOptions {
    fn default() -> Self {
        Self {
            enable_hw_acceleration: Default::default(),
            enable_hw_encoding: Default::default(),
            enable_key_frame_only_extraction: Default::default(),
            interval: Default::default(),
            jpeg_quality: Default::default(),
            process_priority: Default::default(),
            process_threads: Default::default(),
            qscale: Default::default(),
            scan_behavior: Default::default(),
            tile_height: Default::default(),
            tile_width: Default::default(),
            width_resolutions: Default::default(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TrickplayScanBehavior {
    Blocking,
    NonBlocking,
}

impl std::fmt::Display for TrickplayScanBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Blocking => f.write_str("Blocking"),
            Self::NonBlocking => f.write_str("NonBlocking"),
        }
    }
}

impl std::str::FromStr for TrickplayScanBehavior {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Blocking" => Ok(Self::Blocking),
            "NonBlocking" => Ok(Self::NonBlocking),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for TrickplayScanBehavior {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for TrickplayScanBehavior {
    type Error = super::error::ConversionError;
    fn try_from(
        value: &String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for TrickplayScanBehavior {
    type Error = super::error::ConversionError;
    fn try_from(
        value: String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

