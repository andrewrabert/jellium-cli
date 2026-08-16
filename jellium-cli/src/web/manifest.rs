use axum::http::HeaderValue;
use jellium_protocol::Refusal;

use super::playback::pointed::{self, Pointed};
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
/// A url outside `base` is one the Jellyfin server has just pointed this plan
/// at, so it is minted into `pointed` and answered as a handle path; a url
/// carrying neither `http` nor `https` is refused rather than minted.
pub fn relay_path(
    url: &reqwest::Url,
    base: &reqwest::Url,
    seen: &route::Seen,
    pointed: &Pointed,
) -> Result<String, Refusal> {
    if url.scheme() != base.scheme()
        || url.host_str() != base.host_str()
        || url.port_or_known_default() != base.port_or_known_default()
    {
        if !matches!(url.scheme(), "http" | "https") {
            return Err(Refusal::ManifestNotRewritable);
        }
        return Ok(pointed::path(&pointed.mint(url.as_str())));
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

/// The byte range of the reference `line` carries: the `URI="..."` value of a
/// tag's attribute list, and the trimmed span of a bare uri line.
/// A blank line, and a tag whose attribute list carries no `URI`, carry none.
pub fn reference(line: &str) -> Option<std::ops::Range<usize>> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return None;
    }
    if trimmed.starts_with('#') {
        return uri_value(line);
    }
    let start = line.len() - line.trim_start().len();
    Some(start..start + trimmed.len())
}

/// Every reference `body` carries, each as the path under `RELAY_PREFIX` the
/// browser asks for next.
/// A reference that is not a relay path panics.
#[cfg(test)]
pub fn referenced(body: &str) -> Vec<&str> {
    body.lines()
        .filter_map(|line| {
            let carried = &line[reference(line)?];
            Some(
                carried
                    .strip_prefix(jellium_protocol::RELAY_PREFIX)
                    .unwrap_or_else(|| panic!("{carried} is a relay path")),
            )
        })
        .collect()
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
/// A url the route table does not admit is refused; a url outside `base` is
/// minted into `pointed`.
pub fn resolved(
    reference: &str,
    source: &reqwest::Url,
    base: &reqwest::Url,
    seen: &route::Seen,
    pointed: &Pointed,
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
    relay_path(&url, base, seen, pointed)
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
/// Only the reference is replaced: a line's own spacing, and its ending, reach
/// the browser as they arrived.
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
    pointed: &Pointed,
) -> Result<String, Refusal> {
    let mut out = String::with_capacity(body.len());
    for line in body.split_inclusive('\n') {
        let (text, ending) = match line.strip_suffix('\n') {
            Some(rest) => {
                let text = rest.strip_suffix('\r').unwrap_or(rest);
                (text, &line[text.len()..])
            }
            None => (line, ""),
        };
        match reference(text) {
            Some(range) => {
                out.push_str(&text[..range.start]);
                out.push_str(&resolved(
                    &text[range.clone()],
                    source,
                    base,
                    seen,
                    pointed,
                )?);
                out.push_str(&text[range.end..]);
                out.push_str(ending);
            }
            None => out.push_str(line),
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
            &Pointed::new(),
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
        let rewritten = rewrite(
            &body,
            &source(),
            &base(),
            &route::Seen::new(),
            &Pointed::new(),
        )
        .expect("rewritten");
        assert_eq!(
            rewritten,
            format!("/jellyfin/Videos/{ID}/hls1/main/0.ts?x=1\n")
        );
    }

    #[test]
    fn a_uri_attribute_is_rewritten_in_place() {
        let body = "#EXT-X-MAP:URI=\"hls1/main/0.mp4\",BYTERANGE=\"1@0\"\n".to_string();
        let rewritten = rewrite(
            &body,
            &source(),
            &base(),
            &route::Seen::new(),
            &Pointed::new(),
        )
        .expect("rewritten");
        assert_eq!(
            rewritten,
            format!("#EXT-X-MAP:URI=\"/jellyfin/Videos/{ID}/hls1/main/0.mp4\",BYTERANGE=\"1@0\"\n")
        );
    }

    #[test]
    fn a_variant_playlist_rewrites_its_initialization_segment_and_its_segments() {
        let body = concat!(
            "#EXT-X-MAP:URI=\"hls1/main/-1.mp4?api_key=secret\"\n",
            "#EXTINF:6,\n",
            "hls1/main/0.mp4?api_key=secret\n",
        );
        let rewritten = rewrite(
            body,
            &source(),
            &base(),
            &route::Seen::new(),
            &Pointed::new(),
        )
        .expect("rewritten");
        assert_eq!(
            rewritten,
            format!(
                concat!(
                    "#EXT-X-MAP:URI=\"/jellyfin/Videos/{id}/hls1/main/-1.mp4\"\n",
                    "#EXTINF:6,\n",
                    "/jellyfin/Videos/{id}/hls1/main/0.mp4\n",
                ),
                id = ID
            )
        );
    }

    #[test]
    fn every_reference_a_rewritten_variant_playlist_carries_is_admitted_again() {
        let body = concat!(
            "#EXT-X-MAP:URI=\"hls1/main/-1.mp4\"\n",
            "#EXTINF:6,\n",
            "hls1/main/0.mp4\n",
        );
        let rewritten = rewrite(
            body,
            &source(),
            &base(),
            &route::Seen::new(),
            &Pointed::new(),
        )
        .expect("rewritten");
        let paths = referenced(&rewritten);
        assert_eq!(
            paths,
            vec![
                format!("/Videos/{ID}/hls1/main/-1.mp4"),
                format!("/Videos/{ID}/hls1/main/0.mp4"),
            ]
        );
        for path in paths {
            assert!(
                route::Target::admit(&axum::http::Method::GET, path, &route::Seen::new()).is_some(),
                "GET {path} is admitted again"
            );
        }
    }

    #[test]
    fn a_manifest_body_carries_no_token() {
        let body = "hls1/main/0.ts?api_key=secret&X-Emby-Token=secret\n".to_string();
        let rewritten = rewrite(
            &body,
            &source(),
            &base(),
            &route::Seen::new(),
            &Pointed::new(),
        )
        .expect("rewritten");
        assert!(!rewritten.contains("secret"));
    }

    #[test]
    fn a_foreign_origin_becomes_a_handle_this_plan_holds() {
        let body = "https://elsewhere.test/whatever.ts\n";
        let pointed = Pointed::new();
        let rewritten =
            rewrite(body, &source(), &base(), &route::Seen::new(), &pointed).expect("rewritten");
        let handle = rewritten
            .trim()
            .strip_prefix(&format!("{}/", jellium_protocol::POINTED_PREFIX))
            .expect("a handle path");
        assert_eq!(
            pointed.resolve(handle).as_deref(),
            Some("https://elsewhere.test/whatever.ts")
        );
    }

    #[test]
    fn a_foreign_origin_carrying_no_web_scheme_fails_the_rewrite() {
        let body = "file:///etc/passwd\n";
        assert_eq!(
            rewrite(
                body,
                &source(),
                &base(),
                &route::Seen::new(),
                &Pointed::new()
            ),
            Err(Refusal::ManifestNotRewritable)
        );
    }

    #[test]
    fn a_url_outside_the_route_table_fails_the_rewrite() {
        let body = "/QuickConnect/Initiate\n";
        assert_eq!(
            rewrite(
                body,
                &source(),
                &base(),
                &route::Seen::new(),
                &Pointed::new()
            ),
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
        let rewritten = rewrite(
            "hls1/main/0.ts\n",
            &source,
            &base,
            &route::Seen::new(),
            &Pointed::new(),
        )
        .expect("rewritten");
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
        let rewritten = rewrite(&body, &source, &base, &route::Seen::new(), &Pointed::new())
            .expect("rewritten");
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
        let rewritten = rewrite(&body, &source, &base, &route::Seen::new(), &Pointed::new())
            .expect("rewritten");
        assert_eq!(rewritten, format!("/jellyfin/Videos/{ID}/hls1/main/0.ts\n"));
    }

    #[test]
    fn a_tag_the_rewriter_does_not_know_is_rewritten_when_it_carries_a_uri() {
        let body = "#EXT-X-PART:DURATION=1,URI=\"hls1/main/0.ts?api_key=secret\"\n".to_string();
        let rewritten = rewrite(
            &body,
            &source(),
            &base(),
            &route::Seen::new(),
            &Pointed::new(),
        )
        .expect("rewritten");
        assert_eq!(
            rewritten,
            format!("#EXT-X-PART:DURATION=1,URI=\"/jellyfin/Videos/{ID}/hls1/main/0.ts\"\n")
        );
    }

    #[test]
    fn a_rewritten_line_keeps_its_spacing_and_its_ending() {
        let rewritten = rewrite(
            "  hls1/main/0.ts  \r\n#EXTINF:6,\r\n",
            &source(),
            &base(),
            &route::Seen::new(),
            &Pointed::new(),
        )
        .expect("rewritten");
        assert_eq!(
            rewritten,
            format!("  /jellyfin/Videos/{ID}/hls1/main/0.ts  \r\n#EXTINF:6,\r\n")
        );
    }

    #[test]
    fn a_body_still_naming_an_upstream_url_fails_the_rewrite() {
        let body = "#EXT-X-SESSION-DATA:VALUE=\"https://example.test/Auth/Keys\"\n";
        assert_eq!(
            rewrite(
                body,
                &source(),
                &base(),
                &route::Seen::new(),
                &Pointed::new()
            ),
            Err(Refusal::ManifestNotRewritable)
        );
    }
}
