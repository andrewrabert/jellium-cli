use axum::http::StatusCode;
use jellium_protocol::{Credentials, Failure, Refusal};
use uuid::Uuid;

use super::identity::Identity;
use super::link::{Link, forgotten, unreachable};
use super::version;

pub struct Upstream {
    pub state: jellium_protocol::Session,
    token: String,
    user_id: Uuid,
    /// The name the probe that opened this session reported.
    name: String,
    /// The link this session's requests are issued over.
    link: Link,
    /// The answer `System/Endpoint` gave this session, held the way
    /// `this._endPointInfo` is held.
    endpoint: tokio::sync::Mutex<Option<jellyfin_api::types::EndPointInfo>>,
}

/// What the user's policy lets them do with groups; a policy the answer omits
/// reads as `SyncAccess::None`.
fn sync_access(user: &jellyfin_api::types::UserDto) -> jellium_protocol::SyncAccess {
    use jellyfin_api::types::SyncPlayUserAccessType;
    match user
        .policy
        .as_ref()
        .and_then(|policy| policy.sync_play_access)
    {
        Some(SyncPlayUserAccessType::CreateAndJoinGroups) => {
            jellium_protocol::SyncAccess::CreateAndJoin
        }
        Some(SyncPlayUserAccessType::JoinGroups) => jellium_protocol::SyncAccess::Join,
        Some(SyncPlayUserAccessType::None) | None => jellium_protocol::SyncAccess::None,
    }
}

/// The sessions a listing names, as dashboard home shows them; this
/// installation's own session is marked rather than dropped.
pub fn server_sessions(
    sessions: &[jellyfin_api::types::SessionInfoDto],
    identity: &Identity,
) -> Vec<jellium_protocol::ServerSession> {
    let own = identity.device_id().to_owned();
    sessions
        .iter()
        .filter_map(|session| {
            Some(jellium_protocol::ServerSession {
                session: session.id.clone()?,
                device_name: session.device_name.clone().unwrap_or_default(),
                client_name: session.client.clone().unwrap_or_default(),
                user_name: session.user_name.clone().unwrap_or_default(),
                playing: session
                    .now_playing_item
                    .as_ref()
                    .and_then(|item| item.name.clone()),
                own: session.device_id.as_deref() == Some(own.as_str()),
            })
        })
        .collect()
}

/// True when the user's policy carries `IsAdministrator`; a policy the answer
/// omits reads as false.
fn administrator(user: &jellyfin_api::types::UserDto) -> bool {
    user.policy
        .as_ref()
        .and_then(|policy| policy.is_administrator)
        .unwrap_or(false)
}

/// True when the user's policy carries `EnableUserPreferenceAccess`; a policy
/// the answer omits reads as false.
fn preference_access(user: &jellyfin_api::types::UserDto) -> bool {
    user.policy
        .as_ref()
        .and_then(|policy| policy.enable_user_preference_access)
        .unwrap_or(false)
}

/// True when the Jellyfin server reports Quick Connect enabled; a call that
/// fails reads as false.
async fn quick_connect_of(client: &jellyfin_api::Client) -> bool {
    client.get_quick_connect_enabled().await.unwrap_or(false)
}

/// What Live TV `user` may see: `NoService` when `info` reports the service
/// disabled or carries no service, `Denied` when the user's policy denies Live
/// TV access, and `Allowed` otherwise.
/// A policy the answer omits reads as `Denied`.
fn live_tv_access(
    user: &jellyfin_api::types::UserDto,
    info: &jellyfin_api::types::LiveTvInfo,
) -> jellium_protocol::LiveTvAccess {
    use jellium_protocol::LiveTvAccess;
    if !info.is_enabled.unwrap_or(false) || info.services.is_empty() {
        return LiveTvAccess::NoService;
    }
    match user
        .policy
        .as_ref()
        .and_then(|policy| policy.enable_live_tv_access)
    {
        Some(true) => LiveTvAccess::Allowed,
        Some(false) | None => LiveTvAccess::Denied,
    }
}

/// What Live TV this session offers; a `/LiveTv/Info` call that fails reads as
/// `LiveTvAccess::NoService`.
async fn live_tv_of(
    client: &jellyfin_api::Client,
    user: &jellyfin_api::types::UserDto,
) -> jellium_protocol::LiveTvAccess {
    match client.get_live_tv_info().await {
        Ok(info) => live_tv_access(user, &info),
        Err(_) => jellium_protocol::LiveTvAccess::NoService,
    }
}

/// Revokes a saved record's token without holding a session for it; true when
/// the Jellyfin server took it, and true for a token it no longer knows.
pub async fn revoked(identity: &Identity, session: &crate::session::Session) -> bool {
    let Some(link) = Link::signed(identity, &session.server, &session.token) else {
        return false;
    };
    match link.control().report_session_ended().await {
        Ok(()) => true,
        Err(error) => forgotten(&error),
    }
}

impl Upstream {
    /// `/Users/AuthenticateByName` is asked over `device`'s identity with an
    /// empty token; the token the answer carries builds the signed link.
    pub async fn login(
        identity: &Identity,
        server: &str,
        credentials: &Credentials,
        probed: &version::Probed,
    ) -> Result<Upstream, Failure> {
        let server = server.trim_end_matches('/').to_string();
        let identified = Link::identified(identity, &server).ok_or_else(|| {
            unreachable(
                &server,
                "the server text is not an http url or the device identity \
                 cannot be sent in a header",
            )
        })?;

        let result = identified
            .control()
            .authenticate_user_by_name(&jellyfin_api::types::AuthenticateUserByName {
                pw: Some(credentials.password.clone()),
                username: Some(credentials.username.clone()),
            })
            .await
            .map_err(|e| identified.failed(e, Failure::CredentialsRejected))?;

        let token = result
            .access_token
            .ok_or_else(|| unreachable(&server, "no access token in the auth response"))?;
        let user = result
            .user
            .ok_or_else(|| unreachable(&server, "no user in the auth response"))?;
        let user_id = user
            .id
            .ok_or_else(|| unreachable(&server, "no user id in the auth response"))?;

        let link = Link::signed(identity, &server, &token).ok_or_else(|| {
            unreachable(
                &server,
                "the server's access token cannot be sent in a header",
            )
        })?;

        let live_tv = live_tv_of(&link.control(), &user).await;
        let quick_connect = quick_connect_of(&link.control()).await;

        Ok(Upstream {
            state: jellium_protocol::Session {
                server,
                user_id,
                sync_play: sync_access(&user),
                live_tv,
                administrator: administrator(&user),
                preference_access: preference_access(&user),
                quick_connect,
                read_only: false,
                device: identity.device_id().to_owned(),
                client: jellium_protocol::CLIENT.to_owned(),
                user_name: user.name.unwrap_or_default(),
                server_version: probed.version.clone(),
                snapshot_version: jellyfin_api::SNAPSHOT_VERSION.to_string(),
            },
            link,
            token,
            user_id,
            name: probed.name.clone(),
            endpoint: tokio::sync::Mutex::new(None),
        })
    }

    /// `/Users/AuthenticateWithQuickConnect` is asked over `device`'s identity
    /// with an empty token, so the token it mints is this installation's.
    pub async fn quick_connect(
        identity: &Identity,
        server: &str,
        secret: &str,
        probed: &version::Probed,
    ) -> Result<Upstream, Failure> {
        let server = server.trim_end_matches('/').to_string();
        let identified = Link::identified(identity, &server).ok_or_else(|| {
            unreachable(
                &server,
                "the server text is not an http url or the device identity \
                 cannot be sent in a header",
            )
        })?;

        let result = identified
            .control()
            .authenticate_with_quick_connect(&jellyfin_api::types::QuickConnectDto {
                secret: secret.to_string(),
            })
            .await
            .map_err(|e| identified.failed(e, Failure::CredentialsRejected))?;

        let token = result
            .access_token
            .ok_or_else(|| unreachable(&server, "no access token in the auth response"))?;
        let user = result
            .user
            .ok_or_else(|| unreachable(&server, "no user in the auth response"))?;
        let user_id = user
            .id
            .ok_or_else(|| unreachable(&server, "no user id in the auth response"))?;

        let link = Link::signed(identity, &server, &token).ok_or_else(|| {
            unreachable(
                &server,
                "the server's access token cannot be sent in a header",
            )
        })?;

        let live_tv = live_tv_of(&link.control(), &user).await;
        let quick_connect = quick_connect_of(&link.control()).await;

        Ok(Upstream {
            state: jellium_protocol::Session {
                server,
                user_id,
                sync_play: sync_access(&user),
                live_tv,
                administrator: administrator(&user),
                preference_access: preference_access(&user),
                quick_connect,
                read_only: false,
                device: identity.device_id().to_owned(),
                client: jellium_protocol::CLIENT.to_owned(),
                user_name: user.name.unwrap_or_default(),
                server_version: probed.version.clone(),
                snapshot_version: jellyfin_api::SNAPSHOT_VERSION.to_string(),
            },
            link,
            token,
            user_id,
            name: probed.name.clone(),
            endpoint: tokio::sync::Mutex::new(None),
        })
    }

    /// The name the probe that opened this session reported.
    pub fn name(&self) -> &str {
        &self.name
    }

    pub async fn resume(
        identity: &Identity,
        session: &crate::session::Session,
        probed: &version::Probed,
    ) -> Result<Upstream, Failure> {
        let server = session.server.trim_end_matches('/').to_string();
        let link = Link::signed(identity, &server, &session.token).ok_or(Failure::TokenRejected)?;
        let client = link.control();

        let user = client
            .get_current_user()
            .await
            .map_err(|e| link.failed(e, Failure::TokenRejected))?;
        let user_id = user.id.unwrap_or(session.user_id);

        let live_tv = live_tv_of(&client, &user).await;
        let quick_connect = quick_connect_of(&client).await;
        Ok(Upstream {
            state: jellium_protocol::Session {
                server,
                user_id,
                sync_play: sync_access(&user),
                live_tv,
                administrator: administrator(&user),
                preference_access: preference_access(&user),
                quick_connect,
                read_only: false,
                device: identity.device_id().to_owned(),
                client: jellium_protocol::CLIENT.to_owned(),
                user_name: user.name.unwrap_or_default(),
                server_version: probed.version.clone(),
                snapshot_version: jellyfin_api::SNAPSHOT_VERSION.to_string(),
            },
            token: session.token.clone(),
            user_id,
            name: probed.name.clone(),
            link,
            endpoint: tokio::sync::Mutex::new(None),
        })
    }

    /// The document Jellyfin serves for the configuration page `name`, read up
    /// to `page::PAGE_LIMIT`.
    /// A larger document reads as `Refusal::PageTooLarge`, and a document the
    /// Jellyfin server would not serve as `Refusal::PageNotListed`.
    pub async fn configuration_page(&self, name: &str) -> Result<String, Refusal> {
        let mut response = self
            .link
            .streaming()
            .request(reqwest::Method::GET, "/web/ConfigurationPage".into())
            .query("name", name)
            .send_response()
            .await
            .map_err(|_| Refusal::PageNotListed)?;

        let mut body = Vec::new();
        while let Some(chunk) = response.chunk().await.map_err(|_| Refusal::PageNotListed)? {
            if body.len() + chunk.len() > crate::web::page::PAGE_LIMIT {
                return Err(Refusal::PageTooLarge);
            }
            body.extend_from_slice(&chunk);
        }
        String::from_utf8(body).map_err(|_| Refusal::PageNotListed)
    }

    /// Every session on the server, as dashboard home shows them.
    pub async fn sessions(
        &self,
        identity: &Identity,
    ) -> Result<Vec<jellium_protocol::ServerSession>, Failure> {
        let sessions = self
            .control()
            .get_sessions(None, None, None)
            .await
            .map_err(|e| self.failed(e))?;
        Ok(server_sessions(&sessions, identity))
    }

    /// Every scheduled task, as the task list shows them.
    pub async fn tasks(&self) -> Result<Vec<jellium_protocol::TaskState>, Failure> {
        let tasks = self
            .control()
            .get_tasks(None, None)
            .await
            .map_err(|e| self.failed(e))?;
        Ok(tasks
            .into_iter()
            .filter_map(jellium_model::task::taken)
            .collect())
    }

    /// The newest `limit` activity entries.
    pub async fn activity(
        &self,
        limit: i32,
    ) -> Result<Vec<jellium_protocol::ActivityEntry>, Failure> {
        let answered = self
            .control()
            .get_log_entries(None, Some(limit), None, Some(0))
            .await
            .map_err(|e| self.failed(e))?;
        Ok(answered
            .items
            .into_iter()
            .filter_map(crate::web::live::activity_entry)
            .collect())
    }

    pub fn session(&self) -> crate::session::Session {
        crate::session::Session {
            server: self.state.server.clone(),
            token: self.token.clone(),
            user_id: self.user_id,
        }
    }

    pub async fn logout(&self) -> Result<(), Failure> {
        match self.control().report_session_ended().await {
            Ok(()) => Ok(()),
            Err(error) if forgotten(&error) => Ok(()),
            Err(error) => Err(unreachable(&self.state.server, error)),
        }
    }

    /// The link this session's requests are issued over.
    pub fn link(&self) -> &Link {
        &self.link
    }

    /// The Jellyfin client bound to this session, carrying `CONTROL_TIMEOUT`.
    pub fn control(&self) -> jellyfin_api::Client {
        self.link.control()
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    /// The access token, for the one query string that carries `ApiKey`.
    pub fn api_key(&self) -> &str {
        &self.token
    }

    /// What this session's connection to the Jellyfin server looks like from
    /// the server's side.
    /// Memoized for the session the way `this._endPointInfo` is, so a
    /// negotiation over five versions issues one `GET System/Endpoint` and not
    /// five.
    // reference: get-endpoint-info — apiClient.js:3864-3875
    pub async fn endpoint(&self) -> Result<jellyfin_api::types::EndPointInfo, Failure> {
        let mut held = self.endpoint.lock().await;
        if let Some(saved) = held.as_ref() {
            return Ok(saved.clone());
        }
        let answered: jellyfin_api::types::EndPointInfo =
            crate::web::wire::got(self, "System/Endpoint", &crate::web::wire::Query::new()).await?;
        *held = Some(answered.clone());
        Ok(answered)
    }

    /// A 401 or 403 reads as `Failure::TokenRejected`; anything else reads as
    /// `Failure::ServerUnreachable`.
    pub fn failed(&self, error: jellyfin_api::error::Error) -> Failure {
        self.link.failed(error, Failure::TokenRejected)
    }

    /// Declares audio and video playable, and — when `controllable` —
    /// `SupportsMediaControl` with `live::verbs::honoured(live_tv)`; without
    /// it, no media control and no supported command.
    pub async fn declare_capabilities(
        &self,
        controllable: bool,
        live_tv: bool,
    ) -> Result<(), Failure> {
        let commands = if controllable {
            crate::web::live::verbs::honoured(live_tv)
        } else {
            Vec::new()
        };
        self.control()
            .post_capabilities(
                None,
                Some(&vec![
                    jellyfin_api::types::MediaType::Audio,
                    jellyfin_api::types::MediaType::Video,
                ]),
                Some(&commands),
                Some(controllable),
                Some(true),
            )
            .await
            .map_err(|e| self.failed(e))
    }

    /// The Jellyfin websocket url: the link's base with a `ws` or `wss` scheme,
    /// `/socket`, the access token and `device`'s identifier.
    /// It never leaves the local server.
    pub fn socket_url(&self, identity: &Identity) -> String {
        let mut url = self.link.base().clone();
        let scheme = if url.scheme() == "https" { "wss" } else { "ws" };
        let _ = url.set_scheme(scheme);
        url.set_query(None);
        url.set_fragment(None);
        let path = format!("{}/socket", url.path().trim_end_matches('/'));
        url.set_path(&path);
        url.query_pairs_mut()
            .append_pair("api_key", &self.token)
            .append_pair("deviceId", identity.device_id());
        url.into()
    }

    /// One `/GetUtcTime` exchange: the instant the request left, the two
    /// instants the answer carries, and the instant the answer arrived.
    pub async fn utc_time(&self) -> Result<jellyfin_api::types::UtcTimeResponse, Failure> {
        self.control()
            .get_utc_time()
            .await
            .map_err(|e| self.failed(e))
    }

    /// The HTTP client with no total deadline, which the bitrate ladder times
    /// against.
    pub fn streaming(&self) -> &reqwest::Client {
        self.link.transport()
    }
}

pub fn status_for(failure: &Failure) -> StatusCode {
    match failure {
        Failure::CredentialsRejected | Failure::TokenRejected => StatusCode::UNAUTHORIZED,
        Failure::ServerBelowMinimum { .. } => StatusCode::CONFLICT,
        Failure::SetupSignInFailed => StatusCode::UNAUTHORIZED,
        Failure::ServerUnreachable { .. } => StatusCode::BAD_GATEWAY,
    }
}

#[cfg(test)]
use axum::body::Body;
#[cfg(test)]
use axum::http::{HeaderMap, HeaderName, header};
#[cfg(test)]
use axum::response::Response;
#[cfg(test)]
use std::time::Duration;

#[cfg(test)]
pub struct Answering {
    /// `http://127.0.0.1:<port>`, the base the listener serves.
    pub base: String,
    /// Counted before each answer is written.
    pub requests: std::sync::Arc<std::sync::atomic::AtomicUsize>,
    /// Every path asked for with its query, in the order it was asked for.
    pub queried: std::sync::Arc<std::sync::Mutex<Vec<String>>>,
    /// The headers of every request, in the order they arrived.
    pub headers: std::sync::Arc<std::sync::Mutex<Vec<HeaderMap>>>,
    /// Every frame a tab-side socket received, in the order it arrived.
    pub inbound: std::sync::Arc<std::sync::Mutex<Vec<String>>>,
    /// How many upstream sockets have been opened at `/socket`.
    pub sockets: std::sync::Arc<std::sync::atomic::AtomicUsize>,
    /// How far ahead of this machine `/GetUtcTime` answers, and how long it
    /// waits before answering.
    pub clock: std::sync::Arc<std::sync::Mutex<(i64, Duration)>>,
    /// The synthetic Live TV service mounted under `/LiveTv`.
    pub live_tv: super::synthetic::LiveTv,
    /// The synthetic dashboard the administrative routes answer from.
    pub dashboard: super::synthetic::Dashboard,
    /// The synthetic server in startup mode.
    pub startup: super::synthetic::Startup,
    /// The synthetic login area.
    pub login: super::synthetic::Login,
    /// The synthetic library area.
    pub library: super::synthetic::Library,
    /// What every request the stub took carried.
    pub taken: super::synthetic::Taken,
    /// One sender per open socket, and the generation a close bumps.
    open: std::sync::Arc<std::sync::Mutex<Vec<tokio::sync::mpsc::UnboundedSender<Pushed>>>>,
}

/// What the stub server does to an open socket.
#[cfg(test)]
#[derive(Clone)]
enum Pushed {
    Frame(String),
    Close,
}

#[cfg(test)]
impl Answering {
    /// How many times `path` was asked, whether a synthetic route or the
    /// fallback answered it.
    pub fn asked(&self, path: &str) -> usize {
        self.taken
            .tokenless()
            .into_iter()
            .chain(self.taken.credentialed())
            .filter(|asked| asked.as_str() == path)
            .count()
    }

    /// The value the request at `index` carried for `name`.
    pub fn header(&self, index: usize, name: &HeaderName) -> Option<String> {
        self.headers
            .lock()
            .expect("the recorded headers")
            .get(index)?
            .get(name)
            .and_then(|value| value.to_str().ok())
            .map(str::to_string)
    }

    /// Every query asked at `path`, in the order it was asked.
    pub fn queries(&self, path: &str) -> Vec<String> {
        self.queried
            .lock()
            .expect("the recorded queries")
            .iter()
            .filter_map(|asked| asked.strip_prefix(path)?.strip_prefix('?'))
            .map(str::to_string)
            .collect()
    }

    /// Answers `/GetUtcTime` `ahead` milliseconds ahead of this machine's
    /// clock, `after` a delay.
    pub fn skewed(&self, ahead: i64, after: Duration) {
        *self.clock.lock().expect("the stub clock") = (ahead, after);
    }

    /// Sends one raw Jellyfin frame to every open socket.
    pub fn push(&self, frame: &str) {
        let open = self.open.lock().expect("the open sockets");
        for socket in open.iter() {
            let _ = socket.send(Pushed::Frame(frame.to_string()));
        }
    }

    /// Resolves once `count` sockets have been opened.
    pub async fn opened(&self, count: usize) {
        use std::sync::atomic::Ordering;
        for _ in 0..2_000 {
            if self.sockets.load(Ordering::SeqCst) >= count {
                return;
            }
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
        panic!(
            "only {} of {count} sockets opened",
            self.sockets.load(Ordering::SeqCst)
        );
    }

    /// Closes every open socket, which is what a dropped link looks like.
    pub fn drop_sockets(&self) {
        let mut open = self.open.lock().expect("the open sockets");
        for socket in open.drain(..) {
            let _ = socket.send(Pushed::Close);
        }
    }
}

#[cfg(test)]
/// Binds a loopback listener serving the stub upstream's areas, answering every
/// path outside them with `status` and no body.
pub async fn answering(status: u16) -> Answering {
    answering_with(status, &[], "").await
}

#[cfg(test)]
/// Binds a loopback listener serving the stub upstream's areas, answering every
/// path outside them with `status`, the response headers `headers` names, and
/// `body`, and serving a websocket at `/socket`.
pub async fn answering_with(
    status: u16,
    headers: &'static [(&'static str, &'static str)],
    body: &'static str,
) -> Answering {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    let requests = Arc::new(AtomicUsize::new(0));
    let recorded = Arc::new(std::sync::Mutex::new(Vec::new()));
    let queried = Arc::new(std::sync::Mutex::new(Vec::new()));
    let inbound = Arc::new(std::sync::Mutex::new(Vec::new()));
    let sockets = Arc::new(AtomicUsize::new(0));
    let open: Arc<std::sync::Mutex<Vec<tokio::sync::mpsc::UnboundedSender<Pushed>>>> =
        Arc::new(std::sync::Mutex::new(Vec::new()));
    let counter = requests.clone();
    let querier = queried.clone();
    let noter = recorded.clone();
    let code = StatusCode::from_u16(status).expect("a valid status");

    let socket_inbound = inbound.clone();
    let socket_count = sockets.clone();
    let socket_open = open.clone();
    let socketed = move |upgrade: axum::extract::ws::WebSocketUpgrade| {
        let socket_inbound = socket_inbound.clone();
        let socket_count = socket_count.clone();
        let socket_open = socket_open.clone();
        async move {
            // a stub told to refuse everything refuses the handshake too,
            // which is what a rejected access token looks like
            use axum::response::IntoResponse as _;
            if code == StatusCode::UNAUTHORIZED || code == StatusCode::FORBIDDEN {
                return code.into_response();
            }
            upgrade
                .on_upgrade(move |mut socket| async move {
                    use axum::extract::ws::Message;
                    use futures_util::StreamExt;

                    let (pushes, mut pushed) = tokio::sync::mpsc::unbounded_channel();
                    socket_open
                        .lock()
                        .expect("the open sockets")
                        .push(pushes.clone());
                    socket_count.fetch_add(1, Ordering::SeqCst);
                    loop {
                        tokio::select! {
                            push = pushed.recv() => match push {
                                Some(Pushed::Frame(frame)) => {
                                    if socket.send(Message::Text(frame.into())).await.is_err() {
                                        break;
                                    }
                                }
                                Some(Pushed::Close) | None => break,
                            },
                            received = socket.next() => match received {
                                Some(Ok(Message::Text(text))) => socket_inbound
                                    .lock()
                                    .expect("the received frames")
                                    .push(text.to_string()),
                                Some(Ok(_)) => {}
                                Some(Err(_)) | None => break,
                            },
                        }
                    }
                    let mut held = socket_open.lock().expect("the open sockets");
                    held.retain(|socket| !socket.same_channel(&pushes));
                })
                .into_response()
        }
    };

    let super::synthetic::Synthetic {
        router: synthetic_router,
        live_tv,
        dashboard,
        startup,
        login,
        library,
        taken,
    } = super::synthetic::router();
    let clock = Arc::new(std::sync::Mutex::new((0i64, Duration::ZERO)));
    let timing = clock.clone();
    let taken_timed = taken.clone();
    let timed_queries = queried.clone();
    let timed_requests = requests.clone();
    let timed = move |headers: HeaderMap| {
        let timing = timing.clone();
        let taken_timed = taken_timed.clone();
        let timed_queries = timed_queries.clone();
        let timed_requests = timed_requests.clone();
        async move {
            timed_requests.fetch_add(1, Ordering::SeqCst);
            taken_timed.record("/GetUtcTime", &headers);
            timed_queries
                .lock()
                .expect("the recorded queries")
                .push("/GetUtcTime".to_string());
            let (ahead, after) = *timing.lock().expect("the stub clock");
            let received = chrono::Utc::now() + chrono::Duration::milliseconds(ahead);
            if !after.is_zero() {
                tokio::time::sleep(after).await;
            }
            let answered = chrono::Utc::now() + chrono::Duration::milliseconds(ahead);
            axum::Json(jellyfin_api::types::UtcTimeResponse {
                request_reception_time: Some(received),
                response_transmission_time: Some(answered),
            })
        }
    };

    let recording = taken.clone();
    let app = axum::Router::new()
        .merge(synthetic_router)
        .route("/socket", axum::routing::get(socketed))
        .route("/GetUtcTime", axum::routing::get(timed))
        .fallback(move |request: axum::extract::Request| {
            let counter = counter.clone();
            let querier = querier.clone();
            let noter = noter.clone();
            let asked = request
                .uri()
                .path_and_query()
                .map(ToString::to_string)
                .unwrap_or_default();
            let path = request.uri().path().to_string();
            let sent = request.headers().clone();
            let recording = recording.clone();
            async move {
                recording.record(&path, &sent);
                counter.fetch_add(1, Ordering::SeqCst);
                querier.lock().expect("the recorded queries").push(asked);
                noter.lock().expect("the recorded headers").push(sent);
                let mut answer = Response::builder().status(code);
                for (name, value) in headers {
                    answer = answer.header(*name, *value);
                }
                answer.body(Body::from(body)).expect("an answer")
            }
        });
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind a loopback listener");
    let port = listener.local_addr().expect("the bound address").port();
    let base = format!("http://127.0.0.1:{port}");
    library.based(&base);
    tokio::spawn(async move {
        axum::serve(listener, app).await.expect("serve");
    });
    Answering {
        base,
        requests,
        queried,
        headers: recorded,
        inbound,
        sockets,
        clock,
        live_tv,
        dashboard,
        startup,
        login,
        library,
        taken,
        open,
    }
}

#[cfg(test)]
impl Upstream {
    // an upstream naming `server` with a placeholder token and identity
    pub fn stub(server: &str) -> Upstream {
        let identity = Identity::of(jellium_protocol::Identity {
            device: "Firefox".to_owned(),
            device_id: Uuid::nil().to_string(),
        });
        let server = server.trim_end_matches('/').to_string();
        Upstream {
            name: String::new(),
            state: jellium_protocol::Session {
                server: server.clone(),
                user_id: Uuid::nil(),
                sync_play: jellium_protocol::SyncAccess::CreateAndJoin,
                live_tv: jellium_protocol::LiveTvAccess::Allowed,
                administrator: true,
                preference_access: true,
                quick_connect: true,
                read_only: false,
                device: identity.device_id().to_owned(),
                client: jellium_protocol::CLIENT.to_owned(),
                user_name: String::new(),
                server_version: String::new(),
                snapshot_version: jellyfin_api::SNAPSHOT_VERSION.to_string(),
            },
            token: "token".to_string(),
            user_id: Uuid::nil(),
            link: Link::signed(&identity, &server, "token").expect("the stub server is a url"),
            endpoint: tokio::sync::Mutex::new(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::web::route;
    use std::sync::atomic::Ordering;

    /// The relay forwarding one request to the stub, as the browser reaches it.
    async fn through(
        upstream: &Upstream,
        method: reqwest::Method,
        path: &str,
        seen: &route::Seen,
    ) -> axum::response::Response {
        let target = route::Target::admit(&method, path.split('?').next().unwrap_or(path), seen)
            .unwrap_or_else(|| panic!("{path} is relayed"));
        upstream
            .link
            .forward(
                &target,
                path.split_once('?').map(|(_, query)| query),
                &axum::http::HeaderMap::new(),
                axum::body::Bytes::new(),
                seen,
                &crate::web::playback::pointed::Pointed::new(),
            )
            .await
            .expect("the stub answers")
    }

    async fn read(response: axum::response::Response) -> String {
        let bytes = axum::body::to_bytes(response.into_body(), 1 << 20)
            .await
            .expect("a body");
        String::from_utf8_lossy(&bytes).into_owned()
    }

    #[tokio::test]
    async fn a_playlist_holds_two_copies_of_one_item_told_apart_by_entry_id() {
        let server = answering(204).await;
        use crate::web::synthetic::{Entry, Library};
        let entries = server.library.entries(Library::PLAYLIST);
        assert_eq!(
            entries,
            vec![
                Entry {
                    item: Library::ITEM,
                    entry: Library::FIRST.to_string(),
                },
                Entry {
                    item: Library::ITEM,
                    entry: Library::SECOND.to_string(),
                },
            ],
            "the same item is filed twice and told apart by entry id"
        );
    }

    #[tokio::test]
    async fn removing_one_playlist_entry_leaves_the_other_copy_standing() {
        let server = answering(204).await;
        let upstream = Upstream::stub(&server.base);
        let seen = route::Seen::new();
        let playlist = crate::web::synthetic::Library::PLAYLIST;

        let first = crate::web::synthetic::Library::FIRST;
        let path = format!("/Playlists/{playlist}/Items?entryIds={first}");
        let answered = through(&upstream, reqwest::Method::DELETE, &path, &seen).await;
        assert!(answered.status().is_success());

        let entries = server.library.entries(playlist);
        assert_eq!(entries.len(), 1, "one copy was removed, not both");
        assert_eq!(
            entries[0].entry,
            crate::web::synthetic::Library::SECOND.to_string()
        );
        assert_eq!(entries[0].item, crate::web::synthetic::Library::ITEM);
    }

    #[tokio::test]
    async fn a_playlist_entry_moves_into_the_order_a_later_read_shows() {
        let server = answering(204).await;
        let upstream = Upstream::stub(&server.base);
        let seen = route::Seen::new();
        let playlist = crate::web::synthetic::Library::PLAYLIST;

        let second = crate::web::synthetic::Library::SECOND;
        let path = format!("/Playlists/{playlist}/Items/{second}/Move/0");
        let answered = through(&upstream, reqwest::Method::POST, &path, &seen).await;
        assert!(answered.status().is_success());

        let entries = server.library.entries(playlist);
        assert_eq!(
            entries[0].entry,
            crate::web::synthetic::Library::SECOND.to_string()
        );
        assert_eq!(
            entries[1].entry,
            crate::web::synthetic::Library::FIRST.to_string()
        );
    }

    #[tokio::test]
    async fn an_item_is_added_to_and_removed_from_a_collection() {
        let server = answering(204).await;
        let upstream = Upstream::stub(&server.base);
        let seen = route::Seen::new();
        let collection = crate::web::synthetic::Library::COLLECTION;
        let added = Uuid::from_u128(0x9100);

        let path = format!("/Collections/{collection}/Items?ids={added}");
        through(&upstream, reqwest::Method::POST, &path, &seen).await;
        assert!(server.library.collected(collection).contains(&added));

        through(&upstream, reqwest::Method::DELETE, &path, &seen).await;
        assert!(!server.library.collected(collection).contains(&added));
    }

    #[tokio::test]
    async fn a_remote_search_answer_carries_no_provider_url_and_mints_a_handle() {
        let server = answering(204).await;
        let upstream = Upstream::stub(&server.base);
        let seen = route::Seen::new();

        let answered = through(
            &upstream,
            reqwest::Method::POST,
            "/Items/RemoteSearch/Movie",
            &seen,
        )
        .await;
        let body = read(answered).await;

        for url in server.library.foreign() {
            assert!(!body.contains(&url), "{url} reached the browser");
        }
        assert!(
            !body.contains("http://"),
            "a provider url reached the browser"
        );

        let value: serde_json::Value = serde_json::from_str(&body).expect("json");
        let handle = value[0]["ImageUrl"].as_str().expect("a handle");
        assert_eq!(
            seen.observed(handle).as_deref(),
            Some(server.library.foreign()[0].as_str())
        );
    }

    #[tokio::test]
    async fn a_remote_images_answer_carries_no_provider_url() {
        let server = answering(204).await;
        let upstream = Upstream::stub(&server.base);
        let seen = route::Seen::new();
        let item = crate::web::synthetic::Library::ITEM;

        let answered = through(
            &upstream,
            reqwest::Method::GET,
            &format!("/Items/{item}/RemoteImages"),
            &seen,
        )
        .await;
        let body = read(answered).await;

        for url in server.library.foreign() {
            assert!(!body.contains(&url), "{url} reached the browser");
        }
        assert!(!body.contains("http://"));
    }

    #[tokio::test]
    async fn a_download_naming_a_handle_this_run_did_not_mint_is_refused() {
        let server = answering(204).await;
        let upstream = Upstream::stub(&server.base);
        let seen = route::Seen::new();
        let item = crate::web::synthetic::Library::ITEM;

        let answered = through(
            &upstream,
            reqwest::Method::POST,
            &format!("/Items/{item}/RemoteImages/Download?type=Primary&imageUrl=fdeadbeef"),
            &seen,
        )
        .await;
        assert_eq!(answered.status(), axum::http::StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn a_socket_coming_up_declares_media_control_and_the_honoured_commands() {
        let server = answering(204).await;
        let upstream = Upstream::stub(&server.base);
        upstream
            .declare_capabilities(true, true)
            .await
            .expect("the declaration is accepted");
        let queries = server.queries("/Sessions/Capabilities");
        let declared = queries.first().expect("one declaration");
        assert!(declared.contains("supportsMediaControl=true"));
        for verb in crate::web::live::verbs::HONOURED {
            assert!(declared.contains(&verb.to_string()), "{verb} not declared");
        }
    }

    #[tokio::test]
    async fn a_socket_coming_up_declares_the_live_tv_verbs_when_live_tv_is_available() {
        let server = answering(204).await;
        let upstream = Upstream::stub(&server.base);
        upstream
            .declare_capabilities(true, true)
            .await
            .expect("the declaration is accepted");
        let queries = server.queries("/Sessions/Capabilities");
        let declared = queries.first().expect("one declaration");
        for verb in crate::web::live::verbs::LIVE_TV {
            assert!(declared.contains(&verb.to_string()), "{verb} not declared");
        }
    }

    #[tokio::test]
    async fn a_socket_coming_up_declares_no_live_tv_verb_without_live_tv() {
        let server = answering(204).await;
        let upstream = Upstream::stub(&server.base);
        upstream
            .declare_capabilities(true, false)
            .await
            .expect("the declaration is accepted");
        let queries = server.queries("/Sessions/Capabilities");
        let declared = queries.first().expect("one declaration");
        for verb in crate::web::live::verbs::LIVE_TV {
            assert!(!declared.contains(&verb.to_string()), "{verb} was declared");
        }
    }

    fn user_allowing_live_tv(allowed: Option<bool>) -> jellyfin_api::types::UserDto {
        let policy = serde_json::from_value(serde_json::json!({
            "AuthenticationProviderId": "",
            "PasswordResetProviderId": "",
            "EnableCollectionManagement": false,
            "EnableLyricManagement": false,
            "EnableSubtitleManagement": false,
            "EnableLiveTvAccess": allowed,
        }))
        .expect("a user policy");
        jellyfin_api::types::UserDto {
            policy: Some(policy),
            ..jellyfin_api::types::UserDto::default()
        }
    }

    fn service_running() -> jellyfin_api::types::LiveTvInfo {
        jellyfin_api::types::LiveTvInfo {
            is_enabled: Some(true),
            services: vec![jellyfin_api::types::LiveTvServiceInfo::default()],
            ..jellyfin_api::types::LiveTvInfo::default()
        }
    }

    #[tokio::test]
    async fn a_user_whose_policy_denies_live_tv_reads_as_denied() {
        use jellium_protocol::LiveTvAccess;
        assert_eq!(
            live_tv_access(&user_allowing_live_tv(Some(false)), &service_running()),
            LiveTvAccess::Denied
        );
        assert_eq!(
            live_tv_access(&jellyfin_api::types::UserDto::default(), &service_running()),
            LiveTvAccess::Denied
        );
        assert_eq!(
            live_tv_access(&user_allowing_live_tv(Some(true)), &service_running()),
            LiveTvAccess::Allowed
        );
    }

    #[tokio::test]
    async fn a_server_with_no_live_tv_service_reads_as_no_service() {
        use jellium_protocol::LiveTvAccess;
        let user = user_allowing_live_tv(Some(true));
        assert_eq!(
            live_tv_access(&user, &jellyfin_api::types::LiveTvInfo::default()),
            LiveTvAccess::NoService
        );
        assert_eq!(
            live_tv_access(
                &user,
                &jellyfin_api::types::LiveTvInfo {
                    is_enabled: Some(false),
                    services: vec![jellyfin_api::types::LiveTvServiceInfo::default()],
                    ..jellyfin_api::types::LiveTvInfo::default()
                }
            ),
            LiveTvAccess::NoService
        );
        assert_eq!(
            live_tv_access(
                &user,
                &jellyfin_api::types::LiveTvInfo {
                    is_enabled: Some(true),
                    services: Vec::new(),
                    ..jellyfin_api::types::LiveTvInfo::default()
                }
            ),
            LiveTvAccess::NoService
        );
    }

    #[tokio::test]
    async fn a_socket_going_down_declares_no_media_control_and_no_commands() {
        let server = answering(204).await;
        let upstream = Upstream::stub(&server.base);
        upstream
            .declare_capabilities(false, false)
            .await
            .expect("the declaration is accepted");
        let queries = server.queries("/Sessions/Capabilities");
        let declared = queries.first().expect("one declaration");
        assert!(declared.contains("supportsMediaControl=false"));
        assert!(!declared.contains("supportedCommands="));
    }

    #[tokio::test]
    async fn the_socket_url_carries_the_token_and_this_device() {
        let upstream = Upstream::stub("https://jellyfin.example");
        let url = upstream.socket_url(&Identity::of(jellium_protocol::Identity {
            device: "Firefox".to_owned(),
            device_id: Uuid::nil().to_string(),
        }));
        assert!(url.starts_with("wss://jellyfin.example/socket?"), "{url}");
        assert!(url.contains("api_key=token"));
        assert!(url.contains(&format!("deviceId={}", Uuid::nil())));
    }

    #[tokio::test]
    async fn a_logout_the_server_refuses_counts_as_revoked() {
        let server = answering(401).await;
        let upstream = Upstream::stub(&server.base);
        assert!(upstream.logout().await.is_ok());
        assert_eq!(server.requests.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn a_logout_a_server_does_not_accept_is_unreachable() {
        let server = answering(500).await;
        let upstream = Upstream::stub(&server.base);
        assert!(matches!(
            upstream.logout().await,
            Err(Failure::ServerUnreachable { .. })
        ));
    }

    #[tokio::test]
    async fn a_control_character_token_fails_resume_as_token_rejected() {
        let session = crate::session::Session {
            server: "http://127.0.0.1:1".to_string(),
            token: "a\nb".to_string(),
            user_id: Uuid::nil(),
        };
        assert!(matches!(
            Upstream::resume(
                &Identity::of(jellium_protocol::Identity {
                    device: "Firefox".to_owned(),
                    device_id: Uuid::nil().to_string(),
                }),
                &session,
                &version::Probed {
                    version: jellyfin_api::SNAPSHOT_VERSION.to_string(),
                    name: String::new(),
                    startup: false,
                },
            )
            .await,
            Err(Failure::TokenRejected)
        ));
    }

    const UNSERVED: Uuid = Uuid::from_u128(0x9007);

    fn asking_gzip() -> HeaderMap {
        let mut sent = HeaderMap::new();
        sent.insert(header::ACCEPT_ENCODING, "gzip".parse().expect("a value"));
        sent
    }

    /// The variant playlist the relayed master playlist hands out.
    async fn variant(upstream: &Upstream, seen: &route::Seen) -> String {
        let master = read(
            through(
                upstream,
                reqwest::Method::GET,
                &crate::web::synthetic::Stream::master_playlist(),
                seen,
            )
            .await,
        )
        .await;
        let handed = crate::web::manifest::referenced(&master);
        assert_eq!(handed.len(), 1, "{master}");
        read(through(upstream, reqwest::Method::GET, handed[0], seen).await).await
    }

    #[tokio::test]
    async fn a_variant_playlist_the_master_hands_out_is_relayed_and_rewritten() {
        let server = answering(404).await;
        let upstream = Upstream::stub(&server.base);
        let seen = route::Seen::new();

        let variant = variant(&upstream, &seen).await;
        assert!(variant.contains("#EXT-X-MAP"), "{variant}");
        assert!(!variant.contains("://"), "{variant}");
        assert!(
            crate::web::manifest::referenced(&variant)
                .iter()
                .any(|reference| reference.ends_with("hls1/main/-1.mp4")),
            "{variant}"
        );
    }

    #[tokio::test]
    async fn every_reference_a_relayed_variant_playlist_carries_is_relayed_too() {
        let server = answering(404).await;
        let upstream = Upstream::stub(&server.base);
        let seen = route::Seen::new();

        let variant = variant(&upstream, &seen).await;
        let references = crate::web::manifest::referenced(&variant);
        assert_eq!(references.len(), 2, "{variant}");
        for reference in references {
            let answered = through(&upstream, reqwest::Method::GET, reference, &seen).await;
            assert_eq!(answered.status(), StatusCode::OK, "{reference}");
        }
    }

    #[tokio::test]
    async fn a_subtitle_track_the_plan_names_is_relayed_as_webvtt() {
        let server = answering(404).await;
        let upstream = Upstream::stub(&server.base);
        let seen = route::Seen::new();

        let answered = through(
            &upstream,
            reqwest::Method::GET,
            &crate::web::synthetic::Stream::subtitle_track(),
            &seen,
        )
        .await;
        assert_eq!(answered.status(), StatusCode::OK);
        assert_eq!(
            answered
                .headers()
                .get(header::CONTENT_TYPE)
                .and_then(|value| value.to_str().ok()),
            Some("text/vtt")
        );
        assert!(
            read(answered)
                .await
                .contains(crate::web::synthetic::Stream::CUE)
        );
        assert!(
            server
                .taken
                .credentialed()
                .contains(&crate::web::synthetic::Stream::subtitle_track()),
            "{:?}",
            server.taken.credentialed()
        );
    }

    #[tokio::test]
    async fn a_manifest_route_asks_the_server_for_undecoded_bytes() {
        let server = answering(200).await;
        let upstream = Upstream::stub(&server.base);
        let target = route::Target::admit(
            &axum::http::Method::GET,
            &format!("/Videos/{UNSERVED}/master.m3u8"),
            &route::Seen::new(),
        )
        .expect("an admitted route");
        upstream
            .link()
            .forward(
                &target,
                None,
                &asking_gzip(),
                axum::body::Bytes::new(),
                &route::Seen::new(),
                &crate::web::playback::pointed::Pointed::new(),
            )
            .await
            .expect("a response");
        assert_eq!(
            server.header(0, &header::ACCEPT_ENCODING),
            Some("identity".to_string())
        );
    }

    #[tokio::test]
    async fn an_item_request_forwards_the_browsers_accept_encoding() {
        let server = answering(200).await;
        let upstream = Upstream::stub(&server.base);
        let target = route::Target::admit(
            &axum::http::Method::GET,
            &format!("/Items/{UNSERVED}"),
            &route::Seen::new(),
        )
        .expect("an admitted route");
        upstream
            .link()
            .forward(
                &target,
                None,
                &asking_gzip(),
                axum::body::Bytes::new(),
                &route::Seen::new(),
                &crate::web::playback::pointed::Pointed::new(),
            )
            .await
            .expect("a response");
        let carried = server
            .taken
            .headers(&format!("/Items/{UNSERVED}"))
            .expect("the item request reached the stub");
        assert_eq!(
            carried
                .get(header::ACCEPT_ENCODING)
                .and_then(|value| value.to_str().ok()),
            Some("gzip")
        );
    }

    #[tokio::test]
    async fn a_playlist_answered_on_a_streamed_route_is_refused() {
        let server = answering_with(
            200,
            &[("content-type", "application/vnd.apple.mpegurl")],
            "#EXTM3U\n",
        )
        .await;
        let upstream = Upstream::stub(&server.base);
        let target = route::Target::admit(
            &axum::http::Method::GET,
            &format!("/Videos/{UNSERVED}/hls1/main/0.ts"),
            &route::Seen::new(),
        )
        .expect("an admitted route");
        let response = upstream
            .link()
            .forward(
                &target,
                None,
                &HeaderMap::new(),
                axum::body::Bytes::new(),
                &route::Seen::new(),
                &crate::web::playback::pointed::Pointed::new(),
            )
            .await
            .expect("a response");
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn a_playlist_body_that_arrives_encoded_is_refused() {
        let server = answering_with(
            200,
            &[
                ("content-type", "application/vnd.apple.mpegurl"),
                ("content-encoding", "gzip"),
            ],
            "#EXTM3U\n",
        )
        .await;
        let upstream = Upstream::stub(&server.base);
        let target = route::Target::admit(
            &axum::http::Method::GET,
            &format!("/Videos/{UNSERVED}/master.m3u8"),
            &route::Seen::new(),
        )
        .expect("an admitted route");
        let response = upstream
            .link()
            .forward(
                &target,
                None,
                &HeaderMap::new(),
                axum::body::Bytes::new(),
                &route::Seen::new(),
                &crate::web::playback::pointed::Pointed::new(),
            )
            .await
            .expect("a response");
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn a_playback_path_naming_an_unserved_item_reaches_the_configured_answer() {
        let server = answering_with(
            200,
            &[("content-type", "text/plain")],
            "the configured answer",
        )
        .await;
        let upstream = Upstream::stub(&server.base);
        let seen = route::Seen::new();
        for path in [
            format!("/Videos/{UNSERVED}/master.m3u8"),
            format!("/Videos/{UNSERVED}/main.m3u8"),
            format!("/Videos/{UNSERVED}/hls1/main/0.mp4"),
        ] {
            let answered = through(&upstream, reqwest::Method::GET, &path, &seen).await;
            assert_eq!(read(answered).await, "the configured answer", "{path}");
        }
    }

    /// The login screen's sign-in presents this installation's device identity
    /// with an empty token, which is what Jellyfin keys the session it mints
    /// by.
    #[tokio::test]
    async fn a_login_presents_this_device_with_an_empty_token() {
        let server = answering(200).await;
        Link::tokenless(&server.base)
            .expect("the stub server is a url")
            .control()
            .update_startup_user(&jellyfin_api::types::StartupUserDto {
                name: Some("root".to_string()),
                password: Some(String::new()),
            })
            .await
            .expect("the first administrator is posted");

        Upstream::login(
            &Identity::of(jellium_protocol::Identity {
                device: "Firefox".to_owned(),
                device_id: Uuid::nil().to_string(),
            }),
            &server.base,
            &Credentials {
                username: "root".to_string(),
                password: String::new(),
            },
            &version::Probed {
                version: jellyfin_api::SNAPSHOT_VERSION.to_string(),
                name: String::new(),
                startup: false,
            },
        )
        .await
        .expect("the stub signs the administrator in");

        let presented = server
            .taken
            .authorization("/Users/AuthenticateByName")
            .expect("the sign-in presented an identity");
        assert!(presented.starts_with("MediaBrowser "), "{presented}");
        assert!(
            presented.contains(r#"Client="Jellyfin Web""#),
            "{presented}"
        );
        assert!(
            presented.contains(&format!(r#"DeviceId="{}""#, Uuid::nil())),
            "{presented}"
        );
        assert!(presented.contains(r#"Version="10.11.11""#), "{presented}");
        assert!(!presented.contains("Token="), "{presented}");
    }

    #[tokio::test]
    async fn a_server_text_that_is_not_a_url_is_unreachable() {
        assert!(matches!(
            version::probe("not a url").await,
            Err(Failure::ServerUnreachable { .. })
        ));
    }
}

#[cfg(test)]
mod dashboard_tests {
    use super::*;
    use crate::web::route;

    /// The headers a browser sends with a json write.
    fn writing_json() -> HeaderMap {
        let mut sent = HeaderMap::new();
        sent.insert(
            header::CONTENT_TYPE,
            "application/json".parse().expect("a value"),
        );
        sent
    }

    /// The relay reads a listing through `Payload::Observed`, so the names it
    /// carried are admitted afterwards and no other name is.
    #[tokio::test]
    async fn a_package_listing_admits_exactly_the_names_it_carried() {
        let server = answering(200).await;
        let upstream = Upstream::stub(&server.base);
        let seen = route::Seen::new();

        let target = route::Target::admit(&axum::http::Method::GET, "/Packages", &seen)
            .expect("the package listing is relayed");
        upstream
            .link()
            .forward(
                &target,
                None,
                &HeaderMap::new(),
                axum::body::Bytes::new(),
                &seen,
                &crate::web::playback::pointed::Pointed::new(),
            )
            .await
            .expect("a forwarded listing");

        assert!(seen.holds(route::Observed::Package, "Package 0"));
        assert!(!seen.holds(route::Observed::Package, "Package unlisted"));
        assert!(
            route::Target::admit(
                &axum::http::Method::POST,
                "/Packages/Installed/Package%200",
                &seen
            )
            .is_some()
        );
        assert!(
            route::Target::admit(
                &axum::http::Method::POST,
                "/Packages/Installed/Package%20unlisted",
                &seen
            )
            .is_none()
        );
    }

    /// A configuration page name is admitted only because a listing carried it.
    #[tokio::test]
    async fn a_configuration_page_listing_admits_exactly_the_names_it_carried() {
        let server = answering(200).await;
        let upstream = Upstream::stub(&server.base);
        let seen = route::Seen::new();

        assert!(!seen.holds(
            route::Observed::Page,
            super::super::synthetic::Dashboard::PLUGIN_PAGE
        ));

        let target =
            route::Target::admit(&axum::http::Method::GET, "/web/ConfigurationPages", &seen)
                .expect("the page listing is relayed");
        upstream
            .link()
            .forward(
                &target,
                None,
                &HeaderMap::new(),
                axum::body::Bytes::new(),
                &seen,
                &crate::web::playback::pointed::Pointed::new(),
            )
            .await
            .expect("a forwarded listing");

        assert!(seen.holds(
            route::Observed::Page,
            super::super::synthetic::Dashboard::PLUGIN_PAGE
        ));
        assert!(!seen.holds(route::Observed::Page, "Unlisted"));
    }

    /// The log body is delivered as its tail, so the browser never receives
    /// more than `jellium_model::log::TAIL_LIMIT` of an 8 MiB file.
    #[tokio::test]
    async fn a_log_body_is_delivered_as_its_tail_and_names_the_full_length() {
        let server = answering(200).await;
        let upstream = Upstream::stub(&server.base);
        let seen = route::Seen::new();
        let target = route::Target::admit(&axum::http::Method::GET, "/System/Logs/Log", &seen)
            .expect("the log body is relayed");

        let response = upstream
            .link()
            .forward(
                &target,
                Some("name=jellyfin.log"),
                &HeaderMap::new(),
                axum::body::Bytes::new(),
                &seen,
                &crate::web::playback::pointed::Pointed::new(),
            )
            .await
            .expect("a forwarded log");

        assert_eq!(response.status(), StatusCode::PARTIAL_CONTENT);
        let range = response
            .headers()
            .get(header::CONTENT_RANGE)
            .and_then(|value| value.to_str().ok())
            .expect("a content range")
            .to_owned();
        let full = super::super::synthetic::Dashboard::LOG_BYTES;
        assert!(range.ends_with(&format!("/{full}")), "{range}");

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("the delivered body");
        assert_eq!(body.len() as u64, jellium_model::log::TAIL_LIMIT.count());
    }

    /// A configuration page is fetched for the local server itself, and a name
    /// the Jellyfin server does not hold reads as a refusal rather than as a
    /// document.
    #[tokio::test]
    async fn a_configuration_page_is_fetched_by_name() {
        let server = answering(200).await;
        let upstream = Upstream::stub(&server.base);
        let document = upstream
            .configuration_page(super::super::synthetic::Dashboard::PLUGIN_PAGE)
            .await
            .expect("the synthetic page");
        assert!(document.contains("ApiClient.getPluginConfiguration"));
        assert_eq!(
            upstream.configuration_page("Unlisted").await,
            Err(Refusal::PageNotListed)
        );
    }

    /// Every screen's first contents come from one request rather than from
    /// waiting for a push.
    #[tokio::test]
    async fn a_feed_fills_from_one_request() {
        let server = answering(200).await;
        let upstream = Upstream::stub(&server.base);

        let tasks = upstream.tasks().await.expect("the synthetic tasks");
        assert_eq!(tasks.len(), super::super::synthetic::Dashboard::TASKS);
        assert!(
            tasks
                .iter()
                .any(|task| task.state == jellium_protocol::TaskRunState::Running)
        );

        let entries = upstream
            .activity(200)
            .await
            .expect("the synthetic activity");
        assert_eq!(entries.len(), 200);
        assert!(entries.iter().any(|entry| entry.user.is_some()));
    }

    /// A section is read whole and written whole, so a save carries through
    /// every key no control names.
    #[tokio::test]
    async fn a_section_is_read_and_written_whole() {
        let server = answering(200).await;
        let read = server.dashboard.section("encoding");
        assert_eq!(
            read["UnnamedByAnyControl"]["kept"],
            serde_json::json!([1, 2, 3])
        );

        let mut written = read.clone();
        written["DownMixAudioBoost"] = serde_json::json!(4);

        let upstream = Upstream::stub(&server.base);
        let seen = route::Seen::new();
        let target = route::Target::admit(
            &axum::http::Method::POST,
            "/System/Configuration/encoding",
            &seen,
        )
        .expect("the section write is relayed");
        upstream
            .link()
            .forward(
                &target,
                None,
                &writing_json(),
                axum::body::Bytes::from(written.to_string()),
                &seen,
                &crate::web::playback::pointed::Pointed::new(),
            )
            .await
            .expect("a forwarded write");

        let held = server.dashboard.section("encoding");
        assert_eq!(held["DownMixAudioBoost"], serde_json::json!(4));
        assert_eq!(
            held["UnnamedByAnyControl"]["kept"],
            serde_json::json!([1, 2, 3])
        );
    }

    /// A user policy and a set of library options are held whole, so a save
    /// preserves what no control covers.
    #[tokio::test]
    async fn a_policy_and_library_options_are_held_whole() {
        let server = answering(200).await;
        let user = super::super::synthetic::Dashboard::user(0);
        let policy = server.dashboard.policy(user);
        assert_eq!(policy["IsAdministrator"], serde_json::json!(true));
        assert_eq!(
            policy["UnnamedByAnyControl"]["kept"],
            serde_json::json!(true)
        );

        let options = server.dashboard.options("Movies");
        assert_eq!(
            options["UnnamedByAnyControl"]["kept"],
            serde_json::json!("options")
        );
    }

    /// An administrative write the server refuses carries the server's own
    /// message, which is what renders beneath the project's sentence.
    #[tokio::test]
    async fn a_refused_write_carries_the_servers_own_message() {
        let server = answering(200).await;
        server.dashboard.refuse_next("the library is in use");
        let upstream = Upstream::stub(&server.base);
        let seen = route::Seen::new();
        let target = route::Target::admit(
            &axum::http::Method::POST,
            "/System/Configuration/encoding",
            &seen,
        )
        .expect("the section write is relayed");

        let response = upstream
            .link()
            .forward(
                &target,
                None,
                &writing_json(),
                axum::body::Bytes::from_static(b"{}"),
                &seen,
                &crate::web::playback::pointed::Pointed::new(),
            )
            .await
            .expect("a forwarded write");
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("the refusal body");
        let said = String::from_utf8(body.to_vec()).expect("utf-8");
        assert!(said.contains("the library is in use"), "{said}");
    }
}
