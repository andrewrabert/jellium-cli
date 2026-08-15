//! The login target the local server holds while a server's login screen is
//! open, and the endpoints that stage is served by.

pub mod quickconnect;
pub mod reset;
pub mod servers;

use std::sync::Arc;

use axum::Json;
use axum::body::Body;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use jellium_protocol::{
    Credentials, Failure, LoginScreen, PinOutcome, PublicUser, QuickConnectCode, Refusal,
    ResetAnswer, SessionStatus, Targeted,
};
use uuid::Uuid;

use super::AppState;
use super::identity::Device;
use super::link::{Link, status_of, unreachable};
use super::upstream::{self, Upstream};
use super::version;

/// A server url and no credential: never written to the session file, never
/// declaring capabilities, never opening a socket, and admitting no relayed
/// route at all.
pub struct Login {
    link: Link,
    server: String,
    name: String,
    probed: version::Probed,
    rejected: bool,
    /// The opaque handle every login-stage request presents.
    target: String,
    /// The user ids the public list last read carried, which are the only ones
    /// the image endpoint answers for.
    users: std::sync::RwLock<Vec<Uuid>>,
    /// The Quick Connect secret this request holds; it never leaves the local
    /// server and never reaches the session file.
    secret: std::sync::RwLock<Option<String>>,
    /// False once the Jellyfin server reported Quick Connect off.
    quick_connect: std::sync::atomic::AtomicBool,
}

/// What one Quick Connect poll found.
pub enum Connected {
    Pending,
    /// The request was authorized and its secret already exchanged.
    Authorized(Box<Upstream>),
    Expired,
    Disabled,
}

impl Login {
    /// The width every public user's image is asked for at, so no browser
    /// chooses one.
    pub const IMAGE_WIDTH: u32 = 96;

    /// Holds a link to `server` carrying this installation's device identity
    /// with an empty token; `rejected` is true when the saved credential this
    /// server held was just cleared.
    /// `None` when `server` is not an http url.
    pub fn of(
        device: &Device,
        server: &str,
        probed: &version::Probed,
        name: &str,
        rejected: bool,
    ) -> Option<Login> {
        let link = Link::identified(device, server)?;
        Some(Login {
            server: link.server().to_string(),
            link,
            name: name.to_string(),
            probed: probed.clone(),
            rejected,
            target: Uuid::new_v4().to_string(),
            users: std::sync::RwLock::new(Vec::new()),
            secret: std::sync::RwLock::new(None),
            quick_connect: std::sync::atomic::AtomicBool::new(true),
        })
    }

    /// The link every login-stage request is issued over.
    pub fn link(&self) -> &Link {
        &self.link
    }

    /// The opaque handle this target is presented by.
    pub fn target(&self) -> &str {
        &self.target
    }

    /// True when `presented` is this target's handle.
    pub fn holds(&self, presented: &str) -> bool {
        self.target == presented
    }

    pub fn server(&self) -> &str {
        &self.server
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn probed(&self) -> &version::Probed {
        &self.probed
    }

    fn failed(&self, error: jellyfin_api::error::Error) -> Failure {
        self.link.failed(error, Failure::CredentialsRejected)
    }

    /// The public users and the Quick Connect flag, read in one act, with the
    /// ids recorded as the only ones the image endpoint will answer for.
    pub async fn screen(&self, read_only: bool) -> Result<LoginScreen, Failure> {
        let control = self.link.control();
        let listed = control
            .get_public_users()
            .await
            .map_err(|e| self.failed(e))?;
        let users: Vec<PublicUser> = listed
            .iter()
            .filter_map(|user| {
                Some(PublicUser {
                    id: user.id?,
                    name: user.name.clone().unwrap_or_default(),
                    has_image: user.primary_image_tag.is_some(),
                })
            })
            .collect();
        *self.users.write().expect("the public user list") =
            users.iter().map(|user| user.id).collect();

        let quick_connect = control.get_quick_connect_enabled().await.unwrap_or(false);
        self.quick_connect
            .store(quick_connect, std::sync::atomic::Ordering::SeqCst);

        Ok(LoginScreen {
            target: self.target().to_string(),
            server: self.server.clone(),
            name: self.name().to_string(),
            server_version: self.probed().version.clone(),
            snapshot_version: jellyfin_api::SNAPSHOT_VERSION.to_string(),
            users,
            quick_connect,
            rejected: self.rejected,
            read_only,
        })
    }

    /// The user's primary image at [`Login::IMAGE_WIDTH`], and `None` for an id
    /// absent from the public list last read.
    pub async fn image(&self, user: Uuid) -> Option<Result<Response, Failure>> {
        if !self
            .users
            .read()
            .expect("the public user list")
            .contains(&user)
        {
            return None;
        }
        Some(self.fetched(user).await)
    }

    async fn fetched(&self, user: Uuid) -> Result<Response, Failure> {
        let response = self
            .link
            .streaming()
            .request(reqwest::Method::GET, "/UserImage".into())
            .query("userId", user)
            .query("maxWidth", Login::IMAGE_WIDTH)
            .send_response()
            .await
            .map_err(|e| self.failed(e))?;
        let status = StatusCode::from_u16(response.status().as_u16())
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .unwrap_or("application/octet-stream")
            .to_string();
        let bytes = response
            .bytes()
            .await
            .map_err(|e| unreachable(&self.server, e))?;
        Response::builder()
            .status(status)
            .header(axum::http::header::CONTENT_TYPE, content_type)
            .body(Body::from(bytes))
            .map_err(|e| unreachable(&self.server, e))
    }

    /// Initiates a request, holds its secret, and answers the code alone.
    pub async fn initiate(&self) -> Result<QuickConnectCode, Failure> {
        let result = self
            .link
            .control()
            .initiate_quick_connect()
            .await
            .map_err(|e| self.failed(e))?;
        let code = result
            .code
            .ok_or_else(|| unreachable(&self.server, "the server issued no quick connect code"))?;
        let secret = result.secret.ok_or_else(|| {
            unreachable(&self.server, "the server issued no quick connect secret")
        })?;
        *self.secret.write().expect("the quick connect secret") = Some(secret);
        Ok(QuickConnectCode { code })
    }

    /// One `GET /QuickConnect/Connect` against the held secret; an authorized
    /// request is exchanged for a token in the same call, and the secret is
    /// dropped whether the exchange succeeds or not.
    /// `SignIn::Disabled` takes Quick Connect off this target for good.
    pub async fn connect(&self, device: &Device) -> Result<Connected, Failure> {
        let held = self
            .secret
            .read()
            .expect("the quick connect secret")
            .clone();
        let Some(secret) = held else {
            return Ok(Connected::Expired);
        };

        let answered = self.link.control().get_quick_connect_state(&secret).await;
        let standing = match &answered {
            Ok(result) => {
                jellium_model::quickconnect::signed_in(200, result.authenticated.unwrap_or(false))
            }
            Err(error) => {
                jellium_model::quickconnect::signed_in(status_of(error).unwrap_or(500), false)
            }
        };

        match standing {
            jellium_model::quickconnect::SignIn::Pending => Ok(Connected::Pending),
            jellium_model::quickconnect::SignIn::Expired => {
                self.abandon();
                Ok(Connected::Expired)
            }
            jellium_model::quickconnect::SignIn::Disabled => {
                self.abandon();
                self.quick_connect
                    .store(false, std::sync::atomic::Ordering::SeqCst);
                Ok(Connected::Disabled)
            }
            jellium_model::quickconnect::SignIn::Authorized => {
                self.abandon();
                let upstream =
                    Upstream::quick_connect(device, &self.server, &secret, self.probed()).await?;
                Ok(Connected::Authorized(Box::new(upstream)))
            }
        }
    }

    /// Drops the held secret, which is what leaving the screen does.
    pub fn abandon(&self) {
        *self.secret.write().expect("the quick connect secret") = None;
    }

    pub async fn forgot(&self, username: &str) -> Result<ResetAnswer, Failure> {
        let result = self
            .link
            .control()
            .forgot_password(&jellyfin_api::types::ForgotPasswordDto {
                entered_username: username.to_string(),
            })
            .await
            .map_err(|e| self.failed(e))?;
        Ok(match result.action {
            Some(jellyfin_api::types::ForgotPasswordAction::PinCode) => ResetAnswer::PinWritten {
                pin_file: result.pin_file.unwrap_or_default(),
                expires: result
                    .pin_expiration_date
                    .map(|when| when.timestamp_millis()),
            },
            Some(jellyfin_api::types::ForgotPasswordAction::InNetworkRequired) => {
                ResetAnswer::InNetworkRequired
            }
            Some(jellyfin_api::types::ForgotPasswordAction::ContactAdmin) | None => {
                ResetAnswer::ContactAdministrator
            }
        })
    }

    pub async fn redeem(&self, pin: &str) -> Result<PinOutcome, Failure> {
        let result = self
            .link
            .control()
            .forgot_password_pin(&jellyfin_api::types::ForgotPasswordPinDto {
                pin: pin.to_string(),
            })
            .await
            .map_err(|e| self.failed(e))?;
        Ok(if result.success.unwrap_or(false) {
            PinOutcome::Cleared {
                users: result.users_reset,
            }
        } else {
            PinOutcome::Refused
        })
    }

    /// Signs in with the typed name and password against this target's server.
    pub async fn sign_in(
        &self,
        device: &Device,
        credentials: &Credentials,
    ) -> Result<Upstream, Failure> {
        Upstream::login(device, &self.server, credentials, self.probed()).await
    }
}

pub fn refusal(refusal: Refusal) -> Response {
    let status = match refusal {
        Refusal::NoSession => StatusCode::CONFLICT,
        _ => StatusCode::FORBIDDEN,
    };
    (status, Json(refusal)).into_response()
}

pub(super) fn failed(failure: Failure) -> Response {
    (upstream::status_for(&failure), Json(failure)).into_response()
}

/// Enters `server`'s login screen: probes it, gates its version, walks into the
/// wizard when it reports startup mode, and otherwise installs the login target
/// and answers `SessionStatus::Login`.
/// The name a passing probe reports is written to that server's record.
pub async fn entered(state: &Arc<AppState>, server: &str, rejected: bool) -> Response {
    let probed = match version::probe(server).await {
        Ok(probed) => probed,
        Err(failure) => return failed(failure),
    };
    if probed.startup {
        return super::setup::entered(state, server, &probed, false).await;
    }
    state.session.named(server, &probed.name).await;

    let name = if probed.name.is_empty() {
        state
            .session
            .records()
            .await
            .into_iter()
            .find(|saved| {
                crate::session::normalized(&saved.server) == crate::session::normalized(server)
            })
            .map(|saved| saved.name)
            .unwrap_or_default()
    } else {
        probed.name.clone()
    };

    let Some(login) = Login::of(&state.device, server, &probed, &name, rejected) else {
        return failed(unreachable(server, "the server text is not an http url"));
    };
    let screen = match login.screen(state.read_only).await {
        Ok(screen) => screen,
        Err(failure) => return failed(failure),
    };
    super::control::ended(state).await;
    state.session.enter_login(login).await;
    state.live.rebound(state).await;
    Json(SessionStatus::Login(screen)).into_response()
}

/// The login target a login-stage request is served by, or
/// `Refusal::LoginMoved` when the presented handle is not the one held.
pub async fn admitted(state: &AppState, targeted: &Targeted) -> Result<Arc<Login>, Response> {
    let held = state
        .session
        .login()
        .await
        .ok_or_else(|| refusal(Refusal::LoginMoved))?;
    if !held.holds(&targeted.target) {
        return Err(refusal(Refusal::LoginMoved));
    }
    Ok(held)
}

/// Releases the login target, which is what Back off a login screen does.
pub async fn leave(State(state): State<Arc<AppState>>) -> Response {
    if let Some(login) = state.session.login().await {
        login.abandon();
    }
    state.session.leave_login().await;
    StatusCode::NO_CONTENT.into_response()
}

/// Signs in against the held login target, installs the session, writes the
/// active record and answers `SessionStatus::Authenticated`.
pub async fn sign_in(
    State(state): State<Arc<AppState>>,
    Query(targeted): Query<Targeted>,
    Json(credentials): Json<Credentials>,
) -> Response {
    let login = match admitted(&state, &targeted).await {
        Ok(login) => login,
        Err(response) => return response,
    };
    match login.sign_in(&state.device, &credentials).await {
        Ok(upstream) => {
            let installed = state.session.install(upstream).await;
            state.live.rebound(&state).await;
            Json(SessionStatus::Authenticated(super::control::signed(
                &state,
                &installed.state,
            )))
            .into_response()
        }
        Err(failure) => failed(failure),
    }
}

/// One public user's primary image, answered only for an id the public list
/// last read for this target carried.
pub async fn image(
    State(state): State<Arc<AppState>>,
    Path(user): Path<Uuid>,
    Query(targeted): Query<Targeted>,
) -> Response {
    let login = match admitted(&state, &targeted).await {
        Ok(login) => login,
        Err(response) => return response,
    };
    match login.image(user).await {
        Some(Ok(response)) => response,
        Some(Err(failure)) => failed(failure),
        None => refusal(Refusal::NotRelayed),
    }
}

#[cfg(test)]
pub(super) mod harness {
    use super::*;
    use axum::Router;
    use axum::body::Body;
    use axum::http::Request;
    use axum::routing::{any, get, post};
    use tower::ServiceExt;

    pub fn scratch(name: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir()
            .join("jellium-cli-login-tests")
            .join(name);
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("scratch directory");
        path.join("session.env")
    }

    pub fn routed(state: AppState) -> (Router, Arc<AppState>) {
        let state = Arc::new(state);
        let router = Router::new()
            .route(
                jellium_protocol::SESSION_PATH,
                get(super::super::control::status).delete(super::super::control::logout),
            )
            .route(
                jellium_protocol::SERVERS_PATH,
                get(servers::list)
                    .post(servers::add)
                    .delete(servers::remove),
            )
            .route(jellium_protocol::SERVER_SELECT_PATH, post(servers::select))
            .route(jellium_protocol::SWITCH_PATH, post(servers::switch))
            .route(
                jellium_protocol::LOGIN_PATH,
                post(sign_in).delete(super::leave),
            )
            .route(
                &format!("{}/{{user}}/image", jellium_protocol::LOGIN_IMAGE_PREFIX),
                get(image),
            )
            .route(
                jellium_protocol::QUICK_CONNECT_PATH,
                post(quickconnect::initiate)
                    .get(quickconnect::poll)
                    .delete(quickconnect::abandon),
            )
            .route(jellium_protocol::RESET_PATH, post(reset::forgot))
            .route(jellium_protocol::RESET_PIN_PATH, post(reset::redeem))
            .route(
                &format!("{}/{{*path}}", jellium_protocol::RELAY_PREFIX),
                any(super::super::relay::relay),
            )
            .with_state(state.clone());
        (router, state)
    }

    pub fn json<T: serde::Serialize>(body: &T) -> Vec<u8> {
        serde_json::to_vec(body).expect("a serializable body")
    }

    pub async fn sent(
        router: &Router,
        method: &str,
        uri: &str,
        body: Vec<u8>,
    ) -> (StatusCode, Vec<u8>) {
        let response = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(method)
                    .uri(uri)
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .expect("a request"),
            )
            .await
            .expect("a response");
        let status = response.status();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("the body");
        (status, bytes.to_vec())
    }

    pub fn decoded<T: serde::de::DeserializeOwned>(body: &[u8]) -> T {
        serde_json::from_slice(body).expect("a decodable body")
    }

    /// A stub upstream past its setup wizard, which is what a login screen is
    /// answered for.
    pub async fn ready() -> crate::web::upstream::Answering {
        let server = crate::web::upstream::answering(200).await;
        reqwest::Client::new()
            .post(format!("{}/Startup/User", server.base))
            .json(&serde_json::json!({ "Name": "first", "Password": "" }))
            .send()
            .await
            .expect("the stub takes a first administrator");
        server.startup.finish();
        server
    }

    /// Adds `server` and answers the login screen it opened.
    pub async fn opened(router: &Router, server: &str) -> LoginScreen {
        let (status, body) = sent(
            router,
            "POST",
            jellium_protocol::SERVERS_PATH,
            json(&jellium_protocol::AddServer {
                url: server.to_string(),
            }),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{}", String::from_utf8_lossy(&body));
        match decoded::<SessionStatus>(&body) {
            SessionStatus::Login(screen) => screen,
            other => panic!("adding the server opened its login screen: {other:?}"),
        }
    }

    /// `{path}?target={target}`.
    pub fn targeted(path: &str, target: &str) -> String {
        format!("{path}?{}={target}", jellium_protocol::TARGET_QUERY)
    }
}

#[cfg(test)]
mod tests {
    use super::harness::*;
    use super::*;

    #[tokio::test]
    async fn a_login_stage_request_presenting_a_displaced_target_is_refused_by_name() {
        let server = ready().await;
        let (router, _state) = routed(AppState::stub(scratch("displaced-target")));
        let screen = opened(&router, &server.base).await;

        let (status, body) = sent(
            &router,
            "POST",
            &targeted(jellium_protocol::QUICK_CONNECT_PATH, "not-the-held-target"),
            Vec::new(),
        )
        .await;
        assert_eq!(status, StatusCode::FORBIDDEN);
        assert_eq!(decoded::<Refusal>(&body), Refusal::LoginMoved);

        assert_eq!(
            sent(
                &router,
                "POST",
                &targeted(jellium_protocol::QUICK_CONNECT_PATH, &screen.target),
                Vec::new(),
            )
            .await
            .0,
            StatusCode::OK
        );
    }

    #[tokio::test]
    async fn no_relay_entry_of_either_stage_is_admitted_while_a_login_target_is_held() {
        let server = ready().await;
        let (router, _state) = routed(AppState::stub(scratch("no-relay-in-login")));
        opened(&router, &server.base).await;

        for path in [
            "/jellyfin/Users/Me",
            "/jellyfin/System/Info",
            "/jellyfin/Startup/Configuration",
            "/jellyfin/Localization/Options",
        ] {
            let (status, _) = sent(&router, "GET", path, Vec::new()).await;
            assert_eq!(status, StatusCode::FORBIDDEN, "{path}");
        }
    }

    #[tokio::test]
    async fn a_user_image_for_an_id_outside_the_public_list_is_refused() {
        let server = ready().await;
        let (router, _state) = routed(AppState::stub(scratch("image-outside-list")));
        let screen = opened(&router, &server.base).await;
        let listed = screen.users.first().expect("a public user").id;

        assert_eq!(
            sent(
                &router,
                "GET",
                &targeted(
                    &format!("{}/{listed}/image", jellium_protocol::LOGIN_IMAGE_PREFIX),
                    &screen.target,
                ),
                Vec::new(),
            )
            .await
            .0,
            StatusCode::OK
        );

        let outside = Uuid::from_u128(99);
        assert_eq!(
            sent(
                &router,
                "GET",
                &targeted(
                    &format!("{}/{outside}/image", jellium_protocol::LOGIN_IMAGE_PREFIX),
                    &screen.target,
                ),
                Vec::new(),
            )
            .await
            .0,
            StatusCode::FORBIDDEN
        );
    }

    #[tokio::test]
    async fn a_login_target_opens_no_event_socket_no_upstream_socket_and_no_feed() {
        let server = ready().await;
        let (router, state) = routed(AppState::stub(scratch("login-opens-nothing")));
        opened(&router, &server.base).await;

        assert!(state.session.login().await.is_some());
        assert!(state.session.signed().await.is_none());
        assert_eq!(server.sockets.load(std::sync::atomic::Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn a_sign_in_against_the_held_target_writes_the_active_record() {
        let server = ready().await;
        let (router, state) = routed(AppState::stub(scratch("sign-in-writes")));
        let screen = opened(&router, &server.base).await;

        let (status, body) = sent(
            &router,
            "POST",
            &targeted(jellium_protocol::LOGIN_PATH, &screen.target),
            json(&Credentials {
                username: "first".to_string(),
                password: String::new(),
            }),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{}", String::from_utf8_lossy(&body));
        assert!(matches!(
            decoded::<SessionStatus>(&body),
            SessionStatus::Authenticated(_)
        ));

        let records = state.session.records().await;
        assert_eq!(records.len(), 1);
        assert!(records[0].credential.is_some());
    }
}
