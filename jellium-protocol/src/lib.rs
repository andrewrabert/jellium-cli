use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "camelCase")]
pub enum SessionStatus {
    Anonymous,
    Authenticated(Session),
    Failed(Failure),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub server: String,
    pub user_id: Uuid,
    pub user_name: String,
    pub server_version: String,
    pub snapshot_version: String,
}

impl Session {
    pub fn off_snapshot(&self) -> bool {
        self.server_version != self.snapshot_version
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Credentials {
    pub server: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "failure", rename_all = "camelCase")]
pub enum Failure {
    #[serde(rename_all = "camelCase")]
    ServerUnreachable {
        server: String,
        detail: String,
    },
    CredentialsRejected,
    TokenRejected,
    #[serde(rename_all = "camelCase")]
    ServerBelowMinimum {
        server_version: String,
        minimum_version: String,
    },
}

/// A refusal the local server made itself, distinct from anything the Jellyfin
/// server said.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "refusal", rename_all = "camelCase")]
pub enum Refusal {
    /// The session cookie was absent or did not match.
    NotThisBrowser,
    /// The Origin header was not the local server's own.
    ForeignOrigin,
    /// The local server holds no Jellyfin session.
    NoSession,
    /// The method and path are not one of the Jellyfin routes the local
    /// server relays.
    NotRelayed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceIdentity;

pub const RELAY_PREFIX: &str = "/jellyfin";
pub const SESSION_PATH: &str = "/session";
pub const SECRET_QUERY: &str = "s";
pub const COOKIE_NAME: &str = "jellium_web";
