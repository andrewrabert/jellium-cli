use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use jellium_protocol::{Credentials, Failure, SessionStatus};

use super::AppState;
use super::holder::Resumed;
use super::upstream::{self, Upstream};

pub async fn status(State(state): State<Arc<AppState>>) -> Json<SessionStatus> {
    if let Some(upstream) = state.session.held().await {
        return Json(SessionStatus::Authenticated(upstream.state.clone()));
    }

    let Some(saved) = state.session.saved().await else {
        return Json(SessionStatus::Anonymous);
    };

    let upstream = match Upstream::resume(&state.device, &saved).await {
        Ok(upstream) => upstream,
        Err(failure) => {
            return Json(match state.session.held().await {
                Some(held) => SessionStatus::Authenticated(held.state.clone()),
                None => SessionStatus::Failed(failure),
            });
        }
    };

    Json(match state.session.resumed(&saved, upstream).await {
        Resumed::Installed(held) | Resumed::Superseded(Some(held)) => {
            SessionStatus::Authenticated(held.state.clone())
        }
        Resumed::Superseded(None) => SessionStatus::Anonymous,
    })
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(credentials): Json<Credentials>,
) -> Response {
    match Upstream::login(&state.device, &credentials).await {
        Ok(upstream) => {
            let installed = state.session.install(upstream).await;
            Json(SessionStatus::Authenticated(installed.state.clone())).into_response()
        }
        Err(failure) => failure_response(failure),
    }
}

pub async fn logout(State(state): State<Arc<AppState>>) -> Response {
    match state.session.revoke().await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(failure) => failure_response(failure),
    }
}

fn failure_response(failure: Failure) -> Response {
    (upstream::status_for(&failure), Json(failure)).into_response()
}
