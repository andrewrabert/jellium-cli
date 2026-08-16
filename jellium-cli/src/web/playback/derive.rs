//! The stream the browser is handed, derived from the negotiated source the
//! way `createStreamInfo` derives it, and the universal audio url an audio item
//! is answered from without any `PlaybackInfo` at all.

use jellium_protocol::{
    Bitrate, Method, PlayRequest, Playable, PlaybackRefused,
    profile::{self, DeviceProfile},
};
use jellyfin_api::types::{MediaSourceInfo, MediaStreamProtocol, MediaStreamType};
use uuid::Uuid;

use super::negotiate::Negotiated;
use super::pointed::Pointed;
use crate::web::identity::Identity;
use crate::web::upstream::Upstream;
use crate::web::wire::{self, Query};
use crate::web::{manifest, route};

/// The streaming protocol the Jellyfin server reported the source under.
fn sub_protocol(source: &MediaSourceInfo) -> Option<profile::Protocol> {
    match source.transcoding_sub_protocol? {
        MediaStreamProtocol::Hls => Some(profile::Protocol::Hls),
        MediaStreamProtocol::Http => Some(profile::Protocol::Http),
    }
}

/// True when the source carries a video stream, which is what decides the
/// endpoint family the static stream url is built under.
fn visual(source: &MediaSourceInfo) -> bool {
    source
        .media_streams
        .as_deref()
        .unwrap_or_default()
        .iter()
        .any(|stream| stream.type_ == Some(MediaStreamType::Video))
}

/// The static stream url a source that direct-plays or direct-streams is loaded
/// from, built against `base` and carrying the reference's own keys and no
/// `playSessionId`.
fn static_stream(
    upstream: &Upstream,
    source: &MediaSourceInfo,
    item: Uuid,
    identity: &Identity,
    base: &reqwest::Url,
) -> reqwest::Url {
    let family = if visual(source) { "Videos" } else { "Audio" };
    let container = source.container.clone().unwrap_or_default().to_lowercase();
    let query = Query::new()
        .set("Static", true)
        .maybe("mediaSourceId", source.id.clone())
        .set("deviceId", identity.device_id())
        .set("ApiKey", upstream.api_key())
        .maybe("Tag", source.e_tag.clone())
        .maybe("LiveStreamId", source.live_stream_id.clone());
    wire::url(base, &format!("{family}/{item}/stream.{container}"), &query)
}

/// The upstream url and the play method one of jellyfin-web's ordered branches
/// answers, and nothing when no branch does.
fn branched(
    upstream: &Upstream,
    negotiated: &Negotiated,
    request: &PlayRequest,
    identity: &Identity,
    base: &reqwest::Url,
) -> Option<(String, Method)> {
    let source = &negotiated.source;
    let transcode = Method::Transcode {
        subtitle_burn_in: request.always_burn_in_subtitle_when_transcoding,
    };
    if negotiated.enable_direct_play
        && let Some(path) = source.path.as_deref()
    {
        return Some((path.to_string(), Method::DirectPlay));
    }
    if let Some(stream) = negotiated.stream_url.as_deref() {
        return Some((stream.to_string(), transcode));
    }
    if source.supports_direct_play == Some(true) || source.supports_direct_stream == Some(true) {
        let method = if source.supports_direct_play == Some(true) {
            Method::DirectPlay
        } else {
            Method::DirectStream
        };
        let url = static_stream(upstream, source, request.item, identity, base);
        return Some((url.to_string(), method));
    }
    if source.supports_transcoding == Some(true)
        && let Some(transcoding) = source.transcoding_url.as_deref()
    {
        return Some((transcoding.to_string(), transcode));
    }
    if source.supports_direct_play == Some(true)
        && let Some(path) = source.path.as_deref()
    {
        return Some((path.to_string(), Method::DirectPlay));
    }
    None
}

/// The playable stream, chosen by jellyfin-web's ordered branches and rewritten
/// to the local origin.
/// `negotiated.enable_direct_play` answers branch one, whose url is the
/// source's `Path`; then `StreamUrl`, then a static stream under
/// `SupportsDirectPlay` or `SupportsDirectStream`, then `TranscodingUrl`, then
/// `Path` again.
/// A url the relay's route table does not admit refuses the plan, naming the
/// live shape when the source is a live stream.
// reference: create-stream-info — playbackmanager.js:2827-2881
// `&Upstream` because branch three's static-stream query carries `ApiKey`,
// which reaches it through `Upstream::api_key`
pub fn playable(
    upstream: &Upstream,
    negotiated: &Negotiated,
    request: &PlayRequest,
    identity: &Identity,
    base: &reqwest::Url,
    seen: &route::Seen,
    pointed: &Pointed,
) -> Result<Playable, PlaybackRefused> {
    let source = &negotiated.source;
    let refused = |reference: &str| {
        if negotiated.live {
            PlaybackRefused::LiveNotRelayable {
                shape: route::shape(reference),
            }
        } else {
            PlaybackRefused::NotRelayable
        }
    };
    let (url, method) = branched(upstream, negotiated, request, identity, base)
        .ok_or(PlaybackRefused::NoPlayableSource)?;
    let path = manifest::resolved(&url, base, base, seen, pointed).map_err(|_| refused(&url))?;

    Ok(Playable {
        path,
        method,
        sub_protocol: sub_protocol(source),
        container: source.container.clone(),
        run_time_ticks: source.run_time_ticks,
        remote: source.is_remote == Some(true),
        codecs: source
            .media_streams
            .as_deref()
            .unwrap_or_default()
            .iter()
            .filter_map(|stream| stream.codec.clone())
            .collect(),
    })
}

/// The containers an audio item may be direct-played in, joined the way
/// `getAudioStreamUrlFromDeviceProfile` joins them.
fn direct_play_containers(profile: &DeviceProfile) -> String {
    let mut joined = String::new();
    for entry in &profile.direct_play_profiles {
        let profile::DirectPlayProfile::Audio {
            container,
            audio_codec,
        } = entry
        else {
            continue;
        };
        if !joined.is_empty() {
            joined.push(',');
        }
        joined.push_str(container);
        if let Some(audio_codec) = audio_codec {
            joined.push('|');
            joined.push_str(audio_codec);
        }
    }
    joined
}

/// The spelling the Jellyfin server reads a transcoding protocol under.
fn protocol(protocol: profile::Protocol) -> &'static str {
    match protocol {
        profile::Protocol::Hls => "hls",
        profile::Protocol::Http => "http",
    }
}

/// The play session the reference counts up once per universal url, seeded from
/// the clock the way `startingPlaySession` is.
fn next_play_session() -> u64 {
    static COUNTER: std::sync::OnceLock<std::sync::atomic::AtomicU64> = std::sync::OnceLock::new();
    let counter = COUNTER.get_or_init(|| {
        std::sync::atomic::AtomicU64::new(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|since| u64::try_from(since.as_millis()).unwrap_or_default())
                .unwrap_or_default(),
        )
    });
    counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1
}

/// The universal audio path and the query it carries, in the reference's own
/// key order.
/// `MaxAudioSampleRate` and `MaxAudioBitDepth` are absent and
/// `MaxStreamingBitrate` is `ceiling`: `getAudioMaxValues` selects conditions on
/// `AudioBitDepth`, `AudioSampleRate` and `AudioBitrate`, and the builder emits
/// no condition on any of the three, so it answers all-null for every profile
/// this client sends.
/// The url is the relay's own upstream url and never reaches the browser:
/// `playable` hands it to `manifest::relay_path`, which answers a same-origin
/// path, so `ApiKey` is minted here and stripped before the browser sees
/// anything.
// reference: get-audio-stream-url — playbackmanager.js:305-326
// reference: enable-remote-media — playbackmanager.js:323
// EnableRemoteMedia is the constant false: AppFeature.RemoteAudio is declared
// in constants/appFeature.ts and pushed into `features` nowhere, and
// apphost.js:365 answers `supports` from that list
pub fn universal(
    upstream: &Upstream,
    item: Uuid,
    profile: &DeviceProfile,
    ceiling: Bitrate,
    identity: &Identity,
    start_ticks: i64,
) -> (String, Query) {
    let transcoding = profile.transcoding_profiles.iter().find(|entry| {
        entry.kind == profile::MediaKind::Audio && entry.context == profile::Context::Streaming
    });
    let query = Query::new()
        .set("UserId", upstream.user_id())
        .set("DeviceId", identity.device_id())
        .set("MaxStreamingBitrate", ceiling.bits_per_second())
        .set("Container", direct_play_containers(profile))
        .maybe(
            "TranscodingContainer",
            transcoding.map(|entry| entry.container.clone()),
        )
        .maybe(
            "TranscodingProtocol",
            transcoding.map(|entry| protocol(entry.protocol)),
        )
        .maybe(
            "AudioCodec",
            transcoding.map(|entry| entry.audio_codec.clone()),
        )
        .set("ApiKey", upstream.api_key())
        .set("PlaySessionId", next_play_session())
        .set("StartTimeTicks", start_ticks)
        .set("EnableRedirection", true)
        .set("EnableRemoteMedia", false)
        .maybe(
            "EnableAudioVbrEncoding",
            transcoding.and_then(|entry| entry.enable_audio_vbr_encoding),
        );
    (format!("Audio/{item}/universal"), query)
}

/// The value `PlaySessionId` carries in `url`'s query, read the way `getParam`
/// reads it: the name matches without regard to case, `+` stands for a space
/// and the value is percent-decoded.
// reference: get-param — playbackmanager.js:213-225
pub fn play_session(url: &str) -> Option<String> {
    let query = url.split_once('?').map(|(_, rest)| rest)?;
    let query = query.split('#').next().unwrap_or_default();
    query.split('&').find_map(|pair| {
        let (name, value) = pair.split_once('=')?;
        name.eq_ignore_ascii_case("PlaySessionId").then(|| {
            percent_encoding::percent_decode_str(&value.replace('+', " "))
                .decode_utf8_lossy()
                .into_owned()
        })
    })
}
