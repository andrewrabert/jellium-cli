//! Quick Connect sign-in against the held login target.

use std::sync::Arc;

use axum::Json;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use jellium_protocol::{QuickConnectState, Targeted};

use super::super::AppState;
use super::{Connected, admitted, failed};

/// Initiates a request and answers its code; the secret is held here and
/// written to nothing.
pub async fn initiate(
    State(state): State<Arc<AppState>>,
    Query(targeted): Query<Targeted>,
) -> Response {
    let login = match admitted(&state, &targeted).await {
        Ok(login) => login,
        Err(response) => return response,
    };
    match login.initiate().await {
        Ok(code) => Json(code).into_response(),
        Err(failure) => failed(failure),
    }
}

/// One poll: at most one `GET /QuickConnect/Connect` upstream, answering
/// pending, the installed session, expired, or Quick Connect disabled.
pub async fn poll(
    State(state): State<Arc<AppState>>,
    Query(targeted): Query<Targeted>,
) -> Response {
    let login = match admitted(&state, &targeted).await {
        Ok(login) => login,
        Err(response) => return response,
    };
    let Some(identity) = state.identity.held().await else {
        return super::refusal(jellium_protocol::Refusal::NoSession);
    };
    match login.connect(&identity).await {
        Ok(Connected::Pending) => Json(QuickConnectState::Pending).into_response(),
        Ok(Connected::Expired) => Json(QuickConnectState::Expired).into_response(),
        Ok(Connected::Disabled) => Json(QuickConnectState::Disabled).into_response(),
        Ok(Connected::Authorized(upstream)) => {
            let installed = state.session.install(*upstream).await;
            state.live.rebound(&state).await;
            Json(QuickConnectState::Signed(super::super::control::signed(
                &state,
                &installed.state,
            )))
            .into_response()
        }
        Err(failure) => failed(failure),
    }
}

/// Drops the held secret, which is what leaving the screen does.
pub async fn abandon(
    State(state): State<Arc<AppState>>,
    Query(targeted): Query<Targeted>,
) -> Response {
    let login = match admitted(&state, &targeted).await {
        Ok(login) => login,
        Err(response) => return response,
    };
    login.abandon();
    StatusCode::NO_CONTENT.into_response()
}

#[cfg(test)]
mod tests {
    use super::super::harness::*;
    use crate::web::AppState;
    use crate::web::synthetic::Login;
    use axum::http::StatusCode;
    use jellium_protocol::QuickConnectState;

    /// Opens a login screen and initiates one request, answering the target and
    /// every byte the two responses carried.
    async fn initiated(
        name: &str,
    ) -> (
        axum::Router,
        std::sync::Arc<AppState>,
        crate::web::upstream::Answering,
        String,
        Vec<u8>,
    ) {
        let server = ready().await;
        let (router, state) = routed(AppState::stub(scratch(name)));
        let screen = opened(&router, &server.base).await;
        assert!(screen.quick_connect);

        let (status, body) = sent(
            &router,
            "POST",
            &targeted(jellium_protocol::QUICK_CONNECT_PATH, &screen.target),
            Vec::new(),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{}", String::from_utf8_lossy(&body));
        let code: jellium_protocol::QuickConnectCode = decoded(&body);
        assert_eq!(code.code, Login::CODE);
        (router, state, server, screen.target, body)
    }

    #[tokio::test]
    async fn no_browser_facing_response_across_a_whole_sign_in_carries_the_secret() {
        let (router, _state, server, target, mut seen) = initiated("secret-never-leaves").await;
        server.login.authorize();
        let (_, polled) = sent(
            &router,
            "GET",
            &targeted(jellium_protocol::QUICK_CONNECT_PATH, &target),
            Vec::new(),
        )
        .await;
        seen.extend_from_slice(&polled);
        let text = String::from_utf8_lossy(&seen);
        assert!(!text.contains(Login::SECRET), "{text}");
    }

    #[tokio::test]
    async fn one_browser_poll_issues_one_upstream_connect() {
        let (router, _state, server, target, _) = initiated("one-poll-one-connect").await;
        assert_eq!(server.login.connects(), 0);
        for expected in 1..=3 {
            sent(
                &router,
                "GET",
                &targeted(jellium_protocol::QUICK_CONNECT_PATH, &target),
                Vec::new(),
            )
            .await;
            assert_eq!(server.login.connects(), expected);
        }
    }

    #[tokio::test]
    async fn initiate_and_the_exchange_both_present_this_installations_device_identity() {
        let (router, _state, server, target, _) = initiated("identity-presented").await;
        server.login.authorize();
        sent(
            &router,
            "GET",
            &targeted(jellium_protocol::QUICK_CONNECT_PATH, &target),
            Vec::new(),
        )
        .await;

        for path in [
            "/QuickConnect/Initiate",
            "/Users/AuthenticateWithQuickConnect",
        ] {
            let presented = server
                .taken
                .authorization(path)
                .unwrap_or_else(|| panic!("{path} presented an identity"));
            assert!(presented.starts_with("MediaBrowser "), "{presented}");
            assert!(
                presented.contains(r#"Client="Jellyfin Web""#),
                "{presented}"
            );
            assert!(presented.contains(r#"Version="10.11.11""#), "{presented}");
            assert!(!presented.contains("Token="), "{presented}");
        }
    }

    #[tokio::test]
    async fn an_authorized_request_is_exchanged_and_installed_before_the_poll_answers() {
        let (router, state, server, target, _) = initiated("authorized-installed").await;
        server.login.authorize();
        let (status, body) = sent(
            &router,
            "GET",
            &targeted(jellium_protocol::QUICK_CONNECT_PATH, &target),
            Vec::new(),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{}", String::from_utf8_lossy(&body));
        assert!(matches!(
            decoded::<QuickConnectState>(&body),
            QuickConnectState::Signed(_)
        ));
        assert!(state.session.signed().await.is_some());
    }

    #[tokio::test]
    async fn a_disabled_quick_connect_takes_the_option_off_the_login_screen() {
        let (router, _state, server, target, _) = initiated("disabled-quick-connect").await;
        server.login.disable();
        let (_, body) = sent(
            &router,
            "GET",
            &targeted(jellium_protocol::QUICK_CONNECT_PATH, &target),
            Vec::new(),
        )
        .await;
        assert_eq!(
            decoded::<QuickConnectState>(&body),
            QuickConnectState::Disabled
        );

        sent(&router, "DELETE", jellium_protocol::LOGIN_PATH, Vec::new()).await;
        let screen = opened(&router, &server.base).await;
        assert!(!screen.quick_connect);
    }

    #[tokio::test]
    async fn an_expired_request_answers_expired_rather_than_pending() {
        let (router, _state, server, target, _) = initiated("expired-request").await;
        server.login.expire();
        let (_, body) = sent(
            &router,
            "GET",
            &targeted(jellium_protocol::QUICK_CONNECT_PATH, &target),
            Vec::new(),
        )
        .await;
        assert_eq!(
            decoded::<QuickConnectState>(&body),
            QuickConnectState::Expired
        );
    }

    #[tokio::test]
    async fn the_secret_never_reaches_the_session_file() {
        let (router, state, server, target, _) = initiated("secret-not-on-file").await;
        server.login.authorize();
        sent(
            &router,
            "GET",
            &targeted(jellium_protocol::QUICK_CONNECT_PATH, &target),
            Vec::new(),
        )
        .await;
        let records = state.session.records().await;
        assert_eq!(records.len(), 1);
        let credential = records[0].credential.as_ref().expect("a credential");
        assert_eq!(credential.token, Login::TOKEN);
        assert_ne!(credential.token, Login::SECRET);
    }
}
