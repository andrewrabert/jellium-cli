//! The overlay is a mechanism hosting more than one element over the iced
//! canvas, each with its own identity, stacking, pointer behaviour and message
//! channel.

use std::cell::RefCell;

use iced::Subscription;
use iced::futures::Stream;
use iced::futures::channel::mpsc;
use wasm_bindgen::prelude::*;

use crate::failure::{self, Cause, Failure};
use crate::text::Text;

#[wasm_bindgen(module = "/js/overlay.js")]
extern "C" {
    #[wasm_bindgen(js_name = mount, catch)]
    #[allow(clippy::too_many_arguments)]
    fn js_mount(
        id: &str,
        kind: &str,
        stacking: &str,
        pointer: bool,
        source: &str,
        sandbox: &str,
        hidden: bool,
        accept: &str,
        sink: &JsValue,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(js_name = open, catch)]
    fn js_open(id: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(js_name = post, catch)]
    fn js_post(id: &str, payload: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(js_name = unmount, catch)]
    fn js_unmount(id: &str) -> Result<(), JsValue>;
}

/// Which overlaid element a mount, a message or an unmount names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Id {
    /// The one media element, video or audio.
    Media,
    /// The frame one plugin configuration page renders in.
    PluginPage,
    /// The one file input, shared by the profile screen and the dashboard's
    /// user screen.
    File,
}

impl Id {
    fn name(self) -> &'static str {
        match self {
            Id::Media => "media",
            Id::PluginPage => "pluginPage",
            Id::File => "file",
        }
    }
}

/// What DOM element an overlay hosts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Video,
    Audio,
    Frame,
    File,
}

impl Kind {
    fn name(self) -> &'static str {
        match self {
            Kind::Video => "video",
            Kind::Audio => "audio",
            Kind::Frame => "frame",
            Kind::File => "file",
        }
    }
}

/// Where an overlaid element sits against the iced canvas.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stacking {
    Below,
    Above,
}

impl Stacking {
    fn name(self) -> &'static str {
        match self {
            Stacking::Below => "below",
            Stacking::Above => "above",
        }
    }
}

/// What one overlaid element is mounted as.
#[derive(Debug, Clone, PartialEq)]
pub struct Wanted {
    pub id: Id,
    pub kind: Kind,
    pub stacking: Stacking,
    /// True when the element takes pointer events rather than letting them
    /// through to the canvas.
    pub pointer: bool,
    /// The document a `Kind::Frame` loads; absent for a media element.
    pub source: Option<String>,
    /// The sandbox a `Kind::Frame` is given, which is what makes its origin
    /// opaque; absent for a media element.
    pub sandbox: Option<&'static str>,
    /// True when the element is mounted hidden, which is what audio is.
    pub hidden: bool,
    /// The `accept` a `Kind::File` carries; absent for every other kind.
    pub accept: Option<&'static str>,
}

/// The types the file input accepts, which is what it is mounted with.
pub const FILE_ACCEPT: &str = "image/jpeg,image/png,image/webp,image/gif";

/// One file the input reported.
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chosen {
    pub name: String,
    pub mime: String,
    /// The file's length in bytes, as the browser reports it.
    pub size: u64,
    /// The file's bytes, base64-encoded.
    pub data: String,
}

impl Chosen {
    /// The file's bytes; a payload that does not decode raises a failure and
    /// answers empty.
    pub fn bytes(&self) -> Vec<u8> {
        use base64::Engine;
        match base64::engine::general_purpose::STANDARD.decode(&self.data) {
            Ok(bytes) => bytes,
            Err(error) => {
                failure::raise(Failure::told(
                    Text::FailureFileUndecodable,
                    Cause::Malformed {
                        detail: error.to_string(),
                    },
                ));
                Vec::new()
            }
        }
    }
}

/// The file an overlay message carries; a payload that does not parse raises a
/// failure and answers `None`.
pub fn chosen(raised: &Raised) -> Option<Chosen> {
    if raised.id != Id::File {
        return None;
    }
    failure::decoded(Text::FailureFileUnreadable, &raised.payload)
}

/// The sandbox a plugin configuration frame is given: scripts and forms, and
/// no same-origin, so the frame carries no session cookie.
pub const PLUGIN_SANDBOX: &str = "allow-scripts allow-forms";

/// One message an overlaid element raised, named by the element that raised it.
#[derive(Debug, Clone, PartialEq)]
pub struct Raised {
    pub id: Id,
    pub payload: String,
}

/// The callback the glue calls with one payload per message.
type Sink = Closure<dyn FnMut(String)>;

thread_local! {
    /// The one channel every element's callback writes into, held for the life
    /// of the page so a remount keeps reporting into the same subscription.
    static CHANNEL: (mpsc::UnboundedSender<Raised>, RefCell<mpsc::UnboundedReceiver<Raised>>) = {
        let (sender, receiver) = mpsc::unbounded();
        (sender, RefCell::new(receiver))
    };

    /// The callbacks handed to the glue, one per mounted element, each kept
    /// alive while its element is mounted.
    static SINKS: RefCell<Vec<(Id, Sink)>> = const { RefCell::new(Vec::new()) };
}

fn deliver(id: Id, payload: String) {
    CHANNEL.with(|(sender, _)| {
        if sender.unbounded_send(Raised { id, payload }).is_err() {
            web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(
                "the overlay message channel is closed",
            ));
        }
    });
}

/// One mounted element; dropping it removes the element and closes its
/// channel.
pub struct Mounted {
    id: Id,
}

impl Mounted {
    /// Mounts `wanted`, replacing whatever was mounted under its id; glue that
    /// throws raises a failure and mounts nothing.
    pub fn new(wanted: &Wanted) -> Option<Mounted> {
        let id = wanted.id;
        let sink = Sink::new(move |payload: String| deliver(id, payload));
        failure::called(
            "overlay.mount",
            js_mount(
                id.name(),
                wanted.kind.name(),
                wanted.stacking.name(),
                wanted.pointer,
                wanted.source.as_deref().unwrap_or_default(),
                wanted.sandbox.unwrap_or_default(),
                wanted.hidden,
                wanted.accept.unwrap_or_default(),
                sink.as_ref(),
            ),
        )?;
        SINKS.with(|held| {
            let mut held = held.borrow_mut();
            held.retain(|(held, _)| *held != id);
            held.push((id, sink));
        });
        Some(Mounted { id })
    }

    /// Sends one message down this element's channel.
    pub fn post(&self, payload: &str) {
        failure::called("overlay.post", js_post(self.id.name(), payload));
    }

    /// Opens the file input's own picker, which is what a Choose control does.
    pub fn choose(&self) {
        failure::called("overlay.open", js_open(self.id.name()));
    }
}

impl std::fmt::Debug for Mounted {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Mounted").field("id", &self.id).finish()
    }
}

impl Drop for Mounted {
    fn drop(&mut self) {
        failure::called("overlay.unmount", js_unmount(self.id.name()));
        SINKS.with(|held| held.borrow_mut().retain(|(held, _)| *held != self.id));
    }
}

/// The one receiver, drained by whichever subscription is running.
struct Messages;

impl Stream for Messages {
    type Item = Raised;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        context: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Raised>> {
        CHANNEL.with(|(_, receiver)| {
            std::pin::Pin::new(&mut *receiver.borrow_mut()).poll_next(context)
        })
    }
}

/// Every overlaid element's messages, delivered through the callbacks the glue
/// holds rather than a timer, so a hidden tab still reports.
pub fn messages() -> Subscription<Raised> {
    Subscription::run(|| Messages)
}
