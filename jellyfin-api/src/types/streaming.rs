#[doc = "`GetAudioStreamAudioCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetAudioStreamAudioCodec(pub String);
impl std::ops::Deref for GetAudioStreamAudioCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetAudioStreamAudioCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetAudioStreamAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetAudioStreamAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetAudioStreamAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetAudioStreamAudioCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetAudioStreamAudioCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetAudioStreamByContainerAudioCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetAudioStreamByContainerAudioCodec(pub String);
impl std::ops::Deref for GetAudioStreamByContainerAudioCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetAudioStreamByContainerAudioCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetAudioStreamByContainerAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetAudioStreamByContainerAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetAudioStreamByContainerAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetAudioStreamByContainerAudioCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetAudioStreamByContainerAudioCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetAudioStreamByContainerContainer`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetAudioStreamByContainerContainer(pub String);
impl std::ops::Deref for GetAudioStreamByContainerContainer {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetAudioStreamByContainerContainer {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetAudioStreamByContainerContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetAudioStreamByContainerContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetAudioStreamByContainerContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetAudioStreamByContainerContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetAudioStreamByContainerContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetAudioStreamByContainerLevel`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetAudioStreamByContainerLevel(pub String);
impl std::ops::Deref for GetAudioStreamByContainerLevel {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetAudioStreamByContainerLevel {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> =
            std::sync::LazyLock::new(|| regress::Regex::new("-?[0-9]+(?:\\.[0-9]+)?").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"-?[0-9]+(?:\\.[0-9]+)?\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetAudioStreamByContainerLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetAudioStreamByContainerLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetAudioStreamByContainerLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetAudioStreamByContainerLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetAudioStreamByContainerLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetAudioStreamByContainerSegmentContainer`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetAudioStreamByContainerSegmentContainer(pub String);
impl std::ops::Deref for GetAudioStreamByContainerSegmentContainer {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetAudioStreamByContainerSegmentContainer {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetAudioStreamByContainerSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetAudioStreamByContainerSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetAudioStreamByContainerSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetAudioStreamByContainerSegmentContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetAudioStreamByContainerSegmentContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetAudioStreamByContainerSubtitleCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetAudioStreamByContainerSubtitleCodec(pub String);
impl std::ops::Deref for GetAudioStreamByContainerSubtitleCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetAudioStreamByContainerSubtitleCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetAudioStreamByContainerSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetAudioStreamByContainerSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetAudioStreamByContainerSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetAudioStreamByContainerSubtitleCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetAudioStreamByContainerSubtitleCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetAudioStreamByContainerVideoCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetAudioStreamByContainerVideoCodec(pub String);
impl std::ops::Deref for GetAudioStreamByContainerVideoCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetAudioStreamByContainerVideoCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetAudioStreamByContainerVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetAudioStreamByContainerVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetAudioStreamByContainerVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetAudioStreamByContainerVideoCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetAudioStreamByContainerVideoCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetAudioStreamContainer`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetAudioStreamContainer(pub String);
impl std::ops::Deref for GetAudioStreamContainer {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetAudioStreamContainer {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetAudioStreamContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetAudioStreamContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetAudioStreamContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetAudioStreamContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetAudioStreamContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetAudioStreamLevel`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetAudioStreamLevel(pub String);
impl std::ops::Deref for GetAudioStreamLevel {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetAudioStreamLevel {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> =
            std::sync::LazyLock::new(|| regress::Regex::new("-?[0-9]+(?:\\.[0-9]+)?").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"-?[0-9]+(?:\\.[0-9]+)?\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetAudioStreamLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetAudioStreamLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetAudioStreamLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetAudioStreamLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetAudioStreamLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetAudioStreamSegmentContainer`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetAudioStreamSegmentContainer(pub String);
impl std::ops::Deref for GetAudioStreamSegmentContainer {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetAudioStreamSegmentContainer {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetAudioStreamSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetAudioStreamSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetAudioStreamSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetAudioStreamSegmentContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetAudioStreamSegmentContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetAudioStreamSubtitleCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetAudioStreamSubtitleCodec(pub String);
impl std::ops::Deref for GetAudioStreamSubtitleCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetAudioStreamSubtitleCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetAudioStreamSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetAudioStreamSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetAudioStreamSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetAudioStreamSubtitleCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetAudioStreamSubtitleCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetAudioStreamVideoCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetAudioStreamVideoCodec(pub String);
impl std::ops::Deref for GetAudioStreamVideoCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetAudioStreamVideoCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetAudioStreamVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetAudioStreamVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetAudioStreamVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetAudioStreamVideoCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetAudioStreamVideoCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetHlsAudioSegmentAudioCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetHlsAudioSegmentAudioCodec(pub String);
impl std::ops::Deref for GetHlsAudioSegmentAudioCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetHlsAudioSegmentAudioCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetHlsAudioSegmentAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetHlsAudioSegmentAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetHlsAudioSegmentAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetHlsAudioSegmentAudioCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetHlsAudioSegmentAudioCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetHlsAudioSegmentContainer`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetHlsAudioSegmentContainer(pub String);
impl std::ops::Deref for GetHlsAudioSegmentContainer {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetHlsAudioSegmentContainer {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetHlsAudioSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetHlsAudioSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetHlsAudioSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetHlsAudioSegmentContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetHlsAudioSegmentContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetHlsAudioSegmentLevel`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetHlsAudioSegmentLevel(pub String);
impl std::ops::Deref for GetHlsAudioSegmentLevel {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetHlsAudioSegmentLevel {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> =
            std::sync::LazyLock::new(|| regress::Regex::new("-?[0-9]+(?:\\.[0-9]+)?").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"-?[0-9]+(?:\\.[0-9]+)?\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetHlsAudioSegmentLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetHlsAudioSegmentLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetHlsAudioSegmentLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetHlsAudioSegmentLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetHlsAudioSegmentLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetHlsAudioSegmentSegmentContainer`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetHlsAudioSegmentSegmentContainer(pub String);
impl std::ops::Deref for GetHlsAudioSegmentSegmentContainer {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetHlsAudioSegmentSegmentContainer {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetHlsAudioSegmentSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetHlsAudioSegmentSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetHlsAudioSegmentSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetHlsAudioSegmentSegmentContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetHlsAudioSegmentSegmentContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetHlsAudioSegmentSubtitleCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetHlsAudioSegmentSubtitleCodec(pub String);
impl std::ops::Deref for GetHlsAudioSegmentSubtitleCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetHlsAudioSegmentSubtitleCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetHlsAudioSegmentSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetHlsAudioSegmentSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetHlsAudioSegmentSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetHlsAudioSegmentSubtitleCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetHlsAudioSegmentSubtitleCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetHlsAudioSegmentVideoCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetHlsAudioSegmentVideoCodec(pub String);
impl std::ops::Deref for GetHlsAudioSegmentVideoCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetHlsAudioSegmentVideoCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetHlsAudioSegmentVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetHlsAudioSegmentVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetHlsAudioSegmentVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetHlsAudioSegmentVideoCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetHlsAudioSegmentVideoCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetHlsVideoSegmentAudioCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetHlsVideoSegmentAudioCodec(pub String);
impl std::ops::Deref for GetHlsVideoSegmentAudioCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetHlsVideoSegmentAudioCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetHlsVideoSegmentAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetHlsVideoSegmentAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetHlsVideoSegmentAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetHlsVideoSegmentAudioCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetHlsVideoSegmentAudioCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetHlsVideoSegmentContainer`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetHlsVideoSegmentContainer(pub String);
impl std::ops::Deref for GetHlsVideoSegmentContainer {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetHlsVideoSegmentContainer {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetHlsVideoSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetHlsVideoSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetHlsVideoSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetHlsVideoSegmentContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetHlsVideoSegmentContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetHlsVideoSegmentLevel`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetHlsVideoSegmentLevel(pub String);
impl std::ops::Deref for GetHlsVideoSegmentLevel {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetHlsVideoSegmentLevel {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> =
            std::sync::LazyLock::new(|| regress::Regex::new("-?[0-9]+(?:\\.[0-9]+)?").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"-?[0-9]+(?:\\.[0-9]+)?\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetHlsVideoSegmentLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetHlsVideoSegmentLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetHlsVideoSegmentLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetHlsVideoSegmentLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetHlsVideoSegmentLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetHlsVideoSegmentSegmentContainer`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetHlsVideoSegmentSegmentContainer(pub String);
impl std::ops::Deref for GetHlsVideoSegmentSegmentContainer {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetHlsVideoSegmentSegmentContainer {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetHlsVideoSegmentSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetHlsVideoSegmentSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetHlsVideoSegmentSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetHlsVideoSegmentSegmentContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetHlsVideoSegmentSegmentContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetHlsVideoSegmentSubtitleCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetHlsVideoSegmentSubtitleCodec(pub String);
impl std::ops::Deref for GetHlsVideoSegmentSubtitleCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetHlsVideoSegmentSubtitleCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetHlsVideoSegmentSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetHlsVideoSegmentSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetHlsVideoSegmentSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetHlsVideoSegmentSubtitleCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetHlsVideoSegmentSubtitleCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetHlsVideoSegmentVideoCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetHlsVideoSegmentVideoCodec(pub String);
impl std::ops::Deref for GetHlsVideoSegmentVideoCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetHlsVideoSegmentVideoCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetHlsVideoSegmentVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetHlsVideoSegmentVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetHlsVideoSegmentVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetHlsVideoSegmentVideoCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetHlsVideoSegmentVideoCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetLiveHlsStreamAudioCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetLiveHlsStreamAudioCodec(pub String);
impl std::ops::Deref for GetLiveHlsStreamAudioCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetLiveHlsStreamAudioCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetLiveHlsStreamAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetLiveHlsStreamAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetLiveHlsStreamAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetLiveHlsStreamAudioCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetLiveHlsStreamAudioCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetLiveHlsStreamContainer`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetLiveHlsStreamContainer(pub String);
impl std::ops::Deref for GetLiveHlsStreamContainer {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetLiveHlsStreamContainer {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetLiveHlsStreamContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetLiveHlsStreamContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetLiveHlsStreamContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetLiveHlsStreamContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetLiveHlsStreamContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetLiveHlsStreamLevel`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetLiveHlsStreamLevel(pub String);
impl std::ops::Deref for GetLiveHlsStreamLevel {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetLiveHlsStreamLevel {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> =
            std::sync::LazyLock::new(|| regress::Regex::new("-?[0-9]+(?:\\.[0-9]+)?").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"-?[0-9]+(?:\\.[0-9]+)?\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetLiveHlsStreamLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetLiveHlsStreamLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetLiveHlsStreamLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetLiveHlsStreamLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetLiveHlsStreamLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetLiveHlsStreamSegmentContainer`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetLiveHlsStreamSegmentContainer(pub String);
impl std::ops::Deref for GetLiveHlsStreamSegmentContainer {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetLiveHlsStreamSegmentContainer {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetLiveHlsStreamSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetLiveHlsStreamSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetLiveHlsStreamSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetLiveHlsStreamSegmentContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetLiveHlsStreamSegmentContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetLiveHlsStreamSubtitleCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetLiveHlsStreamSubtitleCodec(pub String);
impl std::ops::Deref for GetLiveHlsStreamSubtitleCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetLiveHlsStreamSubtitleCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetLiveHlsStreamSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetLiveHlsStreamSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetLiveHlsStreamSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetLiveHlsStreamSubtitleCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetLiveHlsStreamSubtitleCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetLiveHlsStreamVideoCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetLiveHlsStreamVideoCodec(pub String);
impl std::ops::Deref for GetLiveHlsStreamVideoCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetLiveHlsStreamVideoCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetLiveHlsStreamVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetLiveHlsStreamVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetLiveHlsStreamVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetLiveHlsStreamVideoCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetLiveHlsStreamVideoCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetLiveStreamFileContainer`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetLiveStreamFileContainer(pub String);
impl std::ops::Deref for GetLiveStreamFileContainer {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetLiveStreamFileContainer {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetLiveStreamFileContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetLiveStreamFileContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetLiveStreamFileContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetLiveStreamFileContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetLiveStreamFileContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetMasterHlsAudioPlaylistAudioCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetMasterHlsAudioPlaylistAudioCodec(pub String);
impl std::ops::Deref for GetMasterHlsAudioPlaylistAudioCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetMasterHlsAudioPlaylistAudioCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetMasterHlsAudioPlaylistAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetMasterHlsAudioPlaylistAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetMasterHlsAudioPlaylistAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetMasterHlsAudioPlaylistAudioCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetMasterHlsAudioPlaylistAudioCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetMasterHlsAudioPlaylistLevel`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetMasterHlsAudioPlaylistLevel(pub String);
impl std::ops::Deref for GetMasterHlsAudioPlaylistLevel {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetMasterHlsAudioPlaylistLevel {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> =
            std::sync::LazyLock::new(|| regress::Regex::new("-?[0-9]+(?:\\.[0-9]+)?").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"-?[0-9]+(?:\\.[0-9]+)?\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetMasterHlsAudioPlaylistLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetMasterHlsAudioPlaylistLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetMasterHlsAudioPlaylistLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetMasterHlsAudioPlaylistLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetMasterHlsAudioPlaylistLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetMasterHlsAudioPlaylistSegmentContainer`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetMasterHlsAudioPlaylistSegmentContainer(pub String);
impl std::ops::Deref for GetMasterHlsAudioPlaylistSegmentContainer {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetMasterHlsAudioPlaylistSegmentContainer {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetMasterHlsAudioPlaylistSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetMasterHlsAudioPlaylistSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetMasterHlsAudioPlaylistSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetMasterHlsAudioPlaylistSegmentContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetMasterHlsAudioPlaylistSegmentContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetMasterHlsAudioPlaylistSubtitleCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetMasterHlsAudioPlaylistSubtitleCodec(pub String);
impl std::ops::Deref for GetMasterHlsAudioPlaylistSubtitleCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetMasterHlsAudioPlaylistSubtitleCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetMasterHlsAudioPlaylistSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetMasterHlsAudioPlaylistSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetMasterHlsAudioPlaylistSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetMasterHlsAudioPlaylistSubtitleCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetMasterHlsAudioPlaylistSubtitleCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetMasterHlsAudioPlaylistVideoCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetMasterHlsAudioPlaylistVideoCodec(pub String);
impl std::ops::Deref for GetMasterHlsAudioPlaylistVideoCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetMasterHlsAudioPlaylistVideoCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetMasterHlsAudioPlaylistVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetMasterHlsAudioPlaylistVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetMasterHlsAudioPlaylistVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetMasterHlsAudioPlaylistVideoCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetMasterHlsAudioPlaylistVideoCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetMasterHlsVideoPlaylistAudioCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetMasterHlsVideoPlaylistAudioCodec(pub String);
impl std::ops::Deref for GetMasterHlsVideoPlaylistAudioCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetMasterHlsVideoPlaylistAudioCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetMasterHlsVideoPlaylistAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetMasterHlsVideoPlaylistAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetMasterHlsVideoPlaylistAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetMasterHlsVideoPlaylistAudioCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetMasterHlsVideoPlaylistAudioCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetMasterHlsVideoPlaylistLevel`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetMasterHlsVideoPlaylistLevel(pub String);
impl std::ops::Deref for GetMasterHlsVideoPlaylistLevel {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetMasterHlsVideoPlaylistLevel {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> =
            std::sync::LazyLock::new(|| regress::Regex::new("-?[0-9]+(?:\\.[0-9]+)?").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"-?[0-9]+(?:\\.[0-9]+)?\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetMasterHlsVideoPlaylistLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetMasterHlsVideoPlaylistLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetMasterHlsVideoPlaylistLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetMasterHlsVideoPlaylistLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetMasterHlsVideoPlaylistLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetMasterHlsVideoPlaylistSegmentContainer`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetMasterHlsVideoPlaylistSegmentContainer(pub String);
impl std::ops::Deref for GetMasterHlsVideoPlaylistSegmentContainer {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetMasterHlsVideoPlaylistSegmentContainer {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetMasterHlsVideoPlaylistSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetMasterHlsVideoPlaylistSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetMasterHlsVideoPlaylistSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetMasterHlsVideoPlaylistSegmentContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetMasterHlsVideoPlaylistSegmentContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetMasterHlsVideoPlaylistSubtitleCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetMasterHlsVideoPlaylistSubtitleCodec(pub String);
impl std::ops::Deref for GetMasterHlsVideoPlaylistSubtitleCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetMasterHlsVideoPlaylistSubtitleCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetMasterHlsVideoPlaylistSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetMasterHlsVideoPlaylistSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetMasterHlsVideoPlaylistSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetMasterHlsVideoPlaylistSubtitleCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetMasterHlsVideoPlaylistSubtitleCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetMasterHlsVideoPlaylistVideoCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetMasterHlsVideoPlaylistVideoCodec(pub String);
impl std::ops::Deref for GetMasterHlsVideoPlaylistVideoCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetMasterHlsVideoPlaylistVideoCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetMasterHlsVideoPlaylistVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetMasterHlsVideoPlaylistVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetMasterHlsVideoPlaylistVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetMasterHlsVideoPlaylistVideoCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetMasterHlsVideoPlaylistVideoCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetUniversalAudioStreamAudioCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetUniversalAudioStreamAudioCodec(pub String);
impl std::ops::Deref for GetUniversalAudioStreamAudioCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetUniversalAudioStreamAudioCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetUniversalAudioStreamAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetUniversalAudioStreamAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetUniversalAudioStreamAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetUniversalAudioStreamAudioCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetUniversalAudioStreamAudioCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetUniversalAudioStreamTranscodingContainer`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetUniversalAudioStreamTranscodingContainer(pub String);
impl std::ops::Deref for GetUniversalAudioStreamTranscodingContainer {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetUniversalAudioStreamTranscodingContainer {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetUniversalAudioStreamTranscodingContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetUniversalAudioStreamTranscodingContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetUniversalAudioStreamTranscodingContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetUniversalAudioStreamTranscodingContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetUniversalAudioStreamTranscodingContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetVariantHlsAudioPlaylistAudioCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetVariantHlsAudioPlaylistAudioCodec(pub String);
impl std::ops::Deref for GetVariantHlsAudioPlaylistAudioCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetVariantHlsAudioPlaylistAudioCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetVariantHlsAudioPlaylistAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetVariantHlsAudioPlaylistAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetVariantHlsAudioPlaylistAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetVariantHlsAudioPlaylistAudioCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetVariantHlsAudioPlaylistAudioCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetVariantHlsAudioPlaylistLevel`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetVariantHlsAudioPlaylistLevel(pub String);
impl std::ops::Deref for GetVariantHlsAudioPlaylistLevel {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetVariantHlsAudioPlaylistLevel {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> =
            std::sync::LazyLock::new(|| regress::Regex::new("-?[0-9]+(?:\\.[0-9]+)?").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"-?[0-9]+(?:\\.[0-9]+)?\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetVariantHlsAudioPlaylistLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetVariantHlsAudioPlaylistLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetVariantHlsAudioPlaylistLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetVariantHlsAudioPlaylistLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetVariantHlsAudioPlaylistLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetVariantHlsAudioPlaylistSegmentContainer`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetVariantHlsAudioPlaylistSegmentContainer(pub String);
impl std::ops::Deref for GetVariantHlsAudioPlaylistSegmentContainer {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetVariantHlsAudioPlaylistSegmentContainer {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetVariantHlsAudioPlaylistSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetVariantHlsAudioPlaylistSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetVariantHlsAudioPlaylistSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetVariantHlsAudioPlaylistSegmentContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetVariantHlsAudioPlaylistSegmentContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetVariantHlsAudioPlaylistSubtitleCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetVariantHlsAudioPlaylistSubtitleCodec(pub String);
impl std::ops::Deref for GetVariantHlsAudioPlaylistSubtitleCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetVariantHlsAudioPlaylistSubtitleCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetVariantHlsAudioPlaylistSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetVariantHlsAudioPlaylistSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetVariantHlsAudioPlaylistSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetVariantHlsAudioPlaylistSubtitleCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetVariantHlsAudioPlaylistSubtitleCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetVariantHlsAudioPlaylistVideoCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetVariantHlsAudioPlaylistVideoCodec(pub String);
impl std::ops::Deref for GetVariantHlsAudioPlaylistVideoCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetVariantHlsAudioPlaylistVideoCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetVariantHlsAudioPlaylistVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetVariantHlsAudioPlaylistVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetVariantHlsAudioPlaylistVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetVariantHlsAudioPlaylistVideoCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetVariantHlsAudioPlaylistVideoCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetVariantHlsVideoPlaylistAudioCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetVariantHlsVideoPlaylistAudioCodec(pub String);
impl std::ops::Deref for GetVariantHlsVideoPlaylistAudioCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetVariantHlsVideoPlaylistAudioCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetVariantHlsVideoPlaylistAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetVariantHlsVideoPlaylistAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetVariantHlsVideoPlaylistAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetVariantHlsVideoPlaylistAudioCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetVariantHlsVideoPlaylistAudioCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetVariantHlsVideoPlaylistLevel`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetVariantHlsVideoPlaylistLevel(pub String);
impl std::ops::Deref for GetVariantHlsVideoPlaylistLevel {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetVariantHlsVideoPlaylistLevel {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> =
            std::sync::LazyLock::new(|| regress::Regex::new("-?[0-9]+(?:\\.[0-9]+)?").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"-?[0-9]+(?:\\.[0-9]+)?\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetVariantHlsVideoPlaylistLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetVariantHlsVideoPlaylistLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetVariantHlsVideoPlaylistLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetVariantHlsVideoPlaylistLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetVariantHlsVideoPlaylistLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetVariantHlsVideoPlaylistSegmentContainer`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetVariantHlsVideoPlaylistSegmentContainer(pub String);
impl std::ops::Deref for GetVariantHlsVideoPlaylistSegmentContainer {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetVariantHlsVideoPlaylistSegmentContainer {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetVariantHlsVideoPlaylistSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetVariantHlsVideoPlaylistSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetVariantHlsVideoPlaylistSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetVariantHlsVideoPlaylistSegmentContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetVariantHlsVideoPlaylistSegmentContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetVariantHlsVideoPlaylistSubtitleCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetVariantHlsVideoPlaylistSubtitleCodec(pub String);
impl std::ops::Deref for GetVariantHlsVideoPlaylistSubtitleCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetVariantHlsVideoPlaylistSubtitleCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetVariantHlsVideoPlaylistSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetVariantHlsVideoPlaylistSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetVariantHlsVideoPlaylistSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetVariantHlsVideoPlaylistSubtitleCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetVariantHlsVideoPlaylistSubtitleCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetVariantHlsVideoPlaylistVideoCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetVariantHlsVideoPlaylistVideoCodec(pub String);
impl std::ops::Deref for GetVariantHlsVideoPlaylistVideoCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetVariantHlsVideoPlaylistVideoCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetVariantHlsVideoPlaylistVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetVariantHlsVideoPlaylistVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetVariantHlsVideoPlaylistVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetVariantHlsVideoPlaylistVideoCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetVariantHlsVideoPlaylistVideoCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetVideoStreamAudioCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetVideoStreamAudioCodec(pub String);
impl std::ops::Deref for GetVideoStreamAudioCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetVideoStreamAudioCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetVideoStreamAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetVideoStreamAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetVideoStreamAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetVideoStreamAudioCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetVideoStreamAudioCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetVideoStreamByContainerAudioCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetVideoStreamByContainerAudioCodec(pub String);
impl std::ops::Deref for GetVideoStreamByContainerAudioCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetVideoStreamByContainerAudioCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetVideoStreamByContainerAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetVideoStreamByContainerAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetVideoStreamByContainerAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetVideoStreamByContainerAudioCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetVideoStreamByContainerAudioCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetVideoStreamByContainerContainer`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetVideoStreamByContainerContainer(pub String);
impl std::ops::Deref for GetVideoStreamByContainerContainer {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetVideoStreamByContainerContainer {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetVideoStreamByContainerContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetVideoStreamByContainerContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetVideoStreamByContainerContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetVideoStreamByContainerContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetVideoStreamByContainerContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetVideoStreamByContainerLevel`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetVideoStreamByContainerLevel(pub String);
impl std::ops::Deref for GetVideoStreamByContainerLevel {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetVideoStreamByContainerLevel {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> =
            std::sync::LazyLock::new(|| regress::Regex::new("-?[0-9]+(?:\\.[0-9]+)?").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"-?[0-9]+(?:\\.[0-9]+)?\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetVideoStreamByContainerLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetVideoStreamByContainerLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetVideoStreamByContainerLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetVideoStreamByContainerLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetVideoStreamByContainerLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetVideoStreamByContainerSegmentContainer`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetVideoStreamByContainerSegmentContainer(pub String);
impl std::ops::Deref for GetVideoStreamByContainerSegmentContainer {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetVideoStreamByContainerSegmentContainer {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetVideoStreamByContainerSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetVideoStreamByContainerSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetVideoStreamByContainerSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetVideoStreamByContainerSegmentContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetVideoStreamByContainerSegmentContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetVideoStreamByContainerSubtitleCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetVideoStreamByContainerSubtitleCodec(pub String);
impl std::ops::Deref for GetVideoStreamByContainerSubtitleCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetVideoStreamByContainerSubtitleCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetVideoStreamByContainerSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetVideoStreamByContainerSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetVideoStreamByContainerSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetVideoStreamByContainerSubtitleCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetVideoStreamByContainerSubtitleCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetVideoStreamByContainerVideoCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetVideoStreamByContainerVideoCodec(pub String);
impl std::ops::Deref for GetVideoStreamByContainerVideoCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetVideoStreamByContainerVideoCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetVideoStreamByContainerVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetVideoStreamByContainerVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetVideoStreamByContainerVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetVideoStreamByContainerVideoCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetVideoStreamByContainerVideoCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetVideoStreamContainer`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetVideoStreamContainer(pub String);
impl std::ops::Deref for GetVideoStreamContainer {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetVideoStreamContainer {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetVideoStreamContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetVideoStreamContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetVideoStreamContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetVideoStreamContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetVideoStreamContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetVideoStreamLevel`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetVideoStreamLevel(pub String);
impl std::ops::Deref for GetVideoStreamLevel {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetVideoStreamLevel {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> =
            std::sync::LazyLock::new(|| regress::Regex::new("-?[0-9]+(?:\\.[0-9]+)?").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"-?[0-9]+(?:\\.[0-9]+)?\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetVideoStreamLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetVideoStreamLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetVideoStreamLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetVideoStreamLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetVideoStreamLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetVideoStreamSegmentContainer`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetVideoStreamSegmentContainer(pub String);
impl std::ops::Deref for GetVideoStreamSegmentContainer {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetVideoStreamSegmentContainer {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetVideoStreamSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetVideoStreamSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetVideoStreamSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetVideoStreamSegmentContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetVideoStreamSegmentContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetVideoStreamSubtitleCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetVideoStreamSubtitleCodec(pub String);
impl std::ops::Deref for GetVideoStreamSubtitleCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetVideoStreamSubtitleCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetVideoStreamSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetVideoStreamSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetVideoStreamSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetVideoStreamSubtitleCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetVideoStreamSubtitleCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`GetVideoStreamVideoCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GetVideoStreamVideoCodec(pub String);
impl std::ops::Deref for GetVideoStreamVideoCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for GetVideoStreamVideoCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for GetVideoStreamVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GetVideoStreamVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GetVideoStreamVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for GetVideoStreamVideoCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for GetVideoStreamVideoCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`HeadAudioStreamAudioCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct HeadAudioStreamAudioCodec(pub String);
impl std::ops::Deref for HeadAudioStreamAudioCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for HeadAudioStreamAudioCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for HeadAudioStreamAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for HeadAudioStreamAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for HeadAudioStreamAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for HeadAudioStreamAudioCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for HeadAudioStreamAudioCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`HeadAudioStreamByContainerAudioCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct HeadAudioStreamByContainerAudioCodec(pub String);
impl std::ops::Deref for HeadAudioStreamByContainerAudioCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for HeadAudioStreamByContainerAudioCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for HeadAudioStreamByContainerAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for HeadAudioStreamByContainerAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for HeadAudioStreamByContainerAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for HeadAudioStreamByContainerAudioCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for HeadAudioStreamByContainerAudioCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`HeadAudioStreamByContainerContainer`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct HeadAudioStreamByContainerContainer(pub String);
impl std::ops::Deref for HeadAudioStreamByContainerContainer {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for HeadAudioStreamByContainerContainer {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for HeadAudioStreamByContainerContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for HeadAudioStreamByContainerContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for HeadAudioStreamByContainerContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for HeadAudioStreamByContainerContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for HeadAudioStreamByContainerContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`HeadAudioStreamByContainerLevel`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct HeadAudioStreamByContainerLevel(pub String);
impl std::ops::Deref for HeadAudioStreamByContainerLevel {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for HeadAudioStreamByContainerLevel {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> =
            std::sync::LazyLock::new(|| regress::Regex::new("-?[0-9]+(?:\\.[0-9]+)?").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"-?[0-9]+(?:\\.[0-9]+)?\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for HeadAudioStreamByContainerLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for HeadAudioStreamByContainerLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for HeadAudioStreamByContainerLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for HeadAudioStreamByContainerLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for HeadAudioStreamByContainerLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`HeadAudioStreamByContainerSegmentContainer`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct HeadAudioStreamByContainerSegmentContainer(pub String);
impl std::ops::Deref for HeadAudioStreamByContainerSegmentContainer {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for HeadAudioStreamByContainerSegmentContainer {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for HeadAudioStreamByContainerSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for HeadAudioStreamByContainerSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for HeadAudioStreamByContainerSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for HeadAudioStreamByContainerSegmentContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for HeadAudioStreamByContainerSegmentContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`HeadAudioStreamByContainerSubtitleCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct HeadAudioStreamByContainerSubtitleCodec(pub String);
impl std::ops::Deref for HeadAudioStreamByContainerSubtitleCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for HeadAudioStreamByContainerSubtitleCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for HeadAudioStreamByContainerSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for HeadAudioStreamByContainerSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for HeadAudioStreamByContainerSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for HeadAudioStreamByContainerSubtitleCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for HeadAudioStreamByContainerSubtitleCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`HeadAudioStreamByContainerVideoCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct HeadAudioStreamByContainerVideoCodec(pub String);
impl std::ops::Deref for HeadAudioStreamByContainerVideoCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for HeadAudioStreamByContainerVideoCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for HeadAudioStreamByContainerVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for HeadAudioStreamByContainerVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for HeadAudioStreamByContainerVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for HeadAudioStreamByContainerVideoCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for HeadAudioStreamByContainerVideoCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`HeadAudioStreamContainer`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct HeadAudioStreamContainer(pub String);
impl std::ops::Deref for HeadAudioStreamContainer {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for HeadAudioStreamContainer {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for HeadAudioStreamContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for HeadAudioStreamContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for HeadAudioStreamContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for HeadAudioStreamContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for HeadAudioStreamContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`HeadAudioStreamLevel`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct HeadAudioStreamLevel(pub String);
impl std::ops::Deref for HeadAudioStreamLevel {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for HeadAudioStreamLevel {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> =
            std::sync::LazyLock::new(|| regress::Regex::new("-?[0-9]+(?:\\.[0-9]+)?").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"-?[0-9]+(?:\\.[0-9]+)?\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for HeadAudioStreamLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for HeadAudioStreamLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for HeadAudioStreamLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for HeadAudioStreamLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for HeadAudioStreamLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`HeadAudioStreamSegmentContainer`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct HeadAudioStreamSegmentContainer(pub String);
impl std::ops::Deref for HeadAudioStreamSegmentContainer {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for HeadAudioStreamSegmentContainer {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for HeadAudioStreamSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for HeadAudioStreamSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for HeadAudioStreamSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for HeadAudioStreamSegmentContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for HeadAudioStreamSegmentContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`HeadAudioStreamSubtitleCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct HeadAudioStreamSubtitleCodec(pub String);
impl std::ops::Deref for HeadAudioStreamSubtitleCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for HeadAudioStreamSubtitleCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for HeadAudioStreamSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for HeadAudioStreamSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for HeadAudioStreamSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for HeadAudioStreamSubtitleCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for HeadAudioStreamSubtitleCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`HeadAudioStreamVideoCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct HeadAudioStreamVideoCodec(pub String);
impl std::ops::Deref for HeadAudioStreamVideoCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for HeadAudioStreamVideoCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for HeadAudioStreamVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for HeadAudioStreamVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for HeadAudioStreamVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for HeadAudioStreamVideoCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for HeadAudioStreamVideoCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`HeadMasterHlsAudioPlaylistAudioCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct HeadMasterHlsAudioPlaylistAudioCodec(pub String);
impl std::ops::Deref for HeadMasterHlsAudioPlaylistAudioCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for HeadMasterHlsAudioPlaylistAudioCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for HeadMasterHlsAudioPlaylistAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for HeadMasterHlsAudioPlaylistAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for HeadMasterHlsAudioPlaylistAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for HeadMasterHlsAudioPlaylistAudioCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for HeadMasterHlsAudioPlaylistAudioCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`HeadMasterHlsAudioPlaylistLevel`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct HeadMasterHlsAudioPlaylistLevel(pub String);
impl std::ops::Deref for HeadMasterHlsAudioPlaylistLevel {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for HeadMasterHlsAudioPlaylistLevel {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> =
            std::sync::LazyLock::new(|| regress::Regex::new("-?[0-9]+(?:\\.[0-9]+)?").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"-?[0-9]+(?:\\.[0-9]+)?\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for HeadMasterHlsAudioPlaylistLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for HeadMasterHlsAudioPlaylistLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for HeadMasterHlsAudioPlaylistLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for HeadMasterHlsAudioPlaylistLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for HeadMasterHlsAudioPlaylistLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`HeadMasterHlsAudioPlaylistSegmentContainer`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct HeadMasterHlsAudioPlaylistSegmentContainer(pub String);
impl std::ops::Deref for HeadMasterHlsAudioPlaylistSegmentContainer {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for HeadMasterHlsAudioPlaylistSegmentContainer {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for HeadMasterHlsAudioPlaylistSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for HeadMasterHlsAudioPlaylistSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for HeadMasterHlsAudioPlaylistSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for HeadMasterHlsAudioPlaylistSegmentContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for HeadMasterHlsAudioPlaylistSegmentContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`HeadMasterHlsAudioPlaylistSubtitleCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct HeadMasterHlsAudioPlaylistSubtitleCodec(pub String);
impl std::ops::Deref for HeadMasterHlsAudioPlaylistSubtitleCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for HeadMasterHlsAudioPlaylistSubtitleCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for HeadMasterHlsAudioPlaylistSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for HeadMasterHlsAudioPlaylistSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for HeadMasterHlsAudioPlaylistSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for HeadMasterHlsAudioPlaylistSubtitleCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for HeadMasterHlsAudioPlaylistSubtitleCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`HeadMasterHlsAudioPlaylistVideoCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct HeadMasterHlsAudioPlaylistVideoCodec(pub String);
impl std::ops::Deref for HeadMasterHlsAudioPlaylistVideoCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for HeadMasterHlsAudioPlaylistVideoCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for HeadMasterHlsAudioPlaylistVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for HeadMasterHlsAudioPlaylistVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for HeadMasterHlsAudioPlaylistVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for HeadMasterHlsAudioPlaylistVideoCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for HeadMasterHlsAudioPlaylistVideoCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`HeadMasterHlsVideoPlaylistAudioCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct HeadMasterHlsVideoPlaylistAudioCodec(pub String);
impl std::ops::Deref for HeadMasterHlsVideoPlaylistAudioCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for HeadMasterHlsVideoPlaylistAudioCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for HeadMasterHlsVideoPlaylistAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for HeadMasterHlsVideoPlaylistAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for HeadMasterHlsVideoPlaylistAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for HeadMasterHlsVideoPlaylistAudioCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for HeadMasterHlsVideoPlaylistAudioCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`HeadMasterHlsVideoPlaylistLevel`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct HeadMasterHlsVideoPlaylistLevel(pub String);
impl std::ops::Deref for HeadMasterHlsVideoPlaylistLevel {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for HeadMasterHlsVideoPlaylistLevel {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> =
            std::sync::LazyLock::new(|| regress::Regex::new("-?[0-9]+(?:\\.[0-9]+)?").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"-?[0-9]+(?:\\.[0-9]+)?\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for HeadMasterHlsVideoPlaylistLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for HeadMasterHlsVideoPlaylistLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for HeadMasterHlsVideoPlaylistLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for HeadMasterHlsVideoPlaylistLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for HeadMasterHlsVideoPlaylistLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`HeadMasterHlsVideoPlaylistSegmentContainer`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct HeadMasterHlsVideoPlaylistSegmentContainer(pub String);
impl std::ops::Deref for HeadMasterHlsVideoPlaylistSegmentContainer {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for HeadMasterHlsVideoPlaylistSegmentContainer {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for HeadMasterHlsVideoPlaylistSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for HeadMasterHlsVideoPlaylistSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for HeadMasterHlsVideoPlaylistSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for HeadMasterHlsVideoPlaylistSegmentContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for HeadMasterHlsVideoPlaylistSegmentContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`HeadMasterHlsVideoPlaylistSubtitleCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct HeadMasterHlsVideoPlaylistSubtitleCodec(pub String);
impl std::ops::Deref for HeadMasterHlsVideoPlaylistSubtitleCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for HeadMasterHlsVideoPlaylistSubtitleCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for HeadMasterHlsVideoPlaylistSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for HeadMasterHlsVideoPlaylistSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for HeadMasterHlsVideoPlaylistSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for HeadMasterHlsVideoPlaylistSubtitleCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for HeadMasterHlsVideoPlaylistSubtitleCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`HeadMasterHlsVideoPlaylistVideoCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct HeadMasterHlsVideoPlaylistVideoCodec(pub String);
impl std::ops::Deref for HeadMasterHlsVideoPlaylistVideoCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for HeadMasterHlsVideoPlaylistVideoCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for HeadMasterHlsVideoPlaylistVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for HeadMasterHlsVideoPlaylistVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for HeadMasterHlsVideoPlaylistVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for HeadMasterHlsVideoPlaylistVideoCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for HeadMasterHlsVideoPlaylistVideoCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`HeadUniversalAudioStreamAudioCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct HeadUniversalAudioStreamAudioCodec(pub String);
impl std::ops::Deref for HeadUniversalAudioStreamAudioCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for HeadUniversalAudioStreamAudioCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for HeadUniversalAudioStreamAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for HeadUniversalAudioStreamAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for HeadUniversalAudioStreamAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for HeadUniversalAudioStreamAudioCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for HeadUniversalAudioStreamAudioCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`HeadUniversalAudioStreamTranscodingContainer`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct HeadUniversalAudioStreamTranscodingContainer(pub String);
impl std::ops::Deref for HeadUniversalAudioStreamTranscodingContainer {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for HeadUniversalAudioStreamTranscodingContainer {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for HeadUniversalAudioStreamTranscodingContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for HeadUniversalAudioStreamTranscodingContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for HeadUniversalAudioStreamTranscodingContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for HeadUniversalAudioStreamTranscodingContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for HeadUniversalAudioStreamTranscodingContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`HeadVideoStreamAudioCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct HeadVideoStreamAudioCodec(pub String);
impl std::ops::Deref for HeadVideoStreamAudioCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for HeadVideoStreamAudioCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for HeadVideoStreamAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for HeadVideoStreamAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for HeadVideoStreamAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for HeadVideoStreamAudioCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for HeadVideoStreamAudioCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`HeadVideoStreamByContainerAudioCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct HeadVideoStreamByContainerAudioCodec(pub String);
impl std::ops::Deref for HeadVideoStreamByContainerAudioCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for HeadVideoStreamByContainerAudioCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for HeadVideoStreamByContainerAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for HeadVideoStreamByContainerAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for HeadVideoStreamByContainerAudioCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for HeadVideoStreamByContainerAudioCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for HeadVideoStreamByContainerAudioCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`HeadVideoStreamByContainerContainer`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct HeadVideoStreamByContainerContainer(pub String);
impl std::ops::Deref for HeadVideoStreamByContainerContainer {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for HeadVideoStreamByContainerContainer {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for HeadVideoStreamByContainerContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for HeadVideoStreamByContainerContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for HeadVideoStreamByContainerContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for HeadVideoStreamByContainerContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for HeadVideoStreamByContainerContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`HeadVideoStreamByContainerLevel`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct HeadVideoStreamByContainerLevel(pub String);
impl std::ops::Deref for HeadVideoStreamByContainerLevel {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for HeadVideoStreamByContainerLevel {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> =
            std::sync::LazyLock::new(|| regress::Regex::new("-?[0-9]+(?:\\.[0-9]+)?").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"-?[0-9]+(?:\\.[0-9]+)?\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for HeadVideoStreamByContainerLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for HeadVideoStreamByContainerLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for HeadVideoStreamByContainerLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for HeadVideoStreamByContainerLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for HeadVideoStreamByContainerLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`HeadVideoStreamByContainerSegmentContainer`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct HeadVideoStreamByContainerSegmentContainer(pub String);
impl std::ops::Deref for HeadVideoStreamByContainerSegmentContainer {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for HeadVideoStreamByContainerSegmentContainer {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for HeadVideoStreamByContainerSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for HeadVideoStreamByContainerSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for HeadVideoStreamByContainerSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for HeadVideoStreamByContainerSegmentContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for HeadVideoStreamByContainerSegmentContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`HeadVideoStreamByContainerSubtitleCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct HeadVideoStreamByContainerSubtitleCodec(pub String);
impl std::ops::Deref for HeadVideoStreamByContainerSubtitleCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for HeadVideoStreamByContainerSubtitleCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for HeadVideoStreamByContainerSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for HeadVideoStreamByContainerSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for HeadVideoStreamByContainerSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for HeadVideoStreamByContainerSubtitleCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for HeadVideoStreamByContainerSubtitleCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`HeadVideoStreamByContainerVideoCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct HeadVideoStreamByContainerVideoCodec(pub String);
impl std::ops::Deref for HeadVideoStreamByContainerVideoCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for HeadVideoStreamByContainerVideoCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for HeadVideoStreamByContainerVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for HeadVideoStreamByContainerVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for HeadVideoStreamByContainerVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for HeadVideoStreamByContainerVideoCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for HeadVideoStreamByContainerVideoCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`HeadVideoStreamContainer`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct HeadVideoStreamContainer(pub String);
impl std::ops::Deref for HeadVideoStreamContainer {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for HeadVideoStreamContainer {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for HeadVideoStreamContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for HeadVideoStreamContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for HeadVideoStreamContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for HeadVideoStreamContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for HeadVideoStreamContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`HeadVideoStreamLevel`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct HeadVideoStreamLevel(pub String);
impl std::ops::Deref for HeadVideoStreamLevel {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for HeadVideoStreamLevel {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> =
            std::sync::LazyLock::new(|| regress::Regex::new("-?[0-9]+(?:\\.[0-9]+)?").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"-?[0-9]+(?:\\.[0-9]+)?\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for HeadVideoStreamLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for HeadVideoStreamLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for HeadVideoStreamLevel {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for HeadVideoStreamLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for HeadVideoStreamLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`HeadVideoStreamSegmentContainer`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct HeadVideoStreamSegmentContainer(pub String);
impl std::ops::Deref for HeadVideoStreamSegmentContainer {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for HeadVideoStreamSegmentContainer {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for HeadVideoStreamSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for HeadVideoStreamSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for HeadVideoStreamSegmentContainer {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for HeadVideoStreamSegmentContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for HeadVideoStreamSegmentContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`HeadVideoStreamSubtitleCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct HeadVideoStreamSubtitleCodec(pub String);
impl std::ops::Deref for HeadVideoStreamSubtitleCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for HeadVideoStreamSubtitleCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for HeadVideoStreamSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for HeadVideoStreamSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for HeadVideoStreamSubtitleCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for HeadVideoStreamSubtitleCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for HeadVideoStreamSubtitleCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc = "`HeadVideoStreamVideoCodec`"]
#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct HeadVideoStreamVideoCodec(pub String);
impl std::ops::Deref for HeadVideoStreamVideoCodec {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::str::FromStr for HeadVideoStreamVideoCodec {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        static PATTERN: std::sync::LazyLock<regress::Regex> = std::sync::LazyLock::new(|| {
            regress::Regex::new("^[a-zA-Z0-9\\-\\._,|]{0,40}$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9\\-\\._,|]{0,40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for HeadVideoStreamVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for HeadVideoStreamVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for HeadVideoStreamVideoCodec {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl<'de> serde::Deserialize<'de> for HeadVideoStreamVideoCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: super::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
impl std::fmt::Display for HeadVideoStreamVideoCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
