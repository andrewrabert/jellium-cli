use std::time::Duration;

use jellium_protocol::Stopped;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::failure::{self, Cause, Failure};
use crate::overlay;
use crate::text::Text;

#[wasm_bindgen(module = "/js/player.js")]
extern "C" {
    // the stamp crosses as a wasm i32, which the boundary converts from a JS
    // Number without ToBigInt, so a throw is impossible here
    #[wasm_bindgen(js_name = load, catch)]
    fn js_load(path: &str, hls: bool, start: f64) -> Result<u32, JsValue>;
    #[wasm_bindgen(js_name = play, catch)]
    fn js_play() -> Result<(), JsValue>;
    #[wasm_bindgen(js_name = pause, catch)]
    fn js_pause() -> Result<(), JsValue>;
    #[wasm_bindgen(js_name = position, catch)]
    fn js_position() -> Result<f64, JsValue>;
    #[wasm_bindgen(js_name = seek, catch)]
    fn js_seek(seconds: f64) -> Result<(), JsValue>;
    #[wasm_bindgen(js_name = seekToLive, catch)]
    fn js_seek_to_live() -> Result<(), JsValue>;
    #[wasm_bindgen(js_name = setRate, catch)]
    fn js_set_rate(rate: f64) -> Result<(), JsValue>;
    #[wasm_bindgen(js_name = setVolume, catch)]
    fn js_set_volume(volume: f32) -> Result<(), JsValue>;
    #[wasm_bindgen(js_name = setMuted, catch)]
    fn js_set_muted(muted: bool) -> Result<(), JsValue>;
    #[wasm_bindgen(js_name = setTextTracks, catch)]
    fn js_set_text_tracks(tracks: &str, selected: i32) -> Result<(), JsValue>;
    #[wasm_bindgen(js_name = setCueStyle, catch)]
    fn js_set_cue_style(style: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(js_name = setFullscreen, catch)]
    fn js_set_fullscreen(full: bool) -> Result<(), JsValue>;
    #[wasm_bindgen(js_name = setIdle, catch)]
    fn js_set_idle(idle: bool) -> Result<(), JsValue>;
    #[wasm_bindgen(js_name = setMetadata, catch)]
    fn js_set_metadata(metadata: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(js_name = setBeacon, catch)]
    fn js_set_beacon(path: &str, body: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(js_name = setGroupBeacon, catch)]
    fn js_set_group_beacon(path: &str, armed: bool) -> Result<(), JsValue>;
}

/// One loaded stream, counted up on every load, so an event raised by a stream
/// the player has replaced is told apart from a live one.
// the counter is session-local and one load short of 2^32 loads is unreachable
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fault {
    Decode,
    Network,
    Unsupported,
}

/// A control the operating system's media keys asked for.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Command {
    Play,
    Pause,
    Previous,
    Next,
    SeekTo(Duration),
}

/// What the media element reported.
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Ready {
        duration: Duration,
    },
    Progress {
        position: Duration,
        buffered: Duration,
        paused: bool,
    },
    /// Ten seconds have passed since the last report, playing or paused.
    ReportDue {
        position: Duration,
    },
    Ended,
    Stalled,
    /// The element can play through from where it is.
    Playable {
        position: Duration,
    },
    Failed(Fault),
    Command(Command),
}

/// What the media element reported, as the glue names it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
enum Fired {
    Ready,
    Progress,
    ReportDue,
    Ended,
    Stalled,
    Playable,
    Failed,
    Command,
    /// A name this client does not carry.
    #[serde(other)]
    Unnamed,
}

/// Why the element stopped, as the glue names it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
enum Broke {
    Decode,
    Network,
    Unsupported,
}

/// What the media session asked for, as the glue names it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
enum Asked {
    Play,
    Pause,
    Previous,
    Next,
    Seek,
}

/// The json one report from the glue carries.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Reported {
    event: Fired,
    #[serde(default)]
    generation: u32,
    #[serde(default)]
    duration: f64,
    #[serde(default)]
    position: f64,
    #[serde(default)]
    buffered: f64,
    #[serde(default)]
    paused: bool,
    /// Present on a `failed` report alone.
    #[serde(default)]
    fault: Option<Broke>,
    /// Present on a `command` report alone.
    #[serde(default)]
    command: Option<Asked>,
}

/// The field `named` carried, raising `failurePlayerFrame` when the report the
/// event stands on carried none.
fn missing<T>(held: Option<T>, named: &'static str) -> Option<T> {
    if held.is_none() {
        failure::raise(Failure::told(
            Text::FailurePlayerFrame,
            Cause::Malformed {
                detail: format!("the report carried no {named}"),
            },
        ));
    }
    held
}

fn seconds(value: f64) -> Duration {
    Duration::from_secs_f64(value.max(0.0))
}

/// One media element event and the generation of the stream that raised it.
#[derive(Debug, Clone, PartialEq)]
pub struct Raised {
    pub generation: Generation,
    pub event: Event,
}

impl Reported {
    fn read(self) -> Option<Raised> {
        let generation = Generation(self.generation);
        Some(Raised {
            generation,
            event: self.into_event()?,
        })
    }

    fn into_event(self) -> Option<Event> {
        match self.event {
            Fired::Ready => Some(Event::Ready {
                duration: seconds(self.duration),
            }),
            Fired::Progress => Some(Event::Progress {
                position: seconds(self.position),
                buffered: seconds(self.buffered),
                paused: self.paused,
            }),
            Fired::ReportDue => Some(Event::ReportDue {
                position: seconds(self.position),
            }),
            Fired::Ended => Some(Event::Ended),
            Fired::Stalled => Some(Event::Stalled),
            Fired::Playable => Some(Event::Playable {
                position: seconds(self.position),
            }),
            Fired::Failed => Some(Event::Failed(match missing(self.fault, "fault")? {
                Broke::Decode => Fault::Decode,
                Broke::Network => Fault::Network,
                Broke::Unsupported => Fault::Unsupported,
            })),
            Fired::Command => Some(Event::Command(match missing(self.command, "command")? {
                Asked::Play => Command::Play,
                Asked::Pause => Command::Pause,
                Asked::Previous => Command::Previous,
                Asked::Next => Command::Next,
                Asked::Seek => Command::SeekTo(seconds(self.position)),
            })),
            Fired::Unnamed => {
                failure::raise(Failure::told(
                    Text::FailurePlayerEvent,
                    Cause::Malformed {
                        detail: "the glue named an event this client does not carry".to_owned(),
                    },
                ));
                None
            }
        }
    }
}

/// The media event an overlay message carries, and `None` for a message from
/// another element.
/// One raised payload the player could not read, which raises a failure.
pub fn read(raised: &overlay::Raised) -> Option<Raised> {
    if raised.id != overlay::Id::Media {
        return None;
    }
    failure::decoded::<Reported>(Text::FailurePlayerFrame, &raised.payload)?.read()
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
    /// `Kind::Audio`.
    /// Mounts the media element; glue that throws raises a failure and mounts
    /// nothing.
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

    /// Feeds `path` to hls.js when `hls`, and to the element's own source
    /// otherwise, beginning at `start`, and opens the generation every event
    /// this stream raises carries.
    /// Loads `path`; glue that throws raises a failure and answers no
    /// generation, so no event is matched against one.
    pub fn load(&self, path: &str, hls: bool, start: Duration) -> Option<Generation> {
        failure::called("player.load", js_load(path, hls, start.as_secs_f64())).map(Generation)
    }

    pub fn play(&self) {
        failure::called("player.play", js_play());
    }

    pub fn pause(&self) {
        failure::called("player.pause", js_pause());
    }

    /// Where the element is now, read from the element rather than from the
    /// last progress report.
    /// The element's position; glue that throws raises a failure and answers
    /// none, so no position is reported from a fiction.
    pub fn position(&self) -> Option<Duration> {
        failure::called("player.position", js_position())
            .map(|seconds| Duration::from_secs_f64(seconds.max(0.0)))
    }

    pub fn seek(&self, position: Duration) {
        failure::called("player.seek", js_seek(position.as_secs_f64()));
    }

    /// Moves to the most recent position the stream offers, which is where a
    /// live playback resumes.
    pub fn seek_to_live(&self) {
        failure::called("player.seekToLive", js_seek_to_live());
    }

    /// Plays at `rate`; 1.0 is the element's own pace.
    pub fn set_rate(&self, rate: f64) {
        failure::called("player.setRate", js_set_rate(rate));
    }

    pub fn set_volume(&self, volume: f32) {
        failure::called("player.setVolume", js_set_volume(volume));
    }

    pub fn set_muted(&self, muted: bool) {
        failure::called("player.setMuted", js_set_muted(muted));
    }

    /// Replaces the element's text tracks; `selected` names the one shown and
    /// `None` turns subtitles off.
    pub fn set_text_tracks(&self, tracks: &[TextTrack], selected: Option<usize>) {
        let Some(rendered) = failure::rendered(Text::FailureTextTracks, &tracks) else {
            return;
        };
        #[expect(
            clippy::disallowed_methods,
            reason = "a conversion that carries no cause beyond the value itself"
        )]
        let selected = selected
            .and_then(|index| i32::try_from(index).ok())
            .unwrap_or(-1);
        failure::called(
            "player.setTextTracks",
            js_set_text_tracks(&rendered, selected),
        );
    }

    /// Installs the style native text cues are drawn with.
    pub fn set_cue_style(&self, cues: &jellium_model::prefs::Cues) {
        let Some(rendered) = failure::rendered(Text::FailureCueStyle, cues) else {
            return;
        };
        failure::called("player.setCueStyle", js_set_cue_style(&rendered));
    }

    pub fn set_fullscreen(&self, fullscreen: bool) {
        failure::called("player.setFullscreen", js_set_fullscreen(fullscreen));
    }

    /// Hides the cursor over the canvas while `idle`.
    pub fn set_idle(&self, idle: bool) {
        failure::called("player.setIdle", js_set_idle(idle));
    }

    pub fn set_metadata(&self, metadata: &Metadata) {
        let Some(rendered) = failure::rendered(Text::FailureNowPlaying, metadata) else {
            return;
        };
        failure::called("player.setMetadata", js_set_metadata(&rendered));
    }

    /// Arms the page-hide beacon that reports this position when the tab
    /// closes.
    pub fn set_beacon(&self, stopped: &Stopped) {
        let Some(body) = failure::rendered(Text::FailureBeacon, stopped) else {
            return;
        };
        failure::called(
            "player.setBeacon",
            js_set_beacon(
                &super::control::endpoint(jellium_protocol::PLAYBACK_STOPPED_PATH),
                &body,
            ),
        );
    }
}

/// Arms the page-hide beacon that leaves the group when the page reloads or
/// its last tab closes, or disarms it.
pub fn set_group_beacon(armed: bool) {
    failure::called(
        "player.setGroupBeacon",
        js_set_group_beacon(
            &super::control::endpoint(jellium_protocol::GROUP_LEAVE_PATH),
            armed,
        ),
    );
}
