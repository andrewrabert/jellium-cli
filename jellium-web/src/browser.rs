//! What this browser detects itself as, ported from `src/scripts/browser.js`.
//!
//! The port answers the object `detectBrowser` builds: one entry per assignment
//! site, in the reference's assignment order, an entry the run does not assign
//! absent, and absent distinct from false.

use serde::ser::SerializeMap;

use crate::failure::{self, Call};
use crate::text::Text;

/// The token `uaMatch` names, which the reference makes the object's first key.
// reference: ua-match — browser.js:188-243
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Matched {
    Edg,
    Edga,
    Edgios,
    Edge,
    Titanos,
    Opera,
    Chrome,
    Safari,
    Firefox,
    Mozilla,
}

impl Matched {
    /// The key the reference assigns for this token.
    fn key(self) -> &'static str {
        match self {
            Matched::Edg => "edg",
            Matched::Edga => "edga",
            Matched::Edgios => "edgios",
            Matched::Edge => "edge",
            Matched::Titanos => "titanos",
            Matched::Opera => "opera",
            Matched::Chrome => "chrome",
            Matched::Safari => "safari",
            Matched::Firefox => "firefox",
            Matched::Mozilla => "mozilla",
        }
    }
}

/// The platform token `uaMatch` names, which the reference makes a key of the
/// object, and which an `edge` token clears.
// reference: ua-match-platform — browser.js:207-217
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Ipad,
    Iphone,
    Windows,
    Android,
    Titanos,
}

impl Platform {
    fn key(self) -> &'static str {
        match self {
            Platform::Ipad => "ipad",
            Platform::Iphone => "iphone",
            Platform::Windows => "windows",
            Platform::Android => "android",
            Platform::Titanos => "titanos",
        }
    }
}

/// A version the reference computes as a JavaScript number, which prints with
/// no fractional part when it is integral.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Version(f64);

impl Version {
    pub fn of(value: f64) -> Version {
        Version(value)
    }

    pub fn value(self) -> f64 {
        self.0
    }
}

/// Prints the way `JSON.stringify` prints a number: 6, not 6.0.
impl serde::Serialize for Version {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let whole = self.0.trunc();
        if self.0 == whole && whole.abs() < 9.007_199_254_740_992e15 {
            serializer.serialize_i64(whole as i64)
        } else {
            serializer.serialize_f64(self.0)
        }
    }
}

/// The iOS version the reference reads off `navigator`.
// reference: ios-version — browser.js:78-98
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IosVersion {
    /// The empty list the reference leaves when `navigator.platform` names no
    /// Apple device or `navigator.appVersion` carries no version.
    Unmatched,
    Detected(Version),
}

impl serde::Serialize for IosVersion {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            IosVersion::Unmatched => serializer.collect_seq(std::iter::empty::<u8>()),
            IosVersion::Detected(version) => version.serialize(serializer),
        }
    }
}

/// What this browser detects itself as.
// reference: detect-browser — browser.js:245-346
#[derive(Debug, Clone, PartialEq)]
pub struct Browser {
    /// The object's first key, cleared by the branches that delete it.
    matched: Option<Matched>,
    pub version: Option<String>,
    pub version_major: Option<u32>,
    platform: Option<Platform>,
    /// `browser.edg || browser.edga || browser.edgios`, which is true or
    /// absent.
    pub edge_chromium: bool,
    /// The webkit fallback at :262-264, whose key sits here only when `matched`
    /// is not `Safari`, and which the webOS, Tizen, TitanOS and Vega branches
    /// delete.
    webkit_fallback: bool,
    pub osx: bool,
    /// The iPadOS workaround at :272, whose key sits here only when `platform`
    /// is not `Ipad`.
    ipad_workaround: bool,
    /// Cleared by the Vega branch that deletes it.
    pub mobile: bool,
    pub ps4: bool,
    pub xbox_one: bool,
    pub animate: bool,
    pub hisense: bool,
    pub tizen: bool,
    pub vega: bool,
    pub vidaa: bool,
    pub web0s: bool,
    pub tv: bool,
    /// Cleared by the TitanOS branch that deletes it.
    pub opera_tv: bool,
    /// Absent when neither `edge` nor `edgeChromium` is set.
    pub edge_uwp: Option<bool>,
    /// Absent unless the webOS branch ran, and absent inside it for NetCast,
    /// which the reference leaves undefined.
    // reference: web0s-version — browser.js:101-147
    pub web0s_version: Option<Version>,
    // reference: tizen-version — browser.js:302-303
    pub tizen_version: Option<Version>,
    /// Absent unless the branch that assigns it ran.
    pub orsay: Option<bool>,
    pub slow: bool,
    pub touch: bool,
    pub keyboard: bool,
    pub ios: bool,
    pub ios_version: Option<IosVersion>,
}

/// What `browser.js` reads outside the user agent.
// reference: ios-version — browser.js:78-98
#[derive(Debug, Clone, PartialEq)]
pub struct Runtime {
    pub user_agent: String,
    pub platform: String,
    pub app_version: String,
    pub max_touch_points: u32,
    pub has_touch_start: bool,
    pub tizen_global: bool,
    /// `document.documentElement.animate != null`.
    pub animates: bool,
}

/// A character `\w` or `.` admits, which is what every version run is made of.
fn version_char(value: char) -> bool {
    value.is_ascii_alphanumeric() || value == '_' || value == '.'
}

/// The run of version characters `text` opens with, and `None` when it opens
/// with none, which is what the `+` in `([\w.]+)` refuses.
fn version_run(text: &str) -> Option<&str> {
    let taken = text
        .find(|value: char| !version_char(value))
        .unwrap_or(text.len());
    if taken == 0 {
        None
    } else {
        Some(&text[..taken])
    }
}

/// What `/(token)[ /]([\w.]+)/` captures in `text`, scanning from the left.
fn tokened(text: &str, token: &str) -> Option<String> {
    let mut from = 0;
    while let Some(offset) = text[from..].find(token) {
        let at = from + offset + token.len();
        let rest = &text[at..];
        if rest.starts_with([' ', '/'])
            && let Some(run) = version_run(&rest[1..])
        {
            return Some(run.to_owned());
        }
        from += offset + 1;
    }
    None
}

/// What `/(mozilla)(?:.*? rv:([\w.]+)|)/` captures in `text`: the `rv:` run when
/// one follows the first `mozilla` on the same line, and `None` when the empty
/// alternative is taken.
fn mozilla_version(text: &str) -> Option<String> {
    let at = text.find("mozilla")? + "mozilla".len();
    let line = match text[at..].find('\n') {
        Some(end) => &text[at..at + end],
        None => &text[at..],
    };
    let mut from = 0;
    while let Some(offset) = line[from..].find(" rv:") {
        let after = from + offset + " rv:".len();
        if let Some(run) = version_run(&line[after..]) {
            return Some(run.to_owned());
        }
        from += offset + 1;
    }
    None
}

/// What `uaMatch` answers: the browser token, the version, the platform token
/// and the major version.
// reference: ua-match — browser.js:188-243
struct Match {
    matched: Option<Matched>,
    version: String,
    platform: Option<Platform>,
    version_major: u32,
}

/// The leading integer `parseInt(text, 10)` reads, and `None` where it answers
/// `NaN`.
fn leading_integer(text: &str) -> Option<u32> {
    let read = digits(text);
    if read.is_empty() {
        return None;
    }
    // A run too long for the narrower type is a number `parseInt` reads and
    // every comparison the reference makes against it answers the same way.
    match read.parse() {
        Ok(read) => Some(read),
        Err(_) => Some(u32::MAX),
    }
}

// reference: ua-match — browser.js:188-243
fn ua_match(agent: &str) -> Match {
    let trimmed = agent.replacen("motorola edge", "", 1);
    let ua = trimmed.trim();

    let tokens = [
        ("edg", Matched::Edg),
        ("edga", Matched::Edga),
        ("edgios", Matched::Edgios),
        ("edge", Matched::Edge),
        ("titanos", Matched::Titanos),
        ("opera", Matched::Opera),
        ("opr", Matched::Opera),
        ("chrome", Matched::Chrome),
        ("safari", Matched::Safari),
        ("firefox", Matched::Firefox),
    ];
    let mut matched = None;
    let mut captured = None;
    for (token, name) in tokens {
        if let Some(version) = tokened(ua, token) {
            matched = Some(name);
            captured = Some(version);
            break;
        }
    }
    if matched.is_none() && !ua.contains("compatible") && ua.contains("mozilla") {
        matched = Some(Matched::Mozilla);
        captured = mozilla_version(ua);
    }

    // reference: ua-match-platform — browser.js:207-217
    let mut platform = [
        ("ipad", Platform::Ipad),
        ("iphone", Platform::Iphone),
        ("windows", Platform::Windows),
        ("android", Platform::Android),
        ("titanos", Platform::Titanos),
    ]
    .into_iter()
    .find(|(token, _)| ua.contains(token))
    .map(|(_, name)| name);

    if matched == Some(Matched::Edge) {
        platform = None;
    }

    let version = tokened(ua, "version")
        .or(captured)
        .unwrap_or_else(|| "0".to_owned());
    let version_major = version
        .split('.')
        .next()
        .and_then(leading_integer)
        .unwrap_or(0);

    Match {
        matched,
        version,
        platform,
        version_major,
    }
}

// reference: detect-browser — browser.js:245-346
fn is_web0s(agent: &str) -> bool {
    agent.contains("netcast") || agent.contains("web0s")
}

// reference: detect-browser — browser.js:245-346
fn is_tv(agent: &str) -> bool {
    if agent.contains("oculusbrowser") {
        return false;
    }
    ["tv", "samsungbrowser", "viera", "titanos"]
        .into_iter()
        .any(|term| agent.contains(term))
        || is_web0s(agent)
}

// reference: detect-browser — browser.js:245-346
fn is_mobile(agent: &str) -> bool {
    [
        "mobi",
        "ipad",
        "iphone",
        "ipod",
        "silk",
        "gt-p1000",
        "nexus 7",
        "kindle fire",
        "opera mini",
    ]
    .into_iter()
    .any(|term| agent.contains(term))
}

/// The webOS version the reference reads off the web engine version.
// reference: web0s-version — browser.js:101-147
fn web0s_version(chrome: bool, version_major: Option<u32>, live_agent: &str) -> Option<Version> {
    if chrome {
        if live_agent.contains("netcast") {
            return None;
        }
        let major = version_major?;
        for (floor, answer) in [
            (94, 23.0),
            (87, 22.0),
            (79, 6.0),
            (68, 5.0),
            (53, 4.0),
            (38, 3.0),
            (34, 2.0),
            (26, 1.0),
        ] {
            if major >= floor {
                return Some(Version::of(answer));
            }
        }
        return None;
    }
    let major = version_major?;
    if major >= 538 {
        return Some(Version::of(2.0));
    }
    if major >= 537 {
        return Some(Version::of(1.0));
    }
    None
}

/// The run of digits `text` opens with.
fn digits(text: &str) -> &str {
    let taken = text
        .find(|value: char| !value.is_ascii_digit())
        .unwrap_or(text.len());
    &text[..taken]
}

/// The Tizen version `/Tizen (\d+).(\d+)/` reads off the user agent as it was
/// handed in, which the reference does not lowercase.
// reference: tizen-version — browser.js:302-303
fn tizen_version(agent: &str) -> Option<Version> {
    let mut from = 0;
    while let Some(offset) = agent[from..].find("Tizen ") {
        let at = from + offset + "Tizen ".len();
        let rest = &agent[at..];
        let run = digits(rest).len();
        // `.` between the two groups matches any character, so the first group
        // gives back digits until the second group has one of its own.
        for taken in (1..=run).rev() {
            let after = &rest[taken..];
            let Some(separator) = after.chars().next() else {
                continue;
            };
            if separator == '\n' {
                continue;
            }
            let minor = digits(&after[separator.len_utf8()..]);
            if minor.is_empty() {
                continue;
            }
            if let (Ok(major), Ok(minor)) = (rest[..taken].parse::<i32>(), minor.parse::<i32>()) {
                return Some(Version::of(f64::from(major) + f64::from(minor) / 10.0));
            }
        }
        from += offset + 1;
    }
    None
}

/// The iOS version the reference reads off `navigator.platform` and
/// `navigator.appVersion`.
// reference: ios-version — browser.js:78-98
fn ios_version(runtime: &Runtime) -> IosVersion {
    let apple = ["iPhone", "iPod", "iPad", "MacIntel"]
        .into_iter()
        .any(|name| runtime.platform.contains(name));
    if !apple {
        return IosVersion::Unmatched;
    }
    if let Some(parts) = os_version(&runtime.app_version) {
        return IosVersion::Detected(Version::of(f64::from(parts.0) + f64::from(parts.1) / 10.0));
    }
    if let Some(major) = version_after(&runtime.app_version, "Version/") {
        return IosVersion::Detected(Version::of(f64::from(major)));
    }
    IosVersion::Unmatched
}

/// What `/OS (\d+)_(\d+)_?(\d+)?/` captures: the major and the minor.
fn os_version(text: &str) -> Option<(u32, u32)> {
    let mut from = 0;
    while let Some(offset) = text[from..].find("OS ") {
        let at = from + offset + "OS ".len();
        let rest = &text[at..];
        let major = digits(rest);
        if !major.is_empty()
            && let Some(after) = rest[major.len()..].strip_prefix('_')
        {
            let minor = digits(after);
            if !minor.is_empty()
                && let Some(major) = leading_integer(major)
                && let Some(minor) = leading_integer(minor)
            {
                return Some((major, minor));
            }
        }
        from += offset + 1;
    }
    None
}

/// What `/Version\/(\d+)/` captures.
fn version_after(text: &str, token: &str) -> Option<u32> {
    let mut from = 0;
    while let Some(offset) = text[from..].find(token) {
        let at = from + offset + token.len();
        if let Some(read) = leading_integer(&text[at..]) {
            return Some(read);
        }
        from += offset + 1;
    }
    None
}

impl Browser {
    // reference: detect-browser — browser.js:245-346
    pub fn detect(runtime: &Runtime) -> Browser {
        let normalized = runtime.user_agent.to_lowercase();
        let named = ua_match(&normalized);

        let edge_chromium = matches!(
            named.matched,
            Some(Matched::Edg | Matched::Edga | Matched::Edgios)
        );
        let chrome = named.matched == Some(Matched::Chrome);
        let edge = named.matched == Some(Matched::Edge);
        let opera = named.matched == Some(Matched::Opera);
        let webkit_fallback =
            !chrome && !edge_chromium && !edge && !opera && normalized.contains("webkit");

        let osx = normalized.contains("mac os x");
        let ipad = named.platform == Some(Platform::Ipad);
        let iphone = named.platform == Some(Platform::Iphone);
        let ipad_workaround = osx && !iphone && !ipad && runtime.max_touch_points > 1;

        let ps4 = normalized.contains("playstation 4");
        let xbox_one = normalized.contains("xbox");
        let vega = normalized.contains("kepler");
        let tv = ps4 || vega || xbox_one || is_tv(&normalized);
        let web0s = is_web0s(&normalized);
        let tizen = normalized.contains("tizen") || runtime.tizen_global;

        let mut browser = Browser {
            matched: named.matched,
            version: named.matched.map(|_| named.version),
            version_major: named.matched.map(|_| named.version_major),
            platform: named.platform,
            edge_chromium,
            webkit_fallback,
            osx,
            ipad_workaround,
            mobile: is_mobile(&normalized),
            ps4,
            xbox_one,
            animate: runtime.animates,
            hisense: normalized.contains("hisense"),
            tizen,
            vega,
            vidaa: normalized.contains("vidaa"),
            web0s,
            tv,
            opera_tv: tv && normalized.contains("opr/"),
            edge_uwp: (edge || edge_chromium)
                .then(|| normalized.contains("msapphost") || normalized.contains("webview")),
            web0s_version: None,
            tizen_version: None,
            orsay: None,
            slow: false,
            touch: false,
            keyboard: false,
            ios: false,
            ios_version: None,
        };

        if web0s {
            browser.web0s_version =
                web0s_version(chrome, browser.version_major, &runtime.user_agent);
            browser.forget(Matched::Chrome);
            browser.forget(Matched::Safari);
        } else if tizen {
            browser.tizen_version = tizen_version(&runtime.user_agent);
            browser.forget(Matched::Chrome);
            browser.forget(Matched::Safari);
        } else if browser.titanos() {
            browser.opera_tv = false;
            browser.forget(Matched::Safari);
        } else if vega {
            browser.forget(Matched::Chrome);
            browser.forget(Matched::Safari);
            browser.mobile = false;
        } else {
            browser.orsay = Some(normalized.contains("smarthub"));
        }

        browser.slow = browser.mobile || browser.tv;
        browser.touch = runtime.has_touch_start || runtime.max_touch_points > 0;
        browser.keyboard = browser.touch
            || browser.xbox_one
            || browser.ps4
            || browser.edge_uwp == Some(true)
            || browser.tv;
        browser.ios = browser.ipad() || browser.iphone();
        if browser.ios {
            browser.ios_version = Some(ios_version(runtime));
        }
        browser
    }

    /// Deletes the key `name` assigns, wherever the run put it.
    fn forget(&mut self, name: Matched) {
        if self.matched == Some(name) {
            self.matched = None;
        }
        if name == Matched::Safari {
            self.webkit_fallback = false;
        }
    }

    pub fn chrome(&self) -> bool {
        self.matched == Some(Matched::Chrome)
    }

    pub fn firefox(&self) -> bool {
        self.matched == Some(Matched::Firefox)
    }

    pub fn safari(&self) -> bool {
        self.matched == Some(Matched::Safari) || self.webkit_fallback
    }

    pub fn titanos(&self) -> bool {
        self.matched == Some(Matched::Titanos) || self.platform == Some(Platform::Titanos)
    }

    pub fn ipad(&self) -> bool {
        self.platform == Some(Platform::Ipad) || self.ipad_workaround
    }

    pub fn iphone(&self) -> bool {
        self.platform == Some(Platform::Iphone)
    }

    pub fn edge(&self) -> bool {
        self.matched == Some(Matched::Edge)
    }

    pub fn opera(&self) -> bool {
        self.matched == Some(Matched::Opera)
    }

    pub fn android(&self) -> bool {
        self.platform == Some(Platform::Android)
    }

    pub fn windows(&self) -> bool {
        self.platform == Some(Platform::Windows)
    }

    /// The name this browser announces itself under: the `BrowserName` entry
    /// for the first flag that holds, `Web Browser` when none does, and the
    /// device suffix the reference appends after it.
    // reference: get-device-name — apphost.js:151-172
    pub fn device_name(&self) -> String {
        let named = [
            (self.tizen, "Samsung Smart TV"),
            (self.web0s, "LG Smart TV"),
            (self.titanos(), "Titan OS"),
            (self.vega, "Vega OS"),
            (self.opera_tv, "Opera TV"),
            (self.xbox_one, "Xbox One"),
            (self.ps4, "Sony PS4"),
            (self.chrome(), "Chrome"),
            (self.edge_chromium, "Edge Chromium"),
            (self.edge(), "Edge"),
            (self.firefox(), "Firefox"),
            (self.opera(), "Opera"),
            (self.safari(), "Safari"),
        ];
        let mut name = named
            .into_iter()
            .find_map(|(holds, named)| holds.then_some(named))
            .unwrap_or("Web Browser")
            .to_owned();
        if self.ipad() {
            name.push_str(" iPad");
        } else if self.iphone() {
            name.push_str(" iPhone");
        } else if self.android() {
            name.push_str(" Android");
        }
        name
    }
}

/// Emits every present entry in the reference's assignment order, each entry's
/// key being the one `detectBrowser` assigns.
impl serde::Serialize for Browser {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        if let Some(matched) = self.matched {
            map.serialize_entry(matched.key(), &true)?;
        }
        if let Some(version) = &self.version {
            map.serialize_entry("version", version)?;
        }
        if let Some(major) = self.version_major {
            map.serialize_entry("versionMajor", &major)?;
        }
        if let Some(platform) = self.platform
            && Some(platform.key()) != self.matched.map(Matched::key)
        {
            map.serialize_entry(platform.key(), &true)?;
        }
        if self.edge_chromium {
            map.serialize_entry("edgeChromium", &true)?;
        }
        if self.webkit_fallback && self.matched != Some(Matched::Safari) {
            map.serialize_entry("safari", &true)?;
        }
        map.serialize_entry("osx", &self.osx)?;
        if self.ipad_workaround && self.platform != Some(Platform::Ipad) {
            map.serialize_entry("ipad", &true)?;
        }
        if self.mobile {
            map.serialize_entry("mobile", &true)?;
        }
        map.serialize_entry("ps4", &self.ps4)?;
        map.serialize_entry("xboxOne", &self.xbox_one)?;
        map.serialize_entry("animate", &self.animate)?;
        map.serialize_entry("hisense", &self.hisense)?;
        map.serialize_entry("tizen", &self.tizen)?;
        map.serialize_entry("vega", &self.vega)?;
        map.serialize_entry("vidaa", &self.vidaa)?;
        map.serialize_entry("web0s", &self.web0s)?;
        map.serialize_entry("tv", &self.tv)?;
        map.serialize_entry("operaTv", &self.opera_tv)?;
        if let Some(uwp) = self.edge_uwp {
            map.serialize_entry("edgeUwp", &uwp)?;
        }
        if self.web0s {
            if let Some(version) = self.web0s_version {
                map.serialize_entry("web0sVersion", &version)?;
            }
        } else if self.tizen {
            if let Some(version) = self.tizen_version {
                map.serialize_entry("tizenVersion", &version)?;
            }
        } else if let Some(orsay) = self.orsay {
            map.serialize_entry("orsay", &orsay)?;
        }
        if self.slow {
            map.serialize_entry("slow", &true)?;
        }
        if self.touch {
            map.serialize_entry("touch", &true)?;
        }
        map.serialize_entry("keyboard", &self.keyboard)?;
        if self.ios {
            map.serialize_entry("iOS", &true)?;
        }
        if let Some(version) = self.ios_version {
            map.serialize_entry("iOSVersion", &version)?;
        }
        map.end()
    }
}

/// What `window` holds under `name`, and `None` when it holds nothing.
fn held(window: &web_sys::Window, name: &str) -> Option<wasm_bindgen::JsValue> {
    let value = failure::called(
        Call::ReflectGet,
        js_sys::Reflect::get(window, &wasm_bindgen::JsValue::from_str(name)),
    )?;
    (!value.is_null() && !value.is_undefined()).then_some(value)
}

impl Runtime {
    /// Reads `navigator`, `window` and `document`.
    pub fn probe() -> Runtime {
        let mut runtime = Runtime {
            user_agent: String::new(),
            platform: String::new(),
            app_version: String::new(),
            max_touch_points: 0,
            has_touch_start: false,
            tizen_global: false,
            animates: false,
        };
        let Some(window) = web_sys::window() else {
            return runtime;
        };
        let navigator = window.navigator();
        if let Some(agent) = failure::called(Call::NavigatorUserAgent, navigator.user_agent()) {
            runtime.user_agent = agent;
        }
        if let Some(platform) = failure::called(Call::NavigatorPlatform, navigator.platform()) {
            runtime.platform = platform;
        }
        if let Some(version) = failure::called(Call::NavigatorAppVersion, navigator.app_version()) {
            runtime.app_version = version;
        }
        if let Some(points) =
            failure::narrowed(Text::FailureTouchPoints, navigator.max_touch_points())
        {
            runtime.max_touch_points = points;
        }
        if let Some(has) = failure::called(
            Call::ReflectHas,
            js_sys::Reflect::has(&window, &wasm_bindgen::JsValue::from_str("ontouchstart")),
        ) {
            runtime.has_touch_start = has;
        }
        runtime.tizen_global = held(&window, "tizen").is_some();
        runtime.animates = window
            .document()
            .and_then(|document| document.document_element())
            .and_then(|element| {
                failure::called(
                    Call::ReflectGet,
                    js_sys::Reflect::get(&element, &wasm_bindgen::JsValue::from_str("animate")),
                )
            })
            .is_some_and(|value| !value.is_null() && !value.is_undefined());
        runtime
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reference;
    use wasm_bindgen_test::wasm_bindgen_test;

    /// Installs `agent` into the environment both sides read, then reads it back
    /// through `Runtime::probe`, which is the only way the port ever varies the
    /// agent.
    #[derive(serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Agent<'a> {
        user_agent: &'a str,
    }

    fn wearing(agent: &str) -> Runtime {
        reference::installed(&Agent { user_agent: agent });
        Runtime::probe()
    }

    /// The port's `Browser` and `detectBrowser`'s object serialize to the same
    /// bytes, for every agent the differential names.
    #[wasm_bindgen_test]
    fn detection_agrees_with_the_reference_byte_for_byte() {
        for agent in reference::AGENTS {
            let ported = failure::rendered(Text::FailureStored, &Browser::detect(&wearing(agent)))
                .expect("the detected browser renders");
            let theirs = js_sys::JSON::stringify(&reference::detect_browser(agent))
                .expect("the reference object renders");
            assert_eq!(
                ported,
                String::from(theirs),
                "the port and the reference disagree for {agent}"
            );
        }
    }
}
