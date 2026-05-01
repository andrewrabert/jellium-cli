use super::*;

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ImageFormat {
    Bmp,
    Gif,
    Jpg,
    Png,
    Webp,
    Svg,
}

impl std::fmt::Display for ImageFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Bmp => f.write_str("Bmp"),
            Self::Gif => f.write_str("Gif"),
            Self::Jpg => f.write_str("Jpg"),
            Self::Png => f.write_str("Png"),
            Self::Webp => f.write_str("Webp"),
            Self::Svg => f.write_str("Svg"),
        }
    }
}

impl std::str::FromStr for ImageFormat {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Bmp" => Ok(Self::Bmp),
            "Gif" => Ok(Self::Gif),
            "Jpg" => Ok(Self::Jpg),
            "Png" => Ok(Self::Png),
            "Webp" => Ok(Self::Webp),
            "Svg" => Ok(Self::Svg),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for ImageFormat {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for ImageFormat {
    type Error = super::error::ConversionError;
    fn try_from(
        value: &String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for ImageFormat {
    type Error = super::error::ConversionError;
    fn try_from(
        value: String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "Class ImageInfo."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ImageInfo {
    #[doc = "Gets or sets the blurhash."]
    #[serde(
        rename = "BlurHash",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub blur_hash: Option<String>,
    #[doc = "Gets or sets the height."]
    #[serde(
        rename = "Height",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub height: Option<i32>,
    #[doc = "Gets or sets the index of the image."]
    #[serde(
        rename = "ImageIndex",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub image_index: Option<i32>,
    #[doc = "Gets or sets the image tag."]
    #[serde(
        rename = "ImageTag",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub image_tag: Option<String>,
    #[serde(
        rename = "ImageType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub image_type: Option<ImageType>,
    #[doc = "Gets or sets the path."]
    #[serde(
        rename = "Path",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub path: Option<String>,
    #[doc = "Gets or sets the size."]
    #[serde(
        rename = "Size",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub size: Option<i64>,
    #[doc = "Gets or sets the width."]
    #[serde(
        rename = "Width",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub width: Option<i32>,
}

impl Default for ImageInfo {
    fn default() -> Self {
        Self {
            blur_hash: Default::default(),
            height: Default::default(),
            image_index: Default::default(),
            image_tag: Default::default(),
            image_type: Default::default(),
            path: Default::default(),
            size: Default::default(),
            width: Default::default(),
        }
    }
}

#[doc = "`ImageOption`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ImageOption {
    #[doc = "Gets or sets the limit."]
    #[serde(
        rename = "Limit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub limit: Option<i32>,
    #[doc = "Gets or sets the minimum width."]
    #[serde(
        rename = "MinWidth",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub min_width: Option<i32>,
    #[serde(
        rename = "Type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_: Option<ImageType>,
}

impl Default for ImageOption {
    fn default() -> Self {
        Self {
            limit: Default::default(),
            min_width: Default::default(),
            type_: Default::default(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ImageOrientation {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
    LeftTop,
    RightTop,
    RightBottom,
    LeftBottom,
}

impl std::fmt::Display for ImageOrientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::TopLeft => f.write_str("TopLeft"),
            Self::TopRight => f.write_str("TopRight"),
            Self::BottomRight => f.write_str("BottomRight"),
            Self::BottomLeft => f.write_str("BottomLeft"),
            Self::LeftTop => f.write_str("LeftTop"),
            Self::RightTop => f.write_str("RightTop"),
            Self::RightBottom => f.write_str("RightBottom"),
            Self::LeftBottom => f.write_str("LeftBottom"),
        }
    }
}

impl std::str::FromStr for ImageOrientation {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "TopLeft" => Ok(Self::TopLeft),
            "TopRight" => Ok(Self::TopRight),
            "BottomRight" => Ok(Self::BottomRight),
            "BottomLeft" => Ok(Self::BottomLeft),
            "LeftTop" => Ok(Self::LeftTop),
            "RightTop" => Ok(Self::RightTop),
            "RightBottom" => Ok(Self::RightBottom),
            "LeftBottom" => Ok(Self::LeftBottom),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for ImageOrientation {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for ImageOrientation {
    type Error = super::error::ConversionError;
    fn try_from(
        value: &String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for ImageOrientation {
    type Error = super::error::ConversionError;
    fn try_from(
        value: String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "Class ImageProviderInfo."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ImageProviderInfo {
    #[doc = "Gets the name."]
    #[serde(
        rename = "Name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,
    #[doc = "Gets the supported image types."]
    #[serde(
        rename = "SupportedImages",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_images: Vec<ImageType>,
}

impl Default for ImageProviderInfo {
    fn default() -> Self {
        Self {
            name: Default::default(),
            supported_images: Default::default(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ImageResolution {
    MatchSource,
    P144,
    P240,
    P360,
    P480,
    P720,
    P1080,
    P1440,
    P2160,
}

impl std::fmt::Display for ImageResolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::MatchSource => f.write_str("MatchSource"),
            Self::P144 => f.write_str("P144"),
            Self::P240 => f.write_str("P240"),
            Self::P360 => f.write_str("P360"),
            Self::P480 => f.write_str("P480"),
            Self::P720 => f.write_str("P720"),
            Self::P1080 => f.write_str("P1080"),
            Self::P1440 => f.write_str("P1440"),
            Self::P2160 => f.write_str("P2160"),
        }
    }
}

impl std::str::FromStr for ImageResolution {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "MatchSource" => Ok(Self::MatchSource),
            "P144" => Ok(Self::P144),
            "P240" => Ok(Self::P240),
            "P360" => Ok(Self::P360),
            "P480" => Ok(Self::P480),
            "P720" => Ok(Self::P720),
            "P1080" => Ok(Self::P1080),
            "P1440" => Ok(Self::P1440),
            "P2160" => Ok(Self::P2160),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for ImageResolution {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for ImageResolution {
    type Error = super::error::ConversionError;
    fn try_from(
        value: &String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for ImageResolution {
    type Error = super::error::ConversionError;
    fn try_from(
        value: String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ImageSavingConvention {
    Legacy,
    Compatible,
}

impl std::fmt::Display for ImageSavingConvention {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Legacy => f.write_str("Legacy"),
            Self::Compatible => f.write_str("Compatible"),
        }
    }
}

impl std::str::FromStr for ImageSavingConvention {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Legacy" => Ok(Self::Legacy),
            "Compatible" => Ok(Self::Compatible),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for ImageSavingConvention {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for ImageSavingConvention {
    type Error = super::error::ConversionError;
    fn try_from(
        value: &String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for ImageSavingConvention {
    type Error = super::error::ConversionError;
    fn try_from(
        value: String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "Class RemoteImageInfo."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct RemoteImageInfo {
    #[doc = "Gets or sets the community rating."]
    #[serde(
        rename = "CommunityRating",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub community_rating: Option<f64>,
    #[doc = "Gets or sets the height."]
    #[serde(
        rename = "Height",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub height: Option<i32>,
    #[doc = "Gets or sets the language."]
    #[serde(
        rename = "Language",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub language: Option<String>,
    #[doc = "Gets or sets the name of the provider."]
    #[serde(
        rename = "ProviderName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provider_name: Option<String>,
    #[serde(
        rename = "RatingType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rating_type: Option<RatingType>,
    #[doc = "Gets or sets a url used for previewing a smaller version."]
    #[serde(
        rename = "ThumbnailUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub thumbnail_url: Option<String>,
    #[serde(
        rename = "Type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_: Option<ImageType>,
    #[doc = "Gets or sets the URL."]
    #[serde(
        rename = "Url",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub url: Option<String>,
    #[doc = "Gets or sets the vote count."]
    #[serde(
        rename = "VoteCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vote_count: Option<i32>,
    #[doc = "Gets or sets the width."]
    #[serde(
        rename = "Width",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub width: Option<i32>,
}

impl Default for RemoteImageInfo {
    fn default() -> Self {
        Self {
            community_rating: Default::default(),
            height: Default::default(),
            language: Default::default(),
            provider_name: Default::default(),
            rating_type: Default::default(),
            thumbnail_url: Default::default(),
            type_: Default::default(),
            url: Default::default(),
            vote_count: Default::default(),
            width: Default::default(),
        }
    }
}

#[doc = "Class RemoteImageResult."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct RemoteImageResult {
    #[doc = "Gets or sets the images."]
    #[serde(
        rename = "Images",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub images: Option<Vec<RemoteImageInfo>>,
    #[doc = "Gets or sets the providers."]
    #[serde(
        rename = "Providers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub providers: Option<Vec<String>>,
    #[doc = "Gets or sets the total record count."]
    #[serde(
        rename = "TotalRecordCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_record_count: Option<i32>,
}

impl Default for RemoteImageResult {
    fn default() -> Self {
        Self {
            images: Default::default(),
            providers: Default::default(),
            total_record_count: Default::default(),
        }
    }
}

