//! The stub upstream's login area: the public user list, a user primary image,
//! the four Quick Connect routes and the two password-reset routes.

use std::sync::{Arc, Mutex};

use axum::Json;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};

/// A Jellyfin server answering the login stage's questions.
#[derive(Clone)]
pub struct Login {
    held: Arc<Mutex<Held>>,
}

struct Held {
    authorized: bool,
    disabled: bool,
    expired: bool,
    connects: usize,
    answering: jellyfin_api::types::ForgotPasswordAction,
    refuse_pin: bool,
}

impl Login {
    /// The code `/QuickConnect/Initiate` answers with.
    pub const CODE: &'static str = "424242";

    /// The secret `/QuickConnect/Initiate` answers with, which no browser-facing
    /// response may carry.
    pub const SECRET: &'static str = "synthetic-quick-connect-secret";

    /// The token `/Users/AuthenticateWithQuickConnect` answers with.
    pub const TOKEN: &'static str = "synthetic-quick-connect-token";

    fn new() -> Login {
        Login {
            held: Arc::new(Mutex::new(Held {
                authorized: false,
                disabled: false,
                expired: false,
                connects: 0,
                answering: jellyfin_api::types::ForgotPasswordAction::PinCode,
                refuse_pin: false,
            })),
        }
    }

    fn held(&self) -> std::sync::MutexGuard<'_, Held> {
        self.held.lock().expect("the synthetic login area")
    }

    /// The public users `/Users/Public` answers with.
    pub fn users(&self) -> Vec<jellyfin_api::types::UserDto> {
        vec![
            jellyfin_api::types::UserDto {
                id: Some(uuid::Uuid::from_u128(1)),
                name: Some("first".to_string()),
                primary_image_tag: Some("tag".to_string()),
                ..Default::default()
            },
            jellyfin_api::types::UserDto {
                id: Some(uuid::Uuid::from_u128(2)),
                name: Some("second".to_string()),
                ..Default::default()
            },
        ]
    }

    /// Makes the request report itself authorized, which is what a second
    /// device does.
    pub fn authorize(&self) {
        self.held().authorized = true;
    }

    /// Turns Quick Connect off, so `/QuickConnect/Enabled` reports false and
    /// `/QuickConnect/Connect` answers 401.
    pub fn disable(&self) {
        self.held().disabled = true;
    }

    /// Makes the request age out, so `/QuickConnect/Connect` answers 404.
    pub fn expire(&self) {
        self.held().expired = true;
    }

    /// How many `/QuickConnect/Connect` requests arrived.
    pub fn connects(&self) -> usize {
        self.held().connects
    }

    /// Which of the three answers `/Users/ForgotPassword` gives.
    pub fn answering(&self, action: jellyfin_api::types::ForgotPasswordAction) {
        self.held().answering = action;
    }

    /// Makes the next `/Users/ForgotPassword/Pin` refuse the pin.
    pub fn refuse_pin(&self) {
        self.held().refuse_pin = true;
    }
}

async fn public_users(State(state): State<Login>) -> Json<Vec<jellyfin_api::types::UserDto>> {
    Json(state.users())
}

async fn user_image() -> Response {
    (
        [(axum::http::header::CONTENT_TYPE, "image/png")],
        vec![0x89u8, 0x50, 0x4e, 0x47],
    )
        .into_response()
}

async fn enabled(State(state): State<Login>) -> Json<bool> {
    Json(!state.held().disabled)
}

async fn initiate(State(state): State<Login>) -> Response {
    if state.held().disabled {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    Json(serde_json::json!({
        "Authenticated": false,
        "Code": Login::CODE,
        "Secret": Login::SECRET,
    }))
    .into_response()
}

#[derive(serde::Deserialize)]
struct SecretQuery {
    secret: String,
}

async fn connect(State(state): State<Login>, Query(query): Query<SecretQuery>) -> Response {
    let mut held = state.held();
    held.connects += 1;
    if held.disabled {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    if held.expired || query.secret != Login::SECRET {
        return StatusCode::NOT_FOUND.into_response();
    }
    Json(serde_json::json!({
        "Authenticated": held.authorized,
        "Code": Login::CODE,
        "Secret": Login::SECRET,
    }))
    .into_response()
}

/// Jellyfin mints the session's device identity from the `MediaBrowser`
/// authorization header, so an exchange carrying none is refused here too.
async fn authenticate_with_quick_connect(
    headers: axum::http::HeaderMap,
    Json(body): Json<jellyfin_api::types::QuickConnectDto>,
) -> Response {
    if !super::startup::identified(&headers) || body.secret != Login::SECRET {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    Json(serde_json::json!({
        "AccessToken": Login::TOKEN,
        "User": {
            "Id": uuid::Uuid::from_u128(1),
            "Name": "first",
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

async fn forgot_password(State(state): State<Login>) -> Json<serde_json::Value> {
    let action = state.held().answering;
    Json(serde_json::json!({
        "Action": action,
        "PinFile": "/config/passwordreset-first.json",
        "PinExpirationDate": "2026-01-01T00:00:00Z",
    }))
}

async fn forgot_password_pin(State(state): State<Login>) -> Json<serde_json::Value> {
    let refused = std::mem::replace(&mut state.held().refuse_pin, false);
    Json(if refused {
        serde_json::json!({ "Success": false, "UsersReset": [] })
    } else {
        serde_json::json!({ "Success": true, "UsersReset": ["first"] })
    })
}

/// The synthetic login area and the router serving it.
pub fn router() -> (axum::Router, Login) {
    let state = Login::new();
    let router = axum::Router::new()
        .route("/Users/Public", get(public_users))
        .route("/UserImage", get(user_image))
        .route("/QuickConnect/Enabled", get(enabled))
        .route("/QuickConnect/Initiate", post(initiate))
        .route("/QuickConnect/Connect", get(connect))
        .route(
            "/Users/AuthenticateWithQuickConnect",
            post(authenticate_with_quick_connect),
        )
        .route("/Users/ForgotPassword", post(forgot_password))
        .route("/Users/ForgotPassword/Pin", post(forgot_password_pin))
        .with_state(state.clone());
    (router, state)
}
