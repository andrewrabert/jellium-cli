//! The stub upstream's startup area: the public info probe, the seven
//! `/Startup/*` operations, the localization reads, the two environment reads
//! the wizard adds, and the sign-in that follows completion.

use std::sync::{Arc, Mutex};

use axum::Json;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};

/// A Jellyfin server that has not completed its setup wizard.
#[derive(Clone)]
pub struct Startup {
    held: Arc<Mutex<Held>>,
}

struct Held {
    in_startup: bool,
    /// The version `/System/Info/Public` reports.
    version: String,
    configuration: jellyfin_api::types::StartupConfigurationDto,
    first_user: Option<jellyfin_api::types::StartupUserDto>,
    remote_access: Option<jellyfin_api::types::StartupRemoteAccessDto>,
    authenticated: Option<String>,
    refusing: Option<String>,
}

impl Startup {
    /// The version `/System/Info/Public` reports, which is the snapshot's, so
    /// no off-snapshot warning is drawn unless a test asks for one.
    pub const VERSION: &'static str = jellyfin_api::SNAPSHOT_VERSION;

    /// The token `/Users/AuthenticateByName` answers with.
    pub const TOKEN: &'static str = "synthetic-startup-token";

    fn new() -> Startup {
        Startup {
            held: Arc::new(Mutex::new(Held {
                in_startup: true,
                version: Startup::VERSION.to_string(),
                configuration: jellyfin_api::types::StartupConfigurationDto::default(),
                first_user: None,
                remote_access: None,
                authenticated: None,
                refusing: None,
            })),
        }
    }

    fn held(&self) -> std::sync::MutexGuard<'_, Held> {
        self.held.lock().expect("the synthetic startup area")
    }

    /// True while `/System/Info/Public` reports `startupWizardCompleted`
    /// false.
    pub fn in_startup(&self) -> bool {
        self.held().in_startup
    }

    /// Reports a version under the floor Jellium Web gates on.
    pub fn below_minimum(&self) {
        self.held().version = "10.9.9".to_string();
    }

    /// Ends startup mode, which is what `POST /Startup/Complete` does.
    pub fn finish(&self) {
        self.held().in_startup = false;
    }

    /// The startup configuration held now.
    pub fn configuration(&self) -> jellyfin_api::types::StartupConfigurationDto {
        self.held().configuration.clone()
    }

    /// The first user held now, and `None` when none was posted.
    pub fn first_user(&self) -> Option<jellyfin_api::types::StartupUserDto> {
        self.held().first_user.clone()
    }

    /// The remote access last posted, and `None` when none was.
    pub fn remote_access(&self) -> Option<jellyfin_api::types::StartupRemoteAccessDto> {
        self.held().remote_access.clone()
    }

    /// The name the sign-in after completion presented, and `None` when no
    /// sign-in arrived.
    pub fn authenticated(&self) -> Option<String> {
        self.held().authenticated.clone()
    }

    /// Makes the next `/Startup/*` write refuse with `message`, which is what
    /// a refused step renders beneath its sentence.
    pub fn refuse_next(&self, message: &str) {
        self.held().refusing = Some(message.to_owned());
    }

    /// The refusal the next write takes, if one was asked for.
    fn refusing(&self) -> Option<Response> {
        let message = self.held().refusing.take()?;
        Some(
            (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "Message": message })),
            )
                .into_response(),
        )
    }
}

async fn public_info(State(state): State<Startup>) -> Json<serde_json::Value> {
    let held = state.held();
    Json(serde_json::json!({
        "Version": held.version,
        "StartupWizardCompleted": !held.in_startup,
        "ServerName": "synthetic",
    }))
}

async fn configuration(
    State(state): State<Startup>,
) -> Json<jellyfin_api::types::StartupConfigurationDto> {
    Json(state.configuration())
}

async fn set_configuration(
    State(state): State<Startup>,
    Json(body): Json<jellyfin_api::types::StartupConfigurationDto>,
) -> Response {
    if let Some(refused) = state.refusing() {
        return refused;
    }
    state.held().configuration = body;
    StatusCode::NO_CONTENT.into_response()
}

async fn first_user(State(state): State<Startup>) -> Json<jellyfin_api::types::StartupUserDto> {
    Json(state.first_user().unwrap_or_default())
}

/// Jellyfin's own `POST /Startup/User` renames the administrator it already
/// created rather than making a second one, and so does this.
async fn set_first_user(
    State(state): State<Startup>,
    Json(body): Json<jellyfin_api::types::StartupUserDto>,
) -> Response {
    if let Some(refused) = state.refusing() {
        return refused;
    }
    state.held().first_user = Some(body);
    StatusCode::NO_CONTENT.into_response()
}

async fn set_remote_access(
    State(state): State<Startup>,
    Json(body): Json<jellyfin_api::types::StartupRemoteAccessDto>,
) -> Response {
    if let Some(refused) = state.refusing() {
        return refused;
    }
    state.held().remote_access = Some(body);
    StatusCode::NO_CONTENT.into_response()
}

async fn complete(State(state): State<Startup>) -> Response {
    if let Some(refused) = state.refusing() {
        return refused;
    }
    state.finish();
    StatusCode::NO_CONTENT.into_response()
}

/// Jellyfin mints the session's device identity from the `MediaBrowser`
/// authorization header, so a sign-in carrying none is refused here too.
async fn authenticate(
    State(state): State<Startup>,
    headers: axum::http::HeaderMap,
    Json(body): Json<jellyfin_api::types::AuthenticateUserByName>,
) -> Response {
    if !identified(&headers) {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    let asked = body.username.unwrap_or_default();
    let held = state.first_user();
    let matches = held
        .as_ref()
        .is_some_and(|user| user.name.as_deref() == Some(asked.as_str()));
    if !matches {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    state.held().authenticated = Some(asked.clone());
    Json(serde_json::json!({
        "AccessToken": Startup::TOKEN,
        "User": {
            "Id": uuid::Uuid::from_u128(1),
            "Name": asked,
            "Policy": {
                "AuthenticationProviderId": "",
                "PasswordResetProviderId": "",
                "EnableCollectionManagement": false,
                "EnableLyricManagement": false,
                "EnableSubtitleManagement": false,
                "IsAdministrator": true,
                "EnableLiveTvAccess": true,
            },
        },
    }))
    .into_response()
}

/// True when `headers` carry a `MediaBrowser` authorization naming a non-empty
/// `DeviceId`.
pub(super) fn identified(headers: &axum::http::HeaderMap) -> bool {
    let Some(value) = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
    else {
        return false;
    };
    value.starts_with("MediaBrowser ")
        && value.contains("DeviceId=\"")
        && !value.contains("DeviceId=\"\"")
}

#[derive(serde::Deserialize)]
struct PathQuery {
    path: Option<String>,
}

/// The router serving `/System/Info/Public`, the seven `/Startup/*`
/// operations, `/Users/AuthenticateByName`, the three localization reads and
/// the two environment reads the wizard adds.
pub fn router() -> (axum::Router, Startup) {
    let state = Startup::new();
    let router = axum::Router::new()
        .route("/System/Info/Public", get(public_info))
        .route(
            "/Startup/Configuration",
            get(configuration).post(set_configuration),
        )
        .route("/Startup/User", get(first_user).post(set_first_user))
        .route("/Startup/FirstUser", get(first_user))
        .route("/Startup/RemoteAccess", post(set_remote_access))
        .route("/Startup/Complete", post(complete))
        .route("/Users/AuthenticateByName", post(authenticate))
        .route(
            "/Localization/Options",
            get(|| async {
                Json(serde_json::json!([
                    {"Name": "English (United States)", "Value": "en-US"},
                    {"Name": "Deutsch", "Value": "de"},
                ]))
            }),
        )
        .route(
            "/Localization/Cultures",
            get(|| async {
                Json(serde_json::json!([
                    {
                        "Name": "English",
                        "DisplayName": "English",
                        "TwoLetterISOLanguageName": "en",
                        "ThreeLetterISOLanguageName": "eng",
                        "ThreeLetterISOLanguageNames": ["eng"],
                    },
                ]))
            }),
        )
        .route(
            "/Localization/Countries",
            get(|| async {
                Json(serde_json::json!([
                    {
                        "Name": "US",
                        "DisplayName": "United States",
                        "TwoLetterISORegionName": "US",
                        "ThreeLetterISORegionName": "USA",
                    },
                ]))
            }),
        )
        .route(
            "/Environment/DefaultDirectoryBrowser",
            get(|| async { Json(serde_json::json!({"Path": "/media"})) }),
        )
        .route(
            "/Environment/ParentPath",
            get(|Query(query): Query<PathQuery>| async move {
                let path = query.path.unwrap_or_default();
                let parent = std::path::Path::new(&path)
                    .parent()
                    .map(|parent| parent.to_string_lossy().into_owned())
                    .unwrap_or_default();
                Json(serde_json::Value::String(parent))
            }),
        )
        .with_state(state.clone());
    (router, state)
}
