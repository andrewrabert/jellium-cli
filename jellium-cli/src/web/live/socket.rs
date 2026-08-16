use std::sync::Arc;
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};
use tokio_tungstenite::tungstenite;

use jellium_protocol::Feed;

use super::message::{self, Dispatched};
use crate::web::AppState;

/// A burst of user-data changes inside this window becomes one refresh.
pub const COALESCE: Duration = Duration::from_millis(250);

/// The first retry waits this long, and each later one waits twice the last.
/// The window library changes coalesce over, which is what bounds a running
/// scan to one message a second for all tabs together.
pub const LIBRARY_COALESCE: Duration = Duration::from_secs(1);

pub const BACKOFF: Duration = Duration::from_secs(1);

pub const BACKOFF_CAP: Duration = Duration::from_secs(30);

/// The wait before the attempt that follows `failures` failed ones: `BACKOFF`
/// doubling to `BACKOFF_CAP`.
pub fn backoff(failures: u32) -> Duration {
    let doubled = BACKOFF
        .checked_mul(1u32.checked_shl(failures).unwrap_or(u32::MAX))
        .unwrap_or(BACKOFF_CAP);
    doubled.min(BACKOFF_CAP)
}

/// The cadence used until the Jellyfin server names its own.
pub const KEEPALIVE: Duration = Duration::from_secs(30);

/// The initial delay and interval, in milliseconds, each subscription is
/// started with.
pub const SESSIONS_TIMING: &str = "0,1500";
pub const TASKS_TIMING: &str = "0,1000";
pub const ACTIVITY_TIMING: &str = "0,1000";

/// What the hub asks of the running socket.
enum Wanted {
    /// Start `feed`'s upstream subscription, or stop it.
    Feed(Feed, bool),
}

/// The running upstream socket.
pub struct Link {
    wanted: UnboundedSender<Wanted>,
    task: tokio::task::JoinHandle<()>,
}

/// The upstream frame that starts `feed`'s subscription, or stops it; `None`
/// for a feed the Jellyfin server pushes unasked.
fn subscription(feed: Feed, start: bool) -> Option<String> {
    let (name, timing) = match feed {
        Feed::Targets | Feed::Sessions => ("Sessions", SESSIONS_TIMING),
        Feed::Tasks => ("ScheduledTasksInfo", TASKS_TIMING),
        Feed::Activity => ("ActivityLogEntry", ACTIVITY_TIMING),
        Feed::Groups | Feed::Refresh | Feed::Packages => return None,
    };
    let (kind, data) = if start {
        (format!("{name}Start"), timing)
    } else {
        (format!("{name}Stop"), "")
    };
    Some(serde_json::json!({ "MessageType": kind, "Data": data }).to_string())
}

fn keep_alive() -> String {
    serde_json::json!({ "MessageType": "KeepAlive" }).to_string()
}

/// True when the Jellyfin server closed because it would not take the token.
fn token_rejected(error: &tungstenite::Error) -> bool {
    matches!(
        error,
        tungstenite::Error::Http(response)
            if response.status() == tungstenite::http::StatusCode::UNAUTHORIZED
                || response.status() == tungstenite::http::StatusCode::FORBIDDEN
    )
}

async fn declare(state: &Arc<AppState>, controllable: bool) {
    let Some(upstream) = state.session.signed().await else {
        return;
    };
    if let Err(e) = upstream
        .declare_capabilities(controllable, upstream.state.live_tv.allowed())
        .await
    {
        eprintln!("jellium-cli web: {e:?}");
    }
}

/// What one pass over an open socket ended with.
enum Ended {
    /// The token was refused, so no retry follows; the session it names is
    /// the one to clear.
    TokenRejected(Arc<crate::web::upstream::Upstream>),
    /// The hub asked the socket to stop.
    Stopped,
    /// The link went away, so a retry follows; `connected` is true when this
    /// pass had an open socket.
    Dropped { connected: bool },
}

/// The keep-alive deadline, which an arriving frame does not move.
struct Beat {
    next: tokio::time::Instant,
    cadence: Duration,
}

impl Beat {
    fn new(cadence: Duration) -> Beat {
        Beat {
            next: tokio::time::Instant::now() + cadence,
            cadence,
        }
    }

    fn due(&self) -> tokio::time::Instant {
        self.next
    }

    /// Moves the deadline one cadence on, past any that have already passed.
    fn beaten(&mut self) {
        let now = tokio::time::Instant::now();
        self.next += self.cadence;
        if self.next <= now {
            self.next = now + self.cadence;
        }
    }

    /// Takes the cadence the Jellyfin server asked for, from now.
    fn asked(&mut self, cadence: Duration) {
        self.cadence = cadence;
        self.next = tokio::time::Instant::now() + cadence;
    }
}

/// Holds the user-data changes seen inside `COALESCE` and sends them as one
/// refresh.
struct Coalescing {
    held: std::collections::HashMap<uuid::Uuid, jellium_protocol::Marked>,
    /// The timer changes held, keyed by timer id so a later change naming the
    /// same timer replaces the one held, and kept in arrival order.
    timers: Vec<jellium_protocol::TimerChanged>,
    /// The activity entries held, newest first.
    entries: Vec<jellium_protocol::ActivityEntry>,
    /// The refresh progress held, keyed by item so a later one replaces the
    /// one held.
    refreshes: std::collections::HashMap<uuid::Uuid, jellium_protocol::Refreshed>,
    since: Option<tokio::time::Instant>,
    /// The library ids held, on a window of their own.
    library: super::message::Changed,
    library_since: Option<tokio::time::Instant>,
}

impl Coalescing {
    fn new() -> Coalescing {
        Coalescing {
            held: std::collections::HashMap::new(),
            timers: Vec::new(),
            entries: Vec::new(),
            refreshes: std::collections::HashMap::new(),
            since: None,
            library: super::message::Changed::default(),
            library_since: None,
        }
    }

    /// Holds one library change on its own window.
    fn library(&mut self, changed: super::message::Changed) {
        self.library.absorb(changed);
        self.library_since
            .get_or_insert_with(tokio::time::Instant::now);
    }

    fn library_due(&self) -> Option<tokio::time::Instant> {
        self.library_since.map(|since| since + LIBRARY_COALESCE)
    }

    fn library_take(&mut self) -> super::message::Changed {
        self.library_since = None;
        std::mem::take(&mut self.library)
    }

    fn hold(&mut self, items: Vec<jellium_protocol::Marked>) {
        for item in items {
            self.held.insert(item.item, item);
        }
        self.since.get_or_insert_with(tokio::time::Instant::now);
    }

    /// Holds one timer change; a later change naming the same timer replaces
    /// the one held.
    fn timed(&mut self, changed: jellium_protocol::TimerChanged) {
        match self
            .timers
            .iter_mut()
            .find(|held| held.timer == changed.timer)
        {
            Some(held) => *held = changed,
            None => self.timers.push(changed),
        }
        self.since.get_or_insert_with(tokio::time::Instant::now);
    }

    /// The timer changes held, oldest first.
    fn timers(&mut self) -> Vec<jellium_protocol::TimerChanged> {
        std::mem::take(&mut self.timers)
    }

    /// Holds one activity entry; the window carries them newest first.
    fn logged(&mut self, entry: jellium_protocol::ActivityEntry) {
        self.entries.insert(0, entry);
        self.since.get_or_insert_with(tokio::time::Instant::now);
    }

    fn entries(&mut self) -> Vec<jellium_protocol::ActivityEntry> {
        std::mem::take(&mut self.entries)
    }

    /// Holds one refresh progress; a later one naming the same item replaces
    /// the one held.
    fn refreshed(&mut self, progress: jellium_protocol::Refreshed) {
        self.refreshes.insert(progress.item, progress);
        self.since.get_or_insert_with(tokio::time::Instant::now);
    }

    fn refreshes(&mut self) -> Vec<jellium_protocol::Refreshed> {
        self.refreshes.drain().map(|(_, held)| held).collect()
    }

    fn due(&self) -> Option<tokio::time::Instant> {
        self.since.map(|since| since + COALESCE)
    }

    fn take(&mut self) -> Vec<jellium_protocol::Marked> {
        self.since = None;
        self.held.drain().map(|(_, item)| item).collect()
    }
}

async fn run(
    state: &Arc<AppState>,
    wanted: &mut tokio::sync::mpsc::UnboundedReceiver<Wanted>,
    watching: &mut std::collections::HashSet<Feed>,
) -> Ended {
    let Some(upstream) = state.session.signed().await else {
        return Ended::Dropped { connected: false };
    };
    let Some(identity) = state.identity.held().await else {
        return Ended::Dropped { connected: false };
    };
    let url = upstream.socket_url(&identity);
    let user = upstream.user_id();
    let live_tv = upstream.state.live_tv.allowed();

    let mut socket = match tokio_tungstenite::connect_async(&url).await {
        Ok((socket, _)) => socket,
        Err(e) if token_rejected(&e) => return Ended::TokenRejected(upstream),
        Err(_) => return Ended::Dropped { connected: false },
    };
    drop(upstream);

    declare(state, true).await;
    for feed in Feed::ALL {
        if !watching.contains(&feed) {
            continue;
        }
        let Some(frame) = subscription(feed, true) else {
            continue;
        };
        if socket
            .send(tungstenite::Message::Text(frame.into()))
            .await
            .is_err()
        {
            return Ended::Dropped { connected: true };
        }
    }

    let mut beat = Beat::new(KEEPALIVE);
    let mut coalescing = Coalescing::new();
    let ended = loop {
        let flush = coalescing.due();
        let library_flush = coalescing.library_due();
        let ended = tokio::select! {
            asked = wanted.recv() => match asked {
                Some(Wanted::Feed(feed, start)) => {
                    if start {
                        watching.insert(feed);
                    } else {
                        watching.remove(&feed);
                    }
                    if let Some(frame) = subscription(feed, start)
                        && socket
                            .send(tungstenite::Message::Text(frame.into()))
                            .await
                            .is_err()
                    {
                        break Ended::Dropped { connected: true };
                    }
                    None
                }
                None => break Ended::Stopped,
            },
            _ = tokio::time::sleep_until(beat.due()) => {
                beat.beaten();
                if socket
                    .send(tungstenite::Message::Text(keep_alive().into()))
                    .await
                    .is_err()
                {
                    break Ended::Dropped { connected: true };
                }
                None
            }
            _ = async {
                match flush {
                    Some(due) => tokio::time::sleep_until(due).await,
                    None => std::future::pending().await,
                }
            } => {
                let items = coalescing.take();
                let changes = coalescing.timers();
                let entries = coalescing.entries();
                let refreshes = coalescing.refreshes();
                if !items.is_empty() {
                    state.live.broadcast(jellium_protocol::Event::Marked { items }).await;
                }
                if !changes.is_empty() {
                    state.live.broadcast(jellium_protocol::Event::Timers { changes }).await;
                }
                if !entries.is_empty() {
                    state
                        .live
                        .fed(Feed::Activity, jellium_protocol::Event::Activity { entries })
                        .await;
                }
                if !refreshes.is_empty() {
                    state
                        .live
                        .fed(
                            Feed::Refresh,
                            jellium_protocol::Event::Refreshing { items: refreshes },
                        )
                        .await;
                }
                None
            }
            _ = async {
                match library_flush {
                    Some(due) => tokio::time::sleep_until(due).await,
                    None => std::future::pending().await,
                }
            } => {
                let changed = coalescing.library_take();
                if !changed.is_empty() {
                    state
                        .live
                        .broadcast(jellium_protocol::Event::LibraryChanged {
                            added: changed.added,
                            removed: changed.removed,
                            updated: changed.updated,
                        })
                        .await;
                }
                None
            }
            received = socket.next() => match received {
                Some(Ok(tungstenite::Message::Text(frame))) => {
                    match message::dispatch(&frame, user, live_tv) {
                        Dispatched::Broadcast(event) => state.live.broadcast(event).await,
                        Dispatched::Marked(items) => coalescing.hold(items),
                        Dispatched::Library(changed) => coalescing.library(changed),
                        Dispatched::Timer(changed) => coalescing.timed(changed),
                        Dispatched::Command(control) => state.live.command(state, control).await,
                        Dispatched::Sessions(sessions) => state.live.listed(state, &sessions).await,
                        Dispatched::Scheduled(command) => {
                            state.live.scheduled(&command).await;
                        }
                        Dispatched::Group(update) => {
                            state.live.grouped(state, update).await;
                        }
                        Dispatched::Tasks(tasks) => state.live.tasked(tasks).await,
                        Dispatched::Activity(entry) => coalescing.logged(entry),
                        Dispatched::Refresh(progress) => coalescing.refreshed(progress),
                        Dispatched::Package(event) => {
                            state.live.fed(Feed::Packages, event).await;
                        }
                        Dispatched::UserUpdated {
                            administrator,
                            preference_access,
                        } => {
                            state
                                .live
                                .user_updated(administrator, preference_access)
                                .await;
                        }
                        Dispatched::KeepAlive(interval) => beat.asked(interval / 2),
                        Dispatched::Ignored => {}
                    }
                    None
                }
                Some(Ok(_)) => None,
                Some(Err(_)) | None => break Ended::Dropped { connected: true },
            },
        };
        if let Some(ended) = ended {
            break ended;
        }
    };
    declare(state, false).await;
    ended
}

impl Link {
    /// Opens the socket and keeps it open while tabs remain: it declares media
    /// control on every open and no media control on every close, sends a
    /// keep-alive every half the interval the Jellyfin server named however
    /// much it receives in between, dispatches every frame through
    /// `message::dispatch`, and waits `backoff` before each retry, counting
    /// from the first wait again after any pass that connected.
    /// A close the Jellyfin server made because the access token was rejected
    /// clears the held session and is not retried.
    /// A frame this milestone does not handle is discarded and the socket
    /// stays open.
    pub fn open(state: Arc<AppState>) -> Link {
        let (wanted, mut asking) = unbounded_channel();
        let task = tokio::spawn(async move {
            let mut watching = std::collections::HashSet::new();
            let mut failures = 0u32;
            loop {
                match run(&state, &mut asking, &mut watching).await {
                    Ended::Stopped => return,
                    Ended::TokenRejected(rejected) => {
                        state.session.reject(&rejected).await;
                        return;
                    }
                    Ended::Dropped { connected } => {
                        failures = if connected { 0 } else { failures + 1 };
                    }
                }
                tokio::time::sleep(backoff(failures)).await;
            }
        });
        Link { wanted, task }
    }

    /// Starts `feed`'s upstream subscription, or stops it; a reopened socket
    /// restarts every feed held, so a subscription stopped and restarted
    /// delivers what it would have delivered had it never stopped.
    pub fn watch(&self, feed: Feed, watching: bool) {
        let _ = self.wanted.send(Wanted::Feed(feed, watching));
    }

    /// Stops the socket and declares no media control.
    pub async fn close(self, state: &Arc<AppState>) {
        drop(self.wanted);
        self.task.abort();
        let _ = self.task.await;
        declare(state, false).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::web::upstream::{Answering, Upstream, answering};
    use jellium_protocol::Event;

    fn scratch(name: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir()
            .join("jellium-cli-socket-tests")
            .join(name);
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("scratch directory");
        path.join("session.env")
    }

    /// An app state signed in to `server`, with one tab already connected so
    /// events have somewhere to land.
    async fn signed_in(
        name: &str,
        server: &Answering,
    ) -> (Arc<AppState>, tokio::sync::mpsc::UnboundedReceiver<Event>) {
        let state = Arc::new(AppState::stub(scratch(name)));
        state.session.install(Upstream::stub(&server.base)).await;
        let (_, arriving, _) = state.live.tabs_for_test().add().await;
        (state, arriving)
    }

    /// Waits for the next event, or gives up.
    async fn next(arriving: &mut tokio::sync::mpsc::UnboundedReceiver<Event>) -> Option<Event> {
        tokio::time::timeout(Duration::from_secs(5), arriving.recv())
            .await
            .ok()
            .flatten()
    }

    #[tokio::test]
    async fn an_idle_socket_answers_the_keep_alive_the_server_asked_for() {
        let server = answering(204).await;
        let (state, _arriving) = signed_in("keep-alive", &server).await;
        let link = Link::open(state.clone());
        server.opened(1).await;

        server.push(r#"{"MessageType":"ForceKeepAlive","Data":1}"#);
        tokio::time::sleep(Duration::from_millis(1_400)).await;

        let sent = server.inbound.lock().expect("the sent frames").clone();
        assert!(
            sent.iter().any(|frame| frame.contains("KeepAlive")),
            "no keep-alive was answered: {sent:?}"
        );
        assert_eq!(server.sockets.load(std::sync::atomic::Ordering::SeqCst), 1);
        link.close(&state).await;
    }

    #[tokio::test]
    async fn a_pushing_sessions_subscription_does_not_starve_the_keep_alive() {
        let server = answering(204).await;
        let (state, _arriving) = signed_in("not-starved", &server).await;
        let link = Link::open(state.clone());
        server.opened(1).await;

        server.push(r#"{"MessageType":"ForceKeepAlive","Data":1}"#);
        for _ in 0..30 {
            server.push(r#"{"MessageType":"Sessions","Data":[]}"#);
            tokio::time::sleep(Duration::from_millis(50)).await;
        }

        let sent = server.inbound.lock().expect("the sent frames").clone();
        let beats = sent
            .iter()
            .filter(|frame| frame.contains("KeepAlive"))
            .count();
        assert!(
            beats >= 2,
            "the arriving frames starved the keep-alive: {sent:?}"
        );
        assert_eq!(server.sockets.load(std::sync::atomic::Ordering::SeqCst), 1);
        link.close(&state).await;
    }

    #[test]
    fn the_backoff_doubles_from_the_first_wait_to_the_cap() {
        assert_eq!(backoff(0), BACKOFF);
        assert_eq!(backoff(1), BACKOFF * 2);
        assert_eq!(backoff(2), BACKOFF * 4);
        assert_eq!(backoff(4), BACKOFF * 16);
        assert_eq!(backoff(5), BACKOFF_CAP);
        assert_eq!(backoff(1_000), BACKOFF_CAP);
        assert_eq!(backoff(u32::MAX), BACKOFF_CAP);
    }

    #[tokio::test]
    async fn a_socket_that_connected_retries_from_the_first_backoff() {
        let server = answering(204).await;
        let (state, _arriving) = signed_in("first-backoff", &server).await;
        let link = Link::open(state.clone());
        server.opened(1).await;

        for open in 2..=4 {
            server.drop_sockets();
            let since = tokio::time::Instant::now();
            server.opened(open).await;
            let waited = since.elapsed();
            assert!(
                waited < BACKOFF * 2,
                "the {open}th open waited {waited:?}, so a pass that connected did not \
                 return to the first backoff"
            );
        }
        link.close(&state).await;
    }

    #[tokio::test]
    async fn a_socket_that_drops_is_reopened_and_resubscribes() {
        let server = answering(204).await;
        let (state, _arriving) = signed_in("reopened", &server).await;
        let link = Link::open(state.clone());
        server.opened(1).await;
        link.watch(Feed::Sessions, true);
        tokio::time::sleep(Duration::from_millis(100)).await;

        server.drop_sockets();
        server.opened(2).await;
        tokio::time::sleep(Duration::from_millis(200)).await;

        let sent = server.inbound.lock().expect("the sent frames").clone();
        let started = sent
            .iter()
            .filter(|frame| frame.contains("SessionsStart"))
            .count();
        assert!(
            started >= 2,
            "the reopened socket did not resubscribe: {sent:?}"
        );
        link.close(&state).await;
    }

    #[tokio::test]
    async fn a_burst_of_user_data_changes_produces_one_refresh() {
        let server = answering(204).await;
        let (state, mut arriving) = signed_in("coalesced", &server).await;
        let link = Link::open(state.clone());
        server.opened(1).await;

        let user = uuid::Uuid::nil();
        for index in 0..1_000u128 {
            server.push(&format!(
                r#"{{"MessageType":"UserDataChanged","Data":{{"UserId":"{user}","UserDataList":[{{"ItemId":"{}","Played":true}}]}}}}"#,
                uuid::Uuid::from_u128(index)
            ));
        }

        let event = next(&mut arriving).await.expect("one refresh");
        let Event::Marked { items } = event else {
            panic!("the burst did not produce a refresh: {event:?}");
        };
        assert_eq!(items.len(), 1_000);
        assert!(
            tokio::time::timeout(Duration::from_millis(500), arriving.recv())
                .await
                .is_err(),
            "the burst produced more than one refresh"
        );
        link.close(&state).await;
    }

    #[tokio::test]
    async fn two_timer_changes_inside_the_window_arrive_as_one_event() {
        let server = answering(204).await;
        let (state, mut arriving) = signed_in("timers-coalesced", &server).await;
        let link = Link::open(state.clone());
        server.opened(1).await;

        let program = uuid::Uuid::from_u128(7);
        server.push(&format!(
            r#"{{"MessageType":"TimerCreated","Data":{{"Id":"timer-1","ProgramId":"{program}"}}}}"#
        ));
        server.push(&format!(
            r#"{{"MessageType":"TimerCancelled","Data":{{"Id":"timer-2","ProgramId":"{program}"}}}}"#
        ));

        let event = next(&mut arriving).await.expect("one timer event");
        let Event::Timers { changes } = event else {
            panic!("the burst did not produce a timer event: {event:?}");
        };
        assert_eq!(changes.len(), 2);
        assert_eq!(changes[0].timer, "timer-1");
        assert_eq!(changes[1].timer, "timer-2");
        assert!(
            tokio::time::timeout(Duration::from_millis(500), arriving.recv())
                .await
                .is_err(),
            "the burst produced more than one timer event"
        );
        link.close(&state).await;
    }

    #[tokio::test]
    async fn a_timer_change_reaches_every_tab_within_the_coalescing_window() {
        let server = answering(204).await;
        let state = Arc::new(AppState::stub(scratch("timers-every-tab")));
        state.session.install(Upstream::stub(&server.base)).await;
        let (_, mut one, _) = state.live.tabs_for_test().add().await;
        let (_, mut two, _) = state.live.tabs_for_test().add().await;
        let link = Link::open(state.clone());
        server.opened(1).await;

        let started = std::time::Instant::now();
        server.push(
            r#"{"MessageType":"SeriesTimerCreated","Data":{"Id":"series-1","ProgramId":"00000000-0000-0000-0000-000000000007"}}"#,
        );

        for arriving in [&mut one, &mut two] {
            let event = next(arriving).await.expect("a timer event");
            let Event::Timers { changes } = event else {
                panic!("a tab did not receive the timer event: {event:?}");
            };
            assert_eq!(changes.len(), 1);
            assert_eq!(changes[0].timer, "series-1");
            assert_eq!(
                changes[0].change,
                jellium_protocol::TimerChange::SeriesCreated
            );
        }
        assert!(started.elapsed() < Duration::from_secs(1));
        link.close(&state).await;
    }

    #[tokio::test]
    async fn a_burst_of_library_changes_reaches_a_tab_as_one_message_carrying_every_id() {
        let server = answering(204).await;
        let (state, mut arriving) = signed_in("library", &server).await;
        let link = Link::open(state.clone());
        server.opened(1).await;

        for at in 1..=5u128 {
            server.push(&format!(
                r#"{{"MessageType":"LibraryChanged","Data":{{"ItemsAdded":["{}"],"ItemsUpdated":["{}"]}}}}"#,
                uuid::Uuid::from_u128(at),
                uuid::Uuid::from_u128(100 + at),
            ));
        }

        let Some(Event::LibraryChanged {
            added,
            removed,
            updated,
        }) = next(&mut arriving).await
        else {
            panic!("the burst did not arrive as one library change");
        };
        assert_eq!(added.len(), 5, "every added id reached the tab");
        assert_eq!(updated.len(), 5, "every updated id reached the tab");
        assert!(removed.is_empty());

        link.close(&state).await;
    }

    #[tokio::test]
    async fn a_message_this_milestone_does_not_handle_leaves_the_socket_open() {
        let server = answering(204).await;
        let (state, mut arriving) = signed_in("unhandled", &server).await;
        let link = Link::open(state.clone());
        server.opened(1).await;

        server.push(r#"{"MessageType":"LibraryChanged","Data":{"ItemsAdded":[]}}"#);
        server.push(r#"{"MessageType":"NotAMessageTypeAtAll"}"#);
        server.push("not json at all");
        tokio::time::sleep(Duration::from_millis(200)).await;
        server.push(r#"{"MessageType":"ServerRestarting"}"#);

        assert_eq!(
            next(&mut arriving).await,
            Some(Event::ServerStopping { restarting: true }),
            "the socket did not survive the unhandled frames"
        );
        assert_eq!(server.sockets.load(std::sync::atomic::Ordering::SeqCst), 1);
        link.close(&state).await;
    }

    #[tokio::test]
    async fn a_rejected_token_clears_the_session_and_stops_retrying() {
        let server = answering(401).await;
        let state = Arc::new(AppState::stub(scratch("rejected-token")));
        state.session.install(Upstream::stub(&server.base)).await;

        // the stub answers the handshake with 401, which is the Jellyfin
        // server refusing the access token
        let link = Link::open(state.clone());
        tokio::time::sleep(Duration::from_millis(500)).await;

        assert!(state.session.signed().await.is_none());
        assert_eq!(server.sockets.load(std::sync::atomic::Ordering::SeqCst), 0);
        link.close(&state).await;
    }
}
