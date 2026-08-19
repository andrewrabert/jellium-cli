/// An instant as a Jellyfin query string carries it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timestamp(chrono::DateTime<chrono::Utc>);

impl Timestamp {
    pub fn at(instant: chrono::DateTime<chrono::Utc>) -> Timestamp {
        Timestamp(instant)
    }
}

// writes RFC 3339 with milliseconds under a `Z` offset, which is the spelling
// ASP.NET model binding parses and the spelling the reference sends
impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_rfc3339_opts(chrono::SecondsFormat::Millis, true))
    }
}
