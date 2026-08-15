use jellium_protocol::{Capabilities, Decoding, Grant};

/// True when Media Source Extensions are present, which is what makes
/// adaptive streaming possible.
fn media_source() -> bool {
    web_sys::window().is_some_and(|window| {
        js_sys::Reflect::get(&window, &wasm_bindgen::JsValue::from_str("MediaSource"))
            .is_ok_and(|value| !value.is_undefined() && !value.is_null())
    })
}

/// The element every `canPlayType` probe is asked of.
fn element() -> Option<web_sys::HtmlVideoElement> {
    let element = crate::failure::called(
        "document.createElement",
        web_sys::window()?.document()?.create_element("video"),
    )?;
    crate::failure::cast::<web_sys::HtmlVideoElement>("HTMLVideoElement", element)
}

/// The grants `accepts` passes, gathered in the order the probe table lists
/// them.
fn decoded(accepts: impl Fn(&str) -> bool) -> Decoding {
    let mut decoding = Decoding::default();
    for probe in Capabilities::probes() {
        if !accepts(probe.mime) {
            continue;
        }
        match probe.grants {
            Grant::Container(container) => {
                if !decoding.containers.contains(&container) {
                    decoding.containers.push(container);
                }
            }
            Grant::Video(codec) => {
                if !decoding.video_codecs.contains(&codec) {
                    decoding.video_codecs.push(codec);
                }
            }
            Grant::Audio(codec) => {
                if !decoding.audio_codecs.contains(&codec) {
                    decoding.audio_codecs.push(codec);
                }
            }
        }
    }
    decoding
}

/// Runs every probe in `Capabilities::probes()` against a media element's
/// `canPlayType`, which is what a direct play is decoded by, and against
/// `MediaSource.isTypeSupported`, which is what hls.js feeds.
/// The adaptive set is empty when Media Source Extensions are absent.
pub fn probe() -> Capabilities {
    let adaptive = media_source();
    let media = element();

    Capabilities {
        media_source: adaptive,
        direct: decoded(|mime| {
            media
                .as_ref()
                .is_some_and(|element| !element.can_play_type(mime).is_empty())
        }),
        adaptive: if adaptive {
            decoded(web_sys::MediaSource::is_type_supported)
        } else {
            Decoding::default()
        },
    }
}
