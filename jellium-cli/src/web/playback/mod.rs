use std::sync::Arc;
use std::time::{Duration, Instant};

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use jellium_protocol::{
    Failure, Method, Plan, PlayRequest, Progress, Quality, Repeat, Standing, Stopped,
};
use jellyfin_api::types::{
    PlayMethod, PlaybackProgressInfo, PlaybackStartInfo, PlaybackStopInfo, RepeatMode,
};
use uuid::Uuid;

mod bandwidth;
mod negotiate;
mod plan;
mod profile;

use bandwidth::Bandwidth;
use negotiate::{Negotiated, Refused};

use super::AppState;
use super::holder::Holder;
use super::identity::Device;
use super::upstream::{self, Upstream};

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

fn repeated(repeat: Repeat) -> RepeatMode {
    match repeat {
        Repeat::Off => RepeatMode::RepeatNone,
        Repeat::One => RepeatMode::RepeatOne,
        Repeat::All => RepeatMode::RepeatAll,
    }
}

impl Active {
    fn of(plan: &Plan, negotiated: &Negotiated) -> Active {
        Active {
            play_session: plan.play_session.clone(),
            item: plan.item,
            media_source: plan.media_source.clone(),
            live_stream: negotiated.live_stream.clone(),
            method: plan.method,
            position_ticks: plan.start_ticks,
            touched: Instant::now(),
            paused_since: None,
        }
    }
}

/// Stops the encode and closes the live stream `active` left behind, then
/// reports the stop. Every step is attempted even when an earlier one failed,
/// so nothing is left running because something else was already gone.
async fn end(upstream: &Upstream, device: &Device, active: &Active) {
    let control = upstream.control();
    let stopped = control
        .report_playback_stopped(&PlaybackStopInfo {
            item_id: Some(active.item),
            media_source_id: Some(active.media_source.clone()),
            play_session_id: Some(active.play_session.clone()),
            live_stream_id: active.live_stream.clone(),
            position_ticks: Some(active.position_ticks),
            ..PlaybackStopInfo::default()
        })
        .await;
    let encoding = control
        .stop_encoding_process(&device.id().to_string(), &active.play_session)
        .await;
    let live = match &active.live_stream {
        Some(live_stream) => control.close_live_stream(live_stream).await,
        None => Ok(()),
    };
    for outcome in [stopped, encoding, live] {
        if let Err(e) = outcome {
            eprintln!("jellium-cli web: ending playback: {e}");
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

    pub fn new() -> Playback {
        Playback {
            current: tokio::sync::RwLock::new(None),
            displaced: tokio::sync::RwLock::new(std::collections::VecDeque::new()),
            transition: tokio::sync::Mutex::new(()),
            bandwidth: Bandwidth::new(),
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

    async fn ceiling(&self, upstream: &Upstream, quality: Quality) -> Option<i32> {
        match quality {
            Quality::Auto => self.bandwidth.ceiling(upstream).await,
            Quality::Limit { bits_per_second } => Some(bits_per_second),
        }
    }

    /// The media source a held live session is playing this item from, which
    /// is what a re-negotiation resumes rather than tunes afresh.
    async fn resuming(&self, item: Uuid) -> Option<String> {
        let current = self.current.read().await;
        let held = current.as_ref()?;
        (held.live() && held.item == item).then(|| held.media_source.clone())
    }

    /// Negotiates the request, ends the session it displaces — stopping that
    /// encode and closing that live stream — installs the new one, and reports
    /// playback start to the Jellyfin server.
    /// A request naming the item a held live session is playing re-negotiates
    /// as a resume, so a live stream the Jellyfin server no longer holds reads
    /// as `TunerGone` rather than as a fresh tune.
    pub async fn start(
        &self,
        upstream: &Upstream,
        device: &Device,
        request: &PlayRequest,
        seen: &crate::web::route::Seen,
    ) -> Result<Started, Refused> {
        let ceiling = self.ceiling(upstream, request.quality).await;
        let resuming = self.resuming(request.item).await;
        let negotiated =
            negotiate::negotiate(upstream, request, ceiling, resuming.as_deref()).await?;

        let chapters = upstream
            .control()
            .get_item(&request.item, Some(&upstream.user_id()))
            .await
            .ok()
            .and_then(|item| item.chapters)
            .unwrap_or_default();

        let plan = plan::build(
            &negotiated,
            request,
            &chapters,
            upstream.link().base(),
            seen,
        )
        .map_err(Refused::Playback)?;

        let mut lost = None;
        if let Some(displaced) = self.install(Active::of(&plan, &negotiated)).await {
            lost = Some(displaced.play_session.clone());
            end(upstream, device, &displaced).await;
        }

        if let Err(e) = upstream
            .control()
            .report_playback_start(&PlaybackStartInfo {
                item_id: Some(plan.item),
                media_source_id: Some(plan.media_source.clone()),
                play_session_id: Some(plan.play_session.clone()),
                live_stream_id: negotiated.live_stream.clone(),
                audio_stream_index: plan.audio_stream,
                subtitle_stream_index: plan.subtitle_stream,
                play_method: Some(played(plan.method)),
                position_ticks: Some(plan.start_ticks),
                can_seek: Some(true),
                is_paused: Some(false),
                ..PlaybackStartInfo::default()
            })
            .await
        {
            return Err(Refused::Upstream(upstream.failed(e)));
        }

        Ok(Started {
            plan,
            displaced: lost,
        })
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
        device: &Device,
        progress: &Progress,
    ) -> Result<Standing, Failure> {
        let held = {
            let mut current = self.current.write().await;
            match current.as_mut() {
                Some(active) if active.play_session == progress.play_session => {
                    active.position_ticks = progress.position_ticks;
                    active.touched = Instant::now();
                    active.paused_since = match (progress.paused, active.paused_since) {
                        (true, Some(since)) => Some(since),
                        (true, None) => Some(Instant::now()),
                        (false, _) => None,
                    };
                    active.clone()
                }
                _ => {
                    let records = self.displaced.read().await;
                    let superseded = records
                        .iter()
                        .any(|session| *session == progress.play_session);
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
                end(upstream, device, &released).await;
            }
            return Ok(Standing::Released);
        }

        upstream
            .control()
            .report_playback_progress(&PlaybackProgressInfo {
                item_id: Some(held.item),
                media_source_id: Some(held.media_source.clone()),
                play_session_id: Some(held.play_session.clone()),
                live_stream_id: held.live_stream.clone(),
                audio_stream_index: progress.audio_stream,
                subtitle_stream_index: progress.subtitle_stream,
                play_method: Some(played(held.method)),
                position_ticks: Some(progress.position_ticks),
                is_paused: Some(progress.paused),
                is_muted: Some(progress.muted),
                volume_level: Some(progress.volume),
                repeat_mode: Some(repeated(progress.repeat)),
                can_seek: Some(true),
                ..PlaybackProgressInfo::default()
            })
            .await
            .map_err(|e| upstream.failed(e))?;

        Ok(Standing::Current)
    }

    /// Reports the stop to the Jellyfin server, stops the encode and closes
    /// the live stream, then clears the held session.
    /// A stop naming a session that is no longer held changes nothing.
    pub async fn stopped(
        &self,
        upstream: &Upstream,
        device: &Device,
        stopped: &Stopped,
    ) -> Result<(), Failure> {
        let _transition = self.transition.lock().await;
        let held = {
            let mut current = self.current.write().await;
            match current.as_ref() {
                Some(active) if active.play_session == stopped.play_session => {
                    let mut active = current.take().expect("the session just matched");
                    active.position_ticks = stopped.position_ticks;
                    active
                }
                _ => return Ok(()),
            }
        };
        end(upstream, device, &held).await;
        Ok(())
    }

    /// Ends the held session when its last progress report is older than
    /// `LAPSE`, and a held live session paused longer than `PAUSED_LIVE`,
    /// closing its live stream either way.
    pub async fn sweep(&self, holder: &Holder, device: &Device) {
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
            end(&upstream, device, &lapsed).await;
        }
    }

    /// Ends the held session at its last reported position, which is how the
    /// local server leaves no encode running behind it.
    pub async fn shutdown(&self, holder: &Holder, device: &Device) {
        let _transition = self.transition.lock().await;
        let held = self.current.write().await.take();
        if let Some(held) = held
            && let Some(upstream) = holder.signed().await
        {
            end(&upstream, device, &held).await;
        }
    }
}

/// 409 with the refusal, or the upstream status for a transport failure.
/// A displaced session is told over the event socket before the plan is
/// answered.
pub async fn start(state: State<Arc<AppState>>, request: Json<PlayRequest>) -> Response {
    let Some(upstream) = state.session.signed().await else {
        return (
            StatusCode::CONFLICT,
            Json(jellium_protocol::Refusal::NoSession),
        )
            .into_response();
    };
    match state
        .playback
        .start(&upstream, &state.device, &request, &state.seen)
        .await
    {
        Ok(started) => {
            if let Some(displaced) = started.displaced.as_deref() {
                state.live.displaced(displaced).await;
            }
            (StatusCode::OK, Json(started.plan)).into_response()
        }
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
    match state
        .playback
        .progress(&upstream, &state.device, &progress)
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
    match state
        .playback
        .stopped(&upstream, &state.device, &stopped)
        .await
    {
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

    fn stopped_report(session: &str) -> Stopped {
        Stopped {
            play_session: session.to_string(),
            position_ticks: 5,
        }
    }

    fn progress_report(session: &str) -> Progress {
        Progress {
            play_session: session.to_string(),
            position_ticks: 5,
            paused: false,
            muted: false,
            volume: 100,
            audio_stream: None,
            subtitle_stream: None,
            repeat: Repeat::Off,
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
        }
    }

    #[tokio::test]
    async fn a_report_from_a_displaced_session_is_superseded_and_reaches_no_server() {
        let server = upstream::answering(204).await;
        let upstream = Upstream::stub(&server.base);
        let device = Device::new(Uuid::nil());
        let playback = Playback::new();
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
        let device = Device::new(Uuid::nil());
        let playback = Playback::new();
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
        let device = Device::new(Uuid::nil());
        let playback = Arc::new(Playback::new());
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
        let device = Device::new(Uuid::nil());
        let playback = Playback::new();

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
        let device = Device::new(Uuid::nil());
        let playback = Playback::new();
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
        let device = Device::new(Uuid::nil());
        let playback = Playback::new();
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
        let device = Device::new(Uuid::nil());
        let playback = Playback::new();
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
        let device = Device::new(Uuid::nil());
        let playback = Playback::new();
        *playback.current.write().await = Some(live_active("current", Some(Playback::PAUSED_LIVE)));

        let mut report = progress_report("current");
        report.paused = true;
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
        let device = Device::new(Uuid::nil());
        let playback = Playback::new();
        *playback.current.write().await =
            Some(live_active("current", Some(Duration::from_secs(1))));

        let mut report = progress_report("current");
        report.paused = true;
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
        let device = Device::new(Uuid::nil());
        let playback = Playback::new();
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
        let device = Device::new(Uuid::nil());
        let holder = Holder::new(
            std::env::temp_dir()
                .join("jellium-cli-playback-tests")
                .join("shutdown-live.env"),
            std::sync::Arc::new(Device::new(Uuid::nil())),
        );
        holder.install(Upstream::stub(&server.base)).await;
        let playback = Playback::new();
        *playback.current.write().await = Some(live_active("current", None));

        playback.shutdown(&holder, &device).await;
        assert_eq!(server.asked("/LiveStreams/Close"), 1);
        assert!(playback.current.read().await.is_none());
    }

    #[tokio::test]
    async fn a_displaced_live_session_has_its_stream_closed() {
        let server = upstream::answering(204).await;
        let upstream = Upstream::stub(&server.base);
        let device = Device::new(Uuid::nil());
        let playback = Playback::new();

        let displaced = playback
            .install(live_active("first", None))
            .await
            .is_none()
            .then(|| playback.install(active("second")))
            .expect("the first install displaced nothing")
            .await
            .expect("the second install displaced the first");
        end(&upstream, &device, &displaced).await;

        assert_eq!(displaced.play_session, "first");
        assert_eq!(server.asked("/LiveStreams/Close"), 1);
    }

    #[tokio::test]
    async fn a_sweep_releases_a_live_session_paused_past_the_limit() {
        let server = upstream::answering(204).await;
        let device = Device::new(Uuid::nil());
        let holder = Holder::new(
            std::env::temp_dir()
                .join("jellium-cli-playback-tests")
                .join("sweep-live.env"),
            std::sync::Arc::new(Device::new(Uuid::nil())),
        );
        holder.install(Upstream::stub(&server.base)).await;
        let playback = Playback::new();
        *playback.current.write().await = Some(live_active("current", Some(Playback::PAUSED_LIVE)));

        playback.sweep(&holder, &device).await;
        assert_eq!(server.asked("/LiveStreams/Close"), 1);
        assert!(playback.current.read().await.is_none());
    }

    #[tokio::test]
    async fn a_shutdown_stops_the_encode_it_finds_running() {
        let server = upstream::answering(204).await;
        let device = Device::new(Uuid::nil());
        let holder = Holder::new(
            std::env::temp_dir()
                .join("jellium-cli-playback-tests")
                .join("shutdown.env"),
            std::sync::Arc::new(Device::new(Uuid::nil())),
        );
        holder.install(Upstream::stub(&server.base)).await;
        let playback = Playback::new();
        *playback.current.write().await = Some(active("current"));

        playback.shutdown(&holder, &device).await;
        assert_eq!(server.asked("/Videos/ActiveEncodings"), 1);
        assert!(playback.current.read().await.is_none());
    }
}
