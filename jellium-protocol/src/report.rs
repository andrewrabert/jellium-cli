//! What the browser reports about a stream, in the shape the local server
//! needs to build jellyfin-web's `Sessions/Playing` bodies.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{Repeat, StreamIndex};

/// One entry of `PlayState.BufferedRanges`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Buffered {
    pub start_ticks: i64,
    pub end_ticks: i64,
}

/// One entry of `NowPlayingQueue`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Queued {
    pub item: Uuid,
    pub playlist_item_id: String,
}

/// The progress event jellyfin-web reports under, named the way it names it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Reported {
    #[serde(rename = "timeupdate")]
    TimeUpdate,
    #[serde(rename = "pause")]
    Pause,
    #[serde(rename = "unpause")]
    Unpause,
    #[serde(rename = "volumechange")]
    VolumeChange,
    #[serde(rename = "repeatmodechange")]
    RepeatModeChange,
    #[serde(rename = "shufflequeuemodechange")]
    ShuffleQueueModeChange,
    #[serde(rename = "playlistitemmove")]
    PlaylistItemMove,
    #[serde(rename = "playlistitemremove")]
    PlaylistItemRemove,
    #[serde(rename = "playlistitemadd")]
    PlaylistItemAdd,
}

/// The queue order the Jellyfin server is told about, spelled the way
/// `getQueueShuffleMode` spells it.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Shuffle {
    #[default]
    Sorted,
    Shuffle,
}

/// What the browser knows before a stream exists, sent with the play request so
/// the local server's start report carries the same values jellyfin-web's does;
/// there the reporter is the player and reads them off itself.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reporting {
    pub volume_level: i32,
    pub muted: bool,
    pub repeat: Repeat,
    pub shuffle: Shuffle,
    pub playback_rate: f64,
    pub playlist_item_id: String,
    pub queue: Vec<Queued>,
}

/// Everything the browser knows about the playing stream, reported on start, on
/// progress and on stop.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Playing {
    pub play_session: String,
    pub volume_level: i32,
    pub muted: bool,
    pub paused: bool,
    pub repeat: Repeat,
    pub shuffle: Shuffle,
    pub position_ticks: i64,
    pub playback_start_time_ticks: i64,
    pub playback_rate: f64,
    pub subtitle_stream: Option<StreamIndex>,
    pub secondary_subtitle_stream: Option<StreamIndex>,
    pub audio_stream: Option<StreamIndex>,
    pub buffered: Vec<Buffered>,
    pub playlist_item_id: String,
    pub queue: Vec<Queued>,
}
