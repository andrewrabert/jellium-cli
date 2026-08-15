//! A configuration page is served only under a live grant and only under a name
//! the relay has observed, with every subresource rewritten to the same door
//! and the bridge shim injected.

use std::sync::Arc;

use axum::Json;
use axum::extract::{Path, State};
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use jellium_protocol::{Framed, PageRequest, Refusal};

use super::AppState;
use super::route;

/// The largest configuration page document the local server serves.
pub const PAGE_LIMIT: usize = 1 << 20;

/// The shim a rewritten document carries: the nine bridge verbs over
/// `postMessage`, and nothing else that reaches the application.
const SHIM: &str = include_str!("bridge.js");

/// The content security policy every served page carries.
const POLICY: &str = "default-src 'none'; script-src 'self' 'unsafe-inline'; \
                      style-src 'self' 'unsafe-inline'; img-src 'self' data:; \
                      font-src 'self'; connect-src 'none'; form-action 'none'; \
                      frame-ancestors 'self'; base-uri 'none'";

/// The configuration page frames open now, each holding the one grant its
/// document and its subresources are reachable under, with the instant that
/// grant was last used.
pub struct Grants {
    held: tokio::sync::RwLock<std::collections::HashMap<String, tokio::time::Instant>>,
}

impl Grants {
    /// A grant untouched for this long is no longer live, so a frame whose tab
    /// died without releasing it holds the door open no longer than this.
    pub const IDLE: std::time::Duration = std::time::Duration::from_secs(900);

    pub fn new() -> Grants {
        Grants {
            held: tokio::sync::RwLock::new(std::collections::HashMap::new()),
        }
    }

    /// Mints a grant and returns it; the grant is the only way a configuration
    /// page is reachable.
    pub async fn open(&self) -> String {
        let grant = super::guard::opaque();
        self.held
            .write()
            .await
            .insert(grant.clone(), tokio::time::Instant::now());
        grant
    }

    pub async fn holds(&self, grant: &str) -> bool {
        let now = tokio::time::Instant::now();
        let mut held = self.held.write().await;
        match held.get_mut(grant) {
            Some(used) if now.duration_since(*used) < Grants::IDLE => {
                *used = now;
                true
            }
            Some(_) => {
                held.remove(grant);
                false
            }
            None => false,
        }
    }

    pub async fn close(&self, grant: &str) {
        self.held.write().await.remove(grant);
    }

    /// Drops every grant untouched for `IDLE`.
    pub async fn sweep(&self) {
        let now = tokio::time::Instant::now();
        self.held
            .write()
            .await
            .retain(|_, used| now.duration_since(*used) < Grants::IDLE);
    }
}

impl Default for Grants {
    fn default() -> Grants {
        Grants::new()
    }
}

/// The configuration page name a reference names, and `None` for a reference
/// naming anything else.
/// Only `ConfigurationPage?name=<name>`, however it is spelled relative to the
/// document, names one; an absolute url and a protocol-relative url never do.
fn referenced(reference: &str) -> Option<String> {
    if reference.contains("://") || reference.starts_with("//") {
        return None;
    }
    let (path, query) = reference.split_once('?')?;
    let last = path.rsplit('/').next()?;
    if !last.eq_ignore_ascii_case("ConfigurationPage") {
        return None;
    }
    query.split('&').find_map(|pair| {
        let (name, value) = pair.split_once('=')?;
        name.eq_ignore_ascii_case("name").then(|| {
            percent_encoding::percent_decode_str(value)
                .decode_utf8()
                .map_or_else(|_| value.to_owned(), |decoded| decoded.into_owned())
        })
    })
}

/// One stretch of a document the rewriter treats alike.
enum Span {
    /// Copied byte for byte: text between tags, a comment, a doctype, an end
    /// tag, and the body of a `script` or a `style` element.
    Verbatim(std::ops::Range<usize>),
    /// A start tag: the whole tag, and the element name it opens.
    Tag {
        tag: std::ops::Range<usize>,
        name: std::ops::Range<usize>,
    },
}

/// The elements whose bodies are raw text rather than markup.
const RAW: [&str; 2] = ["script", "style"];

/// A bounded walk over a document's markup: it tells a start tag from a
/// comment, a doctype, an end tag, text between tags, and the raw-text body a
/// `script` or a `style` element carries, and reads no further into the
/// language.
struct Markup<'doc> {
    document: &'doc str,
    at: usize,
    /// The raw-text element whose body the next span is, once its start tag has
    /// been read.
    raw: Option<&'static str>,
}

impl<'doc> Markup<'doc> {
    fn of(document: &'doc str) -> Markup<'doc> {
        Markup {
            document,
            at: 0,
            raw: None,
        }
    }

    /// The end of the start tag opening at `self.at`, one past its `>`, with a
    /// `>` standing in a quoted attribute value passed over.
    fn closes(&self) -> usize {
        let bytes = self.document.as_bytes();
        let mut at = self.at + 1;
        let mut quote = None;
        while at < bytes.len() {
            match (quote, bytes[at]) {
                (None, b'"' | b'\'') => quote = Some(bytes[at]),
                (Some(open), byte) if byte == open => quote = None,
                (None, b'>') => return at + 1,
                _ => {}
            }
            at += 1;
        }
        bytes.len()
    }

    /// The end of the element name opening at `self.at`.
    fn named(&self, closes: usize) -> usize {
        let bytes = self.document.as_bytes();
        let mut at = self.at + 1;
        while at < closes
            && !bytes[at].is_ascii_whitespace()
            && bytes[at] != b'/'
            && bytes[at] != b'>'
        {
            at += 1;
        }
        at
    }
}

impl Iterator for Markup<'_> {
    type Item = Span;

    fn next(&mut self) -> Option<Span> {
        let document = self.document;
        if self.at >= document.len() {
            return None;
        }
        if let Some(raw) = self.raw.take() {
            let end = document[self.at..]
                .to_ascii_lowercase()
                .find(&format!("</{raw}"))
                .map_or(document.len(), |found| self.at + found);
            let span = Span::Verbatim(self.at..end);
            self.at = end;
            return Some(span);
        }
        let rest = &document[self.at..];
        if !rest.starts_with('<') {
            let end = rest[1..]
                .find('<')
                .map_or(document.len(), |found| self.at + 1 + found);
            let span = Span::Verbatim(self.at..end);
            self.at = end;
            return Some(span);
        }
        let opens = rest.as_bytes().get(1).copied();
        if rest.starts_with("<!--") {
            let end = rest
                .find("-->")
                .map_or(document.len(), |found| self.at + found + 3);
            let span = Span::Verbatim(self.at..end);
            self.at = end;
            return Some(span);
        }
        if matches!(opens, Some(b'!' | b'?' | b'/')) {
            let end = rest
                .find('>')
                .map_or(document.len(), |found| self.at + found + 1);
            let span = Span::Verbatim(self.at..end);
            self.at = end;
            return Some(span);
        }
        if !opens.is_some_and(|byte| byte.is_ascii_alphabetic()) {
            let span = Span::Verbatim(self.at..self.at + 1);
            self.at += 1;
            return Some(span);
        }
        let closes = self.closes();
        let name = self.at + 1..self.named(closes);
        let opened = &document[name.clone()];
        self.raw = RAW
            .into_iter()
            .find(|raw| opened.eq_ignore_ascii_case(raw))
            .filter(|_| !document[..closes].trim_end().ends_with("/>"));
        let span = Span::Tag {
            tag: self.at..closes,
            name,
        };
        self.at = closes;
        Some(span)
    }
}

/// The `src` and `href` attribute values the start tag `tag` spells, as byte
/// ranges into `document`, in the order it spells them.
/// A value quoted with `"` or with `'` is the quoted contents; an unquoted
/// value runs to the next whitespace or to the tag's `>`.
fn attributes(document: &str, tag: &std::ops::Range<usize>) -> Vec<std::ops::Range<usize>> {
    let bytes = document.as_bytes();
    let end = tag.end;
    let mut at = tag.start + 1;
    while at < end && !bytes[at].is_ascii_whitespace() && bytes[at] != b'/' && bytes[at] != b'>' {
        at += 1;
    }
    let mut found = Vec::new();
    while at < end {
        while at < end && (bytes[at].is_ascii_whitespace() || bytes[at] == b'/') {
            at += 1;
        }
        if at >= end || bytes[at] == b'>' {
            break;
        }
        let name = at;
        while at < end
            && !bytes[at].is_ascii_whitespace()
            && bytes[at] != b'='
            && bytes[at] != b'/'
            && bytes[at] != b'>'
        {
            at += 1;
        }
        let named = &document[name..at];
        while at < end && bytes[at].is_ascii_whitespace() {
            at += 1;
        }
        if at >= end || bytes[at] != b'=' {
            continue;
        }
        at += 1;
        while at < end && bytes[at].is_ascii_whitespace() {
            at += 1;
        }
        if at >= end {
            break;
        }
        let value = if bytes[at] == b'"' || bytes[at] == b'\'' {
            let quote = bytes[at];
            at += 1;
            let opens = at;
            while at < end && bytes[at] != quote {
                at += 1;
            }
            let closes = at;
            at = (at + 1).min(end);
            opens..closes
        } else {
            let opens = at;
            while at < end && !bytes[at].is_ascii_whitespace() && bytes[at] != b'>' {
                at += 1;
            }
            opens..at
        };
        if named.eq_ignore_ascii_case("src") || named.eq_ignore_ascii_case("href") {
            found.push(value);
        }
    }
    found
}

/// `document` with every `src` and `href` attribute value standing in a start
/// tag resolved to `PAGE_PREFIX/{grant}/{name}`, and `SHIM` injected ahead of
/// the document's first `script` element.
/// A comment, a doctype, text between tags, and the body of a `script` or a
/// `style` element are copied byte for byte, so a `src="` sequence inside one
/// is neither rewritten nor refused.
/// A value naming anything but a configuration page `seen` holds — an absolute
/// url, a protocol-relative url, or a path that is not
/// `ConfigurationPage?name=<observed>` — is refused as
/// `Refusal::PageNotRewritable`.
/// A value naming nothing at all, such as an empty attribute or a fragment, is
/// left as it stands, because it reaches no origin.
pub fn rewrite(document: &str, grant: &str, seen: &route::Seen) -> Result<String, Refusal> {
    let prefix = jellium_protocol::PAGE_PREFIX;
    let mut out = String::with_capacity(document.len() + SHIM.len() + 64);
    let mut scripts = None;
    for span in Markup::of(document) {
        let tag = match span {
            Span::Verbatim(range) => {
                out.push_str(&document[range]);
                continue;
            }
            Span::Tag { tag, name } => {
                if scripts.is_none() && document[name].eq_ignore_ascii_case("script") {
                    scripts = Some(out.len());
                }
                tag
            }
        };
        let mut at = tag.start;
        for value in attributes(document, &tag) {
            let reference = &document[value.clone()];
            out.push_str(&document[at..value.start]);
            if reference.is_empty() || reference.starts_with('#') {
                out.push_str(reference);
            } else {
                let name = referenced(reference).ok_or(Refusal::PageNotRewritable)?;
                if !seen.holds(route::Observed::Page, &name) {
                    return Err(Refusal::PageNotRewritable);
                }
                let encoded = percent_encoding::utf8_percent_encode(
                    &name,
                    percent_encoding::NON_ALPHANUMERIC,
                );
                out.push_str(&format!("{prefix}/{grant}/{encoded}"));
            }
            at = value.end;
        }
        out.push_str(&document[at..tag.end]);
    }

    let shim = format!("<script>{SHIM}</script>");
    out.insert_str(scripts.unwrap_or(out.len()), &shim);
    Ok(out)
}

fn refuse(status: StatusCode, refusal: Refusal) -> Response {
    (status, Json(refusal)).into_response()
}

/// Mints a grant and answers the frame's path.
pub async fn open(
    State(state): State<Arc<AppState>>,
    Json(request): Json<PageRequest>,
) -> Response {
    if !state.seen.holds(route::Observed::Page, &request.name) {
        return refuse(StatusCode::FORBIDDEN, Refusal::PageNotListed);
    }
    let grant = state.pages.open().await;
    let prefix = jellium_protocol::PAGE_PREFIX;
    let encoded =
        percent_encoding::utf8_percent_encode(&request.name, percent_encoding::NON_ALPHANUMERIC);
    Json(Framed {
        path: format!("{prefix}/{grant}/{encoded}"),
        grant,
    })
    .into_response()
}

/// Releases a grant, which is what closing the screen does.
pub async fn close(State(state): State<Arc<AppState>>, Json(framed): Json<Framed>) -> Response {
    state.pages.close(&framed.grant).await;
    StatusCode::NO_CONTENT.into_response()
}

/// Serves the configuration page `name` under `grant`, rewritten, with
/// `POLICY` and `X-Content-Type-Options: nosniff`.
/// A grant that is not live, and a name `Seen` does not hold, are refused as
/// `Refusal::PageNotListed`.
pub async fn serve(
    State(state): State<Arc<AppState>>,
    Path((grant, name)): Path<(String, String)>,
) -> Response {
    if !state.pages.holds(&grant).await || !state.seen.holds(route::Observed::Page, &name) {
        return refuse(StatusCode::FORBIDDEN, Refusal::PageNotListed);
    }

    let Some(upstream) = state.session.signed().await else {
        return refuse(StatusCode::CONFLICT, Refusal::NoSession);
    };

    let document = match upstream.configuration_page(&name).await {
        Ok(document) => document,
        Err(refusal) => return refuse(StatusCode::BAD_GATEWAY, refusal),
    };

    match rewrite(&document, &grant, &state.seen) {
        Ok(rewritten) => (
            StatusCode::OK,
            [
                (header::CONTENT_TYPE, "text/html; charset=utf-8"),
                (header::CONTENT_SECURITY_POLICY, POLICY),
                (header::X_CONTENT_TYPE_OPTIONS, "nosniff"),
            ],
            rewritten,
        )
            .into_response(),
        Err(refusal) => refuse(StatusCode::FORBIDDEN, refusal),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seen() -> route::Seen {
        let seen = route::Seen::new();
        seen.record(
            route::Observed::Page,
            r#"[{"Name":"SyntheticPluginPage"},{"Name":"Other"}]"#,
        );
        seen
    }

    #[test]
    fn a_configuration_page_reference_resolves_to_the_same_door() {
        let rewritten = rewrite(
            r#"<html><head><link href="ConfigurationPage?name=Other"></head></html>"#,
            "grant",
            &seen(),
        )
        .expect("a rewrite");
        assert!(rewritten.contains("/page/grant/Other"));
    }

    #[test]
    fn a_foreign_reference_fails_the_rewrite() {
        for document in [
            r#"<img src="https://example.test/a.png">"#,
            r#"<img src="//example.test/a.png">"#,
            r#"<img src="/web/assets/a.png">"#,
        ] {
            assert_eq!(
                rewrite(document, "grant", &seen()),
                Err(Refusal::PageNotRewritable),
                "{document}"
            );
        }
    }

    #[test]
    fn a_page_the_local_server_has_not_seen_fails_the_rewrite() {
        assert_eq!(
            rewrite(
                r#"<link href="ConfigurationPage?name=Unlisted">"#,
                "grant",
                &seen()
            ),
            Err(Refusal::PageNotRewritable)
        );
    }

    #[test]
    fn the_shim_is_injected_ahead_of_the_first_script() {
        let rewritten = rewrite(
            "<html><body><script>page()</script></body></html>",
            "g",
            &seen(),
        )
        .expect("a rewrite");
        let shim = rewritten.find("window.ApiClient").expect("the shim");
        let page = rewritten.find("page()").expect("the page's own script");
        assert!(shim < page);
    }

    #[test]
    fn a_document_with_no_script_still_carries_the_shim() {
        let rewritten =
            rewrite("<html><body>nothing</body></html>", "g", &seen()).expect("a rewrite");
        assert!(rewritten.contains("window.Dashboard"));
    }

    #[test]
    fn a_fragment_reference_is_left_as_it_stands() {
        let rewritten = rewrite(r##"<a href="#top">top</a>"##, "g", &seen()).expect("a rewrite");
        assert!(rewritten.contains(r##"href="#top""##));
    }

    #[test]
    fn an_inline_script_spelling_a_reference_is_served_byte_for_byte() {
        let document = r#"<html><body><script>var a = 'src="https://example.test/a.png"';</script></body></html>"#;
        let rewritten = rewrite(document, "g", &seen()).expect("a rewrite");
        assert!(rewritten.contains(r#"var a = 'src="https://example.test/a.png"';"#));
    }

    #[test]
    fn a_comment_spelling_a_reference_is_served_rather_than_refused() {
        let document = r#"<html><!-- href="https://example.test/a.css" --><body></body></html>"#;
        let rewritten = rewrite(document, "g", &seen()).expect("a rewrite");
        assert!(rewritten.contains(r#"<!-- href="https://example.test/a.css" -->"#));
    }

    #[test]
    fn a_single_quoted_and_an_unquoted_reference_resolve_to_the_same_door() {
        for document in [
            r#"<link href='ConfigurationPage?name=Other'>"#,
            r#"<link href=ConfigurationPage?name=Other>"#,
        ] {
            let rewritten = rewrite(document, "grant", &seen()).expect("a rewrite");
            assert!(rewritten.contains("/page/grant/Other"), "{document}");
        }
    }

    #[test]
    fn a_style_body_spelling_a_reference_is_served_byte_for_byte() {
        let document = r#"<style>a { background: url("https://example.test/a.png"); }</style>"#;
        let rewritten = rewrite(document, "g", &seen()).expect("a rewrite");
        assert!(rewritten.contains(r#"url("https://example.test/a.png")"#));
    }

    #[tokio::test(start_paused = true)]
    async fn a_grant_untouched_for_the_idle_span_opens_nothing() {
        let grants = Grants::new();
        let unswept = grants.open().await;
        let swept = grants.open().await;
        tokio::time::advance(Grants::IDLE).await;
        assert!(!grants.holds(&unswept).await);
        grants.sweep().await;
        assert!(!grants.holds(&swept).await);
    }

    #[tokio::test(start_paused = true)]
    async fn a_grant_in_use_outlives_the_idle_span() {
        let grants = Grants::new();
        let grant = grants.open().await;
        for _ in 0..3 {
            tokio::time::advance(Grants::IDLE / 2).await;
            assert!(grants.holds(&grant).await);
        }
        grants.sweep().await;
        assert!(grants.holds(&grant).await);
    }

    #[tokio::test]
    async fn a_grant_is_live_until_it_is_closed() {
        let grants = Grants::new();
        let grant = grants.open().await;
        assert!(grants.holds(&grant).await);
        grants.close(&grant).await;
        assert!(!grants.holds(&grant).await);
    }

    #[tokio::test]
    async fn two_grants_are_told_apart() {
        let grants = Grants::new();
        let first = grants.open().await;
        let second = grants.open().await;
        assert_ne!(first, second);
        grants.close(&first).await;
        assert!(grants.holds(&second).await);
    }
}
