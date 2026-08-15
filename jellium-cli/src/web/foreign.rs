//! The one route a foreign image reaches the browser through, fetched by the
//! local server at a url Jellyfin has just pointed it at, over a client that
//! holds no credential.

use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::{StatusCode, header};
use axum::response::Response;

use super::AppState;
use super::link::CONNECT_TIMEOUT;

/// The client every foreign image is fetched over: built with no default
/// headers, so no Jellyfin access token, no device identity and no client name
/// can reach a provider.
pub struct Anonymous {
    client: reqwest::Client,
}

impl Anonymous {
    /// Carries `link::CONNECT_TIMEOUT` and no total deadline, matching the
    /// link's streaming client, and no default header of any kind.
    pub fn new() -> Anonymous {
        Anonymous {
            client: reqwest::Client::builder()
                .connect_timeout(CONNECT_TIMEOUT)
                .build()
                .expect("a client with no default headers"),
        }
    }

    /// The image at `url`, streamed back same-origin.
    /// A request that never reached the host, and a status that is not a
    /// success, both answer `Err(())`.
    pub async fn fetch(&self, url: &str) -> Result<Response, ()> {
        let response = self.client.get(url).send().await.map_err(|_| ())?;
        if !response.status().is_success() {
            return Err(());
        }
        let content_type = response.headers().get(header::CONTENT_TYPE).cloned();
        let mut builder = Response::builder().status(StatusCode::OK);
        if let Some(content_type) = content_type {
            builder = builder.header(header::CONTENT_TYPE, content_type);
        }
        builder
            .body(axum::body::Body::from_stream(response.bytes_stream()))
            .map_err(|_| ())
    }
}

impl Default for Anonymous {
    fn default() -> Anonymous {
        Anonymous::new()
    }
}

/// Serves the image `handle` names, fetched by the local server from the url it
/// observed and streamed back same-origin.
/// A handle this run did not mint answers `404` with no body, which every
/// surface showing one draws as a missing image.
pub async fn image(state: State<Arc<AppState>>, Path(handle): Path<String>) -> Response {
    let Some(url) = state.seen.observed(&handle) else {
        return missing();
    };
    // a run holding no session answers missing, so a signed-out browser pulls
    // no provider image
    if state.session.held().await.is_none() {
        return missing();
    }
    match state.foreign.fetch(&url).await {
        Ok(answer) => answer,
        Err(()) => missing(),
    }
}

fn missing() -> Response {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(axum::body::Body::empty())
        .expect("an empty body")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::web::synthetic::Library;
    use crate::web::upstream::{Upstream, answering};
    use axum::Router;
    use axum::body::Body;
    use axum::http::{Request, header};
    use axum::routing::get;
    use tower::ServiceExt;
    use uuid::Uuid;

    fn scratch(name: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir()
            .join("jellium-cli-foreign-tests")
            .join(name);
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("scratch directory");
        path.join("session.env")
    }

    #[tokio::test]
    async fn a_foreign_image_is_fetched_carrying_no_credential_and_no_identity() {
        let server = answering(204).await;
        let state = Arc::new(AppState::stub(scratch("foreign-headerless")));
        state.session.install(Upstream::stub(&server.base)).await;

        let url = server.library.foreign()[0].clone();
        let minted = state.seen.foreign(&format!(r#"{{"ImageUrl":"{url}"}}"#));
        let handle = serde_json::from_str::<serde_json::Value>(&minted).expect("json")["ImageUrl"]
            .as_str()
            .expect("a handle")
            .to_owned();

        let router = Router::new()
            .route(
                &format!("{}/{{handle}}", jellium_protocol::FOREIGN_PREFIX),
                get(image),
            )
            .with_state(state);
        let answered = router
            .oneshot(
                Request::builder()
                    .uri(format!("{}/{handle}", jellium_protocol::FOREIGN_PREFIX))
                    .body(Body::empty())
                    .expect("a request"),
            )
            .await
            .expect("a response");
        assert_eq!(answered.status(), StatusCode::OK);

        let carried = server
            .taken
            .headers(Library::PROVIDER[0])
            .expect("the stub took the foreign fetch");
        assert!(carried.get(header::AUTHORIZATION).is_none());
        for (name, value) in &carried {
            let value = value.to_str().unwrap_or_default();
            for secret in [
                "MediaBrowser",
                "Jellium Web",
                "token",
                &Uuid::nil().to_string(),
            ] {
                assert!(!value.contains(secret), "{name}: {value}");
            }
        }
    }
}
