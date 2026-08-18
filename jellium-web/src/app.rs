use std::collections::HashSet;
use std::rc::Rc;
use std::time::Duration;

use iced::widget::{center, column};
use iced::{Element, Subscription, Task, Theme};
use jellium_model::item::Mark;
use jellium_protocol::{
    Event, Feed, Group, Marked, Notice, PlaybackRefused, Quality, Report, Session, SessionStatus,
    Standing, Target,
};
use jellyfin_api::types::UserItemDataDto;
use uuid::Uuid;

use crate::api::Api;
use crate::boot;
use crate::control;
use crate::error::Answer;
use crate::failure;
use crate::fonts::Served;
use crate::images::{self, Cache};
use crate::live;
use crate::livetv::{Channel, Program};
use crate::player::group;
use crate::player::remote;
use crate::player::{self, Playing};
use crate::prefs::Device;
use crate::route::Route;
use crate::screen::livetv::{self, guide};
use crate::screen::program;
use crate::screen::{dashboard, detail, home, hub, library, login, search};
use crate::style::space::Room;
use crate::style::{self, Drawn, Viewport, card, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget;
use crate::widget::prose;
use crate::window;

/// A message another client sent stops showing after this long.
pub const NOTICE_HIDE: Duration = Duration::from_secs(6);

pub struct Jellium {
    pub stage: Stage,
    pub images: Cache,
    /// The page's own size, as the page reports it.
    pub viewport: Viewport,
    /// Every failure raised this session and the one shown above the view.
    pub failures: crate::failure::Log,
    /// True while the session's failure list is open.
    pub listing: bool,
}

pub enum Stage {
    /// The boot read of `/session`; `stalled` is set once a read answered a
    /// trouble, and the screen then carries the control that reads again.
    Booting {
        stalled: bool,
    },
    Login(login::State),
    Setup(crate::screen::setup::State),
    Signed(Box<Signed>),
    /// The session is gone; the only stage that replaces the screen.
    Lost(crate::failure::Failure),
}

pub struct Signed {
    pub session: Session,
    pub api: Rc<Api>,
    pub history: Vec<Route>,
    pub view: View,
    /// The overflow menu open now, which is never `Some` under read-only.
    pub overflow: Option<crate::screen::overflow::Open>,
    /// The images the local server minted handles for.
    pub foreign: crate::images::Foreign,
    /// The one playback in progress; audio keeps the screen underneath, video
    /// replaces it.
    pub playing: Option<Playing>,
    /// The element and queue a play request was issued for, held until its
    /// plan arrives.
    pub pending: Option<player::Pending>,
    /// The preferences this browser holds.
    pub device: Device,
    /// The preference bag as the server answered it, with the edits made.
    pub preferences: jellium_model::prefs::Bag,
    /// What the bag holds now, taken whenever the bag changes.
    pub held: jellium_model::prefs::Held,
    /// The user configuration as the server answered it, with the edits made.
    pub configuration: jellium_model::form::Form,
    /// What the configuration and the bag ask of the home screen, taken
    /// whenever either changes.
    pub arrangement: crate::screen::home::Arrangement,
    /// What this browser detects itself as, taken once where the session opens.
    pub browser: crate::browser::Browser,
    /// The event socket and how many attempts have failed.
    pub live: live::Link,
    /// The sessions the local server offered, refreshed while a picker or
    /// panel is open.
    pub targets: Vec<Target>,
    /// The target this tab drives, when it holds remote mode.
    pub remote: Option<remote::Bound>,
    /// A `DisplayMessage` another client sent, and how long it has shown.
    pub message: Option<(Notice, Duration)>,
    /// True while the Jellyfin server says a restart is required.
    pub restart_required: bool,
    /// The file input, mounted while a screen offering an upload is shown.
    pub picker: Option<crate::overlay::Mounted>,
    /// True once a `UserUpdated` named this user, which is what states that the
    /// server's copy changed.
    pub server_changed: bool,
    /// The route a Back or a navigation is waiting on an unsaved-edit
    /// confirmation for.
    pub leaving: Option<Route>,
    /// Set when an administrator restarted or shut the server down from this
    /// screen.
    pub stopped: Option<Stopped>,
    /// The joinable groups the local server offered, refreshed while a picker
    /// or the SyncPlay screen is open.
    pub groups: Vec<Group>,
    /// The group this installation is in.
    pub group: Option<group::Joined>,
    /// The queue view's window, held across navigation.
    pub queue: window::Window,
}

/// Why the client is not talking to the Jellyfin server: an administrator
/// stopped it from this screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stopped {
    /// Reconnection continues on the existing backoff.
    Restarted,
    /// No reconnection is attempted.
    ShutDown,
}

pub enum View {
    Loading,
    LiveTv(Box<livetv::State>),
    Program(Box<program::State>),
    Home(Box<home::State>),
    Library(Box<library::State>),
    Detail(Box<detail::State>),
    Search(Box<search::State>),
    Filtered(Box<crate::screen::browse::Browse>),
    Metadata(Box<crate::screen::metadata::State>),
    Collections(Box<crate::screen::collections::Listed>),
    Collection(Box<crate::screen::collections::State>),
    Playlists(Box<crate::screen::playlists::Listed>),
    Playlist(Box<crate::screen::playlists::State>),
    Queue,
    Remote,
    SyncPlay,
    Dashboard(Box<crate::screen::dashboard::State>),
    Settings(Box<crate::screen::settings::State>),
    /// A Live TV screen that cannot be shown, drawn as the sentence naming
    /// why in the screen's place.
    Unavailable,
}

#[derive(Debug, Clone)]
pub enum Message {
    /// One failure report raised anywhere in the client.
    Failed(crate::failure::Failure),
    /// The press an entry naming what its navigation is already showing
    /// carries, which the reference's own handler answers with no change.
    Unchanged,
    /// Leaves the terminal stage for a fresh login screen.
    SignInAgain,
    /// Reads `/session` again from the boot screen, which is the way out of a
    /// boot whose read failed.
    SessionRechecked,
    /// One request whose answer nothing on screen stands on, already reported
    /// where it was made.
    Quieted,
    FailureDismissed,
    FailuresOpened,
    FailuresClosed,
    Ready,
    SessionChecked(Answer<SessionStatus>),
    LoginAction(login::Action),
    /// What a login-stage request answered: the stage the local server moved
    /// to.
    LoginAnswered(Answer<SessionStatus>),
    /// The saved servers, re-read whenever the list is shown.
    ServersListed(Answer<Vec<jellium_protocol::SavedServer>>),
    ServerRemoved(Answer<jellium_protocol::Removed>),
    QuickConnectInitiated(Answer<jellium_protocol::QuickConnectCode>),
    QuickConnectPolled(Answer<jellium_protocol::QuickConnectState>),
    /// One five-second Quick Connect poll.
    QuickConnectTicked,
    ResetAnswered(Answer<jellium_protocol::ResetAnswer>),
    PinRedeemed(Answer<jellium_protocol::PinOutcome>),
    /// One public user's image, keyed by the user it was asked for.
    PublicImageLoaded(Uuid, Answer<Vec<u8>>),
    /// Leaves the active server for the list, keeping its credential.
    SwitchPressed,
    LogoutPressed,
    LoggedOut(Answer<()>),
    /// One step's values, read from the Jellyfin server on entry.
    SetupLoaded(Answer<(jellium_model::setup::Step, crate::screen::setup::Body)>),
    SetupAction(crate::screen::setup::Action),
    /// A step's write: `Ok` advances to the step after the one shown, and an
    /// error keeps the step with its values intact.
    SetupAdvanced(Answer<()>),
    /// A library write inside the libraries step: `Ok` re-reads the step.
    SetupWrote(Answer<()>),
    SetupBrowsed(Answer<(String, Vec<jellyfin_api::types::FileSystemEntryInfo>)>),
    /// The outcome of `Startup/Complete` and the sign-in that follows it.
    SetupCompleted(Answer<SessionStatus>),
    /// Back on the first step released the setup upstream.
    SetupLeft(Answer<()>),
    Navigated(Route),
    WentBack,
    HomeLoaded(Answer<home::State>),
    LibraryLoaded(Answer<library::State>),
    DetailLoaded(Answer<detail::State>),
    SearchLoaded(Answer<search::State>),
    /// One control of the shared sort and filter surfaces.
    BrowseAction(crate::screen::browse::Action),
    /// The page the browse window moved over, and the total the server
    /// reported.
    BrowsePaged(
        std::ops::Range<usize>,
        Answer<(Vec<jellyfin_api::types::BaseItemDto>, usize)>,
    ),
    /// The index the letter jump scrolls to.
    Jumped(Answer<usize>),
    HubPaged(
        std::ops::Range<usize>,
        Answer<(Vec<jellyfin_api::types::BaseItemDto>, usize)>,
    ),
    FilteredLoaded(Answer<crate::screen::browse::Browse>),
    CollectionsLoaded(Answer<crate::screen::collections::Listed>),
    CollectionLoaded(Answer<crate::screen::collections::State>),
    CollectionAction(crate::screen::collections::Action),
    OverflowAction(crate::screen::overflow::Action),
    MetadataLoaded(Answer<crate::screen::metadata::State>),
    MetadataAction(crate::screen::metadata::Action),
    /// The metadata manager asked for a file, which the picker answers.
    MetadataUploadRequested,
    /// The candidates an identify answered with.
    Identified(Answer<Vec<jellyfin_api::types::RemoteSearchResult>>),
    /// The remote images a search answered with.
    RemoteImagesLoaded(Answer<Vec<jellyfin_api::types::RemoteImageInfo>>),
    /// One foreign image, keyed by the handle it was asked for.
    ForeignLoaded(String, Answer<Vec<u8>>),
    /// The trickplay the playing item describes.
    TrickplayLoaded(Answer<crate::player::trickplay::Trickplay>),
    /// One trickplay tile sheet, keyed by the tile it was asked for.
    TileLoaded(crate::player::trickplay::Tile, Answer<Vec<u8>>),
    /// The pointer has settled on the scrub bar long enough for a preview.
    PreviewSettled,
    /// What a picker offered to file into.
    FilingLoaded(Answer<Vec<jellyfin_api::types::BaseItemDto>>),
    PlaylistsLoaded(Answer<crate::screen::playlists::Listed>),
    PlaylistLoaded(Answer<crate::screen::playlists::State>),
    PlaylistAction(crate::screen::playlists::Action),
    PlaylistPaged(
        std::ops::Range<usize>,
        Answer<(Vec<crate::screen::playlists::Entry>, usize)>,
    ),
    SearchEdited(String),
    SearchSubmitted,
    PlayedToggled(Uuid, Mark),
    FavoriteToggled(Uuid, Mark),
    UserDataUpdated(Uuid, Answer<UserItemDataDto>),
    PlayPressed(player::Intent),
    /// A start resolved from a play control, a Live TV selection or an
    /// inbound command.
    Resolved(Answer<player::Start>),
    /// The item the group commanded, resolved for playback here rather than
    /// pushed back onto the group.
    GroupResolved(Answer<player::Start>),
    Planned(Answer<player::Planned>),
    /// One message an overlaid element raised, dispatched by the element that
    /// raised it.
    Overlaid(crate::overlay::Raised),
    /// The preference bag and the user configuration, read when the session
    /// opens; taking them runs the one-time migration and re-loads the route
    /// shown, so every surface draws at the sizes the server holds.
    PreferencesLoaded(Answer<(jellium_model::prefs::Bag, serde_json::Value)>),
    /// The one-time migration's write, naming the ceiling it carried.
    Migrated(Quality, Answer<()>),
    DashboardLoaded(
        Answer<(
            crate::screen::dashboard::Screen,
            crate::screen::dashboard::Loaded,
        )>,
    ),
    DashboardAction(crate::screen::dashboard::Action),
    SettingsLoaded(
        Answer<(
            crate::screen::settings::Screen,
            crate::screen::settings::Loaded,
        )>,
    ),
    SettingsAction(crate::screen::settings::Action),
    /// One settings write's outcome, named by what it wrote and its object.
    SettingsWrote(crate::error::Wrote, Answer<()>),
    /// One settings save's outcome; `Ok` takes the written values as read and
    /// re-renders every open surface.
    SettingsSaved(crate::error::Wrote, Answer<()>),
    /// What an authorize answered with.
    QuickConnected(Answer<jellium_model::quickconnect::Outcome>),
    /// The user configuration re-read after a `UserUpdated`, which keeps every
    /// unsaved edit.
    ConfigurationRefreshed(Answer<serde_json::Value>),
    /// One administrative write's outcome, named by what it did and the object
    /// it did it to.
    DashboardWrote(crate::error::Wrote, Answer<()>),
    /// One configuration save's outcome.
    DashboardSaved(crate::error::Wrote, Answer<()>),
    /// One bridge answer, sent back down the frame's channel.
    Bridged(u64, Option<serde_json::Value>),
    /// One configuration write a bridge verb asked for: the frame is answered
    /// and a refusal is named on screen.
    BridgedWrote(u64, crate::error::Wrote, Answer<()>),
    /// What the server's filesystem browser answered with.
    DashboardBrowsed(Answer<(String, Vec<jellyfin_api::types::FileSystemEntryInfo>)>),
    /// A dashboard page the window moved over, fetched a page at a time.
    ActivityPaged(
        std::ops::Range<usize>,
        Answer<Vec<jellium_protocol::ActivityEntry>>,
    ),
    /// The tuners a discovery answered with.
    DashboardDiscovered(Answer<Vec<jellyfin_api::types::TunerHostInfo>>),
    /// The lineups a listing provider reported.
    DashboardLineups(Answer<Vec<jellyfin_api::types::NameIdPair>>),
    /// Refreshes one item's metadata, which the overflow menu offers an
    /// administrator alone.
    RefreshItem {
        item: Uuid,
        replace: jellium_model::item::Replace,
        scope: jellium_model::item::Scope,
    },
    /// A configuration page's grant, released when the screen closed.
    PageClosed,
    /// Leaves the route a warning was raised for, discarding the edits.
    LeaveAnyway,
    StayHere,
    PlayerAction(player::Action),
    Reported(Answer<Standing>),
    Ticked,
    ImageLoaded(images::Key, Answer<Vec<u8>>),
    LiveSignalled(live::Signal),
    RemoteAction(remote::Action),
    GroupAction(group::Action),
    GroupTicked,
    GroupItemsLoaded(Answer<Vec<jellyfin_api::types::BaseItemDto>>),
    Resized(Viewport),
    /// A face the origin serves that a glyph on screen needs.
    FontWanted(Served),
    /// The face's woff2 bytes, as the origin answered for them.
    FontFetched(Served, Answer<Vec<u8>>),
    /// A face iced has registered. Registering one bumps the text engine's
    /// version and invalidates every cached paragraph, so this exists to wake
    /// the view for the redraw; which face it was is already settled and read
    /// by nobody.
    FontLoaded,
    Scrolled(window::Scrolled),
    OnNowLoaded(Answer<Vec<Channel>>),
    LiveTvLoaded(Answer<livetv::State>),
    LiveTvAction(livetv::Action),
    GuideFetched(Answer<(guide::Fetched, Vec<Program>)>),
    ProgramLoaded(Answer<program::State>),
    /// The programme now airing on the watched channel.
    AiringFetched(Answer<Option<Program>>),
    /// The server's defaults for a series recording, ready to be edited.
    SeriesPrefilled(Answer<(jellyfin_api::types::SeriesTimerInfoDto, bool)>),
    /// A Live TV write the Jellyfin server answered, named by what it wrote.
    Wrote(crate::error::Operation, Answer<()>),
    LiveTicked,
}

impl Signed {
    fn route(&self) -> Option<&Route> {
        self.history.last()
    }

    fn wanted_images(&self) -> HashSet<images::Key> {
        match &self.view {
            View::Loading => HashSet::new(),
            View::Home(state) => home::images(state, &self.arrangement),
            View::Library(state) => library::images(state),
            View::Detail(state) => detail::images(state),
            View::Search(state) => search::images(state),
            View::Filtered(browse) => crate::screen::browse::images(browse),
            View::Metadata(state) => crate::screen::metadata::images(state),
            View::Collections(state) => crate::screen::collections::listed_images(state),
            View::Collection(state) => crate::screen::collections::images(state),
            View::Playlists(state) => crate::screen::playlists::listed_images(state),
            View::Playlist(state) => crate::screen::playlists::images(state),
            View::LiveTv(state) => livetv::images(state),
            View::Program(state) => program::images(state),
            View::Queue => HashSet::new(),
            View::Remote => crate::screen::remote::images(self.remote.as_ref()),
            View::Dashboard(state) => crate::screen::dashboard::images(state),
            View::Settings(state) => match &state.body {
                crate::screen::settings::Body::Profile(profile) => {
                    crate::screen::settings::profile::images(profile)
                }
                _ => HashSet::new(),
            },
            View::SyncPlay | View::Unavailable => HashSet::new(),
        }
        .into_iter()
        .chain(self.playing.iter().flat_map(player::osd::images))
        .chain(
            matches!(self.view, View::Queue)
                .then(|| {
                    crate::screen::queue::images(
                        self.playing.as_ref(),
                        self.group.as_ref(),
                        self.queue,
                    )
                })
                .unwrap_or_default(),
        )
        .collect()
    }

    fn items_mut(&mut self) -> Vec<&mut jellyfin_api::types::BaseItemDto> {
        match &mut self.view {
            View::Loading => Vec::new(),
            View::Home(state) => state
                .libraries
                .iter_mut()
                .chain(state.continue_watching.iter_mut())
                .chain(state.next_up.iter_mut())
                .collect(),
            View::Library(_) => Vec::new(),
            View::Detail(state) => std::iter::once(&mut state.item)
                .chain(state.children.iter_mut())
                .collect(),
            View::Search(_)
            | View::Filtered(_)
            | View::Collections(_)
            | View::Collection(_)
            | View::Playlists(_)
            | View::Playlist(_)
            | View::Metadata(_) => Vec::new(),
            View::Queue
            | View::Remote
            | View::SyncPlay
            | View::Dashboard(_)
            | View::Settings(_)
            | View::Unavailable => Vec::new(),
            View::LiveTv(state) => match &mut state.body {
                livetv::Body::Recordings(held) => held.recordings.iter_mut().collect(),
                _ => Vec::new(),
            },
            View::Program(_) => Vec::new(),
        }
    }
}

/// The view a route stands in while its screen loads; the queue and remote
/// screens draw from what is already in hand and load nothing, and a Live TV
/// route a session cannot reach stands as the sentence naming which of the two
/// causes it is.
pub fn staged(route: &Route, live_tv: jellium_protocol::LiveTvAccess) -> View {
    match route {
        Route::Queue => View::Queue,
        Route::Remote => View::Remote,
        Route::SyncPlay => View::SyncPlay,
        Route::LiveTv { .. } | Route::Program { .. } => {
            match crate::error::live_tv_denied(live_tv) {
                Some(denied) => {
                    crate::failure::raise(denied);
                    View::Unavailable
                }
                None => View::Loading,
            }
        }
        _ => View::Loading,
    }
}

/// One facet value's items: the header item's name over its own windowed grid.
async fn filtered_load(
    api: Rc<Api>,
    filtered: crate::route::Filtered,
    viewport: Viewport,
    overflow: widget::Overflow,
) -> Answer<crate::screen::browse::Browse> {
    Answer::of(async {
        let heading = match filtered.header {
            Some(header) => api
                .item(header)
                .await
                .or_none(Text::FailureItemUnread)
                .and_then(|item| item.name)
                .unwrap_or_default(),
            None => String::new(),
        };
        let listing = filtered.listing;
        let mut browse = crate::screen::browse::Browse::new(
            window::Id::Browse,
            heading,
            listing.clone(),
            None,
            viewport,
            overflow,
        );
        let answered = api
            .browse(
                filtered.library,
                None,
                &listing,
                0,
                jellium_model::paged::Paged::<jellyfin_api::types::BaseItemDto>::PAGE as i32,
            )
            .await
            .bubbled()?;
        browse.items = jellium_model::paged::Paged::new(answered.total.max(0) as usize);
        browse.filled(0..answered.items.len(), answered.items);
        Ok(browse)
    })
    .await
}

/// Loads the screen `route` names, against the page `signed` is drawn in.
/// A Live TV route a session cannot reach issues no request, and a settings
/// route a session's policy does not admit issues none either.
pub fn load(signed: &Signed, route: &Route, viewport: Viewport) -> Task<Message> {
    let api: Rc<Api> = signed.api.clone();
    let reachable = signed.session.live_tv.allowed();
    let overflow = match signed.session.read_only {
        true => widget::Overflow::Withheld,
        false => widget::Overflow::Offered,
    };
    match route.clone() {
        Route::Home => Task::perform(home::load(api), Message::HomeLoaded),
        Route::Library { id, tab } => Task::perform(
            library::load(api, id, *tab, viewport, overflow),
            Message::LibraryLoaded,
        ),
        Route::Filtered(filtered) => Task::perform(
            filtered_load(api, *filtered, viewport, overflow),
            Message::FilteredLoaded,
        ),
        Route::Collections => Task::perform(
            crate::screen::collections::listed(api, viewport, overflow),
            Message::CollectionsLoaded,
        ),
        Route::Collection { id, listing } => Task::perform(
            crate::screen::collections::load(api, id, *listing, viewport, overflow),
            Message::CollectionLoaded,
        ),
        Route::Playlists => Task::perform(
            crate::screen::playlists::listed(api, viewport, overflow),
            Message::PlaylistsLoaded,
        ),
        Route::Playlist { id } => Task::perform(
            crate::screen::playlists::load(api, id, signed.session.user_id, viewport),
            Message::PlaylistLoaded,
        ),
        Route::Metadata { item, part } => Task::perform(
            crate::screen::metadata::load(api, item, part),
            Message::MetadataLoaded,
        ),
        Route::Detail { id } => Task::perform(detail::load(api, id), Message::DetailLoaded),
        Route::Search { term } => {
            Task::perform(search::load(api, term, viewport), Message::SearchLoaded)
        }
        Route::Queue | Route::Remote | Route::SyncPlay => Task::none(),
        Route::LiveTv { tab } if reachable => Task::perform(
            livetv::load(api, tab, Room::content(viewport)),
            Message::LiveTvLoaded,
        ),
        Route::Program { id } if reachable => {
            Task::perform(program::load(api, id), Message::ProgramLoaded)
        }
        Route::LiveTv { .. } | Route::Program { .. } => Task::none(),
        Route::Dashboard { screen } => {
            let device = signed.session.device.clone();
            Task::perform(
                dashboard::load(api, screen, viewport, device),
                Message::DashboardLoaded,
            )
        }
        Route::Settings { screen } if crate::screen::settings::reaches(&signed.session, screen) => {
            let user = signed.session.user_id;
            let client = signed.session.client.clone();
            Task::perform(
                crate::screen::settings::load(api, screen, user, client),
                Message::SettingsLoaded,
            )
        }
        Route::Settings { .. } => Task::none(),
    }
}

/// Applies a live refresh in place: every rendered copy of a named item takes
/// the new played mark, favourite mark, play count and resume position, and
/// the queue takes them too.
/// Neither the scroll position, the sort order, the paging nor the item
/// playing moves.
fn marked(signed: &mut Signed, items: &[Marked]) -> Task<Message> {
    fn apply(item: &mut jellyfin_api::types::BaseItemDto, mark: &Marked) {
        let mut data = item.user_data.clone().unwrap_or_default();
        data.played = Some(mark.played);
        data.is_favorite = Some(mark.favorite);
        data.play_count = Some(mark.play_count);
        data.playback_position_ticks = Some(mark.position_ticks);
        item.user_data = Some(data);
    }

    for shown in signed.items_mut() {
        if let Some(mark) = items.iter().find(|mark| shown.id == Some(mark.item)) {
            apply(shown, mark);
        }
    }
    if let Some(playing) = signed.playing.as_mut() {
        for queued in playing.queue.items_mut() {
            if let Some(mark) = items.iter().find(|mark| queued.id == Some(mark.item)) {
                apply(queued, mark);
            }
        }
        if let Some(mark) = items.iter().find(|mark| playing.item.id == Some(mark.item)) {
            apply(&mut playing.item, mark);
        }
    }
    Task::none()
}

/// Re-fetches the home screen's server-computed rows when a refresh marks an
/// item played or clears its resume position, and only then.
fn restale(signed: &Signed, items: &[Marked]) -> Task<Message> {
    if !matches!(signed.view, View::Home(_)) {
        return Task::none();
    }
    let stale = items
        .iter()
        .any(|mark| mark.played || mark.position_ticks == 0);
    if !stale {
        return Task::none();
    }
    let api = signed.api.clone();
    Task::perform(home::load(api), Message::HomeLoaded)
}

/// The feeds `route`'s screen consumes.
fn feeds(route: &Route) -> &'static [Feed] {
    match route {
        Route::Remote => &[Feed::Targets],
        Route::SyncPlay => &[Feed::Groups],
        Route::Dashboard { screen } => screen.feeds(),
        _ => &[],
    }
}

/// Sends a `Watch` for every feed `route` consumes and a `Drop` for every feed
/// it does not, so each subscription lives exactly as long as a screen that
/// needs it is open.
fn watching(signed: &Signed, route: &Route) -> Task<Message> {
    let _ = signed;
    let wanted = feeds(route);
    for feed in Feed::ALL {
        live::send(&if wanted.contains(&feed) {
            Report::Watch { feed }
        } else {
            Report::Drop { feed }
        });
    }
    Task::none()
}

/// Announces this browser's identity to the local server and hands the session
/// status it answers to `message`; every read of the session status is this
/// announcement, so the local server holds an identity before it issues an
/// upstream request.
fn announcing(message: fn(Answer<jellium_protocol::SessionStatus>) -> Message) -> Task<Message> {
    let identity = crate::identity::held(&crate::browser::Browser::detect(
        &crate::browser::Runtime::probe(),
    ));
    Task::perform(async move { control::announce(&identity).await }, message)
}

impl Jellium {
    pub fn boot(viewport: Viewport) -> (Jellium, Task<Message>) {
        (
            Jellium {
                stage: Stage::Booting { stalled: false },
                images: Cache::new(),
                viewport,
                failures: crate::failure::Log::default(),
                listing: false,
            },
            Task::batch([
                Task::done(Message::Ready),
                announcing(Message::SessionChecked),
            ]),
        )
    }

    fn signed(&mut self) -> Option<&mut Signed> {
        match &mut self.stage {
            Stage::Signed(signed) => Some(signed),
            _ => None,
        }
    }

    fn route(&self) -> Option<Route> {
        match &self.stage {
            Stage::Signed(signed) => signed.route().cloned(),
            _ => None,
        }
    }

    /// A rejected sign-in keeps the screen that raised it with its typed
    /// fields, clears `working`, and shows the reason under the form.
    fn enter(&mut self, status: SessionStatus) -> Task<Message> {
        let viewport = self.viewport;
        match status {
            SessionStatus::Anonymous { servers, read_only } => {
                let (state, task) = login::enter(servers, read_only);
                self.stage = Stage::Login(state);
                task
            }
            SessionStatus::Login(screen) => {
                let (state, task) = login::entered(screen);
                self.stage = Stage::Login(state);
                task
            }
            SessionStatus::Failed(failure) => {
                crate::failure::raise(crate::error::sign_in_failed(&failure));
                self.quiet_login()
            }
            SessionStatus::Setup(startup) => {
                let (state, task) = crate::screen::setup::enter(startup);
                self.stage = Stage::Setup(state);
                task
            }
            SessionStatus::Authenticated(session) => {
                let api = Rc::new(Api::new(session.user_id));
                let detected = crate::browser::Browser::detect(&crate::browser::Runtime::probe());
                self.stage = Stage::Signed(Box::new(Signed {
                    session,
                    api,
                    history: vec![Route::Home],
                    view: View::Loading,
                    overflow: None,
                    foreign: crate::images::Foreign::new(),
                    playing: None,
                    pending: None,
                    device: Device::load(),
                    preferences: jellium_model::prefs::Bag::missing(),
                    held: jellium_model::prefs::Held::default(),
                    configuration: jellium_model::form::Form::of(serde_json::Value::Null),
                    arrangement: crate::screen::home::Arrangement::of(
                        &jellium_model::form::Form::of(serde_json::Value::Null),
                        jellium_model::prefs::Held::default(),
                    ),
                    browser: detected,
                    live: live::Link::default(),
                    targets: Vec::new(),
                    remote: None,
                    message: None,
                    restart_required: false,
                    picker: None,
                    server_changed: false,
                    leaving: None,
                    stopped: None,
                    groups: Vec::new(),
                    group: None,
                    queue: window::Window::new(
                        window::Id::Queue,
                        crate::screen::queue::ROW.height().drawn(),
                        self.viewport.canvas().height(),
                    ),
                }));
                live::connect();
                let Stage::Signed(signed) = &self.stage else {
                    return Task::none();
                };
                let api = signed.api.clone();
                let client = signed.session.client.clone();
                let user = signed.session.user_id;
                Task::batch([
                    Task::perform(
                        async move {
                            Answer::of(async {
                                let bag = api.preferences(&client).await.bubbled()?;
                                let configuration = api.user_configuration(user).await.bubbled()?;
                                Ok((bag, configuration))
                            })
                            .await
                        },
                        Message::PreferencesLoaded,
                    ),
                    load(signed, &Route::Home, viewport),
                ])
            }
        }
    }

    /// Drives one login-stage control: every act that reaches the local server
    /// sets `working`, and every act clears what the last one left on screen.
    fn login_act(&mut self, action: login::Action) -> Task<Message> {
        let Stage::Login(state) = &mut self.stage else {
            return Task::none();
        };
        if let login::Action::Edited(edit) = action {
            match edit {
                login::Edit::Url(value) => state.add.url = value,
                login::Edit::Username(value) => {
                    state.credentials.username = value;
                    state.credentials.picked = None;
                }
                login::Edit::Password(value) => state.credentials.password = value,
                login::Edit::ResetUsername(value) => state.reset.username = value,
                login::Edit::Pin(value) => state.reset.pin = value,
            }
            return Task::none();
        }
        if state.working {
            return Task::none();
        }
        state.told = None;
        let target = state.target();

        match action {
            login::Action::Edited(_) => Task::none(),
            login::Action::Add => {
                state.screen = jellium_model::login::Screen::Add;
                Task::none()
            }
            login::Action::AddSubmit => {
                state.working = true;
                Task::perform(
                    control::add_server(state.add.url.trim().to_string()),
                    Message::LoginAnswered,
                )
            }
            login::Action::Select { server } => {
                state.working = true;
                Task::perform(control::select_server(server), Message::LoginAnswered)
            }
            login::Action::Remove { server } => {
                state.working = true;
                Task::perform(control::remove_server(server), Message::ServerRemoved)
            }
            login::Action::Show(prompt) => {
                state.credentials.prompt = prompt;
                Task::none()
            }
            login::Action::Pick { user, name } => {
                state.credentials.picked = Some(user);
                state.credentials.username = name;
                state.credentials.password.clear();
                state.credentials.prompt = jellium_model::login::Prompt::Manual;
                Task::none()
            }
            login::Action::Submit => {
                state.working = true;
                let credentials = jellium_protocol::Credentials {
                    username: state.credentials.username.clone(),
                    password: state.credentials.password.clone(),
                };
                Task::perform(
                    control::sign_in(target, credentials),
                    Message::LoginAnswered,
                )
            }
            login::Action::QuickConnect | login::Action::QuickConnectRetry => {
                state.screen = jellium_model::login::Screen::QuickConnect;
                state.quick_connect = crate::screen::login::quickconnect::State::default();
                state.working = true;
                Task::perform(
                    control::quick_connect_initiate(target),
                    Message::QuickConnectInitiated,
                )
            }
            login::Action::Reset => {
                state.screen = jellium_model::login::Screen::Reset;
                state.reset = crate::screen::login::reset::State::default();
                Task::none()
            }
            login::Action::ResetSubmit => {
                state.working = true;
                let username = state.reset.username.clone();
                Task::perform(
                    control::forgot_password(target, username),
                    Message::ResetAnswered,
                )
            }
            login::Action::PinSubmit => {
                state.working = true;
                let pin = state.reset.pin.clone();
                Task::perform(control::redeem_pin(target, pin), Message::PinRedeemed)
            }
            login::Action::Back => self.login_back(),
        }
    }

    /// Back leaves the shown screen for the one it opened from, releasing the
    /// login target when it leaves the last screen holding one.
    fn login_back(&mut self) -> Task<Message> {
        let Stage::Login(state) = &mut self.stage else {
            return Task::none();
        };
        let Some(back) = state.screen.back() else {
            return Task::none();
        };
        let target = state.target();
        let released = state.screen.targeted() && !back.targeted();
        let abandoning = state.screen == jellium_model::login::Screen::QuickConnect;
        state.screen = back;
        if released {
            state.target = None;
            state.images.clear();
            state.credentials = crate::screen::login::credentials::State::default();
        }

        let mut running = Vec::new();
        if abandoning {
            let target = target.clone();
            running.push(Task::perform(
                control::quick_connect_abandon(target),
                |()| Message::Quieted,
            ));
        }
        if released {
            running.push(Task::perform(control::leave_login(target), |()| {
                Message::Quieted
            }));
            running.push(Task::perform(control::servers(), Message::ServersListed));
        }
        Task::batch(running)
    }

    /// A removal that landed re-reads the list; one the Jellyfin server would
    /// not revoke names that beside it.
    fn server_removed(&mut self, removed: Answer<jellium_protocol::Removed>) -> Task<Message> {
        if !matches!(self.stage, Stage::Login(_)) {
            return Task::none();
        }
        let quieted = self.quiet_login();
        let Some(removed) = removed.or_none(Text::FailureServerRemove) else {
            return quieted;
        };
        if removed == jellium_protocol::Removed::DeletedUnrevoked {
            crate::failure::raise(crate::error::told(Text::FailureRemoveUnrevoked));
        }
        Task::batch([
            quieted,
            Task::perform(control::servers(), Message::ServersListed),
        ])
    }

    /// One poll's answer: an installed session enters it, and every ending
    /// names itself on the Quick Connect screen.
    fn quick_connect_polled(
        &mut self,
        polled: jellium_protocol::QuickConnectState,
    ) -> Task<Message> {
        if let jellium_protocol::QuickConnectState::Signed(session) = polled {
            return self.enter(SessionStatus::Authenticated(session));
        }
        let Stage::Login(state) = &mut self.stage else {
            return Task::none();
        };
        match polled {
            jellium_protocol::QuickConnectState::Pending => {
                state.quick_connect.standing = Some(jellium_model::quickconnect::SignIn::Pending);
            }
            jellium_protocol::QuickConnectState::Expired => {
                state.quick_connect.standing = Some(jellium_model::quickconnect::SignIn::Expired);
                crate::failure::raise(crate::error::told(Text::FailureQuickConnectSignInExpired));
            }
            jellium_protocol::QuickConnectState::Disabled => {
                state.quick_connect.standing = Some(jellium_model::quickconnect::SignIn::Disabled);
                crate::failure::raise(crate::error::told(Text::FailureQuickConnectSignInDisabled));
                if let Some(screen) = &mut state.target {
                    screen.quick_connect = false;
                }
            }
            jellium_protocol::QuickConnectState::Signed(_) => {}
        }
        Task::none()
    }

    /// Puts the login stage back on screen with nothing in flight, which is
    /// what every refused login-stage answer leaves behind, a transport
    /// trouble included.
    /// It is the one owner of `working`: no other site sets it false.
    /// A stage it had to build is paired with the saved-server read, so the
    /// list is never empty for want of asking.
    fn quiet_login(&mut self) -> Task<Message> {
        let mut reading = Task::none();
        if !matches!(self.stage, Stage::Login(_)) {
            let (state, task) = login::enter(Vec::new(), false);
            self.stage = Stage::Login(state);
            reading = Task::batch([
                task,
                Task::perform(control::servers(), Message::ServersListed),
            ]);
        }
        if let Stage::Login(state) = &mut self.stage {
            state.working = false;
        }
        reading
    }

    /// Issues one image fetch; empty when no session holds an `Api`.
    fn fetch_image(&self, key: images::Key) -> Task<Message> {
        let Stage::Signed(signed) = &self.stage else {
            return Task::none();
        };
        let api = signed.api.clone();
        let fill = crate::images::card(key.kind).image_width(self.viewport, crate::page::screen());
        let url = api.image_url(key, fill);
        Task::perform(async move { api.image(url).await }, move |result| {
            Message::ImageLoaded(key, result)
        })
    }

    /// Warns before leaving a screen holding unsaved edits, naming what is
    /// lost.
    /// A demotion that ejects the user issues no warning, because it does not
    /// go through here.
    fn leaving(signed: &mut Signed, route: Route) -> Option<Task<Message>> {
        let unsaved = dashboard::dirty(&signed.view)
            || (crate::screen::settings::dirty(&signed.view)
                && (signed.preferences.dirty() || signed.configuration.dirty()));
        if !unsaved {
            return None;
        }
        signed.leaving = Some(route);
        Some(Task::none())
    }

    fn navigate(&mut self, route: Route) -> Task<Message> {
        let viewport = self.viewport;
        let Some(signed) = self.signed() else {
            return Task::none();
        };
        if signed.route() == Some(&route) {
            return Task::none();
        }
        if let Some(waiting) = Self::leaving(signed, route.clone()) {
            return waiting;
        }
        signed.history.push(route.clone());
        signed.view = staged(&route, signed.session.live_tv);
        let telling = watching(signed, &route);
        Task::batch([telling, load(signed, &route, viewport)])
    }

    fn replace(&mut self, route: Route) -> Task<Message> {
        let viewport = self.viewport;
        let Some(signed) = self.signed() else {
            return Task::none();
        };
        signed.history.pop();
        signed.history.push(route.clone());
        signed.view = staged(&route, signed.session.live_tv);
        let telling = watching(signed, &route);
        Task::batch([telling, load(signed, &route, viewport)])
    }

    fn settle(&mut self) {
        let Some(signed) = self.signed() else {
            return;
        };
        signed.arrangement =
            crate::screen::home::Arrangement::of(&signed.configuration, signed.held);
        let wanted = signed.wanted_images();
        self.images.retain(&wanted);
    }

    fn fetch_images(&mut self) -> Task<Message> {
        let Some(signed) = self.signed() else {
            return Task::none();
        };
        let wanted = signed.wanted_images();
        let started: Vec<_> = wanted
            .into_iter()
            .filter(|key| self.images.begin(*key))
            .collect();

        Task::batch(started.into_iter().map(|key| self.fetch_image(key)))
    }

    /// Installs `view`, releasing the grant the screen it replaces held open.
    /// The file input stands while a screen offering an upload is shown, and is
    /// unmounted the moment one is not.
    fn picking(&mut self) {
        let Some(signed) = self.signed() else {
            return;
        };
        let wanted = match &signed.view {
            View::Settings(state) => {
                matches!(state.body, crate::screen::settings::Body::Profile(_))
            }
            View::Dashboard(state) => {
                matches!(state.body, crate::screen::dashboard::Body::User(_))
            }
            View::Metadata(_) => true,
            _ => false,
        };
        if wanted == signed.picker.is_some() {
            return;
        }
        signed.picker = wanted
            .then(|| {
                crate::overlay::Mounted::new(&crate::overlay::Wanted {
                    id: crate::overlay::Id::File,
                    kind: crate::overlay::Kind::File,
                    stacking: crate::overlay::Stacking::Above,
                    pointer: false,
                    source: None,
                    sandbox: None,
                    accept: Some(crate::overlay::FILE_ACCEPT),
                    hidden: true,
                })
            })
            .flatten();
    }

    fn loaded(&mut self, view: View) -> Task<Message> {
        let mut released = Task::none();
        if let Some(signed) = self.signed() {
            if let Some(grant) = dashboard::held_grant(&signed.view) {
                released = Task::perform(crate::screen::dashboard::page::close(grant), |()| {
                    Message::PageClosed
                });
            }
            signed.view = view;
        }
        self.picking();
        self.settle();
        Task::batch([released, self.fetch_images()])
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        let task = self.updating(message);
        if let Some(signed) = self.signed() {
            signed.arrangement =
                crate::screen::home::Arrangement::of(&signed.configuration, signed.held);
        }
        task
    }

    fn updating(&mut self, message: Message) -> Task<Message> {
        let viewport = self.viewport;
        match message {
            Message::Failed(failure) => {
                self.failures.took(failure.clone());
                if failure.weight == crate::failure::Weight::Fatal {
                    live::disconnect();
                    self.stage = Stage::Lost(failure);
                }
                Task::none()
            }
            Message::Quieted => Task::none(),
            Message::SessionRechecked => {
                self.stage = Stage::Booting { stalled: false };
                announcing(Message::SessionChecked)
            }
            Message::Unchanged => Task::none(),
            Message::SignInAgain => {
                let (state, task) = login::enter(Vec::new(), false);
                self.stage = Stage::Login(state);
                Task::batch([
                    task,
                    Task::perform(control::servers(), Message::ServersListed),
                ])
            }
            Message::FailureDismissed => {
                self.failures.dismiss();
                Task::none()
            }
            Message::FailuresOpened => {
                self.listing = true;
                Task::none()
            }
            Message::FailuresClosed => {
                self.listing = false;
                Task::none()
            }
            Message::Ready => {
                boot::hide_static_page();
                Task::none()
            }
            Message::SessionChecked(answered) => {
                match answered.or_none(Text::FailureSessionUnread) {
                    Some(status) => self.enter(status),
                    None => {
                        if matches!(self.stage, Stage::Booting { .. }) {
                            self.stage = Stage::Booting { stalled: true };
                        }
                        Task::none()
                    }
                }
            }
            Message::LoginAction(action) => self.login_act(action),
            Message::LoginAnswered(answered) => {
                let Some(status) = answered.or_none(Text::FailureSessionUnread) else {
                    return self.quiet_login();
                };
                self.enter(status)
            }
            Message::ServersListed(listed) => {
                let Some(servers) = listed.or_none(Text::FailureServersUnread) else {
                    return Task::none();
                };
                if let Stage::Login(state) = &mut self.stage {
                    state.servers = servers;
                }
                Task::none()
            }
            Message::ServerRemoved(removed) => self.server_removed(removed),
            Message::QuickConnectInitiated(initiated) => {
                if !matches!(self.stage, Stage::Login(_)) {
                    return Task::none();
                }
                let quieted = self.quiet_login();
                let Some(code) = initiated.or_none(Text::FailureQuickConnectUnread) else {
                    return quieted;
                };
                let Stage::Login(state) = &mut self.stage else {
                    return quieted;
                };
                state.quick_connect.code = Some(code.code);
                state.quick_connect.standing = Some(jellium_model::quickconnect::SignIn::Pending);
                quieted
            }
            Message::QuickConnectPolled(polled) => {
                let Some(polled) = polled.or_none(Text::FailureQuickConnectUnread) else {
                    return Task::none();
                };
                self.quick_connect_polled(polled)
            }
            Message::QuickConnectTicked => {
                let Stage::Login(state) = &self.stage else {
                    return Task::none();
                };
                Task::perform(
                    control::quick_connect_poll(state.target()),
                    Message::QuickConnectPolled,
                )
            }
            Message::ResetAnswered(answered) => {
                if !matches!(self.stage, Stage::Login(_)) {
                    return Task::none();
                }
                let quieted = self.quiet_login();
                let Some(answered) = answered.or_none(Text::FailureResetPinUnread) else {
                    return quieted;
                };
                let Stage::Login(state) = &mut self.stage else {
                    return quieted;
                };
                match answered {
                    jellium_protocol::ResetAnswer::PinWritten { pin_file, expires } => {
                        state.reset.answered = Some(jellium_model::login::Reset::PinWritten);
                        state.reset.pin_file = Some(pin_file);
                        state.reset.expires = expires;
                    }
                    jellium_protocol::ResetAnswer::ContactAdministrator => {
                        state.reset.answered =
                            Some(jellium_model::login::Reset::ContactAdministrator);
                    }
                    jellium_protocol::ResetAnswer::InNetworkRequired => {
                        state.reset.answered = Some(jellium_model::login::Reset::InNetworkRequired);
                    }
                }
                quieted
            }
            Message::PinRedeemed(redeemed) => {
                if !matches!(self.stage, Stage::Login(_)) {
                    return Task::none();
                }
                let quieted = self.quiet_login();
                let Some(redeemed) = redeemed.or_none(Text::FailureResetPinUnread) else {
                    return quieted;
                };
                match redeemed {
                    jellium_protocol::PinOutcome::Cleared { users } => {
                        let Stage::Login(state) = &mut self.stage else {
                            return Task::none();
                        };
                        state.told = Some(strings::format(
                            Text::LoginResetCleared,
                            &[&users.join(", ")],
                        ));
                    }
                    jellium_protocol::PinOutcome::Refused => {
                        crate::failure::raise(crate::error::told(Text::FailureResetPinRefused));
                    }
                }
                quieted
            }
            Message::PublicImageLoaded(user, loaded) => {
                let Some(bytes) = loaded.or_none(Text::FailureUserImageUnread) else {
                    return Task::none();
                };
                if let Stage::Login(state) = &mut self.stage {
                    state
                        .images
                        .insert(user, iced::widget::image::Handle::from_bytes(bytes));
                }
                Task::none()
            }
            Message::SwitchPressed => {
                if self.signed().is_none() {
                    return Task::none();
                }
                live::disconnect();
                self.images.retain(&HashSet::new());
                Task::perform(control::switch_server(), Message::LoginAnswered)
            }
            Message::LogoutPressed => Task::perform(control::logout(), Message::LoggedOut),
            Message::LoggedOut(answered) => {
                let Some(()) = answered.or_none(Text::FailureSignOut) else {
                    return Task::none();
                };
                live::disconnect();
                self.images.retain(&HashSet::new());
                announcing(Message::LoginAnswered)
            }
            Message::SetupAction(action) => match &mut self.stage {
                Stage::Setup(state) => crate::screen::setup::act(state, action),
                _ => Task::none(),
            },
            Message::SetupLoaded(answered) => {
                let Some((step, body)) = answered.or_refused(&crate::screen::setup::step_write())
                else {
                    if let Stage::Setup(state) = &mut self.stage {
                        crate::screen::setup::refused(state);
                    }
                    return Task::none();
                };
                if let Stage::Setup(state) = &mut self.stage {
                    crate::screen::setup::stepped(state, step, body);
                }
                Task::none()
            }
            Message::SetupAdvanced(answered) => {
                if answered
                    .or_refused(&crate::screen::setup::step_write())
                    .is_none()
                {
                    if let Stage::Setup(state) = &mut self.stage {
                        crate::screen::setup::refused(state);
                    }
                    return Task::none();
                }
                match &mut self.stage {
                    Stage::Setup(state) => crate::screen::setup::advanced(state),
                    _ => Task::none(),
                }
            }
            Message::SetupWrote(answered) => {
                if answered
                    .or_refused(&crate::screen::setup::step_write())
                    .is_none()
                {
                    if let Stage::Setup(state) = &mut self.stage {
                        crate::screen::setup::refused(state);
                    }
                    return Task::none();
                }
                match &mut self.stage {
                    Stage::Setup(state) => crate::screen::setup::reread(state),
                    _ => Task::none(),
                }
            }
            Message::SetupBrowsed(answered) => {
                let Some((path, entries)) =
                    answered.or_refused(&crate::screen::setup::step_write())
                else {
                    if let Stage::Setup(state) = &mut self.stage {
                        crate::screen::setup::refused(state);
                    }
                    return Task::none();
                };
                if let Stage::Setup(state) = &mut self.stage {
                    crate::screen::setup::browsed(state, path, entries);
                }
                Task::none()
            }
            Message::SetupCompleted(answered) => {
                let Some(status) = answered.or_none(Text::FailureSetupUnfinished) else {
                    return Task::none();
                };
                self.enter(status)
            }
            Message::SetupLeft(answered) => {
                let Some(()) = answered.or_none(Text::FailureSetupStep) else {
                    return Task::none();
                };
                announcing(Message::LoginAnswered)
            }
            Message::Navigated(route) => self.navigate(route),
            Message::WentBack => {
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                if signed.history.len() < 2 {
                    return Task::none();
                }
                signed.history.pop();
                let route = signed.route().cloned().unwrap_or(Route::Home);
                signed.view = staged(&route, signed.session.live_tv);
                let telling = watching(signed, &route);
                Task::batch([telling, load(signed, &route, viewport)])
            }
            Message::HomeLoaded(answered) => {
                let Some(state) = answered.or_none(Text::FailureHomeUnread) else {
                    return Task::none();
                };
                let showing = self.loaded(View::Home(Box::new(state)));
                let Some(signed) = self.signed() else {
                    return showing;
                };
                if !signed.session.live_tv.allowed() {
                    return showing;
                }
                let api = signed.api.clone();
                Task::batch([
                    showing,
                    Task::perform(home::on_now(api), Message::OnNowLoaded),
                ])
            }
            Message::CollectionsLoaded(answered) => {
                let Some(state) = answered.or_none(Text::FailureCollectionsUnread) else {
                    return Task::none();
                };
                let showing = self.loaded(View::Collections(Box::new(state)));
                Task::batch([showing, self.browse_fetch()])
            }
            Message::CollectionLoaded(answered) => {
                let Some(state) = answered.or_none(Text::FailureCollectionUnread) else {
                    return Task::none();
                };
                let showing = self.loaded(View::Collection(Box::new(state)));
                Task::batch([showing, self.browse_fetch()])
            }
            Message::PlaylistsLoaded(answered) => {
                let Some(state) = answered.or_none(Text::FailurePlaylistsUnread) else {
                    return Task::none();
                };
                let showing = self.loaded(View::Playlists(Box::new(state)));
                Task::batch([showing, self.browse_fetch()])
            }
            Message::PlaylistLoaded(answered) => {
                let Some(state) = answered.or_none(Text::FailurePlaylistUnread) else {
                    return Task::none();
                };
                let showing = self.loaded(View::Playlist(Box::new(state)));
                Task::batch([showing, self.entries_fetch()])
            }
            Message::MetadataLoaded(answered) => {
                let Some(state) = answered.or_none(Text::FailureMetadataUnread) else {
                    return Task::none();
                };
                let showing = self.loaded(View::Metadata(Box::new(state)));
                Task::batch([showing, self.fetch_foreign()])
            }
            Message::MetadataAction(action) => {
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                crate::screen::metadata::act(signed, action)
            }
            Message::MetadataUploadRequested => Task::none(),
            Message::Identified(answered) => {
                let Some(candidates) = answered.or_none(Text::FailureMatchesUnread) else {
                    return Task::none();
                };
                if let Some(View::Metadata(state)) = self.signed().map(|signed| &mut signed.view) {
                    state.identify.candidates = candidates;
                    state.identify.applying = None;
                }
                self.fetch_foreign()
            }
            Message::RemoteImagesLoaded(answered) => {
                let Some(remote) = answered.or_none(Text::FailureImageProvidersUnread) else {
                    return Task::none();
                };
                if let Some(View::Metadata(state)) = self.signed().map(|signed| &mut signed.view) {
                    state.artwork.remote = remote;
                }
                self.fetch_foreign()
            }
            Message::ForeignLoaded(handle, loaded) => {
                let Some(bytes) = loaded.or_none(Text::FailureImageUnread) else {
                    if let Some(signed) = self.signed() {
                        signed.foreign.missing(&handle);
                    }
                    return Task::none();
                };
                if let Some(signed) = self.signed() {
                    signed.foreign.store(&handle, bytes);
                }
                Task::none()
            }
            Message::TrickplayLoaded(loaded) => {
                let Some(held) = loaded.or_none(Text::FailureTrickplayUnread) else {
                    return Task::none();
                };
                if let Some(playing) = self.signed().and_then(|signed| signed.playing.as_mut()) {
                    playing.trickplay = held;
                }
                Task::none()
            }
            Message::TileLoaded(tile, loaded) => {
                let Some(bytes) = loaded.or_none(Text::FailureTrickplayUnread) else {
                    return Task::none();
                };
                let Some(playing) = self.signed().and_then(|signed| signed.playing.as_mut()) else {
                    return Task::none();
                };
                let source = playing.plan.media_source.clone();
                let Some(described) = playing
                    .trickplay
                    .width_for(&source, card::Fill::of(space::preview(viewport)))
                else {
                    return Task::none();
                };
                if let Some(preview) = playing.preview.as_mut() {
                    preview.frame = crate::player::trickplay::cropped(&bytes, described, tile);
                }
                Task::none()
            }
            Message::PreviewSettled => {
                let chapter = self.chapter_preview();
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                if let Some(playing) = signed.playing.as_mut()
                    && let Some(preview) = playing.preview.as_mut()
                    && preview.frame.is_none()
                {
                    preview.frame = chapter;
                }
                player::act(signed, player::Action::Settled, viewport)
            }
            Message::OverflowAction(action) => {
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                crate::screen::overflow::act(signed, action)
            }
            Message::FilingLoaded(answered) => {
                let Some(offered) = answered.or_none(Text::FailureFilingUnread) else {
                    return Task::none();
                };
                if let Some(filing) = self
                    .signed()
                    .and_then(|signed| signed.overflow.as_mut())
                    .and_then(|open| open.filing.as_mut())
                {
                    filing.offered = offered;
                }
                Task::none()
            }
            Message::CollectionAction(action) => {
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                crate::screen::collections::act(signed, action)
            }
            Message::PlaylistAction(action) => {
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                crate::screen::playlists::act(signed, action)
            }
            Message::PlaylistPaged(page, answered) => {
                let Some((entries, total)) = answered.or_none(Text::FailurePlaylistUnread) else {
                    return Task::none();
                };
                let Some(state) = self.playlisting() else {
                    return Task::none();
                };
                if state.entries.len() != total {
                    state.entries = jellium_model::paged::Paged::new(total);
                }
                state.entries.filled(page, entries);
                self.settle();
                Task::batch([self.entries_fetch(), self.fetch_images()])
            }
            Message::FilteredLoaded(answered) => {
                let Some(browse) = answered.or_none(Text::FailureBrowseUnread) else {
                    return Task::none();
                };
                let showing = self.loaded(View::Filtered(Box::new(browse)));
                Task::batch([showing, self.browse_fetch()])
            }
            Message::HubPaged(page, answered) => {
                let Some((entries, total)) = answered.or_none(Text::FailureBrowseUnread) else {
                    return Task::none();
                };
                let Some(hub) = self.hubbing() else {
                    return Task::none();
                };
                if hub.entries.len() != total {
                    hub.entries = jellium_model::paged::Paged::new(total);
                }
                hub.entries.filled(page, entries);
                self.settle();
                Task::batch([self.hub_fetch(), self.fetch_images()])
            }
            Message::LibraryLoaded(answered) => {
                let Some(state) = answered.or_none(Text::FailureLibraryUnread) else {
                    return Task::none();
                };
                self.loaded(View::Library(Box::new(state)))
            }
            Message::DetailLoaded(answered) => {
                let Some(state) = answered.or_none(Text::FailureItemUnread) else {
                    return Task::none();
                };
                self.loaded(View::Detail(Box::new(state)))
            }
            Message::SearchLoaded(answered) => {
                let Some(mut state) = answered.or_none(Text::FailureSearchUnread) else {
                    return Task::none();
                };
                if let Some(View::Search(open)) = self.signed().map(|signed| &signed.view)
                    && open.term == state.term
                {
                    state.rested(open);
                }
                self.loaded(View::Search(Box::new(state)))
            }
            Message::BrowseAction(action) => self.browse_action(action),
            Message::BrowsePaged(page, answered) => {
                let Some((items, total)) = answered.or_none(Text::FailureBrowseUnread) else {
                    return Task::none();
                };
                let Some(browse) = self.browsing_mut() else {
                    return Task::none();
                };
                if browse.items.len() != total {
                    browse.items = jellium_model::paged::Paged::new(total);
                }
                browse.filled(page, items);
                self.settle();
                Task::batch([self.browse_fetch(), self.fetch_images()])
            }
            Message::Jumped(answered) => {
                let Some(index) = answered.or_none(Text::FailureBrowseUnread) else {
                    return Task::none();
                };
                let Some(browse) = self.browsing_mut() else {
                    return Task::none();
                };
                let offset = browse.grid.resting(index);
                let id = browse.grid.id();
                browse.grid.moved(offset);
                Task::batch([window::resting(id, offset), self.browse_fetch()])
            }
            Message::SearchEdited(term) => {
                if let Some(Signed {
                    view: View::Search(state),
                    ..
                }) = self.signed()
                {
                    state.term = term;
                }
                Task::none()
            }
            Message::SearchSubmitted => {
                let term = match self.signed() {
                    Some(Signed {
                        view: View::Search(state),
                        ..
                    }) => state.term.clone(),
                    _ => return Task::none(),
                };
                self.replace(Route::Search { term })
            }
            Message::PlayedToggled(id, mark) => {
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                let api = signed.api.clone();
                Task::perform(
                    async move { api.set_played(id, mark.set()).await },
                    move |r| Message::UserDataUpdated(id, r),
                )
            }
            Message::FavoriteToggled(id, mark) => {
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                let api = signed.api.clone();
                Task::perform(
                    async move { api.set_favorite(id, mark.set()).await },
                    move |r| Message::UserDataUpdated(id, r),
                )
            }
            Message::UserDataUpdated(id, answered) => {
                let Some(data) = answered.or_none(Text::FailureMarkUnwritten) else {
                    return Task::none();
                };
                if let Some(signed) = self.signed() {
                    for item in signed.items_mut() {
                        if item.id == Some(id) {
                            item.user_data = Some(data.clone());
                        }
                    }
                }
                Task::none()
            }
            Message::PlayPressed(intent) => {
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                let api = signed.api.clone();
                Task::perform(player::resolve(api, intent), Message::Resolved)
            }
            Message::Resolved(answered) => {
                let Some(start) = answered.or_none(Text::FailurePlaybackUnread) else {
                    return Task::none();
                };
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                player::started(signed, start, viewport)
            }
            Message::GroupResolved(resolved) => {
                let start = resolved.or_none(Text::FailureGroupUnresolved);
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                match start {
                    Some(start) => player::begin(signed, start, viewport),
                    None => group::unplayable(signed, viewport),
                }
            }
            Message::Planned(answered) => {
                let outcome = player::planned(answered);
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                match outcome {
                    player::Outcome::Plan(plan) => {
                        live::send(&Report::Playing {
                            play_session: plan.play_session.clone(),
                        });
                        let task = player::installed(signed, *plan, viewport);
                        let reading = self.read_trickplay();
                        let settle = self.fetch_images();
                        Task::batch([task, reading, settle])
                    }
                    player::Outcome::Unchanged => player::unchanged(signed),
                    player::Outcome::Unplanned => player::unplanned(signed, viewport),
                }
            }
            Message::Overlaid(raised) => {
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                if let Some(chosen) = crate::overlay::chosen(&raised) {
                    return match &signed.view {
                        View::Settings(_) => crate::screen::settings::chosen(signed, &chosen),
                        View::Dashboard(_) => dashboard::chosen(signed, &chosen),
                        View::Metadata(_) => crate::screen::metadata::chosen(signed, &chosen),
                        _ => Task::none(),
                    };
                }
                if raised.id == crate::overlay::Id::PluginPage {
                    return dashboard::act(
                        signed,
                        crate::screen::dashboard::Action::Bridged(raised.payload),
                        viewport,
                    );
                }
                match crate::player::element::read(&raised) {
                    Some(event) => player::event(signed, event, viewport),
                    None => Task::none(),
                }
            }
            Message::PreferencesLoaded(answered) => {
                let Some((bag, configuration)) = answered.or_none(Text::FailurePreferencesUnread)
                else {
                    return Task::none();
                };
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                signed.preferences = bag;
                signed.held = signed.preferences.held();
                signed.configuration = jellium_model::form::Form::of(configuration);
                let read_only = signed.session.read_only;
                let migration = jellium_model::prefs::migration(
                    crate::prefs::Device::parked(),
                    &signed.preferences,
                    read_only,
                );
                crate::prefs::Device::settle(jellium_model::prefs::Parked::of(
                    &signed.preferences,
                    read_only,
                ));
                let migrated = match migration {
                    jellium_model::prefs::Migration::Skipped => Task::none(),
                    jellium_model::prefs::Migration::Carried { quality } => {
                        let mut carried = signed.preferences.clone();
                        carried.edit(jellium_model::prefs::Held {
                            quality,
                            ..signed.held
                        });
                        let api = signed.api.clone();
                        let client = signed.session.client.clone();
                        let record = carried.written();
                        Task::perform(
                            async move { api.save_preferences(&client, &record).await },
                            move |result| Message::Migrated(quality, result),
                        )
                    }
                };
                let Some(route) = self.route() else {
                    return migrated;
                };
                let Some(signed) = self.signed() else {
                    return migrated;
                };
                Task::batch([migrated, load(signed, &route, viewport)])
            }
            Message::Migrated(quality, answered) => {
                let Some(()) = answered.or_none(Text::FailurePreferencesUnmigrated) else {
                    return Task::none();
                };
                if let Some(signed) = self.signed() {
                    signed.preferences.carried(quality);
                    signed.held = signed.preferences.held();
                    crate::prefs::Device::settle(jellium_model::prefs::Parked::of(
                        &signed.preferences,
                        signed.session.read_only,
                    ));
                }
                Task::none()
            }
            Message::SettingsLoaded(answered) => {
                let Some((screen, loaded)) = answered.or_none(Text::FailureSettingsUnread) else {
                    return Task::none();
                };
                self.loaded(View::Settings(Box::new(
                    crate::screen::settings::State::of(screen, loaded),
                )))
            }
            Message::SettingsAction(action) => {
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                crate::screen::settings::act(signed, action)
            }
            Message::SettingsWrote(wrote, answered) => {
                if answered.or_refused(&wrote).is_none() {
                    return Task::none();
                }
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                let Some(route) = signed.history.last().cloned() else {
                    return Task::none();
                };
                load(signed, &route, viewport)
            }
            Message::SettingsSaved(wrote, answered) => {
                if answered.or_refused(&wrote).is_none() {
                    return Task::none();
                }
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                crate::screen::settings::saved(signed);
                let Some(route) = signed.history.last().cloned() else {
                    return Task::none();
                };
                load(signed, &route, viewport)
            }
            Message::QuickConnected(answered) => {
                let wrote = crate::error::Wrote {
                    operation: crate::error::Operation::QuickConnect,
                    object: String::new(),
                };
                let Some(outcome) = answered.or_refused(&wrote) else {
                    return Task::none();
                };
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                {
                    if let View::Settings(state) = &mut signed.view
                        && let crate::screen::settings::Body::QuickConnect(quick) = &mut state.body
                    {
                        if outcome == jellium_model::quickconnect::Outcome::Authorized
                            && !quick.authorized.contains(&quick.code)
                        {
                            quick.authorized.push(quick.code.clone());
                        }
                        quick.outcome = Some(outcome);
                    }
                    if outcome != jellium_model::quickconnect::Outcome::Authorized {
                        crate::failure::raise(crate::error::quick_connect_refused(outcome));
                    }
                    Task::none()
                }
            }
            Message::ConfigurationRefreshed(answered) => {
                let Some(read) = answered.or_none(Text::FailureConfigurationUnread) else {
                    return Task::none();
                };
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                signed.configuration.refreshed(read);
                signed.server_changed = true;
                Task::none()
            }
            Message::DashboardLoaded(answered) => {
                let Some((screen, loaded)) = answered.or_none(Text::FailureDashboardUnread) else {
                    return Task::none();
                };
                self.loaded(View::Dashboard(Box::new(
                    crate::screen::dashboard::State::of(screen, loaded),
                )))
            }
            Message::DashboardAction(action) => {
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                dashboard::act(signed, action, viewport)
            }
            Message::DashboardSaved(wrote, answered) => {
                if answered.or_refused(&wrote).is_none() {
                    return Task::none();
                }
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                dashboard::saved(signed);
                Task::none()
            }
            Message::DashboardWrote(wrote, answered) => {
                if answered.or_refused(&wrote).is_none() {
                    return Task::none();
                }
                {
                    let Some(signed) = self.signed() else {
                        return Task::none();
                    };
                    match wrote.operation {
                        // An administrator stopped the server from this screen; the
                        // text names that cause rather than an unreachable server.
                        crate::error::Operation::Restart => {
                            signed.stopped = Some(Stopped::Restarted);
                        }
                        crate::error::Operation::Shutdown => {
                            signed.stopped = Some(Stopped::ShutDown);
                            live::disconnect();
                        }
                        crate::error::Operation::OwnDeviceDelete => {
                            live::disconnect();
                            crate::failure::raise(crate::error::own_device_deleted());
                            return Task::none();
                        }
                        _ => {}
                    }
                    Task::none()
                }
            }
            Message::Bridged(call, value) => {
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                crate::screen::dashboard::page::answered(signed, call, value)
            }
            Message::BridgedWrote(call, wrote, answered) => {
                let value = answered
                    .or_refused(&wrote)
                    .map(|()| serde_json::Value::Null);
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                crate::screen::dashboard::page::answered(signed, call, value)
            }
            Message::DashboardBrowsed(answered) => {
                let Some((path, entries)) = answered.or_none(Text::FailureDirectoryUnread) else {
                    return Task::none();
                };
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                dashboard::browsed(signed, path, entries);
                Task::none()
            }
            Message::ActivityPaged(page, answered) => {
                let Some(rows) = answered.or_none(Text::FailureActivityUnread) else {
                    return Task::none();
                };
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                dashboard::paged(signed, page, rows);
                Task::none()
            }
            Message::DashboardDiscovered(found) => {
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                dashboard::discovered(signed, found.or_default(Text::FailureTunerDiscovery));
                Task::none()
            }
            Message::DashboardLineups(found) => {
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                dashboard::lineups(signed, found.or_default(Text::FailureLineupsUnread));
                Task::none()
            }
            Message::RefreshItem {
                item,
                replace,
                scope,
            } => {
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                let api = signed.api.clone();
                Task::perform(
                    async move { api.refresh_item(item, replace, scope).await },
                    move |outcome| {
                        Message::DashboardWrote(
                            crate::error::Wrote {
                                operation: crate::error::Operation::RefreshItem,
                                object: item.to_string(),
                            },
                            outcome,
                        )
                    },
                )
            }
            Message::PageClosed => Task::none(),
            Message::LeaveAnyway => {
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                let Some(route) = signed.leaving.take() else {
                    return Task::none();
                };
                dashboard::abandoned(signed);
                crate::screen::settings::abandoned(signed);
                self.navigate(route)
            }
            Message::StayHere => {
                if let Some(signed) = self.signed() {
                    signed.leaving = None;
                }
                Task::none()
            }
            Message::PlayerAction(action) => {
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                player::act(signed, action, viewport)
            }
            Message::Reported(answered) => {
                let Some(standing) = answered.or_none(Text::FailurePlaybackReport) else {
                    return Task::none();
                };
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                player::reported(signed, standing, viewport)
            }
            Message::GroupAction(action) => {
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                let task = group::act(signed, action, viewport);
                let settle = self.fetch_images();
                Task::batch([task, settle])
            }
            Message::GroupTicked => {
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                group::ticked(signed, viewport)
            }
            Message::FontWanted(face) => {
                Task::perform(crate::fonts::fetched(face), move |answer| {
                    Message::FontFetched(face, answer)
                })
            }
            Message::FontFetched(face, answer) => {
                crate::fonts::settled(face);
                let Some(packed) = answer.disregarded(Text::FailureFontUnread) else {
                    crate::failure::raise(crate::error::stated(strings::format(
                        Text::FailureFontFamily,
                        &[face.family().name()],
                    )));
                    return Task::none();
                };
                let Some(sfnt) = crate::failure::unpacked(Text::FailureFontUnpacked, &packed)
                else {
                    return Task::none();
                };
                iced::font::load(sfnt).map(|_| Message::FontLoaded)
            }
            Message::FontLoaded => Task::none(),
            Message::Resized(page) => {
                self.viewport = page;
                let canvas = page.canvas();
                if let Some(signed) = self.signed() {
                    signed.queue.resized(canvas.height());
                }
                if let Some(browse) = self.browsing_mut() {
                    browse.resized(page);
                }
                if let Some(hub) = self.hubbing() {
                    let wall = hub::wall(card::Aspect::shared(
                        hub.entries
                            .held()
                            .filter_map(|item| item.primary_image_aspect_ratio)
                            .map(card::Aspect::of),
                    ));
                    let room = Room::content(page);
                    hub.grid
                        .resized(room, wall.card.width(room), wall.row(room));
                }
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                if let View::LiveTv(state) = &mut signed.view {
                    let room = Room::content(page);
                    match &mut state.body {
                        livetv::Body::Guide(held) => held.window.resized(canvas.height()),
                        livetv::Body::Channels(held) => {
                            let card = livetv::channels::CARD;
                            held.grid
                                .resized(room, card.card.width(room), card.row(room));
                        }
                        livetv::Body::Recordings(held) => {
                            let drawn = livetv::recordings::card(&held.recordings);
                            held.grid
                                .resized(room, drawn.card.width(room), drawn.row(room));
                        }
                        livetv::Body::Schedule(_) => {}
                        livetv::Body::Series(held) => {
                            let card = livetv::series::CARD;
                            held.grid
                                .resized(room, card.card.width(room), card.row(room));
                        }
                    }
                }
                self.settle();
                self.fetch_images()
            }
            Message::Scrolled(scrolled) => {
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                if scrolled.id == window::Id::Queue {
                    signed.queue.scrolled(scrolled);
                } else if scrolled.id == window::Id::Entries {
                    if let Some(state) = self.playlisting() {
                        state.window.scrolled(scrolled);
                    }
                    self.settle();
                    return Task::batch([self.entries_fetch(), self.fetch_images()]);
                } else if let window::Id::Section(section) = scrolled.id {
                    if let View::Search(state) = &mut signed.view
                        && let Some(results) = state
                            .sections
                            .iter_mut()
                            .find(|held| held.section == section)
                    {
                        results.window.scrolled(scrolled);
                    }
                    self.settle();
                    return self.fetch_images();
                } else if scrolled.id == window::Id::Browse {
                    if let Some(browse) = self.browsing_mut() {
                        browse.scrolled(scrolled);
                    }
                    if let Some(hub) = self.hubbing() {
                        hub.grid.scrolled(scrolled);
                    }
                    self.settle();
                    return Task::batch([
                        self.browse_fetch(),
                        self.hub_fetch(),
                        self.fetch_images(),
                    ]);
                } else if let View::LiveTv(state) = &mut signed.view {
                    match &mut state.body {
                        livetv::Body::Guide(held) => held.window.scrolled(scrolled),
                        livetv::Body::Channels(held) => held.grid.scrolled(scrolled),
                        livetv::Body::Recordings(held) => held.grid.scrolled(scrolled),
                        livetv::Body::Schedule(_) => {}
                        livetv::Body::Series(held) => held.grid.scrolled(scrolled),
                    }
                } else if let View::Dashboard(state) = &mut signed.view {
                    match &mut state.body {
                        dashboard::Body::Activity(held) => held.window.scrolled(scrolled),
                        dashboard::Body::Log(held) => held.window.scrolled(scrolled),
                        _ => {}
                    }
                }
                let fetching = livetv::fetch_if_stale(signed);
                let paging = dashboard::fetch_if_stale(signed);
                self.settle();
                Task::batch([fetching, paging, self.fetch_images()])
            }
            Message::OnNowLoaded(loaded) => {
                let channels = loaded.or_default(Text::FailureChannelsUnread);
                if let Some(signed) = self.signed()
                    && let View::Home(state) = &mut signed.view
                {
                    state.on_now = channels;
                }
                self.settle();
                self.fetch_images()
            }
            Message::LiveTvLoaded(answered) => {
                let Some(state) = answered.or_none(Text::FailureLiveTvUnread) else {
                    return Task::none();
                };
                let showing = self.loaded(View::LiveTv(Box::new(state)));
                let Some(signed) = self.signed() else {
                    return showing;
                };
                let fetching = livetv::fetch_if_stale(signed);
                Task::batch([showing, fetching])
            }
            Message::ProgramLoaded(answered) => {
                let Some(state) = answered.or_none(Text::FailureProgramUnread) else {
                    return Task::none();
                };
                self.loaded(View::Program(Box::new(state)))
            }
            Message::LiveTvAction(action) => {
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                let acting = livetv::act(signed, action, viewport);
                self.settle();
                Task::batch([acting, self.fetch_images()])
            }
            Message::GuideFetched(answered) => {
                let Some((fetched, programs)) = answered.or_none(Text::FailureGuideUnread) else {
                    return Task::none();
                };
                if let Some(signed) = self.signed()
                    && let View::LiveTv(state) = &mut signed.view
                    && let livetv::Body::Guide(guide) = &mut state.body
                {
                    guide.fetched(fetched, programs);
                }
                self.settle();
                self.fetch_images()
            }
            Message::AiringFetched(answered) => {
                let Some(program) = answered.or_none(Text::FailureAiringsUnread) else {
                    return Task::none();
                };
                if let Some(signed) = self.signed()
                    && let Some(playing) = signed.playing.as_mut()
                    && let Some(live) = playing.live.as_mut()
                {
                    live.advanced(program, chrono::Utc::now());
                }
                Task::none()
            }
            Message::SeriesPrefilled(answered) => {
                let Some((options, creating)) = answered.or_none(Text::FailureSeriesTimerUnread)
                else {
                    return Task::none();
                };
                if let Some(signed) = self.signed()
                    && let View::LiveTv(state) = &mut signed.view
                {
                    state.editing =
                        Some(crate::screen::livetv::series::Editing { options, creating });
                }
                Task::none()
            }
            Message::Wrote(operation, answered) => {
                let wrote = crate::error::Wrote {
                    operation,
                    object: String::new(),
                };
                answered.or_refused(&wrote);
                Task::none()
            }
            Message::LiveTicked => {
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                player::live::ticked(signed, chrono::Utc::now(), viewport)
            }
            Message::GroupItemsLoaded(answered) => {
                let Some(items) = answered.or_none(Text::FailureGroupItemsUnread) else {
                    return Task::none();
                };
                if let Some(signed) = self.signed()
                    && let Some(joined) = signed.group.as_mut()
                {
                    joined.items = items;
                }
                self.settle();
                self.fetch_images()
            }
            Message::Ticked => {
                let mut settled = false;
                if let Some(signed) = self.signed() {
                    settled = player::tick(signed);
                    if let Some((_, shown)) = signed.message.as_mut() {
                        *shown += crate::player::TICK;
                    }
                    if signed
                        .message
                        .as_ref()
                        .is_some_and(|(_, shown)| *shown >= NOTICE_HIDE)
                    {
                        signed.message = None;
                    }
                }
                if settled {
                    return Task::done(Message::PreviewSettled);
                }
                Task::none()
            }
            Message::RemoteAction(action) => {
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                let task = remote::act(signed, action, viewport);
                let settle = self.fetch_images();
                Task::batch([task, settle])
            }
            Message::LiveSignalled(live::Signal::Opened) => {
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                signed.live.signalled(&live::Signal::Opened);
                if let Some(playing) = signed.playing.as_ref() {
                    live::send(&Report::Playing {
                        play_session: playing.plan.play_session.clone(),
                    });
                }
                let route = signed.route().cloned().unwrap_or(Route::Home);
                let watching = watching(signed, &route);
                let loading = load(signed, &route, viewport);
                if signed.group.is_some() {
                    return Task::batch([watching, loading, group::reconnected(signed)]);
                }
                Task::batch([watching, loading])
            }
            Message::LiveSignalled(signal @ live::Signal::Closed { .. }) => {
                if let Some(signed) = self.signed() {
                    signed.live.signalled(&signal);
                    group::disconnected(signed);
                }
                Task::none()
            }
            Message::LiveSignalled(live::Signal::Received(event)) => self.received(event),
            Message::ImageLoaded(key, loaded) => {
                let Some(bytes) = loaded.or_none(Text::FailureImageUnread) else {
                    if !self.images.fail(key) || !self.images.begin(key) {
                        return Task::none();
                    }
                    return self.fetch_image(key);
                };
                self.images.store(key, bytes);
                Task::none()
            }
        }
    }

    /// Applies one event the local server sent.
    fn received(&mut self, event: Event) -> Task<Message> {
        let viewport = self.viewport;
        let Some(signed) = self.signed() else {
            return Task::none();
        };
        match event {
            Event::Marked { items } => {
                let restaling = restale(signed, &items);
                let marking = marked(signed, &items);
                Task::batch([marking, restaling])
            }
            Event::Timers { changes } => {
                if let Some(playing) = signed.playing.as_mut()
                    && let Some(live) = playing.live.as_mut()
                {
                    for changed in &changes {
                        live.timed(changed);
                    }
                }
                let applying = livetv::timed(signed, &changes, viewport);
                self.settle();
                Task::batch([applying, self.fetch_images()])
            }
            Event::Control(control) => player::controlled(signed, control, viewport),
            Event::Targets { targets } => {
                remote::listed(signed, targets);
                self.fetch_images()
            }
            Event::Displaced { cause } => {
                crate::failure::raise(crate::error::displaced(cause));
                match cause {
                    jellium_protocol::Displaced::Group => group::displaced(signed),
                    jellium_protocol::Displaced::Playback => {
                        crate::failure::raise(crate::error::refused(&PlaybackRefused::Superseded));
                    }
                }
                player::leave(signed, viewport)
            }
            Event::Groups { groups } => {
                group::listed(signed, groups);
                Task::none()
            }
            Event::Joined { group, member } => group::joined(
                signed,
                group,
                if member {
                    group::Membership::Holding
                } else {
                    group::Membership::Watching
                },
                viewport,
            ),
            Event::LibraryChanged {
                added,
                removed,
                updated,
            } => {
                let task = library_changed(signed, viewport, &added, &removed, &updated);
                Task::batch([task, self.browse_fetch()])
            }
            Event::GroupQueue(queue) => group::queued(signed, queue, viewport),
            Event::Scheduled(scheduled) => group::scheduled(signed, scheduled, viewport),
            Event::GroupEnded { cause } => {
                group::ended(signed, cause);
                Task::none()
            }
            Event::Clock(exchange) => {
                group::clocked(signed, exchange);
                Task::none()
            }
            Event::RemoteEnded { cause } => {
                remote::ended(signed, cause);
                Task::none()
            }
            Event::ServerStopping { restarting } => {
                crate::failure::raise(crate::error::server_stopping(restarting));
                Task::none()
            }
            Event::RestartRequired => {
                signed.restart_required = true;
                Task::none()
            }
            Event::UserUpdated {
                administrator,
                preference_access,
            } => {
                signed.session.administrator = administrator;
                signed.session.preference_access = preference_access;

                // the configuration is re-read whole, keeping every unsaved
                // edit, and the settings region states that the server's copy
                // changed
                let api = signed.api.clone();
                let user = signed.session.user_id;
                let refreshed = Task::perform(
                    async move { api.user_configuration(user).await },
                    Message::ConfigurationRefreshed,
                );

                // a settings screen this policy no longer admits is left the
                // same way a demotion leaves the dashboard
                let unreachable: Vec<Route> = signed
                    .history
                    .iter()
                    .filter(|route| match route {
                        Route::Settings { screen } => {
                            !crate::screen::settings::reaches(&signed.session, *screen)
                        }
                        _ => false,
                    })
                    .cloned()
                    .collect();
                if !unreachable.is_empty() {
                    signed.history.retain(|route| !unreachable.contains(route));
                    if signed.history.is_empty() {
                        signed.history.push(Route::Home);
                    }
                    signed.leaving = None;
                    let route = signed.route().cloned().unwrap_or(Route::Home);
                    signed.view = staged(&route, signed.session.live_tv);
                    let telling = watching(signed, &route);
                    return Task::batch([refreshed, telling, load(signed, &route, viewport)]);
                }

                if administrator {
                    return refreshed;
                }
                // A demotion ejects the user from every dashboard route at
                // once, naming that cause; it raises no unsaved-edit warning,
                // because it does not go through `leaving`.
                let dashboard = signed
                    .history
                    .iter()
                    .any(|route| matches!(route, Route::Dashboard { .. }));
                if !dashboard {
                    return refreshed;
                }
                signed
                    .history
                    .retain(|route| !matches!(route, Route::Dashboard { .. }));
                if signed.history.is_empty() {
                    signed.history.push(Route::Home);
                }
                signed.leaving = None;
                crate::failure::raise(crate::error::stated(
                    strings::lookup(Text::FailureDemoted).to_string(),
                ));
                let route = signed.route().cloned().unwrap_or(Route::Home);
                signed.view = staged(&route, signed.session.live_tv);
                let telling = watching(signed, &route);
                Task::batch([refreshed, telling, load(signed, &route, viewport)])
            }
            Event::Sessions { .. } => {
                let taken = event.clone();
                dashboard::received(signed, &taken)
            }
            Event::Tasks { ref tasks } => {
                let listing = dashboard::tasked(signed, tasks);
                let applying = dashboard::received(signed, &event);
                Task::batch([applying, listing])
            }
            Event::Activity { ref entries } => dashboard::logged(signed, entries),
            Event::Refreshing { items } => {
                let taken = items.clone();
                dashboard::refreshed(signed, &taken)
            }
            Event::PackageInstalling { .. }
            | Event::PackageInstalled { .. }
            | Event::PackageFailed { .. }
            | Event::PackageCancelled { .. }
            | Event::PackageUninstalled { .. } => {
                let taken = event.clone();
                dashboard::packaged(signed, &taken)
            }
            Event::UserDeleted => {
                live::disconnect();
                crate::failure::raise(crate::error::user_deleted());
                Task::none()
            }
            Event::Refused { refusal } => {
                crate::failure::raise(crate::error::live_refused(refusal));
                Task::none()
            }
        }
    }

    /// The browse surface the view on top holds, and `None` for a view holding
    /// none.
    fn browsing(&self) -> Option<&crate::screen::browse::Browse> {
        let Stage::Signed(signed) = &self.stage else {
            return None;
        };
        match &signed.view {
            View::Library(state) => match &state.body {
                crate::screen::library::Body::Browse(browse)
                | crate::screen::library::Body::Rows(browse) => Some(browse),
                _ => None,
            },
            View::Filtered(browse) => Some(browse),
            View::Collections(state) => Some(&state.browse),
            View::Collection(state) => Some(&state.browse),
            View::Playlists(state) => Some(&state.browse),
            _ => None,
        }
    }

    fn browsing_mut(&mut self) -> Option<&mut crate::screen::browse::Browse> {
        match &mut self.signed()?.view {
            View::Library(state) => match &mut state.body {
                crate::screen::library::Body::Browse(browse)
                | crate::screen::library::Body::Rows(browse) => Some(browse),
                _ => None,
            },
            View::Filtered(browse) => Some(browse),
            View::Collections(state) => Some(&mut state.browse),
            View::Collection(state) => Some(&mut state.browse),
            View::Playlists(state) => Some(&mut state.browse),
            _ => None,
        }
    }

    /// Re-reads the trickplay the playing item describes, which is what a
    /// version change and a new item each need.
    fn read_trickplay(&mut self) -> Task<Message> {
        let Some(signed) = self.signed() else {
            return Task::none();
        };
        let Some(item) = signed.playing.as_ref().and_then(|playing| playing.item.id) else {
            return Task::none();
        };
        let api = signed.api.clone();
        Task::perform(
            async move { api.trickplay(item).await },
            Message::TrickplayLoaded,
        )
    }

    /// The chapter image covering the pointer, which is what a preview falls
    /// back to on an item whose media source has no trickplay; an item with
    /// neither draws no preview.
    fn chapter_preview(&self) -> Option<iced::widget::image::Handle> {
        let signed = match &self.stage {
            Stage::Signed(signed) => signed,
            _ => return None,
        };
        let playing = signed.playing.as_ref()?;
        let at = playing.preview.as_ref()?.at;
        let index = crate::player::trickplay::chapter_at(&playing.plan.chapters, at)?;
        let numbered = failure::narrowed::<i32, _>(Text::FailureChapterIndex, index);
        self.images.handle(images::Key {
            item: playing.item.id?,
            kind: images::Kind::Chapter,
            index: numbered,
        })
    }

    /// Asks for every foreign image the screen on top draws and does not hold.
    /// A handle the local server does not hold is drawn as a missing image and
    /// is not asked for again.
    fn fetch_foreign(&mut self) -> Task<Message> {
        let Some(signed) = self.signed() else {
            return Task::none();
        };
        let wanted = match &signed.view {
            View::Metadata(state) => crate::screen::metadata::handles(state),
            _ => HashSet::new(),
        };
        signed.foreign.retain(&wanted);
        let api = signed.api.clone();
        let started: Vec<String> = wanted
            .into_iter()
            .filter(|handle| signed.foreign.begin(handle))
            .collect();

        Task::batch(started.into_iter().map(move |handle| {
            let api = api.clone();
            let asked = handle.clone();
            Task::perform(
                async move { api.foreign_image(&asked).await },
                move |bytes| Message::ForeignLoaded(handle.clone(), bytes),
            )
        }))
    }

    /// The playlist the view on top holds, and `None` for a view holding none.
    fn playlisting(&mut self) -> Option<&mut crate::screen::playlists::State> {
        match &mut self.signed()?.view {
            View::Playlist(state) => Some(state),
            _ => None,
        }
    }

    /// Asks for the one page the playlist window wants that is neither held nor
    /// in flight.
    fn entries_fetch(&mut self) -> Task<Message> {
        let playlist = match self.route() {
            Some(Route::Playlist { id }) => id,
            _ => return Task::none(),
        };
        let Some(signed) = self.signed() else {
            return Task::none();
        };
        let api = signed.api.clone();
        let Some(state) = self.playlisting() else {
            return Task::none();
        };
        let Some(page) = state.wanted() else {
            return Task::none();
        };
        state.entries.began(page.clone());
        let asked = page.clone();
        Task::perform(
            crate::screen::playlists::page(api, playlist, asked),
            move |answered| Message::PlaylistPaged(page.clone(), answered),
        )
    }

    /// The hub the view on top holds, and `None` for a view holding none.
    fn hubbing(&mut self) -> Option<&mut crate::screen::hub::State> {
        match &mut self.signed()?.view {
            View::Library(state) => match &mut state.body {
                crate::screen::library::Body::Hub(hub) => Some(hub),
                _ => None,
            },
            _ => None,
        }
    }

    /// Asks for the one page the hub window wants that is neither held nor in
    /// flight.
    fn hub_fetch(&mut self) -> Task<Message> {
        let Some(signed) = self.signed() else {
            return Task::none();
        };
        let api = signed.api.clone();
        let Some(hub) = self.hubbing() else {
            return Task::none();
        };
        let Some(page) = hub.wanted() else {
            return Task::none();
        };
        let (facet, library, sort) = (hub.facet, hub.library, hub.sort);
        hub.entries.began(page.clone());
        let asked = page.clone();
        Task::perform(
            crate::screen::hub::page(api, facet, library, sort, asked),
            move |answered| Message::HubPaged(page.clone(), answered),
        )
    }

    /// The parent the browse surface on top is narrowed to, and the term it is
    /// searching for.
    fn browse_source(&self) -> Option<(Option<Uuid>, Option<String>)> {
        match self.route()? {
            Route::Library { id, .. } => Some((Some(id), None)),
            _ => None,
        }
    }

    /// Asks for the one page the browse window wants that is neither held nor
    /// in flight.
    fn browse_fetch(&mut self) -> Task<Message> {
        let Some((parent, term)) = self.browse_source() else {
            return Task::none();
        };
        let Some(signed) = self.signed() else {
            return Task::none();
        };
        let api = signed.api.clone();
        let Some(browse) = self.browsing_mut() else {
            return Task::none();
        };
        let Some(page) = browse.wanted() else {
            return Task::none();
        };
        let listing = browse.listing.clone();
        browse.began(page.clone());
        let asked = page.clone();
        Task::perform(
            crate::screen::browse::page(api, parent, term, listing, asked),
            move |answered| Message::BrowsePaged(page.clone(), answered),
        )
    }

    /// One control of the shared sort and filter surfaces.
    fn browse_action(&mut self, action: crate::screen::browse::Action) -> Task<Message> {
        use crate::screen::browse::Action;

        if let Action::Jumped(letter) = action {
            let Some((parent, _)) = self.browse_source() else {
                return Task::none();
            };
            let Some(signed) = self.signed() else {
                return Task::none();
            };
            let api = signed.api.clone();
            let Some(browse) = self.browsing_mut() else {
                return Task::none();
            };
            let listing = browse.listing.clone();
            return Task::perform(
                async move { api.letter_index(parent, &listing, letter).await },
                Message::Jumped,
            );
        }

        let Some(browse) = self.browsing_mut() else {
            return Task::none();
        };
        match action {
            Action::Open(opened) => {
                browse.opened = Some(opened);
                Task::none()
            }
            Action::Close => {
                browse.opened = None;
                Task::none()
            }
            Action::Sorted(sort) => {
                browse.opened = None;
                let resting = browse.resorting(sort);
                browse.listing.sort = sort;
                let listing = browse.listing.clone();
                let restored = resting.unwrap_or(Drawn::ZERO);
                let id = browse.grid.id();
                browse.grid.moved(restored);
                browse.items = jellium_model::paged::Paged::new(0);
                Task::batch([window::resting(id, restored), self.relisted(listing)])
            }
            Action::Narrowed(narrow) => {
                browse.narrow(narrow);
                let listing = browse.listing.clone();
                browse.grid.moved(Drawn::ZERO);
                browse.items = jellium_model::paged::Paged::new(0);
                self.relisted(listing)
            }
            Action::ClearFilters => {
                browse.listing.facets = jellium_model::facets::Facets::default();
                let listing = browse.listing.clone();
                browse.grid.moved(Drawn::ZERO);
                browse.items = jellium_model::paged::Paged::new(0);
                self.relisted(listing)
            }
            Action::Jumped(_) => Task::none(),
        }
    }

    /// Puts `listing` on the route the browse surface stands under, so Back
    /// restores the list with the order and filters it was left with.
    fn relisted(&mut self, listing: crate::route::Listing) -> Task<Message> {
        let route = match self.route() {
            Some(Route::Library { id, mut tab }) => {
                match tab.as_mut() {
                    crate::screen::library::Tab::Items(held)
                    | crate::screen::library::Tab::Favorites(held)
                    | crate::screen::library::Tab::Episodes(held)
                    | crate::screen::library::Tab::Songs(held) => **held = listing,
                    _ => return Task::none(),
                }
                Route::Library { id, tab }
            }
            Some(Route::Filtered(mut filtered)) => {
                filtered.listing = listing;
                Route::Filtered(filtered)
            }
            _ => return Task::none(),
        };
        self.replace(route)
    }

    pub fn view(&self) -> Element<'_, Message> {
        let listing = match self.listing {
            true => widget::Listing::Open,
            false => widget::Listing::Closed,
        };
        widget::shell(&self.failures, listing, self.staged())
    }

    /// The stage's own screen, under the failure surfaces `view` wraps it in.
    fn staged(&self) -> Element<'_, Message> {
        match &self.stage {
            Stage::Booting { stalled: false } => {
                center(prose(strings::lookup(Text::StatusLoading), typeface::BODY)).into()
            }
            Stage::Booting { stalled: true } => center(
                column![
                    prose(strings::lookup(Text::BootSessionStalled), typeface::BODY),
                    iced::widget::button(prose(strings::lookup(Text::BootRecheck), typeface::BODY))
                        .style(style::raised)
                        .on_press(Message::SessionRechecked),
                ]
                .spacing(style::drawn(space::BLOCK_GAP.drawn())),
            )
            .into(),
            Stage::Login(state) => login::view(state, self.viewport),
            Stage::Setup(state) => crate::screen::setup::view(state, self.viewport),
            Stage::Lost(failure) => {
                center(widget::reported(failure, widget::Ending::SignedInAgain)).into()
            }
            Stage::Signed(signed) => {
                let read_only = signed.session.read_only;
                if let Some(playing) = signed.playing.as_ref()
                    && playing.video()
                {
                    return match &signed.view {
                        View::Queue => crate::screen::queue::view(
                            Some(playing),
                            signed.group.as_ref(),
                            signed.queue,
                            &self.images,
                        ),
                        View::Remote => crate::screen::remote::view(
                            signed.remote.as_ref(),
                            &signed.targets,
                            signed.device,
                            &self.images,
                            self.viewport,
                        ),
                        View::SyncPlay => crate::screen::syncplay::view(
                            signed.group.as_ref(),
                            &signed.groups,
                            signed.session.sync_play,
                        ),
                        _ => player::osd::view(
                            playing,
                            signed.group.as_ref(),
                            signed.session.sync_play,
                            signed.device,
                            signed.held.quality,
                            &self.images,
                            self.viewport,
                        ),
                    };
                }

                let menu: Option<Element<'_, Message>> = signed.overflow.as_ref().map(|open| {
                    crate::screen::overflow::view(
                        open,
                        &self.images,
                        crate::screen::overflow::enclosing(signed.route()),
                        &signed.session,
                    )
                });
                let body: Element<'_, Message> = match &signed.view {
                    View::Loading => {
                        center(prose(strings::lookup(Text::StatusLoading), typeface::BODY)).into()
                    }
                    View::Home(state) => home::view(
                        state,
                        &signed.arrangement,
                        signed.session.live_tv.allowed(),
                        chrono::Utc::now(),
                        self.viewport,
                        &self.images,
                        match read_only {
                            true => widget::Overflow::Withheld,
                            false => widget::Overflow::Offered,
                        },
                    ),
                    View::Library(state) => {
                        library::view(state, self.viewport, &self.images, read_only)
                    }
                    View::Detail(state) => {
                        detail::view(state, self.viewport, &self.images, &signed.session)
                    }
                    View::Search(state) => search::view(state, self.viewport, &self.images),
                    View::Filtered(browse) => {
                        crate::screen::browse::view(browse, self.viewport, &self.images)
                    }
                    View::Metadata(state) => crate::screen::metadata::view(
                        state,
                        self.viewport,
                        &self.images,
                        &signed.foreign,
                        read_only,
                    ),
                    View::Collections(state) => crate::screen::collections::view_listed(
                        state,
                        self.viewport,
                        &self.images,
                        read_only,
                    ),
                    View::Collection(state) => crate::screen::collections::view(
                        state,
                        self.viewport,
                        &self.images,
                        read_only,
                    ),
                    View::Playlists(state) => crate::screen::playlists::view_listed(
                        state,
                        self.viewport,
                        &self.images,
                        read_only,
                    ),
                    View::Playlist(state) => {
                        crate::screen::playlists::view(state, &self.images, read_only)
                    }
                    View::Queue => crate::screen::queue::view(
                        signed.playing.as_ref(),
                        signed.group.as_ref(),
                        signed.queue,
                        &self.images,
                    ),
                    View::Remote => crate::screen::remote::view(
                        signed.remote.as_ref(),
                        &signed.targets,
                        signed.device,
                        &self.images,
                        self.viewport,
                    ),
                    View::SyncPlay => crate::screen::syncplay::view(
                        signed.group.as_ref(),
                        &signed.groups,
                        signed.session.sync_play,
                    ),
                    View::LiveTv(state) => {
                        livetv::view(state, chrono::Utc::now(), &self.images, self.viewport)
                    }
                    View::Dashboard(state) => dashboard::view(
                        state,
                        &signed.session,
                        &self.images,
                        chrono::Utc::now(),
                        self.viewport,
                    ),
                    View::Settings(state) => {
                        crate::screen::settings::view(state, signed, &self.images, self.viewport)
                    }
                    View::Unavailable => center(iced::widget::Space::new()).into(),
                    View::Program(state) => {
                        program::view(state, self.viewport, chrono::Utc::now(), &self.images)
                    }
                };

                let mut page = column![widget::chrome(
                    &signed.session,
                    match signed.history.len() > 1 {
                        true => widget::Back::Offered,
                        false => widget::Back::Withheld,
                    },
                    match matches!(signed.route(), Some(Route::Settings { .. })) {
                        true => widget::Nav::Settings,
                        false => widget::Nav::Browse,
                    },
                    self.viewport,
                    body,
                )];

                if let Some(menu) = menu {
                    page = page.push(menu);
                }

                if signed.leaving.is_some() {
                    page = page.push(widget::leaving());
                }
                if let Some(playing) = signed.playing.as_ref() {
                    let transport = match signed.group.as_ref() {
                        Some(joined) => player::osd::Transport::Group(playing, joined),
                        None => player::osd::Transport::Local(playing),
                    };
                    page = page.push(player::osd::bar(
                        transport,
                        signed.session.sync_play,
                        signed.device,
                        &self.images,
                        self.viewport,
                    ));
                } else if let Some(bound) = signed.remote.as_ref() {
                    page = page.push(player::osd::bar(
                        player::osd::Transport::Remote(bound),
                        signed.session.sync_play,
                        signed.device,
                        &self.images,
                        self.viewport,
                    ));
                }
                let raised = widget::notices(&signed.session, signed.group.as_ref(), signed.live)
                    .into_iter()
                    .chain(signed.stopped.map(|stopped| {
                        widget::toast(
                            None,
                            strings::lookup(match stopped {
                                Stopped::Restarted => Text::FailureAdministratorRestarted,
                                Stopped::ShutDown => Text::FailureAdministratorShutDown,
                            })
                            .to_string(),
                        )
                    }))
                    .chain(signed.restart_required.then(|| {
                        widget::toast(
                            None,
                            strings::lookup(Text::DashboardRestartRequired).to_string(),
                        )
                    }))
                    .chain(signed.message.as_ref().map(|(notice, _)| {
                        widget::toast(Some(notice.header.clone()), notice.text.clone())
                    }))
                    .chain(
                        crate::screen::dashboard::page::shown(signed)
                            .and_then(|page| page.notice.as_ref())
                            .map(|notice| widget::toast(None, notice.clone())),
                    );
                let page = widget::toasted(page.into(), raised);
                match self
                    .browsing()
                    .and_then(|browse| crate::screen::browse::letters(browse, self.viewport))
                {
                    Some(letters) => widget::lettered(page, letters, self.viewport),
                    None => page,
                }
            }
        }
    }

    /// The media element's events, the player's keys and stirs, the idle tick,
    /// the group tick while membership lasts, the guide's keys while it is
    /// open, the live tick while a channel plays, the page's resizes, and the
    /// event socket's signals for the whole signed-in session.
    pub fn subscription(&self) -> Subscription<Message> {
        let layout = self.viewport.layout();
        let everywhere = Subscription::batch([
            crate::failure::reports().map(Message::Failed),
            crate::fonts::wants().map(Message::FontWanted),
            iced::window::resize_events()
                .with(layout)
                .filter_map(|(layout, _)| crate::page::viewport(layout).map(Message::Resized)),
        ]);
        let Stage::Signed(signed) = &self.stage else {
            if let Stage::Login(state) = &self.stage {
                return Subscription::batch([everywhere, login::subscription(state)]);
            }
            return everywhere;
        };

        let mut running = vec![
            everywhere,
            live::signals().map(Message::LiveSignalled),
            crate::overlay::messages().map(Message::Overlaid),
        ];
        if signed.playing.is_some() {
            running.extend([
                player::keys().map(Message::PlayerAction),
                player::stirs().map(Message::PlayerAction),
            ]);
        }
        if signed.playing.is_some() || signed.message.is_some() {
            running.push(player::ticks().map(|()| Message::Ticked));
        }
        if signed.group.is_some() {
            running.push(
                iced::time::every(crate::player::group::GROUP_TICK).map(|_| Message::GroupTicked),
            );
        }
        if let View::LiveTv(state) = &signed.view {
            running.push(livetv::keys(state).map(Message::LiveTvAction));
        }
        if signed
            .playing
            .as_ref()
            .is_some_and(|playing| playing.live.is_some())
        {
            running.push(
                iced::time::every(crate::player::live::LIVE_TICK).map(|_| Message::LiveTicked),
            );
        }
        Subscription::batch(running)
    }

    /// The canvas clears transparent while the video element holds the
    /// viewport, and to the theme's background otherwise.
    pub fn style(&self, theme: &Theme) -> iced::theme::Style {
        let over_video = matches!(
            &self.stage,
            Stage::Signed(signed) if signed.playing.as_ref().is_some_and(Playing::video)
        );
        iced::theme::Style {
            background_color: if over_video {
                iced::Color::TRANSPARENT
            } else {
                theme.palette().background
            },
            text_color: theme.palette().text,
        }
    }

    pub fn title(&self) -> String {
        strings::lookup(Text::AppName).to_string()
    }

    pub fn theme(&self) -> Theme {
        style::theme()
    }

    /// The layout's root size, which is what resolves every design length.
    pub fn scale_factor(&self) -> f32 {
        crate::style::scale(self.viewport.layout())
    }
}

/// Applies a library change: the home screen's library list and rails are
/// re-read, every open window re-fetches only the rows it is showing, and an
/// item removed while its own detail screen is open leaves text naming that
/// cause.
/// Neither the scroll position nor the sort moves.
fn library_changed(
    signed: &mut Signed,
    viewport: Viewport,
    _added: &[uuid::Uuid],
    removed: &[uuid::Uuid],
    updated: &[uuid::Uuid],
) -> Task<Message> {
    let api = signed.api.clone();

    if let Some(Route::Detail { id }) = signed.route()
        && removed.contains(id)
    {
        crate::failure::raise(crate::error::told(Text::FailureItemRemoved));
        signed.view = View::Unavailable;
        return Task::none();
    }

    match &mut signed.view {
        View::Home(_) => Task::perform(home::load(api), Message::HomeLoaded),
        View::Library(state) => match &mut state.body {
            crate::screen::library::Body::Browse(browse)
            | crate::screen::library::Body::Rows(browse) => reread(browse),
            _ => Task::none(),
        },
        View::Filtered(browse) => reread(browse),
        View::Collections(state) => reread(&mut state.browse),
        View::Collection(state) => reread(&mut state.browse),
        View::Playlists(state) => reread(&mut state.browse),
        View::Detail(state) if state.item.id.is_some_and(|id| updated.contains(&id)) => {
            let Some(id) = state.item.id else {
                return Task::none();
            };
            Task::perform(detail::load(api, id), Message::DetailLoaded)
        }
        View::Search(state) => {
            let term = state.term.clone();
            Task::perform(search::load(api, term, viewport), Message::SearchLoaded)
        }
        View::Detail(_)
        | View::Loading
        | View::LiveTv(_)
        | View::Program(_)
        | View::Metadata(_)
        | View::Playlist(_)
        | View::Queue
        | View::Remote
        | View::SyncPlay
        | View::Dashboard(_)
        | View::Settings(_)
        | View::Unavailable => Task::none(),
    }
}

/// Drops the rows the window is showing so the next fetch re-reads them; the
/// grid's offset and the listing's sort both stand.
fn reread(browse: &mut crate::screen::browse::Browse) -> Task<Message> {
    let shown = browse.grid.shown(browse.items.len());
    browse.forget(shown);
    Task::none()
}
