use std::sync::Arc;

use axum::Json;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use jellium_protocol::{Failure, Refusal};

use super::AppState;
use super::route;
use super::upstream;

pub async fn relay(state: State<Arc<AppState>>, request: Request) -> Response {
    let raw = request
        .uri()
        .path()
        .strip_prefix(jellium_protocol::RELAY_PREFIX)
        .unwrap_or_default();
    let Some(target) = route::Target::admit(request.method(), raw) else {
        return (StatusCode::FORBIDDEN, Json(Refusal::NotRelayed)).into_response();
    };

    let Some(upstream) = state.session.held().await else {
        return (StatusCode::CONFLICT, Json(Refusal::NoSession)).into_response();
    };

    let (parts, body) = request.into_parts();
    let outcome = upstream
        .forward(&target, parts.uri.query(), &parts.headers, body)
        .await;

    match outcome {
        Ok(response) => response,
        Err(failure) => {
            if failure == Failure::TokenRejected {
                state.session.reject(&upstream).await;
            }
            (upstream::status_for(&failure), Json(failure)).into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::Router;
    use axum::body::Body;
    use axum::http::Request;
    use axum::routing::any;
    use tower::ServiceExt;

    fn scratch(name: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir()
            .join("jellium-cli-relay-tests")
            .join(name);
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("scratch directory");
        path.join("session.env")
    }

    fn router(name: &str) -> Router {
        let state = Arc::new(AppState::stub(scratch(name)));
        Router::new()
            .route(
                &format!("{}/{{*path}}", jellium_protocol::RELAY_PREFIX),
                any(relay),
            )
            .with_state(state)
    }

    async fn status(router: Router, uri: &str) -> StatusCode {
        router
            .oneshot(
                Request::builder()
                    .uri(uri)
                    .body(Body::empty())
                    .expect("a request"),
            )
            .await
            .expect("a response")
            .status()
    }

    #[tokio::test]
    async fn an_encoded_delimiter_is_refused_at_the_router() {
        for uri in [
            "/jellyfin/Items%2FBad",
            "/jellyfin/Items%2fBad",
            "/jellyfin/Items%3Fx=1",
            "/jellyfin/Items%23x",
        ] {
            assert_eq!(
                status(router("encoded-delimiter"), uri).await,
                StatusCode::FORBIDDEN
            );
        }
    }

    #[tokio::test]
    async fn a_served_path_reaches_the_gate() {
        assert_eq!(
            status(router("served-path"), "/jellyfin/Items").await,
            StatusCode::CONFLICT
        );
    }
}
