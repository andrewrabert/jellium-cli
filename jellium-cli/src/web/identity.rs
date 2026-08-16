use std::sync::Arc;

use axum::http::HeaderValue;

/// What the browser announced about itself, which every upstream request the
/// run issues is identified by.
pub struct Identity {
    announced: jellium_protocol::Identity,
}

impl Identity {
    pub fn of(announced: jellium_protocol::Identity) -> Identity {
        Identity { announced }
    }

    /// The identifier the static-stream and audio-universal query strings
    /// carry, and the one this installation's own session is recognized by.
    pub fn device_id(&self) -> &str {
        &self.announced.device_id
    }

    /// The `Authorization` header carrying `token`, in the order
    /// `setRequestHeaders` pushes its values; `None` when a field holds a byte
    /// no header value admits.
    // reference: set-request-headers — apiClient.js:166-195
    pub fn authorization(&self, token: &str) -> Option<HeaderValue> {
        let escape = |value: &str| value.replace('\\', r"\\").replace('"', "\\\"");
        let mut values = vec![format!(r#"Client="{}""#, escape(jellium_protocol::CLIENT))];
        if !self.announced.device.is_empty() {
            values.push(format!(r#"Device="{}""#, escape(&self.announced.device)));
        }
        if !self.announced.device_id.is_empty() {
            values.push(format!(
                r#"DeviceId="{}""#,
                escape(&self.announced.device_id)
            ));
        }
        values.push(format!(
            r#"Version="{}""#,
            escape(jellium_protocol::VERSION)
        ));
        if !token.is_empty() {
            values.push(format!(r#"Token="{}""#, escape(token)));
        }
        HeaderValue::from_str(&format!("MediaBrowser {}", values.join(", "))).ok()
    }
}

/// The identity the run holds, and nothing until a browser announces one.
pub struct Announced {
    held: tokio::sync::RwLock<Option<Arc<Identity>>>,
}

impl Announced {
    pub fn new() -> Announced {
        Announced {
            held: tokio::sync::RwLock::new(None),
        }
    }

    /// The identity held now, and `None` while no browser has announced one.
    pub async fn held(&self) -> Option<Arc<Identity>> {
        self.held.read().await.clone()
    }

    /// Installs `announced`, answering true when it displaced a different one,
    /// which is what obliges the held upstream's link to be rebuilt.
    pub async fn install(&self, announced: jellium_protocol::Identity) -> bool {
        let mut held = self.held.write().await;
        let displaced = held
            .as_ref()
            .is_some_and(|standing| standing.announced != announced);
        *held = Some(Arc::new(Identity::of(announced)));
        displaced
    }
}

#[cfg(test)]
impl Announced {
    // an identity already announced, which is the state a router test drives
    // the relay in
    pub(crate) fn announcing(announced: jellium_protocol::Identity) -> Announced {
        Announced {
            held: tokio::sync::RwLock::new(Some(Arc::new(Identity::of(announced)))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn identity(device: &str, device_id: &str) -> Identity {
        Identity::of(jellium_protocol::Identity {
            device: device.to_owned(),
            device_id: device_id.to_owned(),
        })
    }

    /// The five values `setRequestHeaders` pushes, in its order.
    #[test]
    fn the_header_carries_the_reference_value_order() {
        let identity = identity("Firefox", "abc1");
        let header = identity.authorization("token").expect("a header");
        assert_eq!(
            header.to_str().expect("ascii"),
            r#"MediaBrowser Client="Jellyfin Web", Device="Firefox", DeviceId="abc1", Version="10.11.11", Token="token""#
        );
    }

    /// An empty token pushes no `Token`, which is what a login-stage request
    /// presents.
    #[test]
    fn an_empty_token_carries_no_token_value() {
        let identity = identity("Firefox", "abc1");
        let header = identity.authorization("").expect("a header");
        assert_eq!(
            header.to_str().expect("ascii"),
            r#"MediaBrowser Client="Jellyfin Web", Device="Firefox", DeviceId="abc1", Version="10.11.11""#
        );
    }

    #[test]
    fn a_control_character_token_has_no_header() {
        let identity = identity("Firefox", "abc1");
        assert!(identity.authorization("a\nb").is_none());
    }

    #[tokio::test]
    async fn a_second_identity_naming_something_else_displaces_the_first() {
        let announced = Announced::new();
        assert!(announced.held().await.is_none());
        assert!(
            !announced
                .install(jellium_protocol::Identity {
                    device: "Firefox".to_owned(),
                    device_id: "abc1".to_owned(),
                })
                .await
        );
        assert!(
            !announced
                .install(jellium_protocol::Identity {
                    device: "Firefox".to_owned(),
                    device_id: "abc1".to_owned(),
                })
                .await
        );
        assert!(
            announced
                .install(jellium_protocol::Identity {
                    device: "Chrome".to_owned(),
                    device_id: "def1".to_owned(),
                })
                .await
        );
        assert_eq!(
            announced.held().await.expect("an identity").device_id(),
            "def1"
        );
    }
}
