//! The seventeen jellyfin-web settings the port reads, split by scope.
//!
//! The four `userSettings` entries are account preferences, held in the
//! preference bag; the thirteen `appSettings` entries the profile and the
//! negotiation read are browser preferences, held in this browser's own
//! `localStorage` entry.

use serde::{Deserialize, Serialize};

use crate::browser::Browser;
use crate::prefs::{self, Entry};

/// What `subtitleburnin` holds, and what every value outside the three the
/// reference compares against behaves as.
// reference: subtitle-burnin-setting — browserDeviceProfile.js:1569
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BurnIn {
    #[default]
    None,
    AllComplexFormats,
    OnlyImageFormats,
    All,
}

/// A count of pixels along one axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Pixels(i32);

impl Pixels {
    /// `None` at or below zero, which is where the reference's number is falsy.
    pub fn of(count: i32) -> Option<Pixels> {
        (count > 0).then_some(Pixels(count))
    }

    pub fn count(self) -> i32 {
        self.0
    }
}

/// A speaker count.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Channels(i32);

impl Channels {
    /// `None` at or below zero, which is the reference's `-1`.
    pub fn of(count: i32) -> Option<Channels> {
        (count > 0).then_some(Channels(count))
    }

    pub fn count(self) -> i32 {
        self.0
    }
}

/// The ceiling `maxVideoWidth` selects; the sign of the stored entry chooses.
// reference: max-video-width-setting — appSettings.js:114-120
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MaxVideoWidth {
    /// A negative entry, which `apphost.js:62` answers from `appHost.screen()`.
    Screen,
    /// A zero or absent entry, which sets no width condition.
    #[default]
    Unset,
    Fixed(Pixels),
}

/// The `appSettings` entries the profile and the negotiation read.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Shared {
    pub disable_vbr_audio: bool,
    pub always_remux_flac: bool,
    pub always_remux_mp3: bool,
    pub enable_dts: bool,
    pub enable_true_hd: bool,
    pub enable_hi10p: bool,
    pub always_burn_in_subtitle_when_transcoding: bool,
    pub subtitle_burn_in: BurnIn,
    pub subtitle_render_pgs: bool,
    pub max_video_width: MaxVideoWidth,
    pub limit_supported_video_resolution: bool,
    pub preferred_transcode_video_codec: String,
    pub preferred_transcode_video_audio_codec: String,
}

impl Default for Shared {
    fn default() -> Shared {
        Shared {
            disable_vbr_audio: false,
            always_remux_flac: false,
            always_remux_mp3: false,
            enable_dts: false,
            enable_true_hd: false,
            enable_hi10p: false,
            always_burn_in_subtitle_when_transcoding: false,
            subtitle_burn_in: BurnIn::None,
            subtitle_render_pgs: false,
            max_video_width: MaxVideoWidth::Unset,
            limit_supported_video_resolution: false,
            preferred_transcode_video_codec: String::new(),
            preferred_transcode_video_audio_codec: String::new(),
        }
    }
}

impl Shared {
    /// Reads this browser's own entry; a missing or malformed entry reads as
    /// every default.
    pub fn load() -> Shared {
        prefs::stored(Entry::Shared).unwrap_or_default()
    }
}

/// The `userSettings` entries the profile and the intro request read, which the
/// reference keeps in `appSettings` under the signed-in user's id.
// reference: user-settings-scope — userSettings.js:116-123
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub prefer_fmp4_hls_container: bool,
    pub limit_segment_length: bool,
    /// An entry at or below zero, which is the reference's `-1`, carries none.
    // reference: allowed-audio-channels-setting — userSettings.js:146-152
    pub allowed_audio_channels: Option<Channels>,
    pub enable_cinema_mode: bool,
}

impl Account {
    /// The defaults the reference answers where this browser holds no entry;
    /// the fMP4-HLS container default is browser-dependent.
    // reference: prefer-fmp4-hls-container — userSettings.js:159-166
    pub fn defaults(browser: &Browser) -> Account {
        Account {
            prefer_fmp4_hls_container: browser.safari()
                || browser.firefox()
                || browser.chrome()
                || browser.edge_chromium,
            limit_segment_length: false,
            allowed_audio_channels: None,
            enable_cinema_mode: true,
        }
    }

    /// Reads the entry `user` holds; a missing or malformed entry reads as the
    /// defaults.
    pub fn read(user: uuid::Uuid, browser: &Browser) -> Account {
        prefs::stored(Entry::Account(user)).unwrap_or_else(|| Account::defaults(browser))
    }
}
