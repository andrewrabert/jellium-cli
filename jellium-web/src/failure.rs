use std::cell::RefCell;

use iced::Subscription;
use iced::futures::Stream;
use iced::futures::channel::mpsc;
use wasm_bindgen::JsValue;

use crate::error::{self, Trouble};
use crate::text::{self, Text};

/// The machine cause a console record carries beside the sentence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cause {
    /// The status the local server or the Jellyfin server answered with, and
    /// the body it carried.
    Http { status: Option<u16>, body: String },
    /// What the JavaScript glue threw, and the call that threw it.
    Threw { call: &'static str, thrown: String },
    /// The event socket's close code and reason.
    Closed { code: u16, reason: String },
    /// A body, an event frame or a stored value that did not parse.
    Malformed { detail: String },
    /// A browser call that answered an error.
    Browser { detail: String },
    /// wgpu's own words for a browser offering no usable backend.
    Graphics { detail: String },
}

/// How a failure is shown.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Weight {
    /// Replaces the screen; reached only when the session is lost.
    Fatal,
    /// Shown above the view until the user dismisses it.
    Passing,
    /// Recorded in the console and in the session's failure list, and shown
    /// above no view; what `disregard` raises.
    Quiet,
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
    /// other trouble is `Passing`.
    pub fn of(trouble: &Trouble) -> Failure {
        Failure {
            sentence: error::sentence(trouble),
            server: error::server_said(trouble),
            cause: cause_of(trouble),
            weight: if trouble.session_lost() {
                Weight::Fatal
            } else {
                Weight::Passing
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
            weight: Weight::Passing,
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
pub fn called<T>(call: &'static str, answered: Result<T, JsValue>) -> Option<T> {
    match answered {
        Ok(held) => Some(held),
        Err(thrown) => {
            raise(Failure::saying(
                text::format(Text::FailureThrew, &[call]),
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
        weight: Weight::Quiet,
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
    #[expect(
        clippy::disallowed_methods,
        reason = "this is the door every render or parse passes through"
    )]
    let answered = serde_json::to_string(value);
    match answered {
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
    #[expect(
        clippy::disallowed_methods,
        reason = "this is the door every render or parse passes through"
    )]
    let answered = serde_json::to_value(value);
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
    #[expect(
        clippy::disallowed_methods,
        reason = "this is the door every render or parse passes through"
    )]
    let answered = serde_json::from_value(held);
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

/// The value `text` reads as; text that will not read is raised as a failure
/// carrying `sentence` and answers `None`.
pub fn decoded<T: serde::de::DeserializeOwned>(sentence: Text, text: &str) -> Option<T> {
    #[expect(
        clippy::disallowed_methods,
        reason = "this is the door every render or parse passes through"
    )]
    let answered = serde_json::from_str(text);
    match answered {
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

/// `bytes` decoded as a JPEG; bytes that will not decode are raised as a
/// failure carrying `sentence` and answer `None`.
pub fn decoded_image(sentence: Text, bytes: &[u8]) -> Option<image::DynamicImage> {
    match image::load_from_memory_with_format(bytes, image::ImageFormat::Jpeg) {
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

/// `value` cast to `T`; a value of another type is raised as a failure naming
/// `call` and answers `None`.
pub fn cast<T: wasm_bindgen::JsCast>(
    call: &'static str,
    value: impl wasm_bindgen::JsCast,
) -> Option<T> {
    match value.dyn_into::<T>() {
        Ok(held) => Some(held),
        Err(other) => {
            raise(Failure::saying(
                text::format(Text::FailureThrew, &[call]),
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
        crate::boot::stopped(&text::format(Text::BootPanicked, &[&info.to_string()]));
        console_error_panic_hook::hook(info);
    }));
}

/// Every failure raised this session and the one shown above the view.
#[derive(Debug, Default)]
pub struct Log {
    raised: Vec<Failure>,
    showing: Option<Failure>,
}

impl Log {
    /// Records `failure` and shows it until it is dismissed; a `Quiet` report
    /// is recorded and shown above no view.
    pub fn took(&mut self, failure: Failure) {
        self.raised.insert(0, failure.clone());
        if failure.weight != Weight::Quiet {
            self.showing = Some(failure);
        }
    }

    /// The failure shown above the view now.
    pub fn showing(&self) -> Option<&Failure> {
        self.showing.as_ref()
    }

    pub fn dismiss(&mut self) {
        self.showing = None;
    }

    /// Every failure raised this session, newest first, dismissed included.
    pub fn raised(&self) -> &[Failure] {
        &self.raised
    }
}
