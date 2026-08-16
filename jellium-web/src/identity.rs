//! What this browser announces about itself, minted once and kept in its own
//! preference entry.

use jellium_protocol::Identity;

use crate::browser::Browser;
use crate::failure::{self, Call};
use crate::prefs::{self, Entry};

/// The user agent and the epoch millisecond, joined and base64'd the way
/// `btoa` encodes them, with every `=` written as `1`.
/// A browser answering no `btoa` is left with the millisecond alone, which is
/// the reference's own fallback.
// reference: generate-device-id — apphost.js:124-134
fn minted() -> String {
    let stamp = js_sys::Date::now();
    let Some(window) = web_sys::window() else {
        return format!("{stamp}");
    };
    let agent = failure::called(Call::NavigatorUserAgent, window.navigator().user_agent())
        .unwrap_or_default();
    match failure::called(Call::WindowBtoa, window.btoa(&format!("{agent}|{stamp}"))) {
        Some(encoded) => encoded.replace('=', "1"),
        None => format!("{stamp}"),
    }
}

/// The identity this browser presents: the name it detects itself under and
/// the id it minted the first time it was asked and has kept since.
// reference: get-device-id — apphost.js:136-148
pub fn held(browser: &Browser) -> Identity {
    let device_id = match prefs::stored::<String>(Entry::DeviceId) {
        Some(held) if !held.is_empty() => held,
        _ => {
            let minted = minted();
            prefs::store(Entry::DeviceId, &minted);
            minted
        }
    };
    Identity {
        device: browser.device_name(),
        device_id,
    }
}
