use std::cell::RefCell;

use iced::Subscription;
use iced::futures::Stream;
use iced::futures::channel::mpsc;
use wasm_bindgen::JsValue;

use crate::error::{self, Trouble};
use crate::text::{self, Template, Text};

/// One call across a foreign boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Call {
    AddEventListener,
    AudioContextNew,
    DocumentCreateElement,
    ElementRemoveAttribute,
    ElementSetAttribute,
    HtmlAudioElement,
    HtmlCanvasElement,
    HtmlCanvasElementGetContext,
    HtmlVideoElement,
    LocalStorage,
    LocalStorageGetItem,
    LocalStorageSetItem,
    LocationHost,
    LocationOrigin,
    LocationProtocol,
    NavigatorAppVersion,
    NavigatorPlatform,
    NavigatorUserAgent,
    OverlayMount,
    OverlayOpen,
    OverlayPost,
    OverlayUnmount,
    PlayerAsk,
    PlayerGroupBeacon,
    PlayerLoad,
    PlayerPosition,
    ReflectGet,
    ReflectHas,
    ScreenAvailWidth,
    ScreenHeight,
    ScreenWidth,
    SetTimeout,
    WebGl2GetParameter,
    WebSocketClose,
    WebSocketNew,
    WebSocketSend,
    WindowBtoa,
    WindowInnerHeight,
    WindowInnerWidth,
    WindowScreen,
}

impl std::fmt::Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Call::AddEventListener => "addEventListener",
            Call::AudioContextNew => "AudioContext.new",
            Call::DocumentCreateElement => "document.createElement",
            Call::ElementRemoveAttribute => "Element.removeAttribute",
            Call::ElementSetAttribute => "Element.setAttribute",
            Call::HtmlAudioElement => "HTMLAudioElement",
            Call::HtmlCanvasElement => "HTMLCanvasElement",
            Call::HtmlCanvasElementGetContext => "HTMLCanvasElement.getContext",
            Call::HtmlVideoElement => "HTMLVideoElement",
            Call::LocalStorage => "localStorage",
            Call::LocalStorageGetItem => "localStorage.getItem",
            Call::LocalStorageSetItem => "localStorage.setItem",
            Call::LocationHost => "location.host",
            Call::LocationOrigin => "location.origin",
            Call::LocationProtocol => "location.protocol",
            Call::NavigatorAppVersion => "navigator.appVersion",
            Call::NavigatorPlatform => "navigator.platform",
            Call::NavigatorUserAgent => "navigator.userAgent",
            Call::OverlayMount => "overlay.mount",
            Call::OverlayOpen => "overlay.open",
            Call::OverlayPost => "overlay.post",
            Call::OverlayUnmount => "overlay.unmount",
            Call::PlayerAsk => "player.ask",
            Call::PlayerGroupBeacon => "player.setGroupBeacon",
            Call::PlayerLoad => "player.load",
            Call::PlayerPosition => "player.position",
            Call::ReflectGet => "Reflect.get",
            Call::ReflectHas => "Reflect.has",
            Call::ScreenAvailWidth => "screen.availWidth",
            Call::ScreenHeight => "screen.height",
            Call::ScreenWidth => "screen.width",
            Call::SetTimeout => "setTimeout",
            Call::WebGl2GetParameter => "WebGL2.getParameter",
            Call::WebSocketClose => "WebSocket.close",
            Call::WebSocketNew => "WebSocket.new",
            Call::WebSocketSend => "WebSocket.send",
            Call::WindowBtoa => "window.btoa",
            Call::WindowInnerHeight => "window.innerHeight",
            Call::WindowInnerWidth => "window.innerWidth",
            Call::WindowScreen => "window.screen",
        })
    }
}

/// The machine cause a console record carries beside the sentence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cause {
    /// The status the local server or the Jellyfin server answered with, and
    /// the body it carried.
    Http { status: Option<u16>, body: String },
    /// What the JavaScript glue threw, and the call that threw it.
    Threw { call: Call, thrown: String },
    /// The event socket's close code and reason.
    Closed { code: u16, reason: String },
    /// A body, an event frame or a stored value that did not parse.
    Malformed { detail: String },
    /// A browser call that answered an error.
    Browser { detail: String },
    /// wgpu's own words for a browser offering no usable backend.
    Graphics { detail: String },
}

/// How a failure is answered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Weight {
    /// Ends the session and replaces the screen.
    Fatal,
    /// Raised as the reference's own toast, which stands until the user
    /// dismisses it.
    Raised,
    /// Recorded in the console and in the session's failure list, and drawn on
    /// no screen; what the client has already answered on screen.
    Recorded,
}

/// One failure report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Failure {
    /// The sentence shown on screen and written to the console.
    pub sentence: String,
    /// The Jellyfin server's own message, shown under the sentence.
    pub server: Option<String>,
    pub cause: Cause,
    pub weight: Weight,
}

impl Failure {
    /// The report a `Trouble` is shown as; a lost session is `Fatal` and every
    /// other trouble is `Raised`.
    pub fn of(trouble: &Trouble) -> Failure {
        Failure {
            sentence: error::sentence(trouble),
            server: error::server_said(trouble),
            cause: cause_of(trouble),
            weight: if trouble.session_lost() {
                Weight::Fatal
            } else {
                Weight::Raised
            },
        }
    }

    /// The report a string-table sentence is shown as.
    pub fn told(key: Text, cause: Cause) -> Failure {
        Failure::saying(text::lookup(key).to_owned(), cause)
    }

    /// The report a formatted sentence is shown as.
    pub fn saying(sentence: String, cause: Cause) -> Failure {
        Failure {
            sentence,
            server: None,
            cause,
            weight: Weight::Raised,
        }
    }
}

/// The machine cause a trouble carries: a relayed answer keeps its status and
/// its body, and every other trouble names itself.
fn cause_of(trouble: &Trouble) -> Cause {
    match trouble {
        Trouble::Relay { status, detail } => Cause::Http {
            status: *status,
            body: detail.clone(),
        },
        other => Cause::Http {
            status: None,
            body: format!("{other:?}"),
        },
    }
}

thread_local! {
    /// The one channel every report is written into, held for the life of the
    /// page so a report raised before the subscription starts is still
    /// delivered.
    static CHANNEL: (mpsc::UnboundedSender<Failure>, RefCell<mpsc::UnboundedReceiver<Failure>>) = {
        let (sender, receiver) = mpsc::unbounded();
        (sender, RefCell::new(receiver))
    };
}

/// Writes the console record and hands the report to the application.
pub fn raise(failure: Failure) {
    record(&failure);
    CHANNEL.with(|(sender, _)| {
        if sender.unbounded_send(failure).is_err() {
            web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(
                "the failure channel is closed",
            ));
        }
    });
}

/// Writes the console record alone, for a failure raised where nothing can be
/// shown.
pub fn record(failure: &Failure) {
    let mut line = failure.sentence.clone();
    if let Some(server) = &failure.server {
        line.push_str(" | server: ");
        line.push_str(server);
    }
    line.push_str(&format!(" | cause: {:?}", failure.cause));
    web_sys::console::error_1(&JsValue::from_str(&line));
}

/// The value a glue binding answered; a throw is raised as a failure naming
/// `call` and answers `None`.
pub fn called<T>(call: Call, answered: Result<T, JsValue>) -> Option<T> {
    match answered {
        Ok(held) => Some(held),
        Err(thrown) => {
            raise(Failure::saying(
                text::format(Template::FailureThrew, &[&call.to_string()]),
                Cause::Threw {
                    call,
                    thrown: thrown.as_string().unwrap_or_else(|| format!("{thrown:?}")),
                },
            ));
            None
        }
    }
}

/// Records `trouble` in the console and in the session's failure list under
/// `reading`, and shows it above no view.
/// This is the only way to hold a trouble without raising it, and it is named
/// so it can be found.
pub(crate) fn disregard(trouble: Trouble, reading: Text) {
    raise(Failure {
        weight: Weight::Recorded,
        ..reading_failed(&trouble, reading)
    });
}

/// The report a trouble met while reading is shown as: the sentence `reading`
/// names, over the server's own message and the trouble's machine cause.
pub fn reading_failed(trouble: &Trouble, reading: Text) -> Failure {
    Failure {
        sentence: text::lookup(reading).to_owned(),
        ..Failure::of(trouble)
    }
}

/// `value` as JSON text; a value that will not render is raised as a failure
/// carrying `sentence` and answers `None`.
pub fn rendered<T: serde::Serialize>(sentence: Text, value: &T) -> Option<String> {
    let mut rendered = Vec::new();
    let answered = value.serialize(&mut serde_json::Serializer::new(&mut rendered));
    match answered.and_then(|()| String::from_utf8(rendered).map_err(serde::ser::Error::custom)) {
        Ok(rendered) => Some(rendered),
        Err(error) => {
            raise(Failure::told(
                sentence,
                Cause::Malformed {
                    detail: error.to_string(),
                },
            ));
            None
        }
    }
}

/// `value` as a JSON value; a value that will not render is raised as a
/// failure carrying `sentence` and answers `None`.
pub fn encoded<T: serde::Serialize>(sentence: Text, value: &T) -> Option<serde_json::Value> {
    let answered = value.serialize(serde_json::value::Serializer);
    match answered {
        Ok(encoded) => Some(encoded),
        Err(error) => {
            raise(Failure::told(
                sentence,
                Cause::Malformed {
                    detail: error.to_string(),
                },
            ));
            None
        }
    }
}

/// The value `held` deserializes to; a value that will not deserialize is
/// raised as a failure carrying `sentence` and answers `None`.
pub fn parsed<T: serde::de::DeserializeOwned>(
    sentence: Text,
    held: serde_json::Value,
) -> Option<T> {
    let answered = T::deserialize(held);
    match answered {
        Ok(parsed) => Some(parsed),
        Err(error) => {
            raise(Failure::told(
                sentence,
                Cause::Malformed {
                    detail: error.to_string(),
                },
            ));
            None
        }
    }
}

/// The value `text` reads as; text that will not read, and text carrying
/// anything after the value, is raised as a failure carrying `sentence` and
/// answers `None`.
pub fn decoded<T: serde::de::DeserializeOwned>(sentence: Text, text: &str) -> Option<T> {
    match unraised::decoded(text) {
        Ok(decoded) => Some(decoded),
        Err(error) => {
            raise(Failure::told(
                sentence,
                Cause::Malformed {
                    detail: error.to_string(),
                },
            ));
            None
        }
    }
}

/// The doors above raising nothing: a value of another shape is not a failure
/// here, and the caller holds the cause and names what becomes of it.
/// This is the only way to read without raising, and it is named so it can be
/// found.
pub mod unraised {
    /// The value `text` reads as with nothing left over; anything after the
    /// value is an error.
    /// This is the only site that constructs a deserializer.
    pub fn decoded<T: serde::de::DeserializeOwned>(text: &str) -> Result<T, serde_json::Error> {
        let mut deserializer = serde_json::Deserializer::from_str(text);
        let read = T::deserialize(&mut deserializer)?;
        deserializer.end()?;
        Ok(read)
    }

    /// The value `text` reads as.
    pub fn read<T: std::str::FromStr>(text: &str) -> Result<T, T::Err> {
        text.parse()
    }
}

struct Reports;

impl Stream for Reports {
    type Item = Failure;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        context: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Failure>> {
        CHANNEL.with(|(_, receiver)| {
            std::pin::Pin::new(&mut *receiver.borrow_mut()).poll_next(context)
        })
    }
}

/// `bytes` decoded as a JPEG; bytes that will not decode are recorded and
/// answer `None`, because the card that asked for them draws the reference's own
/// fallback glyph.
pub fn decoded_image(sentence: Text, bytes: &[u8]) -> Option<image::DynamicImage> {
    match image::load_from_memory_with_format(bytes, image::ImageFormat::Jpeg) {
        Ok(decoded) => Some(decoded),
        Err(error) => {
            raise(Failure {
                weight: Weight::Recorded,
                ..Failure::told(
                    sentence,
                    Cause::Malformed {
                        detail: error.to_string(),
                    },
                )
            });
            None
        }
    }
}

/// `hash` decoded into `pixels` at `size` square; a hash that will not decode is
/// recorded and answers `None`.
/// This is the only site that decodes a BlurHash.
pub fn unblurred(
    sentence: Text,
    hash: &crate::images::Hash,
    size: jellium_model::appearance::blur::Decode,
    punch: jellium_model::appearance::blur::Punch,
) -> Option<Vec<u8>> {
    match blurhash::decode(hash.as_str(), size.count(), size.count(), punch.scale()) {
        Ok(pixels) => Some(pixels),
        Err(error) => {
            raise(Failure {
                weight: Weight::Recorded,
                ..Failure::told(
                    sentence,
                    Cause::Malformed {
                        detail: error.to_string(),
                    },
                )
            });
            None
        }
    }
}

/// The sfnt bytes inside a woff2; a file that will not unpack is raised as a
/// failure carrying `sentence` and answers `None`.
/// This is the only site that decodes a woff2.
pub fn unpacked(sentence: Text, bytes: &[u8]) -> Option<Vec<u8>> {
    match woff2_patched::decode::convert_woff2_to_ttf(&mut std::io::Cursor::new(bytes)) {
        Ok(unpacked) => Some(unpacked),
        Err(error) => {
            raise(Failure::told(
                sentence,
                Cause::Malformed {
                    detail: error.to_string(),
                },
            ));
            None
        }
    }
}

/// `value` cast to `T`; a value of another type is raised as a failure naming
/// `call` and answers `None`.
pub fn cast<T: wasm_bindgen::JsCast>(call: Call, value: impl wasm_bindgen::JsCast) -> Option<T> {
    match value.dyn_into::<T>() {
        Ok(held) => Some(held),
        Err(other) => {
            raise(Failure::saying(
                text::format(Template::FailureThrew, &[&call.to_string()]),
                Cause::Threw {
                    call,
                    thrown: format!(
                        "{:?}",
                        wasm_bindgen::JsCast::unchecked_into::<JsValue>(other)
                    ),
                },
            ));
            None
        }
    }
}

/// Every failure raised, delivered through the channel `raise` writes into.
pub fn reports() -> Subscription<Failure> {
    Subscription::run(|| Reports)
}

/// Renders a panic's own text on the boot page and hands the panic to
/// `console_error_panic_hook`, which writes its message, its Rust source
/// location and its Rust-symbol stack trace to the console.
pub fn install_panic_hook() {
    std::panic::set_hook(Box::new(move |info| {
        crate::boot::stopped(&text::format(Template::BootPanicked, &[&info.to_string()]));
        console_error_panic_hook::hook(info);
    }));
}

/// Every failure raised this session and the one shown above the view.
#[derive(Debug, Default)]
pub struct Log {
    raised: Vec<Failure>,
    raised_now: Option<Failure>,
}

impl Log {
    /// Records `failure` and raises it until it is dismissed; a `Recorded`
    /// report is recorded and raised on no screen.
    pub fn took(&mut self, failure: Failure) {
        self.raised.insert(0, failure.clone());
        if failure.weight != Weight::Recorded {
            self.raised_now = Some(failure);
        }
    }

    /// The failure raised as a toast now, and None while none is live.
    pub fn raised_now(&self) -> Option<&Failure> {
        self.raised_now.as_ref()
    }

    pub fn dismiss(&mut self) {
        self.raised_now = None;
    }

    /// Every failure raised this session, newest first, dismissed included.
    pub fn raised(&self) -> &[Failure] {
        &self.raised
    }
}

/// `value` as the narrower type; a value the narrower type cannot hold is
/// raised as a failure carrying `sentence` and answers `None`.
pub fn narrowed<T, U>(sentence: Text, value: U) -> Option<T>
where
    T: TryFrom<U>,
    U: Copy + std::fmt::Display,
{
    match T::try_from(value) {
        Ok(narrowed) => Some(narrowed),
        Err(_) => {
            raise(Failure::told(
                sentence,
                Cause::Malformed {
                    detail: value.to_string(),
                },
            ));
            None
        }
    }
}

/// The value `text` reads as; text that does not read as one is raised as a
/// failure carrying `sentence` and answers `None`.
pub fn read<T>(sentence: Text, text: &str) -> Option<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    match unraised::read(text) {
        Ok(read) => Some(read),
        Err(error) => {
            raise(Failure::told(
                sentence,
                Cause::Malformed {
                    detail: format!("{text}: {error}"),
                },
            ));
            None
        }
    }
}
