use std::time::Duration;

use iced::Task;
use jellium_protocol::sync::{self, Correction, Exchange, Schedule};
use jellium_protocol::{
    Group, GroupCommand, GroupEnded, GroupQueue, GroupVerb, Queued, Repeat, Report, Scheduled,
};
use jellyfin_api::types::BaseItemDto;
use uuid::Uuid;

use crate::app::{Message, Signed};
use crate::error;
use crate::error::Answer;
use crate::live;
use crate::player::{self, Playing};

/// How often the browser times its own hop to the local server while
/// converging.
pub const CLOCK_CONVERGING: Duration = Duration::from_millis(200);

/// How many exchanges are taken before the cadence settles.
pub const CLOCK_SAMPLES: u32 = 5;

/// How often the hop is timed once converged.
pub const CLOCK_SETTLED: Duration = Duration::from_secs(5);

/// Milliseconds since the unix epoch, on this browser's clock.
fn now() -> i64 {
    js_sys::Date::now() as i64
}

/// The browser's hop to the local server, measured over the event socket and
/// touching nothing upstream.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Hop {
    /// The local server's clock minus this browser's, in milliseconds.
    pub offset: i64,
    /// The round trip to the local server, in milliseconds.
    pub round_trip: i64,
    /// How many exchanges have completed since membership began.
    pub exchanges: u32,
}

impl Hop {
    /// Takes one exchange: the offset and round trip of the shortest round trip
    /// seen while converging, and of the newest once settled.
    pub fn measured(&mut self, exchange: &Exchange) {
        let round_trip = exchange.round_trip();
        let converging = self.exchanges < CLOCK_SAMPLES;
        let keep = self.exchanges == 0 || !converging || round_trip < self.round_trip;
        if keep {
            self.offset = exchange.offset();
            self.round_trip = round_trip;
        }
        self.exchanges = self.exchanges.saturating_add(1);
    }

    /// `at`, an instant on the local server's clock, on this browser's.
    pub fn locally(&self, at: i64) -> i64 {
        at - self.offset
    }

    /// The wait before the next exchange.
    pub fn cadence(&self) -> Duration {
        if self.exchanges < CLOCK_SAMPLES {
            CLOCK_CONVERGING
        } else {
            CLOCK_SETTLED
        }
    }
}

/// The group this installation is in, and everything its schedule is computed
/// from.
pub struct Joined {
    pub group: Group,
    pub queue: GroupQueue,
    /// The queued items, fetched so the playlist renders.
    pub items: Vec<BaseItemDto>,
    /// True while this tab holds the transport and the queue.
    pub member: bool,
    pub hop: Hop,
    /// The command not yet executed.
    pub pending: Option<Scheduled>,
    /// What the group's schedule says, on this browser's clock.
    pub schedule: Schedule,
    /// The instant a rate correction ends and the rate returns to 1.0.
    pub nudged: Option<i64>,
    /// True while this client has reported buffering and not yet ready.
    pub buffering: bool,
    /// How many consecutive corrections of each kind have been made since the
    /// drift was last inside the tolerance.
    pub attempts: jellium_protocol::sync::Attempts,
    /// The instant the last clock exchange left this browser.
    clocked: i64,
    /// The playlist entry playback was last started for.
    started: Option<Uuid>,
}

impl Joined {
    fn new(group: Group, member: bool) -> Joined {
        Joined {
            group,
            queue: GroupQueue::default(),
            items: Vec::new(),
            member,
            hop: Hop::default(),
            pending: None,
            schedule: Schedule {
                position_ticks: 0,
                at: now(),
                running: false,
            },
            nudged: None,
            buffering: false,
            attempts: jellium_protocol::sync::Attempts::default(),
            clocked: 0,
            started: None,
        }
    }

    /// The entry the group is playing.
    pub fn playing(&self) -> Option<Queued> {
        #[expect(
            clippy::disallowed_methods,
            reason = "a conversion that carries no cause beyond the value itself"
        )]
        let index = usize::try_from(self.queue.playing_index).ok()?;
        self.queue.items.get(index).copied()
    }

    /// The item a playlist entry names.
    pub fn item(&self, playlist_item: Uuid) -> Option<&BaseItemDto> {
        let queued = self
            .queue
            .items
            .iter()
            .find(|entry| entry.playlist_item == playlist_item)?;
        self.items.iter().find(|item| item.id == Some(queued.item))
    }

    /// True while the group waits for its members.
    pub fn waiting(&self) -> bool {
        self.group.state == jellium_protocol::GroupState::Waiting
    }

    /// The items the queue names, in queue order.
    pub fn queued(&self) -> Vec<&BaseItemDto> {
        self.queue
            .items
            .iter()
            .filter_map(|entry| self.items.iter().find(|item| item.id == Some(entry.item)))
            .collect()
    }
}

/// Every control the SyncPlay screen and the group entry points resolve to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// Creates a group carrying what is playing here, named for that item, and
    /// for the user when nothing is playing.
    Create,
    Join(Uuid),
    /// Leaves the group, stopping playback for no other member.
    Leave,
    /// Stops the group, which every member obeys.
    Stop,
    Play(Uuid),
    Remove(Uuid),
}

fn issue(verb: GroupVerb) -> Task<Message> {
    live::send(&Report::Group(verb));
    Task::none()
}

/// The name a new group carries: the item playing here, and the user when
/// nothing plays.
fn naming(signed: &Signed) -> String {
    match signed.playing.as_ref() {
        Some(playing) => crate::text::format(
            crate::text::Text::SyncPlayNamedForItem,
            &[&playing.item.name.clone().unwrap_or_default()],
        ),
        None => crate::text::format(
            crate::text::Text::SyncPlayNamedForUser,
            &[&signed.session.user_name],
        ),
    }
}

/// The current item, the remaining queue and the current position, as a queue
/// the group is handed.
fn handed(playing: &Playing) -> (Vec<Uuid>, i64) {
    let items = playing
        .queue
        .current()
        .and_then(|item| item.id)
        .into_iter()
        .chain(playing.queue.upcoming().filter_map(|(_, item)| item.id))
        .collect();
    (items, player::to_ticks(playing.position))
}

/// Creates a group and hands it the current item, the remaining queue and the
/// current position; with nothing playing, the group is named for the user and
/// starts empty.
pub fn create(signed: &mut Signed) -> Task<Message> {
    live::send(&Report::CreateGroup {
        name: naming(signed),
    });
    let Some(playing) = signed.playing.as_ref() else {
        return Task::none();
    };
    let (items, start_ticks) = handed(playing);
    live::send(&Report::Group(GroupVerb::SetQueue {
        items,
        start_index: 0,
        start_ticks,
    }));
    Task::none()
}

/// Joins `group`: the group's queue and position are adopted and local
/// playback stops, including when the group is idle and its queue is empty.
pub fn join(signed: &mut Signed, group: Uuid) -> Task<Message> {
    live::send(&Report::JoinGroup { group });
    player::leave(signed)
}

/// Applies a control.
pub fn act(signed: &mut Signed, action: Action) -> Task<Message> {
    match action {
        Action::Create => create(signed),
        Action::Join(group) => join(signed, group),
        Action::Leave => leave(signed),
        Action::Stop => issue(GroupVerb::Stop),
        Action::Play(playlist_item) => issue(GroupVerb::SetPlaylistItem { playlist_item }),
        Action::Remove(playlist_item) => issue(GroupVerb::RemoveFromPlaylist {
            playlist_items: vec![playlist_item],
        }),
    }
}

/// Leaves the group, returning the transport and the queue to local ownership
/// carrying whatever the group was playing, and resuming nothing.
pub fn leave(signed: &mut Signed) -> Task<Message> {
    if signed.group.take().is_none() {
        return Task::none();
    }
    live::send(&Report::LeaveGroup);
    player::element::set_group_beacon(false);
    player::leave(signed)
}

/// Applies a group listing.
pub fn listed(signed: &mut Signed, groups: Vec<Group>) {
    signed.groups = groups;
}

/// Applies the group this installation is in; membership beginning stops local
/// playback, and the leave beacon is armed in the member tab alone and
/// disarmed in every other.
pub fn joined(signed: &mut Signed, group: Group, member: bool) -> Task<Message> {
    match signed.group.as_mut() {
        Some(joined) if joined.group.id == group.id => {
            joined.group = group;
            joined.member = member;
            player::element::set_group_beacon(member);
            return Task::none();
        }
        _ => {}
    }
    let leaving = player::leave(signed);
    signed.group = Some(Joined::new(group, member));
    player::element::set_group_beacon(member);
    leaving
}

/// Hands the transport and the queue to the tab that took them: this tab stops
/// being the member and disarms the leave beacon.
pub fn displaced(signed: &mut Signed) {
    if let Some(joined) = signed.group.as_mut() {
        joined.member = false;
    }
    player::element::set_group_beacon(false);
}

/// Leaves the group naming what this browser cannot play.
pub fn unplayable(signed: &mut Signed) -> Task<Message> {
    let leaving = leave(signed);
    crate::failure::raise(crate::error::told(
        crate::text::Text::FailureGroupUnplayable,
    ));
    leaving
}

/// The playlist entry the queue is playing, and the item it names.
fn wanted(joined: &Joined) -> Option<Queued> {
    joined.playing()
}

/// Applies a queue update: the group's queue is rendered from it, and an item
/// change starts negotiation here as `Message::GroupResolved`, reporting buffering as
/// it begins.
/// A queue equal to the one held leaves the schedule and the started item
/// standing and fetches only the items this tab has not seen.
pub fn queued(signed: &mut Signed, queue: GroupQueue) -> Task<Message> {
    let Some(joined) = signed.group.as_mut() else {
        return Task::none();
    };
    let standing = joined.queue == queue;
    joined.queue = queue;
    if !standing {
        joined.schedule = Schedule {
            position_ticks: joined.queue.position_ticks,
            at: now(),
            running: joined.queue.playing,
        };
    }

    let wanted = wanted(joined);
    let ids: Vec<Uuid> = joined.queue.items.iter().map(|entry| entry.item).collect();
    let known: Vec<Uuid> = joined.items.iter().filter_map(|item| item.id).collect();
    let fetching = if ids.iter().any(|id| !known.contains(id)) && !ids.is_empty() {
        let api = signed.api.clone();
        Task::perform(
            async move { api.items(&ids).await },
            Message::GroupItemsLoaded,
        )
    } else {
        Task::none()
    };

    if standing {
        return fetching;
    }

    let Some(entry) = wanted else {
        joined.started = None;
        return Task::batch([fetching, player::leave(signed)]);
    };

    if joined.started == Some(entry.playlist_item) {
        return fetching;
    }
    if !joined.member {
        joined.started = Some(entry.playlist_item);
        return fetching;
    }
    joined.started = Some(entry.playlist_item);
    joined.buffering = true;
    let position_ticks = joined.queue.position_ticks;
    live::send(&Report::Group(GroupVerb::Buffering {
        playing: joined.queue.playing,
        playlist_item: entry.playlist_item,
        position_ticks,
    }));

    let api = signed.api.clone();
    let item = entry.item;
    let starting = Task::perform(
        Answer::of(async move {
            let item = api.item(item).await.bubbled()?;
            Ok(player::Start {
                live: None,
                kind: if matches!(item.type_, Some(jellyfin_api::types::BaseItemKind::Audio)) {
                    player::Kind::Audio
                } else {
                    player::Kind::Video
                },
                items: vec![item],
                position: 0,
                start_ticks: position_ticks,
                mode: jellium_protocol::PlayMode::Now,
                selection: player::Selection::default(),
            })
        }),
        Message::GroupResolved,
    );
    Task::batch([fetching, starting])
}

/// Holds a scheduled command until its deadline; a command whose instant has
/// already passed is executed at once.
pub fn scheduled(signed: &mut Signed, scheduled: Scheduled) -> Task<Message> {
    let Some(joined) = signed.group.as_mut() else {
        return Task::none();
    };
    joined.pending = Some(scheduled);
    ticked(signed)
}

/// Ends membership the local server ended, naming the cause on screen, and
/// returns the transport to the local player.
/// A tab that already left the group is told nothing.
pub fn ended(signed: &mut Signed, cause: GroupEnded) {
    if signed.group.take().is_none() {
        return;
    }
    player::element::set_group_beacon(false);
    crate::failure::raise(error::group_ended(cause));
}

/// The group verb a transport control issues while this tab holds membership,
/// or nothing when the control stays local.
/// Play, pause, seek, stop, next, previous, choosing a queue item, removing a
/// queue item, repeat and shuffle each rebind; a chapter choice seeks the
/// group; volume, mute, fullscreen, audio track, subtitle track and quality
/// stay local.
pub fn rebound(
    joined: &Joined,
    playing: Option<&Playing>,
    action: &player::Action,
    held: &jellium_model::prefs::Held,
) -> Option<GroupVerb> {
    if !joined.member {
        return None;
    }
    let entry = joined.playing();
    let position = playing.map_or(Duration::ZERO, |playing| playing.position);
    match action {
        player::Action::TogglePlay => Some(if joined.schedule.running {
            GroupVerb::Pause
        } else {
            GroupVerb::Unpause
        }),
        player::Action::Seek(at) => Some(GroupVerb::Seek {
            position_ticks: player::to_ticks(*at),
        }),
        player::Action::SelectChapter(start) => Some(GroupVerb::Seek {
            position_ticks: *start,
        }),
        player::Action::SkipBack => Some(GroupVerb::Seek {
            position_ticks: player::to_ticks(
                position.saturating_sub(player::skip(held.skip_back_seconds)),
            ),
        }),
        player::Action::SkipForward => Some(GroupVerb::Seek {
            position_ticks: player::to_ticks(position + player::skip(held.skip_forward_seconds)),
        }),
        player::Action::Leave => Some(GroupVerb::Stop),
        player::Action::Next => entry.map(|entry| GroupVerb::NextItem {
            playlist_item: entry.playlist_item,
        }),
        player::Action::Previous => entry.map(|entry| GroupVerb::PreviousItem {
            playlist_item: entry.playlist_item,
        }),
        player::Action::SetRepeat(repeat) => Some(GroupVerb::SetRepeat { repeat: *repeat }),
        player::Action::CycleRepeat => Some(GroupVerb::SetRepeat {
            repeat: joined.queue.repeat.cycled(),
        }),
        player::Action::ToggleShuffle => Some(GroupVerb::SetShuffle {
            shuffled: !joined.queue.shuffled,
        }),
        _ => None,
    }
}

/// Sends a resolved queue to the group instead of playing it here, taking
/// membership first when another tab holds it.
pub fn play(signed: &mut Signed, start: player::Start) -> Task<Message> {
    let Some(joined) = signed.group.as_mut() else {
        return Task::none();
    };
    if !joined.member {
        joined.member = true;
        live::send(&Report::TakeGroup);
    }
    let items = start.items.iter().filter_map(|item| item.id).collect();
    issue(GroupVerb::SetQueue {
        items,
        start_index: start.position as i32,
        start_ticks: start.start_ticks,
    })
}

/// Translates one inbound control command into the group's equivalent; a verb
/// with no group equivalent reads as `None` and takes effect locally.
pub fn controlled(joined: &Joined, control: &jellium_protocol::Control) -> Option<GroupVerb> {
    use jellium_protocol::Control;
    if !joined.member {
        return None;
    }
    let entry = joined.playing();
    match control {
        Control::Stop => Some(GroupVerb::Stop),
        Control::PlayPause => Some(if joined.schedule.running {
            GroupVerb::Pause
        } else {
            GroupVerb::Unpause
        }),
        Control::Pause => joined.schedule.running.then_some(GroupVerb::Pause),
        Control::Unpause => (!joined.schedule.running).then_some(GroupVerb::Unpause),
        Control::Seek { position_ticks } => Some(GroupVerb::Seek {
            position_ticks: *position_ticks,
        }),
        Control::NextTrack => entry.map(|entry| GroupVerb::NextItem {
            playlist_item: entry.playlist_item,
        }),
        Control::PreviousTrack => entry.map(|entry| GroupVerb::PreviousItem {
            playlist_item: entry.playlist_item,
        }),
        Control::SetRepeat { repeat } => Some(GroupVerb::SetRepeat { repeat: *repeat }),
        Control::SetShuffle { shuffled } => Some(GroupVerb::SetShuffle {
            shuffled: *shuffled,
        }),
        _ => None,
    }
}

/// Reports ready once the element can play through at the commanded position.
pub fn playable(signed: &mut Signed, position: Duration) -> Task<Message> {
    let Some(joined) = signed.group.as_mut() else {
        return Task::none();
    };
    if !joined.member || !joined.buffering {
        return Task::none();
    }
    let Some(entry) = joined.playing() else {
        return Task::none();
    };
    joined.buffering = false;
    issue(GroupVerb::Ready {
        playing: joined.schedule.running,
        playlist_item: entry.playlist_item,
        position_ticks: player::to_ticks(position),
    })
}

/// Reports buffering when the element stalls.
pub fn stalled(signed: &mut Signed) -> Task<Message> {
    let Some(joined) = signed.group.as_mut() else {
        return Task::none();
    };
    if !joined.member || joined.buffering {
        return Task::none();
    }
    let Some(entry) = joined.playing() else {
        return Task::none();
    };
    joined.buffering = true;
    let position_ticks = joined.schedule.position_ticks(now());
    issue(GroupVerb::Buffering {
        playing: joined.schedule.running,
        playlist_item: entry.playlist_item,
        position_ticks,
    })
}

/// Pauses local playback and keeps membership, which is what a dropped event
/// socket does.
pub fn disconnected(signed: &mut Signed) {
    let Some(joined) = signed.group.as_mut() else {
        return;
    };
    joined.schedule.running = false;
    joined.pending = None;
    if let Some(playing) = signed.playing.as_mut() {
        playing.element.set_rate(1.0);
        playing.element.pause();
        playing.paused = true;
    }
    crate::failure::raise(crate::error::told(crate::text::Text::SyncPlayLinkDown));
}

/// Reclaims membership on a reopened socket.
pub fn reconnected(signed: &Signed) -> Task<Message> {
    if signed.group.is_some() {
        live::send(&Report::TakeGroup);
    }
    Task::none()
}

/// Applies one clock exchange the local server answered.
pub fn clocked(signed: &mut Signed, exchange: Exchange) {
    let Some(joined) = signed.group.as_mut() else {
        return;
    };
    let mut exchange = exchange;
    exchange.returned = now();
    joined.hop.measured(&exchange);
}

/// Executes `command` here, taking the schedule it declares.
fn execute(signed: &mut Signed, command: Scheduled) -> Task<Message> {
    let Some(joined) = signed.group.as_mut() else {
        return Task::none();
    };
    joined.schedule = Schedule {
        position_ticks: command.position_ticks,
        at: command.at,
        running: matches!(command.command, GroupCommand::Unpause),
    };
    joined.nudged = None;

    match command.command {
        GroupCommand::Stop => {
            joined.started = None;
            return player::leave(signed);
        }
        GroupCommand::Unpause => {
            if let Some(playing) = signed.playing.as_mut() {
                playing.element.set_rate(1.0);
                playing.element.seek(player::span(command.position_ticks));
                playing.element.play();
                playing.paused = false;
            }
        }
        GroupCommand::Pause | GroupCommand::Seek => {
            if let Some(playing) = signed.playing.as_mut() {
                playing.element.set_rate(1.0);
                playing.element.seek(player::span(command.position_ticks));
                if matches!(command.command, GroupCommand::Pause) {
                    playing.element.pause();
                    playing.paused = true;
                }
            }
        }
    }
    Task::none()
}

/// Corrects the drift from the group's schedule, under the tuning the playback
/// screen holds and against the corrections already made.
fn corrected(signed: &mut Signed) {
    let now = now();
    let tuning = signed.held.sync;
    let Some(joined) = signed.group.as_mut() else {
        return;
    };
    if let Some(until) = joined.nudged {
        if now < until {
            return;
        }
        joined.nudged = None;
        if let Some(playing) = signed.playing.as_ref() {
            playing.element.set_rate(1.0);
        }
        return;
    }
    if !joined.schedule.running || joined.buffering {
        return;
    }
    let Some(playing) = signed.playing.as_ref() else {
        return;
    };
    let scheduled = joined.schedule.position_ticks(now) + tuning.extra_offset_ms * 10_000;
    let Some(position) = playing.element.position() else {
        return;
    };
    let here = player::to_ticks(position);
    let drift = (here - scheduled) / 10_000;
    let correction = sync::correction(drift, tuning, joined.attempts);
    joined.attempts.made(correction);
    match correction {
        Correction::Hold => {}
        Correction::Rate(rate) => {
            playing.element.set_rate(rate);
            joined.nudged = Some(now + sync::NUDGE);
        }
        Correction::Seek => {
            playing.element.set_rate(1.0);
            playing.element.seek(player::span(scheduled));
        }
    }
}

/// One 50 ms pass: it fires a command that has come due, times the hop when
/// the cadence calls for it, and applies the correction the drift from the
/// schedule calls for.
pub fn ticked(signed: &mut Signed) -> Task<Message> {
    let now = now();
    let Some(joined) = signed.group.as_mut() else {
        return Task::none();
    };

    let due = joined
        .pending
        .filter(|command| joined.hop.locally(command.at) <= now);
    let firing = match due {
        Some(command) => {
            joined.pending = None;
            let mut command = command;
            command.at = joined.hop.locally(command.at);
            execute(signed, command)
        }
        None => Task::none(),
    };

    let Some(joined) = signed.group.as_mut() else {
        return firing;
    };
    if now - joined.clocked >= joined.hop.cadence().as_millis() as i64 {
        joined.clocked = now;
        live::send(&Report::Clock {
            sent: now,
            round_trip: joined.hop.round_trip,
        });
    }

    corrected(signed);
    firing
}

/// The repeat mode the group's queue holds.
pub fn repeat(joined: &Joined) -> Repeat {
    joined.queue.repeat
}
