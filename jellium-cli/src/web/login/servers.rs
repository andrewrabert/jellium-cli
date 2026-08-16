//! The saved server list and the acts that move it: add, select, switch and
//! remove.

use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use jellium_protocol::{
    AddServer, ChooseServer, Failure, Refusal, Removed, SavedServer, SessionStatus,
};

use super::super::AppState;
use super::super::holder;
use super::super::upstream::Upstream;
use super::super::version;
use super::{failed, refusal};

/// Every saved server, in file order, with the first marked active.
pub async fn saved(state: &AppState) -> Vec<SavedServer> {
    state
        .session
        .records()
        .await
        .into_iter()
        .enumerate()
        .map(|(record, held)| SavedServer {
            server: held.server,
            name: held.name,
            credentialed: held.credential.is_some(),
            active: record == 0,
        })
        .collect()
}

/// The whole anonymous document: the saved servers and this instance's
/// read-only flag.
pub async fn anonymous(state: &AppState) -> SessionStatus {
    SessionStatus::Anonymous {
        servers: saved(state).await,
        read_only: state.read_only,
    }
}

/// Every saved server, read from the session file on every call and never
/// cached.
pub async fn list(State(state): State<Arc<AppState>>) -> Response {
    Json(saved(&state).await).into_response()
}

/// Probes the typed text, gates its version, and answers the login screen of
/// the server that replied.
/// A url normalizing to one already saved selects that record instead of
/// writing a second.
/// A probe that fails and a server below the floor write no record and answer
/// `SessionStatus::Failed`.
/// A server reporting startup mode writes no record and walks into the wizard.
pub async fn add(State(state): State<Arc<AppState>>, Json(added): Json<AddServer>) -> Response {
    let (server, probed) = match version::probe_typed(&added.url).await {
        Ok(answered) => answered,
        Err(failure) => return failed(failure),
    };
    if probed.startup {
        return super::super::setup::entered(&state, &server, &probed, false).await;
    }
    let saved = server.clone();
    state
        .session
        .write(move |file| file.add_server(&saved))
        .await;
    super::entered(&state, &server, false).await
}

/// Selects a saved server: probes it, gates its version, checks startup mode,
/// then installs its session when it holds a credential and its login screen
/// when it does not.
/// A credential the Jellyfin server rejects is cleared from the record, the
/// record is kept, and the login screen is answered with `rejected` set.
/// A url no record holds is refused as `Refusal::NotRelayed`.
pub async fn select(
    State(state): State<Arc<AppState>>,
    Json(chosen): Json<ChooseServer>,
) -> Response {
    let wanted = crate::session::normalized(&chosen.server);
    let Some(record) = state
        .session
        .records()
        .await
        .into_iter()
        .find(|held| crate::session::normalized(&held.server) == wanted)
    else {
        return refusal(Refusal::NotRelayed);
    };

    let Some(identity) = state.identity.held().await else {
        return refusal(Refusal::NoSession);
    };

    let probed = match version::probe(&record.server).await {
        Ok(probed) => probed,
        Err(failure) => return failed(failure),
    };
    if probed.startup {
        return super::super::setup::entered(&state, &record.server, &probed, false).await;
    }

    let Some(session) = record.session() else {
        activated(&state, &record.server).await;
        return super::entered(&state, &record.server, false).await;
    };

    match Upstream::resume(&identity, &session, &probed).await {
        Ok(upstream) => {
            super::super::control::ended(&state).await;
            let installed = state.session.install(upstream).await;
            state.live.rebound(&state).await;
            Json(SessionStatus::Authenticated(super::super::control::signed(
                &state,
                &installed.state,
            )))
            .into_response()
        }
        Err(Failure::TokenRejected) => {
            let server = record.server.clone();
            state
                .session
                .write(move |file| {
                    if let Some(record) = file.find(&server) {
                        file.clear_credential(record);
                    }
                })
                .await;
            activated(&state, &record.server).await;
            super::entered(&state, &record.server, true).await
        }
        Err(failure) => failed(failure),
    }
}

/// Moves `server`'s record to the front, which is what selecting it does.
async fn activated(state: &AppState, server: &str) {
    let server = server.to_string();
    state
        .session
        .write(move |file| {
            if let Some(record) = file.find(&server) {
                file.activate(record);
            }
        })
        .await;
}

/// Releases the held upstream without revoking it, ends everything that session
/// held, and answers `SessionStatus::Anonymous`.
pub async fn switch(State(state): State<Arc<AppState>>) -> Response {
    super::super::control::ended(&state).await;
    state.session.switch().await;
    state.live.rebound(&state).await;
    Json(anonymous(&state).await).into_response()
}

/// Revokes the record's token upstream and deletes the record; removing the
/// active server signs out first.
/// Refused as `Refusal::ReadOnly` while the instance is read-only and the
/// record holds a credential; a record holding none is removed either way.
pub async fn remove(
    State(state): State<Arc<AppState>>,
    Json(chosen): Json<ChooseServer>,
) -> Response {
    let wanted = crate::session::normalized(&chosen.server);
    let Some(record) = state
        .session
        .records()
        .await
        .into_iter()
        .find(|held| crate::session::normalized(&held.server) == wanted)
    else {
        return Json(Removed::Unknown).into_response();
    };
    if state.read_only && record.credential.is_some() {
        return refusal(Refusal::ReadOnly);
    }
    let Some(identity) = state.identity.held().await else {
        return refusal(Refusal::NoSession);
    };
    super::super::control::ended(&state).await;
    let removed = match state.session.remove(&identity, &chosen.server).await {
        holder::Removed::Deleted => Removed::Deleted,
        holder::Removed::DeletedUnrevoked => Removed::DeletedUnrevoked,
        holder::Removed::Unknown => Removed::Unknown,
    };
    state.live.rebound(&state).await;
    Json(removed).into_response()
}

#[cfg(test)]
mod tests {
    use super::super::harness::*;
    use super::*;
    use axum::http::StatusCode;
    use jellium_protocol::{LoginScreen, Session};

    fn credentialed(server: &str) -> crate::session::Session {
        crate::session::Session {
            server: server.to_string(),
            token: "seeded".to_string(),
            user_id: uuid::Uuid::from_u128(1),
        }
    }

    async fn status_of(router: &axum::Router) -> SessionStatus {
        let (status, body) = sent(
            router,
            "POST",
            jellium_protocol::IDENTITY_PATH,
            serde_json::to_vec(&jellium_protocol::Identity {
                device: "Firefox".to_owned(),
                device_id: uuid::Uuid::nil().to_string(),
            })
            .expect("the identity serializes"),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{}", String::from_utf8_lossy(&body));
        decoded(&body)
    }

    #[tokio::test]
    async fn launching_with_a_credentialed_active_record_signs_straight_in() {
        let server = ready().await;
        let path = scratch("launch-credentialed");
        let state = AppState::stub(path);
        let (router, state) = routed(state);
        state
            .session
            .write({
                let record = credentialed(&server.base);
                move |file| {
                    file.add_server("https://other.test");
                    file.set_server(&record);
                }
            })
            .await;

        assert!(matches!(
            status_of(&router).await,
            SessionStatus::Authenticated(Session { .. })
        ));
    }

    #[tokio::test]
    async fn launching_with_saved_servers_and_no_credential_answers_the_list() {
        let (router, state) = routed(AppState::stub(scratch("launch-listed")));
        state
            .session
            .write(|file| {
                file.add_server("https://second.test");
                file.add_server("https://first.test");
            })
            .await;

        match status_of(&router).await {
            SessionStatus::Anonymous { servers, read_only } => {
                assert!(!read_only);
                assert_eq!(servers.len(), 2);
                assert_eq!(servers[0].server, "https://first.test");
                assert!(servers[0].active);
                assert!(!servers[1].active);
            }
            other => panic!("the list was answered: {other:?}"),
        }
    }

    #[tokio::test]
    async fn launching_with_no_saved_server_answers_an_empty_list() {
        let (router, _state) = routed(AppState::stub(scratch("launch-empty")));
        match status_of(&router).await {
            SessionStatus::Anonymous { servers, .. } => assert!(servers.is_empty()),
            other => panic!("the empty list was answered: {other:?}"),
        }
    }

    #[tokio::test]
    async fn a_rejected_saved_credential_clears_it_and_answers_that_servers_login_screen() {
        let server = ready().await;
        let (router, state) = routed(AppState::stub(scratch("rejected-credential")));
        state
            .session
            .write({
                let record = crate::session::Session {
                    server: server.base.clone(),
                    token: String::new(),
                    user_id: uuid::Uuid::from_u128(1),
                };
                move |file| file.set_server(&record)
            })
            .await;

        match status_of(&router).await {
            SessionStatus::Login(LoginScreen { rejected, .. }) => assert!(rejected),
            other => panic!("the login screen named the rejected sign-in: {other:?}"),
        }
        let records = state.session.records().await;
        assert_eq!(records.len(), 1);
        assert!(records[0].credential.is_none());
    }

    #[tokio::test]
    async fn a_url_normalizing_to_a_saved_one_selects_it_rather_than_adding_a_second() {
        let server = ready().await;
        let (router, state) = routed(AppState::stub(scratch("normalizing-add")));
        opened(&router, &server.base).await;
        opened(&router, &format!("{}/", server.base)).await;
        assert_eq!(state.session.records().await.len(), 1);
    }

    #[tokio::test]
    async fn a_server_below_the_floor_writes_no_record() {
        let server = ready().await;
        server.startup.below_minimum();
        let (router, state) = routed(AppState::stub(scratch("below-floor")));

        let (status, body) = sent(
            &router,
            "POST",
            jellium_protocol::SERVERS_PATH,
            json(&jellium_protocol::AddServer {
                url: server.base.clone(),
            }),
        )
        .await;
        assert_eq!(status, StatusCode::CONFLICT);
        assert!(matches!(
            decoded::<Failure>(&body),
            Failure::ServerBelowMinimum { .. }
        ));
        assert!(state.session.records().await.is_empty());
    }

    #[tokio::test]
    async fn after_a_switch_the_command_lines_session_resolution_returns_the_new_active_server() {
        let server = ready().await;
        let (router, state) = routed(AppState::stub(scratch("switch-active")));
        state
            .session
            .write({
                let record = credentialed(&server.base);
                move |file| {
                    file.add_server("https://other.test");
                    file.set_server(&record);
                }
            })
            .await;
        assert!(matches!(
            status_of(&router).await,
            SessionStatus::Authenticated(_)
        ));

        let (status, body) = sent(&router, "POST", jellium_protocol::SWITCH_PATH, Vec::new()).await;
        assert_eq!(status, StatusCode::OK);
        assert!(matches!(
            decoded::<SessionStatus>(&body),
            SessionStatus::Anonymous { .. }
        ));
        assert!(state.session.signed().await.is_none());
        assert!(state.session.saved().await.is_some());
        assert_eq!(server.asked("/Sessions/Logout"), 0);
    }

    #[tokio::test]
    async fn removing_an_entry_revokes_before_deleting_it() {
        let server = ready().await;
        let (router, state) = routed(AppState::stub(scratch("remove-revokes")));
        state
            .session
            .write({
                let record = credentialed(&server.base);
                move |file| file.set_server(&record)
            })
            .await;

        let (status, body) = sent(
            &router,
            "DELETE",
            jellium_protocol::SERVERS_PATH,
            json(&jellium_protocol::ChooseServer {
                server: server.base.clone(),
            }),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(decoded::<Removed>(&body), Removed::Deleted);
        assert_eq!(server.asked("/Sessions/Logout"), 1);
        assert!(state.session.records().await.is_empty());
    }

    #[tokio::test]
    async fn a_read_only_instance_removes_a_credentialless_record_and_refuses_a_credentialed_one() {
        let server = ready().await;
        let mut state = AppState::stub(scratch("read-only-remove"));
        state.read_only = true;
        let (router, state) = routed(state);
        state
            .session
            .write({
                let record = credentialed(&server.base);
                move |file| {
                    file.add_server("https://bare.test");
                    file.set_server(&record);
                }
            })
            .await;

        let (status, _) = sent(
            &router,
            "DELETE",
            jellium_protocol::SERVERS_PATH,
            json(&jellium_protocol::ChooseServer {
                server: server.base.clone(),
            }),
        )
        .await;
        assert_eq!(status, StatusCode::FORBIDDEN);

        let (status, body) = sent(
            &router,
            "DELETE",
            jellium_protocol::SERVERS_PATH,
            json(&jellium_protocol::ChooseServer {
                server: "https://bare.test".to_string(),
            }),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(decoded::<Removed>(&body), Removed::Deleted);
        assert_eq!(state.session.records().await.len(), 1);
    }
}
