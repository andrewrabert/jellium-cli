use jellium_protocol::Failure;

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
