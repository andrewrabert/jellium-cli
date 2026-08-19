use std::sync::Arc;

use axum::extract::State;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::Response;
use futures_util::{SinkExt, StreamExt};
use jellium_protocol::{
    Displaced, Drive, Event, Feed, GroupEnded, GroupVerb, LiveRefusal, RemoteEnded, Report,
    Scheduled, Target,
};
use jellyfin_api::types::{SendCommand, SendCommandType, SessionInfoDto};

mod clock;
mod group;
mod message;
mod remote;
mod socket;
mod tabs;
pub mod verbs;

pub use message::activity_entry;
pub use tabs::TabId;

use super::AppState;

/// The connected tabs, the upstream socket, remote mode, group membership and
/// the upstream clock.
pub struct Hub {
    tabs: tabs::Tabs,
    remote: remote::Remote,
    group: group::Membership,
    clock: clock::Clock,
    /// Running while a picker or the SyncPlay screen is open somewhere.
    listing: tokio::sync::Mutex<Option<group::Listing>>,
    /// Running while at least one tab is connected.
    socket: tokio::sync::Mutex<Option<socket::Link>>,
    /// Every change to the pair of tab count and socket is taken under this.
    transition: tokio::sync::Mutex<()>,
}

impl Hub {
    pub fn new() -> Hub {
        Hub {
            tabs: tabs::Tabs::new(),
            remote: remote::Remote::new(),
            group: group::Membership::new(),
            clock: clock::Clock::new(),
            listing: tokio::sync::Mutex::new(None),
            socket: tokio::sync::Mutex::new(None),
            transition: tokio::sync::Mutex::new(()),
        }
    }

    /// Every tab receives it.
    pub async fn broadcast(&self, event: Event) {
        self.tabs.broadcast(&event).await;
    }

    /// The tab holding group membership receives it, then the tab holding the
    /// playback session, then the most recently connected tab; nothing happens
    /// when no tab is connected.
    /// A `Play` ends remote mode and tells the tab that held it; membership
    /// stands, because the destination translates the command into the group's
    /// equivalent.
    pub async fn command(&self, state: &Arc<AppState>, control: jellium_protocol::Control) {
        let held = state.playback.held_session().await;
        let member = self.group.holder().await;
        let destination = match member {
            Some(tab) => Some(tab),
            None => self.tabs.destination(held.as_deref()).await,
        };
        let Some(tab) = destination else {
            return;
        };
        if matches!(control, jellium_protocol::Control::Play { .. })
            && let Some(bound) = self.remote.end().await
        {
            self.tabs
                .send(
                    bound.tab,
                    Event::RemoteEnded {
                        cause: RemoteEnded::Controlled,
                    },
                )
                .await;
        }
        self.tabs.send(tab, Event::Control(control)).await;
    }

    /// The tab holding `play_session` is told it was displaced.
    pub async fn displaced(&self, play_session: &str) {
        if let Some(tab) = self.tabs.holding(play_session).await {
            self.tabs
                .send(
                    tab,
                    Event::Displaced {
                        cause: Displaced::Playback,
                    },
                )
                .await;
        }
    }

    /// Applies a session listing to both projections: the targets a picker
    /// shows and the sessions dashboard home shows.
    /// It records every session id as addressable, pushes the targets to every
    /// tab holding `Feed::Targets`, pushes the server's sessions to every tab
    /// holding `Feed::Sessions`, and ends remote mode when its target is gone
    /// or no longer controllable.
    pub async fn listed(&self, state: &Arc<AppState>, sessions: &[SessionInfoDto]) {
        let Some(upstream) = state.session.signed().await else {
            return;
        };
        let Some(identity) = state.identity.held().await else {
            return;
        };
        let targets = remote::read(sessions, upstream.user_id(), &identity);
        drop(upstream);
        self.pushed(targets).await;
        if self.tabs.watched(Feed::Sessions).await {
            let sessions = crate::web::upstream::server_sessions(sessions, &identity);
            self.fed(Feed::Sessions, Event::Sessions { sessions }).await;
        }
    }

    /// Applies the tasks one `ScheduledTasksInfo` carried.
    pub async fn tasked(&self, tasks: Vec<jellium_protocol::TaskState>) {
        self.fed(Feed::Tasks, Event::Tasks { tasks }).await;
    }

    /// Applies a policy change for the signed-in user, which is what removes
    /// the dashboard from a demoted user and the preference screens from one
    /// whose preference access was taken.
    pub async fn user_updated(&self, administrator: bool, preference_access: bool) {
        self.broadcast(Event::UserUpdated {
            administrator,
            preference_access,
        })
        .await;
    }

    /// Delivers `event` to the tabs holding `feed`, and to no other.
    pub async fn fed(&self, feed: Feed, event: Event) {
        for tab in self.tabs.watchers(feed).await {
            self.tabs.send(tab, event.clone()).await;
        }
    }

    /// Starts the upstream subscription `feed` needs when the first tab opens a
    /// screen consuming it, and stops it when the last such screen closes.
    /// `Feed::Targets` and `Feed::Sessions` share one upstream `Sessions`
    /// subscription, which runs while either is held.
    /// `Feed::Refresh` and `Feed::Packages` need none, because the Jellyfin
    /// server pushes them unasked; holding them only decides which tabs the
    /// events reach.
    async fn feeding(&self, state: &Arc<AppState>, feed: Feed) {
        match feed {
            Feed::Groups => {
                let watching = self.tabs.watched(Feed::Groups).await;
                self.listing(state, watching).await;
            }
            Feed::Targets | Feed::Sessions => {
                let watching = self.tabs.watched(Feed::Targets).await
                    || self.tabs.watched(Feed::Sessions).await;
                if let Some(link) = self.socket.lock().await.as_ref() {
                    link.watch(Feed::Sessions, watching);
                }
            }
            Feed::Tasks | Feed::Activity => {
                let watching = self.tabs.watched(feed).await;
                if let Some(link) = self.socket.lock().await.as_ref() {
                    link.watch(feed, watching);
                }
            }
            Feed::Refresh | Feed::Packages => {}
        }
    }

    /// Records `targets` as addressable, tells every watching tab, and ends a
    /// mode whose target is gone.
    async fn pushed(&self, targets: Vec<Target>) {
        self.remote.listed(&targets).await;
        if let Some(tab) = self.remote.checked(&targets).await {
            self.tabs
                .send(
                    tab,
                    Event::RemoteEnded {
                        cause: RemoteEnded::TargetGone,
                    },
                )
                .await;
        }
        for tab in self.tabs.watchers(Feed::Targets).await {
            self.tabs
                .send(
                    tab,
                    Event::Targets {
                        targets: targets.clone(),
                    },
                )
                .await;
        }
    }

    /// Applies one group update: the held group and queue take it, every tab is
    /// told, and an update saying this session is not in the group, that the
    /// group does not exist, or that library access was denied ends membership
    /// naming that cause.
    pub async fn grouped(&self, state: &Arc<AppState>, update: group::Update) {
        match update {
            group::Update::Joined(group) => {
                let tab = self
                    .group
                    .expecting()
                    .await
                    .or(self.tabs.destination(None).await);
                let Some(tab) = tab else {
                    return;
                };
                self.entered(state, tab, group).await;
            }
            group::Update::Left => self.ungrouped(GroupEnded::Removed).await,
            group::Update::UserJoined(participant) => {
                if let Some(group) = self.group.welcomed(participant).await {
                    self.told(group).await;
                }
            }
            group::Update::UserLeft(participant) => {
                if let Some(group) = self.group.parted(&participant).await {
                    self.told(group).await;
                }
            }
            group::Update::State(standing) => {
                if let Some(group) = self.group.standing(standing).await {
                    self.told(group).await;
                }
            }
            group::Update::Queue(queue) => {
                self.group.queued(queue.clone()).await;
                self.tabs.broadcast(&Event::GroupQueue(queue)).await;
            }
            group::Update::NotInGroup => self.ungrouped(GroupEnded::NotInGroup).await,
            group::Update::NoSuchGroup => self.ungrouped(GroupEnded::NoSuchGroup).await,
            group::Update::LibraryDenied => self.ungrouped(GroupEnded::LibraryDenied).await,
        }
    }

    /// Binds `tab` to `group`, ends remote mode, starts the clock, and tells
    /// every tab.
    async fn entered(&self, state: &Arc<AppState>, tab: TabId, group: jellium_protocol::Group) {
        self.unbound().await;
        self.group.joined(tab, group.clone()).await;
        self.clock.start(state.clone()).await;
        self.told(group).await;
    }

    /// Tells every tab the group stands as `group`, with `member` true for the
    /// tab holding it and false for every other.
    async fn told(&self, group: jellium_protocol::Group) {
        let holder = self.group.holder().await;
        for tab in self.tabs.every().await {
            self.tabs
                .send(
                    tab,
                    Event::Joined {
                        group: group.clone(),
                        member: holder == Some(tab),
                    },
                )
                .await;
        }
    }

    /// Ends remote mode and tells the tab that held it, which is what entering
    /// a group does.
    async fn unbound(&self) {
        if let Some(bound) = self.remote.end().await {
            self.tabs
                .send(
                    bound.tab,
                    Event::RemoteEnded {
                        cause: RemoteEnded::Grouped,
                    },
                )
                .await;
        }
    }

    /// Ends membership, stops the clock, and names the cause to every tab.
    async fn ungrouped(&self, cause: GroupEnded) {
        self.group.end().await;
        self.clock.stop().await;
        self.tabs.broadcast(&Event::GroupEnded { cause }).await;
    }

    /// Converts a scheduled command's instant onto this machine's clock and
    /// sends it to the tab holding membership.
    /// A command carrying no instant is sent due now.
    pub async fn scheduled(&self, command: &SendCommand) {
        let Some(tab) = self.group.holder().await else {
            return;
        };
        let verb = match command.command {
            Some(SendCommandType::Unpause) => jellium_protocol::GroupCommand::Unpause,
            Some(SendCommandType::Pause) => jellium_protocol::GroupCommand::Pause,
            Some(SendCommandType::Stop) => jellium_protocol::GroupCommand::Stop,
            Some(SendCommandType::Seek) => jellium_protocol::GroupCommand::Seek,
            None => return,
        };
        let at = match command.when {
            Some(when) => self.clock.locally(when).await,
            None => chrono::Utc::now().timestamp_millis(),
        };
        self.tabs
            .send(
                tab,
                Event::Scheduled(Scheduled {
                    command: verb,
                    playlist_item: command.playlist_item_id,
                    position_ticks: command.position_ticks.unwrap_or(0),
                    at,
                }),
            )
            .await;
    }

    /// Leaves the group upstream when the grace window has run out with no tab
    /// reclaiming membership, which is what a reloaded page leaves behind.
    pub async fn swept(&self, state: &Arc<AppState>) {
        if !self.group.abandoned().await {
            return;
        }
        self.left(state, GroupEnded::Left).await;
    }

    /// Leaves the group upstream, ends membership, stops the clock, and names
    /// `cause` to every tab.
    async fn left(&self, state: &Arc<AppState>, cause: GroupEnded) {
        if let Some(upstream) = state.session.signed().await {
            let _ = group::leave(&upstream).await;
        }
        self.ungrouped(cause).await;
    }

    /// Starts the listing loop when a tab opens a group picker, and stops it
    /// when the last one closes.
    async fn listing(&self, state: &Arc<AppState>, watching: bool) {
        let mut running = self.listing.lock().await;
        match (watching, running.take()) {
            (true, Some(listing)) => *running = Some(listing),
            (true, None) => *running = Some(group::Listing::start(state.clone())),
            (false, Some(listing)) => listing.stop(),
            (false, None) => {}
        }
    }

    /// Closes the upstream socket and opens a new one when tabs remain, which
    /// is how a sign-in or a revoke moves the socket to the session that
    /// replaced it.
    pub async fn rebound(&self, state: &Arc<AppState>) {
        let _held = self.transition.lock().await;
        if let Some(bound) = self.remote.forget().await {
            self.tabs
                .send(
                    bound.tab,
                    Event::RemoteEnded {
                        cause: RemoteEnded::TargetGone,
                    },
                )
                .await;
        }
        let mut socket = self.socket.lock().await;
        if let Some(link) = socket.take() {
            link.close(state).await;
        }
        if self.tabs.destination(None).await.is_some() {
            let link = socket::Link::open(state.clone());
            for feed in Feed::ALL {
                if feed.sessions() {
                    continue;
                }
                link.watch(feed, self.tabs.watched(feed).await);
            }
            let sessions =
                self.tabs.watched(Feed::Targets).await || self.tabs.watched(Feed::Sessions).await;
            link.watch(Feed::Sessions, sessions);
            *socket = Some(link);
        }
    }

    /// Ends every tab, the upstream socket, remote mode and the clock, leaves
    /// the group upstream, and declares no media control.
    pub async fn shutdown(&self, state: &Arc<AppState>) {
        let _held = self.transition.lock().await;
        self.remote.forget().await;
        if let Some(listing) = self.listing.lock().await.take() {
            listing.stop();
        }
        if self.group.grouped().await {
            self.left(state, GroupEnded::Left).await;
        }
        let mut socket = self.socket.lock().await;
        if let Some(link) = socket.take() {
            link.close(state).await;
        }
    }

    #[cfg(test)]
    pub(crate) fn tabs_for_test(&self) -> &tabs::Tabs {
        &self.tabs
    }
}

/// Upgrades the request to the event socket, opening the upstream socket when
/// this is the first tab and closing it when the last tab goes away.
pub async fn events(state: State<Arc<AppState>>, upgrade: WebSocketUpgrade) -> Response {
    let state = state.0;
    upgrade.on_upgrade(move |socket| serve(state, socket))
}

async fn serve(state: Arc<AppState>, socket: WebSocket) {
    let (tab, mut sending, count) = state.live.tabs.add().await;
    if count == 1 {
        let _held = state.live.transition.lock().await;
        let mut link = state.live.socket.lock().await;
        if link.is_none() {
            *link = Some(socket::Link::open(state.clone()));
        }
    }

    let (mut down, mut up) = socket.split();
    let sender = tokio::spawn(async move {
        while let Some(event) = sending.recv().await {
            let Ok(frame) = serde_json::to_string(&event) else {
                continue;
            };
            if down.send(Message::Text(frame.into())).await.is_err() {
                return;
            }
        }
    });

    while let Some(Ok(received)) = up.next().await {
        let Message::Text(frame) = received else {
            continue;
        };
        let Ok(report) = serde_json::from_str::<Report>(&frame) else {
            continue;
        };
        reported(&state, tab, report).await;
    }

    sender.abort();
    state.live.remote.leave(tab).await;
    state.live.group.orphaned(tab).await;
    let remaining = state.live.tabs.remove(tab).await;
    for feed in Feed::ALL {
        state.live.feeding(&state, feed).await;
    }
    if remaining == 0 {
        let _held = state.live.transition.lock().await;
        let mut link = state.live.socket.lock().await;
        if state.live.tabs.destination(None).await.is_none()
            && let Some(link) = link.take()
        {
            link.close(&state).await;
        }
    }
}

/// Leaves the group upstream and ends membership, which is what the page-hide
/// beacon posts to when the page reloads or its last tab closes.
pub async fn leaving(state: State<Arc<AppState>>) -> Response {
    use axum::response::IntoResponse;
    state.0.live.left(&state.0, GroupEnded::Left).await;
    axum::http::StatusCode::NO_CONTENT.into_response()
}

async fn refuse(state: &Arc<AppState>, tab: TabId, refusal: LiveRefusal) {
    state.live.tabs.send(tab, Event::Refused { refusal }).await;
}

/// The first targets come from one request, so a picker fills without waiting
/// for a push.
async fn first_targets(state: &Arc<AppState>, tab: TabId) {
    let Some(upstream) = state.session.signed().await else {
        return;
    };
    let Some(identity) = state.identity.held().await else {
        return;
    };
    let Ok(targets) = remote::targets(&upstream, &identity).await else {
        return;
    };
    state.live.remote.listed(&targets).await;
    state.live.tabs.send(tab, Event::Targets { targets }).await;
}

/// How many activity entries a screen's first contents carry.
const FIRST_ACTIVITY: i32 = 200;

/// The contents `feed` would push, fetched with one request, so a screen fills
/// without waiting for a push.
async fn first(state: &Arc<AppState>, tab: TabId, feed: Feed) {
    match feed {
        Feed::Targets => first_targets(state, tab).await,
        Feed::Groups => first_groups(state, tab).await,
        Feed::Sessions => {
            let Some(upstream) = state.session.signed().await else {
                return;
            };
            let Some(identity) = state.identity.held().await else {
                return;
            };
            if let Ok(sessions) = upstream.sessions(&identity).await {
                state
                    .live
                    .tabs
                    .send(tab, Event::Sessions { sessions })
                    .await;
            }
        }
        Feed::Tasks => {
            let Some(upstream) = state.session.signed().await else {
                return;
            };
            if let Ok(tasks) = upstream.tasks().await {
                state.live.tabs.send(tab, Event::Tasks { tasks }).await;
            }
        }
        Feed::Activity => {
            let Some(upstream) = state.session.signed().await else {
                return;
            };
            if let Ok(entries) = upstream.activity(FIRST_ACTIVITY).await {
                state.live.tabs.send(tab, Event::Activity { entries }).await;
            }
        }
        Feed::Refresh | Feed::Packages => {}
    }
}

/// The first groups come from one request, so a picker fills without waiting
/// for a push.
async fn first_groups(state: &Arc<AppState>, tab: TabId) {
    let Some(upstream) = state.session.signed().await else {
        return;
    };
    let Ok(groups) = group::groups(&upstream).await else {
        return;
    };
    state.live.tabs.send(tab, Event::Groups { groups }).await;
}

/// Acts on one report from `tab`.
/// A report `--read-only` forecloses is refused as `ReadOnly` to that tab
/// alone, before the Jellyfin server is reached.
/// A `TakeRemote` naming a target the local server has not seen, and a `Drive`
/// from a tab holding no remote mode, are refused to that tab alone.
/// A `TakeGroup`, a group verb and a `Clock` from a tab holding no membership
/// are refused as `NotGrouped` to that tab alone.
/// A `CreateGroup` or `JoinGroup` ends remote mode, telling the tab that held
/// it; a `TakeRemote` leaves the group, telling the tab that held that.
/// A `Playing` from a tab that is not the member moves membership to it,
/// because the playback session and the group are held together.
async fn reported(state: &Arc<AppState>, tab: TabId, report: Report) {
    if state.read_only && !report.read_only() {
        return refuse(state, tab, LiveRefusal::ReadOnly).await;
    }
    match report {
        Report::Playing { play_session } => {
            state.live.tabs.playing(tab, Some(play_session)).await;
            bound_group(state, tab).await;
        }
        Report::Watch { feed } => {
            state.live.tabs.watch(tab, feed, true).await;
            state.live.feeding(state, feed).await;
            first(state, tab, feed).await;
        }
        Report::Drop { feed } => {
            state.live.tabs.watch(tab, feed, false).await;
            state.live.feeding(state, feed).await;
        }
        Report::TakeRemote { target } => match state.live.remote.take(tab, &target).await {
            Ok(taken) => {
                if state.live.group.grouped().await {
                    state.live.left(state, GroupEnded::Remote).await;
                }
                if let Some(taken) = taken {
                    state
                        .live
                        .tabs
                        .send(
                            taken,
                            Event::RemoteEnded {
                                cause: RemoteEnded::Taken,
                            },
                        )
                        .await;
                }
            }
            Err(refusal) => refuse(state, tab, refusal).await,
        },
        Report::LeaveRemote => state.live.remote.leave(tab).await,
        Report::Drive(drive) => drove(state, tab, drive).await,
        Report::CreateGroup { name } => {
            let Some(upstream) = state.session.signed().await else {
                return refuse(state, tab, LiveRefusal::GroupRefused).await;
            };
            match group::create(&upstream, &name).await {
                Ok(group) => state.live.entered(state, tab, group).await,
                Err(refusal) => refuse(state, tab, refusal).await,
            }
        }
        Report::JoinGroup { group: wanted } => {
            let Some(upstream) = state.session.signed().await else {
                return refuse(state, tab, LiveRefusal::GroupRefused).await;
            };
            match group::join(&upstream, wanted).await {
                Ok(()) => {
                    state.live.unbound().await;
                    state.live.group.asking(tab).await;
                }
                Err(refusal) => refuse(state, tab, refusal).await,
            }
        }
        Report::TakeGroup => took_group(state, tab).await,
        Report::LeaveGroup => {
            if state.live.group.member(tab).await.is_ok() {
                state.live.left(state, GroupEnded::Left).await;
            }
        }
        Report::Group(verb) => grouped_verb(state, tab, verb).await,
        Report::Clock { sent, round_trip } => clocked(state, tab, sent, round_trip).await,
    }
}

/// Binds `tab` as the member, hands it the group's queue, tells every tab which
/// one holds the group, and tells the tab membership was taken from once.
/// A call while this installation is in no group is refused as `NotGrouped` to
/// `tab`.
async fn took_group(state: &Arc<AppState>, tab: TabId) {
    if !bind_group(state, tab).await {
        refuse(state, tab, LiveRefusal::NotGrouped).await;
    }
}

/// Binds `tab` as the member the way a playback start does, and changes
/// nothing while this installation is in no group.
async fn bound_group(state: &Arc<AppState>, tab: TabId) {
    bind_group(state, tab).await;
}

/// Binds `tab` as the member, and answers whether this installation is in a
/// group at all.
async fn bind_group(state: &Arc<AppState>, tab: TabId) -> bool {
    let Some((group, queue, taken)) = state.live.group.bound(tab).await else {
        return false;
    };
    if let Some(taken) = taken {
        state
            .live
            .tabs
            .send(
                taken,
                Event::Displaced {
                    cause: Displaced::Group,
                },
            )
            .await;
    }
    state.live.told(group).await;
    state.live.tabs.send(tab, Event::GroupQueue(queue)).await;
    true
}

async fn grouped_verb(state: &Arc<AppState>, tab: TabId, verb: GroupVerb) {
    if state.live.group.member(tab).await.is_err() {
        return refuse(state, tab, LiveRefusal::NotGrouped).await;
    }
    let Some(upstream) = state.session.signed().await else {
        return refuse(state, tab, LiveRefusal::GroupRefused).await;
    };
    let stamped = state.live.clock.upstream_now().await;
    if let Err(refusal) = group::issue(&upstream, &verb, stamped).await {
        refuse(state, tab, refusal).await;
    }
}

/// Answers one clock exchange and reports the composed ping to the group.
async fn clocked(state: &Arc<AppState>, tab: TabId, sent: i64, round_trip: i64) {
    if state.live.group.member(tab).await.is_err() {
        return refuse(state, tab, LiveRefusal::NotGrouped).await;
    }
    let received = chrono::Utc::now().timestamp_millis();
    state
        .live
        .tabs
        .send(
            tab,
            Event::Clock(jellium_protocol::sync::Exchange {
                sent,
                received,
                answered: chrono::Utc::now().timestamp_millis(),
                returned: 0,
            }),
        )
        .await;
    let upstream_trip = state
        .live
        .clock
        .estimate()
        .await
        .map_or(0, |held| held.round_trip);
    if let Some(upstream) = state.session.signed().await {
        let _ = group::ping(&upstream, upstream_trip + round_trip).await;
    }
}

async fn drove(state: &Arc<AppState>, tab: TabId, drive: Drive) {
    let target = match state.live.remote.driving(tab).await {
        Ok(target) => target,
        Err(refusal) => return refuse(state, tab, refusal).await,
    };
    let Some(upstream) = state.session.signed().await else {
        return refuse(state, tab, LiveRefusal::TargetRefused).await;
    };
    if let Err(refusal) = remote::drive(&upstream, &target, &drive).await {
        refuse(state, tab, refusal).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jellium_protocol::Control;

    async fn hub_with(tabs: usize) -> (Hub, Vec<tokio::sync::mpsc::UnboundedReceiver<Event>>) {
        let hub = Hub::new();
        let mut queues = Vec::new();
        for _ in 0..tabs {
            let (_, arriving, _) = hub.tabs.add().await;
            queues.push(arriving);
        }
        (hub, queues)
    }

    fn scratch(name: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir()
            .join("jellium-cli-hub-tests")
            .join(name);
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("scratch directory");
        path.join("session.env")
    }

    #[tokio::test]
    async fn the_upstream_socket_opens_with_the_first_tab_and_closes_with_the_last() {
        let server = crate::web::upstream::answering(204).await;
        let state = Arc::new(AppState::stub(scratch("socket-span")));
        state
            .session
            .install(crate::web::upstream::Upstream::stub(&server.base))
            .await;

        // the first tab brings the one upstream socket up
        let (first, _first_events, count) = state.live.tabs.add().await;
        assert_eq!(count, 1);
        *state.live.socket.lock().await = Some(socket::Link::open(state.clone()));
        server.opened(1).await;

        // a second tab opens no second socket
        let (second, _second_events, count) = state.live.tabs.add().await;
        assert_eq!(count, 2);
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        assert_eq!(server.sockets.load(std::sync::atomic::Ordering::SeqCst), 1);

        // the last tab leaving takes it down
        assert_eq!(state.live.tabs.remove(first).await, 1);
        assert_eq!(state.live.tabs.remove(second).await, 0);
        state.live.shutdown(&state).await;
        assert!(state.live.socket.lock().await.is_none());
    }

    #[tokio::test]
    async fn a_live_refresh_reaches_every_connected_tab() {
        let (hub, queues) = hub_with(8).await;
        hub.broadcast(Event::Marked { items: Vec::new() }).await;
        for mut arriving in queues {
            assert_eq!(arriving.try_recv(), Ok(Event::Marked { items: Vec::new() }));
        }
    }

    #[tokio::test]
    async fn a_control_command_reaches_one_tab_and_no_other() {
        let hub = Hub::new();
        let mut queues = Vec::new();
        for _ in 0..8 {
            let (_, arriving, _) = hub.tabs.add().await;
            queues.push(arriving);
        }
        let destination = hub.tabs.destination(None).await.expect("a destination");
        hub.tabs
            .send(destination, Event::Control(Control::PlayPause))
            .await;

        let mut reached = 0;
        for mut arriving in queues {
            if arriving.try_recv() == Ok(Event::Control(Control::PlayPause)) {
                reached += 1;
            }
        }
        assert_eq!(reached, 1);
    }

    #[tokio::test]
    async fn a_displaced_tab_is_told_over_the_socket() {
        let hub = Hub::new();
        let (first, mut to_first, _) = hub.tabs.add().await;
        let (_, mut to_second, _) = hub.tabs.add().await;
        hub.tabs.playing(first, Some("session".to_owned())).await;

        hub.displaced("session").await;

        assert_eq!(
            to_first.try_recv(),
            Ok(Event::Displaced {
                cause: Displaced::Playback
            })
        );
        assert!(to_second.try_recv().is_err());
    }

    #[tokio::test]
    async fn a_target_listing_reaches_only_the_watching_tabs() {
        let hub = Hub::new();
        let (watcher, mut to_watcher, _) = hub.tabs.add().await;
        let (_, mut to_other, _) = hub.tabs.add().await;
        hub.tabs.watch(watcher, Feed::Targets, true).await;

        let targets = vec![Target {
            session: "target".to_owned(),
            device_name: "Device".to_owned(),
            client_name: "Client".to_owned(),
            now_playing: None,
        }];
        hub.pushed(targets.clone()).await;

        assert_eq!(to_watcher.try_recv(), Ok(Event::Targets { targets }));
        assert!(to_other.try_recv().is_err());
    }

    async fn bound_hub(
        state: &Arc<AppState>,
    ) -> (TabId, tokio::sync::mpsc::UnboundedReceiver<Event>) {
        let (tab, arriving, _) = state.live.tabs.add().await;
        let target = Target {
            session: "target".to_owned(),
            device_name: String::new(),
            client_name: String::new(),
            now_playing: None,
        };
        state.live.pushed(vec![target]).await;
        state
            .live
            .remote
            .take(tab, "target")
            .await
            .expect("the mode is taken");
        (tab, arriving)
    }

    #[tokio::test]
    async fn an_inbound_play_ends_remote_mode() {
        let state = Arc::new(AppState::stub(scratch("inbound-play")));
        let (tab, mut arriving) = bound_hub(&state).await;

        state
            .live
            .command(
                &state,
                Control::Play {
                    items: Vec::new(),
                    mode: jellium_protocol::PlayMode::Now,
                    start_index: 0,
                    start_ticks: 0,
                    media_source: None,
                    audio_stream: None,
                    subtitles: jellium_protocol::Subtitles::Default,
                },
            )
            .await;

        assert_eq!(
            arriving.try_recv(),
            Ok(Event::RemoteEnded {
                cause: RemoteEnded::Controlled
            })
        );
        assert_eq!(
            state.live.remote.driving(tab).await,
            Err(LiveRefusal::NotDriving)
        );
    }

    #[tokio::test]
    async fn a_command_that_is_not_a_play_leaves_remote_mode_standing() {
        let state = Arc::new(AppState::stub(scratch("standing-mode")));
        let (tab, mut arriving) = bound_hub(&state).await;

        state.live.command(&state, Control::PlayPause).await;

        assert_eq!(
            arriving.try_recv(),
            Ok(Event::Control(Control::PlayPause)),
            "the command did not reach the tab"
        );
        assert_eq!(
            state.live.remote.driving(tab).await,
            Ok("target".to_owned()),
            "a command that is not a play ended remote mode"
        );
    }

    fn a_group() -> jellium_protocol::Group {
        jellium_protocol::Group {
            id: uuid::Uuid::from_u128(1),
            name: "Group".to_owned(),
            participants: Vec::new(),
            state: jellium_protocol::GroupState::Idle,
        }
    }

    /// A state signed in to `server` with one tab already in a group.
    async fn grouped_state(
        name: &str,
        server: &crate::web::upstream::Answering,
    ) -> (
        Arc<AppState>,
        TabId,
        tokio::sync::mpsc::UnboundedReceiver<Event>,
    ) {
        let state = Arc::new(AppState::stub(scratch(name)));
        state
            .session
            .install(crate::web::upstream::Upstream::stub(&server.base))
            .await;
        let (tab, mut arriving, _) = state.live.tabs.add().await;
        state.live.entered(&state, tab, a_group()).await;
        while arriving.try_recv().is_ok() {}
        (state, tab, arriving)
    }

    fn drained(arriving: &mut tokio::sync::mpsc::UnboundedReceiver<Event>) -> Vec<Event> {
        let mut seen = Vec::new();
        while let Ok(event) = arriving.try_recv() {
            seen.push(event);
        }
        seen
    }

    #[tokio::test]
    async fn a_scheduled_command_reaches_the_member_tab_and_no_other() {
        let server = crate::web::upstream::answering(204).await;
        let (state, _tab, mut arriving) = grouped_state("scheduled", &server).await;
        let (_, mut to_other, _) = state.live.tabs.add().await;

        state
            .live
            .scheduled(&jellyfin_api::types::SendCommand {
                command: Some(SendCommandType::Unpause),
                position_ticks: Some(42),
                when: None,
                ..Default::default()
            })
            .await;

        let seen = drained(&mut arriving);
        assert!(
            seen.iter()
                .any(|event| matches!(event, Event::Scheduled(command)
                    if command.command == jellium_protocol::GroupCommand::Unpause
                        && command.position_ticks == 42)),
            "the member tab did not receive the command: {seen:?}"
        );
        assert!(to_other.try_recv().is_err());
    }

    #[tokio::test]
    async fn a_control_command_reaches_the_member_tab_before_the_newest_one() {
        let server = crate::web::upstream::answering(204).await;
        let (state, _tab, mut arriving) = grouped_state("control-member", &server).await;
        let (_, mut to_newest, _) = state.live.tabs.add().await;

        state.live.command(&state, Control::PlayPause).await;

        assert_eq!(
            drained(&mut arriving),
            vec![Event::Control(Control::PlayPause)]
        );
        assert!(to_newest.try_recv().is_err());
    }

    #[tokio::test]
    async fn entering_a_group_ends_remote_mode_naming_that_cause() {
        let server = crate::web::upstream::answering(204).await;
        let state = Arc::new(AppState::stub(scratch("entering")));
        state
            .session
            .install(crate::web::upstream::Upstream::stub(&server.base))
            .await;
        let (tab, mut arriving) = bound_hub(&state).await;

        state.live.entered(&state, tab, a_group()).await;

        let seen = drained(&mut arriving);
        assert!(
            seen.contains(&Event::RemoteEnded {
                cause: RemoteEnded::Grouped
            }),
            "{seen:?}"
        );
    }

    #[tokio::test]
    async fn a_second_tab_taking_the_group_displaces_the_first_once() {
        let server = crate::web::upstream::answering(204).await;
        let (state, _first, mut to_first) = grouped_state("second-tab", &server).await;
        let (second, mut to_second, _) = state.live.tabs.add().await;

        reported(&state, second, Report::TakeGroup).await;
        reported(&state, second, Report::TakeGroup).await;

        let displaced = drained(&mut to_first)
            .into_iter()
            .filter(|event| {
                event
                    == &Event::Displaced {
                        cause: Displaced::Group,
                    }
            })
            .count();
        assert_eq!(displaced, 1);
        assert!(
            drained(&mut to_second)
                .iter()
                .any(|event| matches!(event, Event::Joined { .. }))
        );
    }

    #[tokio::test]
    async fn a_playback_start_in_a_second_tab_moves_the_group_to_it() {
        let server = crate::web::upstream::answering(204).await;
        let (state, first, mut to_first) = grouped_state("playback-moves", &server).await;
        let (second, _to_second, _) = state.live.tabs.add().await;

        reported(
            &state,
            second,
            Report::Playing {
                play_session: "session".to_owned(),
            },
        )
        .await;

        assert_eq!(state.live.group.holder().await, Some(second));
        assert!(state.live.group.member(first).await.is_err());
        assert!(drained(&mut to_first).contains(&Event::Displaced {
            cause: Displaced::Group
        }));
    }

    #[tokio::test]
    async fn a_group_update_saying_this_session_is_not_in_the_group_returns_to_local() {
        let server = crate::web::upstream::answering(204).await;
        let (state, tab, mut arriving) = grouped_state("not-in-group", &server).await;

        state.live.grouped(&state, group::Update::NotInGroup).await;

        assert!(drained(&mut arriving).contains(&Event::GroupEnded {
            cause: GroupEnded::NotInGroup
        }));
        assert!(state.live.group.member(tab).await.is_err());
        assert!(!state.live.group.grouped().await);
    }

    #[tokio::test]
    async fn a_shutdown_leaves_the_group_upstream() {
        let server = crate::web::upstream::answering(204).await;
        let (state, _tab, _arriving) = grouped_state("shutdown-leaves", &server).await;

        state.live.shutdown(&state).await;

        assert_eq!(server.asked("/SyncPlay/Leave"), 1);
        assert!(!state.live.group.grouped().await);
    }

    #[tokio::test]
    async fn a_beacon_leave_leaves_the_group_upstream() {
        let server = crate::web::upstream::answering(204).await;
        let (state, _tab, _arriving) = grouped_state("beacon-leaves", &server).await;

        leaving(State(state.clone())).await;

        assert_eq!(server.asked("/SyncPlay/Leave"), 1);
        assert!(!state.live.group.grouped().await);
    }

    #[tokio::test]
    async fn an_ordinary_playback_start_outside_a_group_refuses_nothing() {
        let state = Arc::new(AppState::stub(scratch("ungrouped-play")));
        let (tab, mut arriving, _) = state.live.tabs.add().await;

        reported(
            &state,
            tab,
            Report::Playing {
                play_session: "session".to_owned(),
            },
        )
        .await;

        assert_eq!(drained(&mut arriving), Vec::new());
    }

    #[tokio::test]
    async fn a_leave_names_the_cause_to_every_tab_and_ends_membership() {
        let server = crate::web::upstream::answering(204).await;
        let (state, member, mut to_member) = grouped_state("leave-names", &server).await;
        let (_, mut to_other, _) = state.live.tabs.add().await;

        reported(&state, member, Report::LeaveGroup).await;

        let ended = Event::GroupEnded {
            cause: GroupEnded::Left,
        };
        assert!(drained(&mut to_member).contains(&ended));
        assert!(drained(&mut to_other).contains(&ended));
        assert!(!state.live.group.grouped().await);
        assert_eq!(server.asked("/SyncPlay/Leave"), 1);
    }

    #[tokio::test]
    async fn a_group_left_push_names_the_cause_to_every_tab_and_ends_membership() {
        let server = crate::web::upstream::answering(204).await;
        let (state, _member, mut to_member) = grouped_state("left-push", &server).await;
        let (_, mut to_other, _) = state.live.tabs.add().await;

        state.live.grouped(&state, group::Update::Left).await;

        let ended = Event::GroupEnded {
            cause: GroupEnded::Removed,
        };
        assert!(drained(&mut to_member).contains(&ended));
        assert!(drained(&mut to_other).contains(&ended));
        assert!(!state.live.group.grouped().await);
    }

    #[tokio::test]
    async fn a_beacon_leave_names_the_cause_to_every_tab() {
        let server = crate::web::upstream::answering(204).await;
        let (state, _member, mut to_member) = grouped_state("beacon-names", &server).await;
        let (_, mut to_other, _) = state.live.tabs.add().await;

        leaving(State(state.clone())).await;

        let ended = Event::GroupEnded {
            cause: GroupEnded::Left,
        };
        assert!(drained(&mut to_member).contains(&ended));
        assert!(drained(&mut to_other).contains(&ended));
    }

    #[tokio::test]
    async fn only_the_tab_holding_the_group_is_told_it_is_the_member() {
        let server = crate::web::upstream::answering(204).await;
        let state = Arc::new(AppState::stub(scratch("who-holds")));
        state
            .session
            .install(crate::web::upstream::Upstream::stub(&server.base))
            .await;
        let (member, mut to_member, _) = state.live.tabs.add().await;
        let (_, mut to_other, _) = state.live.tabs.add().await;

        state.live.entered(&state, member, a_group()).await;

        assert!(drained(&mut to_member).contains(&Event::Joined {
            group: a_group(),
            member: true,
        }));
        assert!(drained(&mut to_other).contains(&Event::Joined {
            group: a_group(),
            member: false,
        }));
    }

    #[tokio::test]
    async fn a_second_tab_taking_the_group_tells_the_first_it_is_no_longer_the_member() {
        let server = crate::web::upstream::answering(204).await;
        let (state, _first, mut to_first) = grouped_state("no-longer-member", &server).await;
        let (second, mut to_second, _) = state.live.tabs.add().await;

        reported(&state, second, Report::TakeGroup).await;

        assert!(drained(&mut to_first).contains(&Event::Joined {
            group: a_group(),
            member: false,
        }));
        assert!(drained(&mut to_second).contains(&Event::Joined {
            group: a_group(),
            member: true,
        }));
    }

    #[tokio::test]
    async fn a_join_publishes_no_group_until_the_jellyfin_server_names_one() {
        let server = crate::web::upstream::answering(204).await;
        let state = Arc::new(AppState::stub(scratch("join-waits")));
        state
            .session
            .install(crate::web::upstream::Upstream::stub(&server.base))
            .await;
        let (tab, mut arriving, _) = state.live.tabs.add().await;

        reported(
            &state,
            tab,
            Report::JoinGroup {
                group: a_group().id,
            },
        )
        .await;

        assert!(!state.live.group.grouped().await);
        assert!(
            !drained(&mut arriving)
                .iter()
                .any(|event| matches!(event, Event::Joined { .. }))
        );

        state
            .live
            .grouped(&state, group::Update::Joined(a_group()))
            .await;

        assert_eq!(state.live.group.holder().await, Some(tab));
        assert!(drained(&mut arriving).contains(&Event::Joined {
            group: a_group(),
            member: true,
        }));
    }

    #[tokio::test]
    async fn a_play_queue_update_pushes_nothing_back_onto_the_group() {
        let server = crate::web::upstream::answering(204).await;
        let (state, _member, mut arriving) = grouped_state("queue-push", &server).await;
        let queue = jellium_protocol::GroupQueue::default();

        state
            .live
            .grouped(&state, group::Update::Queue(queue.clone()))
            .await;

        assert!(drained(&mut arriving).contains(&Event::GroupQueue(queue)));
        assert_eq!(server.asked("/SyncPlay/SetNewQueue"), 0);
    }

    #[tokio::test]
    async fn a_target_that_disappears_ends_the_mode_naming_the_cause() {
        let hub = Hub::new();
        let (tab, mut arriving, _) = hub.tabs.add().await;
        let target = Target {
            session: "target".to_owned(),
            device_name: String::new(),
            client_name: String::new(),
            now_playing: None,
        };
        hub.pushed(vec![target]).await;
        hub.remote.take(tab, "target").await.expect("taken");

        hub.pushed(Vec::new()).await;

        assert_eq!(
            arriving.try_recv(),
            Ok(Event::RemoteEnded {
                cause: RemoteEnded::TargetGone
            })
        );
    }
}
