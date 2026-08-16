//! One link to a Jellyfin server: the base url relayed urls extend, the two
//! clients requests are issued over, and everything the relay does to a
//! response.

use std::time::Duration;

use axum::body::Body;
use axum::http::{HeaderMap, HeaderName, StatusCode, header};
use axum::response::Response;
use jellium_protocol::{Failure, Refusal};
use reqwest::header::AUTHORIZATION;

use super::foreign::redirection;
use super::identity::Identity;
use super::manifest;
use super::playback::pointed::Pointed;
use super::route;

const COPIED_RESPONSE_HEADERS: &[HeaderName] = &[
    header::CONTENT_TYPE,
    header::CONTENT_ENCODING,
    header::CONTENT_LENGTH,
    header::CONTENT_RANGE,
    header::ACCEPT_RANGES,
    header::ETAG,
    header::CACHE_CONTROL,
    header::LAST_MODIFIED,
    header::VARY,
];

/// Bounds establishing the connection to the Jellyfin server; a body already
/// streaming is unaffected.
pub const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);

/// Bounds a whole control call: the version probe, the sign-in, the revoke.
pub const CONTROL_TIMEOUT: Duration = Duration::from_secs(20);

pub fn unreachable(server: &str, detail: impl std::fmt::Display) -> Failure {
    Failure::ServerUnreachable {
        server: server.to_string(),
        detail: detail.to_string(),
    }
}

/// True when the Jellyfin server refused the token: a 401 or a 403.
pub fn forgotten(error: &jellyfin_api::error::Error) -> bool {
    matches!(
        error,
        jellyfin_api::error::Error::Status { status, .. }
            if *status == reqwest::StatusCode::UNAUTHORIZED
                || *status == reqwest::StatusCode::FORBIDDEN
    )
}

/// The status the Jellyfin server answered with, and `None` for an error that
/// never reached one.
pub fn status_of(error: &jellyfin_api::error::Error) -> Option<u16> {
    match error {
        jellyfin_api::error::Error::Status { status, .. } => Some(status.as_u16()),
        _ => None,
    }
}

pub fn rejected(error: &jellyfin_api::error::Error, rejection: Failure, server: &str) -> Failure {
    if forgotten(error) {
        rejection
    } else {
        unreachable(server, error)
    }
}

fn built(builder: reqwest::ClientBuilder) -> reqwest::Client {
    builder
        .build()
        .expect("failed to build the upstream HTTP client")
}

/// The server text as an absolute http url; a text that is not one is `None`.
fn parsed(server: &str) -> Option<reqwest::Url> {
    let url = reqwest::Url::parse(server).ok()?;
    matches!(url.scheme(), "http" | "https").then_some(url)
}

pub struct Link {
    /// The server text, trimmed of its trailing slash, that every failure names.
    server: String,
    /// The Jellyfin server's url, parsed once, that relayed urls extend.
    base: reqwest::Url,
    /// Carries `CONNECT_TIMEOUT` and no total deadline.
    streaming: reqwest::Client,
    /// Carries `CONNECT_TIMEOUT` and `CONTROL_TIMEOUT`.
    control: reqwest::Client,
}

impl Link {
    /// A link whose every request carries `identity`'s authorization for `token`.
    /// `None` when `server` is not an http url or the token cannot be sent in a
    /// header.
    pub fn signed(identity: &Identity, server: &str, token: &str) -> Option<Link> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(AUTHORIZATION, identity.authorization(token)?);
        Link::of(server, headers)
    }

    /// A link presenting `identity` with an empty token, which is what
    /// `/Users/AuthenticateByName` is asked over: Jellyfin keys the session it
    /// mints, its device-list entry and its transcode cleanup by the device
    /// this header names.
    /// `None` when `server` is not an http url or the identity cannot be sent
    /// in a header.
    pub fn identified(identity: &Identity, server: &str) -> Option<Link> {
        Link::signed(identity, server, "")
    }

    /// A link carrying no `Authorization` header at all, which is what every
    /// setup-stage request is issued over.
    /// `None` when `server` is not an http url.
    pub fn tokenless(server: &str) -> Option<Link> {
        Link::of(server, reqwest::header::HeaderMap::new())
    }

    fn of(server: &str, headers: reqwest::header::HeaderMap) -> Option<Link> {
        let server = server.trim_end_matches('/').to_string();
        let base = parsed(&server)?;
        let builder = || {
            reqwest::Client::builder()
                .default_headers(headers.clone())
                .connect_timeout(CONNECT_TIMEOUT)
        };
        Some(Link {
            server,
            base,
            streaming: built(builder().redirect(reqwest::redirect::Policy::none())),
            control: built(builder().timeout(CONTROL_TIMEOUT)),
        })
    }

    pub fn server(&self) -> &str {
        &self.server
    }

    /// The Jellyfin server's url that relayed urls extend.
    pub fn base(&self) -> &reqwest::Url {
        &self.base
    }

    /// The Jellyfin client bound to this link, carrying `CONTROL_TIMEOUT`.
    pub fn control(&self) -> jellyfin_api::Client {
        jellyfin_api::Client::new(&self.server, self.control.clone())
    }

    /// The Jellyfin client bound to this link with no total deadline, which is
    /// what a streamed body is read over.
    pub fn streaming(&self) -> jellyfin_api::Client {
        jellyfin_api::Client::new(&self.server, self.streaming.clone())
    }

    /// The HTTP client every streamed request is issued over, carrying
    /// `CONNECT_TIMEOUT`, no total deadline, and no redirect policy of its
    /// own, so the relay decides each hop.
    pub fn transport(&self) -> &reqwest::Client {
        &self.streaming
    }

    /// A 401 or 403 reads as `rejection`; anything else reads as
    /// `Failure::ServerUnreachable`.
    pub fn failed(&self, error: jellyfin_api::error::Error, rejection: Failure) -> Failure {
        rejected(&error, rejection, &self.server)
    }

    fn unreachable(&self, detail: impl std::fmt::Display) -> Failure {
        unreachable(&self.server, detail)
    }

    /// Asks the Jellyfin server for undecoded bytes on a `Payload::Manifest`
    /// route, whose body the relay reads, and forwards the browser's
    /// `Accept-Encoding` on every other route.
    /// The browser's `Authorization` header is never forwarded.
    /// A manifest body is read up to `manifest::LIMIT` and rewritten; every
    /// other body is streamed in the encoding it arrives in.
    /// A manifest content type on a `Payload::Streamed` route is refused, and
    /// so is a manifest body that arrives content-encoded.
    /// A url the manifest names outside the Jellyfin server is minted into
    /// `pointed`, the register the plan this forward serves carries, and so is
    /// the `Location` a redirect answers, which the client follows no hop of.
    pub async fn forward(
        &self,
        target: &route::Target,
        query: Option<&str>,
        headers: &HeaderMap,
        body: axum::body::Bytes,
        seen: &route::Seen,
        pointed: &Pointed,
    ) -> Result<Response, Failure> {
        let query = match resolved_query(target, query, seen) {
            Ok(query) => query,
            Err(refusal) => return Ok(refused(StatusCode::FORBIDDEN, refusal)),
        };
        let body = match resolved_body(target, body, seen) {
            Ok(body) => body,
            Err(refusal) => return Ok(refused(StatusCode::FORBIDDEN, refusal)),
        };
        let url = target.url(&self.base, query.as_deref());

        let undecoded = matches!(
            target.payload(),
            route::Payload::Manifest | route::Payload::Foreign
        );
        let rewritable = target.payload() == route::Payload::Manifest;
        let mut request = self.streaming.request(target.method(), url.clone());
        if undecoded {
            request = request.header(header::ACCEPT_ENCODING, "identity");
        }
        for (name, value) in headers {
            if matches!(
                name,
                &header::HOST
                    | &header::COOKIE
                    | &header::AUTHORIZATION
                    | &header::ORIGIN
                    | &header::CONTENT_LENGTH
            ) || (undecoded && *name == header::ACCEPT_ENCODING)
            {
                continue;
            }
            request = request.header(name, value);
        }

        let response = request
            .body(body)
            .send()
            .await
            .map_err(|e| self.unreachable(e))?;

        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(Failure::TokenRejected);
        }

        if let Some(location) = redirection(&response) {
            return Ok(self.hop(response.status(), &url, &location, pointed));
        }

        if manifest::is_manifest(response.headers().get(header::CONTENT_TYPE)) {
            if !rewritable {
                return Ok(refused(
                    StatusCode::FORBIDDEN,
                    Refusal::ManifestNotRewritable,
                ));
            }
            return self.rewritten(response, &url, seen, pointed).await;
        }

        match target.payload() {
            route::Payload::Observed(observed) => {
                return self.observed(response, observed, seen).await;
            }
            route::Payload::Tail => return self.tailed(response).await,
            route::Payload::Foreign => return self.foreign(response, seen).await,
            route::Payload::Manifest | route::Payload::Streamed => {}
        }

        let mut builder = Response::builder().status(response.status());
        for name in COPIED_RESPONSE_HEADERS {
            if let Some(value) = response.headers().get(name) {
                builder = builder.header(name, value);
            }
        }
        builder
            .body(Body::from_stream(response.bytes_stream()))
            .map_err(|e| self.unreachable(e))
    }

    /// The hop `location` names, admitted the way any foreign origin is: the
    /// absolute url is minted into `pointed` and the browser is answered the
    /// same-origin path that handle is served at, which the relay fetches over
    /// a client holding no credential.
    /// A `Location` that is not an http url, and one no url can be resolved
    /// from, is refused rather than followed.
    fn hop(
        &self,
        status: reqwest::StatusCode,
        from: &reqwest::Url,
        location: &str,
        pointed: &Pointed,
    ) -> Response {
        let Ok(hop) = from.join(location) else {
            return refused(StatusCode::BAD_GATEWAY, Refusal::ForeignNotObserved);
        };
        if !matches!(hop.scheme(), "http" | "https") {
            return refused(StatusCode::FORBIDDEN, Refusal::ForeignNotObserved);
        }
        let handle = pointed.mint(hop.as_str());
        let status = StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::FOUND);
        Response::builder()
            .status(status)
            .header(header::LOCATION, super::playback::pointed::path(&handle))
            .body(Body::empty())
            .unwrap_or_else(|_| refused(StatusCode::BAD_GATEWAY, Refusal::ForeignNotObserved))
    }

    /// Reads the answer whole up to `route::OBSERVED_LIMIT`, mints a handle for
    /// every foreign image url it carries, and forwards the rewritten bytes, so
    /// no provider url reaches the browser.
    /// A body over the limit is refused rather than forwarded, because
    /// forwarding it unread would hand the browser the urls it carries.
    async fn foreign(
        &self,
        mut response: reqwest::Response,
        seen: &route::Seen,
    ) -> Result<Response, Failure> {
        let status = response.status();
        let content_type = response.headers().get(header::CONTENT_TYPE).cloned();

        let mut body = Vec::new();
        while let Some(chunk) = response.chunk().await.map_err(|e| self.unreachable(e))? {
            if body.len() + chunk.len() > route::OBSERVED_LIMIT {
                return Ok(refused(
                    StatusCode::BAD_GATEWAY,
                    Refusal::ForeignNotObserved,
                ));
            }
            body.extend_from_slice(&chunk);
        }

        let Ok(text) = String::from_utf8(body) else {
            return Ok(refused(
                StatusCode::BAD_GATEWAY,
                Refusal::ForeignNotObserved,
            ));
        };

        let mut builder = Response::builder().status(status);
        if let Some(content_type) = content_type {
            builder = builder.header(header::CONTENT_TYPE, content_type);
        }
        builder
            .body(Body::from(seen.foreign(&text)))
            .map_err(|e| self.unreachable(e))
    }

    /// Reads the listing whole up to `route::OBSERVED_LIMIT`, records every
    /// name it carries under `observed`, and forwards the bytes unchanged.
    /// A body over the limit is forwarded unread, so no name it carries is
    /// admitted later.
    async fn observed(
        &self,
        response: reqwest::Response,
        observed: route::Observed,
        seen: &route::Seen,
    ) -> Result<Response, Failure> {
        let status = response.status();
        let content_type = response.headers().get(header::CONTENT_TYPE).cloned();
        let body = response.bytes().await.map_err(|e| self.unreachable(e))?;
        if body.len() <= route::OBSERVED_LIMIT
            && let Ok(text) = std::str::from_utf8(&body)
        {
            seen.record(observed, text);
        }
        let mut builder = Response::builder().status(status);
        if let Some(content_type) = content_type {
            builder = builder.header(header::CONTENT_TYPE, content_type);
        }
        builder
            .body(Body::from(body))
            .map_err(|e| self.unreachable(e))
    }

    /// Delivers the last `route::TAIL_LIMIT` bytes of the body, answered `206`
    /// with the full length in `Content-Range` when the body is longer, and
    /// forwarded whole otherwise.
    async fn tailed(&self, mut response: reqwest::Response) -> Result<Response, Failure> {
        let status = response.status();
        let content_type = response.headers().get(header::CONTENT_TYPE).cloned();

        let mut held: std::collections::VecDeque<u8> = std::collections::VecDeque::new();
        let mut size: u64 = 0;
        while let Some(chunk) = response.chunk().await.map_err(|e| self.unreachable(e))? {
            size += chunk.len() as u64;
            held.extend(chunk.iter().copied());
            while held.len() > route::TAIL_LIMIT {
                held.pop_front();
            }
        }
        let body: Vec<u8> = held.into();

        let mut builder = Response::builder();
        if (body.len() as u64) < size {
            let from = size - body.len() as u64;
            builder = builder.status(StatusCode::PARTIAL_CONTENT).header(
                header::CONTENT_RANGE,
                format!("bytes {}-{}/{}", from, size.saturating_sub(1), size),
            );
        } else {
            builder = builder.status(status);
        }
        if let Some(content_type) = content_type {
            builder = builder.header(header::CONTENT_TYPE, content_type);
        }
        builder
            .body(Body::from(body))
            .map_err(|e| self.unreachable(e))
    }

    async fn rewritten(
        &self,
        mut response: reqwest::Response,
        source: &reqwest::Url,
        seen: &route::Seen,
        pointed: &Pointed,
    ) -> Result<Response, Failure> {
        if response
            .headers()
            .get(header::CONTENT_ENCODING)
            .is_some_and(|encoding| !encoding.as_bytes().eq_ignore_ascii_case(b"identity"))
        {
            return Ok(refused(
                StatusCode::FORBIDDEN,
                Refusal::ManifestNotRewritable,
            ));
        }

        let status = response.status();
        let content_type = response.headers().get(header::CONTENT_TYPE).cloned();

        let mut body = Vec::new();
        while let Some(chunk) = response.chunk().await.map_err(|e| self.unreachable(e))? {
            if body.len() + chunk.len() > manifest::LIMIT {
                return Ok(refused(StatusCode::BAD_GATEWAY, Refusal::ManifestTooLarge));
            }
            body.extend_from_slice(&chunk);
        }

        let Ok(text) = String::from_utf8(body) else {
            return Ok(refused(
                StatusCode::FORBIDDEN,
                Refusal::ManifestNotRewritable,
            ));
        };

        match manifest::rewrite(&text, source, &self.base, seen, pointed) {
            Ok(rewritten) => {
                let mut builder = Response::builder().status(status);
                if let Some(content_type) = content_type {
                    builder = builder.header(header::CONTENT_TYPE, content_type);
                }
                builder
                    .body(Body::from(rewritten))
                    .map_err(|e| self.unreachable(e))
            }
            Err(Refusal::ManifestTooLarge) => {
                Ok(refused(StatusCode::BAD_GATEWAY, Refusal::ManifestTooLarge))
            }
            Err(refusal) => Ok(refused(StatusCode::FORBIDDEN, refusal)),
        }
    }
}

/// `query` with an `imageUrl` handle swapped for the url `seen` observed; a
/// handle this run did not mint refuses the request before the Jellyfin server
/// is reached.
fn resolved_query(
    target: &route::Target,
    query: Option<&str>,
    seen: &route::Seen,
) -> Result<Option<String>, Refusal> {
    if target.resolves() != route::Resolves::Query {
        return Ok(query.map(str::to_owned));
    }
    let Some(query) = query else {
        return Ok(None);
    };
    let Ok(read) = reqwest::Url::parse(&format!("http://relay/?{query}")) else {
        return Ok(Some(query.to_owned()));
    };
    let mut held: Vec<(String, String)> = Vec::new();
    for (name, value) in read.query_pairs() {
        if name == "imageUrl" {
            let url = seen
                .observed(value.as_ref())
                .ok_or(Refusal::ForeignNotObserved)?;
            held.push((name.into_owned(), url));
        } else {
            held.push((name.into_owned(), value.into_owned()));
        }
    }

    let mut written = reqwest::Url::parse("http://relay/").expect("a valid base");
    written.query_pairs_mut().extend_pairs(held);
    Ok(written.query().map(str::to_owned))
}

/// `body` with its `ImageUrl` handle swapped for the url `seen` observed.
fn resolved_body(
    target: &route::Target,
    body: axum::body::Bytes,
    seen: &route::Seen,
) -> Result<axum::body::Bytes, Refusal> {
    if target.resolves() != route::Resolves::Body || body.is_empty() {
        return Ok(body);
    }
    let Ok(mut value) = serde_json::from_slice::<serde_json::Value>(&body) else {
        return Ok(body);
    };
    let Some(held) = value.as_object_mut() else {
        return Ok(body);
    };
    let Some(carried) = held.get("ImageUrl").and_then(serde_json::Value::as_str) else {
        return Ok(body);
    };
    let url = seen.observed(carried).ok_or(Refusal::ForeignNotObserved)?;
    held.insert("ImageUrl".to_owned(), serde_json::Value::String(url));
    Ok(axum::body::Bytes::from(
        serde_json::to_vec(&value).map_err(|_| Refusal::ForeignNotObserved)?,
    ))
}

fn refused(status: StatusCode, refusal: Refusal) -> Response {
    use axum::response::IntoResponse;
    (status, axum::Json(refusal)).into_response()
}
