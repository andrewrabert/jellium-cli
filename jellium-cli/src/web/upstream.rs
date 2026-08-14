use std::time::Duration;

use axum::body::Body;
use axum::http::{HeaderMap, HeaderName, StatusCode, header};
use axum::response::Response;
use jellium_protocol::{Credentials, Failure};
use reqwest::header::AUTHORIZATION;
use uuid::Uuid;

use super::identity::Device;
use super::route;
use super::version;

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
const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);

/// Bounds a whole control call: the version probe, the sign-in, the revoke.
const CONTROL_TIMEOUT: Duration = Duration::from_secs(20);

pub struct Upstream {
    pub state: jellium_protocol::Session,
    token: String,
    user_id: Uuid,
    /// The Jellyfin server's url, parsed once, that relayed urls extend.
    base: reqwest::Url,
    /// Carries `CONNECT_TIMEOUT` and no total deadline.
    relay: reqwest::Client,
    /// Carries `CONNECT_TIMEOUT` and `CONTROL_TIMEOUT`.
    control: reqwest::Client,
}

fn unreachable(server: &str, detail: impl std::fmt::Display) -> Failure {
    Failure::ServerUnreachable {
        server: server.to_string(),
        detail: detail.to_string(),
    }
}

/// The server text as an absolute url; a text that is not one reads as
/// `Failure::ServerUnreachable`.
fn base(server: &str) -> Result<reqwest::Url, Failure> {
    let url = reqwest::Url::parse(server).map_err(|e| unreachable(server, e))?;
    match url.scheme() {
        "http" | "https" => Ok(url),
        other => Err(unreachable(
            server,
            format!("unsupported url scheme: {other}"),
        )),
    }
}

/// True when the Jellyfin server refused the token: a 401 or a 403.
fn forgotten(error: &jellyfin_api::error::Error) -> bool {
    matches!(
        error,
        jellyfin_api::error::Error::Status { status, .. }
            if *status == reqwest::StatusCode::UNAUTHORIZED
                || *status == reqwest::StatusCode::FORBIDDEN
    )
}

fn rejected(error: &jellyfin_api::error::Error, rejection: Failure, server: &str) -> Failure {
    if forgotten(error) {
        rejection
    } else {
        unreachable(server, error)
    }
}

fn builder(device: &Device, token: &str) -> Option<reqwest::ClientBuilder> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(AUTHORIZATION, device.authorization(token)?);
    Some(
        reqwest::Client::builder()
            .default_headers(headers)
            .connect_timeout(CONNECT_TIMEOUT),
    )
}

fn built(builder: reqwest::ClientBuilder) -> reqwest::Client {
    builder
        .build()
        .expect("failed to build the upstream HTTP client")
}

fn relay_client(device: &Device, token: &str) -> Option<reqwest::Client> {
    Some(built(builder(device, token)?))
}

fn control_client(device: &Device, token: &str) -> Option<reqwest::Client> {
    Some(built(builder(device, token)?.timeout(CONTROL_TIMEOUT)))
}

fn api(server: &str, http: reqwest::Client) -> jellyfin_api::Client {
    jellyfin_api::Client::new(server.trim_end_matches('/'), http)
}

async fn gate_server(server: &str, api: &jellyfin_api::Client) -> Result<String, Failure> {
    let info = api
        .get_public_system_info()
        .await
        .map_err(|e| unreachable(server, e))?;
    let reported = info
        .version
        .ok_or_else(|| unreachable(server, "the server did not report a version"))?;
    version::gate(&reported)?;
    Ok(reported)
}

impl Upstream {
    pub async fn login(device: &Device, credentials: &Credentials) -> Result<Upstream, Failure> {
        let server = credentials.server.trim_end_matches('/').to_string();
        let anonymous = api(
            &server,
            control_client(device, "")
                .ok_or_else(|| unreachable(&server, "the request client could not be built"))?,
        );
        let server_version = gate_server(&server, &anonymous).await?;

        let result = anonymous
            .authenticate_user_by_name(&jellyfin_api::types::AuthenticateUserByName {
                pw: Some(credentials.password.clone()),
                username: Some(credentials.username.clone()),
            })
            .await
            .map_err(|e| rejected(&e, Failure::CredentialsRejected, &server))?;

        let token = result
            .access_token
            .ok_or_else(|| unreachable(&server, "no access token in the auth response"))?;
        let user = result
            .user
            .ok_or_else(|| unreachable(&server, "no user in the auth response"))?;
        let user_id = user
            .id
            .ok_or_else(|| unreachable(&server, "no user id in the auth response"))?;

        let relay = relay_client(device, &token).ok_or_else(|| {
            unreachable(
                &server,
                "the server's access token cannot be sent in a header",
            )
        })?;
        let control = control_client(device, &token).ok_or_else(|| {
            unreachable(
                &server,
                "the server's access token cannot be sent in a header",
            )
        })?;

        Ok(Upstream {
            state: jellium_protocol::Session {
                server: server.clone(),
                user_id,
                user_name: user.name.unwrap_or_default(),
                server_version,
                snapshot_version: jellyfin_api::SNAPSHOT_VERSION.to_string(),
            },
            base: base(&server)?,
            relay,
            control,
            token,
            user_id,
        })
    }

    pub async fn resume(
        device: &Device,
        session: &crate::session::Session,
    ) -> Result<Upstream, Failure> {
        let server = session.server.trim_end_matches('/').to_string();
        let control = control_client(device, &session.token).ok_or(Failure::TokenRejected)?;
        let client = api(&server, control.clone());
        let server_version = gate_server(&server, &client).await?;

        let user = client
            .get_current_user()
            .await
            .map_err(|e| rejected(&e, Failure::TokenRejected, &server))?;
        let user_id = user.id.unwrap_or(session.user_id);

        let base = base(&server)?;
        Ok(Upstream {
            state: jellium_protocol::Session {
                server,
                user_id,
                user_name: user.name.unwrap_or_default(),
                server_version,
                snapshot_version: jellyfin_api::SNAPSHOT_VERSION.to_string(),
            },
            token: session.token.clone(),
            user_id,
            base,
            relay: relay_client(device, &session.token).ok_or(Failure::TokenRejected)?,
            control,
        })
    }

    pub fn session(&self) -> crate::session::Session {
        crate::session::Session {
            server: self.state.server.clone(),
            token: self.token.clone(),
            user_id: self.user_id,
        }
    }

    pub async fn logout(&self) -> Result<(), Failure> {
        match api(&self.state.server, self.control.clone())
            .report_session_ended()
            .await
        {
            Ok(()) => Ok(()),
            Err(error) if forgotten(&error) => Ok(()),
            Err(error) => Err(unreachable(&self.state.server, error)),
        }
    }

    pub async fn forward(
        &self,
        target: &route::Target,
        query: Option<&str>,
        headers: &HeaderMap,
        body: Body,
    ) -> Result<Response, Failure> {
        let url = target.url(&self.base, query);

        let mut request = self.relay.request(target.method(), url);
        for (name, value) in headers {
            if matches!(
                name,
                &header::HOST
                    | &header::COOKIE
                    | &header::AUTHORIZATION
                    | &header::ORIGIN
                    | &header::CONTENT_LENGTH
            ) {
                continue;
            }
            request = request.header(name, value);
        }

        let response = request
            .body(reqwest::Body::wrap_stream(body.into_data_stream()))
            .send()
            .await
            .map_err(|e| unreachable(&self.state.server, e))?;

        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(Failure::TokenRejected);
        }

        let mut builder = Response::builder().status(response.status());
        for name in COPIED_RESPONSE_HEADERS {
            if let Some(value) = response.headers().get(name) {
                builder = builder.header(name, value);
            }
        }
        builder
            .body(Body::from_stream(response.bytes_stream()))
            .map_err(|e| unreachable(&self.state.server, e))
    }
}

pub fn status_for(failure: &Failure) -> StatusCode {
    match failure {
        Failure::CredentialsRejected | Failure::TokenRejected => StatusCode::UNAUTHORIZED,
        Failure::ServerBelowMinimum { .. } => StatusCode::CONFLICT,
        Failure::ServerUnreachable { .. } => StatusCode::BAD_GATEWAY,
    }
}

#[cfg(test)]
pub struct Answering {
    /// `http://127.0.0.1:<port>`, the base the listener serves.
    pub base: String,
    /// Counted before each answer is written.
    pub requests: std::sync::Arc<std::sync::atomic::AtomicUsize>,
}

#[cfg(test)]
// binds a loopback listener answering every request with `status` and no body
pub async fn answering(status: u16) -> Answering {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    let requests = Arc::new(AtomicUsize::new(0));
    let counter = requests.clone();
    let code = StatusCode::from_u16(status).expect("a valid status");
    let app = axum::Router::new().fallback(move || {
        let counter = counter.clone();
        async move {
            counter.fetch_add(1, Ordering::SeqCst);
            code
        }
    });
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind a loopback listener");
    let port = listener.local_addr().expect("the bound address").port();
    tokio::spawn(async move {
        axum::serve(listener, app).await.expect("serve");
    });
    Answering {
        base: format!("http://127.0.0.1:{port}"),
        requests,
    }
}

#[cfg(test)]
impl Upstream {
    // an upstream naming `server` with a placeholder token and identity
    pub fn stub(server: &str) -> Upstream {
        let device = Device::new(Uuid::nil());
        let server = server.trim_end_matches('/').to_string();
        Upstream {
            state: jellium_protocol::Session {
                server: server.clone(),
                user_id: Uuid::nil(),
                user_name: String::new(),
                server_version: String::new(),
                snapshot_version: jellyfin_api::SNAPSHOT_VERSION.to_string(),
            },
            token: "token".to_string(),
            user_id: Uuid::nil(),
            base: base(&server).expect("the stub server is a url"),
            relay: relay_client(&device, "token").expect("the stub token is header-safe"),
            control: control_client(&device, "token").expect("the stub token is header-safe"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::Ordering;

    #[tokio::test]
    async fn a_logout_the_server_refuses_counts_as_revoked() {
        let server = answering(401).await;
        let upstream = Upstream::stub(&server.base);
        assert!(upstream.logout().await.is_ok());
        assert_eq!(server.requests.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn a_logout_a_server_does_not_accept_is_unreachable() {
        let server = answering(500).await;
        let upstream = Upstream::stub(&server.base);
        assert!(matches!(
            upstream.logout().await,
            Err(Failure::ServerUnreachable { .. })
        ));
    }

    #[tokio::test]
    async fn a_control_character_token_fails_resume_as_token_rejected() {
        let session = crate::session::Session {
            server: "http://127.0.0.1:1".to_string(),
            token: "a\nb".to_string(),
            user_id: Uuid::nil(),
        };
        assert!(matches!(
            Upstream::resume(&Device::new(Uuid::nil()), &session).await,
            Err(Failure::TokenRejected)
        ));
    }

    #[tokio::test]
    async fn a_server_text_that_is_not_a_url_is_unreachable() {
        assert!(matches!(
            base("not a url"),
            Err(Failure::ServerUnreachable { .. })
        ));
    }
}
