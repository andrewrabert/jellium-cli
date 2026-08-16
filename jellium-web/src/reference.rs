//! The sliced reference's exports, bound for the differentials that run them.
//!
//! Nothing outside a test reaches this module: the reference is what the port
//! is measured against, never what the port is built from.

use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module = "/reference/jellyfin-web.mjs")]
unsafe extern "C" {
    #[wasm_bindgen(js_name = detectBrowser)]
    pub fn detect_browser(user_agent: &str) -> JsValue;

    #[wasm_bindgen(js_name = getBaseProfileOptions)]
    pub fn base_profile_options(browser: &JsValue, item: &JsValue) -> JsValue;

    #[wasm_bindgen(js_name = getDeviceProfile)]
    pub fn device_profile(
        browser: &JsValue,
        app_settings: &JsValue,
        user_settings: &JsValue,
        item: &JsValue,
    ) -> js_sys::Promise;

    #[wasm_bindgen(js_name = canPlaySecondaryAudio)]
    pub fn can_play_secondary_audio(browser: &JsValue, element: &web_sys::HtmlVideoElement)
    -> bool;

    #[wasm_bindgen(js_name = enableHlsJsPlayer)]
    pub fn enable_hls_js_player(
        browser: &JsValue,
        run_time_ticks: &JsValue,
        media_type: &str,
    ) -> bool;
}

/// The `window`, `document` and `navigator` both sides read, installed from one
/// authored spec that names no browser.
#[wasm_bindgen(module = "/reference/environment.mjs")]
unsafe extern "C" {
    fn install(spec: &JsValue);
}

/// Installs `spec`, which crosses into JavaScript as the object it renders to.
pub fn installed<T: serde::Serialize>(spec: &T) {
    let rendered = crate::failure::rendered(crate::text::Text::FailureStored, spec)
        .expect("the environment spec renders");
    let value = js_sys::JSON::parse(&rendered).expect("the environment spec parses");
    install(&value);
}

/// The six user agents every differential runs beside the live one: Firefox on
/// Linux, Chromium on Linux, Edge Chromium on Windows, Safari on macOS, webOS 6
/// and Tizen 6.
pub const AGENTS: [&str; 6] = [
    "Mozilla/5.0 (X11; Linux x86_64; rv:128.0) Gecko/20100101 Firefox/128.0",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36 Edg/128.0.0.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.5 Safari/605.1.15",
    "Mozilla/5.0 (Web0S; Linux/SmartTV) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/79.0.3945.79 Safari/537.36 WebAppManager",
    "Mozilla/5.0 (SMART-TV; LINUX; Tizen 6.0) AppleWebKit/537.36 (KHTML, like Gecko) 76.0.3809.146/6.0 TV Safari/537.36",
];
