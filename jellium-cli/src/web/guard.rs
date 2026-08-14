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

pub async fn guard(State(state): State<Arc<AppState>>, request: Request, next: Next) -> Response {
    if is_entry(&request) {
        return next.run(request).await;
    }

    if let Some(presented) = request.headers().get(header::ORIGIN)
        && !state.origin.matches(presented.as_bytes())
    {
        return refuse(StatusCode::FORBIDDEN, Refusal::ForeignOrigin);
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
