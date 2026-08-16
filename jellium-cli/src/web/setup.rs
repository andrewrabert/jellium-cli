//! The upstream the local server holds while the Jellyfin server is in startup
//! mode, and the endpoints the wizard's steps are served by.

use std::sync::Arc;
use std::sync::atomic::Ordering;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use jellium_protocol::{
    Credentials, Failure, Refusal, SessionStatus, SetupConfiguration, SetupRemoteAccess, SetupUser,
};

use super::AppState;
use super::link::{Link, unreachable};
use super::upstream::{self, Upstream};
use super::version;

/// A server url and no credential: never written to the session file, never
/// declaring capabilities, and never opening a socket.
pub struct Setup {
    link: Link,
    server_version: String,
    resumed: bool,
    /// The first administrator's password this run posted, and `None` when it
    /// posted none.
    posted: std::sync::RwLock<Option<SetupUser>>,
    /// The remote access this run last posted, and `None` when it posted none.
    access: std::sync::RwLock<Option<SetupRemoteAccess>>,
}

impl Setup {
    /// Holds a tokenless link to `server`; `resumed` is true when the wizard
    /// was entered by resuming a saved session.
    /// `None` when `server` is not an http url.
    pub fn of(server: &str, probed: &version::Probed, resumed: bool) -> Option<Setup> {
        Some(Setup {
            link: Link::tokenless(server)?,
            server_version: probed.version.clone(),
            resumed,
            posted: std::sync::RwLock::new(None),
            access: std::sync::RwLock::new(None),
        })
    }

    /// What the browser is told about the server it is configuring.
    pub fn startup(&self) -> jellium_protocol::Startup {
        jellium_protocol::Startup {
            server: self.link.server().to_string(),
            server_version: self.server_version.clone(),
            snapshot_version: jellyfin_api::SNAPSHOT_VERSION.to_string(),
            resumed: self.resumed,
        }
    }

    /// The link every setup-stage request is issued over.
    pub fn link(&self) -> &Link {
        &self.link
    }

    fn failed(&self, error: jellyfin_api::error::Error) -> Failure {
        self.link.failed(error, Failure::CredentialsRejected)
    }

    /// The whole startup configuration the Jellyfin server holds; a field it
    /// reports absent reads as empty.
    pub async fn configuration(&self) -> Result<SetupConfiguration, Failure> {
        let held = self
            .link
            .control()
            .get_startup_configuration()
            .await
            .map_err(|e| self.failed(e))?;
        Ok(SetupConfiguration {
            server_name: held.server_name.unwrap_or_default(),
            ui_culture: held.ui_culture.unwrap_or_default(),
            preferred_metadata_language: held.preferred_metadata_language.unwrap_or_default(),
            metadata_country_code: held.metadata_country_code.unwrap_or_default(),
        })
    }

    /// Writes the whole startup configuration back, so no step erases a field
    /// another step set.
    pub async fn set_configuration(
        &self,
        configuration: &SetupConfiguration,
    ) -> Result<(), Failure> {
        self.link
            .control()
            .update_initial_configuration(&jellyfin_api::types::StartupConfigurationDto {
                server_name: Some(configuration.server_name.clone()),
                ui_culture: Some(configuration.ui_culture.clone()),
                preferred_metadata_language: Some(
                    configuration.preferred_metadata_language.clone(),
                ),
                metadata_country_code: Some(configuration.metadata_country_code.clone()),
            })
            .await
            .map_err(|e| self.failed(e))
    }

    /// The first administrator's name as the Jellyfin server reports it, with
    /// the password this run posted; the password is empty when this run
    /// posted none.
    pub async fn first_user(&self) -> Result<SetupUser, Failure> {
        let held = self
            .link
            .control()
            .get_startup_user()
            .await
            .map_err(|e| self.failed(e))?;
        let posted = self.posted.read().expect("the posted first user").clone();
        Ok(SetupUser {
            name: held.name.unwrap_or_default(),
            password: posted.map(|user| user.password).unwrap_or_default(),
        })
    }

    /// Posts the first administrator and holds the credential the sign-in
    /// after completion presents.
    pub async fn set_first_user(&self, user: &SetupUser) -> Result<(), Failure> {
        self.link
            .control()
            .update_startup_user(&jellyfin_api::types::StartupUserDto {
                name: Some(user.name.clone()),
                password: Some(user.password.clone()),
            })
            .await
            .map_err(|e| self.failed(e))?;
        *self.posted.write().expect("the posted first user") = Some(user.clone());
        Ok(())
    }

    /// What this run last posted, and both fields enabled when it posted none.
    pub fn remote_access(&self) -> SetupRemoteAccess {
        self.access
            .read()
            .expect("the posted remote access")
            .unwrap_or(SetupRemoteAccess {
                enable_remote_access: true,
                enable_automatic_port_mapping: true,
            })
    }

    /// Posts both remote-access fields and holds them.
    pub async fn set_remote_access(&self, access: &SetupRemoteAccess) -> Result<(), Failure> {
        self.link
            .control()
            .set_remote_access(&jellyfin_api::types::StartupRemoteAccessDto {
                enable_remote_access: access.enable_remote_access,
                enable_automatic_port_mapping: access.enable_automatic_port_mapping,
            })
            .await
            .map_err(|e| self.failed(e))?;
        *self.access.write().expect("the posted remote access") = Some(*access);
        Ok(())
    }

    /// Posts `Startup/Complete`; the Jellyfin server is not restarted.
    pub async fn complete(&self) -> Result<(), Failure> {
        self.link
            .control()
            .complete_wizard()
            .await
            .map_err(|e| self.failed(e))
    }

    /// The first administrator's name and password the sign-in after
    /// completion presents, and `None` when this run posted none.
    pub fn posted(&self) -> Option<SetupUser> {
        self.posted.read().expect("the posted first user").clone()
    }

    /// The server this wizard is configuring.
    pub fn server(&self) -> &str {
        self.link.server()
    }
}

fn refusal(refusal: Refusal) -> Response {
    let status = match refusal {
        Refusal::NoSession => StatusCode::CONFLICT,
        _ => StatusCode::FORBIDDEN,
    };
    (status, Json(refusal)).into_response()
}

fn failed(failure: Failure) -> Response {
    (upstream::status_for(&failure), Json(failure)).into_response()
}

/// The setup upstream a `/setup/*` request is served by, or the refusal that
/// forecloses it: `Refusal::SetupReadOnly` while the instance is read-only,
/// `Refusal::SetupFinished` once this run posted `Startup/Complete`, and
/// `Refusal::NoSession` while no setup upstream is held, each before any
/// upstream request is issued.
async fn admitted(state: &AppState) -> Result<Arc<Setup>, Response> {
    if state.read_only {
        return Err(refusal(Refusal::SetupReadOnly));
    }
    if state.completed.load(Ordering::SeqCst) {
        return Err(refusal(Refusal::SetupFinished));
    }
    state
        .session
        .setup()
        .await
        .ok_or_else(|| refusal(Refusal::NoSession))
}

/// Answers the whole startup configuration.
pub async fn configuration(State(state): State<Arc<AppState>>) -> Response {
    let setup = match admitted(&state).await {
        Ok(setup) => setup,
        Err(response) => return response,
    };
    match setup.configuration().await {
        Ok(configuration) => Json(configuration).into_response(),
        Err(failure) => failed(failure),
    }
}

/// Writes the whole startup configuration.
pub async fn set_configuration(
    State(state): State<Arc<AppState>>,
    Json(configuration): Json<SetupConfiguration>,
) -> Response {
    let setup = match admitted(&state).await {
        Ok(setup) => setup,
        Err(response) => return response,
    };
    match setup.set_configuration(&configuration).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(failure) => failed(failure),
    }
}

pub async fn user(State(state): State<Arc<AppState>>) -> Response {
    let setup = match admitted(&state).await {
        Ok(setup) => setup,
        Err(response) => return response,
    };
    match setup.first_user().await {
        Ok(user) => Json(user).into_response(),
        Err(failure) => failed(failure),
    }
}

/// Creates the first administrator, and renames the one already created when
/// the name differs, so the server holds exactly one user either way.
pub async fn set_user(State(state): State<Arc<AppState>>, Json(user): Json<SetupUser>) -> Response {
    let setup = match admitted(&state).await {
        Ok(setup) => setup,
        Err(response) => return response,
    };
    match setup.set_first_user(&user).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(failure) => failed(failure),
    }
}

pub async fn remote_access(State(state): State<Arc<AppState>>) -> Response {
    let setup = match admitted(&state).await {
        Ok(setup) => setup,
        Err(response) => return response,
    };
    Json(setup.remote_access()).into_response()
}

pub async fn set_remote_access(
    State(state): State<Arc<AppState>>,
    Json(access): Json<SetupRemoteAccess>,
) -> Response {
    let setup = match admitted(&state).await {
        Ok(setup) => setup,
        Err(response) => return response,
    };
    match setup.set_remote_access(&access).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(failure) => failed(failure),
    }
}

/// Posts `Startup/Complete`, then authenticates as the first administrator
/// with the name and password it posted, installs the session, writes the
/// session file, and answers `SessionStatus::Authenticated`.
/// A sign-in that fails releases the setup upstream and answers
/// `Failure::SetupSignInFailed`.
/// An empty first-administrator password succeeds through this.
pub async fn complete(State(state): State<Arc<AppState>>) -> Response {
    let setup = match admitted(&state).await {
        Ok(setup) => setup,
        Err(response) => return response,
    };
    if let Err(failure) = setup.complete().await {
        return failed(failure);
    }
    state.completed.store(true, Ordering::SeqCst);

    let signed_in = async {
        let posted = setup.posted().ok_or(Failure::SetupSignInFailed)?;
        let server = setup.server().to_string();
        let probed = version::probe(&server)
            .await
            .map_err(|_| Failure::SetupSignInFailed)?;
        Upstream::login(
            &state.device,
            &server,
            &Credentials {
                username: posted.name,
                password: posted.password,
            },
            &probed,
        )
        .await
        .map_err(|_| Failure::SetupSignInFailed)
    }
    .await;

    match signed_in {
        Ok(upstream) => {
            let installed = state.session.install(upstream).await;
            state.live.rebound(&state).await;
            Json(SessionStatus::Authenticated(super::control::signed(
                &state,
                &installed.state,
            )))
            .into_response()
        }
        Err(failure) => {
            state.session.leave_setup().await;
            failed(failure)
        }
    }
}

/// Releases the setup upstream, which is what Back on the first step does.
pub async fn leave(State(state): State<Arc<AppState>>) -> Response {
    if state.read_only {
        return refusal(Refusal::SetupReadOnly);
    }
    state.session.leave_setup().await;
    StatusCode::NO_CONTENT.into_response()
}

/// Enters the wizard for `server`, which is what a login submit and a resume
/// against a server in startup mode both do.
/// Under `--read-only` no wizard is offered and the answer is
/// `Refusal::SetupReadOnly`.
pub async fn entered(
    state: &Arc<AppState>,
    server: &str,
    probed: &version::Probed,
    resumed: bool,
) -> Response {
    if state.read_only {
        return refusal(Refusal::SetupReadOnly);
    }
    let Some(setup) = Setup::of(server, probed, resumed) else {
        return failed(unreachable(server, "the server text is not an http url"));
    };
    let held = state.session.enter_setup(setup).await;
    state.live.rebound(state).await;
    Json(SessionStatus::Setup(held.startup())).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::web::route;
    use crate::web::upstream::{Answering, answering};
    use axum::Router;
    use axum::body::Body;
    use axum::http::Request;
    use axum::routing::{any, delete, get, post};
    use tower::ServiceExt;

    fn scratch(name: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir()
            .join("jellium-cli-setup-tests")
            .join(name);
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("scratch directory");
        path.join("session.env")
    }

    fn routed(state: AppState) -> (Router, Arc<AppState>) {
        let state = Arc::new(state);
        let router = Router::new()
            .route(
                jellium_protocol::SESSION_PATH,
                get(super::super::control::status),
            )
            .route(
                jellium_protocol::SERVERS_PATH,
                post(super::super::login::servers::add),
            )
            .route(jellium_protocol::SETUP_PATH, delete(leave))
            .route(
                jellium_protocol::SETUP_CONFIGURATION_PATH,
                get(configuration).post(set_configuration),
            )
            .route(jellium_protocol::SETUP_USER_PATH, get(user).post(set_user))
            .route(
                jellium_protocol::SETUP_REMOTE_ACCESS_PATH,
                get(remote_access).post(set_remote_access),
            )
            .route(jellium_protocol::SETUP_COMPLETE_PATH, post(complete))
            .route(
                &format!("{}/{{*path}}", jellium_protocol::RELAY_PREFIX),
                any(super::super::relay::relay),
            )
            .with_state(state.clone());
        (router, state)
    }

    /// A local server whose stub upstream has not been set up, with the wizard
    /// already entered by a login submit.
    async fn entered_wizard(name: &str) -> (Router, Arc<AppState>, Answering) {
        let server = answering(200).await;
        let (router, state) = routed(AppState::stub(scratch(name)));
        let status = signed_in(&router, &server.base).await;
        assert!(
            matches!(status, SessionStatus::Setup(_)),
            "adding the server opened the wizard: {status:?}"
        );
        (router, state, server)
    }

    /// Adds `server`, which is what opens either the wizard or its login
    /// screen.
    async fn signed_in(router: &Router, server: &str) -> SessionStatus {
        let added = jellium_protocol::AddServer {
            url: server.to_string(),
        };
        let response = router
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(jellium_protocol::SERVERS_PATH)
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_vec(&added).expect("the server text"),
                    ))
                    .expect("a request"),
            )
            .await
            .expect("a response");
        read(response).await
    }

    async fn read<T: serde::de::DeserializeOwned>(response: axum::response::Response) -> T {
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("the body");
        serde_json::from_slice(&body).expect("a decodable body")
    }

    async fn sent(
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
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("the body");
        (status, body.to_vec())
    }

    fn json(value: &impl serde::Serialize) -> Vec<u8> {
        serde_json::to_vec(value).expect("a serializable body")
    }

    /// A server that has never been configured is taken from first contact to a
    /// signed-in administrator without a restart and without a second sign-in;
    /// every request the local server issued over the setup upstream carried no
    /// `Authorization` header, and the sign-in that followed presented this
    /// installation's device identity with an empty token.
    #[tokio::test]
    async fn the_wizard_runs_from_first_contact_to_a_signed_in_administrator() {
        let (router, _state, server) = entered_wizard("whole-wizard").await;

        let (status, body) = sent(
            &router,
            "GET",
            jellium_protocol::SETUP_CONFIGURATION_PATH,
            Vec::new(),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        let held: SetupConfiguration = serde_json::from_slice(&body).expect("the configuration");
        assert_eq!(held, SetupConfiguration::default());

        let language = SetupConfiguration {
            server_name: "kept".to_string(),
            ui_culture: "de".to_string(),
            ..SetupConfiguration::default()
        };
        assert_eq!(
            sent(
                &router,
                "POST",
                jellium_protocol::SETUP_CONFIGURATION_PATH,
                json(&language)
            )
            .await
            .0,
            StatusCode::NO_CONTENT
        );

        let user = SetupUser {
            name: "root".to_string(),
            password: String::new(),
        };
        assert_eq!(
            sent(
                &router,
                "POST",
                jellium_protocol::SETUP_USER_PATH,
                json(&user)
            )
            .await
            .0,
            StatusCode::NO_CONTENT
        );

        // the metadata step writes the whole configuration back, so the server
        // name and the culture the language step set survive it
        let metadata = SetupConfiguration {
            preferred_metadata_language: "en".to_string(),
            metadata_country_code: "US".to_string(),
            ..language.clone()
        };
        assert_eq!(
            sent(
                &router,
                "POST",
                jellium_protocol::SETUP_CONFIGURATION_PATH,
                json(&metadata)
            )
            .await
            .0,
            StatusCode::NO_CONTENT
        );
        assert_eq!(
            server.startup.configuration().server_name.as_deref(),
            Some("kept")
        );
        assert_eq!(
            server.startup.configuration().ui_culture.as_deref(),
            Some("de")
        );

        let access = SetupRemoteAccess {
            enable_remote_access: false,
            enable_automatic_port_mapping: true,
        };
        assert_eq!(
            sent(
                &router,
                "POST",
                jellium_protocol::SETUP_REMOTE_ACCESS_PATH,
                json(&access)
            )
            .await
            .0,
            StatusCode::NO_CONTENT
        );
        assert_eq!(
            server
                .startup
                .remote_access()
                .map(|held| held.enable_remote_access),
            Some(false)
        );

        let (status, body) = sent(
            &router,
            "POST",
            jellium_protocol::SETUP_COMPLETE_PATH,
            Vec::new(),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{}", String::from_utf8_lossy(&body));
        let signed: SessionStatus = serde_json::from_slice(&body).expect("the session");
        match signed {
            SessionStatus::Authenticated(session) => assert_eq!(session.user_name, "root"),
            other => panic!("the wizard signed in: {other:?}"),
        }
        assert!(!server.startup.in_startup());
        assert_eq!(server.startup.authenticated().as_deref(), Some("root"));

        // every request issued before the sign-in carried no credential
        let credentialed = server.taken.credentialed();
        assert!(
            !credentialed
                .iter()
                .any(|path| path.starts_with("/Startup/")),
            "{credentialed:?}"
        );
        let presented = server
            .taken
            .authorization("/Users/AuthenticateByName")
            .expect("the sign-in presented an identity");
        assert!(presented.starts_with("MediaBrowser "), "{presented}");
        assert!(presented.contains(r#"Client="Jellium Web""#), "{presented}");
        assert!(
            presented.contains(&format!(r#"DeviceId="{}""#, uuid::Uuid::nil())),
            "{presented}"
        );
        assert!(presented.contains(r#"Token="""#), "{presented}");
    }

    /// A relay entry admissible during setup reaches the Jellyfin server over
    /// the setup upstream carrying no `Authorization` header.
    #[tokio::test]
    async fn a_setup_stage_relay_read_carries_no_authorization() {
        let (router, _state, server) = entered_wizard("setup-relay-read").await;
        let (status, _) = sent(&router, "GET", "/jellyfin/Localization/Options", Vec::new()).await;
        assert_eq!(status, StatusCode::OK);
        assert!(
            server
                .taken
                .tokenless()
                .contains(&"/Localization/Options".to_string())
        );
        assert!(
            server.taken.credentialed().is_empty(),
            "{:?}",
            server.taken.credentialed()
        );
    }

    /// The browser never receives the field by which a server reports its
    /// startup mode.
    #[tokio::test]
    async fn the_wizard_document_carries_no_startup_wizard_field() {
        let (_router, _state, server) = entered_wizard("no-startup-field").await;
        let startup = jellium_protocol::Startup {
            server: server.base.clone(),
            server_version: super::super::synthetic::Startup::VERSION.to_string(),
            snapshot_version: jellyfin_api::SNAPSHOT_VERSION.to_string(),
            resumed: false,
        };
        let document = serde_json::to_string(&SessionStatus::Setup(startup)).expect("the document");
        assert!(!document.contains("startupWizardCompleted"), "{document}");
        assert!(!document.contains("StartupWizardCompleted"), "{document}");
    }

    /// Exactly fourteen entries of the relay table are reachable while the
    /// setup upstream is held, and the five `/Startup/*` paths are unreachable
    /// through the relay.
    #[tokio::test]
    async fn exactly_fourteen_relay_entries_are_reachable_during_setup() {
        let admitted: Vec<(&str, &str)> = vec![
            ("GET", "/Localization/Options"),
            ("GET", "/Localization/Cultures"),
            ("GET", "/Localization/Countries"),
            ("GET", "/Environment/DefaultDirectoryBrowser"),
            ("GET", "/Environment/ParentPath"),
            ("GET", "/Environment/Drives"),
            ("GET", "/Environment/DirectoryContents"),
            ("GET", "/Library/VirtualFolders"),
            ("POST", "/Library/VirtualFolders"),
            ("DELETE", "/Library/VirtualFolders"),
            ("POST", "/Library/VirtualFolders/Name"),
            ("POST", "/Library/VirtualFolders/Paths"),
            ("POST", "/Library/VirtualFolders/Paths/Update"),
            ("DELETE", "/Library/VirtualFolders/Paths"),
        ];
        assert_eq!(admitted.len(), 14);

        let seen = route::Seen::new();
        let reachable = |method: &str, path: &str| {
            let method = axum::http::Method::from_bytes(method.as_bytes()).expect("a method");
            route::Target::admit(&method, path, &seen)
                .is_some_and(|target| target.stage().admits(jellium_protocol::Admits::Setup))
        };
        for (method, path) in &admitted {
            assert!(reachable(method, path), "{method} {path}");
        }
        for path in [
            "/Startup/Complete",
            "/Startup/Configuration",
            "/Startup/User",
            "/Startup/FirstUser",
            "/Startup/RemoteAccess",
        ] {
            for method in ["GET", "POST"] {
                let verb = axum::http::Method::from_bytes(method.as_bytes()).expect("a method");
                assert!(
                    route::Target::admit(&verb, path, &seen).is_none(),
                    "{method} {path} is relayed"
                );
            }
        }
    }

    /// A relayed request for an entry declaring the other stage is refused by
    /// name, in both directions, and reaches the Jellyfin server not at all.
    #[tokio::test]
    async fn a_route_asked_for_outside_its_stage_is_refused_by_name() {
        let (router, _state, server) = entered_wizard("wrong-stage-setup").await;
        let took = server.taken.tokenless().len() + server.taken.credentialed().len();
        let (status, body) = sent(&router, "GET", "/jellyfin/Items", Vec::new()).await;
        assert_eq!(status, StatusCode::FORBIDDEN);
        let refused: Refusal = serde_json::from_slice(&body).expect("a refusal");
        assert_eq!(
            refused,
            Refusal::NotInStage {
                admits: jellium_protocol::Admits::Signed
            }
        );
        assert_eq!(
            server.taken.tokenless().len() + server.taken.credentialed().len(),
            took,
            "the refused request reached the Jellyfin server"
        );

        let signed = answering(200).await;
        let (signed_router, state) = routed(AppState::stub(scratch("wrong-stage-signed")));
        state
            .session
            .install(super::super::upstream::Upstream::stub(&signed.base))
            .await;
        let (status, body) = sent(
            &signed_router,
            "GET",
            "/jellyfin/Localization/Options",
            Vec::new(),
        )
        .await;
        assert_eq!(status, StatusCode::FORBIDDEN);
        let refused: Refusal = serde_json::from_slice(&body).expect("a refusal");
        assert_eq!(
            refused,
            Refusal::NotInStage {
                admits: jellium_protocol::Admits::Setup
            }
        );
    }

    /// A setup request arriving after `Startup/Complete` is refused from the
    /// local server's own record of having posted it, and issues no upstream
    /// request.
    #[tokio::test]
    async fn a_setup_request_after_completion_is_refused_without_reaching_the_server() {
        let (router, _state, server) = entered_wizard("after-completion").await;
        sent(
            &router,
            "POST",
            jellium_protocol::SETUP_USER_PATH,
            json(&SetupUser {
                name: "root".to_string(),
                password: String::new(),
            }),
        )
        .await;
        assert_eq!(
            sent(
                &router,
                "POST",
                jellium_protocol::SETUP_COMPLETE_PATH,
                Vec::new()
            )
            .await
            .0,
            StatusCode::OK
        );

        let took = server.taken.tokenless().len() + server.taken.credentialed().len();
        let (status, body) = sent(
            &router,
            "GET",
            jellium_protocol::SETUP_CONFIGURATION_PATH,
            Vec::new(),
        )
        .await;
        assert_eq!(status, StatusCode::FORBIDDEN);
        let refused: Refusal = serde_json::from_slice(&body).expect("a refusal");
        assert_eq!(refused, Refusal::SetupFinished);

        let (status, body) =
            sent(&router, "GET", "/jellyfin/Localization/Options", Vec::new()).await;
        assert_eq!(status, StatusCode::FORBIDDEN);
        let refused: Refusal = serde_json::from_slice(&body).expect("a refusal");
        assert_eq!(refused, Refusal::SetupFinished);

        assert_eq!(
            server.taken.tokenless().len() + server.taken.credentialed().len(),
            took
        );
    }

    /// Completing the wizard leaves `jellium-cli` reading the same session the
    /// browser now holds.
    #[tokio::test]
    async fn completing_the_wizard_writes_the_session_file() {
        let (router, state, _server) = entered_wizard("completion-writes-file").await;
        sent(
            &router,
            "POST",
            jellium_protocol::SETUP_USER_PATH,
            json(&SetupUser {
                name: "root".to_string(),
                password: "secret".to_string(),
            }),
        )
        .await;
        sent(
            &router,
            "POST",
            jellium_protocol::SETUP_COMPLETE_PATH,
            Vec::new(),
        )
        .await;
        let saved = state.session.saved().await.expect("a saved session");
        assert_eq!(saved.token, super::super::synthetic::Startup::TOKEN);
        assert!(state.session.signed().await.is_some());
        assert!(state.session.setup().await.is_none());
    }

    /// A sign-in that fails after `Startup/Complete` lands on the login screen
    /// with `Failure::SetupSignInFailed`, and the wizard is not reachable
    /// again.
    #[tokio::test]
    async fn a_sign_in_that_fails_after_completion_releases_the_wizard() {
        // no first administrator was posted, so there is no credential to
        // present after completion
        let (router, state, _server) = entered_wizard("sign-in-fails").await;
        let (status, body) = sent(
            &router,
            "POST",
            jellium_protocol::SETUP_COMPLETE_PATH,
            Vec::new(),
        )
        .await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        let failure: Failure = serde_json::from_slice(&body).expect("a failure");
        assert_eq!(failure, Failure::SetupSignInFailed);
        assert!(state.session.setup().await.is_none());
        assert!(state.session.signed().await.is_none());

        let (status, body) = sent(
            &router,
            "GET",
            jellium_protocol::SETUP_CONFIGURATION_PATH,
            Vec::new(),
        )
        .await;
        assert_eq!(status, StatusCode::FORBIDDEN);
        let refused: Refusal = serde_json::from_slice(&body).expect("a refusal");
        assert_eq!(refused, Refusal::SetupFinished);
    }

    /// Under `--read-only`, a server in startup mode produces `SetupReadOnly`
    /// and no wizard.
    #[tokio::test]
    async fn a_read_only_instance_offers_no_wizard() {
        let server = answering(200).await;
        let mut state = AppState::stub(scratch("read-only-wizard"));
        state.read_only = true;
        let (router, state) = routed(state);

        let (status, body) = sent(
            &router,
            "POST",
            jellium_protocol::SERVERS_PATH,
            json(&jellium_protocol::AddServer {
                url: server.base.clone(),
            }),
        )
        .await;
        assert_eq!(status, StatusCode::FORBIDDEN);
        let refused: Refusal = serde_json::from_slice(&body).expect("a refusal");
        assert_eq!(refused, Refusal::SetupReadOnly);
        assert!(state.session.setup().await.is_none());
    }

    /// Back on the first step releases the setup upstream and leaves the login
    /// screen reachable with a second server url.
    #[tokio::test]
    async fn back_on_the_first_step_releases_the_setup_upstream() {
        let (router, state, _server) = entered_wizard("back-leaves").await;
        assert_eq!(
            sent(&router, "DELETE", jellium_protocol::SETUP_PATH, Vec::new())
                .await
                .0,
            StatusCode::NO_CONTENT
        );
        assert!(state.session.setup().await.is_none());

        let (status, body) = sent(&router, "GET", jellium_protocol::SESSION_PATH, Vec::new()).await;
        assert_eq!(status, StatusCode::OK);
        let status: SessionStatus = serde_json::from_slice(&body).expect("the session document");
        assert!(
            matches!(status, SessionStatus::Anonymous { .. }),
            "the wizard left for the server list: {status:?}"
        );
    }

    /// A step's write that Jellyfin refuses carries the server's own message.
    #[tokio::test]
    async fn a_refused_step_carries_the_servers_own_message() {
        let (router, _state, server) = entered_wizard("refused-step").await;
        server.startup.refuse_next("the culture is not installed");
        let (status, body) = sent(
            &router,
            "POST",
            jellium_protocol::SETUP_CONFIGURATION_PATH,
            json(&SetupConfiguration::default()),
        )
        .await;
        assert_eq!(status, StatusCode::BAD_GATEWAY);
        let said = String::from_utf8(body).expect("utf-8");
        assert!(said.contains("the culture is not installed"), "{said}");
    }

    /// Revisiting the first-administrator step with a different name leaves the
    /// Jellyfin server holding exactly one user.
    #[tokio::test]
    async fn revisiting_the_first_administrator_step_renames_the_one_user() {
        let (router, _state, server) = entered_wizard("rename-first-user").await;
        for name in ["root", "admin"] {
            sent(
                &router,
                "POST",
                jellium_protocol::SETUP_USER_PATH,
                json(&SetupUser {
                    name: name.to_string(),
                    password: "secret".to_string(),
                }),
            )
            .await;
        }
        let held = server.startup.first_user().expect("one first user");
        assert_eq!(held.name.as_deref(), Some("admin"));

        let (_, body) = sent(
            &router,
            "GET",
            jellium_protocol::SETUP_USER_PATH,
            Vec::new(),
        )
        .await;
        let read: SetupUser = serde_json::from_slice(&body).expect("the first user");
        assert_eq!(read.name, "admin");
        assert_eq!(read.password, "secret");
    }

    /// Reloading the browser on any step resumes with every value the earlier
    /// steps stored, because the values live on the Jellyfin server.
    #[tokio::test]
    async fn a_reload_mid_wizard_reads_back_every_stored_value() {
        let (router, _state, _server) = entered_wizard("reload-mid-wizard").await;
        let written = SetupConfiguration {
            server_name: "attic".to_string(),
            ui_culture: "fr".to_string(),
            preferred_metadata_language: "fr".to_string(),
            metadata_country_code: "FR".to_string(),
        };
        sent(
            &router,
            "POST",
            jellium_protocol::SETUP_CONFIGURATION_PATH,
            json(&written),
        )
        .await;
        sent(
            &router,
            "POST",
            jellium_protocol::SETUP_REMOTE_ACCESS_PATH,
            json(&SetupRemoteAccess {
                enable_remote_access: false,
                enable_automatic_port_mapping: false,
            }),
        )
        .await;

        let (status, body) = sent(&router, "GET", jellium_protocol::SESSION_PATH, Vec::new()).await;
        assert_eq!(status, StatusCode::OK);
        let held: SessionStatus = serde_json::from_slice(&body).expect("the session document");
        assert!(matches!(held, SessionStatus::Setup(_)), "{held:?}");

        let (_, body) = sent(
            &router,
            "GET",
            jellium_protocol::SETUP_CONFIGURATION_PATH,
            Vec::new(),
        )
        .await;
        let read: SetupConfiguration = serde_json::from_slice(&body).expect("the configuration");
        assert_eq!(read, written);

        let (_, body) = sent(
            &router,
            "GET",
            jellium_protocol::SETUP_REMOTE_ACCESS_PATH,
            Vec::new(),
        )
        .await;
        let read: SetupRemoteAccess = serde_json::from_slice(&body).expect("the remote access");
        assert!(!read.enable_remote_access);
        assert!(!read.enable_automatic_port_mapping);
    }

    /// A saved session whose server reports startup mode enters the wizard with
    /// `Startup::resumed` set, and the saved record is left on file.
    #[tokio::test]
    async fn a_saved_session_on_a_server_in_startup_mode_resumes_into_the_wizard() {
        let server = answering(200).await;
        let path = scratch("resume-into-wizard");
        let record = crate::session::Session {
            server: server.base.clone(),
            token: "stale".to_string(),
            user_id: uuid::Uuid::nil(),
        };
        crate::session::SessionFile::update_async(path.clone(), {
            let record = record.clone();
            move |file| file.set_server(&record)
        })
        .await
        .expect("seed record");

        let (router, state) = routed(AppState::stub(path));
        let (status, body) = sent(&router, "GET", jellium_protocol::SESSION_PATH, Vec::new()).await;
        assert_eq!(status, StatusCode::OK);
        let held: SessionStatus = serde_json::from_slice(&body).expect("the session document");
        match held {
            SessionStatus::Setup(startup) => assert!(startup.resumed),
            other => panic!("the resume opened the wizard: {other:?}"),
        }
        assert_eq!(state.session.saved().await, Some(record));
    }
}
