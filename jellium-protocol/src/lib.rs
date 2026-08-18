use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod profile;
pub mod report;
pub mod sync;

/// What the Jellyfin server's policy lets this user do with groups.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SyncAccess {
    None,
    Join,
    CreateAndJoin,
}

/// What Live TV this session offers, and why it offers none.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LiveTvAccess {
    /// The Jellyfin server reports no Live TV service.
    NoService,
    /// The user's policy denies Live TV.
    Denied,
    Allowed,
}

impl LiveTvAccess {
    /// True for `Allowed`, which is what every Live TV surface is drawn on and
    /// what the declared command list follows.
    pub fn allowed(self) -> bool {
        matches!(self, LiveTvAccess::Allowed)
    }
}

/// One saved server as the list and the status document carry it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedServer {
    pub server: String,
    /// The name the server reported at its last successful probe, empty until
    /// one succeeds.
    pub name: String,
    pub credentialed: bool,
    /// True for the first record, which is the active server.
    pub active: bool,
}

/// One user `GET /Users/Public` named, as the picker draws it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicUser {
    pub id: Uuid,
    pub name: String,
    /// True when the Jellyfin server holds a primary image for this user.
    pub has_image: bool,
}

/// One server's login screen, populated by the probe that opened it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginScreen {
    /// The opaque handle every login-stage request presents; it names no host.
    pub target: String,
    pub server: String,
    pub name: String,
    pub server_version: String,
    pub snapshot_version: String,
    /// Empty when the server reports no public user, which is what takes the
    /// picker off the screen.
    pub users: Vec<PublicUser>,
    /// True when the Jellyfin server reports Quick Connect enabled, which is
    /// what puts the Quick Connect option on the screen.
    pub quick_connect: bool,
    /// True when the saved credential this server held was rejected, which is
    /// what the screen states.
    pub rejected: bool,
    pub read_only: bool,
}

impl LoginScreen {
    pub fn off_snapshot(&self) -> bool {
        self.server_version != self.snapshot_version
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "camelCase")]
pub enum SessionStatus {
    #[serde(rename_all = "camelCase")]
    Anonymous {
        /// In file order, the active server first.
        servers: Vec<SavedServer>,
        read_only: bool,
    },
    /// One server's login screen, held as the login target while it shows.
    Login(LoginScreen),
    Authenticated(Session),
    /// The Jellyfin server has not completed its setup wizard.
    Setup(Startup),
    Failed(Failure),
}

/// A Jellyfin server in startup mode, as the wizard's chrome names it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Startup {
    pub server: String,
    pub server_version: String,
    pub snapshot_version: String,
    /// True when the wizard was entered by resuming a saved session, which is
    /// what the chrome names on screen.
    pub resumed: bool,
}

impl Startup {
    pub fn off_snapshot(&self) -> bool {
        self.server_version != self.snapshot_version
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub server: String,
    pub user_id: Uuid,
    pub user_name: String,
    pub server_version: String,
    pub snapshot_version: String,
    /// What this user's policy lets them do with groups.
    pub sync_play: SyncAccess,
    /// What Live TV this session offers.
    pub live_tv: LiveTvAccess,
    /// True when the signed-in user's policy carries `IsAdministrator`, which
    /// is what puts the dashboard in the chrome and Refresh Metadata on item
    /// detail.
    pub administrator: bool,
    /// True when the signed-in user's policy carries
    /// `EnableUserPreferenceAccess`, which is what puts every settings screen
    /// but profile and password in the settings column.
    pub preference_access: bool,
    /// True when the Jellyfin server reports Quick Connect enabled, which is
    /// what puts the Quick Connect screen in the settings column.
    pub quick_connect: bool,
    /// True when this instance was started `--read-only`, which is what takes
    /// every write action out of the chrome.
    pub read_only: bool,
    /// The device id this installation reports to the Jellyfin server, which is
    /// what names its own device in the device list.
    pub device: String,
    /// The client name this installation presents, which is the client the
    /// preference bag is read and written under.
    pub client: String,
}

impl Session {
    pub fn off_snapshot(&self) -> bool {
        self.server_version != self.snapshot_version
    }
}

/// The credentials one server's login screen submits; the server is the held
/// login target and is never named here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

/// The server text typed into add-server, before any scheme is tried.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddServer {
    pub url: String,
}

/// A saved server named by the url stored for it; a url no record holds is
/// refused.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChooseServer {
    pub server: String,
}

/// What removing a saved server did.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "removed", rename_all = "camelCase")]
pub enum Removed {
    /// The record is gone; its token was revoked, or it held none.
    Deleted,
    /// The record is gone and the Jellyfin server would not revoke its token.
    DeletedUnrevoked,
    /// No saved server holds that url.
    Unknown,
}

/// The user-facing code of one Quick Connect request; the secret stays on the
/// local server.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuickConnectCode {
    pub code: String,
}

/// Where a Quick Connect sign-in stands, as each poll answers.
/// The tag is `connect` rather than `quickConnect`, which `Session` already
/// carries as a field and which would be emitted twice by `Signed`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "connect", rename_all = "camelCase")]
pub enum QuickConnectState {
    /// No device has authorized the request yet.
    Pending,
    /// The request was authorized, the secret exchanged and the session
    /// installed.
    Signed(Session),
    /// The Jellyfin server no longer holds the request.
    Expired,
    /// The Jellyfin server turned Quick Connect off while the request stood.
    Disabled,
}

/// The username a password reset is asked for.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResetRequest {
    pub username: String,
}

/// Which of Jellyfin's three answers a password reset got.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "reset", rename_all = "camelCase")]
pub enum ResetAnswer {
    /// A pin was written to a file on the Jellyfin server.
    #[serde(rename_all = "camelCase")]
    PinWritten {
        /// The server's own path, shown as quoted server output.
        pin_file: String,
        /// Milliseconds since the unix epoch on the local server's clock,
        /// absent when the server named no expiry.
        expires: Option<i64>,
    },
    ContactAdministrator,
    InNetworkRequired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResetPin {
    pub pin: String,
}

/// What redeeming a pin answered.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "pin", rename_all = "camelCase")]
pub enum PinOutcome {
    /// The accounts whose password is now unset.
    Cleared { users: Vec<String> },
    /// The Jellyfin server refused the pin.
    Refused,
}

/// The login target a login-stage request presents, so a tab displaced by
/// another tab is refused rather than answered about a server it left.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Targeted {
    pub target: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "failure", rename_all = "camelCase")]
pub enum Failure {
    #[serde(rename_all = "camelCase")]
    ServerUnreachable {
        server: String,
        detail: String,
    },
    CredentialsRejected,
    TokenRejected,
    #[serde(rename_all = "camelCase")]
    ServerBelowMinimum {
        server_version: String,
        minimum_version: String,
    },
    /// `Startup/Complete` succeeded and the sign-in that follows it did not.
    SetupSignInFailed,
}

/// The stage a relayed route's entry admits it in, as a refusal names it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Admits {
    /// The entry is admissible only while a session is held.
    Signed,
    /// The entry is admissible only while the setup upstream is held.
    Setup,
    /// The entry would be admissible only while a login target is held, which
    /// no entry is.
    Login,
}

/// A refusal the local server made itself, distinct from anything the Jellyfin
/// server said.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "refusal", rename_all = "camelCase")]
pub enum Refusal {
    /// The session cookie was absent or did not match.
    NotThisBrowser,
    /// The Origin header was not the local server's own.
    ForeignOrigin,
    /// The local server holds no Jellyfin session.
    NoSession,
    /// The method and path are not one of the Jellyfin routes the local
    /// server relays.
    NotRelayed,
    /// A manifest body named a url the relay's route table refuses.
    ManifestNotRewritable,
    /// A manifest body was larger than the relay buffers.
    ManifestTooLarge,
    /// A request carried a foreign-image handle this run did not mint.
    ForeignNotObserved,
    /// A request body on a body-carrying route was larger than
    /// `route::BODY_LIMIT`.
    #[serde(rename_all = "camelCase")]
    BodyTooLarge {
        /// The length the request declared, absent when it declared none.
        bytes: Option<u64>,
        cap: usize,
    },
    /// The route's entry declares a write and the instance is read-only.
    ReadOnly,
    /// A configuration page name the local server has not seen in a
    /// configuration-page listing during this run.
    PageNotListed,
    /// A configuration page referenced a subresource the rewriter cannot
    /// resolve to a configuration page the local server has observed.
    PageNotRewritable,
    /// A configuration page document was larger than `page::PAGE_LIMIT`.
    PageTooLarge,
    /// The route's entry declares a stage this instance is not in.
    #[serde(rename_all = "camelCase")]
    NotInStage { admits: Admits },
    /// A setup request arrived after this run posted `Startup/Complete`.
    SetupFinished,
    /// The Jellyfin server is in startup mode and this instance is read-only,
    /// so no wizard is offered.
    SetupReadOnly,
    /// A login-stage request presenting a login target the local server does
    /// not hold, because another tab opened a different server's login screen
    /// or the stage was left.
    LoginMoved,
}

/// The whole `StartupConfigurationDto`, read and written by the language step
/// and the metadata step; a field the server reports absent reads as empty.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetupConfiguration {
    pub server_name: String,
    pub ui_culture: String,
    pub preferred_metadata_language: String,
    pub metadata_country_code: String,
}

/// The first administrator: the name the Jellyfin server reports, and the
/// password this run posted.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetupUser {
    pub name: String,
    pub password: String,
}

/// Both remote-access fields, sent on every save.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetupRemoteAccess {
    pub enable_remote_access: bool,
    pub enable_automatic_port_mapping: bool,
}

/// One live subscription a screen holds open while it is shown.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Feed {
    /// The device picker's and the remote panel's session listing.
    Targets,
    /// Dashboard home's session listing.
    Sessions,
    Groups,
    Tasks,
    Activity,
    Refresh,
    Packages,
}

impl Feed {
    pub const ALL: [Feed; 7] = [
        Feed::Targets,
        Feed::Sessions,
        Feed::Groups,
        Feed::Tasks,
        Feed::Activity,
        Feed::Refresh,
        Feed::Packages,
    ];

    /// True for the two feeds one upstream `Sessions` subscription serves.
    pub fn sessions(self) -> bool {
        matches!(self, Feed::Targets | Feed::Sessions)
    }
}

/// Where a scheduled task stands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TaskRunState {
    Idle,
    Cancelling,
    Running,
}

/// How a scheduled task's last run ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TaskEnding {
    Completed,
    Failed,
    Cancelled,
    Aborted,
}

/// A scheduled task's last run: when it opened, when it closed, and how it
/// ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskRun {
    pub started: chrono::DateTime<chrono::Utc>,
    pub ended: chrono::DateTime<chrono::Utc>,
    pub ending: TaskEnding,
}

/// One scheduled task, as the task list, task detail and dashboard home take
/// it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskState {
    pub id: String,
    pub name: String,
    pub category: String,
    pub description: String,
    pub state: TaskRunState,
    /// 0.0..=100.0 while the task runs, absent otherwise.
    pub progress: Option<f64>,
    /// The run that closed last, and nothing where the server reports none or
    /// reports it without both of its two moments.
    pub last_ran: Option<TaskRun>,
}

/// One activity log entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityEntry {
    pub id: i64,
    pub name: String,
    pub overview: String,
    pub kind: String,
    pub severity: String,
    /// The user the entry names, absent when it names none.
    pub user: Option<Uuid>,
    /// The item the entry names, absent when it names none.
    pub item: Option<Uuid>,
    /// Milliseconds since the unix epoch on the local server's clock.
    pub at: i64,
}

/// How far a refresh of one item has got.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Refreshed {
    pub item: Uuid,
    /// 0.0..=100.0.
    pub progress: f64,
}

/// One package install, as the five package messages carry it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Packaged {
    pub name: String,
    pub version: String,
    /// The plugin the package installs, absent when the message names none.
    pub plugin: Option<Uuid>,
}

/// One Jellyfin session as dashboard home shows it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerSession {
    pub session: String,
    pub device_name: String,
    pub client_name: String,
    pub user_name: String,
    /// What the session is playing, absent when it plays nothing.
    pub playing: Option<String>,
    /// True for this installation's own session.
    pub own: bool,
}

/// The ceiling the user chose for a stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "quality", rename_all = "camelCase")]
pub enum Quality {
    /// The local server's measurement of its own link to Jellyfin.
    Auto,
    #[serde(rename_all = "camelCase")]
    Limit { bits_per_second: Bitrate },
}

impl Quality {
    /// Auto and the fixed ladder the quality menu offers, in the order it
    /// offers them.
    pub const LADDER: [Quality; 13] = [
        Quality::Auto,
        Quality::Limit {
            bits_per_second: Bitrate::of(120_000_000),
        },
        Quality::Limit {
            bits_per_second: Bitrate::of(60_000_000),
        },
        Quality::Limit {
            bits_per_second: Bitrate::of(40_000_000),
        },
        Quality::Limit {
            bits_per_second: Bitrate::of(20_000_000),
        },
        Quality::Limit {
            bits_per_second: Bitrate::of(15_000_000),
        },
        Quality::Limit {
            bits_per_second: Bitrate::of(10_000_000),
        },
        Quality::Limit {
            bits_per_second: Bitrate::of(8_000_000),
        },
        Quality::Limit {
            bits_per_second: Bitrate::of(6_000_000),
        },
        Quality::Limit {
            bits_per_second: Bitrate::of(4_000_000),
        },
        Quality::Limit {
            bits_per_second: Bitrate::of(3_000_000),
        },
        Quality::Limit {
            bits_per_second: Bitrate::of(2_000_000),
        },
        Quality::Limit {
            bits_per_second: Bitrate::of(1_000_000),
        },
    ];
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Repeat {
    #[default]
    Off,
    One,
    All,
}

impl Repeat {
    /// The mode a repeat control moves to: off, then one, then all.
    pub fn cycled(self) -> Repeat {
        match self {
            Repeat::Off => Repeat::One,
            Repeat::One => Repeat::All,
            Repeat::All => Repeat::Off,
        }
    }
}

/// One media stream of a source, addressed the way the Jellyfin server numbers
/// them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct StreamIndex(i32);

impl StreamIndex {
    /// The stream `number` names; a negative number names none.
    pub fn named(number: i32) -> Option<StreamIndex> {
        (number >= 0).then_some(StreamIndex(number))
    }

    /// The number the Jellyfin server addresses this stream by.
    pub fn number(self) -> i32 {
        self.0
    }
}

impl<'de> serde::Deserialize<'de> for StreamIndex {
    /// A number naming no stream is refused, so no sentinel enters from a body.
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<StreamIndex, D::Error> {
        let number = i32::deserialize(deserializer)?;
        StreamIndex::named(number)
            .ok_or_else(|| serde::de::Error::custom(format!("{number} names no stream")))
    }
}

/// A rate in bits per second.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct Bitrate(i64);

impl Bitrate {
    /// The rate `bits_per_second` names.
    pub const fn of(bits_per_second: i64) -> Bitrate {
        Bitrate(bits_per_second)
    }

    /// The number the Jellyfin server addresses this rate by.
    pub fn bits_per_second(self) -> i64 {
        self.0
    }
}

impl<'de> serde::Deserialize<'de> for Bitrate {
    /// A number naming no rate is refused, so no sentinel enters from a body.
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Bitrate, D::Error> {
        let number = i64::deserialize(deserializer)?;
        if number > 0 {
            Ok(Bitrate(number))
        } else {
            Err(serde::de::Error::custom(format!("{number} names no rate")))
        }
    }
}

/// Which subtitle a play request asks for.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Subtitles {
    /// The stream the negotiated source names as its own default.
    #[default]
    Default,
    /// No subtitle at all.
    Off,
    /// The stream at this index.
    Stream { index: StreamIndex },
}

impl Subtitles {
    /// The choice the Jellyfin server's subtitle stream index names: an absent
    /// number is `Default` and a negative one is `Off`.
    pub fn named(number: Option<i32>) -> Subtitles {
        match number {
            None => Subtitles::Default,
            Some(number) => match StreamIndex::named(number) {
                Some(index) => Subtitles::Stream { index },
                None => Subtitles::Off,
            },
        }
    }

    /// The choice a resolved selection stands for; `None` is `Off`.
    pub fn selected(chosen: Option<StreamIndex>) -> Subtitles {
        match chosen {
            Some(index) => Subtitles::Stream { index },
            None => Subtitles::Off,
        }
    }

    /// The number the Jellyfin server addresses this choice by; `Default`
    /// answers `None` and `Off` answers `-1`.
    pub fn number(self) -> Option<i32> {
        match self {
            Subtitles::Default => None,
            Subtitles::Off => Some(-1),
            Subtitles::Stream { index } => Some(index.number()),
        }
    }

    /// The stream this choice names, resolved against the source's own default.
    pub fn stream(self, default: Option<StreamIndex>) -> Option<StreamIndex> {
        match self {
            Subtitles::Default => default,
            Subtitles::Off => None,
            Subtitles::Stream { index } => Some(index),
        }
    }
}

/// What the browser asks the local server to negotiate.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayRequest {
    pub item: Uuid,
    /// The version the player selected; the first source when absent.
    pub media_source: Option<String>,
    pub audio_stream: Option<StreamIndex>,
    /// Which subtitle to start with.
    pub subtitles: Subtitles,
    pub start_ticks: i64,
    pub quality: Quality,
    /// The device profile the browser built for this item.
    pub profile: profile::DeviceProfile,
    /// The browser's `appSettings` preference, which lives in the browser and
    /// which the local server holds no copy of.
    // reference: always-burn-in-setting — playbackmanager.js:497
    pub always_burn_in_subtitle_when_transcoding: bool,
    /// Absent where jellyfin-web sends no such field; `Some(false)` on the one
    /// retry after the media element refused a direct play.
    pub allow_direct_play: Option<bool>,
    pub allow_direct_stream: Option<bool>,
    pub allow_video_stream_copy: Option<bool>,
    pub allow_audio_stream_copy: Option<bool>,
    /// The appHost grants only the browser can answer.
    pub grants: HostGrants,
    /// The browser's `userSettings` entry, which the local server holds no copy
    /// of.
    pub cinema_mode: bool,
    /// Whether this play asked for the full screen.
    pub fullscreen: bool,
    /// The queue position the play started at, which `getIntros` is skipped
    /// for; absent where the play started at the head.
    pub start_index: Option<usize>,
    /// What the browser knows about its own reporting before this stream
    /// exists, which is what the local server's start report carries.
    pub reporting: report::Reporting,
}

/// The appHost grants the relay reads, which are browser facts and cross with
/// the request because nothing on the relay can answer them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostGrants {
    // reference: remote-video-grant — apphost.js:265-267
    pub remote_video: bool,
}

/// The stream the browser plays, derived from the negotiated source.
/// `path` is same-origin; every other field is what the browser needs to pick a
/// player and to decide `crossOrigin`.
// reference: create-stream-info — playbackmanager.js:2827-2881
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Playable {
    pub path: String,
    pub method: Method,
    /// The streaming protocol the Jellyfin server reported for this stream.
    pub sub_protocol: Option<profile::Protocol>,
    pub container: Option<String>,
    pub run_time_ticks: Option<i64>,
    pub remote: bool,
    pub codecs: Vec<String>,
}

/// What the Jellyfin server settled on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Method {
    DirectPlay,
    DirectStream,
    #[serde(rename_all = "camelCase")]
    Transcode {
        /// True when the request asked that a burned-in subtitle be encoded
        /// into this transcode.
        subtitle_burn_in: bool,
    },
}

/// One version of an item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceChoice {
    pub id: String,
    pub name: String,
}

/// One audio stream of the negotiated source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioChoice {
    pub index: StreamIndex,
    pub label: String,
}

/// One subtitle stream of the negotiated source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubtitleChoice {
    pub index: StreamIndex,
    pub label: String,
    pub language: Option<String>,
    pub codec: Option<String>,
    pub delivery: SubtitleDelivery,
}

/// How the Jellyfin server said this subtitle stream reaches the browser.
// reference: get-delivery-method — playbackmanager.js:1500-1507
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "delivery", rename_all = "camelCase")]
pub enum SubtitleDelivery {
    /// Fetched separately, from the same-origin path the server's
    /// `DeliveryUrl` maps to.
    External { path: String },
    /// Carried inside the stream the browser is already loading.
    Embed,
    /// Burned into the picture by the Jellyfin server.
    Encode,
    /// Carried as a rendition of the HLS manifest.
    Hls,
    /// Dropped by the Jellyfin server, which reaches the browser as a choice
    /// carrying nothing to fetch.
    Drop,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chapter {
    pub name: String,
    pub start_ticks: i64,
}

/// Everything the browser needs to play one item, carrying no token and no
/// url outside the local server.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plan {
    pub play_session: String,
    pub item: Uuid,
    pub media_source: String,
    pub playable: Playable,
    pub start_ticks: i64,
    pub run_time_ticks: Option<i64>,
    pub sources: Vec<SourceChoice>,
    pub audio_streams: Vec<AudioChoice>,
    pub subtitle_streams: Vec<SubtitleChoice>,
    pub audio_stream: Option<StreamIndex>,
    /// The subtitle stream the plan starts with.
    pub subtitle_stream: Option<StreamIndex>,
    pub chapters: Vec<Chapter>,
    /// The ceiling that went to the Jellyfin server, measured or chosen.
    pub max_bitrate: Bitrate,
    /// True when the negotiated source is a live stream, which is what draws
    /// the live display and forecloses seeking.
    pub live: bool,
    /// Whether the Jellyfin server would transcode this source, which is the
    /// first conjunct of the transcoding retry.
    // reference: enable-playback-retry-with-transcoding — playbackmanager.js:3384-3387
    pub supports_transcoding: bool,
    /// The intros the Jellyfin server named for this item, and an empty list
    /// wherever no intros were asked for.
    pub intros: Vec<Uuid>,
}

/// What a play request answered.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "planned", rename_all = "camelCase")]
pub enum Planned {
    Started(Box<Plan>),
    /// The change was not made and the stream that was playing still is.
    Unchanged,
}

/// What the browser reports while playing.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Progress {
    pub playing: report::Playing,
    /// The progress event jellyfin-web would have reported under.
    pub event: report::Reported,
}

/// What the browser reports when playback ends.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stopped {
    pub playing: report::Playing,
}

/// Whether the reporting tab still holds the one playback session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "standing", rename_all = "camelCase")]
pub enum Standing {
    Current,
    /// A later start took the one playback session from this one.
    Superseded,
    /// This session ended because its reports stopped arriving, and no other
    /// tab holds one.
    Lapsed,
    /// This session was live and paused too long, so its tuner was released.
    Released,
}

/// A playback request the local server or the Jellyfin server would not
/// honour, distinct from a transport failure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "refused", rename_all = "camelCase")]
pub enum PlaybackRefused {
    /// The item declares no media source.
    NoMediaSource,
    /// No source this browser can play, even transcoded.
    NoPlayableSource,
    /// The Jellyfin server would not start a transcode.
    TranscodeRefused { code: String },
    /// Another tab took the one playback session.
    Superseded,
    /// The Jellyfin server negotiated a stream at a url the relay's route
    /// table does not admit.
    NotRelayable,
    /// The session ended because its reports stopped arriving.
    Lapsed,
    /// The Jellyfin server would not open a live stream: no tuner carrying the
    /// channel is free.
    NoTuner,
    /// The live stream this session held is gone upstream, found when the one
    /// resume after a dropped live stream re-negotiated.
    TunerGone,
    /// The Jellyfin server negotiated a live source at a path the relay's
    /// route table does not admit; `shape` names the path with every id
    /// segment replaced and no query.
    LiveNotRelayable { shape: String },
    /// This session was live and paused past `Playback::PAUSED_LIVE`, so its
    /// tuner was released.
    TunerReleased,
}

/// One item's user data, as a live refresh carries it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Marked {
    pub item: Uuid,
    pub played: bool,
    pub favorite: bool,
    pub play_count: i32,
    pub position_ticks: i64,
}

/// What a remote target is playing, as its session listing reports it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NowPlaying {
    pub item: Uuid,
    pub title: String,
    pub subtitle: String,
    pub position_ticks: i64,
    pub run_time_ticks: i64,
    pub paused: bool,
    pub muted: bool,
    pub volume: i32,
    pub repeat: Repeat,
    pub shuffled: bool,
}

/// One Jellyfin session this client may drive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Target {
    pub session: String,
    pub device_name: String,
    pub client_name: String,
    pub now_playing: Option<NowPlaying>,
}

/// What a play command does to the queue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PlayMode {
    Now,
    Next,
    Last,
    InstantMix,
    Shuffle,
}

impl PlayMode {
    /// True when the mode plays its items in a shuffled order.
    pub fn shuffles(self) -> bool {
        matches!(self, PlayMode::Shuffle)
    }

    /// True when the items name the seed of an instant mix rather than the
    /// queue itself.
    pub fn mixes(self) -> bool {
        matches!(self, PlayMode::InstantMix)
    }
}

/// A message another client asked this one to show.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notice {
    pub header: String,
    pub text: String,
}

/// One control command another client sent this one, named by the effect it
/// has here rather than by the Jellyfin verb that carried it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "control", rename_all = "camelCase")]
pub enum Control {
    #[serde(rename_all = "camelCase")]
    Play {
        items: Vec<Uuid>,
        mode: PlayMode,
        start_index: i32,
        start_ticks: i64,
        media_source: Option<String>,
        audio_stream: Option<StreamIndex>,
        subtitles: Subtitles,
    },
    Stop,
    PlayPause,
    Pause,
    Unpause,
    NextTrack,
    PreviousTrack,
    #[serde(rename_all = "camelCase")]
    Seek {
        position_ticks: i64,
    },
    Rewind,
    FastForward,
    VolumeUp,
    VolumeDown,
    SetVolume {
        level: i32,
    },
    Mute,
    Unmute,
    ToggleMute,
    SetAudioStream {
        index: StreamIndex,
    },
    SetSubtitleStream {
        subtitles: Subtitles,
    },
    SetMediaSource {
        id: String,
    },
    /// Absent is the Auto ceiling.
    #[serde(rename_all = "camelCase")]
    SetMaxBitrate {
        bits_per_second: Option<Bitrate>,
    },
    SetRepeat {
        repeat: Repeat,
    },
    SetShuffle {
        shuffled: bool,
    },
    /// Moves to the next channel during live playback.
    ChannelUp,
    /// Moves to the previous channel during live playback.
    ChannelDown,
    /// Opens the guide.
    Guide,
    ToggleFullscreen,
    /// Shows the on-screen display when it is hidden, and hides it otherwise.
    ToggleDisplay,
    GoHome,
    GoToSearch,
    /// Opens the item's detail screen.
    Show {
        item: Uuid,
    },
    Notify(Notice),
}

/// Where a group stands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GroupState {
    Idle,
    Waiting,
    Paused,
    Playing,
}

/// One SyncPlay group, as a listing or as the one this installation is in.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub id: Uuid,
    pub name: String,
    pub participants: Vec<String>,
    pub state: GroupState,
}

/// One entry of the group's queue, addressed by its playlist item id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Queued {
    pub playlist_item: Uuid,
    pub item: Uuid,
}

/// The queue the Jellyfin server owns on the group's behalf.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupQueue {
    pub items: Vec<Queued>,
    /// Where the group is in `items`, and `None` when it is playing nothing.
    pub playing_index: Option<usize>,
    pub position_ticks: i64,
    pub playing: bool,
    pub repeat: Repeat,
    pub shuffled: bool,
}

/// What a scheduled group command asks every member to do.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GroupCommand {
    Unpause,
    Pause,
    Stop,
    Seek,
}

/// One group command, its instant already converted off the Jellyfin server's
/// clock.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scheduled {
    pub command: GroupCommand,
    pub playlist_item: Option<Uuid>,
    pub position_ticks: i64,
    /// Milliseconds since the unix epoch on the local server's clock.
    pub at: i64,
}

/// One control a tab issues against the group it is in; the group is never
/// named, because the local server holds the membership.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "verb", rename_all = "camelCase")]
pub enum GroupVerb {
    /// Replaces the group's queue, which is where every Play, Play All and
    /// instant mix goes while membership lasts.
    #[serde(rename_all = "camelCase")]
    SetQueue {
        items: Vec<Uuid>,
        start_index: i32,
        start_ticks: i64,
    },
    Unpause,
    Pause,
    Stop,
    #[serde(rename_all = "camelCase")]
    Seek {
        position_ticks: i64,
    },
    #[serde(rename_all = "camelCase")]
    NextItem {
        playlist_item: Uuid,
    },
    #[serde(rename_all = "camelCase")]
    PreviousItem {
        playlist_item: Uuid,
    },
    #[serde(rename_all = "camelCase")]
    SetPlaylistItem {
        playlist_item: Uuid,
    },
    #[serde(rename_all = "camelCase")]
    RemoveFromPlaylist {
        playlist_items: Vec<Uuid>,
    },
    SetRepeat {
        repeat: Repeat,
    },
    SetShuffle {
        shuffled: bool,
    },
    /// The element stalled, an item change began, or a seek began.
    #[serde(rename_all = "camelCase")]
    Buffering {
        playing: bool,
        playlist_item: Uuid,
        position_ticks: i64,
    },
    /// The element can play through at the commanded position.
    #[serde(rename_all = "camelCase")]
    Ready {
        playing: bool,
        playlist_item: Uuid,
        position_ticks: i64,
    },
}

/// Why this tab stopped holding the transport and the queue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Displaced {
    /// A later playback start took the playback session.
    Playback,
    /// Another tab entered the group, which carries the playback session with
    /// it.
    Group,
}

/// Why membership ended without the user leaving the group.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GroupEnded {
    /// This installation left the group.
    Left,
    /// The Jellyfin server took this session out of the group.
    Removed,
    /// This installation took remote mode.
    Remote,
    /// The Jellyfin server says this session is not in the group.
    NotInGroup,
    /// The Jellyfin server says the group does not exist.
    NoSuchGroup,
    /// The Jellyfin server denied this user the group's library.
    LibraryDenied,
}

/// Why remote mode ended without the user leaving it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RemoteEnded {
    /// Another tab took remote mode.
    Taken,
    /// The target left the session listing, or stopped being controllable.
    TargetGone,
    /// A control command naming this client arrived.
    Controlled,
    /// This installation entered a group.
    Grouped,
}

/// A report the local server would not act on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "refusal", rename_all = "camelCase")]
pub enum LiveRefusal {
    /// A session id the local server has not seen in a listing for this user
    /// during this run.
    UnknownTarget,
    /// A drive from a tab holding no remote mode.
    NotDriving,
    /// The Jellyfin server would not carry out the command.
    TargetRefused,
    /// A group verb from a tab holding no membership.
    NotGrouped,
    /// The Jellyfin server would not carry out the group verb.
    GroupRefused,
    /// A verb the instance forecloses because it was started `--read-only`.
    ReadOnly,
}

/// Which of the four timer changes one event carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TimerChange {
    Created,
    Cancelled,
    SeriesCreated,
    SeriesCancelled,
}

/// One timer or series timer created or cancelled anywhere, as the guide's
/// record markers, the Schedule tab and the Series tab take it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimerChanged {
    pub change: TimerChange,
    /// The timer's or series timer's id.
    pub timer: String,
    /// The program the timer covers; absent when the Jellyfin server named
    /// none.
    pub program: Option<Uuid>,
}

/// Everything the local server sends a browser tab, and nothing else.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "event", rename_all = "camelCase")]
pub enum Event {
    Marked {
        items: Vec<Marked>,
    },
    /// The timer changes one coalescing window carried, sent to every tab.
    Timers {
        changes: Vec<TimerChanged>,
    },
    Control(Control),
    Targets {
        targets: Vec<Target>,
    },
    /// The joinable groups, sent to every tab with a picker or the SyncPlay
    /// screen open.
    Groups {
        groups: Vec<Group>,
    },
    /// The group this installation is in, sent to every tab when membership
    /// begins and whenever the group's name, participants, state or holder
    /// change; `member` is true for the tab holding the transport and the
    /// queue and false for every other tab.
    #[serde(rename_all = "camelCase")]
    Joined {
        group: Group,
        member: bool,
    },
    /// The ids one coalescing window carried, sent to every connected tab
    /// together at most once a second.
    #[serde(rename_all = "camelCase")]
    LibraryChanged {
        added: Vec<Uuid>,
        removed: Vec<Uuid>,
        updated: Vec<Uuid>,
    },
    /// The group's queue, sent to every tab.
    GroupQueue(GroupQueue),
    /// A group command, sent to the tab holding membership.
    Scheduled(Scheduled),
    GroupEnded {
        cause: GroupEnded,
    },
    /// The local server's side of one clock exchange.
    Clock(sync::Exchange),
    /// This tab stopped holding the transport and the queue.
    Displaced {
        cause: Displaced,
    },
    RemoteEnded {
        cause: RemoteEnded,
    },
    ServerStopping {
        restarting: bool,
    },
    /// The signed-in user was deleted on the Jellyfin server.
    UserDeleted,
    Refused {
        refusal: LiveRefusal,
    },
    /// Every scheduled task, sent to every tab holding `Feed::Tasks`.
    Tasks {
        tasks: Vec<TaskState>,
    },
    /// The activity entries one coalescing window carried, newest first, sent
    /// to every tab holding `Feed::Activity`.
    Activity {
        entries: Vec<ActivityEntry>,
    },
    /// The refresh progress one coalescing window carried, sent to every tab
    /// holding `Feed::Refresh`.
    Refreshing {
        items: Vec<Refreshed>,
    },
    PackageInstalling {
        package: Packaged,
    },
    PackageInstalled {
        package: Packaged,
    },
    PackageFailed {
        package: Packaged,
    },
    PackageCancelled {
        package: Packaged,
    },
    PackageUninstalled {
        package: Packaged,
    },
    /// The Jellyfin server says a restart is required.
    RestartRequired,
    /// The signed-in user's policy changed; both bits are what it carries now.
    #[serde(rename_all = "camelCase")]
    UserUpdated {
        administrator: bool,
        preference_access: bool,
    },
    /// Every session on the server, sent to every tab holding
    /// `Feed::Sessions`.
    Sessions {
        sessions: Vec<ServerSession>,
    },
}

/// Everything a browser tab sends the local server, and nothing else.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "report", rename_all = "camelCase")]
pub enum Report {
    /// This tab holds `play_session`.
    #[serde(rename_all = "camelCase")]
    Playing {
        play_session: String,
    },
    /// A screen consuming `feed` opened in this tab, and closed.
    Watch {
        feed: Feed,
    },
    Drop {
        feed: Feed,
    },
    TakeRemote {
        target: String,
    },
    LeaveRemote,
    Drive(Drive),
    /// Creates a group named `name` and joins it.
    CreateGroup {
        name: String,
    },
    JoinGroup {
        group: Uuid,
    },
    /// Binds this tab as the one holding the group, which is how a reopened
    /// socket reclaims membership and how a Play in a second tab takes it.
    TakeGroup,
    LeaveGroup,
    Group(GroupVerb),
    /// One clock exchange, carrying the browser's own round trip to the local
    /// server so the ping reported to the group can be composed.
    #[serde(rename_all = "camelCase")]
    Clock {
        sent: i64,
        round_trip: i64,
    },
}

impl Report {
    pub fn read_only(&self) -> bool {
        match self {
            Report::Playing { .. }
            | Report::Watch { .. }
            | Report::Drop { .. }
            | Report::Clock { .. } => true,
            Report::TakeRemote { .. }
            | Report::LeaveRemote
            | Report::Drive(_)
            | Report::CreateGroup { .. }
            | Report::JoinGroup { .. }
            | Report::TakeGroup
            | Report::LeaveGroup
            | Report::Group(_) => false,
        }
    }
}

/// One control against the target the reporting tab is bound to; the target
/// is never named, because the local server holds it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "drive", rename_all = "camelCase")]
pub enum Drive {
    #[serde(rename_all = "camelCase")]
    Play {
        items: Vec<Uuid>,
        start_index: i32,
        start_ticks: i64,
        mode: PlayMode,
    },
    PlayPause,
    Stop,
    #[serde(rename_all = "camelCase")]
    Seek {
        position_ticks: i64,
    },
    SkipBack,
    SkipForward,
    NextTrack,
    PreviousTrack,
    SetVolume {
        level: i32,
    },
    ToggleMute,
    SetRepeat {
        repeat: Repeat,
    },
    SetShuffle {
        shuffled: bool,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceIdentity;

/// The client name this browser presents to Jellyfin.
// reference: app-name — apphost.js:10
pub const CLIENT: &str = "Jellyfin Web";

/// The version this browser presents to Jellyfin.
// reference: app-version — package.json:3
pub const VERSION: &str = "10.11.11";

/// What the browser announces about itself, which is what every upstream
/// request is then issued under.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identity {
    /// The name `getDeviceName` answers for this browser.
    // reference: get-device-name — apphost.js:151-172
    pub device: String,
    /// The id this browser minted once and has kept since.
    // reference: generate-device-id — apphost.js:124-134
    pub device_id: String,
}

/// The prefix a foreign image the local server minted a handle for is fetched
/// under.
pub const FOREIGN_PREFIX: &str = "/foreign";

/// The prefix a url the current playback plan has been pointed at is fetched
/// under; a handle here dies with the plan that minted it.
pub const POINTED_PREFIX: &str = "/pointed";

pub const RELAY_PREFIX: &str = "/jellyfin";
pub const IDENTITY_PATH: &str = "/session";
pub const PLAYBACK_PATH: &str = "/playback";
/// The door a user-initiated play uses, and the only one that requests intros.
pub const PLAYBACK_ENTER_PATH: &str = "/playback/enter";
/// Swaps the source under the session already playing, which a track, quality
/// or version change asks for.
pub const PLAYBACK_CHANGE_PATH: &str = "/playback/change";
pub const PLAYBACK_PROGRESS_PATH: &str = "/playback/progress";
pub const PLAYBACK_STOPPED_PATH: &str = "/playback/stopped";
pub const LIVE_PATH: &str = "/live";
pub const GROUP_LEAVE_PATH: &str = "/group/leave";
/// Releases the setup upstream, which is what Back on the first step does.
pub const SETUP_PATH: &str = "/setup";
pub const SETUP_CONFIGURATION_PATH: &str = "/setup/configuration";
pub const SETUP_USER_PATH: &str = "/setup/user";
pub const SETUP_REMOTE_ACCESS_PATH: &str = "/setup/remote-access";
/// Posts `Startup/Complete` and signs in the administrator it created.
pub const SETUP_COMPLETE_PATH: &str = "/setup/complete";
/// Lists the saved servers, adds one, and removes one.
pub const SERVERS_PATH: &str = "/servers";
/// Selects a saved server, which is what makes it the active one.
pub const SERVER_SELECT_PATH: &str = "/servers/select";
/// Releases the held upstream, keeps its credential, and answers the list.
pub const SWITCH_PATH: &str = "/switch";
/// Releases the login target, which is what Back off a login screen does.
pub const LOGIN_PATH: &str = "/login";
/// `{LOGIN_IMAGE_PREFIX}/{user}/image`, answered only for a user id in the
/// public list last fetched for the held login target.
pub const LOGIN_IMAGE_PREFIX: &str = "/login/user";
/// Initiates, polls and abandons one Quick Connect sign-in.
pub const QUICK_CONNECT_PATH: &str = "/login/quickconnect";
pub const RESET_PATH: &str = "/login/reset";
pub const RESET_PIN_PATH: &str = "/login/reset/pin";
/// The query name every login-stage request carries `Targeted` under.
pub const TARGET_QUERY: &str = "target";
pub const SECRET_QUERY: &str = "s";
pub const COOKIE_NAME: &str = "jellium_web";

/// Mints and releases a configuration page's grant.
pub const PLUGIN_PATH: &str = "/plugin";

/// Serves a rewritten configuration page under a live grant.
pub const PAGE_PREFIX: &str = "/page";

/// What the browser asks the local server to open a configuration page as.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageRequest {
    pub name: String,
}

/// The frame one configuration page is opened in.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Framed {
    /// The same-origin path the frame loads.
    pub path: String,
    /// The grant that path carries; closing the page releases it.
    pub grant: String,
}

#[cfg(test)]
mod tests {
    use super::{
        Drive, Feed, GroupVerb, HostGrants, LiveTvAccess, Method, PlayMode, PlayRequest, Playable,
        Quality, QuickConnectState, Repeat, Report, Session, StreamIndex, Subtitles, SyncAccess,
        profile,
    };

    fn session() -> Session {
        Session {
            server: "https://example.test".to_string(),
            user_id: uuid::Uuid::nil(),
            user_name: "first".to_string(),
            server_version: "10.11.0".to_string(),
            snapshot_version: "10.11.0".to_string(),
            sync_play: SyncAccess::CreateAndJoin,
            live_tv: LiveTvAccess::Allowed,
            administrator: true,
            preference_access: true,
            quick_connect: true,
            read_only: false,
            device: "device".to_string(),
            client: "Jellium Web".to_string(),
        }
    }

    #[test]
    fn a_signed_quick_connect_state_round_trips_with_its_session() {
        let signed = QuickConnectState::Signed(session());
        let text = serde_json::to_string(&signed).expect("the state serializes");
        assert_eq!(
            serde_json::from_str::<QuickConnectState>(&text).expect("the state deserializes"),
            signed
        );

        let document: serde_json::Map<String, serde_json::Value> =
            serde_json::from_str(&text).expect("an object");
        assert_eq!(
            document.get("connect").and_then(|tag| tag.as_str()),
            Some("signed")
        );
        assert_eq!(
            document.get("quickConnect").and_then(|held| held.as_bool()),
            Some(true)
        );
    }

    /// An internally tagged variant whose payload carries the tag's own name
    /// emits that name twice, which no reader can decode.
    #[test]
    fn no_internally_tagged_variant_names_a_field_its_payload_carries() {
        for state in [
            QuickConnectState::Pending,
            QuickConnectState::Signed(session()),
            QuickConnectState::Expired,
            QuickConnectState::Disabled,
        ] {
            let text = serde_json::to_string(&state).expect("the state serializes");
            assert_eq!(
                text.matches("\"connect\":").count(),
                1,
                "the tag is emitted once: {text}"
            );
            assert_eq!(
                serde_json::from_str::<QuickConnectState>(&text).expect("the state deserializes"),
                state
            );
        }
    }

    #[test]
    fn read_only_admits_playback_and_subscription_and_forecloses_the_rest() {
        for report in [
            Report::Playing {
                play_session: String::new(),
            },
            Report::Watch {
                feed: Feed::Sessions,
            },
            Report::Drop {
                feed: Feed::Sessions,
            },
            Report::Clock {
                sent: 0,
                round_trip: 0,
            },
        ] {
            assert!(report.read_only(), "{report:?}");
        }
        for report in [
            Report::TakeRemote {
                target: String::new(),
            },
            Report::LeaveRemote,
            Report::Drive(Drive::PlayPause),
            Report::CreateGroup {
                name: String::new(),
            },
            Report::JoinGroup {
                group: uuid::Uuid::nil(),
            },
            Report::TakeGroup,
            Report::LeaveGroup,
            Report::Group(GroupVerb::Pause),
        ] {
            assert!(!report.read_only(), "{report:?}");
        }
    }

    fn play_request() -> PlayRequest {
        PlayRequest {
            item: uuid::Uuid::nil(),
            media_source: None,
            audio_stream: StreamIndex::named(1),
            subtitles: Subtitles::Default,
            start_ticks: 0,
            quality: Quality::Auto,
            profile: profile::DeviceProfile::default(),
            always_burn_in_subtitle_when_transcoding: false,
            allow_direct_play: None,
            allow_direct_stream: None,
            allow_video_stream_copy: None,
            allow_audio_stream_copy: None,
            grants: HostGrants { remote_video: true },
            cinema_mode: true,
            fullscreen: true,
            start_index: None,
            reporting: crate::report::Reporting {
                volume_level: 100,
                muted: false,
                repeat: Repeat::Off,
                shuffle: crate::report::Shuffle::Sorted,
                playback_rate: 1.0,
                playlist_item_id: "playlistItem1".to_string(),
                queue: Vec::new(),
            },
        }
    }

    /// A body naming a stream by a negative number decodes as nothing, so no
    /// sentinel reaches a plan.
    #[test]
    fn a_posted_stream_index_below_zero_is_refused() {
        let text = serde_json::to_string(&play_request()).expect("the request serializes");
        let mut document: serde_json::Value = serde_json::from_str(&text).expect("an object");

        document["audioStream"] = serde_json::json!(-1);
        assert!(serde_json::from_str::<PlayRequest>(&document.to_string()).is_err());

        document["audioStream"] = serde_json::Value::Null;
        document["subtitles"] = serde_json::json!({ "stream": { "index": -1 } });
        assert!(serde_json::from_str::<PlayRequest>(&document.to_string()).is_err());
    }

    #[test]
    fn a_subtitle_choice_names_no_key_after_its_own_type() {
        assert_eq!(
            serde_json::to_string(&Subtitles::Off).expect("the choice serializes"),
            "\"off\""
        );
        assert_eq!(
            serde_json::to_string(&Subtitles::Stream {
                index: StreamIndex::named(3).expect("a stream"),
            })
            .expect("the choice serializes"),
            "{\"stream\":{\"index\":3}}"
        );
    }

    #[test]
    fn a_playable_carries_its_protocol_and_a_method_names_no_key_after_its_own_type() {
        assert_eq!(
            serde_json::to_string(&Playable {
                path: "/jellyfin/Videos/x/master.m3u8".to_string(),
                method: Method::DirectStream,
                sub_protocol: Some(profile::Protocol::Hls),
                container: Some("mkv".to_string()),
                run_time_ticks: Some(100),
                remote: false,
                codecs: vec!["h264".to_string()],
            })
            .expect("the playable serializes"),
            "{\"path\":\"/jellyfin/Videos/x/master.m3u8\",\"method\":\"directStream\",\
             \"subProtocol\":\"hls\",\"container\":\"mkv\",\"runTimeTicks\":100,\
             \"remote\":false,\"codecs\":[\"h264\"]}"
        );
        assert_eq!(
            serde_json::to_string(&Method::Transcode {
                subtitle_burn_in: true,
            })
            .expect("the method serializes"),
            "{\"transcode\":{\"subtitleBurnIn\":true}}"
        );
    }

    #[test]
    fn the_repeat_cycle_is_off_then_one_then_all() {
        assert_eq!(Repeat::Off.cycled(), Repeat::One);
        assert_eq!(Repeat::One.cycled(), Repeat::All);
        assert_eq!(Repeat::All.cycled(), Repeat::Off);
    }

    #[test]
    fn every_play_mode_says_whether_it_shuffles_and_mixes() {
        for (mode, shuffles, mixes) in [
            (PlayMode::Now, false, false),
            (PlayMode::Next, false, false),
            (PlayMode::Last, false, false),
            (PlayMode::InstantMix, false, true),
            (PlayMode::Shuffle, true, false),
        ] {
            assert_eq!(mode.shuffles(), shuffles);
            assert_eq!(mode.mixes(), mixes);
        }
    }
}
