//! The Live TV service the stub upstream answers from.

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use chrono::{DateTime, TimeDelta, Utc};
use jellyfin_api::types::{
    BaseItemDto, BaseItemDtoQueryResult, ChannelType, GetProgramsDto, GuideInfo,
    LiveStreamResponse, LiveTvInfo, LiveTvServiceInfo, MediaSourceInfo, MediaStream,
    MediaStreamType, PlaybackInfoResponse, SeriesTimerInfoDto, SeriesTimerInfoDtoQueryResult,
    TimerInfoDto, TimerInfoDtoQueryResult,
};
use uuid::Uuid;

/// The synthetic Live TV service: `CHANNELS` channels, a program every
/// `PROGRAM` for `DAYS`, the timers and series timers created against it, and
/// a tuner that can be made busy.
#[derive(Clone)]
pub struct LiveTv {
    busy: Arc<AtomicBool>,
    timers: Arc<Mutex<Vec<TimerInfoDto>>>,
    series: Arc<Mutex<Vec<SeriesTimerInfoDto>>>,
    queries: Arc<AtomicUsize>,
    asked: Arc<Mutex<Vec<GetProgramsDto>>>,
}

impl LiveTv {
    /// How many channels the synthetic guide carries.
    pub const CHANNELS: usize = 500;

    /// How many days of programs it carries, from midnight today.
    pub const DAYS: i64 = 14;

    /// How long one program runs.
    pub const PROGRAM: TimeDelta = TimeDelta::minutes(30);

    fn new() -> LiveTv {
        LiveTv {
            busy: Arc::new(AtomicBool::new(false)),
            timers: Arc::new(Mutex::new(Vec::new())),
            series: Arc::new(Mutex::new(Vec::new())),
            queries: Arc::new(AtomicUsize::new(0)),
            asked: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Makes every tuner busy, which makes `OpenLiveStream` refuse, or frees
    /// them.
    pub fn tuners_busy(&self, busy: bool) {
        self.busy.store(busy, Ordering::SeqCst);
    }

    /// The timers held now, in the order they were created.
    pub fn timers(&self) -> Vec<TimerInfoDto> {
        self.timers.lock().expect("the synthetic timers").clone()
    }

    /// The series timers held now, in the order they were created.
    pub fn series(&self) -> Vec<SeriesTimerInfoDto> {
        self.series
            .lock()
            .expect("the synthetic series timers")
            .clone()
    }

    /// How many program queries have been answered, which is what bounds the
    /// guide to one request per screenful.
    pub fn program_queries(&self) -> usize {
        self.queries.load(Ordering::SeqCst)
    }

    /// The channel ids and date bounds of every program query answered, in the
    /// order they arrived.
    pub fn asked(&self) -> Vec<GetProgramsDto> {
        self.asked.lock().expect("the recorded queries").clone()
    }

    /// The instant the synthetic guide begins at: midnight today.
    fn opens() -> DateTime<Utc> {
        Utc::now()
            .date_naive()
            .and_hms_opt(0, 0, 0)
            .expect("midnight")
            .and_utc()
    }

    /// The instant it ends at.
    fn closes() -> DateTime<Utc> {
        Self::opens() + TimeDelta::days(Self::DAYS)
    }

    /// The id of the channel at `index`.
    fn channel(index: usize) -> Uuid {
        Uuid::from_u128(0xC0_0000_0000_0000_0000_0000_0000_0000u128 + index as u128)
    }

    /// The channel at `index`, as the guide and the channel list read it.
    fn channel_item(index: usize) -> BaseItemDto {
        BaseItemDto {
            id: Some(Self::channel(index)),
            name: Some(format!("Channel {}", index + 1)),
            channel_number: Some(format!("{}", index + 1)),
            type_: Some(jellyfin_api::types::BaseItemKind::TvChannel),
            channel_type: Some(if index % 10 == 9 {
                ChannelType::Radio
            } else {
                ChannelType::Tv
            }),
            media_type: Some(jellyfin_api::types::MediaType::Video),
            ..BaseItemDto::default()
        }
    }

    /// The program on `channel` beginning at `start`.
    fn program_item(index: usize, start: DateTime<Utc>) -> BaseItemDto {
        let slot = (start - Self::opens()).num_minutes() / Self::PROGRAM.num_minutes();
        BaseItemDto {
            id: Some(Uuid::from_u128(
                0x9000_0000_0000_0000_0000_0000_0000_0000u128
                    + (index as u128) * 1_000
                    + slot as u128,
            )),
            name: Some(format!("Programme {} on channel {}", slot, index + 1)),
            overview: Some("A synthetic programme.".to_string()),
            type_: Some(jellyfin_api::types::BaseItemKind::Program),
            channel_id: Some(Self::channel(index)),
            channel_name: Some(format!("Channel {}", index + 1)),
            channel_number: Some(format!("{}", index + 1)),
            start_date: Some(start),
            end_date: Some(start + Self::PROGRAM),
            genres: Some(vec!["Synthetic".to_string()]),
            is_live: Some(slot % 7 == 0),
            is_series: Some(slot % 3 == 0),
            is_premiere: Some(slot % 11 == 0),
            is_repeat: Some(slot % 5 == 0),
            ..BaseItemDto::default()
        }
    }

    /// Every program on `channel` overlapping `from..to`.
    fn programs_on(
        &self,
        index: usize,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Vec<BaseItemDto> {
        let opens = Self::opens();
        let closes = Self::closes();
        let from = from.max(opens);
        let to = to.min(closes);
        if from >= to {
            return Vec::new();
        }
        let minutes = Self::PROGRAM.num_minutes();
        let first = (from - opens).num_minutes().div_euclid(minutes);
        let last = (to - opens).num_minutes().div_euclid(minutes);
        let timers = self.timers();
        let series = self.series();
        (first..=last)
            .map(|slot| opens + TimeDelta::minutes(slot * minutes))
            .filter(|start| *start < closes && *start + Self::PROGRAM > from && *start < to)
            .map(|start| {
                let mut item = Self::program_item(index, start);
                let program = item.id;
                item.timer_id = timers
                    .iter()
                    .find(|timer| timer.program_id == program.map(|id| id.to_string()))
                    .and_then(|timer| timer.id.clone());
                item.series_timer_id = series
                    .iter()
                    .find(|timer| timer.id.is_some())
                    .filter(|_| item.timer_id.is_none())
                    .and_then(|timer| timer.id.clone());
                item
            })
            .collect()
    }
}

fn result(items: Vec<BaseItemDto>) -> Json<BaseItemDtoQueryResult> {
    let total = items.len() as i32;
    Json(BaseItemDtoQueryResult {
        items,
        total_record_count: Some(total),
        start_index: Some(0),
    })
}

async fn info() -> Json<LiveTvInfo> {
    Json(LiveTvInfo {
        is_enabled: Some(true),
        services: vec![LiveTvServiceInfo {
            name: Some("Synthetic".to_string()),
            ..LiveTvServiceInfo::default()
        }],
        ..LiveTvInfo::default()
    })
}

async fn guide_info() -> Json<GuideInfo> {
    Json(GuideInfo {
        start_date: Some(LiveTv::opens()),
        end_date: Some(LiveTv::closes()),
    })
}

async fn channels(State(live): State<LiveTv>) -> Json<BaseItemDtoQueryResult> {
    let _ = &live;
    result(
        (0..LiveTv::CHANNELS)
            .map(LiveTv::channel_item)
            .map(|mut channel| {
                let index = channel
                    .channel_number
                    .as_deref()
                    .and_then(|number| number.parse::<usize>().ok())
                    .unwrap_or(1)
                    - 1;
                channel.current_program = Box::new(
                    LiveTv::new()
                        .programs_on(index, Utc::now(), Utc::now())
                        .into_iter()
                        .next(),
                );
                channel
            })
            .collect(),
    )
}

async fn programs(
    State(live): State<LiveTv>,
    Json(query): Json<GetProgramsDto>,
) -> Json<BaseItemDtoQueryResult> {
    live.queries.fetch_add(1, Ordering::SeqCst);
    live.asked
        .lock()
        .expect("the recorded queries")
        .push(query.clone());

    let from = query.min_start_date.unwrap_or_else(LiveTv::opens);
    let to = query.max_start_date.unwrap_or_else(LiveTv::closes);
    let named = query.channel_ids.clone().unwrap_or_default();
    let wanted: Vec<usize> = if named.is_empty() {
        (0..LiveTv::CHANNELS).collect()
    } else {
        named
            .iter()
            .filter_map(|id| (0..LiveTv::CHANNELS).find(|index| LiveTv::channel(*index) == *id))
            .collect()
    };
    result(
        wanted
            .into_iter()
            .flat_map(|index| live.programs_on(index, from, to))
            .collect(),
    )
}

async fn program(Path(id): Path<String>, State(live): State<LiveTv>) -> Response {
    let found = live
        .programs_on(0, LiveTv::opens(), LiveTv::closes())
        .into_iter()
        .find(|item| item.id.map(|id| id.to_string()).as_deref() == Some(id.as_str()));
    match found {
        Some(item) => Json(item).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn recordings() -> Json<BaseItemDtoQueryResult> {
    result(Vec::new())
}

async fn timers(State(live): State<LiveTv>) -> Json<TimerInfoDtoQueryResult> {
    let items = live.timers();
    let total = items.len() as i32;
    Json(TimerInfoDtoQueryResult {
        items,
        total_record_count: Some(total),
        start_index: Some(0),
    })
}

async fn timer_defaults() -> Json<SeriesTimerInfoDto> {
    Json(SeriesTimerInfoDto {
        id: Some("defaults".to_string()),
        pre_padding_seconds: Some(60),
        post_padding_seconds: Some(120),
        record_any_channel: Some(false),
        record_any_time: Some(false),
        record_new_only: Some(true),
        ..SeriesTimerInfoDto::default()
    })
}

async fn create_timer(
    State(live): State<LiveTv>,
    Json(mut timer): Json<TimerInfoDto>,
) -> StatusCode {
    let mut held = live.timers.lock().expect("the synthetic timers");
    timer.id = Some(format!("timer-{}", held.len() + 1));
    held.push(timer);
    StatusCode::NO_CONTENT
}

async fn cancel_timer(Path(id): Path<String>, State(live): State<LiveTv>) -> StatusCode {
    live.timers
        .lock()
        .expect("the synthetic timers")
        .retain(|timer| timer.id.as_deref() != Some(id.as_str()));
    StatusCode::NO_CONTENT
}

async fn series_timers(State(live): State<LiveTv>) -> Json<SeriesTimerInfoDtoQueryResult> {
    let items = live.series();
    let total = items.len() as i32;
    Json(SeriesTimerInfoDtoQueryResult {
        items,
        total_record_count: Some(total),
        start_index: Some(0),
    })
}

async fn create_series(
    State(live): State<LiveTv>,
    Json(mut timer): Json<SeriesTimerInfoDto>,
) -> StatusCode {
    let mut held = live.series.lock().expect("the synthetic series timers");
    timer.id = Some(format!("series-{}", held.len() + 1));
    held.push(timer);
    StatusCode::NO_CONTENT
}

async fn cancel_series(Path(id): Path<String>, State(live): State<LiveTv>) -> StatusCode {
    live.series
        .lock()
        .expect("the synthetic series timers")
        .retain(|timer| timer.id.as_deref() != Some(id.as_str()));
    StatusCode::NO_CONTENT
}

/// The live source a channel negotiates, which requires opening.
fn live_source() -> MediaSourceInfo {
    MediaSourceInfo {
        id: Some("livesource00000000000000000000".to_string()),
        path: Some("/LiveTv/LiveStreamFiles/tuner-1/stream.ts".to_string()),
        container: Some("ts".to_string()),
        requires_opening: Some(true),
        open_token: Some("open-token".to_string()),
        supports_direct_play: Some(true),
        media_streams: Some(vec![MediaStream {
            index: Some(0),
            type_: Some(MediaStreamType::Video),
            ..MediaStream::default()
        }]),
        ..MediaSourceInfo::default()
    }
}

async fn playback_info() -> Json<PlaybackInfoResponse> {
    Json(PlaybackInfoResponse {
        media_sources: vec![live_source()],
        play_session_id: Some("synthetic-session".to_string()),
        ..PlaybackInfoResponse::default()
    })
}

async fn open_live_stream(State(live): State<LiveTv>) -> Response {
    if live.busy.load(Ordering::SeqCst) {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }
    let mut source = live_source();
    source.requires_opening = Some(false);
    source.live_stream_id = Some("live-stream-1".to_string());
    Json(LiveStreamResponse {
        media_source: Some(source),
    })
    .into_response()
}

async fn delivered() -> Response {
    (StatusCode::OK, "synthetic live bytes").into_response()
}

/// The router serving `/LiveTv/GuideInfo`, `/LiveTv/Info`,
/// `/LiveTv/Channels`, `/LiveTv/Programs`, `/LiveTv/Recordings`,
/// `/LiveTv/Timers`, `/LiveTv/SeriesTimers` and the two live delivery shapes,
/// and the service it answers from.
pub fn router() -> (axum::Router, LiveTv) {
    let live = LiveTv::new();
    let router = axum::Router::new()
        .route("/LiveTv/Info", get(info))
        .route("/LiveTv/GuideInfo", get(guide_info))
        .route("/LiveTv/Channels", get(channels))
        .route("/LiveTv/Programs", post(programs))
        .route("/LiveTv/Programs/{id}", get(program))
        .route("/LiveTv/Recordings", get(recordings))
        .route("/LiveTv/Timers", get(timers).post(create_timer))
        .route("/LiveTv/Timers/Defaults", get(timer_defaults))
        .route("/LiveTv/Timers/{id}", axum::routing::delete(cancel_timer))
        .route(
            "/LiveTv/SeriesTimers",
            get(series_timers).post(create_series),
        )
        .route(
            "/LiveTv/SeriesTimers/{id}",
            axum::routing::delete(cancel_series),
        )
        .route("/LiveTv/LiveStreamFiles/{token}/{stream}", get(delivered))
        .route("/LiveTv/LiveRecordings/{id}/stream", get(delivered))
        .route("/LiveStreams/Open", post(open_live_stream))
        .route(
            "/Items/{id}/PlaybackInfo",
            get(playback_info).post(playback_info),
        )
        .with_state(live.clone());
    (router, live)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_synthetic_guide_carries_five_hundred_channels_over_fourteen_days() {
        assert_eq!(LiveTv::CHANNELS, 500);
        assert_eq!(LiveTv::DAYS, 14);
        assert_eq!(LiveTv::closes() - LiveTv::opens(), TimeDelta::days(14));

        let live = LiveTv::new();
        let day = live.programs_on(0, LiveTv::opens(), LiveTv::opens() + TimeDelta::days(1));
        assert_eq!(day.len(), 24 * 60 / LiveTv::PROGRAM.num_minutes() as usize);
        assert!(
            (0..LiveTv::CHANNELS)
                .map(LiveTv::channel)
                .collect::<std::collections::HashSet<_>>()
                .len()
                == LiveTv::CHANNELS
        );
    }

    #[tokio::test]
    async fn a_program_query_answers_only_the_channels_and_span_it_names() {
        let (_, live) = router();
        let from = LiveTv::opens() + TimeDelta::hours(4);
        let to = from + TimeDelta::hours(2);
        let query = GetProgramsDto {
            channel_ids: Some(vec![LiveTv::channel(3)]),
            min_start_date: Some(from),
            max_start_date: Some(to),
            ..GetProgramsDto::default()
        };

        let answered = programs(State(live.clone()), Json(query)).await;
        assert!(!answered.items.is_empty());
        for program in &answered.items {
            assert_eq!(program.channel_id, Some(LiveTv::channel(3)));
            assert!(program.end_date.expect("an end") > from);
            assert!(program.start_date.expect("a start") < to);
        }

        assert_eq!(live.program_queries(), 1);
        let asked = live.asked();
        assert_eq!(asked.len(), 1);
        assert_eq!(asked[0].channel_ids, Some(vec![LiveTv::channel(3)]));
        assert_eq!(asked[0].min_start_date, Some(from));
        assert_eq!(asked[0].max_start_date, Some(to));

        assert!(
            live.programs_on(3, LiveTv::closes(), LiveTv::closes())
                .is_empty()
        );
    }

    #[tokio::test]
    async fn a_busy_tuner_refuses_to_open_a_live_stream() {
        let (_, live) = router();
        live.tuners_busy(true);
        assert_eq!(
            open_live_stream(State(live.clone())).await.status(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
        live.tuners_busy(false);
        assert_eq!(open_live_stream(State(live)).await.status(), StatusCode::OK);
    }
}
