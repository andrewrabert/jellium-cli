use std::sync::Arc;
use std::time::{Duration, Instant};

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use jellium_protocol::{
    Bitrate, Failure, Method, Plan, PlayRequest, Progress, Repeat, Standing, Stopped,
    profile::MediaKind,
};
use jellyfin_api::types::{PlayMethod, RepeatMode};
use uuid::Uuid;

pub mod bandwidth;
mod derive;
mod encodings;
mod intros;
mod negotiate;
mod plan;
pub mod pointed;
mod reachable;
mod report;
#[cfg(test)]
mod requests;
mod subtitles;

use bandwidth::Bandwidth;
use negotiate::{Negotiated, Refused};
use pointed::Pointed;
use report::Throttle;

use super::AppState;
use super::holder::Holder;
use super::identity::Identity;
use super::upstream::{self, Upstream};
use super::wire::{self, Query};

/// The body `POST LiveStreams/MediaInfo` carries.
#[derive(serde::Serialize)]
#[serde(rename_all = "PascalCase")]
struct MediaInfo<'a> {
    live_stream_id: &'a str,
}

/// Whether the door a negotiation came through requests intros. The reference
/// reaches `getIntros` from its one `play()` entry point and from nowhere else,
/// so the entry door asks and no other door does.
#[derive(Clone, Copy)]
enum Introducing {
    Requested,
    Skipped,
}

/// What a start installed.
pub struct Started {
    pub plan: Plan,
    /// The playback session this start displaced.
    pub displaced: Option<String>,
}

/// The one playback session this local server permits, and the link
/// measurement that feeds it.
pub struct Playback {
    current: tokio::sync::RwLock<Option<Active>>,
    /// The sessions later starts displaced, oldest first, so a report naming
    /// one is told it was superseded rather than that it lapsed.
    displaced: tokio::sync::RwLock<std::collections::VecDeque<String>>,
    transition: tokio::sync::Mutex<()>,
    bandwidth: Bandwidth,
    /// The rate limit progress reports are held to before they go upstream.
    reports: Throttle,
}

/// How many displaced sessions are remembered.
const REMEMBERED: usize = 8;

/// The session in progress.
#[derive(Clone)]
pub struct Active {
    pub play_session: String,
    pub item: Uuid,
    pub media_source: String,
    pub live_stream: Option<String>,
    pub method: Method,
    pub position_ticks: i64,
    pub touched: Instant,
    /// When the browser last reported this session paused, and nothing while
    /// it plays.
    pub paused_since: Option<Instant>,
    /// The foreign origins this plan has been pointed at, which no later plan
    /// inherits.
    pub pointed: Arc<Pointed>,
    /// The ceiling that went to the Jellyfin server for this session.
    pub max_bitrate: Bitrate,
    /// When this session last re-read its live stream's media info, and nothing
    /// while it never has.
    pub last_media_info: Option<Instant>,
    /// The negotiated source's run time, which is what `CanSeek` is computed
    /// from.
    pub run_time_ticks: Option<i64>,
    /// The last state the browser reported, which starts as what the play
    /// request carried and is replaced by every progress report.
    pub reported: jellium_protocol::report::Playing,
}

impl Active {
    /// True while this session holds a live stream, which is what a tuner
    /// release is owed to.
    fn live(&self) -> bool {
        self.live_stream.is_some()
    }

    /// True when this live session has been paused past `Playback::PAUSED_LIVE`.
    fn overpaused(&self) -> bool {
        self.live()
            && self
                .paused_since
                .is_some_and(|since| since.elapsed() >= Playback::PAUSED_LIVE)
    }
}

fn played(method: Method) -> PlayMethod {
    match method {
        Method::DirectPlay => PlayMethod::DirectPlay,
        Method::DirectStream => PlayMethod::DirectStream,
        Method::Transcode { .. } => PlayMethod::Transcode,
    }
}

/// The media kind the bitrate ladder keys its measurement by, which is the
/// item's own `MediaType`; everything that is not audio is negotiated as video.
fn kind(item: &jellyfin_api::types::BaseItemDto) -> MediaKind {
    match item.media_type {
        Some(jellyfin_api::types::MediaType::Audio) => MediaKind::Audio,
        _ => MediaKind::Video,
    }
}

fn repeated(repeat: Repeat) -> RepeatMode {
    match repeat {
        Repeat::Off => RepeatMode::RepeatNone,
        Repeat::One => RepeatMode::RepeatOne,
        Repeat::All => RepeatMode::RepeatAll,
    }
}

/// The clock jellyfin-web stamps a playback start with, which is milliseconds
/// since the epoch in ten-thousandths.
fn started_at() -> i64 {
    let since = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    since.as_millis() as i64 * 10_000
}

impl Active {
    /// `last_media_info` is what the session this one continues had read, which
    /// a stream change carries across and a fresh start carries nothing of.
    fn of(
        plan: &Plan,
        negotiated: &Negotiated,
        request: &PlayRequest,
        ceiling: Bitrate,
        pointed: Arc<Pointed>,
        last_media_info: Option<Instant>,
    ) -> Active {
        let reporting = &request.reporting;
        Active {
            play_session: plan.play_session.clone(),
            item: plan.item,
            media_source: plan.media_source.clone(),
            live_stream: negotiated.live_stream.clone(),
            method: plan.playable.method,
            position_ticks: plan.start_ticks,
            touched: Instant::now(),
            paused_since: None,
            pointed,
            max_bitrate: ceiling,
            last_media_info,
            run_time_ticks: plan.run_time_ticks,
            reported: jellium_protocol::report::Playing {
                play_session: plan.play_session.clone(),
                volume_level: reporting.volume_level,
                muted: reporting.muted,
                paused: false,
                repeat: reporting.repeat,
                shuffle: reporting.shuffle,
                position_ticks: plan.start_ticks,
                playback_start_time_ticks: started_at(),
                playback_rate: reporting.playback_rate,
                subtitle_stream: plan.subtitle_stream,
                secondary_subtitle_stream: None,
                audio_stream: plan.audio_stream,
                buffered: Vec::new(),
                playlist_item_id: reporting.playlist_item_id.clone(),
                queue: reporting.queue.clone(),
            },
        }
    }
}

/// Stops the encode and closes the live stream `active` left behind, then
/// reports the stop. Every step is attempted even when an earlier one failed,
/// so nothing is left running because something else was already gone.
async fn end(upstream: &Upstream, identity: &Identity, reports: &Throttle, active: &Active) {
    reports.cancel().await;
    let stopped = report::stopped(upstream, &report::Body::of(active)).await;
    let encoding = encodings::stop(upstream, identity, &active.play_session).await;
    let live = match &active.live_stream {
        Some(live_stream) => {
            wire::poked(
                upstream,
                "LiveStreams/Close",
                &Query::new().set("liveStreamId", live_stream),
            )
            .await
        }
        None => Ok(()),
    };
    for outcome in [stopped, encoding, live] {
        if let Err(e) = outcome {
            eprintln!("jellium-cli web: ending playback: {e:?}");
        }
    }
}

impl Playback {
    /// A session whose progress reports have lapsed this long is ended; it
    /// clears the once-a-minute cadence a browser throttles a hidden, silent
    /// tab's timers to.
    pub const LAPSE: Duration = Duration::from_secs(180);

    /// How often lapsed sessions are looked for.
    pub const SWEEP: Duration = Duration::from_secs(5);

    /// A live session paused this long is stopped and its tuner released.
    pub const PAUSED_LIVE: Duration = Duration::from_secs(300);

    /// How long a live session's media info is good for before a progress
    /// report re-reads it.
    // reference: get-live-stream-media-info — playbackmanager.js:3687-3698
    pub const MEDIA_INFO: Duration = Duration::from_secs(600);

    /// `session` is the session file the measured bitrate is persisted into.
    pub fn new(session: std::path::PathBuf) -> Playback {
        Playback {
            current: tokio::sync::RwLock::new(None),
            displaced: tokio::sync::RwLock::new(std::collections::VecDeque::new()),
            transition: tokio::sync::Mutex::new(()),
            bandwidth: Bandwidth::new(session),
            reports: Throttle::new(),
        }
    }

    /// Installs `active` as the held session and records the session it
    /// displaces, both under the transition lock, so no report is answered
    /// between the two writes.
    /// A record naming `active` itself is dropped, and so is the oldest record
    /// past `REMEMBERED`.
    /// Returns the displaced session, which the caller ends.
    async fn install(&self, active: Active) -> Option<Active> {
        let installed = active.play_session.clone();
        let _transition = self.transition.lock().await;
        let displaced = self.current.write().await.replace(active);
        if let Some(displaced) = displaced.as_ref() {
            displaced.pointed.clear();
        }
        let mut records = self.displaced.write().await;
        records.retain(|session| *session != installed);
        if let Some(displaced) = displaced.as_ref() {
            records.push_back(displaced.play_session.clone());
        }
        while records.len() > REMEMBERED {
            records.pop_front();
        }
        displaced
    }

    /// The register the held plan carries, and an empty one while no plan
    /// holds, which is what a relayed body is rewritten against.
    pub async fn pointed(&self) -> Arc<Pointed> {
        match self.current.read().await.as_ref() {
            Some(active) => Arc::clone(&active.pointed),
            None => Arc::new(Pointed::new()),
        }
    }

    /// The media source a held live session is playing this item from, which
    /// is what a re-negotiation resumes rather than tunes afresh.
    async fn resuming(&self, item: Uuid) -> Option<String> {
        let current = self.current.read().await;
        let held = current.as_ref()?;
        (held.live() && held.item == item).then(|| held.media_source.clone())
    }

    /// The door a user-initiated play comes through, which is the one entry
    /// point the reference asks `getIntros` from.
    pub async fn enter(
        &self,
        upstream: &Upstream,
        identity: &Identity,
        request: &PlayRequest,
        seen: &crate::web::route::Seen,
    ) -> Result<Started, Refused> {
        self.begun(upstream, identity, request, seen, Introducing::Requested)
            .await
    }

    /// The door a queue advance, an ended item and a version change come
    /// through, none of which the reference asks `getIntros` from.
    pub async fn start(
        &self,
        upstream: &Upstream,
        identity: &Identity,
        request: &PlayRequest,
        seen: &crate::web::route::Seen,
    ) -> Result<Started, Refused> {
        self.begun(upstream, identity, request, seen, Introducing::Skipped)
            .await
    }

    /// Negotiates the request, ends the session it displaces — stopping that
    /// encode and closing that live stream — installs the new one, and reports
    /// playback start to the Jellyfin server.
    /// A request naming the item a held live session is playing re-negotiates
    /// as a resume, so a live stream the Jellyfin server no longer holds reads
    /// as `TunerGone` rather than as a fresh tune.
    async fn begun(
        &self,
        upstream: &Upstream,
        identity: &Identity,
        request: &PlayRequest,
        seen: &crate::web::route::Seen,
        introducing: Introducing,
    ) -> Result<Started, Refused> {
        let (plan, active) = self
            .negotiated(upstream, identity, request, seen, None, introducing)
            .await?;
        let opening = report::Body::of(&active);

        let mut lost = None;
        if let Some(displaced) = self.install(active).await {
            lost = Some(displaced.play_session.clone());
            end(upstream, identity, &self.reports, &displaced).await;
        }

        self.reports.cancel().await;
        report::started(upstream, &opening)
            .await
            .map_err(Refused::Upstream)?;

        Ok(Started {
            plan,
            displaced: lost,
        })
    }

    /// Swaps the source under the session already playing: the encodes the held
    /// session left running are stopped before the swap and again after it, and
    /// no stop is reported, so the Jellyfin server sees one session throughout.
    /// A change asked for while nothing is held swaps in a session of its own
    /// and stops no encode, which is what the reference does with no
    /// `streamInfo` to change from.
    /// A first stop that fails answers `Unchanged`: the reference swaps the
    /// source inside that stop's success handler, so the stream that was
    /// playing goes on playing and the held plan goes on standing.
    // reference: change-stream-to-url — playbackmanager.js:1766-1782
    pub async fn change(
        &self,
        upstream: &Upstream,
        identity: &Identity,
        request: &PlayRequest,
        seen: &crate::web::route::Seen,
    ) -> Result<jellium_protocol::Planned, Refused> {
        let (stopping, last_media_info) = self
            .current
            .read()
            .await
            .as_ref()
            .map(|held| (held.play_session.clone(), held.last_media_info))
            .unzip();
        let last_media_info = last_media_info.flatten();

        let (plan, active) = self
            .negotiated(
                upstream,
                identity,
                request,
                seen,
                last_media_info,
                Introducing::Skipped,
            )
            .await?;

        if let Some(play_session) = stopping.as_deref()
            && let Err(e) = encodings::stop(upstream, identity, play_session).await
        {
            eprintln!("jellium-cli web: stopping the encode before the change: {e:?}");
            return Ok(jellium_protocol::Planned::Unchanged);
        }

        // the session this change swaps out is displaced, not ended: nothing
        // reports it stopped and nothing closes its live stream
        self.install(active).await;

        if let Some(play_session) = stopping.as_deref()
            && let Err(e) = encodings::stop(upstream, identity, play_session).await
        {
            eprintln!("jellium-cli web: stopping the changed encode: {e:?}");
        }

        Ok(jellium_protocol::Planned::Started(Box::new(plan)))
    }

    /// The plan `request` settles on and the session that would hold it,
    /// carrying `last_media_info` from the session this one continues.
    async fn negotiated(
        &self,
        upstream: &Upstream,
        identity: &Identity,
        request: &PlayRequest,
        seen: &crate::web::route::Seen,
        last_media_info: Option<Instant>,
        introducing: Introducing,
    ) -> Result<(Plan, Active), Refused> {
        let resuming = self.resuming(request.item).await;
        let item = upstream
            .control()
            .get_item(&request.item, Some(&upstream.user_id()))
            .await
            .map_err(|e| Refused::Upstream(upstream.failed(e)))?;
        let ceiling = self
            .bandwidth
            .ceiling(upstream, request.quality, kind(&item))
            .await;
        let negotiated = negotiate::negotiate(
            upstream,
            request,
            &item,
            identity,
            ceiling,
            resuming.as_deref(),
        )
        .await?;

        let chapters = item.chapters.clone().unwrap_or_default();
        let introduced = match introducing {
            Introducing::Requested => {
                intros::intros(
                    upstream,
                    &item,
                    request.start_ticks,
                    request.start_index,
                    request.fullscreen,
                    request.cinema_mode,
                )
                .await
            }
            Introducing::Skipped => Vec::new(),
        };

        let pointed = Arc::new(Pointed::new());
        let plan = plan::build(
            upstream,
            &negotiated,
            request,
            identity,
            plan::Described {
                chapters: &chapters,
                intros: introduced,
            },
            seen,
            &pointed,
        )
        .map_err(Refused::Playback)?;

        let active = Active::of(
            &plan,
            &negotiated,
            request,
            ceiling,
            pointed,
            last_media_info,
        );
        Ok((plan, active))
    }

    /// The playback session held now, which is the tab a control command goes
    /// to.
    pub async fn held_session(&self) -> Option<String> {
        self.current
            .read()
            .await
            .as_ref()
            .map(|held| held.play_session.clone())
    }

    /// Records the position, the paused state and the deadline, then reports
    /// progress to the Jellyfin server.
    /// A report naming a session a later start displaced answers `Superseded`;
    /// a report naming any other session that is not held answers `Lapsed`.
    /// Neither reaches the Jellyfin server.
    /// A live session paused for `PAUSED_LIVE` is ended, its live stream
    /// closed, and answered `Standing::Released`.
    pub async fn progress(
        &self,
        upstream: &Upstream,
        identity: &Identity,
        progress: &Progress,
    ) -> Result<Standing, Failure> {
        let held = {
            let mut current = self.current.write().await;
            match current.as_mut() {
                Some(active) if active.play_session == progress.playing.play_session => {
                    active.position_ticks = progress.playing.position_ticks;
                    active.touched = Instant::now();
                    active.paused_since = match (progress.playing.paused, active.paused_since) {
                        (true, Some(since)) => Some(since),
                        (true, None) => Some(Instant::now()),
                        (false, _) => None,
                    };
                    active.reported = progress.playing.clone();
                    active.clone()
                }
                _ => {
                    let records = self.displaced.read().await;
                    let superseded = records
                        .iter()
                        .any(|session| *session == progress.playing.play_session);
                    return Ok(if superseded {
                        Standing::Superseded
                    } else {
                        Standing::Lapsed
                    });
                }
            }
        };

        if held.overpaused() {
            let _transition = self.transition.lock().await;
            let released = {
                let mut current = self.current.write().await;
                match current.as_ref() {
                    Some(active) if active.play_session == held.play_session => current.take(),
                    _ => None,
                }
            };
            if let Some(released) = released {
                end(upstream, identity, &self.reports, &released).await;
            }
            return Ok(Standing::Released);
        }

        let mut body = report::Body::of(&held);
        body.event_name = Some(progress.event);
        self.reports.progress(upstream, body).await?;
        self.media_info(upstream, &held).await;

        Ok(Standing::Current)
    }

    /// Re-reads the live stream `held` is playing when the last read is older
    /// than `MEDIA_INFO`, which is what every progress update does; a failure
    /// is swallowed, the way the reference swallows one.
    // reference: get-live-stream-media-info — playbackmanager.js:3687-3698
    async fn media_info(&self, upstream: &Upstream, held: &Active) {
        let Some(live_stream) = held.live_stream.as_deref() else {
            return;
        };
        if held
            .last_media_info
            .is_some_and(|read| read.elapsed() < Self::MEDIA_INFO)
        {
            return;
        }
        {
            let mut current = self.current.write().await;
            match current.as_mut() {
                Some(active) if active.play_session == held.play_session => {
                    active.last_media_info = Some(Instant::now());
                }
                _ => return,
            }
        }
        let read: Result<jellyfin_api::types::MediaSourceInfo, Failure> = wire::posted(
            upstream,
            "LiveStreams/MediaInfo",
            &Query::new(),
            &MediaInfo {
                live_stream_id: live_stream,
            },
        )
        .await;
        if let Err(e) = read {
            eprintln!("jellium-cli web: re-reading the live stream's media info: {e:?}");
        }
    }

    /// Reports the stop to the Jellyfin server, stops the encode and closes
    /// the live stream, then clears the held session.
    /// A stop naming a session that is no longer held changes nothing.
    pub async fn stopped(
        &self,
        upstream: &Upstream,
        identity: &Identity,
        stopped: &Stopped,
    ) -> Result<(), Failure> {
        let _transition = self.transition.lock().await;
        let held = {
            let mut current = self.current.write().await;
            match current.as_ref() {
                Some(active) if active.play_session == stopped.playing.play_session => {
                    let mut active = current.take().expect("the session just matched");
                    active.position_ticks = stopped.playing.position_ticks;
                    active.reported = stopped.playing.clone();
                    active
                }
                _ => return Ok(()),
            }
        };
        end(upstream, identity, &self.reports, &held).await;
        Ok(())
    }

    /// Ends the held session when its last progress report is older than
    /// `LAPSE`, and a held live session paused longer than `PAUSED_LIVE`,
    /// closing its live stream either way.
    pub async fn sweep(&self, holder: &Holder, identity: &Identity) {
        let _transition = self.transition.lock().await;
        let lapsed = {
            let mut current = self.current.write().await;
            match current.as_ref() {
                Some(active) if active.touched.elapsed() >= Self::LAPSE || active.overpaused() => {
                    current.take()
                }
                _ => None,
            }
        };
        if let Some(lapsed) = lapsed
            && let Some(upstream) = holder.signed().await
        {
            end(&upstream, identity, &self.reports, &lapsed).await;
        }
    }

    /// Ends the held session at its last reported position, which is how the
    /// local server leaves no encode running behind it.
    pub async fn shutdown(&self, holder: &Holder, identity: &Identity) {
        let _transition = self.transition.lock().await;
        let held = self.current.write().await.take();
        if let Some(held) = held
            && let Some(upstream) = holder.signed().await
        {
            end(&upstream, identity, &self.reports, &held).await;
        }
    }
}

/// The signed upstream and the announced identity, and nothing while either is
/// missing.
async fn ready(state: &AppState) -> Option<(Arc<Upstream>, Arc<Identity>)> {
    Some((state.session.signed().await?, state.identity.held().await?))
}

fn no_session() -> Response {
    (
        StatusCode::CONFLICT,
        Json(jellium_protocol::Refusal::NoSession),
    )
        .into_response()
}

/// The door a user-initiated play comes through, which is the one that asks
/// the Jellyfin server for this item's intros.
pub async fn enter(state: State<Arc<AppState>>, request: Json<PlayRequest>) -> Response {
    let Some((upstream, identity)) = ready(&state).await else {
        return no_session();
    };
    let started = state
        .playback
        .enter(&upstream, &identity, &request, &state.seen)
        .await;
    installed(&state, &upstream, started).await
}

/// The door a queue advance, an ended item and a version change come through,
/// which asks for no intros.
pub async fn start(state: State<Arc<AppState>>, request: Json<PlayRequest>) -> Response {
    let Some((upstream, identity)) = ready(&state).await else {
        return no_session();
    };
    let started = state
        .playback
        .start(&upstream, &identity, &request, &state.seen)
        .await;
    installed(&state, &upstream, started).await
}

/// 409 with the refusal, or the upstream status for a transport failure.
/// A displaced session is told over the event socket before the plan is
/// answered.
async fn installed(
    state: &AppState,
    upstream: &Arc<Upstream>,
    started: Result<Started, Refused>,
) -> Response {
    match started {
        Ok(started) => {
            if let Some(displaced) = started.displaced.as_deref() {
                state.live.displaced(displaced).await;
            }
            (
                StatusCode::OK,
                Json(jellium_protocol::Planned::Started(Box::new(started.plan))),
            )
                .into_response()
        }
        Err(Refused::Playback(refused)) => (StatusCode::CONFLICT, Json(refused)).into_response(),
        Err(Refused::Upstream(failure)) => {
            if failure == Failure::TokenRejected {
                state.session.reject(upstream).await;
            }
            (upstream::status_for(&failure), Json(failure)).into_response()
        }
    }
}

/// The plan the swapped-in stream plays under; a refusal and a transport
/// failure answer the way a start's do.
pub async fn change(state: State<Arc<AppState>>, request: Json<PlayRequest>) -> Response {
    let Some(upstream) = state.session.signed().await else {
        return (
            StatusCode::CONFLICT,
            Json(jellium_protocol::Refusal::NoSession),
        )
            .into_response();
    };
    let Some(identity) = state.identity.held().await else {
        return (
            StatusCode::CONFLICT,
            Json(jellium_protocol::Refusal::NoSession),
        )
            .into_response();
    };
    match state
        .playback
        .change(&upstream, &identity, &request, &state.seen)
        .await
    {
        Ok(planned) => (StatusCode::OK, Json(planned)).into_response(),
        Err(Refused::Playback(refused)) => (StatusCode::CONFLICT, Json(refused)).into_response(),
        Err(Refused::Upstream(failure)) => {
            if failure == Failure::TokenRejected {
                state.session.reject(&upstream).await;
            }
            (upstream::status_for(&failure), Json(failure)).into_response()
        }
    }
}

pub async fn progress(state: State<Arc<AppState>>, progress: Json<Progress>) -> Response {
    let Some(upstream) = state.session.signed().await else {
        return (
            StatusCode::CONFLICT,
            Json(jellium_protocol::Refusal::NoSession),
        )
            .into_response();
    };
    let Some(identity) = state.identity.held().await else {
        return (
            StatusCode::CONFLICT,
            Json(jellium_protocol::Refusal::NoSession),
        )
            .into_response();
    };
    match state
        .playback
        .progress(&upstream, &identity, &progress)
        .await
    {
        Ok(standing) => (StatusCode::OK, Json(standing)).into_response(),
        Err(failure) => {
            if failure == Failure::TokenRejected {
                state.session.reject(&upstream).await;
            }
            (upstream::status_for(&failure), Json(failure)).into_response()
        }
    }
}

pub async fn stopped(state: State<Arc<AppState>>, stopped: Json<Stopped>) -> Response {
    let Some(upstream) = state.session.signed().await else {
        return (
            StatusCode::CONFLICT,
            Json(jellium_protocol::Refusal::NoSession),
        )
            .into_response();
    };
    let Some(identity) = state.identity.held().await else {
        return (
            StatusCode::CONFLICT,
            Json(jellium_protocol::Refusal::NoSession),
        )
            .into_response();
    };
    match state.playback.stopped(&upstream, &identity, &stopped).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(failure) => {
            if failure == Failure::TokenRejected {
                state.session.reject(&upstream).await;
            }
            (upstream::status_for(&failure), Json(failure)).into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// What a browser reports about `session`, five ticks in.
    fn playing(session: &str) -> jellium_protocol::report::Playing {
        jellium_protocol::report::Playing {
            play_session: session.to_string(),
            volume_level: 100,
            muted: false,
            paused: false,
            repeat: Repeat::Off,
            shuffle: jellium_protocol::report::Shuffle::Sorted,
            position_ticks: 5,
            playback_start_time_ticks: 0,
            playback_rate: 1.0,
            subtitle_stream: None,
            secondary_subtitle_stream: None,
            audio_stream: None,
            buffered: Vec::new(),
            playlist_item_id: "playlistItem1".to_string(),
            queue: Vec::new(),
        }
    }

    fn stopped_report(session: &str) -> Stopped {
        Stopped {
            playing: playing(session),
        }
    }

    fn progress_report(session: &str) -> Progress {
        Progress {
            playing: playing(session),
            event: jellium_protocol::report::Reported::TimeUpdate,
        }
    }

    fn active(session: &str) -> Active {
        Active {
            play_session: session.to_string(),
            item: Uuid::nil(),
            media_source: "source".to_string(),
            live_stream: None,
            method: Method::DirectPlay,
            position_ticks: 0,
            touched: Instant::now(),
            paused_since: None,
            pointed: Arc::new(Pointed::new()),
            max_bitrate: Bitrate::of(1_500_000),
            last_media_info: None,
            run_time_ticks: Some(1_000),
            reported: playing(session),
        }
    }

    /// A holder whose session file lives in a directory that goes away with the
    /// returned guard, so no test writes the developer's own session file.
    fn on_a_temporary_session() -> (tempfile::TempDir, Playback) {
        let directory = tempfile::tempdir().expect("the test's session directory is creatable");
        let session = directory.path().join("session.json");
        (directory, Playback::new(session))
    }

    #[tokio::test]
    async fn a_report_from_a_displaced_session_is_superseded_and_reaches_no_server() {
        let server = upstream::answering(204).await;
        let upstream = Upstream::stub(&server.base);
        let device = Identity::of(jellium_protocol::Identity {
            device: "Firefox".to_owned(),
            device_id: Uuid::nil().to_string(),
        });
        let (_session, playback) = on_a_temporary_session();
        *playback.current.write().await = Some(active("current"));
        playback
            .displaced
            .write()
            .await
            .push_back("displaced".to_string());

        assert_eq!(
            playback
                .progress(&upstream, &device, &progress_report("displaced"))
                .await
                .expect("a standing"),
            Standing::Superseded
        );
        assert_eq!(server.requests.load(std::sync::atomic::Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn a_session_displaced_before_the_last_one_is_still_superseded() {
        let server = upstream::answering(204).await;
        let upstream = Upstream::stub(&server.base);
        let device = Identity::of(jellium_protocol::Identity {
            device: "Firefox".to_owned(),
            device_id: Uuid::nil().to_string(),
        });
        let (_session, playback) = on_a_temporary_session();
        for session in ["first", "second", "third"] {
            playback.install(active(session)).await;
        }

        assert_eq!(
            playback
                .progress(&upstream, &device, &progress_report("first"))
                .await
                .expect("a standing"),
            Standing::Superseded
        );
        assert_eq!(
            playback
                .progress(&upstream, &device, &progress_report("second"))
                .await
                .expect("a standing"),
            Standing::Superseded
        );
        assert_eq!(server.requests.load(std::sync::atomic::Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn two_installs_at_once_supersede_every_session_they_displace() {
        let server = upstream::answering(204).await;
        let upstream = Upstream::stub(&server.base);
        let device = Identity::of(jellium_protocol::Identity {
            device: "Firefox".to_owned(),
            device_id: Uuid::nil().to_string(),
        });
        let (_session, playback) = on_a_temporary_session();
        let playback = Arc::new(playback);
        playback.install(active("first")).await;

        let one = {
            let playback = Arc::clone(&playback);
            tokio::spawn(async move { playback.install(active("second")).await })
        };
        let two = {
            let playback = Arc::clone(&playback);
            tokio::spawn(async move { playback.install(active("third")).await })
        };
        one.await.expect("an install");
        two.await.expect("an install");

        let held = playback
            .current
            .read()
            .await
            .as_ref()
            .expect("a held session")
            .play_session
            .clone();
        for session in ["first", "second", "third"] {
            if session == held {
                continue;
            }
            assert_eq!(
                playback
                    .progress(&upstream, &device, &progress_report(session))
                    .await
                    .expect("a standing"),
                Standing::Superseded
            );
        }
        assert_eq!(server.requests.load(std::sync::atomic::Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn a_report_from_a_session_that_lapsed_is_lapsed_and_reaches_no_server() {
        let server = upstream::answering(204).await;
        let upstream = Upstream::stub(&server.base);
        let device = Identity::of(jellium_protocol::Identity {
            device: "Firefox".to_owned(),
            device_id: Uuid::nil().to_string(),
        });
        let (_session, playback) = on_a_temporary_session();

        assert_eq!(
            playback
                .progress(&upstream, &device, &progress_report("lapsed"))
                .await
                .expect("a standing"),
            Standing::Lapsed
        );
        assert_eq!(server.requests.load(std::sync::atomic::Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn a_stop_from_a_displaced_session_changes_nothing() {
        let server = upstream::answering(204).await;
        let upstream = Upstream::stub(&server.base);
        let device = Identity::of(jellium_protocol::Identity {
            device: "Firefox".to_owned(),
            device_id: Uuid::nil().to_string(),
        });
        let (_session, playback) = on_a_temporary_session();
        *playback.current.write().await = Some(active("current"));

        playback
            .stopped(&upstream, &device, &stopped_report("displaced"))
            .await
            .expect("a stop");
        assert_eq!(server.requests.load(std::sync::atomic::Ordering::SeqCst), 0);
        assert!(playback.current.read().await.is_some());
    }

    #[tokio::test]
    async fn a_stop_reports_it_stops_the_encode_and_clears_the_session() {
        let server = upstream::answering(204).await;
        let upstream = Upstream::stub(&server.base);
        let device = Identity::of(jellium_protocol::Identity {
            device: "Firefox".to_owned(),
            device_id: Uuid::nil().to_string(),
        });
        let (_session, playback) = on_a_temporary_session();
        *playback.current.write().await = Some(active("current"));

        playback
            .stopped(&upstream, &device, &stopped_report("current"))
            .await
            .expect("a stop");
        assert_eq!(server.asked("/Sessions/Playing/Stopped"), 1);
        assert_eq!(server.asked("/Videos/ActiveEncodings"), 1);
        assert!(playback.current.read().await.is_none());
    }

    #[tokio::test]
    async fn a_progress_report_refreshes_the_deadline_and_reaches_the_server() {
        let server = upstream::answering(204).await;
        let upstream = Upstream::stub(&server.base);
        let device = Identity::of(jellium_protocol::Identity {
            device: "Firefox".to_owned(),
            device_id: Uuid::nil().to_string(),
        });
        let (_session, playback) = on_a_temporary_session();
        let mut stale = active("current");
        stale.touched = Instant::now() - Playback::LAPSE;
        *playback.current.write().await = Some(stale);

        assert_eq!(
            playback
                .progress(&upstream, &device, &progress_report("current"))
                .await
                .expect("a standing"),
            Standing::Current
        );
        assert_eq!(server.asked("/Sessions/Playing/Progress"), 1);
        let held = playback.current.read().await;
        let held = held.as_ref().expect("the held session");
        assert_eq!(held.position_ticks, 5);
        assert!(held.touched.elapsed() < Playback::LAPSE);
    }

    /// A held session carrying a live stream, paused `paused` ago.
    fn live_active(session: &str, paused: Option<Duration>) -> Active {
        Active {
            live_stream: Some("live-stream-1".to_string()),
            paused_since: paused.map(|ago| Instant::now() - ago),
            ..active(session)
        }
    }

    #[tokio::test]
    async fn a_live_session_paused_past_the_limit_is_released_and_its_stream_closed() {
        let server = upstream::answering(204).await;
        let upstream = Upstream::stub(&server.base);
        let device = Identity::of(jellium_protocol::Identity {
            device: "Firefox".to_owned(),
            device_id: Uuid::nil().to_string(),
        });
        let (_session, playback) = on_a_temporary_session();
        *playback.current.write().await = Some(live_active("current", Some(Playback::PAUSED_LIVE)));

        let mut report = progress_report("current");
        report.playing.paused = true;
        assert_eq!(
            playback
                .progress(&upstream, &device, &report)
                .await
                .expect("a standing"),
            Standing::Released
        );
        assert_eq!(server.asked("/LiveStreams/Close"), 1);
        assert_eq!(server.asked("/Sessions/Playing/Progress"), 0);
        assert!(playback.current.read().await.is_none());
    }

    #[tokio::test]
    async fn a_live_session_paused_inside_the_limit_keeps_its_stream() {
        let server = upstream::answering(204).await;
        let upstream = Upstream::stub(&server.base);
        let device = Identity::of(jellium_protocol::Identity {
            device: "Firefox".to_owned(),
            device_id: Uuid::nil().to_string(),
        });
        let (_session, playback) = on_a_temporary_session();
        *playback.current.write().await =
            Some(live_active("current", Some(Duration::from_secs(1))));

        let mut report = progress_report("current");
        report.playing.paused = true;
        assert_eq!(
            playback
                .progress(&upstream, &device, &report)
                .await
                .expect("a standing"),
            Standing::Current
        );
        assert_eq!(server.asked("/LiveStreams/Close"), 0);
        assert_eq!(server.asked("/Sessions/Playing/Progress"), 1);
        assert!(playback.current.read().await.is_some());
    }

    #[tokio::test]
    async fn a_stop_closes_the_live_stream_before_the_sweeper_could() {
        let server = upstream::answering(204).await;
        let upstream = Upstream::stub(&server.base);
        let device = Identity::of(jellium_protocol::Identity {
            device: "Firefox".to_owned(),
            device_id: Uuid::nil().to_string(),
        });
        let (_session, playback) = on_a_temporary_session();
        *playback.current.write().await = Some(live_active("current", None));

        playback
            .stopped(&upstream, &device, &stopped_report("current"))
            .await
            .expect("a stop");
        assert_eq!(server.asked("/LiveStreams/Close"), 1);
        assert!(playback.current.read().await.is_none());
    }

    #[tokio::test]
    async fn a_shutdown_closes_the_live_stream_it_finds_open() {
        let server = upstream::answering(204).await;
        let device = Identity::of(jellium_protocol::Identity {
            device: "Firefox".to_owned(),
            device_id: Uuid::nil().to_string(),
        });
        let holder = Holder::new(
            std::env::temp_dir()
                .join("jellium-cli-playback-tests")
                .join("shutdown-live.env"),
        );
        holder.install(Upstream::stub(&server.base)).await;
        let (_session, playback) = on_a_temporary_session();
        *playback.current.write().await = Some(live_active("current", None));

        playback.shutdown(&holder, &device).await;
        assert_eq!(server.asked("/LiveStreams/Close"), 1);
        assert!(playback.current.read().await.is_none());
    }

    #[tokio::test]
    async fn a_displaced_live_session_has_its_stream_closed() {
        let server = upstream::answering(204).await;
        let upstream = Upstream::stub(&server.base);
        let device = Identity::of(jellium_protocol::Identity {
            device: "Firefox".to_owned(),
            device_id: Uuid::nil().to_string(),
        });
        let (_session, playback) = on_a_temporary_session();

        let displaced = playback
            .install(live_active("first", None))
            .await
            .is_none()
            .then(|| playback.install(active("second")))
            .expect("the first install displaced nothing")
            .await
            .expect("the second install displaced the first");
        end(&upstream, &device, &playback.reports, &displaced).await;

        assert_eq!(displaced.play_session, "first");
        assert_eq!(server.asked("/LiveStreams/Close"), 1);
    }

    #[tokio::test]
    async fn a_sweep_releases_a_live_session_paused_past_the_limit() {
        let server = upstream::answering(204).await;
        let device = Identity::of(jellium_protocol::Identity {
            device: "Firefox".to_owned(),
            device_id: Uuid::nil().to_string(),
        });
        let holder = Holder::new(
            std::env::temp_dir()
                .join("jellium-cli-playback-tests")
                .join("sweep-live.env"),
        );
        holder.install(Upstream::stub(&server.base)).await;
        let (_session, playback) = on_a_temporary_session();
        *playback.current.write().await = Some(live_active("current", Some(Playback::PAUSED_LIVE)));

        playback.sweep(&holder, &device).await;
        assert_eq!(server.asked("/LiveStreams/Close"), 1);
        assert!(playback.current.read().await.is_none());
    }

    #[tokio::test]
    async fn a_shutdown_stops_the_encode_it_finds_running() {
        let server = upstream::answering(204).await;
        let device = Identity::of(jellium_protocol::Identity {
            device: "Firefox".to_owned(),
            device_id: Uuid::nil().to_string(),
        });
        let holder = Holder::new(
            std::env::temp_dir()
                .join("jellium-cli-playback-tests")
                .join("shutdown.env"),
        );
        holder.install(Upstream::stub(&server.base)).await;
        let (_session, playback) = on_a_temporary_session();
        *playback.current.write().await = Some(active("current"));

        playback.shutdown(&holder, &device).await;
        assert_eq!(server.asked("/Videos/ActiveEncodings"), 1);
        assert!(playback.current.read().await.is_none());
    }
}
