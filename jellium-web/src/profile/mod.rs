//! The port of `browserDeviceProfile.js` followed by `apphost.js`'s mutations,
//! rebuilt on every call.

pub mod probe;

#[cfg(test)]
mod parity;

use jellium_protocol::Bitrate;
use jellium_protocol::profile::{
    CodecKind, Comparison, Condition, ContainerProfile, Context, DeviceProfile, DirectPlayProfile,
    MediaKind, Property, Protocol, ResponseProfile, SubtitleMethod, SubtitleProfile,
    TranscodingProfile,
};

use crate::browser::{Browser, IosVersion, Version};
use crate::settings::{Account, BurnIn, MaxVideoWidth, Shared};
use probe::{Engine, Media};

/// The Tizen version the reference compares against, and `None` where it
/// compares against `undefined` and every comparison answers false.
fn tizen_version(browser: &Browser) -> Option<f64> {
    browser.tizen_version.map(Version::value)
}

/// The webOS version the reference compares against.
fn web0s_version(browser: &Browser) -> Option<f64> {
    browser.web0s_version.map(Version::value)
}

/// The iOS version the reference compares against, which reads as zero where
/// the run left the empty match, and which every caller gates on `browser.ios`.
fn ios_version(browser: &Browser) -> f64 {
    match browser.ios_version {
        Some(IosVersion::Detected(version)) => version.value(),
        Some(IosVersion::Unmatched) | None => 0.0,
    }
}

fn version_major_at_least(browser: &Browser, floor: u32) -> bool {
    browser.version_major.is_some_and(|major| major >= floor)
}

fn edge_uwp(browser: &Browser) -> bool {
    browser.edge_uwp == Some(true)
}

fn can_play_h264(media: &Media) -> bool {
    media.video("video/mp4; codecs=\"avc1.42E01E, mp4a.40.2\"")
}

fn can_play_hevc(browser: &Browser, media: &Media) -> bool {
    if browser.tizen || browser.xbox_one || browser.web0s {
        return true;
    }
    if browser.ps4 {
        return false;
    }
    media.video("video/mp4; codecs=\"hvc1.1.L120\"")
        || media.video("video/mp4; codecs=\"hev1.1.L120\"")
        || media.video("video/mp4; codecs=\"hvc1.1.0.L120\"")
        || media.video("video/mp4; codecs=\"hev1.1.0.L120\"")
}

fn can_play_av1(browser: &Browser, media: &Media) -> bool {
    if tizen_version(browser).is_some_and(|version| version >= 5.5)
        || web0s_version(browser).is_some_and(|version| version >= 5.0)
    {
        return true;
    }
    if browser.xbox_one {
        return false;
    }
    media.video("video/mp4; codecs=\"av01.0.15M.08\"")
        && media.video("video/mp4; codecs=\"av01.0.15M.10\"")
}

fn supports_text_tracks(browser: &Browser, engine: &Engine) -> bool {
    browser.tizen || engine.text_tracks()
}

fn can_play_native_hls(browser: &Browser, media: &Media) -> bool {
    if browser.tizen {
        return true;
    }
    media.video("application/x-mpegURL") || media.video("application/vnd.apple.mpegURL")
}

fn can_play_hls(browser: &Browser, engine: &Engine, media: &Media) -> bool {
    can_play_native_hls(browser, media) || engine.media_source()
}

fn can_play_native_hls_in_fmp4(browser: &Browser) -> bool {
    if tizen_version(browser).is_some_and(|version| version >= 5.0)
        || web0s_version(browser).is_some_and(|version| version >= 3.5)
    {
        return true;
    }
    (browser.ios && ios_version(browser) >= 11.0) || browser.osx
}

fn supports_ac3(browser: &Browser, media: &Media) -> bool {
    if edge_uwp(browser) || browser.tizen || browser.web0s {
        return true;
    }
    if browser.ios && ios_version(browser) < 11.0 {
        return false;
    }
    media.video("audio/mp4; codecs=\"ac-3\"")
}

/// `true` where the device plays DTS, `false` where it does not, and `None`
/// where the reference leaves the answer unknown.
fn can_play_dts(browser: &Browser, media: &Media) -> Option<bool> {
    if tizen_version(browser).is_some_and(|version| version >= 4.0)
        || web0s_version(browser).is_some_and(|version| (5.0..23.0).contains(&version))
    {
        return Some(false);
    }
    if media.video("video/mp4; codecs=\"dts-\"") || media.video("video/mp4; codecs=\"dts+\"") {
        return Some(true);
    }
    None
}

fn supports_eac3(browser: &Browser, media: &Media) -> bool {
    if browser.tizen || browser.web0s {
        return true;
    }
    if browser.ios && ios_version(browser) < 11.0 {
        return false;
    }
    media.video("audio/mp4; codecs=\"ec-3\"")
}

fn supports_ac3_in_hls(browser: &Browser, media: &Media) -> bool {
    if browser.tizen || browser.web0s {
        return true;
    }
    media.video("application/x-mpegurl; codecs=\"avc1.42E01E, ac-3\"")
        || media.video("application/vnd.apple.mpegURL; codecs=\"avc1.42E01E, ac-3\"")
}

fn supports_mp3_in_hls(media: &Media) -> bool {
    media.video("application/x-mpegurl; codecs=\"avc1.64001E, mp4a.40.34\"")
        || media.video("application/vnd.apple.mpegURL; codecs=\"avc1.64001E, mp4a.40.34\"")
}

fn can_play_audio_format(browser: &Browser, media: &Media, format: &str) -> bool {
    let mut mime = None;
    match format {
        "flac" | "asf" => {
            if browser.tizen || browser.web0s || edge_uwp(browser) {
                return true;
            }
        }
        "wma" => {
            if browser.tizen || edge_uwp(browser) {
                return true;
            }
        }
        "opus" => {
            if browser.web0s {
                return web0s_version(browser).is_some_and(|version| version >= 3.5);
            }
            if browser.xbox_one {
                return false;
            }
            mime = Some("audio/ogg; codecs=\"opus\"".to_owned());
        }
        "alac" => {
            if browser.ios || (browser.osx && browser.safari()) {
                return true;
            }
        }
        "mp2" => return false,
        _ => {}
    }
    let mime = if format == "webma" {
        "audio/webm".to_owned()
    } else {
        mime.unwrap_or_else(|| format!("audio/{format}"))
    };
    media.audio(&mime)
}

fn test_can_play_mkv(browser: &Browser, media: &Media) -> bool {
    if browser.vidaa {
        return false;
    }
    if browser.tizen || browser.web0s {
        return true;
    }
    if browser.firefox() {
        return false;
    }
    if media.video("video/x-matroska") || media.video("video/mkv") {
        return true;
    }
    if browser.edge_chromium && browser.windows() {
        return true;
    }
    edge_uwp(browser)
}

fn test_can_play_ts(browser: &Browser) -> bool {
    browser.tizen || browser.web0s || edge_uwp(browser)
}

fn supports_mpeg2_video(browser: &Browser) -> bool {
    browser.tizen || browser.web0s || edge_uwp(browser)
}

fn supports_vc1(browser: &Browser, media: &Media) -> bool {
    browser.tizen || browser.web0s || edge_uwp(browser) || media.video("video/mp4; codecs=\"vc-1\"")
}

fn supports_hdr10(browser: &Browser) -> bool {
    browser.vidaa
        || browser.tizen
        || browser.web0s
        || (browser.safari() && ((browser.ios && ios_version(browser) >= 11.0) || browser.osx))
        || (browser.edge_chromium && version_major_at_least(browser, 121))
        || (browser.chrome() && !browser.mobile)
        || (browser.firefox()
            && browser.osx
            && !browser.iphone()
            && !browser.ipad()
            && version_major_at_least(browser, 100))
}

fn supports_hlg(browser: &Browser) -> bool {
    supports_hdr10(browser)
}

fn supports_dolby_vision(browser: &Browser) -> bool {
    browser.safari() && ((browser.ios && ios_version(browser) >= 13.0) || browser.osx)
}

fn supported_dolby_vision_profiles_hevc(browser: &Browser, media: &Media) -> Vec<u8> {
    if browser.xbox_one {
        return vec![5, 8];
    }
    let mut supported = Vec::new();
    if media.video("video/mp4; codecs=\"dvh1.05.06\"") {
        supported.push(5);
    }
    if media.video("video/mp4; codecs=\"dvh1.08.06\"")
        || web0s_version(browser).is_some_and(|version| version >= 4.0)
    {
        supported.push(8);
    }
    supported
}

fn supported_dolby_vision_profile_av1(media: &Media) -> bool {
    media.video("video/mp4; codecs=\"dav1.10.06\"")
}

fn direct_play_profile_for_video_container(
    browser: &Browser,
    media: &Media,
    container: &str,
    video_audio_codecs: &[String],
) -> Option<DirectPlayProfile> {
    let mut supported = false;
    let mut profile_container = container.to_owned();
    let mut video_codecs: Vec<&str> = Vec::new();
    let mut audio_codecs = video_audio_codecs.to_vec();

    match container {
        "asf" | "wmv" => {
            supported = browser.tizen || browser.web0s || edge_uwp(browser);
            audio_codecs = Vec::new();
        }
        "avi" => {
            supported = browser.tizen || browser.web0s || edge_uwp(browser);
            if tizen_version(browser).is_some_and(|version| version >= 4.0) {
                video_codecs.push("h264");
                if can_play_hevc(browser, media) {
                    video_codecs.push("hevc");
                }
            }
        }
        "mpg" | "mpeg" => {
            supported = browser.tizen || browser.web0s || edge_uwp(browser);
        }
        "flv" => supported = browser.tizen,
        "3gp" | "mts" | "trp" | "vob" | "vro" => supported = browser.tizen,
        "mov" => {
            supported = browser.safari()
                || browser.tizen
                || browser.web0s
                || browser.chrome()
                || browser.edge_chromium
                || edge_uwp(browser);
            video_codecs.push("h264");
        }
        "m2ts" => {
            supported = browser.tizen || browser.web0s || edge_uwp(browser);
            video_codecs.push("h264");
            if supports_vc1(browser, media) {
                video_codecs.push("vc1");
            }
            if supports_mpeg2_video(browser) {
                video_codecs.push("mpeg2video");
            }
        }
        "ts" => {
            supported = test_can_play_ts(browser);
            video_codecs.push("h264");
            if (browser.tizen || browser.web0s) && can_play_hevc(browser, media) {
                video_codecs.push("hevc");
            }
            if supports_vc1(browser, media) {
                video_codecs.push("vc1");
            }
            if supports_mpeg2_video(browser) {
                video_codecs.push("mpeg2video");
            }
            profile_container = "ts,mpegts".to_owned();
        }
        _ => {}
    }

    supported.then(|| DirectPlayProfile::Video {
        container: profile_container,
        video_codec: video_codecs.join(","),
        audio_codec: audio_codecs.join(","),
    })
}

/// The ceiling `getGlobalMaxVideoBitrate` answers, rendered the way the
/// reference renders it: the empty string where it answers none.
fn global_max_video_bitrate(browser: &Browser) -> String {
    if browser.ps4 {
        return "8000000".to_owned();
    }
    if browser.xbox_one {
        return "12000000".to_owned();
    }
    String::new()
}

fn physical_audio_channels(
    browser: &Browser,
    engine: &Engine,
    media: &Media,
    account: &Account,
) -> i32 {
    if let Some(allowed) = account.allowed_audio_channels {
        return allowed.count();
    }
    let surround = browser.safari()
        || browser.chrome()
        || browser.edge_chromium
        || browser.firefox()
        || browser.tv
        || browser.ps4
        || browser.xbox_one;
    let dolby = supports_ac3(browser, media) || supports_eac3(browser, media);
    let speakers = engine.speakers().map_or(-1, |count| count.count());

    if dolby && surround {
        return if speakers > 6 { speakers } else { 6 };
    }
    if speakers > 2 {
        return if surround { speakers } else { 2 };
    }
    if speakers > 0 {
        return speakers;
    }
    if surround { 6 } else { 2 }
}

/// Whether hls.js is what would play the item, which is what gates the codecs
/// `getBaseProfileOptions` strikes off.
// reference: enable-hls-js-player — htmlMediaHelper.js:41-75
pub fn enable_hls_js_player(
    browser: &Browser,
    engine: &Engine,
    media: &Media,
    run_time_ticks: Option<i64>,
    kind: MediaKind,
) -> bool {
    if !engine.media_source() {
        return false;
    }
    if browser.ios {
        return false;
    }
    if browser.tizen || browser.web0s {
        return false;
    }
    if media.video("application/x-mpegURL") || media.video("application/vnd.apple.mpegURL") {
        if browser.android() && matches!(kind, MediaKind::Audio | MediaKind::Video) {
            return true;
        }
        if browser.chrome() || browser.edge_chromium || browser.opera() {
            return true;
        }
        if run_time_ticks.is_some_and(|ticks| ticks != 0) {
            return false;
        }
    }
    true
}

/// Whether hls.js is what plays this source, which is the wrapper the
/// reference's load site calls: a non-iOS Safari carrying a vp9 stream
/// short-circuits to hls.js and every other source falls through.
// reference: enable-hls-js-player-for-codecs — htmlMediaHelper.js:31-39
pub fn enable_hls_js_player_for_codecs(
    browser: &Browser,
    engine: &Engine,
    media: &Media,
    playable: &jellium_protocol::Playable,
    kind: MediaKind,
) -> bool {
    if !browser.ios && browser.safari() && playable.codecs.iter().any(|codec| codec == "vp9") {
        return true;
    }
    enable_hls_js_player(browser, engine, media, playable.run_time_ticks, kind)
}

/// What `apphost.js` hands the builder.
// reference: get-base-profile-options — apphost.js:28-48
pub struct Options {
    pub disable_hls_video_audio_codecs: Vec<String>,
}

impl Options {
    pub fn of(
        browser: &Browser,
        engine: &Engine,
        media: &Media,
        run_time_ticks: Option<i64>,
        kind: MediaKind,
    ) -> Options {
        let mut disable_hls_video_audio_codecs = Vec::new();
        if enable_hls_js_player(browser, engine, media, run_time_ticks, kind) {
            if browser.edge() {
                disable_hls_video_audio_codecs.push("mp3".to_owned());
            }
            if !browser.edge_chromium {
                disable_hls_video_audio_codecs.push("ac3".to_owned());
                disable_hls_video_audio_codecs.push("eac3".to_owned());
            }
            if !(browser.chrome() || browser.edge_chromium || browser.firefox()) {
                disable_hls_video_audio_codecs.push("opus".to_owned());
            }
        }
        Options {
            disable_hls_video_audio_codecs,
        }
    }
}

/// Whether the web engine plays a secondary audio track.
// reference: can-play-secondary-audio — browserDeviceProfile.js:476-486
pub fn secondary_audio(browser: &Browser, media: &Media) -> bool {
    media.audio_tracks()
        && !browser.firefox()
        && (tizen_version(browser).is_some_and(|version| (5.5..8.0).contains(&version))
            || !browser.tizen)
        && web0s_version(browser).is_none_or(|version| version >= 4.0 || version == 0.0)
}

fn condition(comparison: Comparison, property: Property, value: &str, required: bool) -> Condition {
    Condition {
        comparison,
        property,
        value: value.to_owned(),
        required: Some(required),
    }
}

/// The `AudioChannels` ceiling the webOS FLAC and Safari opus splits apply.
fn two_channels() -> Vec<Condition> {
    vec![condition(
        Comparison::LessThanEqual,
        Property::AudioChannels,
        "2",
        false,
    )]
}

/// Splits every video transcoding profile carrying `codec` into one that offers
/// `codec` alone under `conditions` and one that offers it not at all, the way
/// the reference splits FLAC on webOS and opus on Safari.
fn split_out(profiles: &mut Vec<TranscodingProfile>, codec: &str, conditions: &[Condition]) {
    let mut split = Vec::new();
    for profile in profiles.iter_mut() {
        if profile.kind != MediaKind::Video {
            continue;
        }
        let codecs: Vec<&str> = profile.audio_codec.split(',').collect();
        if !codecs.contains(&codec) {
            continue;
        }
        let mut only = TranscodingProfile {
            audio_codec: codec.to_owned(),
            apply_conditions: profile
                .apply_conditions
                .iter()
                .cloned()
                .chain(conditions.iter().cloned())
                .collect(),
            ..profile.clone()
        };
        only.conditions.clone_from(&profile.conditions);
        split.push(only);
        profile.audio_codec = codecs
            .into_iter()
            .filter(|held| *held != codec)
            .collect::<Vec<_>>()
            .join(",");
    }
    profiles.extend(split);
}

/// Moves `preferred` to the front of a comma-joined codec list, and leaves the
/// list standing when it does not carry it.
fn preferred_first(codecs: &str, preferred: &str) -> String {
    let mut held: Vec<&str> = codecs.split(',').collect();
    let Some(at) = held.iter().position(|codec| *codec == preferred) else {
        return codecs.to_owned();
    };
    held.remove(at);
    held.insert(0, preferred);
    held.join(",")
}

// reference: browser-device-profile — browserDeviceProfile.js:488-1606
// reference: get-device-profile — apphost.js:50-122
pub fn build(
    browser: &Browser,
    engine: &Engine,
    media: &Media,
    shared: &Shared,
    account: &Account,
    options: &Options,
) -> DeviceProfile {
    let bitrate_setting = Bitrate::of(120_000_000);
    let channels = physical_audio_channels(browser, engine, media, account);
    let channels_text = channels.to_string();

    let can_play_vp8 = media.video("video/webm; codecs=\"vp8\"");
    let can_play_vp9 = media.video("video/webm; codecs=\"vp9\"");
    let safari_supports_opus = browser.safari()
        && version_major_at_least(browser, 17)
        && media.audio("audio/x-caf; codecs=\"opus\"");
    let mut webm_audio_codecs = vec!["vorbis".to_owned()];
    let can_play_mkv = test_can_play_mkv(browser, media);

    let mut profile = DeviceProfile {
        max_streaming_bitrate: bitrate_setting,
        music_streaming_transcoding_bitrate: bitrate_setting.min(Bitrate::of(384_000)),
        ..DeviceProfile::default()
    };

    let mut video_audio_codecs: Vec<String> = Vec::new();
    let mut hls_in_ts_video_audio_codecs: Vec<String> = Vec::new();
    let mut hls_in_fmp4_video_audio_codecs: Vec<String> = Vec::new();

    let supports_mp3_video_audio = media.video("video/mp4; codecs=\"avc1.640029, mp4a.69\"")
        || media.video("video/mp4; codecs=\"avc1.640029, mp4a.6B\"")
        || media.video("video/mp4; codecs=\"avc1.640029, mp3\"");

    let mut supports_mp2_video_audio = edge_uwp(browser) || browser.tizen || browser.web0s;
    if supports_mp3_video_audio
        && (browser.chrome()
            || browser.edge_chromium
            || (browser.firefox() && version_major_at_least(browser, 83)))
    {
        supports_mp2_video_audio = true;
    }
    if browser.android() {
        supports_mp2_video_audio = false;
    }

    // reference: xbox-screen-width — browserDeviceProfile.js:532
    let builder_max_video_width = if browser.xbox_one {
        engine.screen_width()
    } else {
        None
    };

    let can_play_aac_video_audio = media.video("video/mp4; codecs=\"avc1.640029, mp4a.40.2\"");
    let can_play_mp3_video_audio_in_hls = supports_mp3_in_hls(media);
    let can_play_ac3_video_audio = supports_ac3(browser, media);
    let can_play_eac3_video_audio = supports_eac3(browser, media);
    let can_play_ac3_video_audio_in_hls = supports_ac3_in_hls(browser, media);

    if can_play_aac_video_audio {
        video_audio_codecs.push("aac".to_owned());
        hls_in_ts_video_audio_codecs.push("aac".to_owned());
        hls_in_fmp4_video_audio_codecs.push("aac".to_owned());
    }
    if supports_mp3_video_audio {
        video_audio_codecs.push("mp3".to_owned());
    }
    if browser.safari() || (supports_mp3_video_audio && !browser.ps4) {
        hls_in_ts_video_audio_codecs.push("mp3".to_owned());
    }
    if can_play_mp3_video_audio_in_hls {
        hls_in_fmp4_video_audio_codecs.push("mp3".to_owned());
    }
    if can_play_ac3_video_audio {
        video_audio_codecs.push("ac3".to_owned());
        if browser.edge_chromium {
            hls_in_fmp4_video_audio_codecs.push("ac3".to_owned());
        }
        if can_play_eac3_video_audio {
            video_audio_codecs.push("eac3".to_owned());
            if browser.edge_chromium {
                hls_in_fmp4_video_audio_codecs.push("eac3".to_owned());
            }
        }
        if can_play_ac3_video_audio_in_hls {
            hls_in_ts_video_audio_codecs.push("ac3".to_owned());
            hls_in_fmp4_video_audio_codecs.push("ac3".to_owned());
            if can_play_eac3_video_audio {
                hls_in_ts_video_audio_codecs.push("eac3".to_owned());
                hls_in_fmp4_video_audio_codecs.push("eac3".to_owned());
            }
        }
    }
    if supports_mp2_video_audio {
        video_audio_codecs.push("mp2".to_owned());
        hls_in_ts_video_audio_codecs.push("mp2".to_owned());
        hls_in_fmp4_video_audio_codecs.push("mp2".to_owned());
    }

    let supports_dts = shared.enable_dts || can_play_dts(browser, media) == Some(true);
    if supports_dts {
        video_audio_codecs.push("dca".to_owned());
        video_audio_codecs.push("dts".to_owned());
    }
    if browser.tizen || browser.web0s {
        video_audio_codecs.push("pcm_s16le".to_owned());
        video_audio_codecs.push("pcm_s24le".to_owned());
    }
    if shared.enable_true_hd {
        video_audio_codecs.push("truehd".to_owned());
    }
    if browser.tizen {
        video_audio_codecs.push("aac_latm".to_owned());
    }
    if can_play_audio_format(browser, media, "opus") {
        video_audio_codecs.push("opus".to_owned());
        webm_audio_codecs.push("opus".to_owned());
        if browser.tizen {
            hls_in_ts_video_audio_codecs.push("opus".to_owned());
        }
        hls_in_fmp4_video_audio_codecs.push("opus".to_owned());
    } else if safari_supports_opus {
        video_audio_codecs.push("opus".to_owned());
        webm_audio_codecs.push("opus".to_owned());
        hls_in_fmp4_video_audio_codecs.push("opus".to_owned());
    }
    if can_play_audio_format(browser, media, "flac") && !browser.tizen {
        video_audio_codecs.push("flac".to_owned());
        hls_in_fmp4_video_audio_codecs.push("flac".to_owned());
    }
    if can_play_audio_format(browser, media, "alac") {
        video_audio_codecs.push("alac".to_owned());
        hls_in_fmp4_video_audio_codecs.push("alac".to_owned());
    }

    let struck = |codecs: Vec<String>| -> Vec<String> {
        codecs
            .into_iter()
            .filter(|codec| !options.disable_hls_video_audio_codecs.contains(codec))
            .collect()
    };
    hls_in_ts_video_audio_codecs = struck(hls_in_ts_video_audio_codecs);
    hls_in_fmp4_video_audio_codecs = struck(hls_in_fmp4_video_audio_codecs);

    let mut mp4_video_codecs: Vec<String> = Vec::new();
    let mut webm_video_codecs: Vec<String> = Vec::new();
    let mut hls_in_ts_video_codecs: Vec<String> = Vec::new();
    let mut hls_in_fmp4_video_codecs: Vec<String> = Vec::new();

    if can_play_av1(browser, media)
        && (browser.safari()
            || (!browser.mobile
                && (browser.edge_chromium
                    || browser.firefox()
                    || browser.chrome()
                    || browser.opera())))
    {
        hls_in_fmp4_video_codecs.push("av1".to_owned());
    }
    if can_play_hevc(browser, media)
        && (browser.edge_chromium
            || browser.safari()
            || browser.tizen
            || browser.web0s
            || (browser.chrome() && (!browser.android() || version_major_at_least(browser, 105)))
            || (browser.opera() && !browser.mobile)
            || (browser.firefox() && version_major_at_least(browser, 134)))
    {
        hls_in_fmp4_video_codecs.push("hevc".to_owned());
    }
    if can_play_h264(media) {
        mp4_video_codecs.push("h264".to_owned());
        hls_in_ts_video_codecs.push("h264".to_owned());
        hls_in_fmp4_video_codecs.push("h264".to_owned());
    }
    if can_play_hevc(browser, media) {
        mp4_video_codecs.push("hevc".to_owned());
        if browser.tizen || browser.web0s || browser.vidaa {
            hls_in_ts_video_codecs.push("hevc".to_owned());
        }
    }
    if supports_mpeg2_video(browser) {
        mp4_video_codecs.push("mpeg2video".to_owned());
    }
    if supports_vc1(browser, media) {
        mp4_video_codecs.push("vc1".to_owned());
    }
    if browser.tizen {
        mp4_video_codecs.push("msmpeg4v2".to_owned());
    }
    if can_play_vp8 {
        webm_video_codecs.push("vp8".to_owned());
    }
    let webm_admits_recent = |browser: &Browser| {
        !browser.safari()
            || (version_major_at_least(browser, 15) && !version_major_at_least(browser, 17))
    };
    if can_play_vp9 {
        if !browser.ios && !(browser.firefox() && browser.osx) {
            mp4_video_codecs.push("vp9".to_owned());
        }
        if browser.safari() || browser.edge_chromium || browser.chrome() || browser.firefox() {
            hls_in_fmp4_video_codecs.push("vp9".to_owned());
        }
        if webm_admits_recent(browser) {
            webm_video_codecs.push("vp9".to_owned());
        }
    }
    if can_play_av1(browser, media) {
        mp4_video_codecs.push("av1".to_owned());
        if webm_admits_recent(browser) {
            webm_video_codecs.push("av1".to_owned());
        }
    }
    if (!browser.safari() && can_play_vp8) || browser.tizen {
        video_audio_codecs.push("vorbis".to_owned());
    }

    if !webm_video_codecs.is_empty() {
        profile.direct_play_profiles.push(DirectPlayProfile::Video {
            container: "webm".to_owned(),
            video_codec: webm_video_codecs.join(","),
            audio_codec: webm_audio_codecs.join(","),
        });
    }
    if !mp4_video_codecs.is_empty() {
        profile.direct_play_profiles.push(DirectPlayProfile::Video {
            container: "mp4,m4v".to_owned(),
            video_codec: mp4_video_codecs.join(","),
            audio_codec: video_audio_codecs.join(","),
        });
    }
    if can_play_mkv && !mp4_video_codecs.is_empty() {
        profile.direct_play_profiles.push(DirectPlayProfile::Video {
            container: "mkv".to_owned(),
            video_codec: mp4_video_codecs.join(","),
            audio_codec: video_audio_codecs.join(","),
        });
    }

    for container in [
        "m2ts", "wmv", "ts", "asf", "avi", "mpg", "mpeg", "flv", "3gp", "mts", "trp", "vob", "vro",
        "mov",
    ] {
        if let Some(entry) =
            direct_play_profile_for_video_container(browser, media, container, &video_audio_codecs)
        {
            profile.direct_play_profiles.push(entry);
        }
    }

    for format in [
        "opus", "mp3", "mp2", "aac", "flac", "alac", "webma", "wma", "wav", "ogg", "oga",
    ] {
        if !can_play_audio_format(browser, media, format) {
            continue;
        }
        if format == "mp3" && !can_play_mp3_video_audio_in_hls {
            profile.direct_play_profiles.push(DirectPlayProfile::Audio {
                container: "ts".to_owned(),
                audio_codec: Some("mp3".to_owned()),
            });
        }
        if format == "flac" && shared.always_remux_flac {
            profile.direct_play_profiles.push(DirectPlayProfile::Audio {
                container: "mp4".to_owned(),
                audio_codec: Some("flac".to_owned()),
            });
        } else if format != "mp3" || !shared.always_remux_mp3 {
            profile.direct_play_profiles.push(DirectPlayProfile::Audio {
                container: format.to_owned(),
                audio_codec: None,
            });
        }
        if format == "opus" || format == "webma" {
            profile.direct_play_profiles.push(DirectPlayProfile::Audio {
                container: "webm".to_owned(),
                audio_codec: Some(format.to_owned()),
            });
        }
        if format == "aac" || format == "alac" {
            profile.direct_play_profiles.push(DirectPlayProfile::Audio {
                container: "m4a".to_owned(),
                audio_codec: Some(format.to_owned()),
            });
            profile.direct_play_profiles.push(DirectPlayProfile::Audio {
                container: "m4b".to_owned(),
                audio_codec: Some(format.to_owned()),
            });
        }
    }
    if safari_supports_opus {
        profile.direct_play_profiles.push(DirectPlayProfile::Audio {
            container: "mp4".to_owned(),
            audio_codec: Some("opus".to_owned()),
        });
    }

    let hls_break_on_non_key_frames =
        browser.ios || browser.osx || browser.edge() || !can_play_native_hls(browser, media);
    let mut enable_fmp4_hls = account.prefer_fmp4_hls_container;
    if (browser.safari() || browser.tizen || browser.web0s) && !can_play_native_hls_in_fmp4(browser)
    {
        enable_fmp4_hls = false;
    }
    if browser.firefox() && browser.version_major == Some(149) {
        enable_fmp4_hls = false;
    }
    let min_segments = if browser.ios || browser.osx { "2" } else { "1" };

    if can_play_hls(browser, engine, media) {
        profile.transcoding_profiles.push(TranscodingProfile {
            container: if enable_fmp4_hls { "mp4" } else { "ts" }.to_owned(),
            kind: MediaKind::Audio,
            audio_codec: "aac".to_owned(),
            video_codec: None,
            context: Context::Streaming,
            protocol: Protocol::Hls,
            max_audio_channels: channels_text.clone(),
            min_segments: Some(min_segments.to_owned()),
            break_on_non_key_frames: Some(hls_break_on_non_key_frames),
            enable_audio_vbr_encoding: Some(!shared.disable_vbr_audio),
            segment_length: None,
            apply_conditions: Vec::new(),
            conditions: Vec::new(),
        });
    }

    for format in ["aac", "mp3", "opus", "wav"] {
        if !can_play_audio_format(browser, media, format) {
            continue;
        }
        profile.transcoding_profiles.push(TranscodingProfile {
            container: format.to_owned(),
            kind: MediaKind::Audio,
            audio_codec: format.to_owned(),
            video_codec: None,
            context: Context::Streaming,
            protocol: Protocol::Http,
            max_audio_channels: channels_text.clone(),
            min_segments: None,
            break_on_non_key_frames: None,
            enable_audio_vbr_encoding: None,
            segment_length: None,
            apply_conditions: Vec::new(),
            conditions: Vec::new(),
        });
    }
    for format in ["opus", "mp3", "aac", "wav"] {
        if !can_play_audio_format(browser, media, format) {
            continue;
        }
        profile.transcoding_profiles.push(TranscodingProfile {
            container: format.to_owned(),
            kind: MediaKind::Audio,
            audio_codec: format.to_owned(),
            video_codec: None,
            context: Context::Static,
            protocol: Protocol::Http,
            max_audio_channels: channels_text.clone(),
            min_segments: None,
            break_on_non_key_frames: None,
            enable_audio_vbr_encoding: None,
            segment_length: None,
            apply_conditions: Vec::new(),
            conditions: Vec::new(),
        });
    }

    if can_play_hls(browser, engine, media) {
        let segment_length = account.limit_segment_length.then_some(1);
        if !hls_in_fmp4_video_codecs.is_empty()
            && !hls_in_fmp4_video_audio_codecs.is_empty()
            && enable_fmp4_hls
        {
            profile.direct_play_profiles.push(DirectPlayProfile::Video {
                container: "hls".to_owned(),
                video_codec: hls_in_fmp4_video_codecs.join(","),
                audio_codec: hls_in_fmp4_video_audio_codecs.join(","),
            });
            profile.transcoding_profiles.push(TranscodingProfile {
                container: "mp4".to_owned(),
                kind: MediaKind::Video,
                audio_codec: hls_in_fmp4_video_audio_codecs.join(","),
                video_codec: Some(hls_in_fmp4_video_codecs.join(",")),
                context: Context::Streaming,
                protocol: Protocol::Hls,
                max_audio_channels: channels_text.clone(),
                min_segments: Some(min_segments.to_owned()),
                break_on_non_key_frames: Some(hls_break_on_non_key_frames),
                enable_audio_vbr_encoding: None,
                segment_length,
                apply_conditions: Vec::new(),
                conditions: Vec::new(),
            });
        }
        if !hls_in_ts_video_codecs.is_empty() && !hls_in_ts_video_audio_codecs.is_empty() {
            profile.direct_play_profiles.push(DirectPlayProfile::Video {
                container: "hls".to_owned(),
                video_codec: hls_in_ts_video_codecs.join(","),
                audio_codec: hls_in_ts_video_audio_codecs.join(","),
            });
            profile.transcoding_profiles.push(TranscodingProfile {
                container: "ts".to_owned(),
                kind: MediaKind::Video,
                audio_codec: hls_in_ts_video_audio_codecs.join(","),
                video_codec: Some(hls_in_ts_video_codecs.join(",")),
                context: Context::Streaming,
                protocol: Protocol::Hls,
                max_audio_channels: channels_text.clone(),
                min_segments: Some(min_segments.to_owned()),
                break_on_non_key_frames: Some(hls_break_on_non_key_frames),
                enable_audio_vbr_encoding: None,
                segment_length,
                apply_conditions: Vec::new(),
                conditions: Vec::new(),
            });
        }
    }

    if tizen_version(browser).is_some_and(|version| version < 6.5) {
        profile.container_profiles.push(ContainerProfile {
            kind: MediaKind::Video,
            conditions: vec![condition(
                Comparison::LessThanEqual,
                Property::NumStreams,
                "32",
                false,
            )],
        });
    }

    let supports_secondary_audio = secondary_audio(browser, media);

    let mut aac_conditions = Vec::new();
    if !media.video("video/mp4; codecs=\"avc1.640029, mp4a.40.5\"") {
        aac_conditions.push(Condition {
            comparison: Comparison::NotEquals,
            property: Property::AudioProfile,
            value: "HE-AAC".to_owned(),
            required: None,
        });
    }
    if !supports_secondary_audio {
        aac_conditions.push(condition(
            Comparison::Equals,
            Property::IsSecondaryAudio,
            "false",
            false,
        ));
    }
    if !aac_conditions.is_empty() {
        profile
            .codec_profiles
            .push(jellium_protocol::profile::CodecProfile::Codec {
                kind: CodecKind::VideoAudio,
                codec: "aac".to_owned(),
                conditions: aac_conditions,
            });
    }

    let mut global_audio_conditions = Vec::new();
    let mut global_video_audio_conditions = Vec::new();
    if account.allowed_audio_channels.is_some() {
        global_audio_conditions.push(condition(
            Comparison::LessThanEqual,
            Property::AudioChannels,
            &channels_text,
            false,
        ));
        global_video_audio_conditions.push(condition(
            Comparison::LessThanEqual,
            Property::AudioChannels,
            &channels_text,
            false,
        ));
    }
    if !supports_secondary_audio {
        global_video_audio_conditions.push(condition(
            Comparison::Equals,
            Property::IsSecondaryAudio,
            "false",
            false,
        ));
    }
    if !global_audio_conditions.is_empty() {
        profile
            .codec_profiles
            .push(jellium_protocol::profile::CodecProfile::Any {
                kind: CodecKind::Audio,
                conditions: global_audio_conditions,
            });
    }
    if !global_video_audio_conditions.is_empty() {
        profile
            .codec_profiles
            .push(jellium_protocol::profile::CodecProfile::Any {
                kind: CodecKind::VideoAudio,
                conditions: global_video_audio_conditions,
            });
    }

    if browser.web0s {
        let flac_conditions = two_channels();
        profile
            .codec_profiles
            .push(jellium_protocol::profile::CodecProfile::Codec {
                kind: CodecKind::VideoAudio,
                codec: "flac".to_owned(),
                conditions: flac_conditions.clone(),
            });
        split_out(&mut profile.transcoding_profiles, "flac", &flac_conditions);
    }

    if safari_supports_opus {
        let opus_conditions = two_channels();
        profile
            .codec_profiles
            .push(jellium_protocol::profile::CodecProfile::Codec {
                kind: CodecKind::VideoAudio,
                codec: "opus".to_owned(),
                conditions: opus_conditions.clone(),
            });
        split_out(&mut profile.transcoding_profiles, "opus", &opus_conditions);
    }

    let mut max_h264_level = 42;
    let mut h264_profiles = "high|main|baseline|constrained baseline".to_owned();
    if browser.tizen || browser.web0s || media.video("video/mp4; codecs=\"avc1.640833\"") {
        max_h264_level = 51;
    }
    if media.video("video/mp4; codecs=\"avc1.640834\"") {
        max_h264_level = 52;
    }
    if media.video("video/mp4; codecs=\"avc1.6e0033\"")
        && !browser.safari()
        && !browser.ios
        && !browser.web0s
        && !browser.edge()
        && !browser.mobile
        && !browser.tizen
    {
        h264_profiles += "|high 10";
    }

    let mut max_hevc_level = 120;
    let mut hevc_profiles = "main".to_owned();
    if media.video("video/mp4; codecs=\"hvc1.1.4.L123\"")
        || media.video("video/mp4; codecs=\"hev1.1.4.L123\"")
    {
        max_hevc_level = 123;
    }
    for (hvc, hev, level) in [
        (
            "video/mp4; codecs=\"hvc1.2.4.L123\"",
            "video/mp4; codecs=\"hev1.2.4.L123\"",
            123,
        ),
        (
            "video/mp4; codecs=\"hvc1.2.4.L153\"",
            "video/mp4; codecs=\"hev1.2.4.L153\"",
            153,
        ),
        (
            "video/mp4; codecs=\"hvc1.2.4.L183\"",
            "video/mp4; codecs=\"hev1.2.4.L183\"",
            183,
        ),
        (
            "video/mp4; codecs=\"hvc1.2.4.L186\"",
            "video/mp4; codecs=\"hev1.2.4.L186\"",
            186,
        ),
    ] {
        if media.video(hvc) || media.video(hev) {
            max_hevc_level = level;
            hevc_profiles = "main|main 10".to_owned();
        }
    }

    let mut max_av1_level = 15;
    let av1_profiles = "main";
    for (eight, ten, level) in [
        (
            "video/mp4; codecs=\"av01.0.16M.08\"",
            "video/mp4; codecs=\"av01.0.16M.10\"",
            16,
        ),
        (
            "video/mp4; codecs=\"av01.0.17M.08\"",
            "video/mp4; codecs=\"av01.0.17M.10\"",
            17,
        ),
        (
            "video/mp4; codecs=\"av01.0.18M.08\"",
            "video/mp4; codecs=\"av01.0.18M.10\"",
            18,
        ),
        (
            "video/mp4; codecs=\"av01.0.19M.08\"",
            "video/mp4; codecs=\"av01.0.19M.10\"",
            19,
        ),
    ] {
        if media.video(eight) && media.video(ten) {
            max_av1_level = level;
        }
    }

    let h264_video_range_types = "SDR";
    let mut hevc_video_range_types = "SDR".to_owned();
    let mut vp9_video_range_types = "SDR".to_owned();
    let mut av1_video_range_types = "SDR".to_owned();

    let web0s_without_dolby_vision = browser.web0s && !supports_dolby_vision(browser);
    if tizen_version(browser).is_some_and(|version| version >= 3.0) || web0s_without_dolby_vision {
        hevc_video_range_types += "|DOVIWithSDR";
    }
    if supports_hdr10(browser) {
        hevc_video_range_types += "|HDR10|HDR10Plus";
        vp9_video_range_types += "|HDR10|HDR10Plus";
        av1_video_range_types += "|HDR10|HDR10Plus";
        if tizen_version(browser).is_some_and(|version| version >= 3.0)
            || browser.vidaa
            || web0s_without_dolby_vision
        {
            hevc_video_range_types +=
                "|DOVIWithHDR10|DOVIWithHDR10Plus|DOVIWithEL|DOVIWithELHDR10Plus|DOVIInvalid";
            av1_video_range_types +=
                "|DOVIWithHDR10|DOVIWithHDR10Plus|DOVIWithEL|DOVIWithELHDR10Plus|DOVIInvalid";
        }
    }
    if supports_hlg(browser) {
        hevc_video_range_types += "|HLG";
        vp9_video_range_types += "|HLG";
        av1_video_range_types += "|HLG";
        if tizen_version(browser).is_some_and(|version| version >= 3.0)
            || web0s_without_dolby_vision
        {
            hevc_video_range_types += "|DOVIWithHLG";
        }
    }
    if supports_dolby_vision(browser) {
        let profiles = supported_dolby_vision_profiles_hevc(browser, media);
        if profiles.contains(&5) {
            hevc_video_range_types += "|DOVI";
        }
        if profiles.contains(&8) {
            hevc_video_range_types += "|DOVIWithHDR10|DOVIWithHLG|DOVIWithSDR|DOVIWithHDR10Plus";
        }
        if browser.web0s {
            hevc_video_range_types += "|DOVIWithEL|DOVIWithELHDR10Plus|DOVIInvalid";
        }
        if supported_dolby_vision_profile_av1(media) {
            av1_video_range_types +=
                "|DOVI|DOVIWithHDR10|DOVIWithHLG|DOVIWithSDR|DOVIWithHDR10Plus";
            if browser.web0s {
                av1_video_range_types += "|DOVIWithEL|DOVIWithELHDR10Plus|DOVIInvalid";
            }
        }
    }

    let mut h264_conditions = vec![
        condition(Comparison::NotEquals, Property::IsAnamorphic, "true", false),
        condition(
            Comparison::EqualsAny,
            Property::VideoProfile,
            &h264_profiles,
            false,
        ),
        condition(
            Comparison::EqualsAny,
            Property::VideoRangeType,
            h264_video_range_types,
            false,
        ),
        condition(
            Comparison::LessThanEqual,
            Property::VideoLevel,
            &max_h264_level.to_string(),
            false,
        ),
    ];
    let mut hevc_conditions = vec![
        condition(Comparison::NotEquals, Property::IsAnamorphic, "true", false),
        condition(
            Comparison::EqualsAny,
            Property::VideoProfile,
            &hevc_profiles,
            false,
        ),
        condition(
            Comparison::EqualsAny,
            Property::VideoRangeType,
            &hevc_video_range_types,
            false,
        ),
        condition(
            Comparison::LessThanEqual,
            Property::VideoLevel,
            &max_hevc_level.to_string(),
            false,
        ),
    ];
    let vp9_conditions = vec![condition(
        Comparison::EqualsAny,
        Property::VideoRangeType,
        &vp9_video_range_types,
        false,
    )];
    let mut av1_conditions = vec![
        condition(Comparison::NotEquals, Property::IsAnamorphic, "true", false),
        condition(
            Comparison::EqualsAny,
            Property::VideoProfile,
            av1_profiles,
            false,
        ),
        condition(
            Comparison::EqualsAny,
            Property::VideoRangeType,
            &av1_video_range_types,
            false,
        ),
        condition(
            Comparison::LessThanEqual,
            Property::VideoLevel,
            &max_av1_level.to_string(),
            false,
        ),
    ];

    if !edge_uwp(browser) && !browser.tizen && !browser.web0s {
        h264_conditions.push(condition(
            Comparison::NotEquals,
            Property::IsInterlaced,
            "true",
            false,
        ));
        hevc_conditions.push(condition(
            Comparison::NotEquals,
            Property::IsInterlaced,
            "true",
            false,
        ));
    }

    if let Some(width) = builder_max_video_width {
        let width = width.count().to_string();
        for conditions in [
            &mut h264_conditions,
            &mut hevc_conditions,
            &mut av1_conditions,
        ] {
            conditions.push(condition(
                Comparison::LessThanEqual,
                Property::Width,
                &width,
                false,
            ));
        }
    }

    let global_max_video_bitrate = global_max_video_bitrate(browser);
    if !global_max_video_bitrate.is_empty() {
        for conditions in [
            &mut h264_conditions,
            &mut hevc_conditions,
            &mut av1_conditions,
        ] {
            conditions.push(condition(
                Comparison::LessThanEqual,
                Property::VideoBitrate,
                &global_max_video_bitrate,
                true,
            ));
        }
    }

    if browser.safari() {
        hevc_conditions.push(condition(
            Comparison::EqualsAny,
            Property::VideoCodecTag,
            "hvc1|dvh1",
            true,
        ));
        hevc_conditions.push(condition(
            Comparison::LessThanEqual,
            Property::VideoFramerate,
            "60",
            true,
        ));
    }

    if browser.ios && ios_version(browser) < 13.0 {
        for container in ["ts", "mp4"] {
            let mut conditions: Vec<Condition> = h264_conditions
                .iter()
                .filter(|held| held.property != Property::VideoLevel)
                .cloned()
                .collect();
            conditions.push(condition(
                Comparison::LessThanEqual,
                Property::VideoLevel,
                "42",
                false,
            ));
            profile
                .codec_profiles
                .push(jellium_protocol::profile::CodecProfile::Contained {
                    kind: CodecKind::Video,
                    codec: "h264".to_owned(),
                    container: container.to_owned(),
                    conditions,
                });
        }
    }

    if browser.safari() && shared.enable_hi10p {
        profile
            .codec_profiles
            .push(jellium_protocol::profile::CodecProfile::SubContained {
                kind: CodecKind::Video,
                container: "hls".to_owned(),
                sub_container: "mp4".to_owned(),
                codec: "h264".to_owned(),
                conditions: vec![condition(
                    Comparison::EqualsAny,
                    Property::VideoProfile,
                    &format!("{h264_profiles}|high 10"),
                    false,
                )],
            });
    }

    profile
        .codec_profiles
        .push(jellium_protocol::profile::CodecProfile::Codec {
            kind: CodecKind::Video,
            codec: "h264".to_owned(),
            conditions: h264_conditions,
        });

    if browser.web0s && supports_dolby_vision(browser) {
        let without_dovi = hevc_video_range_types
            .split('|')
            .filter(|range| !range.starts_with("DOVI"))
            .collect::<Vec<_>>()
            .join("|");
        profile
            .codec_profiles
            .push(jellium_protocol::profile::CodecProfile::Barred {
                kind: CodecKind::Video,
                container: "-mp4,ts".to_owned(),
                codec: "hevc".to_owned(),
                conditions: vec![condition(
                    Comparison::EqualsAny,
                    Property::VideoRangeType,
                    &without_dovi,
                    false,
                )],
            });
    }

    for (codec, conditions) in [
        ("hevc", hevc_conditions),
        ("vp9", vp9_conditions),
        ("av1", av1_conditions),
    ] {
        profile
            .codec_profiles
            .push(jellium_protocol::profile::CodecProfile::Codec {
                kind: CodecKind::Video,
                codec: codec.to_owned(),
                conditions,
            });
    }

    let mut global_video_conditions = Vec::new();
    if !global_max_video_bitrate.is_empty() {
        global_video_conditions.push(Condition {
            comparison: Comparison::LessThanEqual,
            property: Property::VideoBitrate,
            value: global_max_video_bitrate.clone(),
            required: None,
        });
    }
    if let Some(width) = builder_max_video_width {
        global_video_conditions.push(condition(
            Comparison::LessThanEqual,
            Property::Width,
            &width.count().to_string(),
            false,
        ));
    }
    if !global_video_conditions.is_empty() {
        profile
            .codec_profiles
            .push(jellium_protocol::profile::CodecProfile::Any {
                kind: CodecKind::Video,
                conditions: global_video_conditions,
            });
    }

    // reference: subtitle-burnin-setting — browserDeviceProfile.js:1569
    if shared.subtitle_burn_in != BurnIn::All {
        if supports_text_tracks(browser, engine) {
            profile.subtitle_profiles.push(SubtitleProfile {
                format: "vtt".to_owned(),
                method: SubtitleMethod::External,
            });
        }
        if shared.subtitle_burn_in != BurnIn::AllComplexFormats {
            for format in ["ass", "ssa"] {
                profile.subtitle_profiles.push(SubtitleProfile {
                    format: format.to_owned(),
                    method: SubtitleMethod::External,
                });
            }
        }
        if engine.canvas_2d()
            && shared.subtitle_render_pgs
            && shared.subtitle_burn_in != BurnIn::AllComplexFormats
            && shared.subtitle_burn_in != BurnIn::OnlyImageFormats
        {
            profile.subtitle_profiles.push(SubtitleProfile {
                format: "pgssub".to_owned(),
                method: SubtitleMethod::External,
            });
        }
    }

    profile.response_profiles.push(ResponseProfile {
        kind: MediaKind::Video,
        container: "m4v".to_owned(),
        mime_type: "video/mp4".to_owned(),
    });

    // reference: max-video-width-setting — appSettings.js:114-120
    let transcoding_width = match shared.max_video_width {
        MaxVideoWidth::Screen => engine.screen(browser).map(probe::Screen::max_allowed_width),
        MaxVideoWidth::Unset => None,
        MaxVideoWidth::Fixed(width) => Some(width),
    };
    if let Some(width) = transcoding_width {
        let width = condition(
            Comparison::LessThanEqual,
            Property::Width,
            &width.count().to_string(),
            false,
        );
        if shared.limit_supported_video_resolution {
            profile
                .codec_profiles
                .push(jellium_protocol::profile::CodecProfile::Any {
                    kind: CodecKind::Video,
                    conditions: vec![width.clone()],
                });
        }
        for transcoding in &mut profile.transcoding_profiles {
            if transcoding.kind != MediaKind::Video {
                continue;
            }
            transcoding
                .conditions
                .retain(|held| held.property != Property::Width);
            transcoding.conditions.push(width.clone());
        }
    }

    if !shared.preferred_transcode_video_codec.is_empty() {
        for transcoding in &mut profile.transcoding_profiles {
            if transcoding.kind != MediaKind::Video {
                continue;
            }
            transcoding.video_codec = transcoding
                .video_codec
                .as_deref()
                .map(|codecs| preferred_first(codecs, &shared.preferred_transcode_video_codec));
        }
    }
    if !shared.preferred_transcode_video_audio_codec.is_empty() {
        for transcoding in &mut profile.transcoding_profiles {
            if transcoding.kind != MediaKind::Video {
                continue;
            }
            transcoding.audio_codec = preferred_first(
                &transcoding.audio_codec,
                &shared.preferred_transcode_video_audio_codec,
            );
        }
    }

    profile
}
