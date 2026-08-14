use jellium_protocol::{Failure, Refusal};

use crate::text::{self, Text};

#[derive(Debug, Clone)]
pub enum Trouble {
    /// jellium-cli refused the request itself.
    Refused(Refusal),
    /// jellium-cli could not be reached, or answered with something this client
    /// does not understand.
    Relay { detail: String },
    /// The Jellyfin server refused, or jellium-cli could not reach it.
    Upstream(Failure),
}

impl Trouble {
    /// True when jellium-cli no longer holds a Jellyfin session: a token the
    /// Jellyfin server rejected, or a relay holding none.
    pub fn session_lost(&self) -> bool {
        matches!(
            self,
            Trouble::Refused(Refusal::NoSession) | Trouble::Upstream(Failure::TokenRejected)
        )
    }

    pub fn message(&self) -> String {
        match self {
            Trouble::Refused(Refusal::NotThisBrowser) => {
                text::lookup(Text::FailureNotThisBrowser).to_string()
            }
            Trouble::Refused(Refusal::ForeignOrigin) => {
                text::lookup(Text::FailureForeignOrigin).to_string()
            }
            Trouble::Refused(Refusal::NoSession) => {
                text::lookup(Text::FailureNoSession).to_string()
            }
            Trouble::Refused(Refusal::NotRelayed) => {
                text::lookup(Text::FailureNotRelayed).to_string()
            }
            Trouble::Relay { detail } => text::format(Text::FailureRelay, &[detail]),
            Trouble::Upstream(Failure::ServerUnreachable { server, .. }) => {
                text::format(Text::FailureServerUnreachable, &[server])
            }
            Trouble::Upstream(Failure::CredentialsRejected) => {
                text::lookup(Text::FailureCredentialsRejected).to_string()
            }
            Trouble::Upstream(Failure::TokenRejected) => {
                text::lookup(Text::FailureTokenRejected).to_string()
            }
            Trouble::Upstream(Failure::ServerBelowMinimum {
                server_version,
                minimum_version,
            }) => text::format(
                Text::FailureServerBelowMinimum,
                &[server_version, minimum_version],
            ),
        }
    }
}

/// Reads a non-2xx answer: a `Refusal` body is `Refused`, a `Failure` body is
/// `Upstream`, anything else is `Relay` carrying the status.
pub async fn classify(response: reqwest::Response) -> Trouble {
    let status = response.status();
    match response.text().await {
        Ok(body) => classify_body(status, &body),
        Err(error) => Trouble::Relay {
            detail: error.to_string(),
        },
    }
}

/// The same classification over a body already read.
pub fn classify_body(status: reqwest::StatusCode, body: &str) -> Trouble {
    if let Ok(refusal) = serde_json::from_str::<Refusal>(body) {
        return Trouble::Refused(refusal);
    }
    if let Ok(failure) = serde_json::from_str::<Failure>(body) {
        return Trouble::Upstream(failure);
    }
    Trouble::Relay {
        detail: status.to_string(),
    }
}

impl From<jellyfin_api::error::Error> for Trouble {
    fn from(error: jellyfin_api::error::Error) -> Trouble {
        match &error {
            jellyfin_api::error::Error::Status { status, body, .. } => classify_body(*status, body),
            _ => Trouble::Relay {
                detail: error.to_string(),
            },
        }
    }
}

impl From<reqwest::Error> for Trouble {
    fn from(error: reqwest::Error) -> Trouble {
        Trouble::Relay {
            detail: error.to_string(),
        }
    }
}
