//! What the playback chain puts on the wire, counted against the stub upstream:
//! how many requests each step issues, and which requests it issues none of.

use std::sync::Arc;

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::routing::get;
use jellium_protocol::profile::{DeviceProfile, MediaKind};
use jellium_protocol::{Bitrate, HostGrants, PlayRequest, Quality, Subtitles, report::Reporting};
use jellyfin_api::types::{
    BaseItemDto, MediaSourceInfo, MediaStream, MediaStreamType, MediaType, SubtitleDeliveryMethod,
};
use tower::ServiceExt as _;
use uuid::Uuid;

use super::negotiate::{self, Negotiated};
use super::pointed::Pointed;
use super::{Active, Playback, encodings, plan};
use crate::web::identity::Identity;
use crate::web::route;
use crate::web::upstream::{Answering, Upstream, answering, answering_with};
use crate::web::{AppState, foreign};

/// The media source the served source names itself by.
const MEDIA_SOURCE: &str = "d0000000000000000000000000000000";

/// The ceiling every test here negotiates under.
const CEILING: Bitrate = Bitrate::of(1_500_000);

fn device() -> Identity {
    Identity::of(jellium_protocol::Identity {
        device: "Firefox".to_owned(),
        device_id: Uuid::nil().to_string(),
    })
}

fn request() -> PlayRequest {
    PlayRequest {
        item: Uuid::nil(),
        media_source: None,
        audio_stream: None,
        subtitles: Subtitles::Default,
        start_ticks: 0,
        quality: Quality::Auto,
        profile: DeviceProfile::default(),
        always_burn_in_subtitle_when_transcoding: false,
        allow_direct_play: None,
        allow_direct_stream: None,
        allow_video_stream_copy: None,
        allow_audio_stream_copy: None,
        grants: HostGrants { remote_video: true },
        cinema_mode: true,
        fullscreen: true,
        start_index: None,
        reporting: Reporting {
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

fn item(media_type: MediaType) -> BaseItemDto {
    BaseItemDto {
        media_type: Some(media_type),
        ..BaseItemDto::default()
    }
}

fn playback_info(item: Uuid) -> String {
    format!("/Items/{item}/PlaybackInfo")
}

/// The item id the stub describes as audio, which the nil id cannot be, since
/// every other test here plays the nil id as video.
const AUDIO: Uuid = Uuid::from_u128(0xa0d10);

/// What every path the stub's own routes miss is answered with, which is the
/// `System/Endpoint` answer the ceiling reads and the memo holds; an out-of-
/// network link is the one the ladder measures rather than floors.
const ENDPOINT: &str = r#"{"IsInNetwork":false,"IsLocal":false}"#;

const ENDPOINT_HEADERS: &[(&str, &str)] = &[("content-type", "application/json")];

/// The same request, for `item`.
fn playing(item: Uuid) -> PlayRequest {
    PlayRequest { item, ..request() }
}

/// The path the entry door asks intros of.
fn intros(item: Uuid) -> String {
    format!("/Users/{}/Items/{item}/Intros", Uuid::nil())
}

/// A stub whose library describes the nil id as video and `AUDIO` as audio, and
/// the doors' own `Playback`, holding its measurement in a directory of the
/// test's own.
async fn driving(name: &str) -> (Answering, Upstream, Playback) {
    let server = answering_with(200, ENDPOINT_HEADERS, ENDPOINT).await;
    server.library.describes(Uuid::nil(), MediaKind::Video);
    server.library.describes(AUDIO, MediaKind::Audio);
    let upstream = Upstream::stub(&server.base);
    let playback = Playback::new(scratch(name));
    (server, upstream, playback)
}

#[tokio::test]
async fn a_video_negotiation_issues_exactly_one_request_to_playback_info() {
    let server = answering(204).await;
    let upstream = Upstream::stub(&server.base);

    negotiate::negotiate(
        &upstream,
        &request(),
        &item(MediaType::Video),
        &device(),
        CEILING,
        None,
    )
    .await
    .ok()
    .expect("a negotiation");

    assert_eq!(server.asked(&playback_info(Uuid::nil())), 1);
}

#[tokio::test]
async fn an_audio_negotiation_issues_no_request_to_playback_info() {
    let server = answering(204).await;
    let upstream = Upstream::stub(&server.base);

    negotiate::negotiate(
        &upstream,
        &request(),
        &item(MediaType::Audio),
        &device(),
        CEILING,
        None,
    )
    .await
    .ok()
    .expect("a negotiation");

    assert_eq!(server.asked(&playback_info(Uuid::nil())), 0);
    assert!(server.taken.credentialed().is_empty());
    assert!(server.taken.tokenless().is_empty());
}

/// One external subtitle stream, carrying the `DeliveryUrl` the Jellyfin server
/// answers for it.
fn subtitle_stream(index: i32) -> MediaStream {
    MediaStream {
        index: Some(index),
        type_: Some(MediaStreamType::Subtitle),
        codec: Some("subrip".to_string()),
        display_title: Some(format!("Subtitle {index}")),
        delivery_method: Some(SubtitleDeliveryMethod::External),
        delivery_url: Some(format!(
            "/Videos/{}/{MEDIA_SOURCE}/Subtitles/{index}/0/Stream.vtt",
            Uuid::nil()
        )),
        ..MediaStream::default()
    }
}

fn negotiated(source: MediaSourceInfo) -> Negotiated {
    Negotiated {
        play_session: "session".to_string(),
        source,
        stream_url: None,
        sources: Vec::new(),
        live_stream: None,
        enable_direct_play: false,
        max_bitrate: CEILING,
        live: false,
    }
}

#[tokio::test]
async fn a_plan_built_from_a_source_with_twenty_eight_subtitle_streams_fetches_none() {
    let server = answering(204).await;
    let upstream = Upstream::stub(&server.base);
    let mut streams = vec![MediaStream {
        index: Some(0),
        type_: Some(MediaStreamType::Video),
        ..MediaStream::default()
    }];
    streams.extend((1..=28).map(subtitle_stream));

    let plan = plan::build(
        &upstream,
        &negotiated(MediaSourceInfo {
            id: Some(MEDIA_SOURCE.to_string()),
            container: Some("mkv".to_string()),
            supports_direct_play: Some(true),
            media_streams: Some(streams),
            run_time_ticks: Some(100),
            ..MediaSourceInfo::default()
        }),
        &request(),
        &device(),
        plan::Described {
            chapters: &[],
            intros: Vec::new(),
        },
        &route::Seen::new(),
        &Pointed::new(),
    )
    .expect("a plan");

    assert_eq!(plan.subtitle_streams.len(), 28);
    assert!(server.taken.credentialed().is_empty());
    assert!(server.taken.tokenless().is_empty());
}

#[tokio::test]
async fn the_active_encodings_delete_names_device_id_then_play_session_id() {
    let server = answering(204).await;
    let upstream = Upstream::stub(&server.base);

    encodings::stop(&upstream, &device(), "session-1")
        .await
        .expect("the encode stops");

    assert_eq!(
        server.queries("/Videos/ActiveEncodings"),
        [format!("deviceId={}&PlaySessionId=session-1", Uuid::nil())]
    );
}

/// The session file every state here holds, in a directory of the test's own.
fn scratch(name: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir()
        .join("jellium-cli-requests-tests")
        .join(name);
    let _ = std::fs::remove_dir_all(&path);
    std::fs::create_dir_all(&path).expect("scratch directory");
    path.join("session.env")
}

/// The relay's one pointed route, driven the way the browser drives it.
fn pointing(state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            &format!("{}/{{handle}}", jellium_protocol::POINTED_PREFIX),
            get(foreign::pointed),
        )
        .with_state(state)
}

async fn answered(router: Router, handle: &str) -> StatusCode {
    router
        .oneshot(
            Request::builder()
                .uri(format!("{}/{handle}", jellium_protocol::POINTED_PREFIX))
                .body(Body::empty())
                .expect("a request"),
        )
        .await
        .expect("a response")
        .status()
}

/// A held session whose register holds whatever `minted` names.
fn active(session: &str, pointed: Arc<Pointed>) -> Active {
    Active {
        play_session: session.to_string(),
        item: Uuid::nil(),
        media_source: MEDIA_SOURCE.to_string(),
        live_stream: None,
        method: jellium_protocol::Method::DirectPlay,
        position_ticks: 0,
        touched: std::time::Instant::now(),
        paused_since: None,
        pointed,
        max_bitrate: CEILING,
        last_media_info: None,
        run_time_ticks: Some(100),
        reported: jellium_protocol::report::Playing {
            play_session: session.to_string(),
            volume_level: 100,
            muted: false,
            paused: false,
            repeat: jellium_protocol::Repeat::Off,
            shuffle: jellium_protocol::report::Shuffle::Sorted,
            position_ticks: 0,
            playback_start_time_ticks: 0,
            playback_rate: 1.0,
            subtitle_stream: None,
            secondary_subtitle_stream: None,
            audio_stream: None,
            buffered: Vec::new(),
            playlist_item_id: "playlistItem1".to_string(),
            queue: Vec::new(),
        },
    }
}

#[tokio::test]
async fn a_foreign_fetch_the_current_plan_does_not_carry_is_refused() {
    let state = Arc::new(AppState::stub(scratch("invented")));
    let pointed = Arc::new(Pointed::new());
    pointed.mint("https://elsewhere.test/a.vtt");
    state.playback.install(active("current", pointed)).await;

    assert_eq!(
        answered(pointing(Arc::clone(&state)), "p0000000000000009").await,
        StatusCode::FORBIDDEN
    );
}

#[tokio::test]
async fn a_subtitle_handle_the_replaced_plan_minted_resolves_to_nothing() {
    let state = Arc::new(AppState::stub(scratch("replaced")));
    let pointed = Arc::new(Pointed::new());
    let handle = pointed.mint("https://elsewhere.test/a.vtt");
    state.playback.install(active("first", pointed)).await;
    state
        .playback
        .install(active("second", Arc::new(Pointed::new())))
        .await;

    assert_eq!(
        answered(pointing(Arc::clone(&state)), &handle).await,
        StatusCode::FORBIDDEN
    );
}

#[tokio::test]
async fn a_stream_change_stops_the_encode_before_and_after_and_reports_no_stop() {
    let (server, upstream, playback) = driving("change").await;
    let seen = route::Seen::new();

    playback
        .start(&upstream, &device(), &request(), &seen)
        .await
        .ok()
        .expect("a start");
    assert_eq!(server.asked("/Videos/ActiveEncodings"), 0);

    playback
        .change(&upstream, &device(), &request(), &seen)
        .await
        .ok()
        .expect("a change");

    assert_eq!(server.asked("/Videos/ActiveEncodings"), 2);
    assert_eq!(server.asked("/Sessions/Playing/Stopped"), 0);
}

#[tokio::test]
async fn an_entered_play_requests_intros_once_and_a_queue_advance_requests_none() {
    let (server, upstream, playback) = driving("intros").await;
    let seen = route::Seen::new();

    playback
        .enter(&upstream, &device(), &request(), &seen)
        .await
        .ok()
        .expect("an entered play");
    assert_eq!(server.asked(&intros(Uuid::nil())), 1);

    playback
        .start(&upstream, &device(), &request(), &seen)
        .await
        .ok()
        .expect("a queue advance");
    assert_eq!(server.asked(&intros(Uuid::nil())), 1);
}

#[tokio::test]
async fn a_video_start_issues_one_playback_info_and_one_sessions_playing() {
    let (server, upstream, playback) = driving("video-start").await;

    playback
        .start(&upstream, &device(), &request(), &route::Seen::new())
        .await
        .ok()
        .expect("a video start");

    assert_eq!(server.asked(&playback_info(Uuid::nil())), 1);
    assert_eq!(server.asked("/Sessions/Playing"), 1);
}

#[tokio::test]
async fn an_audio_start_issues_no_playback_info() {
    let (server, upstream, playback) = driving("audio-start").await;

    playback
        .start(&upstream, &device(), &playing(AUDIO), &route::Seen::new())
        .await
        .ok()
        .expect("an audio start");

    assert_eq!(server.asked(&playback_info(AUDIO)), 0);
    assert_eq!(server.asked("/Sessions/Playing"), 1);
}

#[tokio::test]
async fn a_displaced_session_reports_stopped_once() {
    let (server, upstream, playback) = driving("displaced").await;
    let seen = route::Seen::new();

    playback
        .start(&upstream, &device(), &request(), &seen)
        .await
        .ok()
        .expect("the first start");
    playback
        .start(&upstream, &device(), &request(), &seen)
        .await
        .ok()
        .expect("the start that displaces it");

    assert_eq!(server.asked("/Sessions/Playing/Stopped"), 1);
}

#[tokio::test]
async fn a_negotiation_over_five_versions_issues_one_system_endpoint() {
    let (server, upstream, playback) = driving("versions").await;
    let seen = route::Seen::new();

    for _ in 0..5 {
        playback
            .start(&upstream, &device(), &request(), &seen)
            .await
            .ok()
            .expect("a version change");
    }

    assert_eq!(server.asked("/System/Endpoint"), 1);
}
