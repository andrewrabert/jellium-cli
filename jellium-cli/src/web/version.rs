use jellium_protocol::Failure;

use super::link::{Link, unreachable};

/// What one `/System/Info/Public` probe reports.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Probed {
    pub version: String,
    /// The name the server reported, empty when it reported none.
    pub name: String,
    /// True when the Jellyfin server has not completed its setup wizard.
    pub startup: bool,
}

/// Probes `server` with no credential, refuses a version below
/// `Version::MINIMUM` as `Failure::ServerBelowMinimum`, and reports whether the
/// server is in startup mode.
/// A server reporting no version is `Failure::ServerUnreachable`.
pub async fn probe(server: &str) -> Result<Probed, Failure> {
    let link = Link::tokenless(server)
        .ok_or_else(|| unreachable(server, "the server text is not an http url"))?;
    let info = link
        .control()
        .get_public_system_info()
        .await
        .map_err(|e| unreachable(link.server(), e))?;
    let version = info
        .version
        .ok_or_else(|| unreachable(link.server(), "the server did not report a version"))?;
    gate(&version)?;
    Ok(Probed {
        startup: !info.startup_wizard_completed.unwrap_or(true),
        name: info.server_name.unwrap_or_default(),
        version,
    })
}

/// The urls a typed server text is tried as, in order: the text itself when it
/// carries a scheme, and `https://` then `http://` when it carries none.
/// No path suffix and no port is guessed.
pub fn candidates(typed: &str) -> Vec<String> {
    let typed = typed.trim().trim_end_matches('/');
    if typed.is_empty() {
        return Vec::new();
    }
    if typed.contains("://") {
        return vec![typed.to_string()];
    }
    vec![format!("https://{typed}"), format!("http://{typed}")]
}

/// Probes each of [`candidates`] in order and answers the first that replies,
/// with the url that replied.
/// A version below `Version::MINIMUM` ends the walk rather than falling to the
/// next candidate.
/// The failure answered is the last candidate's.
pub async fn probe_typed(typed: &str) -> Result<(String, Probed), Failure> {
    let candidates = candidates(typed);
    let mut last = unreachable(typed, "the server text is empty");
    for candidate in candidates {
        match probe(&candidate).await {
            Ok(probed) => return Ok((candidate, probed)),
            Err(failure @ Failure::ServerBelowMinimum { .. }) => return Err(failure),
            Err(failure) => last = failure,
        }
    }
    Err(last)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version([u32; 4]);

impl Version {
    pub const MINIMUM: Version = Version([10, 10, 0, 0]);

    /// Reads the leading dotted digits and ignores any suffix, so `10.11.0-rc1`
    /// reads as `10.11.0.0`.
    pub fn parse(text: &str) -> Option<Version> {
        let text = text.trim();
        let digits = text
            .find(|c: char| !c.is_ascii_digit() && c != '.')
            .map_or(text, |end| &text[..end]);
        let digits = digits.trim_end_matches('.');

        let mut components = [0u32; 4];
        let mut count = 0;
        for part in digits.split('.') {
            if count == components.len() {
                return None;
            }
            components[count] = part.parse().ok()?;
            count += 1;
        }
        (count > 0).then_some(Version(components))
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [a, b, c, d] = self.0;
        write!(f, "{a}.{b}.{c}.{d}")
    }
}

pub fn gate(reported: &str) -> Result<Version, Failure> {
    let below = || Failure::ServerBelowMinimum {
        server_version: reported.to_string(),
        minimum_version: "10.10.0".to_string(),
    };
    let version = Version::parse(reported).ok_or_else(below)?;
    if version < Version::MINIMUM {
        return Err(below());
    }
    Ok(version)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_release_candidate_parses_as_its_release() {
        assert_eq!(Version::parse("10.11.0-rc1"), Some(Version([10, 11, 0, 0])));
    }

    #[test]
    fn a_version_without_leading_digits_is_refused() {
        assert_eq!(Version::parse("unstable"), None);
    }

    #[test]
    fn a_text_with_no_scheme_is_tried_as_https_then_http() {
        assert_eq!(
            candidates("host:8096"),
            vec![
                "https://host:8096".to_string(),
                "http://host:8096".to_string()
            ]
        );
    }

    #[test]
    fn a_text_carrying_a_scheme_is_tried_as_itself_alone() {
        assert_eq!(
            candidates("http://host:8096/"),
            vec!["http://host:8096".to_string()]
        );
    }

    #[test]
    fn a_server_below_the_minimum_is_gated() {
        assert_eq!(
            gate("10.9.9"),
            Err(Failure::ServerBelowMinimum {
                server_version: "10.9.9".to_string(),
                minimum_version: "10.10.0".to_string(),
            })
        );
        assert!(gate("10.11.0-rc1").is_ok());
    }
}
