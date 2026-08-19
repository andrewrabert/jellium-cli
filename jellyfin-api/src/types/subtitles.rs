#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum EmbeddedSubtitleOptions {
    AllowAll,
    AllowText,
    AllowImage,
    AllowNone,
}

impl std::fmt::Display for EmbeddedSubtitleOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::AllowAll => f.write_str("AllowAll"),
            Self::AllowText => f.write_str("AllowText"),
            Self::AllowImage => f.write_str("AllowImage"),
            Self::AllowNone => f.write_str("AllowNone"),
        }
    }
}

impl std::str::FromStr for EmbeddedSubtitleOptions {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "AllowAll" => Ok(Self::AllowAll),
            "AllowText" => Ok(Self::AllowText),
            "AllowImage" => Ok(Self::AllowImage),
            "AllowNone" => Ok(Self::AllowNone),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for EmbeddedSubtitleOptions {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for EmbeddedSubtitleOptions {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for EmbeddedSubtitleOptions {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "Class FontFile."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct FontFile {
    #[doc = "Gets or sets the date created."]
    #[serde(
        rename = "DateCreated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub date_created: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets the date modified."]
    #[serde(
        rename = "DateModified",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub date_modified: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets the name."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the size."]
    #[serde(rename = "Size", default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

#[doc = "LyricResponse model."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct LyricDto {
    #[doc = "Gets or sets a collection of individual lyric lines."]
    #[serde(rename = "Lyrics", default, skip_serializing_if = "Vec::is_empty")]
    pub lyrics: Vec<LyricLine>,
    #[doc = "Gets or sets Metadata for the lyrics."]
    #[serde(rename = "Metadata", default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<LyricMetadata>,
}

#[doc = "Lyric model."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct LyricLine {
    #[doc = "Gets the time-aligned cues for the song's lyrics."]
    #[serde(rename = "Cues", default, skip_serializing_if = "Option::is_none")]
    pub cues: Option<Vec<LyricLineCue>>,
    #[doc = "Gets the start time in ticks."]
    #[serde(rename = "Start", default, skip_serializing_if = "Option::is_none")]
    pub start: Option<i64>,
    #[doc = "Gets the text of this lyric line."]
    #[serde(rename = "Text", default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[doc = "LyricLineCue model, holds information about the timing of words within a LyricLine."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct LyricLineCue {
    #[doc = "Gets the end timestamp the lyric is synced to in ticks."]
    #[serde(rename = "End", default, skip_serializing_if = "Option::is_none")]
    pub end: Option<i64>,
    #[doc = "Gets the end character index of the cue."]
    #[serde(
        rename = "EndPosition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub end_position: Option<i32>,
    #[doc = "Gets the start character index of the cue."]
    #[serde(rename = "Position", default, skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[doc = "Gets the timestamp the lyric is synced to in ticks."]
    #[serde(rename = "Start", default, skip_serializing_if = "Option::is_none")]
    pub start: Option<i64>,
}

#[doc = "LyricMetadata model."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct LyricMetadata {
    #[doc = "Gets or sets the album this song is on."]
    #[serde(rename = "Album", default, skip_serializing_if = "Option::is_none")]
    pub album: Option<String>,
    #[doc = "Gets or sets the song artist."]
    #[serde(rename = "Artist", default, skip_serializing_if = "Option::is_none")]
    pub artist: Option<String>,
    #[doc = "Gets or sets the author of the lyric data."]
    #[serde(rename = "Author", default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[doc = "Gets or sets who the LRC file was created by."]
    #[serde(rename = "By", default, skip_serializing_if = "Option::is_none")]
    pub by: Option<String>,
    #[doc = "Gets or sets the software used to create the LRC file."]
    #[serde(rename = "Creator", default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    #[doc = "Gets or sets a value indicating whether this lyric is synced."]
    #[serde(rename = "IsSynced", default, skip_serializing_if = "Option::is_none")]
    pub is_synced: Option<bool>,
    #[doc = "Gets or sets the length of the song in ticks."]
    #[serde(rename = "Length", default, skip_serializing_if = "Option::is_none")]
    pub length: Option<i64>,
    #[doc = "Gets or sets the lyric offset compared to audio in ticks."]
    #[serde(rename = "Offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[doc = "Gets or sets the title of the song."]
    #[serde(rename = "Title", default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Gets or sets the version of the creator used."]
    #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[doc = "The remote lyric info dto."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct RemoteLyricInfoDto {
    #[doc = "Gets or sets the id for the lyric."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets the lyrics."]
    #[serde(rename = "Lyrics", default, skip_serializing_if = "Option::is_none")]
    pub lyrics: Option<LyricDto>,
    #[doc = "Gets the provider name."]
    #[serde(
        rename = "ProviderName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provider_name: Option<String>,
}

#[doc = "`RemoteSubtitleInfo`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct RemoteSubtitleInfo {
    #[serde(
        rename = "AiTranslated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ai_translated: Option<bool>,
    #[serde(rename = "Author", default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(rename = "Comment", default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(
        rename = "CommunityRating",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub community_rating: Option<f32>,
    #[serde(
        rename = "DateCreated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub date_created: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        rename = "DownloadCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub download_count: Option<i32>,
    #[serde(rename = "Forced", default, skip_serializing_if = "Option::is_none")]
    pub forced: Option<bool>,
    #[serde(rename = "Format", default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "FrameRate", default, skip_serializing_if = "Option::is_none")]
    pub frame_rate: Option<f32>,
    #[serde(
        rename = "HearingImpaired",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hearing_impaired: Option<bool>,
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "IsHashMatch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_hash_match: Option<bool>,
    #[serde(
        rename = "MachineTranslated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub machine_translated: Option<bool>,
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "ProviderName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provider_name: Option<String>,
    #[serde(
        rename = "ThreeLetterISOLanguageName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub three_letter_iso_language_name: Option<String>,
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum SubtitleDeliveryMethod {
    Encode,
    Embed,
    External,
    Hls,
    Drop,
}

impl std::fmt::Display for SubtitleDeliveryMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Encode => f.write_str("Encode"),
            Self::Embed => f.write_str("Embed"),
            Self::External => f.write_str("External"),
            Self::Hls => f.write_str("Hls"),
            Self::Drop => f.write_str("Drop"),
        }
    }
}

impl std::str::FromStr for SubtitleDeliveryMethod {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Encode" => Ok(Self::Encode),
            "Embed" => Ok(Self::Embed),
            "External" => Ok(Self::External),
            "Hls" => Ok(Self::Hls),
            "Drop" => Ok(Self::Drop),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for SubtitleDeliveryMethod {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for SubtitleDeliveryMethod {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for SubtitleDeliveryMethod {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "`SubtitleOptions`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct SubtitleOptions {
    #[serde(
        rename = "DownloadEpisodeSubtitles",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub download_episode_subtitles: Option<bool>,
    #[serde(
        rename = "DownloadLanguages",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub download_languages: Option<Vec<String>>,
    #[serde(
        rename = "DownloadMovieSubtitles",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub download_movie_subtitles: Option<bool>,
    #[serde(
        rename = "IsOpenSubtitleVipAccount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_open_subtitle_vip_account: Option<bool>,
    #[serde(
        rename = "OpenSubtitlesPasswordHash",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub open_subtitles_password_hash: Option<String>,
    #[serde(
        rename = "OpenSubtitlesUsername",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub open_subtitles_username: Option<String>,
    #[serde(
        rename = "RequirePerfectMatch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub require_perfect_match: Option<bool>,
    #[serde(
        rename = "SkipIfAudioTrackMatches",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub skip_if_audio_track_matches: Option<bool>,
    #[serde(
        rename = "SkipIfEmbeddedSubtitlesPresent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub skip_if_embedded_subtitles_present: Option<bool>,
}

#[doc = "Upload subtitles dto."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct UploadSubtitleDto {
    #[doc = "Gets or sets the subtitle data."]
    #[serde(rename = "Data")]
    pub data: String,
    #[doc = "Gets or sets the subtitle format."]
    #[serde(rename = "Format")]
    pub format: String,
    #[doc = "Gets or sets a value indicating whether the subtitle is forced."]
    #[serde(rename = "IsForced")]
    pub is_forced: bool,
    #[doc = "Gets or sets a value indicating whether the subtitle is for hearing impaired."]
    #[serde(rename = "IsHearingImpaired")]
    pub is_hearing_impaired: bool,
    #[doc = "Gets or sets the subtitle language."]
    #[serde(rename = "Language")]
    pub language: String,
}
