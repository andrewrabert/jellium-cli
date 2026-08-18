use jellium_protocol::{
    Displaced, Failure, GroupEnded, LiveRefusal, LiveTvAccess, PlaybackRefused, Refusal,
    RemoteEnded,
};

use crate::text::{self, Text};

#[must_use = "a trouble must be raised, disregarded by name, or propagated"]
#[derive(Debug, Clone)]
pub enum Trouble {
    /// jellium-cli refused the request itself.
    Refused(Refusal),
    /// jellium-cli could not be reached, or answered with something this
    /// client does not understand; `status` is the answer's status where the
    /// answer had one.
    Relay { status: Option<u16>, detail: String },
    /// The Jellyfin server refused, or jellium-cli could not reach it.
    Upstream(Failure),
    /// The signed-in user was deleted on the Jellyfin server.
    UserDeleted,
    /// The Jellyfin server does not hold the log file named.
    LogMissing { name: String },
    /// This installation's own device was deleted, so its session ended.
    OwnDeviceDeleted,
}

impl Trouble {
    /// True when jellium-cli no longer holds a Jellyfin session: a token the
    /// Jellyfin server rejected, or a relay holding none.
    pub fn session_lost(&self) -> bool {
        matches!(
            self,
            Trouble::Refused(Refusal::NoSession)
                | Trouble::Upstream(Failure::TokenRejected)
                | Trouble::UserDeleted
                | Trouble::OwnDeviceDeleted
        )
    }

    fn message(&self) -> String {
        match self {
            Trouble::Refused(Refusal::NotThisBrowser) => {
                text::lookup(Text::FailureNotThisBrowser).to_string()
            }
            Trouble::Refused(Refusal::ForeignOrigin) => {
                text::lookup(Text::FailureForeignOrigin).to_string()
            }
            Trouble::Refused(Refusal::NoSession) => {
                text::lookup(Text::FailureNoSession).to_string()
            }
            Trouble::Refused(Refusal::NotRelayed) => {
                text::lookup(Text::FailureNotRelayed).to_string()
            }
            Trouble::Refused(Refusal::ManifestNotRewritable) => {
                text::lookup(Text::FailureManifestNotRewritable).to_string()
            }
            Trouble::Refused(Refusal::ManifestTooLarge) => {
                text::lookup(Text::FailureManifestTooLarge).to_string()
            }
            Trouble::Refused(Refusal::ForeignNotObserved) => {
                text::lookup(Text::FailureForeignNotObserved).to_string()
            }
            Trouble::Refused(Refusal::BodyTooLarge { bytes, cap }) => match bytes {
                Some(bytes) => text::format(
                    Text::FailureBodyTooLarge,
                    &[&bytes.to_string(), &cap.to_string()],
                ),
                None => text::format(Text::FailureBodyOverCap, &[&cap.to_string()]),
            },
            Trouble::Refused(Refusal::ReadOnly) => {
                text::lookup(Text::DashboardReadOnly).to_string()
            }
            Trouble::Refused(Refusal::PageNotListed) => {
                text::lookup(Text::FailurePageNotListed).to_string()
            }
            Trouble::Refused(Refusal::PageNotRewritable) => {
                text::lookup(Text::FailurePageNotRewritable).to_string()
            }
            Trouble::LogMissing { name } => text::format(Text::FailureLogMissing, &[name]),
            Trouble::OwnDeviceDeleted => text::lookup(Text::FailureOwnDeviceDeleted).to_string(),
            Trouble::Refused(Refusal::PageTooLarge) => {
                text::lookup(Text::FailurePageTooLarge).to_string()
            }
            Trouble::Relay { detail, .. } => text::format(Text::FailureRelay, &[detail]),
            Trouble::Upstream(Failure::ServerUnreachable { server, .. }) => {
                text::format(Text::FailureServerUnreachable, &[server])
            }
            Trouble::Upstream(Failure::CredentialsRejected) => {
                text::lookup(Text::FailureCredentialsRejected).to_string()
            }
            Trouble::Upstream(Failure::TokenRejected) => {
                text::lookup(Text::FailureTokenRejected).to_string()
            }
            Trouble::Upstream(Failure::ServerBelowMinimum {
                server_version,
                minimum_version,
            }) => text::format(
                Text::FailureServerBelowMinimum,
                &[server_version, minimum_version],
            ),
            Trouble::UserDeleted => text::lookup(Text::FailureUserDeleted).to_string(),
            Trouble::Refused(Refusal::NotInStage { admits }) => text::lookup(match admits {
                jellium_protocol::Admits::Signed => Text::FailureStageSignedOnly,
                jellium_protocol::Admits::Setup => Text::FailureStageSetupOnly,
                jellium_protocol::Admits::Login => Text::FailureStageLoginOnly,
            })
            .to_string(),
            Trouble::Refused(Refusal::LoginMoved) => {
                text::lookup(Text::FailureLoginMoved).to_string()
            }
            Trouble::Refused(Refusal::SetupFinished) => {
                text::lookup(Text::FailureSetupFinished).to_string()
            }
            Trouble::Refused(Refusal::SetupReadOnly) => {
                text::lookup(Text::FailureSetupReadOnly).to_string()
            }
            Trouble::Upstream(Failure::SetupSignInFailed) => {
                text::lookup(Text::FailureSetupSignIn).to_string()
            }
        }
    }
}

/// The report a playback refusal is shown as.
pub fn refused(refused: &PlaybackRefused) -> crate::failure::Failure {
    stated(match refused {
        PlaybackRefused::NoMediaSource => text::lookup(Text::FailureNoMediaSource).to_string(),
        PlaybackRefused::NoPlayableSource => {
            text::lookup(Text::FailureNoPlayableSource).to_string()
        }
        PlaybackRefused::TranscodeRefused { code } => {
            text::format(Text::FailureTranscodeRefused, &[code])
        }
        PlaybackRefused::Superseded => text::lookup(Text::FailurePlaybackSuperseded).to_string(),
        PlaybackRefused::NotRelayable => text::lookup(Text::FailureNotRelayable).to_string(),
        PlaybackRefused::Lapsed => text::lookup(Text::FailurePlaybackLapsed).to_string(),
        PlaybackRefused::NoTuner => text::lookup(Text::FailureNoTuner).to_string(),
        PlaybackRefused::TunerGone => text::lookup(Text::FailureTunerGone).to_string(),
        PlaybackRefused::LiveNotRelayable { shape } => {
            text::format(Text::FailureLiveNotRelayable, &[shape])
        }
        PlaybackRefused::TunerReleased => text::lookup(Text::FailureTunerReleased).to_string(),
    })
}

/// The report a Live TV access that offers nothing is shown as, and nothing
/// for `Allowed`.
pub fn live_tv_denied(access: LiveTvAccess) -> Option<crate::failure::Failure> {
    match access {
        LiveTvAccess::NoService => Some(told(Text::FailureLiveTvNoService)),
        LiveTvAccess::Denied => Some(told(Text::FailureLiveTvDenied)),
        LiveTvAccess::Allowed => None,
    }
}

/// Reads a non-2xx answer: a `Refusal` body is `Refused`, a `Failure` body is
/// `Upstream`, anything else is `Relay` carrying the status.
pub(crate) async fn classify(response: reqwest::Response) -> Trouble {
    let status = response.status();
    match response.text().await {
        Ok(body) => classify_body(status, &body),
        Err(error) => Trouble::Relay {
            status: Some(status.as_u16()),
            detail: error.to_string(),
        },
    }
}

/// What a relayed body carries.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(untagged)]
enum Said {
    Refused(Refusal),
    Failed(Failure),
    /// Any other JSON body, kept for the message it may hold under `detail`.
    Other(serde_json::Value),
}

/// The same classification over a body already read.
pub(crate) fn classify_body(status: reqwest::StatusCode, body: &str) -> Trouble {
    match crate::failure::unraised::decoded::<Said>(body) {
        Ok(Said::Refused(refusal)) => Trouble::Refused(refusal),
        Ok(Said::Failed(failure)) => Trouble::Upstream(failure),
        Ok(Said::Other(_)) | Err(_) => Trouble::Relay {
            status: Some(status.as_u16()),
            detail: body.to_owned(),
        },
    }
}

impl From<jellyfin_api::error::Error> for Trouble {
    fn from(error: jellyfin_api::error::Error) -> Trouble {
        match &error {
            jellyfin_api::error::Error::Status { status, body, .. } => classify_body(*status, body),
            _ => Trouble::Relay {
                status: None,
                detail: error.to_string(),
            },
        }
    }
}

impl From<reqwest::Error> for Trouble {
    fn from(error: reqwest::Error) -> Trouble {
        Trouble::Relay {
            status: None,
            detail: error.to_string(),
        }
    }
}

/// The report a report the local server would not act on is shown as.
pub fn live_refused(refusal: LiveRefusal) -> crate::failure::Failure {
    let key = match refusal {
        LiveRefusal::UnknownTarget => Text::FailureUnknownTarget,
        LiveRefusal::NotDriving => Text::FailureNotDriving,
        LiveRefusal::TargetRefused => Text::FailureTargetRefused,
        LiveRefusal::NotGrouped => Text::FailureNotGrouped,
        LiveRefusal::GroupRefused => Text::FailureGroupRefused,
        LiveRefusal::ReadOnly => Text::DashboardReadOnly,
    };
    told(key)
}

/// The report the end of remote mode is shown as.
pub fn remote_ended(cause: RemoteEnded) -> crate::failure::Failure {
    let key = match cause {
        RemoteEnded::Taken => Text::FailureRemoteTaken,
        RemoteEnded::TargetGone => Text::FailureRemoteTargetGone,
        RemoteEnded::Controlled => Text::FailureRemoteControlled,
        RemoteEnded::Grouped => Text::FailureRemoteGrouped,
    };
    told(key)
}

/// The report the end of group membership is shown as.
pub fn group_ended(cause: GroupEnded) -> crate::failure::Failure {
    let key = match cause {
        GroupEnded::Left => Text::FailureGroupLeft,
        GroupEnded::Removed => Text::FailureGroupRemoved,
        GroupEnded::Remote => Text::FailureGroupRemote,
        GroupEnded::NotInGroup => Text::FailureGroupNotInGroup,
        GroupEnded::NoSuchGroup => Text::FailureGroupNoSuchGroup,
        GroupEnded::LibraryDenied => Text::FailureGroupLibraryDenied,
    };
    told(key)
}

/// The report losing the transport to another tab is shown as.
pub fn displaced(cause: Displaced) -> crate::failure::Failure {
    let key = match cause {
        Displaced::Playback => Text::FailureDisplacedByPlayback,
        Displaced::Group => Text::FailureDisplacedByGroup,
    };
    told(key)
}

/// The report the Jellyfin server stopping is shown as, told apart from an
/// unreachable server.
pub fn server_stopping(restarting: bool) -> crate::failure::Failure {
    let key = if restarting {
        Text::FailureServerRestarting
    } else {
        Text::FailureServerShuttingDown
    };
    told(key)
}

/// The longest server message rendered.
pub const SERVER_LIMIT: usize = 512;

/// What an administrative write acted on, which is the sentence a refusal
/// renders through the lookup.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    CollectionCreate,
    CollectionRename,
    CollectionDelete,
    CollectionAdd,
    CollectionRemove,
    PlaylistCreate,
    PlaylistRename,
    PlaylistDelete,
    PlaylistAdd,
    PlaylistRemove,
    PlaylistMove,
    PlaylistShare,
    ItemSave,
    ItemDelete,
    ItemContentType,
    ItemIdentify,
    ItemImageUpload,
    ItemImageRemove,
    ItemImageMove,
    ItemImageDownload,
    Configuration,
    TunerAdd,
    TunerDelete,
    TunerReset,
    ProviderAdd,
    ProviderDelete,
    ChannelMapping,
    Dvr,
    RefreshItem,
    PackageInstall,
    PackageCancel,
    RepositoryAdd,
    RepositoryRemove,
    DeviceRename,
    DeviceDelete,
    /// Deleting this installation's own device, which ends the session it
    /// holds.
    OwnDeviceDelete,
    KeyCreate,
    KeyRevoke,
    TaskStart,
    TaskStop,
    TaskTriggers,
    UserCreate,
    UserSave,
    UserDelete,
    UserPassword,
    UserImage,
    LibraryCreate,
    LibraryRename,
    LibraryDelete,
    LibraryPath,
    LibraryOptions,
    Scan,
    Restart,
    Shutdown,
    Timer,
    SeriesTimer,
    PluginEnable,
    PluginDisable,
    PluginUninstall,
    PluginConfiguration,
    /// A wizard step's write, including the library writes the wizard makes.
    SetupStep,
    /// The display name write.
    UserName,
    /// The user configuration write.
    UserConfiguration,
    /// The preference bag write.
    Preferences,
    QuickConnect,
}

impl Operation {
    fn text(self) -> Text {
        match self {
            Operation::SetupStep => Text::FailureWroteSetupStep,
            Operation::UserName => Text::FailureWroteUserName,
            Operation::UserConfiguration => Text::FailureWroteUserConfiguration,
            Operation::Preferences => Text::FailureWrotePreferences,
            Operation::QuickConnect => Text::FailureWroteQuickConnect,
            Operation::CollectionCreate => Text::FailureWroteCollectionCreate,
            Operation::CollectionRename => Text::FailureWroteCollectionRename,
            Operation::CollectionDelete => Text::FailureWroteCollectionDelete,
            Operation::CollectionAdd => Text::FailureWroteCollectionAdd,
            Operation::CollectionRemove => Text::FailureWroteCollectionRemove,
            Operation::PlaylistCreate => Text::FailureWrotePlaylistCreate,
            Operation::PlaylistRename => Text::FailureWrotePlaylistRename,
            Operation::PlaylistDelete => Text::FailureWrotePlaylistDelete,
            Operation::PlaylistAdd => Text::FailureWrotePlaylistAdd,
            Operation::PlaylistRemove => Text::FailureWrotePlaylistRemove,
            Operation::PlaylistMove => Text::FailureWrotePlaylistMove,
            Operation::PlaylistShare => Text::FailureWrotePlaylistShare,
            Operation::ItemSave => Text::FailureWroteItemSave,
            Operation::ItemDelete => Text::FailureWroteItemDelete,
            Operation::ItemContentType => Text::FailureWroteItemContentType,
            Operation::ItemIdentify => Text::FailureWroteItemIdentify,
            Operation::ItemImageUpload => Text::FailureWroteItemImageUpload,
            Operation::ItemImageRemove => Text::FailureWroteItemImageRemove,
            Operation::ItemImageMove => Text::FailureWroteItemImageMove,
            Operation::ItemImageDownload => Text::FailureWroteItemImageDownload,
            Operation::Configuration => Text::FailureWroteConfiguration,
            Operation::TunerAdd => Text::FailureWroteTunerAdd,
            Operation::TunerDelete => Text::FailureWroteTunerDelete,
            Operation::TunerReset => Text::FailureWroteTunerReset,
            Operation::ProviderAdd => Text::FailureWroteProviderAdd,
            Operation::ProviderDelete => Text::FailureWroteProviderDelete,
            Operation::ChannelMapping => Text::FailureWroteChannelMapping,
            Operation::Dvr => Text::FailureWroteDvr,
            Operation::RefreshItem => Text::FailureWroteRefreshItem,
            Operation::PackageInstall => Text::FailureWrotePackageInstall,
            Operation::PackageCancel => Text::FailureWrotePackageCancel,
            Operation::RepositoryAdd => Text::FailureWroteRepositoryAdd,
            Operation::RepositoryRemove => Text::FailureWroteRepositoryRemove,
            Operation::DeviceRename => Text::FailureWroteDeviceRename,
            Operation::DeviceDelete | Operation::OwnDeviceDelete => Text::FailureWroteDeviceDelete,
            Operation::KeyCreate => Text::FailureWroteKeyCreate,
            Operation::KeyRevoke => Text::FailureWroteKeyRevoke,
            Operation::TaskStart => Text::FailureWroteTaskStart,
            Operation::TaskStop => Text::FailureWroteTaskStop,
            Operation::TaskTriggers => Text::FailureWroteTaskTriggers,
            Operation::UserCreate => Text::FailureWroteUserCreate,
            Operation::UserSave => Text::FailureWroteUserSave,
            Operation::UserDelete => Text::FailureWroteUserDelete,
            Operation::UserPassword => Text::FailureWroteUserPassword,
            Operation::UserImage => Text::FailureWroteUserImage,
            Operation::LibraryCreate => Text::FailureWroteLibraryCreate,
            Operation::LibraryRename => Text::FailureWroteLibraryRename,
            Operation::LibraryDelete => Text::FailureWroteLibraryDelete,
            Operation::LibraryPath => Text::FailureWroteLibraryPath,
            Operation::LibraryOptions => Text::FailureWroteLibraryOptions,
            Operation::Scan => Text::FailureWroteScan,
            Operation::Restart => Text::FailureWroteRestart,
            Operation::Shutdown => Text::FailureWroteShutdown,
            Operation::Timer => Text::FailureTimerRefused,
            Operation::SeriesTimer => Text::FailureSeriesTimerRefused,
            Operation::PluginEnable => Text::FailureWrotePluginEnable,
            Operation::PluginDisable => Text::FailureWrotePluginDisable,
            Operation::PluginUninstall => Text::FailureWrotePluginUninstall,
            Operation::PluginConfiguration => Text::FailureWrotePluginConfiguration,
        }
    }
}

/// One write, named by what it did and the object it did it to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Wrote {
    pub operation: Operation,
    pub object: String,
}

/// The sentence a refused write is shown as, naming the operation and its
/// object, over the server's own message.
fn write_refused(wrote: &Wrote, trouble: &Trouble) -> crate::failure::Failure {
    crate::failure::Failure {
        sentence: text::format(wrote.operation.text(), &[&wrote.object]),
        server: server_said(trouble),
        ..crate::failure::Failure::of(trouble)
    }
}

/// The Jellyfin server's own message, and `None` when the answer carried none.
/// A `ProblemDetails` body reads as its detail and then its title; any other
/// body reads as its text, trimmed to `SERVER_LIMIT` bytes.
pub(crate) fn server_said(trouble: &Trouble) -> Option<String> {
    let Trouble::Relay { detail, .. } = trouble else {
        return None;
    };
    let said = match crate::failure::unraised::decoded::<Said>(detail) {
        Ok(Said::Other(body)) => body
            .get("detail")
            .or_else(|| body.get("title"))
            .and_then(serde_json::Value::as_str)
            .map_or_else(|| detail.clone(), str::to_owned),
        _ => detail.clone(),
    };
    let said = said.trim();
    if said.is_empty() {
        return None;
    }
    let mut kept = said.to_owned();
    if kept.len() > SERVER_LIMIT {
        let mut at = SERVER_LIMIT;
        while at > 0 && !kept.is_char_boundary(at) {
            at -= 1;
        }
        kept.truncate(at);
    }
    Some(kept)
}

/// The sentence a bridge verb outside the nine is shown as, naming the verb.
pub fn bridge_refused(refused: &jellium_model::bridge::Refused) -> crate::failure::Failure {
    crate::failure::Failure::saying(
        text::format(Text::FailureBridgeVerb, &[&refused.verb]),
        crate::failure::Cause::Malformed {
            detail: format!("bridge verb {}", refused.verb),
        },
    )
}

/// The sentence an authorize that did not authorize is shown as, distinct for
/// an unknown code, an expired code, a code already used, and Quick Connect
/// switched off.
pub fn quick_connect_refused(
    outcome: jellium_model::quickconnect::Outcome,
) -> crate::failure::Failure {
    use jellium_model::quickconnect::Outcome;
    told(match outcome {
        Outcome::Authorized => Text::QuickConnectAuthorized,
        Outcome::Unknown => Text::FailureQuickConnectUnknown,
        Outcome::Expired => Text::FailureQuickConnectExpired,
        Outcome::Used => Text::FailureQuickConnectUsed,
        Outcome::Disabled => Text::FailureQuickConnectDisabled,
    })
}

/// The sentence a file the browser will not send is shown as, naming the type,
/// or the file's size and the cap.
pub fn upload_refused(refused: &jellium_model::upload::Refused) -> crate::failure::Failure {
    use jellium_model::upload::Refused;
    stated(match refused {
        Refused::Type { mime } => text::format(Text::FailureImageType, &[mime]),
        Refused::TooLarge { bytes, cap } => text::format(
            Text::FailureImageTooLarge,
            &[&bytes.to_string(), &cap.to_string()],
        ),
    })
}

/// The sentence `trouble` reads as, which only `Failure::of` renders.
pub fn sentence(trouble: &Trouble) -> String {
    trouble.message()
}

/// The report a sentence the local server's own event carried is shown as.
pub fn stated(sentence: String) -> crate::failure::Failure {
    crate::failure::Failure::saying(
        sentence.clone(),
        crate::failure::Cause::Http {
            status: None,
            body: sentence,
        },
    )
}

/// The report a string-table sentence the local server's answer carried is
/// shown as.
pub fn told(key: Text) -> crate::failure::Failure {
    stated(text::lookup(key).to_owned())
}

/// One answer from the local server, or from the Jellyfin server through it.
/// Its body crosses behind a pointer, so every message carrying an answer is
/// one width whatever the answer holds. An `Answer` gives up its value only to
/// a caller that propagates its trouble or names what becomes of it: it offers
/// no `ok`, no `unwrap_or_default` and no `Err` to match.
#[must_use = "the answer carries a trouble that must be raised or propagated"]
#[derive(Debug, Clone)]
pub struct Answer<T>(Result<Box<T>, Trouble>);

/// A trouble on its way to the request that will answer it. It carries no
/// accessor, no `Display` and no way out but `Answer::of`, so a caller holding
/// one can only return it.
#[must_use = "a bubbled trouble must be returned to the request that answers it"]
#[derive(Debug)]
pub struct Bubble(Trouble);

impl From<Trouble> for Bubble {
    fn from(trouble: Trouble) -> Bubble {
        Bubble(trouble)
    }
}

impl From<reqwest::Error> for Bubble {
    fn from(error: reqwest::Error) -> Bubble {
        Bubble(Trouble::from(error))
    }
}

impl From<jellyfin_api::error::Error> for Bubble {
    fn from(error: jellyfin_api::error::Error) -> Bubble {
        Bubble(Trouble::from(error))
    }
}

impl<T> Answer<T> {
    /// The answer `read` resolves to, which is what every request is built
    /// with.
    pub async fn of(read: impl Future<Output = Result<T, Bubble>>) -> Answer<T> {
        Answer(read.await.map(Box::new).map_err(|bubbled| bubbled.0))
    }

    /// The value, propagating the trouble as a `Bubble` that only `Answer::of`
    /// can absorb, so `?` inside a request body is the one way past it.
    pub fn bubbled(self) -> Result<T, Bubble> {
        self.0.map(|held| *held).map_err(Bubble)
    }

    /// The value; a trouble is raised as a passing failure naming `reading`,
    /// and answers `None`.
    pub fn or_none(self, reading: Text) -> Option<T> {
        match self.0 {
            Ok(held) => Some(*held),
            Err(trouble) => {
                crate::failure::raise(crate::failure::reading_failed(&trouble, reading));
                None
            }
        }
    }

    /// The value; a trouble is raised as a passing failure naming `reading`,
    /// and answers the type's default.
    pub fn or_default(self, reading: Text) -> T
    where
        T: Default,
    {
        self.or_none(reading).unwrap_or_default()
    }

    /// The value; a trouble is recorded under `reading` in the console and the
    /// session's failure list and shown above no view.
    pub fn disregarded(self, reading: Text) -> Option<T> {
        match self.0 {
            Ok(held) => Some(*held),
            Err(trouble) => {
                crate::failure::disregard(trouble, reading);
                None
            }
        }
    }

    /// The value; a trouble is raised as the refusal of `wrote`, naming the
    /// operation and its object over the Jellyfin server's own message, and
    /// answers `None`.
    /// This is what every administrative write reads its answer through.
    pub fn or_refused(self, wrote: &Wrote) -> Option<T> {
        match self.0 {
            Ok(held) => Some(*held),
            Err(trouble) => {
                crate::failure::raise(write_refused(wrote, &trouble));
                None
            }
        }
    }

    /// The answer with its value mapped and its trouble untouched.
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Answer<U> {
        Answer(self.0.map(|held| Box::new(f(*held))))
    }
}

/// The report a Jellyfin sign-in the local server relayed as failed is shown
/// as; a rejected token ends the session.
pub fn sign_in_failed(failure: &Failure) -> crate::failure::Failure {
    crate::failure::Failure::of(&Trouble::Upstream(failure.clone()))
}

/// The report this installation's own device being deleted is shown as; the
/// session it held is gone.
pub fn own_device_deleted() -> crate::failure::Failure {
    crate::failure::Failure::of(&Trouble::OwnDeviceDeleted)
}

/// The report the signed-in user being deleted is shown as; the session it
/// held is gone.
pub fn user_deleted() -> crate::failure::Failure {
    crate::failure::Failure::of(&Trouble::UserDeleted)
}
