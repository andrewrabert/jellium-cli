use std::time::Duration;

use iced::Task;
use jellium_protocol::{Drive, PlayMode, RemoteEnded, Repeat, Report, Target};
use uuid::Uuid;

use crate::app::{Message, Signed};
use crate::error;
use crate::live;
use crate::player;

/// A tick is a hundred nanoseconds, which is what Jellyfin counts positions in.
const TICKS_PER_SECOND: i64 = 10_000_000;

fn ticks(span: Duration) -> i64 {
    (span.as_secs_f64() * TICKS_PER_SECOND as f64) as i64
}

fn span(ticks: i64) -> Duration {
    Duration::from_secs_f64(ticks.max(0) as f64 / TICKS_PER_SECOND as f64)
}

/// The target this tab drives while remote mode is active.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bound {
    pub target: Target,
    /// Where the scrub handle is being dragged to, while it is held.
    pub scrubbing: Option<Duration>,
}

impl Bound {
    /// The position the panel draws: the one being dragged to while the handle
    /// is held, and the target's otherwise.
    pub fn shown(&self) -> Duration {
        self.scrubbing.unwrap_or_else(|| {
            self.target
                .now_playing
                .as_ref()
                .map_or(Duration::ZERO, |playing| span(playing.position_ticks))
        })
    }

    pub fn duration(&self) -> Duration {
        self.target
            .now_playing
            .as_ref()
            .map_or(Duration::ZERO, |playing| span(playing.run_time_ticks))
    }
}

/// Every control the picker, the remote panel and the rebound transport
/// resolve to.
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Take(String),
    Leave,
    TogglePlay,
    Stop,
    Seek(Duration),
    Scrub(Duration),
    ScrubReleased,
    SkipBack,
    SkipForward,
    Next,
    Previous,
    SetVolume(f32),
    ToggleMute,
    CycleRepeat,
    ToggleShuffle,
}

fn drive(drive: Drive) -> Task<Message> {
    live::send(&Report::Drive(drive));
    Task::none()
}

/// The queue this tab is playing, as the item ids a target is handed.
fn handed(playing: &player::Playing) -> (Vec<Uuid>, i64) {
    let items = playing
        .queue
        .current()
        .and_then(|item| item.id)
        .into_iter()
        .chain(playing.queue.upcoming().filter_map(|(_, item)| item.id))
        .collect();
    (items, ticks(playing.position))
}

/// Takes `target`: sends it the current item, the remaining queue's item ids
/// and the current position and stops local playback when something is
/// playing here, and binds without sending anything when nothing is.
pub fn take(signed: &mut Signed, target: String) -> Task<Message> {
    live::send(&Report::TakeRemote {
        target: target.clone(),
    });
    signed.remote = Some(Bound {
        target: Target {
            session: target,
            device_name: String::new(),
            client_name: String::new(),
            now_playing: None,
        },
        scrubbing: None,
    });
    if let Some(named) = signed
        .targets
        .iter()
        .find(|listed| Some(&listed.session) == signed.remote.as_ref().map(|b| &b.target.session))
        .cloned()
        && let Some(bound) = signed.remote.as_mut()
    {
        bound.target = named;
    }

    let Some(playing) = signed.playing.as_ref() else {
        return Task::none();
    };
    let (items, start_ticks) = handed(playing);
    let sending = drive(Drive::Play {
        items,
        start_index: 0,
        start_ticks,
        mode: PlayMode::Now,
    });
    let leaving = player::leave(signed);
    Task::batch([sending, leaving])
}

/// Applies a control against the bound target.
pub fn act(signed: &mut Signed, action: Action) -> Task<Message> {
    match action {
        Action::Take(target) => return take(signed, target),
        Action::Leave => return leave(signed),
        _ => {}
    }

    let Some(bound) = signed.remote.as_mut() else {
        return Task::none();
    };
    let playing = bound.target.now_playing.clone();

    match action {
        Action::Take(_) | Action::Leave => Task::none(),
        Action::TogglePlay => drive(Drive::PlayPause),
        Action::Stop => drive(Drive::Stop),
        Action::Seek(position) => {
            bound.scrubbing = None;
            drive(Drive::Seek {
                position_ticks: ticks(position),
            })
        }
        Action::Scrub(position) => {
            bound.scrubbing = Some(position);
            Task::none()
        }
        Action::ScrubReleased => match bound.scrubbing.take() {
            Some(position) => act(signed, Action::Seek(position)),
            None => Task::none(),
        },
        Action::SkipBack => drive(Drive::SkipBack),
        Action::SkipForward => drive(Drive::SkipForward),
        Action::Next => drive(Drive::NextTrack),
        Action::Previous => drive(Drive::PreviousTrack),
        Action::SetVolume(level) => drive(Drive::SetVolume {
            level: (level.clamp(0.0, 1.0) * 100.0).round() as i32,
        }),
        Action::ToggleMute => drive(Drive::ToggleMute),
        Action::CycleRepeat => drive(Drive::SetRepeat {
            repeat: playing
                .as_ref()
                .map_or(Repeat::Off, |playing| playing.repeat)
                .cycled(),
        }),
        Action::ToggleShuffle => drive(Drive::SetShuffle {
            shuffled: !playing.as_ref().is_some_and(|playing| playing.shuffled),
        }),
    }
}

/// Sends a resolved queue to the target instead of playing it here, which is
/// where every Play, Play All and instant mix goes while the mode is active.
pub fn play(signed: &mut Signed, start: player::Start) -> Task<Message> {
    if signed.remote.is_none() {
        return Task::none();
    }
    let items = start.items.iter().filter_map(|item| item.id).collect();
    drive(Drive::Play {
        items,
        start_index: start.position as i32,
        start_ticks: start.start_ticks,
        mode: start.mode,
    })
}

/// Leaves the mode, returning the transport to the local player and resuming
/// nothing.
pub fn leave(signed: &mut Signed) -> Task<Message> {
    if signed.remote.take().is_none() {
        return Task::none();
    }
    live::send(&Report::LeaveRemote);
    Task::none()
}

/// Applies a target listing: the picker's contents, and the bound target's
/// state.
pub fn listed(signed: &mut Signed, targets: Vec<Target>) {
    if let Some(bound) = signed.remote.as_mut()
        && let Some(named) = targets
            .iter()
            .find(|listed| listed.session == bound.target.session)
    {
        bound.target = named.clone();
    }
    signed.targets = targets;
}

/// Ends the mode the local server ended, naming the cause on screen.
pub fn ended(signed: &mut Signed, cause: RemoteEnded) {
    signed.remote = None;
    crate::failure::raise(error::remote_ended(cause));
}
