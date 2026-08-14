use std::net::SocketAddr;
use std::sync::Arc;

use axum::Router;
use axum::extract::{Request, State};
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::{any, get};

use super::{AppState, assets, control, guard, relay};

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
}

impl Server {
    /// Serves the origin recorded in `state`; `secret` is the base64url form
    /// minted for this run.
    pub async fn bind(
        address: SocketAddr,
        state: Arc<AppState>,
        secret: String,
    ) -> Result<Server, std::io::Error> {
        let listener = tokio::net::TcpListener::bind(address).await?;
        let origin = state.origin.to_string();

        let router = Router::new()
            .route(
                jellium_protocol::SESSION_PATH,
                get(control::status)
                    .post(control::login)
                    .delete(control::logout),
            )
            .route(
                &format!("{}/{{*path}}", jellium_protocol::RELAY_PREFIX),
                any(relay::relay),
            )
            .fallback(any(dispatch))
            .layer(axum::middleware::from_fn_with_state(
                state.clone(),
                guard::guard,
            ))
            .with_state(state);

        Ok(Server {
            listener,
            origin,
            router,
            secret,
        })
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

    pub async fn serve(self) -> Result<(), std::io::Error> {
        axum::serve(self.listener, self.router).await
    }
}
