//! The port and the sliced reference, run under Node in one process, both
//! reading one installed environment.

use jellium_protocol::profile::{DeviceProfile, MediaKind, SubtitleMethod};
use serde::Serialize;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::wasm_bindgen_test;

use super::probe::{Engine, Media};
use super::{Options, build, enable_hls_js_player, secondary_audio};
use crate::browser::{Browser, Runtime};
use crate::failure;
use crate::reference;
use crate::settings::{Account, BurnIn, Channels, MaxVideoWidth, Pixels, Shared};
use crate::text::Text;

/// The mimes an authored environment answers, and what it answers them with.
/// The tables name no browser: one admits every mime the port asks about, one
/// admits none, and one admits a middle band.
type Table = std::collections::BTreeMap<&'static str, &'static str>;

/// Every video mime the port asks `canPlayType` about.
const VIDEO_MIMES: [&str; 34] = [
    "video/mp4; codecs=\"avc1.42E01E, mp4a.40.2\"",
    "video/mp4; codecs=\"hvc1.1.L120\"",
    "video/mp4; codecs=\"hev1.1.L120\"",
    "video/mp4; codecs=\"hvc1.1.0.L120\"",
    "video/mp4; codecs=\"hev1.1.0.L120\"",
    "video/mp4; codecs=\"av01.0.15M.08\"",
    "video/mp4; codecs=\"av01.0.15M.10\"",
    "application/x-mpegURL",
    "application/vnd.apple.mpegURL",
    "audio/mp4; codecs=\"ac-3\"",
    "audio/mp4; codecs=\"ec-3\"",
    "video/mp4; codecs=\"dts-\"",
    "video/mp4; codecs=\"dts+\"",
    "application/x-mpegurl; codecs=\"avc1.42E01E, ac-3\"",
    "application/vnd.apple.mpegURL; codecs=\"avc1.42E01E, ac-3\"",
    "application/x-mpegurl; codecs=\"avc1.64001E, mp4a.40.34\"",
    "application/vnd.apple.mpegURL; codecs=\"avc1.64001E, mp4a.40.34\"",
    "video/x-matroska",
    "video/mkv",
    "video/mp4; codecs=\"vc-1\"",
    "video/mp4; codecs=\"dvh1.05.06\"",
    "video/mp4; codecs=\"dvh1.08.06\"",
    "video/mp4; codecs=\"dav1.10.06\"",
    "video/webm; codecs=\"vp8\"",
    "video/webm; codecs=\"vp9\"",
    "video/mp4; codecs=\"avc1.640029, mp4a.69\"",
    "video/mp4; codecs=\"avc1.640029, mp4a.6B\"",
    "video/mp4; codecs=\"avc1.640029, mp3\"",
    "video/mp4; codecs=\"avc1.640029, mp4a.40.2\"",
    "video/mp4; codecs=\"avc1.640029, mp4a.40.5\"",
    "video/mp4; codecs=\"avc1.640833\"",
    "video/mp4; codecs=\"avc1.640834\"",
    "video/mp4; codecs=\"avc1.6e0033\"",
    "video/mp4; codecs=\"hvc1.2.4.L153\"",
];

/// Every audio mime the port asks `canPlayType` about.
const AUDIO_MIMES: [&str; 11] = [
    "audio/ogg; codecs=\"opus\"",
    "audio/mp3",
    "audio/aac",
    "audio/flac",
    "audio/alac",
    "audio/webm",
    "audio/wma",
    "audio/wav",
    "audio/ogg",
    "audio/oga",
    "audio/x-caf; codecs=\"opus\"",
];

/// The mimes every authored environment admits however narrow it is, so that
/// every run has an h264 path for the HLS split to be measured against.
const FLOOR: [&str; 2] = [
    "video/mp4; codecs=\"avc1.42E01E, mp4a.40.2\"",
    "video/mp4; codecs=\"avc1.640029, mp4a.40.2\"",
];

fn answering(mimes: &[&'static str], answer: &'static str) -> Table {
    let mut table: Table = mimes.iter().map(|mime| (*mime, answer)).collect();
    for mime in FLOOR {
        if let Some(held) = table.get_mut(mime) {
            *held = "probably";
        }
    }
    table
}

/// The environment spec `install` reads, whose every field names a capability
/// rather than a browser.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Environment {
    user_agent: &'static str,
    platform: &'static str,
    app_version: &'static str,
    max_touch_points: u32,
    has_touch_start: bool,
    tizen_global: bool,
    animates: bool,
    width: i32,
    height: i32,
    device_pixel_ratio: f64,
    speakers: Option<i32>,
    media_source: bool,
    text_tracks: bool,
    canvas2d: bool,
    audio_tracks: bool,
    video: Table,
    audio: Table,
}

/// The `appSettings` object `browserDeviceProfile.js` and `apphost.js` read,
/// which addresses the burn-in and PGS entries by the keys `get` is called with.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AppSettings {
    enable_dts: bool,
    enable_true_hd: bool,
    enable_hi10p: bool,
    disable_vbr_audio: bool,
    always_remux_flac: bool,
    always_remux_mp3: bool,
    always_burn_in_subtitle_when_transcoding: bool,
    max_video_width: i32,
    limit_supported_video_resolution: bool,
    preferred_transcode_video_codec: String,
    preferred_transcode_video_audio_codec: String,
    #[serde(rename = "subtitleburnin")]
    subtitle_burn_in: BurnIn,
    #[serde(rename = "subtitlerenderpgs")]
    subtitle_render_pgs: &'static str,
}

/// The `BaseItemDto` `getDeviceProfile` and `getBaseProfileOptions` are called
/// with.
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct Item {
    run_time_ticks: Option<i64>,
    media_type: MediaKind,
}

/// One differential run: the environment both sides read, the settings pair both
/// sides read, and the item `getDeviceProfile` is called with.
struct Run {
    agent: &'static str,
    environment: Environment,
    shared: Shared,
    account: Account,
    run_time_ticks: Option<i64>,
    kind: MediaKind,
}

/// The `appSettings` and `userSettings` objects `browserDeviceProfile.js` reads,
/// built from the port's own values.
fn settings(shared: &Shared, account: &Account) -> (JsValue, JsValue) {
    let app = AppSettings {
        enable_dts: shared.enable_dts,
        enable_true_hd: shared.enable_true_hd,
        enable_hi10p: shared.enable_hi10p,
        disable_vbr_audio: shared.disable_vbr_audio,
        always_remux_flac: shared.always_remux_flac,
        always_remux_mp3: shared.always_remux_mp3,
        always_burn_in_subtitle_when_transcoding: shared.always_burn_in_subtitle_when_transcoding,
        max_video_width: match shared.max_video_width {
            MaxVideoWidth::Screen => -1,
            MaxVideoWidth::Unset => 0,
            MaxVideoWidth::Fixed(width) => width.count(),
        },
        limit_supported_video_resolution: shared.limit_supported_video_resolution,
        preferred_transcode_video_codec: shared.preferred_transcode_video_codec.clone(),
        preferred_transcode_video_audio_codec: shared.preferred_transcode_video_audio_codec.clone(),
        subtitle_burn_in: shared.subtitle_burn_in,
        subtitle_render_pgs: if shared.subtitle_render_pgs {
            "true"
        } else {
            "false"
        },
    };
    (crossed(&app), crossed(account))
}

/// What `value` crosses into JavaScript as.
fn crossed<T: Serialize>(value: &T) -> JsValue {
    let rendered = failure::rendered(Text::FailureStored, value).expect("the value renders");
    js_sys::JSON::parse(&rendered).expect("the value parses")
}

fn item(run_time_ticks: Option<i64>, kind: MediaKind) -> JsValue {
    crossed(&Item {
        run_time_ticks,
        media_type: kind,
    })
}

/// The four settings pairs every agent is crossed with: the defaults, every
/// `appSettings` flag on, `BurnIn::All` with PGS rendering on, and a fixed max
/// video width with a preferred transcode codec pair.
fn pairs(browser: &Browser) -> Vec<(Shared, Account)> {
    let account = Account::defaults(browser);
    vec![
        (Shared::default(), account),
        (
            Shared {
                disable_vbr_audio: true,
                always_remux_flac: true,
                always_remux_mp3: true,
                enable_dts: true,
                enable_true_hd: true,
                enable_hi10p: true,
                always_burn_in_subtitle_when_transcoding: true,
                limit_supported_video_resolution: true,
                ..Shared::default()
            },
            Account {
                limit_segment_length: true,
                allowed_audio_channels: Channels::of(6),
                ..account
            },
        ),
        (
            Shared {
                subtitle_burn_in: BurnIn::All,
                subtitle_render_pgs: true,
                ..Shared::default()
            },
            account,
        ),
        (
            Shared {
                max_video_width: MaxVideoWidth::Fixed(
                    Pixels::of(1920).expect("a positive width is a count of pixels"),
                ),
                preferred_transcode_video_codec: "hevc".to_owned(),
                preferred_transcode_video_audio_codec: "ac3".to_owned(),
                ..Shared::default()
            },
            Account {
                prefer_fmp4_hls_container: !account.prefer_fmp4_hls_container,
                ..account
            },
        ),
    ]
}

/// Each of `reference::AGENTS` crossed with the four settings pairs.
fn runs() -> Vec<Run> {
    let mut runs = Vec::new();
    for (at, agent) in reference::AGENTS.into_iter().enumerate() {
        let admitted = match at % 3 {
            0 => "probably",
            1 => "",
            _ => "maybe",
        };
        let environment = Environment {
            user_agent: agent,
            platform: "MacIntel",
            app_version: "5.0 (Macintosh; Intel Mac OS X 10_15_7)",
            max_touch_points: 0,
            has_touch_start: false,
            tizen_global: false,
            animates: true,
            width: 1920,
            height: 1080,
            device_pixel_ratio: 2.0,
            speakers: Some(6),
            media_source: at % 2 == 0,
            text_tracks: true,
            canvas2d: true,
            audio_tracks: at % 2 == 0,
            video: answering(&VIDEO_MIMES, admitted),
            audio: answering(&AUDIO_MIMES, admitted),
        };
        reference::installed(&environment);
        let browser = Browser::detect(&Runtime::probe());
        for (shared, account) in pairs(&browser) {
            runs.push(Run {
                agent,
                environment: Environment {
                    video: answering(&VIDEO_MIMES, admitted),
                    audio: answering(&AUDIO_MIMES, admitted),
                    ..environment
                },
                shared,
                account,
                run_time_ticks: if at % 2 == 0 {
                    Some(6_000_000_000)
                } else {
                    None
                },
                kind: MediaKind::Video,
            });
        }
    }
    runs
}

/// What one run installs, and what both sides then read from it.
struct Worn {
    browser: Browser,
    detected: JsValue,
    engine: Engine,
    media: Media,
}

fn worn(run: &Run) -> Worn {
    reference::installed(&run.environment);
    Worn {
        browser: Browser::detect(&Runtime::probe()),
        detected: reference::detect_browser(run.agent),
        engine: Engine::read(),
        media: Media::created().expect("the document creates a video and an audio element"),
    }
}

fn built(run: &Run, worn: &Worn) -> DeviceProfile {
    let options = Options::of(
        &worn.browser,
        &worn.engine,
        &worn.media,
        run.run_time_ticks,
        run.kind,
    );
    build(
        &worn.browser,
        &worn.engine,
        &worn.media,
        &run.shared,
        &run.account,
        &options,
    )
}

async fn theirs(run: &Run, worn: &Worn) -> JsValue {
    let (app, user) = settings(&run.shared, &run.account);
    let promise = reference::device_profile(
        &worn.detected,
        &app,
        &user,
        &item(run.run_time_ticks, run.kind),
    );
    wasm_bindgen_futures::JsFuture::from(promise)
        .await
        .expect("the reference builds a profile")
}

fn rendered(value: &JsValue) -> String {
    String::from(js_sys::JSON::stringify(value).expect("the reference object renders"))
}

#[wasm_bindgen_test]
async fn every_run_builds_the_reference_profile_byte_for_byte() {
    for run in runs() {
        let worn = worn(&run);
        let ours = failure::rendered(Text::FailureStored, &built(&run, &worn))
            .expect("the built profile renders");
        assert_eq!(
            ours,
            rendered(&theirs(&run, &worn).await),
            "the port and the reference disagree for {}",
            run.agent
        );
    }
}

#[wasm_bindgen_test]
fn every_run_builds_the_reference_base_profile_options() {
    for run in runs() {
        let worn = worn(&run);
        let options = Options::of(
            &worn.browser,
            &worn.engine,
            &worn.media,
            run.run_time_ticks,
            run.kind,
        );
        let ours = failure::rendered(Text::FailureStored, &options.disable_hls_video_audio_codecs)
            .expect("the struck codecs render");
        let theirs =
            reference::base_profile_options(&worn.detected, &item(run.run_time_ticks, run.kind));
        let struck =
            js_sys::Reflect::get(&theirs, &JsValue::from_str("disableHlsVideoAudioCodecs"))
                .expect("the reference answers the struck codecs");
        assert_eq!(
            ours,
            rendered(&struck),
            "the port and the reference disagree for {}",
            run.agent
        );
    }
}

#[wasm_bindgen_test]
fn secondary_audio_agrees_with_the_reference() {
    for run in runs() {
        let worn = worn(&run);
        assert_eq!(
            secondary_audio(&worn.browser, &worn.media),
            reference::can_play_secondary_audio(&worn.detected, worn.media.element()),
            "the port and the reference disagree for {}",
            run.agent
        );
    }
}

#[wasm_bindgen_test]
fn hls_js_eligibility_agrees_with_the_reference() {
    for run in runs() {
        let worn = worn(&run);
        let ticks = match run.run_time_ticks {
            Some(ticks) => JsValue::from_f64(ticks as f64),
            None => JsValue::NULL,
        };
        let kind = match run.kind {
            MediaKind::Video => "Video",
            MediaKind::Audio => "Audio",
        };
        assert_eq!(
            enable_hls_js_player(
                &worn.browser,
                &worn.engine,
                &worn.media,
                run.run_time_ticks,
                run.kind
            ),
            reference::enable_hls_js_player(&worn.detected, &ticks, kind),
            "the port and the reference disagree for {}",
            run.agent
        );
    }
}

#[wasm_bindgen_test]
async fn a_run_offering_hevc_in_fmp4_also_offers_h264_in_mpeg_ts() {
    let mut offered = false;
    for run in runs() {
        let worn = worn(&run);
        let profile = built(&run, &worn);
        let fmp4 = profile.transcoding_profiles.iter().any(|transcoding| {
            transcoding.container == "mp4"
                && transcoding
                    .video_codec
                    .as_deref()
                    .is_some_and(|codecs| codecs.split(',').any(|codec| codec == "hevc"))
        });
        if !fmp4 {
            continue;
        }
        offered = true;
        assert!(
            profile.transcoding_profiles.iter().any(|transcoding| {
                transcoding.container == "ts"
                    && transcoding
                        .video_codec
                        .as_deref()
                        .is_some_and(|codecs| codecs.split(',').any(|codec| codec == "h264"))
            }),
            "a run offering hevc in fmp4 offers no h264 in mpeg-ts for {}",
            run.agent
        );
    }
    assert!(offered, "no run offered hevc in fmp4");
}

#[wasm_bindgen_test]
async fn no_run_carries_an_encode_subtitle_entry() {
    for run in runs() {
        let worn = worn(&run);
        for subtitle in built(&run, &worn).subtitle_profiles {
            assert_eq!(
                subtitle.method,
                SubtitleMethod::External,
                "a run carries a subtitle entry that is not external for {}",
                run.agent
            );
        }
    }
}
