//! Jellyfin's built-in password reset, asked of the held login target.

use std::sync::Arc;

use axum::Json;
use axum::extract::{Query, State};
use axum::response::{IntoResponse, Response};
use jellium_protocol::{Refusal, ResetPin, ResetRequest, Targeted};

use super::super::AppState;
use super::{admitted, failed, refusal};

/// Asks `/Users/ForgotPassword` and answers which of Jellyfin's three answers
/// it gave; refused as `Refusal::ReadOnly` while the instance is read-only.
pub async fn forgot(
    State(state): State<Arc<AppState>>,
    Query(targeted): Query<Targeted>,
    Json(request): Json<ResetRequest>,
) -> Response {
    if state.read_only {
        return refusal(Refusal::ReadOnly);
    }
    let login = match admitted(&state, &targeted).await {
        Ok(login) => login,
        Err(response) => return response,
    };
    match login.forgot(&request.username).await {
        Ok(answer) => Json(answer).into_response(),
        Err(failure) => failed(failure),
    }
}

/// Redeems a pin and answers the accounts whose password was cleared; refused
/// as `Refusal::ReadOnly` while the instance is read-only.
pub async fn redeem(
    State(state): State<Arc<AppState>>,
    Query(targeted): Query<Targeted>,
    Json(pin): Json<ResetPin>,
) -> Response {
    if state.read_only {
        return refusal(Refusal::ReadOnly);
    }
    let login = match admitted(&state, &targeted).await {
        Ok(login) => login,
        Err(response) => return response,
    };
    match login.redeem(&pin.pin).await {
        Ok(outcome) => Json(outcome).into_response(),
        Err(failure) => failed(failure),
    }
}

#[cfg(test)]
mod tests {
    use super::super::harness::*;
    use crate::web::AppState;
    use axum::http::StatusCode;
    use jellium_protocol::{PinOutcome, ResetAnswer};

    #[tokio::test]
    async fn each_of_the_three_answers_is_carried_to_the_browser_distinctly() {
        let server = ready().await;
        let (router, _state) = routed(AppState::stub(scratch("three-answers")));
        let screen = opened(&router, &server.base).await;

        for (action, expected) in [
            (
                jellyfin_api::types::ForgotPasswordAction::PinCode,
                ResetAnswer::PinWritten {
                    pin_file: "/config/passwordreset-first.json".to_string(),
                    expires: Some(1_767_225_600_000),
                },
            ),
            (
                jellyfin_api::types::ForgotPasswordAction::ContactAdmin,
                ResetAnswer::ContactAdministrator,
            ),
            (
                jellyfin_api::types::ForgotPasswordAction::InNetworkRequired,
                ResetAnswer::InNetworkRequired,
            ),
        ] {
            server.login.answering(action);
            let (status, body) = sent(
                &router,
                "POST",
                &targeted(jellium_protocol::RESET_PATH, &screen.target),
                json(&jellium_protocol::ResetRequest {
                    username: "first".to_string(),
                }),
            )
            .await;
            assert_eq!(status, StatusCode::OK, "{}", String::from_utf8_lossy(&body));
            assert_eq!(decoded::<ResetAnswer>(&body), expected);
        }
    }

    #[tokio::test]
    async fn a_pin_the_server_refused_answers_refused_rather_than_cleared() {
        let server = ready().await;
        let (router, _state) = routed(AppState::stub(scratch("pin-refused")));
        let screen = opened(&router, &server.base).await;

        server.login.refuse_pin();
        let (_, body) = sent(
            &router,
            "POST",
            &targeted(jellium_protocol::RESET_PIN_PATH, &screen.target),
            json(&jellium_protocol::ResetPin {
                pin: "0000".to_string(),
            }),
        )
        .await;
        assert_eq!(decoded::<PinOutcome>(&body), PinOutcome::Refused);

        let (_, body) = sent(
            &router,
            "POST",
            &targeted(jellium_protocol::RESET_PIN_PATH, &screen.target),
            json(&jellium_protocol::ResetPin {
                pin: "0000".to_string(),
            }),
        )
        .await;
        assert_eq!(
            decoded::<PinOutcome>(&body),
            PinOutcome::Cleared {
                users: vec!["first".to_string()]
            }
        );
    }

    #[tokio::test]
    async fn a_read_only_instance_refuses_both_reset_endpoints() {
        let server = ready().await;
        let mut state = AppState::stub(scratch("read-only-reset"));
        state.read_only = true;
        let (router, _state) = routed(state);
        let screen = opened(&router, &server.base).await;
        assert!(screen.read_only);

        for (path, body) in [
            (
                jellium_protocol::RESET_PATH,
                json(&jellium_protocol::ResetRequest {
                    username: "first".to_string(),
                }),
            ),
            (
                jellium_protocol::RESET_PIN_PATH,
                json(&jellium_protocol::ResetPin {
                    pin: "0000".to_string(),
                }),
            ),
        ] {
            let (status, _) = sent(&router, "POST", &targeted(path, &screen.target), body).await;
            assert_eq!(status, StatusCode::FORBIDDEN, "{path}");
        }
        assert_eq!(server.asked("/Users/ForgotPassword"), 0);
    }
}
