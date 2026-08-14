use std::net::IpAddr;

const SCHEME: &str = "http://";
const DEFAULT_PORT: u16 = 80;

/// A host a browser can use to reach this server, held in the form a browser
/// serializes: ascii-lowercased, an ipv6 literal bracketed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Host(String);

fn nameable(text: &str) -> bool {
    !text.is_empty()
        && text
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '.'))
}

impl Host {
    /// Accepts an ip address, bracketed or bare, and a name of ascii letters,
    /// digits, `-` and `.`; refuses an empty text, and one carrying any other
    /// character.
    pub fn parse(text: &str) -> Option<Host> {
        let bare = text
            .strip_prefix('[')
            .and_then(|rest| rest.strip_suffix(']'))
            .unwrap_or(text);
        if let Ok(address) = bare.parse::<IpAddr>() {
            return Some(Host::of(address));
        }
        nameable(text).then(|| Host(text.to_ascii_lowercase()))
    }

    pub fn of(address: IpAddr) -> Host {
        Host(match address {
            IpAddr::V4(address) => address.to_string(),
            IpAddr::V6(address) => format!("[{address}]"),
        })
    }

    pub fn origin(&self, port: u16) -> Origin {
        Origin {
            host: self.clone(),
            port,
        }
    }
}

/// The one origin this server answers for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Origin {
    host: Host,
    port: u16,
}

impl Origin {
    /// True when `presented` names this origin: the scheme http, the same host
    /// ignoring ascii case, and the same port, 80 when the presented origin
    /// elides it. An opaque origin, an https origin and any other scheme are
    /// false.
    pub fn matches(&self, presented: &[u8]) -> bool {
        let Ok(text) = str::from_utf8(presented) else {
            return false;
        };
        let Some(authority) = text.strip_prefix(SCHEME) else {
            return false;
        };
        let (host, port) = match authority.rsplit_once(':') {
            Some((host, port)) if !host.ends_with(':') && !host.is_empty() => {
                let Ok(port) = port.parse::<u16>() else {
                    return false;
                };
                (host, port)
            }
            _ => (authority, DEFAULT_PORT),
        };
        port == self.port && host.eq_ignore_ascii_case(&self.host.0)
    }
}

/// `http://<host>`, the port elided when it is 80, as a browser serializes it.
impl std::fmt::Display for Origin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(SCHEME)?;
        f.write_str(&self.host.0)?;
        if self.port != DEFAULT_PORT {
            write!(f, ":{}", self.port)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn host(text: &str) -> Host {
        Host::parse(text).expect("a well formed host")
    }

    #[test]
    fn a_mixed_case_host_is_held_lowercased() {
        assert_eq!(
            host("Media.LAN").origin(8096).to_string(),
            "http://media.lan:8096"
        );
    }

    #[test]
    fn a_browser_origin_matches_a_mixed_case_advertised_host() {
        assert!(
            host("Media.LAN")
                .origin(8096)
                .matches(b"http://media.lan:8096")
        );
    }

    #[test]
    fn a_browser_eliding_the_default_port_matches() {
        let origin = host("media.lan").origin(80);
        assert!(origin.matches(b"http://media.lan"));
        assert_eq!(origin.to_string(), "http://media.lan");
    }

    #[test]
    fn an_explicit_default_port_matches_an_elided_one() {
        assert!(host("media.lan").origin(80).matches(b"http://media.lan:80"));
    }

    #[test]
    fn another_port_is_a_foreign_origin() {
        assert!(
            !host("media.lan")
                .origin(80)
                .matches(b"http://media.lan:8096")
        );
    }

    #[test]
    fn an_https_origin_is_a_foreign_origin() {
        assert!(!host("media.lan").origin(80).matches(b"https://media.lan"));
    }

    #[test]
    fn an_opaque_origin_is_a_foreign_origin() {
        assert!(!host("media.lan").origin(80).matches(b"null"));
    }

    #[test]
    fn an_ipv6_origin_matches_its_bracketed_form() {
        let origin = host("::1").origin(8096);
        assert_eq!(origin.to_string(), "http://[::1]:8096");
        assert!(origin.matches(b"http://[::1]:8096"));
    }

    #[test]
    fn a_host_carrying_a_space_is_refused() {
        assert!(Host::parse("media lan").is_none());
        assert!(Host::parse("").is_none());
    }
}
