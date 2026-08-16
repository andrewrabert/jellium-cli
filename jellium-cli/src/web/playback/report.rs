//! The three `Sessions/Playing` bodies, in jellyfin-web's key order, and the
//! rate limit `reportPlaybackProgress` holds them to.

use std::time::{Duration, Instant};

use jellium_protocol::report::{Buffered, Queued, Reported, Shuffle};
use jellium_protocol::{Bitrate, Failure, StreamIndex};
use jellyfin_api::types::{PlayMethod, RepeatMode};
use serde::Serialize;
use uuid::Uuid;

use super::Active;
use crate::web::upstream::Upstream;
use crate::web::wire::{self, Query};

/// Where the three reports are posted.
const STARTED: &str = "Sessions/Playing";
const PROGRESS: &str = "Sessions/Playing/Progress";
const ENDED: &str = "Sessions/Playing/Stopped";

/// The body a `Sessions/Playing` report carries, declared in the order
/// `getPlayerState` assigns its keys and `reportPlayback` appends to them.
// reference: get-player-state — playbackmanager.js:2152-2213
// reference: report-playback — playbackmanager.js:75-100
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Body {
    pub volume_level: i32,
    pub is_muted: bool,
    pub is_paused: bool,
    pub repeat_mode: RepeatMode,
    pub shuffle_mode: Shuffle,
    pub max_streaming_bitrate: Bitrate,
    pub position_ticks: i64,
    pub playback_start_time_ticks: i64,
    pub playback_rate: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle_stream_index: Option<StreamIndex>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_subtitle_stream_index: Option<StreamIndex>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_stream_index: Option<StreamIndex>,
    pub buffered_ranges: Vec<Buffered>,
    pub play_method: PlayMethod,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_stream_id: Option<String>,
    pub play_session_id: String,
    pub playlist_item_id: String,
    pub media_source_id: String,
    pub can_seek: bool,
    pub item_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_name: Option<Reported>,
    pub now_playing_queue: Vec<Queued>,
}

impl Body {
    /// The report `active` stands for now: the session's own facts and the last
    /// state the browser reported, which starts as what the play request
    /// carried, so no field is one the local server invented.
    pub fn of(active: &Active) -> Body {
        let reported = &active.reported;
        Body {
            volume_level: reported.volume_level,
            is_muted: reported.muted,
            is_paused: reported.paused,
            repeat_mode: super::repeated(reported.repeat),
            shuffle_mode: reported.shuffle,
            max_streaming_bitrate: active.max_bitrate,
            position_ticks: active.position_ticks,
            playback_start_time_ticks: reported.playback_start_time_ticks,
            playback_rate: reported.playback_rate,
            subtitle_stream_index: reported.subtitle_stream,
            secondary_subtitle_stream_index: reported.secondary_subtitle_stream,
            audio_stream_index: reported.audio_stream,
            buffered_ranges: reported.buffered.clone(),
            play_method: super::played(active.method),
            live_stream_id: active.live_stream.clone(),
            play_session_id: active.play_session.clone(),
            playlist_item_id: reported.playlist_item_id.clone(),
            media_source_id: active.media_source.clone(),
            // reference: get-player-state — playbackmanager.js:2152-2213
            can_seek: active.run_time_ticks.unwrap_or(0) > 0,
            item_id: active.item,
            event_name: None,
            now_playing_queue: reported.queue.clone(),
        }
    }
}

// reference: report-rate-limits — apiClient.js:6-9
const TIME_UPDATE: Duration = Duration::from_millis(10_000);
const VOLUME_CHANGE: Duration = Duration::from_millis(3_000);

/// How long a report of this event is held back from the one before it.
fn limited(event: Reported) -> Duration {
    match event {
        Reported::TimeUpdate => TIME_UPDATE,
        Reported::VolumeChange => VOLUME_CHANGE,
        _ => Duration::ZERO,
    }
}

/// How far a position may diverge from the extrapolated one before the rate
/// limit is dropped and the report goes at once.
const DIVERGENCE: i64 = 50_000_000;

/// Ticks in one millisecond, which is what a report's position is extrapolated
/// at.
const TICKS_PER_MILLISECOND: i64 = 10_000;

/// A report waiting out its rate limit; a later report replaces its body rather
/// than issuing a second request.
struct Deferred {
    /// Tells one wait from the next, so a wait whose report was cancelled sends
    /// nothing when a later report installs its own.
    id: u64,
    body: Body,
    limit: Duration,
    due: Instant,
}

#[derive(Default)]
struct Pending {
    /// When the last report went out, and the position it named.
    last: Option<(Instant, i64)>,
    deferred: Option<Deferred>,
    next: u64,
}

/// Holds progress reports to jellyfin-web's rate limits, coalescing a deferred
/// report to the latest and sending at once when the position diverges.
// reference: report-playback-progress — apiClient.js:3245-3326
pub struct Throttle {
    held: tokio::sync::Mutex<Pending>,
}

impl Throttle {
    pub fn new() -> Throttle {
        Throttle {
            held: tokio::sync::Mutex::new(Pending::default()),
        }
    }

    /// Sends `body` once its rate limit has passed; a report arriving while one
    /// waits replaces it and issues no request of its own.
    pub async fn progress(&self, upstream: &Upstream, body: Body) -> Result<(), Failure> {
        let mine = {
            let mut held = self.held.lock().await;
            let now = Instant::now();
            let event = body.event_name.unwrap_or(Reported::TimeUpdate);
            let mut limit = limited(event);
            let since = held.last.map(|(at, _)| now.duration_since(at));
            if let Some((at, ticks)) = held.last
                && now.duration_since(at) < limit
                && event == Reported::TimeUpdate
            {
                let elapsed = now.duration_since(at).as_millis() as i64;
                let expected = TICKS_PER_MILLISECOND * elapsed + ticks;
                if (body.position_ticks - expected).abs() >= DIVERGENCE {
                    limit = Duration::ZERO;
                }
            }
            let delay = match since {
                Some(since) => limit.saturating_sub(since),
                None => Duration::ZERO,
            };

            if let Some(deferred) = held.deferred.as_mut() {
                deferred.body = body;
                if limit < deferred.limit {
                    deferred.limit = limit;
                    deferred.due = now + delay;
                }
                return Ok(());
            }
            held.next += 1;
            let id = held.next;
            held.deferred = Some(Deferred {
                id,
                body,
                limit,
                due: now + delay,
            });
            id
        };

        loop {
            let due = {
                let held = self.held.lock().await;
                match held.deferred.as_ref() {
                    Some(deferred) if deferred.id == mine => deferred.due,
                    _ => return Ok(()),
                }
            };
            let now = Instant::now();
            if due <= now {
                break;
            }
            tokio::time::sleep(due - now).await;
        }

        let body = {
            let mut held = self.held.lock().await;
            match held.deferred.take() {
                Some(deferred) if deferred.id == mine => {
                    held.last = Some((Instant::now(), deferred.body.position_ticks));
                    deferred.body
                }
                other => {
                    held.deferred = other;
                    return Ok(());
                }
            }
        };
        wire::told(upstream, PROGRESS, &Query::new(), &body).await
    }

    /// Drops the report waiting out its rate limit, which is what
    /// `resetReportPlaybackProgress(this, false)` does before a start and a
    /// stop; the request it would have sent never goes.
    // reference: report-playback-progress — apiClient.js:3245-3326
    pub async fn cancel(&self) {
        self.held.lock().await.deferred = None;
    }
}

pub async fn started(upstream: &Upstream, body: &Body) -> Result<(), Failure> {
    wire::told(upstream, STARTED, &Query::new(), body).await
}

pub async fn stopped(upstream: &Upstream, body: &Body) -> Result<(), Failure> {
    wire::told(upstream, ENDED, &Query::new(), body).await
}
