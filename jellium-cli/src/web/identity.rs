use axum::http::HeaderValue;
use uuid::Uuid;

pub struct Device {
    client: &'static str,
    /// Non-empty printable ascii.
    name: String,
    id: Uuid,
    version: &'static str,
}

/// `raw` when it is non-empty and every character is printable ascii,
/// "browser" otherwise.
fn device_name(raw: &str) -> String {
    if !raw.is_empty() && raw.chars().all(|c| c.is_ascii_graphic() || c == ' ') {
        raw.to_string()
    } else {
        "browser".to_string()
    }
}

impl Device {
    pub fn new(id: Uuid) -> Device {
        let raw = hostname::get()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        Device {
            client: "Jellium Web",
            name: device_name(&raw),
            id,
            version: env!("CARGO_PKG_VERSION"),
        }
    }

    // the Authorization header carrying `token`, or None when `token` holds a
    // byte no header value admits (a control byte); the caller fails the
    // request rather than panicking on a token it did not choose
    pub fn authorization(&self, token: &str) -> Option<HeaderValue> {
        let escape = |value: &str| value.replace('\\', r"\\").replace('"', "\\\"");
        let value = format!(
            r#"MediaBrowser Client="{}", Device="{}", DeviceId="{}", Version="{}", Token="{}""#,
            escape(self.client),
            escape(&self.name),
            self.id,
            escape(self.version),
            escape(token),
        );
        HeaderValue::from_str(&value).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_ascii_hostname_is_the_device_name() {
        assert_eq!(device_name("my-host"), "my-host");
    }

    #[test]
    fn a_non_ascii_hostname_falls_back() {
        assert_eq!(device_name("café"), "browser");
    }

    #[test]
    fn a_device_named_from_a_non_ascii_hostname_builds_its_header() {
        let device = Device {
            client: "Jellium Web",
            name: device_name("café"),
            id: Uuid::nil(),
            version: "0",
        };
        assert!(device.authorization("token").is_some());
    }

    #[test]
    fn a_control_character_token_has_no_header() {
        let device = Device {
            client: "Jellium Web",
            name: device_name("my-host"),
            id: Uuid::nil(),
            version: "0",
        };
        assert!(device.authorization("a\nb").is_none());
        assert!(device.authorization("token").is_some());
    }

    #[test]
    fn an_empty_hostname_falls_back() {
        assert_eq!(device_name(""), "browser");
    }
}
