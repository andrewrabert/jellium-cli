use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use jellium_protocol::{Failure, Refusal, Session, SessionStatus};

use super::AppState;
use super::holder::{Held, Resumed};
use super::upstream::{self, Upstream};
use super::{login, setup, version};

/// The session as the browser takes it: what the upstream holds, carrying the
/// mode this instance was started in.
pub fn signed(state: &AppState, held: &Session) -> Session {
    Session {
        read_only: state.read_only,
        ..held.clone()
    }
}

/// Answers the held session, the wizard, one server's login screen, or the
/// saved server list.
/// A saved credential the Jellyfin server rejects is cleared from its record,
/// the record is kept, and that server's login screen is answered with
/// `rejected` set.
/// A login target held when the identity is announced is released, so a reload
/// resumes on the list.
/// Installing an identity that displaces a different one rebuilds the held
/// session's link, so every later request carries what this browser announced.
pub async fn announce(
    State(state): State<Arc<AppState>>,
    Json(announced): Json<jellium_protocol::Identity>,
) -> Response {
    if state.identity.install(announced).await {
        state.session.switch().await;
    }
    let Some(identity) = state.identity.held().await else {
        return (StatusCode::CONFLICT, Json(Refusal::NoSession)).into_response();
    };

    match state.session.held().await {
        Some(Held::Signed(upstream)) => {
            return Json(SessionStatus::Authenticated(signed(
                &state,
                &upstream.state,
            )))
            .into_response();
        }
        Some(Held::Setup(held)) => {
            return Json(SessionStatus::Setup(held.startup())).into_response();
        }
        Some(Held::Login(_)) => state.session.leave_login().await,
        None => {}
    }

    let Some(saved) = state.session.saved().await else {
        return Json(login::servers::anonymous(&state).await).into_response();
    };

    let probed = match version::probe(&saved.server).await {
        Ok(probed) => probed,
        Err(failure) => return Json(SessionStatus::Failed(failure)).into_response(),
    };
    if probed.startup {
        return setup::entered(&state, &saved.server, &probed, true).await;
    }

    let upstream = match Upstream::resume(&identity, &saved, &probed).await {
        Ok(upstream) => upstream,
        Err(Failure::TokenRejected) => {
            let server = saved.server.clone();
            state
                .session
                .write(move |file| {
                    if let Some(record) = file.find(&server) {
                        file.clear_credential(record);
                    }
                })
                .await;
            return login::entered(&state, &saved.server, true).await;
        }
        Err(failure) => {
            return Json(match state.session.signed().await {
                Some(held) => SessionStatus::Authenticated(signed(&state, &held.state)),
                None => SessionStatus::Failed(failure),
            })
            .into_response();
        }
    };

    match state.session.resumed(&saved, upstream).await {
        Resumed::Installed(held) | Resumed::Superseded(Some(held)) => {
            Json(SessionStatus::Authenticated(signed(&state, &held.state))).into_response()
        }
        Resumed::Superseded(None) => Json(login::servers::anonymous(&state).await).into_response(),
    }
}

/// Ends everything the held session owned: the playback session, any SyncPlay
/// group, remote mode and the event socket.
pub async fn ended(state: &Arc<AppState>) {
    if let Some(identity) = state.identity.held().await {
        state.playback.shutdown(&state.session, &identity).await;
    }
    state.live.shutdown(state).await;
}

/// Revokes the held session's token and clears its record's credential,
/// leaving the server saved; refused as `Refusal::ReadOnly` while the instance
/// is read-only, because ending a session under that mode is jellium-cli's job.
pub async fn logout(State(state): State<Arc<AppState>>) -> Response {
    if state.read_only {
        return (StatusCode::FORBIDDEN, Json(Refusal::ReadOnly)).into_response();
    }
    ended(&state).await;
    match state.session.revoke().await {
        Ok(()) => {
            state.live.rebound(&state).await;
            StatusCode::NO_CONTENT.into_response()
        }
        Err(failure) => failure_response(failure),
    }
}

fn failure_response(failure: Failure) -> Response {
    (upstream::status_for(&failure), Json(failure)).into_response()
}
