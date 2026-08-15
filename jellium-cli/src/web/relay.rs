use std::sync::Arc;

use axum::Json;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use jellium_protocol::{Failure, Refusal};

use super::AppState;
use super::holder::Held;
use super::route;
use super::upstream;

/// Relays one request the route table admits.
/// A route whose entry declares a write is refused as `Refusal::ReadOnly` while
/// the instance is read-only, before the request body is read and before the
/// Jellyfin server is reached.
/// A route whose entry declares `Stage::Setup` is refused as
/// `Refusal::SetupFinished` once this run posted `Startup/Complete`.
/// A route asked for outside the stage its entry declares is refused as
/// `Refusal::NotInStage` naming that stage, before the request body is read and
/// before the Jellyfin server is reached.
/// A route declaring `Carried::None` forwards no request body; one declaring
/// `Carried::Capped` reads the body up to the cap its own entry declares and
/// refuses a larger one as `Refusal::BodyTooLarge` carrying the length the
/// request declared and that cap; one declaring `Carried::Encoded` reads it
/// under the same rule and forwards it base64-encoded.
pub async fn relay(state: State<Arc<AppState>>, request: Request) -> Response {
    let raw = request
        .uri()
        .path()
        .strip_prefix(jellium_protocol::RELAY_PREFIX)
        .unwrap_or_default();
    let Some(target) = route::Target::admit(request.method(), raw, &state.seen) else {
        return (StatusCode::FORBIDDEN, Json(Refusal::NotRelayed)).into_response();
    };

    if state.read_only && !target.read_only() {
        return (StatusCode::FORBIDDEN, Json(Refusal::ReadOnly)).into_response();
    }

    if target.stage() == route::Stage::Setup
        && state.completed.load(std::sync::atomic::Ordering::SeqCst)
    {
        return (StatusCode::FORBIDDEN, Json(Refusal::SetupFinished)).into_response();
    }

    let held = state.session.held().await;

    if let Some(held) = &held
        && !target.stage().admits(held.admits())
    {
        let admits = target
            .stage()
            .only()
            .unwrap_or(jellium_protocol::Admits::Signed);
        return (StatusCode::FORBIDDEN, Json(Refusal::NotInStage { admits })).into_response();
    }

    let declared = request
        .headers()
        .get(axum::http::header::CONTENT_LENGTH)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<u64>().ok());

    let (parts, body) = request.into_parts();
    let (cap, encode) = match target.body() {
        route::Carried::None => (None, false),
        route::Carried::Capped { cap } => (Some(cap), false),
        route::Carried::Encoded { cap } => (Some(cap), true),
    };
    let carried = match cap {
        None => axum::body::Bytes::new(),
        Some(cap) => match axum::body::to_bytes(body, cap).await {
            Ok(bytes) if encode => {
                use base64::Engine;
                axum::body::Bytes::from(base64::engine::general_purpose::STANDARD.encode(&bytes))
            }
            Ok(bytes) => bytes,
            Err(_) => {
                return (
                    StatusCode::PAYLOAD_TOO_LARGE,
                    Json(Refusal::BodyTooLarge {
                        bytes: declared,
                        cap,
                    }),
                )
                    .into_response();
            }
        },
    };

    let Some(held) = held else {
        return (StatusCode::CONFLICT, Json(Refusal::NoSession)).into_response();
    };

    let outcome = held
        .link()
        .forward(
            &target,
            parts.uri.query(),
            &parts.headers,
            carried,
            &state.seen,
        )
        .await;

    match outcome {
        Ok(response) => response,
        Err(failure) => {
            if failure == Failure::TokenRejected
                && let Held::Signed(upstream) = &held
            {
                state.session.reject(upstream).await;
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
        routed(AppState::stub(scratch(name)))
    }

    /// The relay under an instance started `--read-only`.
    fn read_only(name: &str) -> Router {
        let mut state = AppState::stub(scratch(name));
        state.read_only = true;
        routed(state)
    }

    fn routed(state: AppState) -> Router {
        let state = Arc::new(state);
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

    async fn posted(router: Router, uri: &str, body: Vec<u8>) -> StatusCode {
        router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(uri)
                    .body(Body::from(body))
                    .expect("a request"),
            )
            .await
            .expect("a response")
            .status()
    }

    #[tokio::test]
    async fn a_body_over_the_cap_is_refused_and_reaches_no_server() {
        assert_eq!(
            posted(
                router("body-over-cap"),
                "/jellyfin/LiveTv/Programs",
                vec![b'x'; route::BODY_LIMIT + 1],
            )
            .await,
            StatusCode::PAYLOAD_TOO_LARGE
        );
    }

    #[tokio::test]
    async fn a_body_under_the_cap_reaches_the_gate() {
        assert_eq!(
            posted(
                router("body-under-cap"),
                "/jellyfin/LiveTv/Programs",
                vec![b'x'; 128],
            )
            .await,
            StatusCode::CONFLICT
        );
    }

    /// The user image route declares a cap of its own, so a body the old cap
    /// forecloses still reaches the gate and one over the image cap does not.
    #[tokio::test]
    async fn the_user_image_route_caps_at_its_own_declared_limit() {
        let image = "/jellyfin/Users/0191b2f0-1c3d-4e5f-8a9b-0c1d2e3f4a5b/Images/Primary";
        assert_eq!(
            posted(
                router("image-over-old-cap"),
                image,
                vec![b'x'; route::BODY_LIMIT + 1],
            )
            .await,
            StatusCode::CONFLICT
        );
        assert_eq!(
            posted(
                router("image-over-image-cap"),
                image,
                vec![b'x'; route::IMAGE_LIMIT + 1],
            )
            .await,
            StatusCode::PAYLOAD_TOO_LARGE
        );
    }

    /// Every route that carried a body before this milestone still caps at the
    /// old limit.
    #[tokio::test]
    async fn the_preference_bag_still_caps_at_the_old_limit() {
        assert_eq!(
            posted(
                router("bag-over-cap"),
                "/jellyfin/DisplayPreferences/usersettings",
                vec![b'x'; route::BODY_LIMIT + 1],
            )
            .await,
            StatusCode::PAYLOAD_TOO_LARGE
        );
    }

    async fn deleted(router: Router, uri: &str) -> StatusCode {
        router
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri(uri)
                    .body(Body::empty())
                    .expect("a request"),
            )
            .await
            .expect("a response")
            .status()
    }

    /// A route whose entry declares a write is refused before the body is read
    /// and before the Jellyfin server is reached, so it never answers
    /// `CONFLICT` for the missing session.
    #[tokio::test]
    async fn a_write_is_refused_while_the_instance_is_read_only() {
        assert_eq!(
            deleted(
                read_only("read-only-write"),
                "/jellyfin/Users/0191b2f0-1c3d-4e5f-8a9b-0c1d2e3f4a5b"
            )
            .await,
            StatusCode::FORBIDDEN
        );
        assert_eq!(
            posted(
                read_only("read-only-body"),
                "/jellyfin/System/Configuration/encoding",
                vec![b'x'; route::BODY_LIMIT + 1],
            )
            .await,
            StatusCode::FORBIDDEN
        );
    }

    /// A read reaches the gate under read-only, so the reading half is offered
    /// whole.
    #[tokio::test]
    async fn a_read_still_reaches_the_gate_while_the_instance_is_read_only() {
        assert_eq!(
            status(read_only("read-only-read"), "/jellyfin/System/Info").await,
            StatusCode::CONFLICT
        );
        assert_eq!(
            posted(
                read_only("read-only-posted-read"),
                "/jellyfin/LiveTv/Programs",
                vec![b'x'; 128],
            )
            .await,
            StatusCode::CONFLICT
        );
    }

    /// A route outside the table is refused whatever the mode.
    #[tokio::test]
    async fn a_route_outside_the_table_is_refused_under_read_only_too() {
        assert_eq!(
            status(
                read_only("read-only-unrelayed"),
                "/jellyfin/Sessions/Logout"
            )
            .await,
            StatusCode::FORBIDDEN
        );
    }

    /// An install names a package only because a listing carried it, so the
    /// route is refused until one has been read.
    #[tokio::test]
    async fn an_install_of_an_unobserved_package_is_not_relayed() {
        assert_eq!(
            posted(
                router("unobserved-package"),
                "/jellyfin/Packages/Installed/Package%200",
                Vec::new(),
            )
            .await,
            StatusCode::FORBIDDEN
        );
    }
}
