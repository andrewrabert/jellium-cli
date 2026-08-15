use std::collections::HashMap;
use std::sync::Arc;

use axum::Json;
use axum::extract::{Query, Request, State};
use axum::http::{HeaderValue, StatusCode, header};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use base64::Engine as _;
use jellium_protocol::Refusal;
use rand::TryRng as _;
use subtle::ConstantTimeEq as _;

use super::AppState;

const ENCODING: base64::engine::general_purpose::GeneralPurpose =
    base64::engine::general_purpose::URL_SAFE_NO_PAD;

fn mint_bytes() -> [u8; 32] {
    let mut bytes = [0u8; 32];
    rand::rngs::SysRng
        .try_fill_bytes(&mut bytes)
        .expect("the operating system could not supply randomness");
    bytes
}

/// One unguessable base64url token, which is what a configuration page's grant
/// is.
pub fn opaque() -> String {
    ENCODING.encode(mint_bytes())
}

fn matches(expected: &[u8; 32], presented: &str) -> bool {
    let Ok(decoded) = ENCODING.decode(presented) else {
        return false;
    };
    decoded.len() == expected.len() && bool::from(decoded.ct_eq(expected))
}

pub struct Secret {
    bytes: [u8; 32],
    spent: std::sync::atomic::AtomicBool,
}

impl Secret {
    /// The base64url form is handed back once, here, and never rendered again.
    pub fn mint() -> (Secret, String) {
        let bytes = mint_bytes();
        let rendered = ENCODING.encode(bytes);
        (
            Secret {
                bytes,
                spent: std::sync::atomic::AtomicBool::new(false),
            },
            rendered,
        )
    }

    /// Constant-time; the first match spends the secret, a mismatch does not,
    /// and every later presentation of the spent secret fails.
    pub fn redeem(&self, presented: &str) -> bool {
        matches(&self.bytes, presented)
            && !self.spent.swap(true, std::sync::atomic::Ordering::SeqCst)
    }
}

pub struct Cookie([u8; 32]);

impl Cookie {
    pub fn mint() -> Cookie {
        Cookie(mint_bytes())
    }

    pub fn matches(&self, presented: &str) -> bool {
        matches(&self.0, presented)
    }

    pub fn set_cookie_header(&self) -> HeaderValue {
        let value = format!(
            "{}={self}; Path=/; HttpOnly; SameSite=Strict",
            jellium_protocol::COOKIE_NAME
        );
        HeaderValue::from_str(&value).expect("cookie value is base64url")
    }
}

impl std::fmt::Display for Cookie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&ENCODING.encode(self.0))
    }
}

fn presented_cookie(request: &Request) -> Option<&str> {
    request
        .headers()
        .get(header::COOKIE)?
        .to_str()
        .ok()?
        .split(';')
        .filter_map(|pair| pair.trim().split_once('='))
        .find(|(name, _)| *name == jellium_protocol::COOKIE_NAME)
        .map(|(_, value)| value)
}

/// True for the one request that carries the per-run secret: the URL the
/// browser is opened with, which trades the secret for the session cookie.
pub fn is_entry(request: &Request) -> bool {
    request.uri().path() == "/"
        && request
            .uri()
            .query()
            .into_iter()
            .flat_map(|query| query.split('&'))
            .any(|pair| pair.split('=').next() == Some(jellium_protocol::SECRET_QUERY))
}

fn refuse(status: StatusCode, refusal: Refusal) -> Response {
    (status, Json(refusal)).into_response()
}

/// True for a request asking to become a websocket.
fn upgrading(request: &Request) -> bool {
    request
        .headers()
        .get(header::UPGRADE)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value.eq_ignore_ascii_case("websocket"))
}

/// The grant a path under `PAGE_PREFIX` carries, and `None` for any other
/// path.
fn granted(request: &Request) -> Option<&str> {
    let prefix = jellium_protocol::PAGE_PREFIX;
    let rest = request.uri().path().strip_prefix(prefix)?;
    let rest = rest.strip_prefix('/')?;
    let grant = rest.split('/').next()?;
    (!grant.is_empty()).then_some(grant)
}

/// An upgrade with no `Origin`, and any request whose `Origin` is not this
/// server's own, is refused as `ForeignOrigin`; a request without the session
/// cookie is refused as `NotThisBrowser`. Both are answered before the
/// upgrade.
/// A path under `PAGE_PREFIX` passes without the session cookie and with a
/// `null` origin when its first segment is a grant `Grants` holds, because the
/// frame that loads it has an opaque origin; every other path is refused as
/// before.
pub async fn guard(State(state): State<Arc<AppState>>, request: Request, next: Next) -> Response {
    if is_entry(&request) {
        return next.run(request).await;
    }

    if let Some(grant) = granted(&request) {
        if state.pages.holds(grant).await {
            return next.run(request).await;
        }
        return refuse(StatusCode::FORBIDDEN, Refusal::PageNotListed);
    }

    match request.headers().get(header::ORIGIN) {
        Some(presented) if !state.origin.matches(presented.as_bytes()) => {
            return refuse(StatusCode::FORBIDDEN, Refusal::ForeignOrigin);
        }
        None if upgrading(&request) => {
            return refuse(StatusCode::FORBIDDEN, Refusal::ForeignOrigin);
        }
        _ => {}
    }

    match presented_cookie(&request) {
        Some(presented) if state.cookie.matches(presented) => next.run(request).await,
        _ => refuse(StatusCode::UNAUTHORIZED, Refusal::NotThisBrowser),
    }
}

/// Spends the secret for the session cookie and redirects to `/`; a request
/// already carrying the session cookie redirects whether or not the secret is
/// still unspent.
pub async fn entry(State(state): State<Arc<AppState>>, request: Request) -> Response {
    let held = presented_cookie(&request).is_some_and(|presented| state.cookie.matches(presented));
    let redeemed = || {
        let Ok(Query(query)) = Query::<HashMap<String, String>>::try_from_uri(request.uri()) else {
            return false;
        };
        query
            .get(jellium_protocol::SECRET_QUERY)
            .is_some_and(|presented| state.secret.redeem(presented))
    };

    if !held && !redeemed() {
        return refuse(StatusCode::UNAUTHORIZED, Refusal::NotThisBrowser);
    }

    (
        StatusCode::SEE_OTHER,
        [
            (header::LOCATION, HeaderValue::from_static("/")),
            (header::SET_COOKIE, state.cookie.set_cookie_header()),
        ],
    )
        .into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::Router;
    use axum::body::Body;
    use axum::routing::get;
    use tower::ServiceExt as _;

    fn scratch(name: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir()
            .join("jellium-cli-guard-tests")
            .join(name);
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("scratch directory");
        path.join("session.env")
    }

    /// The guard in front of one route that would accept anything.
    fn guarded(name: &str) -> (Router, String) {
        let state = Arc::new(AppState::stub(scratch(name)));
        let cookie = format!("{}={}", jellium_protocol::COOKIE_NAME, state.cookie);
        let router = Router::new()
            .route(jellium_protocol::LIVE_PATH, get(|| async { "reached" }))
            .layer(axum::middleware::from_fn_with_state(state.clone(), guard))
            .with_state(state);
        (router, cookie)
    }

    /// A request shaped like the handshake a browser sends.
    fn upgrade() -> axum::http::request::Builder {
        Request::builder()
            .uri(jellium_protocol::LIVE_PATH)
            .header(header::CONNECTION, "Upgrade")
            .header(header::UPGRADE, "websocket")
            .header(header::SEC_WEBSOCKET_VERSION, "13")
            .header(header::SEC_WEBSOCKET_KEY, "dGhlIHNhbXBsZSBub25jZQ==")
    }

    async fn answered(router: Router, request: Request) -> (StatusCode, String) {
        let response = router.oneshot(request).await.expect("an answer");
        let status = response.status();
        let body = axum::body::to_bytes(response.into_body(), 4096)
            .await
            .expect("the body");
        (status, String::from_utf8_lossy(&body).into_owned())
    }

    #[tokio::test]
    async fn an_upgrade_without_an_origin_is_refused() {
        let (router, cookie) = guarded("upgrade-no-origin");
        let request = upgrade()
            .header(header::COOKIE, cookie)
            .body(Body::empty())
            .expect("a request");
        let (status, body) = answered(router, request).await;
        assert_eq!(status, StatusCode::FORBIDDEN);
        assert!(body.contains("foreignOrigin"), "{body}");
    }

    #[tokio::test]
    async fn an_upgrade_from_a_foreign_origin_is_refused() {
        let (router, cookie) = guarded("upgrade-foreign-origin");
        let request = upgrade()
            .header(header::ORIGIN, "http://evil.example")
            .header(header::COOKIE, cookie)
            .body(Body::empty())
            .expect("a request");
        let (status, body) = answered(router, request).await;
        assert_eq!(status, StatusCode::FORBIDDEN);
        assert!(body.contains("foreignOrigin"), "{body}");
    }

    #[tokio::test]
    async fn an_upgrade_without_the_session_cookie_is_refused() {
        let (router, _) = guarded("upgrade-no-cookie");
        let request = upgrade()
            .header(header::ORIGIN, "http://127.0.0.1:0")
            .body(Body::empty())
            .expect("a request");
        let (status, body) = answered(router, request).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert!(body.contains("notThisBrowser"), "{body}");
    }

    #[tokio::test]
    async fn an_ordinary_request_without_an_origin_still_passes() {
        let (router, cookie) = guarded("plain-no-origin");
        let request = Request::builder()
            .uri(jellium_protocol::LIVE_PATH)
            .header(header::COOKIE, cookie)
            .body(Body::empty())
            .expect("a request");
        let (status, _) = answered(router, request).await;
        assert_eq!(status, StatusCode::OK);
    }
}
