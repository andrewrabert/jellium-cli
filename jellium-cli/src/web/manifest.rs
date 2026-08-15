use axum::http::HeaderValue;
use jellium_protocol::Refusal;

use super::route::{self, Target};

/// The largest manifest body the relay buffers; a longer one is refused
/// rather than held.
pub const LIMIT: usize = 1 << 20;

/// The response content types whose bodies are rewritten rather than
/// streamed, matched without regard to ascii case.
pub const MANIFEST_TYPES: [&str; 2] = ["application/vnd.apple.mpegurl", "application/x-mpegurl"];

/// True when `content_type` names an HLS manifest.
pub fn is_manifest(content_type: Option<&HeaderValue>) -> bool {
    let Some(value) = content_type.and_then(|value| value.to_str().ok()) else {
        return false;
    };
    let essence = value.split(';').next().unwrap_or_default().trim();
    MANIFEST_TYPES
        .iter()
        .any(|manifest| essence.eq_ignore_ascii_case(manifest))
}

/// The relay path an upstream url maps to, or a refusal when the route table
/// does not admit it.
/// Urls outside `base` are refused.
pub fn relay_path(
    url: &reqwest::Url,
    base: &reqwest::Url,
    seen: &route::Seen,
) -> Result<String, Refusal> {
    if url.scheme() != base.scheme()
        || url.host_str() != base.host_str()
        || url.port_or_known_default() != base.port_or_known_default()
    {
        return Err(Refusal::ManifestNotRewritable);
    }
    let prefix = base.path().trim_end_matches('/');
    let rest = url
        .path()
        .strip_prefix(prefix)
        .ok_or(Refusal::ManifestNotRewritable)?;
    if !prefix.is_empty() && !rest.is_empty() && !rest.starts_with('/') {
        return Err(Refusal::ManifestNotRewritable);
    }
    Target::admit(&axum::http::Method::GET, rest, seen)
        .map(|target| target.path(url.query()))
        .ok_or(Refusal::ManifestNotRewritable)
}

/// The `URI="..."` value of an attribute list, as a byte range into `line`.
fn uri_value(line: &str) -> Option<std::ops::Range<usize>> {
    let mut rest = line;
    let mut at = 0;
    loop {
        let found = rest.find("URI=\"")?;
        let opens = at + found + "URI=\"".len();
        let closes = opens + line[opens..].find('"')?;
        let before = line[..at + found].chars().next_back();
        if before.is_none_or(|character| character == ',' || character == ':') {
            return Some(opens..closes);
        }
        at = closes + 1;
        rest = &line[at..];
    }
}

/// True when `line` is a tag whose attribute list carries a `URI`.
fn carries_uri(line: &str) -> bool {
    line.starts_with('#') && uri_value(line).is_some()
}

/// True when `reference` already begins with `prefix` at a segment boundary.
fn behind(reference: &str, prefix: &str) -> bool {
    reference.strip_prefix(prefix).is_some_and(|rest| {
        rest.is_empty() || rest.starts_with('/') || rest.starts_with('?') || rest.starts_with('#')
    })
}

/// The relay path a reference the Jellyfin server supplied maps to.
/// An absolute-path reference extends `base`'s own path, unless it already
/// begins with that path at a segment boundary; every other reference resolves
/// against `source`, the url the reference arrived with.
/// A url outside `base`, and one the route table does not admit, is refused.
pub fn resolved(
    reference: &str,
    source: &reqwest::Url,
    base: &reqwest::Url,
    seen: &route::Seen,
) -> Result<String, Refusal> {
    let prefix = base.path().trim_end_matches('/');
    let extends = reference.starts_with('/')
        && !reference.starts_with("//")
        && !prefix.is_empty()
        && !behind(reference, prefix);
    let url = if extends {
        base.join(&format!("{prefix}{reference}"))
    } else {
        source.join(reference)
    }
    .map_err(|_| Refusal::ManifestNotRewritable)?;
    relay_path(&url, base, seen)
}

/// True when no line of `body` names a url scheme or a parameter in
/// `route::STRIPPED`, which is what no byte served to the browser may carry.
fn tokenless(body: &str) -> bool {
    !body.lines().any(|line| {
        let lowered = line.to_ascii_lowercase();
        lowered.contains("://")
            || route::STRIPPED
                .iter()
                .any(|name| lowered.contains(&name.to_ascii_lowercase()))
    })
}

/// `body` with every url it names — bare uri lines, and the `URI` attribute of
/// every tag carrying one — replaced by the relay path it maps to.
/// Relative urls resolve against `source`, the upstream url the manifest came
/// from.
/// A url the route table refuses fails the whole rewrite, and so does a
/// rewritten body still naming a url scheme or a parameter in
/// `route::STRIPPED`.
pub fn rewrite(
    body: &str,
    source: &reqwest::Url,
    base: &reqwest::Url,
    seen: &route::Seen,
) -> Result<String, Refusal> {
    let mut out = String::with_capacity(body.len());
    for line in body.split_inclusive('\n') {
        let (text, ending) = match line.strip_suffix('\n') {
            Some(text) => (text.strip_suffix('\r').unwrap_or(text), &line[text.len()..]),
            None => (line, ""),
        };
        let trimmed = text.trim();
        if trimmed.is_empty() {
            out.push_str(line);
        } else if let Some(range) = carries_uri(trimmed).then(|| uri_value(text)).flatten() {
            out.push_str(&text[..range.start]);
            out.push_str(&resolved(&text[range.clone()], source, base, seen)?);
            out.push_str(&text[range.end..]);
            out.push_str(ending);
        } else if trimmed.starts_with('#') {
            out.push_str(line);
        } else {
            out.push_str(&resolved(trimmed, source, base, seen)?);
            out.push_str(ending);
        }
    }
    if !tokenless(&out) {
        return Err(Refusal::ManifestNotRewritable);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ID: &str = "0191b2f0-1c3d-4e5f-8a9b-0c1d2e3f4a5b";

    fn base() -> reqwest::Url {
        reqwest::Url::parse("https://example.test").expect("base")
    }

    fn source() -> reqwest::Url {
        reqwest::Url::parse(&format!(
            "https://example.test/Videos/{ID}/main.m3u8?api_key=secret"
        ))
        .expect("source")
    }

    #[test]
    fn a_manifest_content_type_is_recognized() {
        assert!(is_manifest(Some(&HeaderValue::from_static(
            "application/vnd.apple.mpegurl"
        ))));
        assert!(is_manifest(Some(&HeaderValue::from_static(
            "application/x-mpegURL; charset=utf-8"
        ))));
        assert!(!is_manifest(Some(&HeaderValue::from_static("video/mp4"))));
        assert!(!is_manifest(None));
    }

    #[test]
    fn a_relative_segment_becomes_a_same_origin_relay_path() {
        let rewritten = rewrite(
            "#EXTINF:6,\nhls1/main/0.ts?api_key=secret\n",
            &source(),
            &base(),
            &route::Seen::new(),
        )
        .expect("rewritten");
        assert_eq!(
            rewritten,
            format!("#EXTINF:6,\n/jellyfin/Videos/{ID}/hls1/main/0.ts\n")
        );
    }

    #[test]
    fn an_absolute_segment_becomes_a_same_origin_relay_path() {
        let body = format!("https://example.test/Videos/{ID}/hls1/main/0.ts?x=1\n");
        let rewritten = rewrite(&body, &source(), &base(), &route::Seen::new()).expect("rewritten");
        assert_eq!(
            rewritten,
            format!("/jellyfin/Videos/{ID}/hls1/main/0.ts?x=1\n")
        );
    }

    #[test]
    fn a_uri_attribute_is_rewritten_in_place() {
        let body = "#EXT-X-MAP:URI=\"hls1/main/0.mp4\",BYTERANGE=\"1@0\"\n".to_string();
        let rewritten = rewrite(&body, &source(), &base(), &route::Seen::new()).expect("rewritten");
        assert_eq!(
            rewritten,
            format!("#EXT-X-MAP:URI=\"/jellyfin/Videos/{ID}/hls1/main/0.mp4\",BYTERANGE=\"1@0\"\n")
        );
    }

    #[test]
    fn a_manifest_body_carries_no_token() {
        let body = "hls1/main/0.ts?api_key=secret&X-Emby-Token=secret\n".to_string();
        let rewritten = rewrite(&body, &source(), &base(), &route::Seen::new()).expect("rewritten");
        assert!(!rewritten.contains("secret"));
    }

    #[test]
    fn a_foreign_origin_fails_the_rewrite() {
        let body = "https://elsewhere.test/whatever.ts\n";
        assert_eq!(
            rewrite(body, &source(), &base(), &route::Seen::new()),
            Err(Refusal::ManifestNotRewritable)
        );
    }

    #[test]
    fn a_url_outside_the_route_table_fails_the_rewrite() {
        let body = "/QuickConnect/Initiate\n";
        assert_eq!(
            rewrite(body, &source(), &base(), &route::Seen::new()),
            Err(Refusal::ManifestNotRewritable)
        );
    }

    #[test]
    fn a_server_path_prefix_is_kept_out_of_the_relay_path() {
        let base = reqwest::Url::parse("https://example.test/jellyfin").expect("base");
        let source = reqwest::Url::parse(&format!(
            "https://example.test/jellyfin/Videos/{ID}/main.m3u8"
        ))
        .expect("source");
        let rewritten =
            rewrite("hls1/main/0.ts\n", &source, &base, &route::Seen::new()).expect("rewritten");
        assert_eq!(rewritten, format!("/jellyfin/Videos/{ID}/hls1/main/0.ts\n"));
    }

    #[test]
    fn an_absolute_reference_keeps_the_servers_path_prefix() {
        let base = reqwest::Url::parse("https://example.test/jellyfin").expect("base");
        let source = reqwest::Url::parse(&format!(
            "https://example.test/jellyfin/Videos/{ID}/main.m3u8"
        ))
        .expect("source");
        let body = format!("/Videos/{ID}/hls1/main/0.ts\n");
        let rewritten = rewrite(&body, &source, &base, &route::Seen::new()).expect("rewritten");
        assert_eq!(rewritten, format!("/jellyfin/Videos/{ID}/hls1/main/0.ts\n"));
    }

    #[test]
    fn an_absolute_reference_that_repeats_the_prefix_resolves_once() {
        let base = reqwest::Url::parse("https://example.test/jellyfin").expect("base");
        let source = reqwest::Url::parse(&format!(
            "https://example.test/jellyfin/Videos/{ID}/main.m3u8"
        ))
        .expect("source");
        let body = format!("/jellyfin/Videos/{ID}/hls1/main/0.ts\n");
        let rewritten = rewrite(&body, &source, &base, &route::Seen::new()).expect("rewritten");
        assert_eq!(rewritten, format!("/jellyfin/Videos/{ID}/hls1/main/0.ts\n"));
    }

    #[test]
    fn a_tag_the_rewriter_does_not_know_is_rewritten_when_it_carries_a_uri() {
        let body = "#EXT-X-PART:DURATION=1,URI=\"hls1/main/0.ts?api_key=secret\"\n".to_string();
        let rewritten = rewrite(&body, &source(), &base(), &route::Seen::new()).expect("rewritten");
        assert_eq!(
            rewritten,
            format!("#EXT-X-PART:DURATION=1,URI=\"/jellyfin/Videos/{ID}/hls1/main/0.ts\"\n")
        );
    }

    #[test]
    fn a_body_still_naming_an_upstream_url_fails_the_rewrite() {
        let body = "#EXT-X-SESSION-DATA:VALUE=\"https://example.test/Auth/Keys\"\n";
        assert_eq!(
            rewrite(body, &source(), &base(), &route::Seen::new()),
            Err(Refusal::ManifestNotRewritable)
        );
    }
}
