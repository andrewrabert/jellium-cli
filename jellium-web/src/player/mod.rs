use std::ops::Not as _;
use std::rc::Rc;
use std::time::Duration;

use iced::Subscription;
use iced::keyboard;
use iced::{Task, event, mouse, touch};
use jellium_protocol::{
    Control, Plan, PlayMode, PlayRequest, PlaybackRefused, Progress, Quality, Repeat, Standing,
    Stopped,
};
use jellyfin_api::types::{BaseItemDto, BaseItemKind};
use uuid::Uuid;

pub mod binding;
pub mod capability;
pub mod control;
pub mod element;
pub mod group;
pub mod osd;
pub mod queue;
pub mod remote;
pub mod scrub;
mod seconds;
pub mod trickplay;

pub use control::Planned;
pub use element::{Asked, Element, Event, Fault, Generation, Kind, Metadata, Raised, TextTrack};
pub use queue::Queue;

pub mod live;

use crate::api::Api;
use crate::app::{Message, Signed};
use crate::error::{Answer, Trouble};
use crate::images::{self, Kind as ImageKind};
use crate::route::Route;
use crate::text::Text;
use crate::theme;

pub const TICKS_PER_SECOND: i64 = 10_000_000;

/// How far one arrow key moves the volume.
pub(crate) const VOLUME_STEP: f32 = 0.05;

pub fn to_ticks(position: Duration) -> i64 {
    (position.as_secs_f64() * TICKS_PER_SECOND as f64).round() as i64
}

/// The ask that arms the page-hide beacon reporting `stopped`.
fn beacon(stopped: &Stopped) -> Asked<'_> {
    Asked::Beacon {
        path: control::endpoint(jellium_protocol::PLAYBACK_STOPPED_PATH),
        stopped,
    }
}

pub fn span(ticks: i64) -> Duration {
    Duration::from_secs_f64(ticks.max(0) as f64 / TICKS_PER_SECOND as f64)
}

/// One chosen skip length as the element's clock takes it.
pub(crate) fn skip(seconds: i64) -> Duration {
    Duration::from_secs(seconds.max(0) as u64)
}

/// What a play control asked for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Intent {
    /// The item alone, from its start or from its stored position.
    Item { item: Uuid, resume: bool },
    /// The item's children, in order or shuffled.
    All { item: Uuid, shuffle: bool },
    /// The Jellyfin server's instant mix for the item.
    Mix { item: Uuid },
}

/// The stream selections a control command carried.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Selection {
    pub media_source: Option<String>,
    pub audio_stream: Option<i32>,
    pub subtitle_stream: Option<i32>,
}

/// A queue resolved from an intent or a control command, ready to play.
#[derive(Debug, Clone)]
pub struct Start {
    /// The channel this start plays and the channel list its display moves
    /// through; absent for every ordinary item.
    pub live: Option<live::Live>,
    pub kind: Kind,
    pub items: Vec<BaseItemDto>,
    pub position: usize,
    pub start_ticks: i64,
    /// What the start does to the queue already playing, and whether it
    /// shuffles.
    pub mode: PlayMode,
    /// Applied to the first item this start plays.
    pub selection: Selection,
}

/// What an inbound `Play` asked for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Commanded {
    pub items: Vec<Uuid>,
    pub mode: PlayMode,
    pub start_index: i32,
    pub start_ticks: i64,
    pub selection: Selection,
}

/// Which menu the on-screen display has open, if any.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Menu {
    Audio,
    Subtitle,
    Quality,
    Chapters,
    Version,
}

/// Every control the on-screen display, the now-playing bar, the keyboard and
/// the operating system's media keys resolve to.
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    TogglePlay,
    Seek(Duration),
    /// The scrub handle being dragged: the display follows it and the element
    /// does not move.
    Scrub(Duration),
    /// The scrub handle released, which is the one seek a drag performs.
    ScrubReleased,
    SkipBack,
    SkipForward,
    SetVolume(f32),
    ToggleMute,
    SelectAudio(i32),
    SelectSubtitle(Option<i32>),
    SelectQuality(Quality),
    SelectVersion(String),
    SelectChapter(i64),
    /// The pointer moved over the scrub bar without pressing it.
    Hovered(Duration),
    /// The pointer left the scrub bar.
    Unhovered,
    /// The pointer has settled long enough for a preview to be asked for.
    Settled,
    OpenMenu(Menu),
    CloseMenu,
    ToggleShuffle,
    CycleRepeat,
    /// The repeat mode a control command named, taken whatever the queue
    /// holds now.
    SetRepeat(Repeat),
    RemoveQueued(usize),
    Next,
    Previous,
    ToggleFullscreen,
    Stirred,
    Leave,
    /// Shows the display when it is hidden, and hides it otherwise.
    ToggleDisplay,
    /// Creates a timer for the program being watched.
    Record,
}

/// The element and queue a play request was issued for, held until its plan
/// arrives.
pub struct Pending {
    pub element: Element,
    pub queue: Queue,
    /// The live context the plan installs, carried across a re-negotiation.
    pub live: Option<live::Live>,
}

/// The playback in progress: one item, its plan, its element and its queue.
pub struct Playing {
    /// The live context while a channel plays.
    pub live: Option<live::Live>,
    pub item: BaseItemDto,
    pub plan: Plan,
    pub element: Element,
    /// The stream the element is playing; an event from any other is stale.
    /// The stream the element loaded, and `None` when the glue would not load
    /// one, so no event is matched against a generation.
    pub generation: Option<Generation>,
    pub queue: Queue,
    pub position: Duration,
    /// Where the scrub handle is being dragged to, while it is held.
    pub scrubbing: Option<Duration>,
    pub buffered: Duration,
    pub duration: Duration,
    pub paused: bool,
    pub fullscreen: bool,
    pub menu: Option<Menu>,
    /// Time since the last input, which hides the display and the cursor at
    /// `theme::IDLE_HIDE`.
    pub idle: Duration,
    /// True once this item has retried with direct play disabled.
    pub retried: bool,
    /// True once this item has been resumed after an interruption: a dropped
    /// stream or a lapsed session.
    pub resumed: bool,
    /// The failure shown over the player.
    pub trouble: Option<Text>,
    /// True once the item finished and next-episode autoplay held the queue at
    /// the boundary; the next control still advances.
    pub ended: bool,
    /// The trickplay the playing item describes, re-read when another version
    /// is selected.
    pub trickplay: crate::player::trickplay::Trickplay,
    /// The scrub preview shown now.
    pub preview: Option<crate::player::trickplay::Preview>,
}

impl Playing {
    /// The position the display draws: the one being dragged to while the
    /// scrub handle is held, and the element's otherwise.
    pub fn shown(&self) -> Duration {
        self.scrubbing.unwrap_or(self.position)
    }

    /// True while the video element holds the viewport, which is when the
    /// canvas clears transparent and the browsing screens are not drawn.
    pub fn video(&self) -> bool {
        self.element.kind() == Kind::Video
    }

    fn hidden(&self) -> bool {
        !self.paused && self.menu.is_none() && self.idle >= theme::IDLE_HIDE
    }

    /// The ask that matches whether the pointer is hidden over the canvas.
    fn idling(&self) -> Asked<'static> {
        if self.hidden() {
            Asked::Idle
        } else {
            Asked::Awake
        }
    }

    fn text_tracks(&self) -> Vec<TextTrack> {
        self.plan
            .subtitle_streams
            .iter()
            .filter_map(|stream| {
                Some(TextTrack {
                    path: stream.track.clone()?,
                    label: stream.label.clone(),
                    language: stream.language.clone(),
                })
            })
            .collect()
    }

    fn selected_track(&self) -> Option<usize> {
        let wanted = self.plan.subtitle_stream?;
        self.plan
            .subtitle_streams
            .iter()
            .filter(|stream| stream.track.is_some())
            .position(|stream| stream.index == wanted)
    }

    fn stopped(&self) -> Stopped {
        Stopped {
            play_session: self.plan.play_session.clone(),
            position_ticks: to_ticks(self.position),
        }
    }

    fn progress(&self, device: crate::prefs::Device) -> Progress {
        Progress {
            play_session: self.plan.play_session.clone(),
            position_ticks: to_ticks(self.position),
            paused: self.paused,
            muted: device.muted,
            volume: (device.volume * 100.0).round() as i32,
            audio_stream: self.plan.audio_stream,
            subtitle_stream: self.plan.subtitle_stream,
            repeat: self.queue.repeat(),
        }
    }

    fn metadata(&self) -> Metadata {
        Metadata {
            title: self.item.name.clone().unwrap_or_default(),
            subtitle: self
                .item
                .series_name
                .clone()
                .or_else(|| self.item.album_artist.clone())
                .or_else(|| self.item.album.clone())
                .unwrap_or_default(),
            artwork: self.item.id.map(|item| {
                crate::images::url(images::Key {
                    item,
                    kind: ImageKind::Primary,
                    index: None,
                    width: theme::IMAGE_WIDTH,
                })
            }),
        }
    }
}

/// The stored position an item resumes at.
fn resume_ticks(item: &BaseItemDto) -> i64 {
    item.user_data
        .as_ref()
        .and_then(|data| data.playback_position_ticks)
        .unwrap_or_default()
}

fn audible(item: &BaseItemDto) -> bool {
    matches!(
        item.type_,
        Some(BaseItemKind::Audio | BaseItemKind::MusicAlbum | BaseItemKind::MusicArtist)
    )
}

/// Resolves an intent into a queue: an episode queues the rest of its season,
/// a series, season, album or artist queues its children, and a mix queues the
/// Jellyfin server's instant mix.
pub async fn resolve(api: Rc<Api>, intent: Intent) -> Answer<Start> {
    Answer::of(async {
        let id = match intent {
            Intent::Item { item, .. } | Intent::All { item, .. } | Intent::Mix { item } => item,
        };
        let item = api.item(id).await.bubbled()?;

        let (items, position, shuffle, start_ticks) = match intent {
            Intent::Item { resume, .. } => {
                let queued = api.queue(&item).await.bubbled()?;
                let position = queued
                    .iter()
                    .position(|queued| queued.id == item.id)
                    .unwrap_or(0);
                let start = if resume { resume_ticks(&item) } else { 0 };
                let items = if queued.is_empty() {
                    vec![item.clone()]
                } else {
                    queued
                };
                (items, position, false, start)
            }
            Intent::All { shuffle, .. } => {
                let queued = api.queue(&item).await.bubbled()?;
                (queued, 0, shuffle, 0)
            }
            Intent::Mix { .. } => (api.instant_mix(&item).await.bubbled()?, 0, false, 0),
        };

        let kind = if items.first().is_some_and(audible) || audible(&item) {
            Kind::Audio
        } else {
            Kind::Video
        };

        Ok(Start {
            live: None,
            kind,
            items,
            position,
            start_ticks,
            mode: if shuffle {
                PlayMode::Shuffle
            } else {
                PlayMode::Now
            },
            selection: Selection::default(),
        })
    })
    .await
}

/// The queue an inbound `Play` names: its items fetched in one request and
/// held in the order it named them, or the Jellyfin server's instant mix for
/// the first of them when the mode asks for one.
/// An id the Jellyfin server hands over as a channel resolves as that channel
/// alone, from the live edge, whatever mode the command named.
/// A command naming no item the Jellyfin server hands over is a relay trouble.
pub async fn commanded(api: Rc<Api>, asked: Commanded) -> Answer<Start> {
    Answer::of(async {
        let handed = api.items(&asked.items).await.bubbled()?;
        let mut fetched = Vec::with_capacity(handed.len());
        for id in &asked.items {
            if let Some(item) = handed.iter().find(|item| item.id == Some(*id)) {
                fetched.push(item.clone());
            }
        }
        if let Some(channel) = fetched
            .first()
            .filter(|item| crate::livetv::Channel::read(item).is_some())
            .cloned()
        {
            return live::tuned(api, channel).await.bubbled();
        }
        if asked.mode.mixes() {
            let Some(seed) = fetched.first() else {
                return Err(crate::error::Bubble::from(Trouble::Relay {
                    status: None,
                    detail: "the play command named no item this server would hand over"
                        .to_string(),
                }));
            };
            fetched = api.instant_mix(seed).await.bubbled()?;
        }
        if fetched.is_empty() {
            return Err(crate::error::Bubble::from(Trouble::Relay {
                status: None,
                detail: "the play command named no item this server would hand over".to_string(),
            }));
        }
        let position = (asked.start_index.max(0) as usize).min(fetched.len() - 1);
        let kind = if fetched.get(position).is_some_and(audible) {
            Kind::Audio
        } else {
            Kind::Video
        };
        Ok(Start {
            live: None,
            kind,
            items: fetched,
            position,
            start_ticks: asked.start_ticks,
            mode: asked.mode,
            selection: asked.selection,
        })
    })
    .await
}

fn request(signed: &Signed, item: Uuid, start_ticks: i64, selection: &Selection) -> PlayRequest {
    PlayRequest {
        item,
        media_source: selection.media_source.clone(),
        audio_stream: selection.audio_stream,
        subtitle_stream: selection.subtitle_stream,
        start_ticks,
        quality: signed.held.quality,
        capabilities: signed.capabilities.clone(),
        allow_direct_play: true,
    }
}

/// Mounts the element, stopping any audio already playing, and asks the local
/// server for a plan for the queue's first item, on the streams the start
/// selected.
/// A start whose mode is `Next` or `Last` queues its items after the current
/// item, or at the end, when something of the same kind plays here, and
/// disturbs neither that playback nor its position; with nothing playing here,
/// or with the other kind playing, it starts playback with its items.
pub fn begin(signed: &mut Signed, start: Start) -> Task<Message> {
    if matches!(start.mode, PlayMode::Next | PlayMode::Last)
        && let Some(playing) = signed.playing.as_mut()
        && playing.element.kind() == start.kind
    {
        match start.mode {
            PlayMode::Next => playing.queue.insert_next(start.items),
            _ => playing.queue.append(start.items),
        }
        return Task::none();
    }

    let leaving = leave(signed);

    let queue = Queue::new(start.items, start.position, start.mode.shuffles());
    let Some(item) = queue.current().and_then(|item| item.id) else {
        return leaving;
    };
    let Some(element) = Element::mount(start.kind) else {
        return leaving;
    };
    element.ask(&Asked::Volume {
        volume: signed.device.volume,
    });
    element.ask(&Asked::CueStyle {
        cues: &signed.held.cues(),
    });
    element.ask(&muting(signed.device));

    signed.pending = Some(Pending {
        element,
        queue,
        live: start.live,
    });

    let request = request(signed, item, start.start_ticks, &start.selection);
    Task::batch([
        leaving,
        Task::perform(control::start(request), Message::Planned),
    ])
}

/// The ask that matches the mute state `device` holds.
fn muting(device: crate::prefs::Device) -> Asked<'static> {
    if device.muted {
        Asked::Muted
    } else {
        Asked::Unmuted
    }
}

/// Asks for a plan for `item` reusing the element and queue already mounted.
fn replan(signed: &mut Signed, request: PlayRequest) -> Task<Message> {
    let Some(playing) = signed.playing.take() else {
        return Task::none();
    };
    let Playing {
        element,
        queue,
        live,
        ..
    } = playing;
    signed.pending = Some(Pending {
        element,
        queue,
        live,
    });
    Task::perform(control::start(request), Message::Planned)
}

/// Installs a plan: loads the element, applies the held volume, mute and
/// subtitle selection, publishes the media session metadata and reports
/// playback start.
pub fn installed(signed: &mut Signed, plan: Plan) -> Task<Message> {
    let Some(Pending {
        element,
        queue,
        live,
    }) = signed.pending.take()
    else {
        return Task::none();
    };
    let Some(item) = queue.current().cloned() else {
        return Task::none();
    };

    let generation = element.load(&plan.delivery, span(plan.start_ticks));

    let playing = Playing {
        live: live.map(|mut live| {
            live.tuning = false;
            live
        }),
        item,
        generation,
        position: span(plan.start_ticks),
        scrubbing: None,
        buffered: Duration::ZERO,
        duration: plan.run_time_ticks.map(span).unwrap_or_default(),
        paused: false,
        fullscreen: false,
        menu: None,
        idle: Duration::ZERO,
        retried: plan.method != jellium_protocol::Method::DirectPlay,
        resumed: false,
        trouble: None,
        ended: false,
        trickplay: trickplay::Trickplay::default(),
        preview: None,
        plan,
        element,
        queue,
    };

    playing.element.ask(&Asked::Volume {
        volume: signed.device.volume,
    });
    playing.element.ask(&muting(signed.device));
    playing.element.ask(&Asked::TextTracks {
        tracks: &playing.text_tracks(),
        selected: playing.selected_track(),
    });
    playing.element.ask(&Asked::CueStyle {
        cues: &signed.held.cues(),
    });
    playing.element.ask(&Asked::Metadata {
        metadata: &playing.metadata(),
    });
    playing.element.ask(&beacon(&playing.stopped()));

    let progress = playing.progress(signed.device);
    signed.playing = Some(playing);
    Task::perform(control::progress(progress), Message::Reported)
}

/// Plays a resolved start where this tab's mode puts it: a bound target, a
/// joined group's queue, or here.
/// A channel goes to a bound target and plays here otherwise; it never becomes
/// a group queue.
pub fn started(signed: &mut Signed, start: Start) -> Task<Message> {
    if start.live.is_some() {
        if signed.remote.is_some() {
            return remote::play(signed, start);
        }
        return begin(signed, start);
    }
    if signed.group.is_some() {
        return group::play(signed, start);
    }
    if signed.remote.is_some() {
        return remote::play(signed, start);
    }
    begin(signed, start)
}

/// What the application acts on once a play request's answer has been
/// reported.
#[must_use = "a play request's outcome leaves a pending request that must be installed or dropped"]
pub enum Outcome {
    Plan(Box<Plan>),
    /// No plan; whatever named that was raised inside `planned`, so nothing is
    /// carried out of it.
    Unplanned,
}

/// `answered` reported and reduced to what the stage acts on: a refusal and a
/// trouble are each raised as a failure here, before any stage is looked at,
/// so an answer arriving after the session is gone is still reported.
pub fn planned(answered: crate::error::Answer<control::Planned>) -> Outcome {
    match answered.or_none(crate::text::Text::FailurePlaybackUnplanned) {
        Some(control::Planned::Plan(plan)) => Outcome::Plan(plan),
        Some(control::Planned::Refused(refused)) => {
            crate::failure::raise(crate::error::refused(&refused));
            Outcome::Unplanned
        }
        None => Outcome::Unplanned,
    }
}

/// Applies a play request that yielded no plan, already reported: the pending
/// request and the playback are dropped, and a held group is left under
/// `failureGroupUnplayable`, which names no refusal and so takes none.
pub fn unplanned(signed: &mut Signed) -> Task<Message> {
    signed.pending = None;
    signed.playing = None;
    if signed
        .group
        .as_ref()
        .is_some_and(|joined| joined.membership == group::Membership::Holding)
    {
        return crate::player::group::unplayable(signed);
    }
    Task::none()
}

/// Plays the queue's current item from `start_ticks`, reusing the element.
fn play_current(signed: &mut Signed, start_ticks: i64) -> Task<Message> {
    let Some(playing) = signed.playing.as_ref() else {
        return Task::none();
    };
    let Some(item) = playing.queue.current().and_then(|item| item.id) else {
        return leave(signed);
    };
    let request = request(signed, item, start_ticks, &Selection::default());
    replan(signed, request)
}

/// Applies a media element event, dropping one raised by a stream the player
/// has already replaced.
/// An ended episode with next-episode autoplay off and no group membership
/// advances nothing: the element stops, the stop is reported, and the player
/// view stays on the finished item with its next-item indication shown.
/// A decode fault retries the same item once with direct play disabled; a
/// dropped stream resumes once at the last reported position; a second failure
/// of either kind is shown.
/// An ended item advances the queue, and an exhausted queue leaves the player.
pub fn event(signed: &mut Signed, raised: Raised) -> Task<Message> {
    let Some(generation) = signed
        .playing
        .as_ref()
        .and_then(|playing| playing.generation)
    else {
        return Task::none();
    };
    if raised.generation != generation {
        return Task::none();
    }
    match raised.event {
        Event::Stalled => return group::stalled(signed),
        Event::Playable { position } => return group::playable(signed, position),
        _ => {}
    }
    let Some(playing) = signed.playing.as_mut() else {
        return Task::none();
    };
    match raised.event {
        Event::Ready { duration } => {
            if !duration.is_zero() {
                playing.duration = duration;
            }
            Task::none()
        }
        Event::Progress {
            position,
            buffered,
            paused,
        } => {
            playing.position = position;
            playing.buffered = buffered;
            playing.paused = paused;
            let stopped = playing.stopped();
            playing.element.ask(&beacon(&stopped));
            Task::none()
        }
        Event::ReportDue { position } => {
            playing.position = position;
            let progress = playing.progress(signed.device);
            Task::perform(control::progress(progress), Message::Reported)
        }
        Event::Stalled | Event::Playable { .. } => Task::none(),
        Event::Ended => {
            // an episode the configuration says not to follow stops at the
            // boundary; an audio queue, an instant mix, Play All and a group's
            // queue advance regardless
            let episode = playing.item.type_ == Some(jellyfin_api::types::BaseItemKind::Episode);
            let following = jellium_model::user::next_episode_autoplay(&signed.configuration);
            if episode && !following && signed.group.is_none() {
                playing.ended = true;
                playing.paused = true;
                playing.element.ask(&Asked::Pause);
                let stopped = playing.stopped();
                return Task::perform(
                    async move { control::stopped(stopped).await.map(|()| Standing::Current) },
                    Message::Reported,
                );
            }
            let advanced = playing.queue.advance().and_then(|item| item.id);
            match advanced {
                Some(_) => {
                    let start = playing
                        .queue
                        .current()
                        .map(resume_ticks)
                        .unwrap_or_default();
                    let stopped = playing.stopped();
                    let reported = Task::perform(
                        async move { control::stopped(stopped).await.map(|()| Standing::Current) },
                        Message::Reported,
                    );
                    Task::batch([reported, play_current(signed, start)])
                }
                None => leave(signed),
            }
        }
        Event::Failed { fault } => failed(signed, fault),
        Event::Command { command } => {
            let action = match command {
                element::Command::Play | element::Command::Pause => Action::TogglePlay,
                element::Command::Previous => Action::Previous,
                element::Command::Next => Action::Next,
                element::Command::SeekTo { position } => Action::Seek(position),
            };
            act(signed, action)
        }
    }
}

fn failed(signed: &mut Signed, fault: Fault) -> Task<Message> {
    let Some(playing) = signed.playing.as_mut() else {
        return Task::none();
    };
    match fault {
        Fault::Decode | Fault::Unsupported if !playing.retried => {
            playing.retried = true;
            let Some(item) = playing.queue.current().and_then(|item| item.id) else {
                return leave(signed);
            };
            let position = to_ticks(playing.position);
            let mut request = request(signed, item, position, &Selection::default());
            request.allow_direct_play = false;
            replan(signed, request)
        }
        Fault::Network if playing.live.is_some() => {
            let resumed = playing.live.as_ref().is_some_and(|live| live.resumed);
            if resumed {
                playing.trouble = Some(Text::FailureLiveStreamDropped);
                return Task::none();
            }
            if let Some(live) = playing.live.as_mut() {
                live.resumed = true;
            }
            playing.element.ask(&Asked::SeekToLive);
            play_current(signed, 0)
        }
        Fault::Network if !playing.resumed => {
            playing.resumed = true;
            let position = to_ticks(playing.position);
            play_current(signed, position)
        }
        Fault::Network => {
            playing.trouble = Some(Text::FailureStreamDropped);
            Task::none()
        }
        Fault::Decode | Fault::Unsupported => {
            playing.trouble = Some(Text::FailureDecode);
            Task::none()
        }
    }
}

/// Applies a control, re-negotiating when the selection changes the stream a
/// track, quality or version choice needs.
/// While this tab holds group membership, a control the group owns issues the
/// group's equivalent instead of acting here.
pub fn act(signed: &mut Signed, action: Action) -> Task<Message> {
    if let Some(joined) = signed.group.as_ref()
        && let Some(verb) = group::rebound(joined, signed.playing.as_ref(), &action, &signed.held)
    {
        crate::live::send(&jellium_protocol::Report::Group(verb));
        if let Some(playing) = signed.playing.as_mut() {
            playing.idle = Duration::ZERO;
        }
        return Task::none();
    }
    let Some(playing) = signed.playing.as_mut() else {
        return Task::none();
    };
    playing.idle = Duration::ZERO;

    let live = playing.live.is_some();
    if live
        && matches!(
            action,
            Action::SkipBack
                | Action::SkipForward
                | Action::Seek(_)
                | Action::Scrub(_)
                | Action::ScrubReleased
                | Action::SelectChapter(_)
        )
    {
        return Task::none();
    }
    if live && matches!(action, Action::TogglePlay) && playing.paused {
        return live::unpause(signed);
    }
    if live && matches!(action, Action::Next) {
        return live::step(signed, true);
    }
    if live && matches!(action, Action::Previous) {
        return live::step(signed, false);
    }

    match action {
        Action::Stirred => Task::none(),
        Action::Record => live::record(signed),
        Action::TogglePlay => {
            if playing.paused {
                playing.element.ask(&Asked::Play);
            } else {
                playing.element.ask(&Asked::Pause);
            }
            playing.paused = !playing.paused;
            let progress = playing.progress(signed.device);
            Task::perform(control::progress(progress), Message::Reported)
        }
        Action::Seek(position) => {
            playing.element.ask(&Asked::Seek { position });
            playing.position = position;
            let progress = playing.progress(signed.device);
            Task::perform(control::progress(progress), Message::Reported)
        }
        Action::Scrub(position) => {
            playing.scrubbing = Some(position);
            Task::none()
        }
        Action::ScrubReleased => match playing.scrubbing.take() {
            Some(position) => act(signed, Action::Seek(position)),
            None => Task::none(),
        },
        Action::SkipBack => {
            let position = playing
                .position
                .saturating_sub(skip(signed.held.skip_back_seconds));
            act(signed, Action::Seek(position))
        }
        Action::SkipForward => {
            let position = playing.position + skip(signed.held.skip_forward_seconds);
            act(signed, Action::Seek(position))
        }
        Action::SetVolume(volume) => {
            signed.device.volume = volume.clamp(0.0, 1.0);
            signed.device.store();
            playing.element.ask(&Asked::Volume {
                volume: signed.device.volume,
            });
            Task::none()
        }
        Action::ToggleMute => {
            signed.device.muted = !signed.device.muted;
            signed.device.store();
            playing.element.ask(&muting(signed.device));
            Task::none()
        }
        Action::SelectAudio(index) => {
            playing.menu = None;
            let position = to_ticks(playing.position);
            let source = playing.plan.media_source.clone();
            let subtitle = playing.plan.subtitle_stream;
            let Some(item) = playing.queue.current().and_then(|item| item.id) else {
                return Task::none();
            };
            let mut request = request(signed, item, position, &Selection::default());
            request.media_source = Some(source);
            request.audio_stream = Some(index);
            request.subtitle_stream = subtitle;
            replan(signed, request)
        }
        Action::SelectSubtitle(index) => {
            playing.menu = None;
            let position = to_ticks(playing.position);
            let source = playing.plan.media_source.clone();
            let audio = playing.plan.audio_stream;
            let Some(item) = playing.queue.current().and_then(|item| item.id) else {
                return Task::none();
            };
            let mut request = request(signed, item, position, &Selection::default());
            request.media_source = Some(source);
            request.audio_stream = audio;
            request.subtitle_stream = index;
            replan(signed, request)
        }
        Action::SelectQuality(quality) => {
            playing.menu = None;
            // the on-screen display's choice applies for the run; only a save
            // on the playback screen writes the ceiling to the server
            signed.held.quality = quality;
            let position = to_ticks(playing.position);
            play_current(signed, position)
        }
        Action::SelectVersion(id) => {
            playing.menu = None;
            let Some(item) = playing.queue.current().and_then(|item| item.id) else {
                return Task::none();
            };
            let mut request = request(signed, item, 0, &Selection::default());
            request.media_source = Some(id);
            replan(signed, request)
        }
        Action::SelectChapter(start) => {
            playing.menu = None;
            act(signed, Action::Seek(span(start)))
        }
        Action::Hovered(at) => {
            match playing.preview.as_mut() {
                Some(preview) => {
                    preview.at = at;
                    preview.settled = Duration::ZERO;
                }
                None => {
                    playing.preview = Some(trickplay::Preview {
                        at,
                        settled: Duration::ZERO,
                        frame: None,
                    });
                }
            }
            Task::none()
        }
        Action::Unhovered => {
            playing.preview = None;
            Task::none()
        }
        Action::Settled => tile_wanted(signed),
        Action::OpenMenu(menu) => {
            playing.menu = Some(menu);
            Task::none()
        }
        Action::CloseMenu => {
            playing.menu = None;
            Task::none()
        }
        Action::ToggleShuffle => {
            let shuffled = playing.queue.shuffled();
            playing.queue.set_shuffle(!shuffled);
            Task::none()
        }
        Action::CycleRepeat => {
            let next = playing.queue.repeat().cycled();
            act(signed, Action::SetRepeat(next))
        }
        Action::SetRepeat(repeat) => {
            playing.queue.set_repeat(repeat);
            let progress = playing.progress(signed.device);
            Task::perform(control::progress(progress), Message::Reported)
        }
        Action::RemoveQueued(position) => {
            playing.queue.remove(position);
            Task::none()
        }
        Action::Next => {
            if playing.queue.advance().is_none() {
                return leave(signed);
            }
            let start = playing
                .queue
                .current()
                .map(resume_ticks)
                .unwrap_or_default();
            play_current(signed, start)
        }
        Action::Previous => {
            if playing.queue.back().is_none() {
                return act(signed, Action::Seek(Duration::ZERO));
            }
            let start = playing
                .queue
                .current()
                .map(resume_ticks)
                .unwrap_or_default();
            play_current(signed, start)
        }
        Action::ToggleFullscreen => {
            playing.fullscreen = !playing.fullscreen;
            playing.element.ask(if playing.fullscreen {
                &Asked::Fullscreen
            } else {
                &Asked::Windowed
            });
            Task::none()
        }
        Action::Leave => leave(signed),
        Action::ToggleDisplay => {
            playing.idle = if playing.idle >= crate::theme::IDLE_HIDE {
                Duration::ZERO
            } else {
                crate::theme::IDLE_HIDE
            };
            playing.element.ask(&playing.idling());
            Task::none()
        }
    }
}

/// Applies one control command from another Jellyfin client.
/// A `Play` starts playback here and takes the playback session, leaving
/// remote mode first when this tab held it; a `PlayNext` and a `PlayLast` add
/// to what plays here instead.
/// While this tab holds group membership, a `Play` sets the group's queue and a
/// playstate command issues the group's equivalent; a verb with no group
/// equivalent takes effect locally, unchanged.
/// A command this tab cannot honour right now changes nothing and shows
/// nothing; only `Notify` renders.
pub fn controlled(signed: &mut Signed, control: Control) -> Task<Message> {
    if let Control::Notify(notice) = control {
        signed.message = Some((notice, Duration::ZERO));
        return Task::none();
    }

    if let Some(joined) = signed.group.as_ref()
        && let Some(verb) = group::controlled(joined, &control)
    {
        crate::live::send(&jellium_protocol::Report::Group(verb));
        return Task::none();
    }

    if let Control::Play {
        items,
        mode,
        start_index,
        start_ticks,
        media_source,
        audio_stream,
        subtitle_stream,
    } = control
    {
        let leaving = remote::leave(signed);
        let api = signed.api.clone();
        let asked = Commanded {
            items,
            mode,
            start_index,
            start_ticks,
            selection: Selection {
                media_source,
                audio_stream,
                subtitle_stream,
            },
        };
        return Task::batch([
            leaving,
            Task::perform(commanded(api, asked), Message::Resolved),
        ]);
    }

    match control {
        Control::GoHome => return Task::done(Message::Navigated(Route::Home)),
        Control::GoToSearch => {
            return Task::done(Message::Navigated(Route::Search {
                term: String::new(),
                listing: Box::default(),
            }));
        }
        Control::Show { item } => {
            return Task::done(Message::Navigated(Route::Detail { id: item }));
        }
        _ => {}
    }

    let Some(action) = honoured(signed, control) else {
        return Task::none();
    };
    act(signed, action)
}

/// The local control a command has here, or nothing when this tab cannot
/// honour it right now.
fn honoured(signed: &Signed, control: Control) -> Option<Action> {
    let playing = signed.playing.as_ref()?;
    match control {
        Control::Stop => Some(Action::Leave),
        Control::PlayPause => Some(Action::TogglePlay),
        Control::Pause => playing.paused.not().then_some(Action::TogglePlay),
        Control::Unpause => playing.paused.then_some(Action::TogglePlay),
        Control::NextTrack => Some(Action::Next),
        Control::PreviousTrack => Some(Action::Previous),
        Control::Seek { position_ticks } => Some(Action::Seek(span(position_ticks))),
        Control::Rewind => Some(Action::SkipBack),
        Control::FastForward => Some(Action::SkipForward),
        Control::VolumeUp => Some(Action::SetVolume(
            (signed.device.volume + VOLUME_STEP).min(1.0),
        )),
        Control::VolumeDown => Some(Action::SetVolume(
            (signed.device.volume - VOLUME_STEP).max(0.0),
        )),
        Control::SetVolume { level } => {
            Some(Action::SetVolume((level as f32 / 100.0).clamp(0.0, 1.0)))
        }
        Control::Mute => signed.device.muted.not().then_some(Action::ToggleMute),
        Control::Unmute => signed.device.muted.then_some(Action::ToggleMute),
        Control::ToggleMute => Some(Action::ToggleMute),
        Control::SetAudioStream { index } => Some(Action::SelectAudio(index)),
        Control::SetSubtitleStream { index } => Some(Action::SelectSubtitle(index)),
        Control::SetMediaSource { id } => Some(Action::SelectVersion(id)),
        Control::SetMaxBitrate { bits_per_second } => {
            Some(Action::SelectQuality(match bits_per_second {
                Some(bits_per_second) => Quality::Limit { bits_per_second },
                None => Quality::Auto,
            }))
        }
        Control::SetRepeat { repeat } => Some(Action::SetRepeat(repeat)),
        Control::SetShuffle { shuffled } => {
            (playing.queue.shuffled() != shuffled).then_some(Action::ToggleShuffle)
        }
        Control::ToggleFullscreen => Some(Action::ToggleFullscreen),
        Control::ToggleDisplay => Some(Action::ToggleDisplay),
        Control::ChannelUp => playing.live.is_some().then_some(Action::Next),
        Control::ChannelDown => playing.live.is_some().then_some(Action::Previous),
        Control::Guide => None,
        Control::Play { .. }
        | Control::GoHome
        | Control::GoToSearch
        | Control::Show { .. }
        | Control::Notify(_) => None,
    }
}

pub fn leave(signed: &mut Signed) -> Task<Message> {
    signed.pending = None;
    let Some(playing) = signed.playing.take() else {
        return Task::none();
    };
    let stopped = playing.stopped();
    if playing.fullscreen {
        playing.element.ask(&Asked::Windowed);
    }
    drop(playing);

    let route = signed.history.last().cloned().unwrap_or(Route::Home);
    if matches!(route, Route::Queue) {
        signed.history.pop();
    }
    let route = signed.history.last().cloned().unwrap_or(Route::Home);
    signed.view = crate::app::staged(&route, signed.session.live_tv);
    Task::batch([
        Task::perform(
            async move { control::stopped(stopped).await.map(|()| Standing::Current) },
            Message::Reported,
        ),
        crate::app::load(signed, &route),
    ])
}

/// A `Superseded` standing leaves the player showing that another tab took the
/// session.
/// A `Lapsed` standing plays the current item again from the position it last
/// reported, once; a second lapse leaves the player showing that the session
/// timed out.
pub fn reported(signed: &mut Signed, standing: Standing) -> Task<Message> {
    match standing {
        Standing::Current => Task::none(),
        Standing::Superseded => {
            signed.playing = None;
            signed.pending = None;
            crate::failure::raise(crate::error::refused(&PlaybackRefused::Superseded));
            Task::none()
        }
        Standing::Released => {
            signed.playing = None;
            signed.pending = None;
            crate::failure::raise(crate::error::refused(&PlaybackRefused::TunerReleased));
            Task::none()
        }
        Standing::Lapsed => {
            let Some(playing) = signed.playing.as_mut() else {
                return Task::none();
            };
            if playing.resumed {
                signed.playing = None;
                signed.pending = None;
                crate::failure::raise(crate::error::refused(&PlaybackRefused::Lapsed));
                return Task::none();
            }
            playing.resumed = true;
            let position = to_ticks(playing.position);
            play_current(signed, position)
        }
    }
}

/// Space and `k` toggle play, the arrows seek and change volume, `f` toggles
/// fullscreen, `m` toggles mute, `n` and `p` move through the queue, and
/// Escape leaves the player; every one is an entry of `binding::BINDINGS` and
/// no key outside the table is honoured.
pub fn keys() -> Subscription<Action> {
    keyboard::listen().with(()).filter_map(|((), event)| {
        let keyboard::Event::KeyPressed { key, .. } = event else {
            return None;
        };
        binding::bound(&key.as_ref(), crate::prefs::Device::load().volume)
    })
}

/// Mouse movement, key presses and touches that keep the display visible.
pub fn stirs() -> Subscription<Action> {
    event::listen_with(|event, _status, _window| match event {
        iced::Event::Mouse(mouse::Event::CursorMoved { .. } | mouse::Event::ButtonPressed(_))
        | iced::Event::Keyboard(keyboard::Event::KeyPressed { .. })
        | iced::Event::Touch(
            touch::Event::FingerMoved { .. } | touch::Event::FingerPressed { .. },
        ) => Some(Action::Stirred),
        _ => None,
    })
}

/// A 250 ms tick that ages the idle timer; it drives no reporting.
pub fn ticks() -> Subscription<()> {
    iced::time::every(theme::TICK).map(|_| ())
}

/// Ages the idle timer and hides the display and the cursor once it passes
/// `theme::IDLE_HIDE`.
/// Ages the scrub preview's settle timer too, and answers true the once it
/// crosses `trickplay::SETTLE`, which is when a preview is asked for.
pub fn tick(signed: &mut Signed) -> bool {
    let Some(playing) = signed.playing.as_mut() else {
        return false;
    };
    let mut settled = false;
    if let Some(preview) = playing.preview.as_mut()
        && preview.settled < trickplay::SETTLE
    {
        preview.settled += theme::TICK;
        settled = preview.settled >= trickplay::SETTLE;
    }
    if playing.paused || playing.menu.is_some() {
        playing.idle = Duration::ZERO;
    } else {
        playing.idle += theme::TICK;
    }
    playing.element.ask(&playing.idling());
    settled
}

/// The tile the preview needs, asked for once the pointer has settled.
/// An item whose media source has no trickplay asks for nothing, and the
/// preview falls back to the chapter image the display already holds.
fn tile_wanted(signed: &mut Signed) -> Task<Message> {
    let Some(playing) = signed.playing.as_ref() else {
        return Task::none();
    };
    let Some(preview) = playing.preview.as_ref() else {
        return Task::none();
    };
    let Some(item) = playing.item.id else {
        return Task::none();
    };
    let source = playing.plan.media_source.clone();
    let Some(described) = playing
        .trickplay
        .width_for(&source, crate::theme::CARD_WIDTH as u16)
    else {
        return Task::none();
    };
    let Some(tile) = trickplay::tile(described, preview.at) else {
        return Task::none();
    };

    let api = signed.api.clone();
    let width = described.width;
    Task::perform(
        async move { api.trickplay_tile(item, &source, width, tile.index).await },
        move |bytes| Message::TileLoaded(tile, bytes),
    )
}
