use std::sync::Arc;

use axum::Router;
use axum::extract::{Request, State};
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::{any, delete, get, post};

use super::{AppState, assets, control, foreign, guard, live, login, page, playback, relay, setup};

fn serve_asset(request: &Request) -> Response {
    match assets::lookup(request.uri().path()) {
        Some(asset) => ([(header::CONTENT_TYPE, asset.content_type)], asset.bytes).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn dispatch(state: State<Arc<AppState>>, request: Request) -> Response {
    if guard::is_entry(&request) {
        return guard::entry(state, request).await;
    }
    serve_asset(&request)
}

pub struct Server {
    listener: tokio::net::TcpListener,
    origin: String,
    router: Router,
    secret: String,
    state: Arc<AppState>,
}

/// Resolves on an interrupt, and on a termination signal where the platform
/// raises one.
async fn interrupted() {
    #[cfg(unix)]
    let terminate = async {
        match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            Ok(mut signal) => {
                signal.recv().await;
            }
            Err(_) => std::future::pending().await,
        }
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {}
        () = terminate => {}
    }
}

impl Server {
    /// Serves the origin recorded in `state` on `listener`; `secret` is the
    /// base64url form minted for this run.
    pub async fn on(
        listener: tokio::net::TcpListener,
        state: Arc<AppState>,
        secret: String,
    ) -> Server {
        let origin = state.origin.to_string();

        let router = Router::new()
            .route(
                &format!("{}/{{handle}}", jellium_protocol::FOREIGN_PREFIX),
                get(foreign::image),
            )
            .route(
                jellium_protocol::SESSION_PATH,
                get(control::status).delete(control::logout),
            )
            .route(
                jellium_protocol::SERVERS_PATH,
                get(login::servers::list)
                    .post(login::servers::add)
                    .delete(login::servers::remove),
            )
            .route(
                jellium_protocol::SERVER_SELECT_PATH,
                post(login::servers::select),
            )
            .route(jellium_protocol::SWITCH_PATH, post(login::servers::switch))
            .route(
                jellium_protocol::LOGIN_PATH,
                post(login::sign_in).delete(login::leave),
            )
            .route(
                &format!("{}/{{user}}/image", jellium_protocol::LOGIN_IMAGE_PREFIX),
                get(login::image),
            )
            .route(
                jellium_protocol::QUICK_CONNECT_PATH,
                post(login::quickconnect::initiate)
                    .get(login::quickconnect::poll)
                    .delete(login::quickconnect::abandon),
            )
            .route(jellium_protocol::RESET_PATH, post(login::reset::forgot))
            .route(jellium_protocol::RESET_PIN_PATH, post(login::reset::redeem))
            .route(jellium_protocol::SETUP_PATH, delete(setup::leave))
            .route(
                jellium_protocol::SETUP_CONFIGURATION_PATH,
                get(setup::configuration).post(setup::set_configuration),
            )
            .route(
                jellium_protocol::SETUP_USER_PATH,
                get(setup::user).post(setup::set_user),
            )
            .route(
                jellium_protocol::SETUP_REMOTE_ACCESS_PATH,
                get(setup::remote_access).post(setup::set_remote_access),
            )
            .route(jellium_protocol::SETUP_COMPLETE_PATH, post(setup::complete))
            .route(jellium_protocol::LIVE_PATH, get(live::events))
            .route(jellium_protocol::GROUP_LEAVE_PATH, post(live::leaving))
            .route(jellium_protocol::PLAYBACK_PATH, post(playback::start))
            .route(
                jellium_protocol::PLAYBACK_PROGRESS_PATH,
                post(playback::progress),
            )
            .route(
                jellium_protocol::PLAYBACK_STOPPED_PATH,
                post(playback::stopped),
            )
            .route(
                &format!("{}/{{*path}}", jellium_protocol::RELAY_PREFIX),
                any(relay::relay),
            )
            .route(
                jellium_protocol::PLUGIN_PATH,
                post(page::open).delete(page::close),
            )
            .route(
                &format!("{}/{{grant}}/{{name}}", jellium_protocol::PAGE_PREFIX),
                get(page::serve),
            )
            .fallback(any(dispatch))
            .layer(axum::middleware::from_fn_with_state(
                state.clone(),
                guard::guard,
            ))
            .with_state(state.clone());

        Server {
            listener,
            origin,
            router,
            secret,
            state,
        }
    }

    pub fn url(&self) -> String {
        format!("{}/", self.origin)
    }

    pub fn entry_url(&self) -> String {
        format!(
            "{}?{}={}",
            self.url(),
            jellium_protocol::SECRET_QUERY,
            self.secret
        )
    }

    /// Serves until the listener fails or an interrupt or termination signal
    /// arrives, sweeping lapsed playback sessions while it runs and ending the
    /// held one before it returns.
    /// The signal drops the connections still in flight, so a media element
    /// holding a stream open does not hold the exit.
    pub async fn serve(self) -> Result<(), std::io::Error> {
        let sweeper = tokio::spawn({
            let state = self.state.clone();
            async move {
                let mut ticks = tokio::time::interval(playback::Playback::SWEEP);
                loop {
                    ticks.tick().await;
                    state.playback.sweep(&state.session, &state.device).await;
                    state.live.swept(&state).await;
                    state.pages.sweep().await;
                }
            }
        });

        let served = tokio::select! {
            served = axum::serve(self.listener, self.router) => served,
            () = interrupted() => Ok(()),
        };

        sweeper.abort();
        self.state
            .playback
            .shutdown(&self.state.session, &self.state.device)
            .await;
        self.state.live.shutdown(&self.state).await;
        served
    }
}
