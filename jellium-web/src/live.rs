use std::cell::RefCell;
use std::time::Duration;

use iced::Subscription;
use iced::futures::Stream;
use iced::futures::channel::mpsc;
use jellium_protocol::{Event, LIVE_PATH, Report};
use wasm_bindgen::prelude::*;

use crate::failure::{self, Cause, Failure};
use crate::text::{self, Text};

pub const BACKOFF: Duration = Duration::from_secs(1);

pub const BACKOFF_CAP: Duration = Duration::from_secs(30);

/// Where the event socket stands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// No attempt has failed yet, so nothing is shown.
    Connecting,
    Open,
    /// An attempt has failed, so the indicator is shown.
    Down,
}

/// What the socket told the application.
#[derive(Debug, Clone, PartialEq)]
pub enum Signal {
    Opened,
    Received(Event),
    /// `attempts` counts the attempts that have failed since the socket was
    /// last open; the loss of an open socket carries zero, so the first
    /// attempt is silent.
    Closed {
        attempts: u32,
    },
}

/// The socket, and how many attempts have failed since it was last open.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Link {
    pub state: State,
    pub attempts: u32,
}

impl Default for Link {
    fn default() -> Link {
        Link {
            state: State::Connecting,
            attempts: 0,
        }
    }
}

impl Link {
    /// Applies a signal.
    pub fn signalled(&mut self, signal: &Signal) {
        match signal {
            Signal::Opened => {
                self.state = State::Open;
                self.attempts = 0;
            }
            Signal::Closed { attempts } => {
                self.attempts = *attempts;
                self.state = State::Down;
            }
            Signal::Received(_) => {}
        }
    }

    /// True once an attempt has failed and the socket is not open.
    pub fn down(&self) -> bool {
        self.state == State::Down && self.attempts > 0
    }
}

/// The delay before the attempt that follows `attempts` failed ones: `BACKOFF`
/// doubling to `BACKOFF_CAP`.
pub fn backoff(attempts: u32) -> Duration {
    let doubled = BACKOFF
        .checked_mul(1u32.checked_shl(attempts).unwrap_or(u32::MAX))
        .unwrap_or(BACKOFF_CAP);
    doubled.min(BACKOFF_CAP)
}

thread_local! {
    /// The one channel the socket's callbacks write into, held for the life of
    /// the page so a reconnect keeps reporting into the same subscription.
    static CHANNEL: (mpsc::UnboundedSender<Signal>, RefCell<mpsc::UnboundedReceiver<Signal>>) = {
        let (sender, receiver) = mpsc::unbounded();
        (sender, RefCell::new(receiver))
    };

    /// The open socket and the callbacks it holds, kept alive while it is.
    static HELD: RefCell<Option<Held>> = const { RefCell::new(None) };

    /// How many attempts have failed since the socket was last open; the loss
    /// of an open socket is not one of them.
    static ATTEMPTS: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };

    /// True once the socket now held has opened, which tells the loss of a
    /// live socket from an attempt that never connected.
    static OPENED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };

    /// False once `disconnect` has run, which stops every retry.
    static WANTED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// One open socket and the closures the browser calls on it.
struct Held {
    socket: web_sys::WebSocket,
    _opened: Closure<dyn FnMut()>,
    _message: Closure<dyn FnMut(web_sys::MessageEvent)>,
    _closed: Closure<dyn FnMut(web_sys::CloseEvent)>,
    _errored: Closure<dyn FnMut(web_sys::Event)>,
}

fn raise(signal: Signal) {
    CHANNEL.with(|(sender, _)| {
        if sender.unbounded_send(signal).is_err() {
            web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(
                "the live signal channel is closed",
            ));
        }
    });
}

/// `LIVE_PATH` on the page's own origin, with the websocket scheme its scheme
/// implies.
fn socket_url() -> Option<String> {
    let Some(window) = web_sys::window() else {
        failure::raise(Failure::told(
            Text::FailureLiveNoWindow,
            Cause::Browser {
                detail: "window() answered none".to_owned(),
            },
        ));
        return None;
    };
    let protocol = failure::called("location.protocol", window.location().protocol())?;
    let host = failure::called("location.host", window.location().host())?;
    let scheme = if protocol == "https:" { "wss" } else { "ws" };
    Some(format!("{scheme}://{host}{LIVE_PATH}"))
}

/// Schedules the next attempt after a close, unless `disconnect` ran.
fn retry() {
    if !WANTED.with(std::cell::Cell::get) {
        return;
    }
    let attempts = if OPENED.with(|held| held.replace(false)) {
        0
    } else {
        ATTEMPTS.with(std::cell::Cell::get).saturating_add(1)
    };
    ATTEMPTS.with(|held| held.set(attempts));
    raise(Signal::Closed { attempts });

    let Some(window) = web_sys::window() else {
        failure::raise(Failure::told(
            Text::FailureLiveNoWindow,
            Cause::Browser {
                detail: "window() answered none".to_owned(),
            },
        ));
        return;
    };
    let again = Closure::once_into_js(open);
    if failure::called(
        "setTimeout",
        window.set_timeout_with_callback_and_timeout_and_arguments_0(
            again.as_ref().unchecked_ref(),
            backoff(attempts).as_millis() as i32,
        ),
    )
    .is_none()
    {
        failure::raise(Failure::told(
            Text::FailureLiveNoRetry,
            Cause::Browser {
                detail: "setTimeout was refused".to_owned(),
            },
        ));
    }
}

/// A close and an error both mean the same thing here: the socket is gone.
fn dropped(closed: Option<(u16, String)>) {
    let held = HELD.with(|held| held.borrow_mut().take());
    if held.is_none() {
        return;
    }
    if let Some((code, reason)) = closed {
        failure::record(&Failure::saying(
            text::format(Text::FailureSocketClosed, &[&code.to_string(), &reason]),
            Cause::Closed { code, reason },
        ));
    }
    retry();
}

fn open() {
    if !WANTED.with(std::cell::Cell::get) {
        return;
    }
    let Some(url) = socket_url() else {
        failure::raise(Failure::told(
            Text::FailureLiveNoUrl,
            Cause::Browser {
                detail: "the page named no socket address".to_owned(),
            },
        ));
        return;
    };
    let socket = match web_sys::WebSocket::new(&url) {
        Ok(socket) => socket,
        Err(thrown) => {
            failure::called::<()>("WebSocket.new", Err(thrown));
            retry();
            return;
        }
    };

    let opened = Closure::<dyn FnMut()>::new(move || {
        ATTEMPTS.with(|held| held.set(0));
        OPENED.with(|held| held.set(true));
        raise(Signal::Opened);
    });
    socket.set_onopen(Some(opened.as_ref().unchecked_ref()));

    let message =
        Closure::<dyn FnMut(web_sys::MessageEvent)>::new(move |received: web_sys::MessageEvent| {
            let Some(text) = received.data().as_string() else {
                failure::raise(Failure::told(
                    Text::FailureLiveFrame,
                    Cause::Malformed {
                        detail: "the frame carried no text".to_owned(),
                    },
                ));
                return;
            };
            if let Some(event) = failure::decoded::<Event>(Text::FailureLiveFrame, &text) {
                raise(Signal::Received(event));
            }
        });
    socket.set_onmessage(Some(message.as_ref().unchecked_ref()));

    let closed =
        Closure::<dyn FnMut(web_sys::CloseEvent)>::new(move |event: web_sys::CloseEvent| {
            dropped(Some((event.code(), event.reason())));
        });
    socket.set_onclose(Some(closed.as_ref().unchecked_ref()));

    let errored = Closure::<dyn FnMut(web_sys::Event)>::new(move |_: web_sys::Event| dropped(None));
    socket.set_onerror(Some(errored.as_ref().unchecked_ref()));

    HELD.with(|held| {
        *held.borrow_mut() = Some(Held {
            socket,
            _opened: opened,
            _message: message,
            _closed: closed,
            _errored: errored,
        });
    });
}

/// Opens `LIVE_PATH` on the page's own origin and keeps it open: a close is
/// retried after `backoff`, and the count resets when it opens.
pub fn connect() {
    if WANTED.with(std::cell::Cell::get) {
        return;
    }
    WANTED.with(|held| held.set(true));
    ATTEMPTS.with(|held| held.set(0));
    OPENED.with(|held| held.set(false));
    open();
}

/// Sends one report; a report sent while the socket is not open is dropped.
pub fn send(report: &Report) {
    let Some(frame) = failure::rendered(Text::FailureLiveReport, report) else {
        return;
    };
    HELD.with(|held| {
        if let Some(held) = held.borrow().as_ref()
            && held.socket.ready_state() == web_sys::WebSocket::OPEN
        {
            failure::called("WebSocket.send", held.socket.send_with_str(&frame));
        }
    });
}

/// Closes the socket and stops retrying.
pub fn disconnect() {
    WANTED.with(|held| held.set(false));
    if let Some(held) = HELD.with(|held| held.borrow_mut().take()) {
        held.socket.set_onclose(None);
        held.socket.set_onerror(None);
        failure::called("WebSocket.close", held.socket.close());
    }
    ATTEMPTS.with(|held| held.set(0));
    OPENED.with(|held| held.set(false));
}

struct Signals;

impl Stream for Signals {
    type Item = Signal;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        context: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Signal>> {
        CHANNEL.with(|(_, receiver)| {
            std::pin::Pin::new(&mut *receiver.borrow_mut()).poll_next(context)
        })
    }
}

/// The socket's signals, delivered through the callbacks it holds.
pub fn signals() -> Subscription<Signal> {
    Subscription::run(|| Signals)
}
