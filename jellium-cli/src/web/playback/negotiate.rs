use jellium_protocol::{
    Bitrate, Failure, PlayRequest, PlaybackRefused, SourceChoice, StreamIndex, Subtitles,
    profile::{DeviceProfile, MediaKind},
};
use jellyfin_api::types::{
    BaseItemDto, LiveStreamResponse, MediaSourceInfo, PlaybackErrorCode, PlaybackInfoResponse,
};
use serde::Serialize;
use uuid::Uuid;

use super::{derive, reachable};
use crate::web::identity::Identity;
use crate::web::upstream::Upstream;
use crate::web::wire::{self, Query};

/// The body `POST /Items/{id}/PlaybackInfo` carries, declared in the order
/// `getPlaybackInfo` assigns its keys.
// reference: get-playback-info — playbackmanager.js:434-503
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Body {
    pub user_id: Uuid,
    pub start_time_ticks: i64,
    pub is_playback: bool,
    pub auto_open_live_stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_stream_index: Option<StreamIndex>,
    /// Three cases, not two: `Default` writes no key, `Off` writes `-1`, and a
    /// chosen stream writes its number.
    // reference: set-subtitle-stream-index — playbackmanager.js:1530, :1549
    #[serde(serialize_with = "numbered", skip_serializing_if = "unnumbered")]
    pub subtitle_stream_index: Subtitles,
    #[serde(serialize_with = "numbered", skip_serializing_if = "unnumbered")]
    pub secondary_subtitle_stream_index: Subtitles,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_direct_play: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_direct_stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_video_stream_copy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_audio_stream_copy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_source_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_stream_id: Option<String>,
    pub max_streaming_bitrate: Bitrate,
    pub always_burn_in_subtitle_when_transcoding: bool,
    pub device_profile: DeviceProfile,
}

/// True where the choice is the source's own default, which the reference
/// leaves out of the body entirely.
fn unnumbered(subtitles: &Subtitles) -> bool {
    subtitles.number().is_none()
}

/// The number the Jellyfin server reads the choice as: `-1` for none and the
/// stream's own number otherwise.
fn numbered<S: serde::Serializer>(subtitles: &Subtitles, serializer: S) -> Result<S::Ok, S::Error> {
    match subtitles.number() {
        Some(number) => serializer.serialize_i32(number),
        None => serializer.serialize_none(),
    }
}

/// The body `POST /LiveStreams/Open` carries; everything else it names sits in
/// the query.
// reference: get-live-stream — playbackmanager.js:536-574
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Opening {
    pub device_profile: DeviceProfile,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_token: Option<String>,
}

/// What the Jellyfin server settled on for one play request.
pub struct Negotiated {
    pub play_session: String,
    pub source: MediaSourceInfo,
    /// The url the client wrote onto the source itself, which only the audio
    /// short circuit mints and which no Jellyfin answer carries.
    pub stream_url: Option<String>,
    pub sources: Vec<SourceChoice>,
    pub live_stream: Option<String>,
    /// The client's own direct-play answer, written onto the chosen version the
    /// way the reference writes it, which `create-stream-info`'s first branch
    /// reads back.
    // reference: get-optimal-media-source — playbackmanager.js:516
    pub enable_direct_play: bool,
    pub max_bitrate: Bitrate,
    /// True when the chosen source is a live stream this session opened.
    pub live: bool,
}

/// A negotiation outcome the browser is shown, or a transport failure.
pub enum Refused {
    Playback(PlaybackRefused),
    Upstream(Failure),
}

fn choices(sources: &[MediaSourceInfo]) -> Vec<SourceChoice> {
    sources
        .iter()
        .filter_map(|source| {
            let id = source.id.clone()?;
            let name = source.name.clone().unwrap_or_else(|| id.clone());
            Some(SourceChoice { id, name })
        })
        .collect()
}

fn refused(code: PlaybackErrorCode) -> PlaybackRefused {
    match code {
        PlaybackErrorCode::NoCompatibleStream => PlaybackRefused::NoPlayableSource,
        PlaybackErrorCode::NotAllowed => PlaybackRefused::TranscodeRefused {
            code: "NotAllowed".to_string(),
        },
        PlaybackErrorCode::RateLimitExceeded => PlaybackRefused::TranscodeRefused {
            code: "RateLimitExceeded".to_string(),
        },
    }
}

/// True where the item is answered from the profile alone and no
/// `PlaybackInfo` is posted; every reader of that branch asks this.
fn short_circuited(kind: MediaKind) -> bool {
    kind == MediaKind::Audio
}

/// The one source an audio item is answered from, carrying the universal url
/// the browser's own profile builds, with no `PlaybackInfo` posted at all.
// reference: audio-stream-short-circuit — playbackmanager.js:416-426
fn short_circuit(
    upstream: &Upstream,
    request: &PlayRequest,
    item: &BaseItemDto,
    identity: &Identity,
    ceiling: Bitrate,
) -> Result<Negotiated, Refused> {
    let (path, query) = derive::universal(
        upstream,
        request.item,
        &request.profile,
        ceiling,
        identity,
        request.start_ticks,
    );
    let url = wire::url(upstream.link().base(), &path, &query).to_string();
    let play_session =
        derive::play_session(&url).ok_or(Refused::Playback(PlaybackRefused::NoPlayableSource))?;
    let source = MediaSourceInfo {
        id: Some(request.item.simple().to_string()),
        media_streams: Some(Vec::new()),
        run_time_ticks: item.run_time_ticks,
        ..MediaSourceInfo::default()
    };
    Ok(Negotiated {
        play_session,
        sources: choices(std::slice::from_ref(&source)),
        source,
        stream_url: Some(url),
        live_stream: None,
        enable_direct_play: false,
        max_bitrate: ceiling,
        live: false,
    })
}

/// Posts `PlaybackInfo` once with the profile the browser built, picks the
/// requested source or the first one, and opens a live stream when the chosen
/// source requires opening and carries none.
/// A `NoCompatibleStream` error code reads as `NoPlayableSource`; `NotAllowed`
/// and `RateLimitExceeded` read as `TranscodeRefused`; an item with no source
/// reads as `NoMediaSource`.
/// A live source the Jellyfin server will not open reads as `NoTuner`, without
/// retry and naming nothing else.
/// A re-negotiation for the channel a held live session was playing that finds
/// `resuming` gone from the offered sources reads as `TunerGone`.
/// An `Audio` item is answered from the profile alone, its one source carrying
/// the universal url, the item's id and the item's run time, and posts no
/// `PlaybackInfo`; every other item posts once.
// reference: get-playback-media-source — playbackmanager.js:2941-2976
// `item` is the BaseItemDto the reference's caller already holds when it calls
// getPlaybackInfo; MediaType answers the audio short circuit and RunTimeTicks
// fills the synthesized source
pub async fn negotiate(
    upstream: &Upstream,
    request: &PlayRequest,
    item: &BaseItemDto,
    identity: &Identity,
    ceiling: Bitrate,
    resuming: Option<&str>,
) -> Result<Negotiated, Refused> {
    if short_circuited(super::kind(item)) {
        return short_circuit(upstream, request, item, identity, ceiling);
    }

    let user = upstream.user_id();
    let burn_in = request.always_burn_in_subtitle_when_transcoding;

    let body = Body {
        user_id: user,
        start_time_ticks: request.start_ticks,
        is_playback: true,
        auto_open_live_stream: true,
        audio_stream_index: request.audio_stream,
        subtitle_stream_index: request.subtitles,
        secondary_subtitle_stream_index: Subtitles::Default,
        enable_direct_play: request.allow_direct_play,
        enable_direct_stream: request.allow_direct_stream,
        allow_video_stream_copy: request.allow_video_stream_copy,
        allow_audio_stream_copy: request.allow_audio_stream_copy,
        media_source_id: request.media_source.clone(),
        live_stream_id: None,
        max_streaming_bitrate: ceiling,
        always_burn_in_subtitle_when_transcoding: burn_in,
        device_profile: request.profile.clone(),
    };

    let negotiated: PlaybackInfoResponse = wire::posted(
        upstream,
        &format!("Items/{}/PlaybackInfo", request.item),
        &Query::new(),
        &body,
    )
    .await
    .map_err(Refused::Upstream)?;

    if let Some(code) = negotiated.error_code {
        return Err(Refused::Playback(refused(code)));
    }

    let play_session = negotiated
        .play_session_id
        .clone()
        .ok_or(Refused::Playback(PlaybackRefused::NoPlayableSource))?;
    if let Some(resuming) = resuming
        && !negotiated
            .media_sources
            .iter()
            .any(|source| source.id.as_deref() == Some(resuming))
    {
        return Err(Refused::Playback(PlaybackRefused::TunerGone));
    }

    let sources = choices(&negotiated.media_sources);
    let (mut source, mut enable_direct_play) =
        reachable::optimal(upstream, &request.grants, negotiated.media_sources)
            .await
            .ok_or(Refused::Playback(PlaybackRefused::NoMediaSource))?;

    let mut live_stream = source.live_stream_id.clone();
    let live = source.requires_opening == Some(true);
    if live && live_stream.is_none() {
        let query = Query::new()
            .set("UserId", user)
            .set("StartTimeTicks", request.start_ticks)
            .set("ItemId", request.item)
            .set("PlaySessionId", &play_session)
            .set("MaxStreamingBitrate", ceiling.bits_per_second());
        let opened: LiveStreamResponse = wire::posted(
            upstream,
            "LiveStreams/Open",
            &query,
            &Opening {
                device_profile: request.profile.clone(),
                open_token: source.open_token.clone(),
            },
        )
        .await
        .map_err(|_| Refused::Playback(PlaybackRefused::NoTuner))?;
        source = opened
            .media_source
            .ok_or(Refused::Playback(PlaybackRefused::NoTuner))?;
        live_stream = source.live_stream_id.clone();
        enable_direct_play = false;
    }

    Ok(Negotiated {
        play_session,
        source,
        stream_url: None,
        sources,
        live_stream,
        enable_direct_play,
        max_bitrate: ceiling,
        live,
    })
}

/// The body the port builds and the body `getPlaybackInfo` builds, compared
/// through `tools/reference/parity.mjs`.
#[cfg(test)]
mod differential {
    use super::*;

    /// One differential case: what `getPlaybackInfo` is called with, in the
    /// reference's own vocabulary, which is what crosses to Node as JSON.
    struct Case {
        user: Uuid,
        item: Uuid,
        kind: MediaKind,
        profile: DeviceProfile,
        media_source: Option<String>,
        live_stream: Option<String>,
        start_ticks: i64,
        playback: bool,
        audio_stream: Option<i32>,
        /// `-1` is no subtitle at all and an absent number is the source's own
        /// default, which is the three-way the reference's `!= null` reads.
        subtitle_stream: Option<i32>,
        secondary_subtitle_stream: Option<i32>,
        direct_play: Option<bool>,
        direct_stream: Option<bool>,
        video_stream_copy: Option<bool>,
        audio_stream_copy: Option<bool>,
        max_bitrate: i64,
        always_burn_in_subtitle_when_transcoding: bool,
    }

    /// A video case with every optional absent, which every other case is
    /// written as a departure from.
    fn plain() -> Case {
        Case {
            user: Uuid::from_u128(0x5e5),
            item: Uuid::from_u128(0x17e),
            kind: MediaKind::Video,
            profile: DeviceProfile::default(),
            media_source: None,
            live_stream: None,
            start_ticks: 0,
            playback: true,
            audio_stream: None,
            subtitle_stream: None,
            secondary_subtitle_stream: None,
            direct_play: None,
            direct_stream: None,
            video_stream_copy: None,
            audio_stream_copy: None,
            max_bitrate: 1_500_000,
            always_burn_in_subtitle_when_transcoding: false,
        }
    }

    /// Every optional present, every optional absent, an audio item, a live
    /// stream being opened, and each remaining optional alone.
    fn cases() -> Vec<Case> {
        vec![
            plain(),
            Case {
                media_source: Some("d0000000000000000000000000000000".to_string()),
                live_stream: Some("live-stream-1".to_string()),
                start_ticks: 12_345_678,
                playback: false,
                audio_stream: Some(1),
                subtitle_stream: Some(2),
                secondary_subtitle_stream: Some(3),
                direct_play: Some(true),
                direct_stream: Some(false),
                video_stream_copy: Some(true),
                audio_stream_copy: Some(false),
                max_bitrate: 120_000_000,
                always_burn_in_subtitle_when_transcoding: true,
                ..plain()
            },
            Case {
                kind: MediaKind::Audio,
                ..plain()
            },
            Case {
                live_stream: Some("live-stream-1".to_string()),
                ..plain()
            },
            Case {
                media_source: Some("d0000000000000000000000000000000".to_string()),
                ..plain()
            },
            Case {
                audio_stream: Some(0),
                ..plain()
            },
            Case {
                subtitle_stream: Some(-1),
                ..plain()
            },
            Case {
                subtitle_stream: Some(4),
                ..plain()
            },
            Case {
                secondary_subtitle_stream: Some(-1),
                ..plain()
            },
            Case {
                direct_play: Some(false),
                ..plain()
            },
            Case {
                direct_stream: Some(true),
                ..plain()
            },
            Case {
                video_stream_copy: Some(false),
                ..plain()
            },
            Case {
                audio_stream_copy: Some(true),
                ..plain()
            },
            Case {
                start_ticks: 1,
                ..plain()
            },
            Case {
                playback: false,
                ..plain()
            },
            Case {
                always_burn_in_subtitle_when_transcoding: true,
                ..plain()
            },
        ]
    }

    /// The arguments `getPlaybackInfo` is called with, as `parity.mjs` reads
    /// them off stdin. This is the one site the case crosses into the
    /// reference's vocabulary.
    fn asked(case: &Case) -> String {
        let named = serde_json::json!({
            "userId": case.user.to_string(),
            "item": {
                "Id": case.item.to_string(),
                "MediaType": match case.kind {
                    MediaKind::Audio => "Audio",
                    MediaKind::Video => "Video",
                },
            },
            "mediaSourceId": case.media_source,
            "liveStreamId": case.live_stream,
            "alwaysBurnInSubtitleWhenTranscoding": case.always_burn_in_subtitle_when_transcoding,
            "options": {
                "startPosition": case.start_ticks,
                "isPlayback": case.playback,
                "audioStreamIndex": case.audio_stream,
                "subtitleStreamIndex": case.subtitle_stream,
                "secondarySubtitleStreamIndex": case.secondary_subtitle_stream,
                "enableDirectPlay": case.direct_play,
                "enableDirectStream": case.direct_stream,
                "allowVideoStreamCopy": case.video_stream_copy,
                "allowAudioStreamCopy": case.audio_stream_copy,
                "maxBitrate": case.max_bitrate,
            },
        })
        .to_string();
        // the profile is spliced in as the port serialized it, since a
        // `serde_json::Value` sorts an object's keys and the reference echoes
        // the profile back into the body in the order it received it
        let profile = serde_json::to_string(&case.profile).expect("the profile serializes");
        let inner = named
            .strip_prefix('{')
            .and_then(|named| named.strip_suffix('}'))
            .expect("a serialized object");
        format!("{{\"deviceProfile\":{profile},{inner}}}")
    }

    /// The reference's answer for `case`, and `None` where it posts nothing;
    /// runs `node tools/reference/parity.mjs` and feeds the case on stdin.
    fn reference(case: &Case) -> Option<String> {
        use std::io::Write as _;

        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("the package directory sits inside the workspace");
        let mut node = std::process::Command::new("node")
            .arg(root.join("tools").join("reference").join("parity.mjs"))
            .current_dir(root)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .expect("node runs the pinned reference");
        node.stdin
            .as_mut()
            .expect("the reference reads its case on stdin")
            .write_all(asked(case).as_bytes())
            .expect("the case reaches the reference");
        let answered = node.wait_with_output().expect("the reference answers");
        assert!(
            answered.status.success(),
            "the reference refused the case: {}",
            String::from_utf8_lossy(&answered.stderr)
        );
        let body = String::from_utf8(answered.stdout).expect("the answer is text");
        (body != "null").then_some(body)
    }

    /// The body the port posts for `case`, and `None` where it posts none,
    /// serialized by the port's own serializer rather than by a copy of it.
    fn posted(case: &Case) -> Option<String> {
        if short_circuited(case.kind) {
            return None;
        }
        let body = Body {
            user_id: case.user,
            start_time_ticks: case.start_ticks,
            is_playback: case.playback,
            auto_open_live_stream: case.playback,
            audio_stream_index: case.audio_stream.and_then(StreamIndex::named),
            subtitle_stream_index: Subtitles::named(case.subtitle_stream),
            secondary_subtitle_stream_index: Subtitles::named(case.secondary_subtitle_stream),
            enable_direct_play: case.direct_play,
            enable_direct_stream: case.direct_stream,
            allow_video_stream_copy: case.video_stream_copy,
            allow_audio_stream_copy: case.audio_stream_copy,
            media_source_id: case.media_source.clone(),
            live_stream_id: case.live_stream.clone(),
            max_streaming_bitrate: Bitrate::of(case.max_bitrate),
            always_burn_in_subtitle_when_transcoding: case.always_burn_in_subtitle_when_transcoding,
            device_profile: case.profile.clone(),
        };
        Some(serde_json::to_string(&body).expect("the body serializes"))
    }

    #[test]
    fn every_case_builds_the_reference_body_byte_for_byte() {
        for case in cases().iter().filter(|case| case.kind == MediaKind::Video) {
            assert_eq!(posted(case), reference(case), "asked {}", asked(case));
        }
    }

    #[test]
    fn an_audio_case_posts_nothing_on_either_side() {
        let case = Case {
            kind: MediaKind::Audio,
            ..plain()
        };
        assert_eq!(reference(&case), None);
        assert_eq!(posted(&case), None);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::web::upstream::{Upstream, answering};
    use jellium_protocol::Quality;
    use jellyfin_api::types::MediaType;

    /// The keys the serialized body carries at its top level, in the order it
    /// carries them, which is what a JSON object's `Map` would not preserve.
    fn keys(body: &Body) -> Vec<String> {
        let rendered = serde_json::to_string(body).expect("the body serializes");
        let mut found = Vec::new();
        let mut depth = 0usize;
        let mut characters = rendered.chars().peekable();
        while let Some(character) = characters.next() {
            match character {
                '{' | '[' => depth += 1,
                '}' | ']' => depth -= 1,
                '"' => {
                    let mut name = String::new();
                    for character in characters.by_ref() {
                        if character == '"' {
                            break;
                        }
                        name.push(character);
                    }
                    if depth == 1 && characters.peek() == Some(&':') {
                        found.push(name);
                    }
                }
                _ => {}
            }
        }
        found
    }

    fn request() -> PlayRequest {
        PlayRequest {
            item: uuid::Uuid::nil(),
            media_source: None,
            audio_stream: None,
            subtitles: Subtitles::default(),
            start_ticks: 0,
            quality: Quality::Auto,
            profile: DeviceProfile::default(),
            always_burn_in_subtitle_when_transcoding: false,
            allow_direct_play: None,
            allow_direct_stream: None,
            allow_video_stream_copy: None,
            allow_audio_stream_copy: None,
            grants: jellium_protocol::HostGrants { remote_video: true },
            cinema_mode: true,
            fullscreen: true,
            start_index: None,
            reporting: jellium_protocol::report::Reporting {
                volume_level: 100,
                muted: false,
                repeat: jellium_protocol::Repeat::Off,
                shuffle: jellium_protocol::report::Shuffle::Sorted,
                playback_rate: 1.0,
                playlist_item_id: "playlistItem1".to_string(),
                queue: Vec::new(),
            },
        }
    }

    fn video() -> BaseItemDto {
        BaseItemDto {
            media_type: Some(MediaType::Video),
            ..BaseItemDto::default()
        }
    }

    fn device() -> Identity {
        Identity::of(jellium_protocol::Identity {
            device: "Firefox".to_owned(),
            device_id: uuid::Uuid::nil().to_string(),
        })
    }

    /// The keys `playbackInfoBody` assigns, in the order it assigns them.
    #[test]
    fn the_body_carries_the_reference_key_order() {
        let body = Body {
            user_id: uuid::Uuid::nil(),
            start_time_ticks: 5,
            is_playback: true,
            auto_open_live_stream: true,
            audio_stream_index: StreamIndex::named(1),
            subtitle_stream_index: Subtitles::named(Some(2)),
            secondary_subtitle_stream_index: Subtitles::named(Some(3)),
            enable_direct_play: Some(true),
            enable_direct_stream: Some(false),
            allow_video_stream_copy: Some(true),
            allow_audio_stream_copy: Some(false),
            media_source_id: Some("msid".to_string()),
            live_stream_id: Some("lsid".to_string()),
            max_streaming_bitrate: Bitrate::of(1000),
            always_burn_in_subtitle_when_transcoding: true,
            device_profile: DeviceProfile::default(),
        };
        assert_eq!(
            keys(&body),
            [
                "UserId",
                "StartTimeTicks",
                "IsPlayback",
                "AutoOpenLiveStream",
                "AudioStreamIndex",
                "SubtitleStreamIndex",
                "SecondarySubtitleStreamIndex",
                "EnableDirectPlay",
                "EnableDirectStream",
                "AllowVideoStreamCopy",
                "AllowAudioStreamCopy",
                "MediaSourceId",
                "LiveStreamId",
                "MaxStreamingBitrate",
                "AlwaysBurnInSubtitleWhenTranscoding",
                "DeviceProfile",
            ]
        );
    }

    /// A field the reference assigns only when present carries no key at all;
    /// the ceiling is not one of them, since the reference always resolves one.
    #[test]
    fn an_absent_field_carries_no_key() {
        let body = Body {
            user_id: uuid::Uuid::nil(),
            start_time_ticks: 0,
            is_playback: true,
            auto_open_live_stream: true,
            audio_stream_index: None,
            subtitle_stream_index: Subtitles::Default,
            secondary_subtitle_stream_index: Subtitles::Default,
            enable_direct_play: None,
            enable_direct_stream: None,
            allow_video_stream_copy: None,
            allow_audio_stream_copy: None,
            media_source_id: None,
            live_stream_id: None,
            max_streaming_bitrate: Bitrate::of(1000),
            always_burn_in_subtitle_when_transcoding: false,
            device_profile: DeviceProfile::default(),
        };
        assert_eq!(
            keys(&body),
            [
                "UserId",
                "StartTimeTicks",
                "IsPlayback",
                "AutoOpenLiveStream",
                "MaxStreamingBitrate",
                "AlwaysBurnInSubtitleWhenTranscoding",
                "DeviceProfile",
            ]
        );
    }

    #[tokio::test]
    async fn a_busy_tuner_reads_as_no_tuner() {
        let server = answering(204).await;
        server.live_tv.tuners_busy(true);
        let upstream = Upstream::stub(&server.base);

        let refused = negotiate(
            &upstream,
            &request(),
            &video(),
            &device(),
            Bitrate::of(1_500_000),
            None,
        )
        .await
        .err()
        .expect("a refusal");
        let Refused::Playback(PlaybackRefused::NoTuner) = refused else {
            panic!("a busy tuner reads as no tuner");
        };
    }

    #[tokio::test]
    async fn a_resume_whose_source_is_gone_reads_as_tuner_gone() {
        let server = answering(204).await;
        let upstream = Upstream::stub(&server.base);

        let refused = negotiate(
            &upstream,
            &request(),
            &video(),
            &device(),
            Bitrate::of(1_500_000),
            Some("a-source-that-left"),
        )
        .await
        .err()
        .expect("a refusal");
        let Refused::Playback(PlaybackRefused::TunerGone) = refused else {
            panic!("a resume whose source is gone reads as tuner gone");
        };

        negotiate(
            &upstream,
            &request(),
            &video(),
            &device(),
            Bitrate::of(1_500_000),
            Some("livesource00000000000000000000"),
        )
        .await
        .ok()
        .expect("a source still offered negotiates");
    }
}
