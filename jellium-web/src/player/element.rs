use std::time::Duration;

use jellium_protocol::Stopped;
use serde::{Deserialize, Serialize};

use crate::failure::{self, Call, Cause, Failure};
use crate::overlay;
use crate::text::Text;

mod glue {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "/js/player.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn load(stream: &str) -> Result<u32, JsValue>;
        #[wasm_bindgen(catch)]
        pub fn position() -> Result<f64, JsValue>;
        #[wasm_bindgen(catch)]
        pub fn ask(asked: &str) -> Result<(), JsValue>;
        #[wasm_bindgen(js_name = setGroupBeacon, catch)]
        pub fn set_group_beacon(beacon: &str) -> Result<(), JsValue>;
    }
}

/// One loaded stream, counted up on every load, so an event raised by a stream
/// the player has replaced is told apart from a live one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub struct Generation(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Video,
    Audio,
}

/// One text subtitle track offered to the media element.
#[derive(Debug, Clone, Serialize)]
pub struct TextTrack {
    pub path: String,
    pub label: String,
    pub language: Option<String>,
}

/// What the operating system's media controls display.
#[derive(Debug, Clone, Serialize)]
pub struct Metadata {
    pub title: String,
    pub subtitle: String,
    pub artwork: Option<String>,
}

/// Why the media element stopped.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Fault {
    Decode,
    Network,
    Unsupported,
}

/// A control the operating system's media keys asked for.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(tag = "command", rename_all = "camelCase")]
pub enum Command {
    Play,
    Pause,
    Previous,
    Next,
    SeekTo {
        #[serde(with = "super::seconds")]
        position: Duration,
    },
}

/// What the media element reported.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "event", rename_all = "camelCase")]
pub enum Event {
    Ready {
        #[serde(with = "super::seconds")]
        duration: Duration,
    },
    Progress {
        #[serde(with = "super::seconds")]
        position: Duration,
        #[serde(with = "super::seconds")]
        buffered: Duration,
        paused: bool,
    },
    /// Ten seconds have passed since the last report, playing or paused.
    ReportDue {
        #[serde(with = "super::seconds")]
        position: Duration,
    },
    Ended,
    Stalled,
    /// The element can play through from where it is.
    Playable {
        #[serde(with = "super::seconds")]
        position: Duration,
    },
    Failed {
        fault: Fault,
    },
    Command {
        command: Command,
    },
}

/// One media element event and the generation of the stream that raised it.
#[derive(Debug, Clone, PartialEq)]
pub struct Raised {
    pub generation: Generation,
    pub event: Event,
}

/// What the glue is asked to do.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "ask", rename_all = "camelCase")]
pub enum Asked<'a> {
    Play,
    Pause,
    Seek {
        #[serde(with = "super::seconds")]
        position: Duration,
    },
    /// Moves to the most recent position the stream offers, which is where a
    /// live playback resumes.
    SeekToLive,
    /// Plays at `rate`; 1.0 is the element's own pace.
    Rate {
        rate: f64,
    },
    Volume {
        volume: f32,
    },
    Muted,
    Unmuted,
    /// `selected` names the track shown and its absence turns subtitles off.
    TextTracks {
        tracks: &'a [TextTrack],
        selected: Option<usize>,
    },
    /// The style native text cues are drawn with.
    CueStyle {
        cues: &'a jellium_model::prefs::Cues,
    },
    Fullscreen,
    Windowed,
    /// Hides the cursor over the canvas.
    Idle,
    Awake,
    Metadata {
        metadata: &'a Metadata,
    },
    /// Arms the page-hide beacon that reports this position when the tab
    /// closes.
    Beacon {
        path: String,
        stopped: &'a Stopped,
    },
}

/// Whether the page-hide beacon that leaves the group when the page reloads or
/// its last tab closes is armed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupBeacon {
    Armed,
    Disarmed,
}

/// One frame the glue raised.
#[derive(Debug, Deserialize)]
#[serde(tag = "frame", rename_all = "camelCase")]
enum Frame {
    /// What the media element reported.
    Media {
        generation: Generation,
        event: Event,
    },
    /// A browser call the glue could not run.
    Broke { call: Broke, cause: String },
}

/// A browser call only the glue makes.
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
enum Broke {
    Fullscreen,
    MediaSession,
    Beacon,
}

/// What one load asks for.
#[derive(Debug, Serialize)]
struct Loading<'a> {
    delivery: &'a jellium_protocol::Delivery,
    #[serde(with = "super::seconds")]
    start: Duration,
}

/// The page-hide beacon that leaves the group, and the endpoint it posts to.
#[derive(Debug, Serialize)]
#[serde(tag = "beacon", rename_all = "camelCase")]
enum Beaconing {
    Armed { path: String },
    Disarmed,
}

/// The mounted media element; dropping it removes the element, detaches
/// hls.js and clears the media session.
pub struct Element {
    kind: Kind,
    /// The overlay mount, held for its `Drop`, which is what removes the
    /// element.
    _mounted: overlay::Mounted,
}

impl Element {
    /// Mounts the media element through the overlay: below the canvas, taking
    /// no pointer events, full-viewport for `Kind::Video` and hidden for
    /// `Kind::Audio`; glue that throws raises a failure and mounts nothing.
    pub fn mount(kind: Kind) -> Option<Element> {
        let mounted = overlay::Mounted::new(&overlay::Wanted {
            id: overlay::Id::Media,
            kind: match kind {
                Kind::Video => overlay::Kind::Video,
                Kind::Audio => overlay::Kind::Audio,
            },
            stacking: overlay::Stacking::Below,
            pointer: false,
            source: None,
            sandbox: None,
            accept: None,
            hidden: kind == Kind::Audio,
        })?;
        Some(Element {
            kind,
            _mounted: mounted,
        })
    }

    pub fn kind(&self) -> Kind {
        self.kind
    }

    /// Feeds `delivery` to the element or to hls.js, beginning at `start`, and
    /// opens the generation every event this stream raises carries; glue that
    /// throws raises a failure and answers none.
    pub fn load(
        &self,
        delivery: &jellium_protocol::Delivery,
        start: Duration,
    ) -> Option<Generation> {
        let rendered = failure::rendered(Text::FailurePlayerFrame, &Loading { delivery, start })?;
        failure::called(Call::PlayerLoad, glue::load(&rendered)).map(Generation)
    }

    /// Where the element is now, read from the element rather than from the
    /// last progress report; glue that throws raises a failure and answers
    /// none.
    pub fn position(&self) -> Option<Duration> {
        let seconds = failure::called(Call::PlayerPosition, glue::position())?;
        Some(super::seconds::span(seconds))
    }

    /// Asks the element for `asked`; glue that throws raises a failure.
    pub fn ask(&self, asked: &Asked<'_>) {
        let Some(rendered) = failure::rendered(Text::FailurePlayerFrame, asked) else {
            return;
        };
        failure::called(Call::PlayerAsk, glue::ask(&rendered));
    }
}

/// The media event `raised` carries; a message from another element, a frame
/// naming a browser call the glue could not run, and a frame that does not
/// read all answer `None`.
pub fn read(raised: &overlay::Raised) -> Option<Raised> {
    if raised.id != overlay::Id::Media {
        return None;
    }
    match failure::decoded::<Frame>(Text::FailurePlayerFrame, &raised.payload)? {
        Frame::Media { generation, event } => Some(Raised { generation, event }),
        Frame::Broke { call, cause } => {
            failure::raise(Failure::told(
                match call {
                    Broke::Fullscreen => Text::FailureFullscreen,
                    Broke::MediaSession => Text::FailureMediaSession,
                    Broke::Beacon => Text::FailureBeacon,
                },
                Cause::Browser { detail: cause },
            ));
            None
        }
    }
}

/// Arms the page-hide beacon that leaves the group when the page reloads or
/// its last tab closes, or disarms it.
pub fn set_group_beacon(beacon: GroupBeacon) {
    let beaconing = match beacon {
        GroupBeacon::Armed => Beaconing::Armed {
            path: super::control::endpoint(jellium_protocol::GROUP_LEAVE_PATH),
        },
        GroupBeacon::Disarmed => Beaconing::Disarmed,
    };
    let Some(rendered) = failure::rendered(Text::FailurePlayerFrame, &beaconing) else {
        return;
    };
    failure::called(Call::PlayerGroupBeacon, glue::set_group_beacon(&rendered));
}
