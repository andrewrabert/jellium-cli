//! Every question the device profile port asks the running browser, asked where
//! `browserDeviceProfile.js` and `apphost.js` ask it.

use wasm_bindgen::JsValue;

use crate::browser::Browser;
use crate::failure::{self, Call, Cause, Failure};
use crate::settings::{Channels, Pixels};
use crate::text::Text;

/// A screen axis, reported as the count of pixels along it.
fn sized(count: i32) -> Option<Pixels> {
    let pixels = Pixels::of(count);
    if pixels.is_none() {
        failure::raise(Failure::told(
            Text::FailureScreenSize,
            Cause::Malformed {
                detail: count.to_string(),
            },
        ));
    }
    pixels
}

/// The answer `canPlayType` gives once the reference's `.replace(/no/, '')` has
/// run: an empty answer is false, and every other answer is true.
fn admitted(answer: &str) -> bool {
    !answer.replacen("no", "", 1).is_empty()
}

/// The elements the builder creates, asked the mimes the reference asks them.
pub struct Media {
    // reference: video-test-element — browserDeviceProfile.js:493
    video: web_sys::HtmlVideoElement,
    // reference: supports-audio-codec — browserDeviceProfile.js:204
    // reference: safari-opus-probe — browserDeviceProfile.js:499
    audio: web_sys::HtmlAudioElement,
}

fn created<T: wasm_bindgen::JsCast>(
    document: &web_sys::Document,
    tag: &str,
    call: Call,
) -> Option<T> {
    let element = failure::called(Call::DocumentCreateElement, document.create_element(tag))?;
    failure::cast::<T>(call, element)
}

impl Media {
    /// `None` when the document creates neither element.
    pub fn created() -> Option<Media> {
        let document = web_sys::window()?.document()?;
        Some(Media {
            video: created(&document, "video", Call::HtmlVideoElement)?,
            audio: created(&document, "audio", Call::HtmlAudioElement)?,
        })
    }

    /// `HTMLVideoElement.canPlayType`, read the way the reference reads it.
    pub fn video(&self, mime: &str) -> bool {
        admitted(&self.video.can_play_type(mime))
    }

    /// `HTMLAudioElement.canPlayType`, read the same way.
    pub fn audio(&self, mime: &str) -> bool {
        admitted(&self.audio.can_play_type(mime))
    }

    /// `videoTestElement.audioTracks != null`, which `web-sys` does not bind.
    pub fn audio_tracks(&self) -> bool {
        failure::called(
            Call::ReflectGet,
            js_sys::Reflect::get(&self.video, &JsValue::from_str("audioTracks")),
        )
        .is_some_and(|tracks| !tracks.is_null() && !tracks.is_undefined())
    }

    /// The element `canPlaySecondaryAudio` is handed, which only the
    /// differential hands it.
    #[cfg(test)]
    pub fn element(&self) -> &web_sys::HtmlVideoElement {
        &self.video
    }
}

/// `window.screen.width`, `window.screen.height` and `window.devicePixelRatio`,
/// as read.
// reference: app-host-screen — apphost.js:411-431
pub struct Screen {
    pub width: Pixels,
    pub height: Pixels,
    pub pixel_ratio: f64,
}

impl Screen {
    /// The larger of width and height, each multiplied by the pixel ratio and
    /// floored, which is what `apphost.js:62` reads.
    pub fn max_allowed_width(&self) -> Pixels {
        let scaled = |count: Pixels| (f64::from(count.count()) * self.pixel_ratio).floor();
        let larger = scaled(self.width).max(scaled(self.height));
        Pixels::of(larger as i32).unwrap_or(self.width)
    }
}

/// What `browserDeviceProfile.js` and `apphost.js` read from the running
/// browser.
pub struct Engine {
    // reference: supports-text-tracks — browserDeviceProfile.js:42-54
    text_tracks: bool,
    // reference: supports-canvas-2d — browserDeviceProfile.js:56-63
    canvas_2d: bool,
    // reference: can-play-hls-with-mse — browserDeviceProfile.js:92-95
    media_source: bool,
    // reference: get-speaker-count — browserDeviceProfile.js:412-430
    speakers: Option<Channels>,
    screen: Option<Screen>,
}

/// What `window` holds under `name`, and `None` when it holds nothing.
fn held(window: &web_sys::Window, name: &str) -> Option<JsValue> {
    let value = failure::called(
        Call::ReflectGet,
        js_sys::Reflect::get(window, &JsValue::from_str(name)),
    )?;
    (!value.is_null() && !value.is_undefined()).then_some(value)
}

/// `new (window.AudioContext ?? window.webkitAudioContext)().destination
/// .maxChannelCount`, read through `Reflect` because the generated binding
/// resolves the constructor before the page installs one.
fn speakers(window: &web_sys::Window) -> Option<Channels> {
    let constructor =
        held(window, "AudioContext").or_else(|| held(window, "webkitAudioContext"))?;
    let constructor = failure::cast::<js_sys::Function>(Call::AudioContextNew, constructor)?;
    let context = failure::called(
        Call::AudioContextNew,
        js_sys::Reflect::construct(&constructor, &js_sys::Array::new()),
    )?;
    let destination = failure::called(
        Call::ReflectGet,
        js_sys::Reflect::get(&context, &JsValue::from_str("destination")),
    )?;
    let count = failure::called(
        Call::ReflectGet,
        js_sys::Reflect::get(&destination, &JsValue::from_str("maxChannelCount")),
    )?;
    let count = failure::narrowed(Text::FailureSpeakerCount, count.as_f64()? as i64)?;
    Channels::of(count)
}

fn screen(window: &web_sys::Window) -> Option<Screen> {
    let screen = failure::called(Call::WindowScreen, window.screen())?;
    let width = failure::called(Call::ScreenWidth, screen.width())?;
    let height = failure::called(Call::ScreenHeight, screen.height())?;
    Some(Screen {
        width: sized(width)?,
        height: sized(height)?,
        pixel_ratio: window.device_pixel_ratio(),
    })
}

impl Engine {
    pub fn read() -> Engine {
        let mut engine = Engine {
            text_tracks: false,
            canvas_2d: false,
            media_source: false,
            speakers: None,
            screen: None,
        };
        let Some(window) = web_sys::window() else {
            return engine;
        };
        if let Some(document) = window.document() {
            engine.text_tracks =
                created::<web_sys::HtmlVideoElement>(&document, "video", Call::HtmlVideoElement)
                    .and_then(|video| {
                        failure::called(
                            Call::ReflectGet,
                            js_sys::Reflect::get(&video, &JsValue::from_str("textTracks")),
                        )
                    })
                    .is_some_and(|tracks| !tracks.is_null() && !tracks.is_undefined());
            engine.canvas_2d =
                created::<web_sys::HtmlCanvasElement>(&document, "canvas", Call::HtmlCanvasElement)
                    .and_then(|canvas| {
                        failure::called(Call::HtmlCanvasElementGetContext, canvas.get_context("2d"))
                    })
                    .flatten()
                    .is_some();
        }
        engine.media_source = held(&window, "MediaSource").is_some();
        engine.speakers = speakers(&window);
        engine.screen = screen(&window);
        engine
    }

    pub fn text_tracks(&self) -> bool {
        self.text_tracks
    }

    pub fn canvas_2d(&self) -> bool {
        self.canvas_2d
    }

    pub fn media_source(&self) -> bool {
        self.media_source
    }

    pub fn speakers(&self) -> Option<Channels> {
        self.speakers
    }

    /// The raw screen width the xbox branch reads, before any pixel ratio.
    // reference: xbox-screen-width — browserDeviceProfile.js:532
    pub fn screen_width(&self) -> Option<Pixels> {
        self.screen.as_ref().map(|screen| screen.width)
    }

    /// `None` on a tv, which is where the reference reads no screen.
    // reference: app-host-screen — apphost.js:411-431
    pub fn screen(&self, browser: &Browser) -> Option<&Screen> {
        if browser.tv {
            return None;
        }
        self.screen.as_ref()
    }
}
