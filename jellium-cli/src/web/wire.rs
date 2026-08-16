//! Every upstream request the playback chain issues, built here so the bytes
//! on the wire are this module's and not a generated client's.

use jellium_protocol::Failure;

use super::link::unreachable;
use super::upstream::Upstream;

/// The characters `encodeURIComponent` leaves alone: the unreserved set and
/// `!`, `'`, `(`, `)`, `*`.
const UNESCAPED: &percent_encoding::AsciiSet = &percent_encoding::NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'_')
    .remove(b'.')
    .remove(b'!')
    .remove(b'~')
    .remove(b'*')
    .remove(b'\'')
    .remove(b'(')
    .remove(b')');

/// The query a url carries, in the order the pairs were set.
// reference: params-to-string — apiClient.js:55-66
pub struct Query(Vec<(&'static str, String)>);

impl Query {
    pub fn new() -> Query {
        Query(Vec::new())
    }

    /// The query with `name` carrying `value`; an empty value is dropped, the
    /// way `paramsToString` drops one.
    pub fn set(mut self, name: &'static str, value: impl std::fmt::Display) -> Query {
        let value = value.to_string();
        if !value.is_empty() {
            self.0.push((name, value));
        }
        self
    }

    /// The query with `name` carrying `value` when it is present, and
    /// untouched when it is absent.
    pub fn maybe(self, name: &'static str, value: Option<impl std::fmt::Display>) -> Query {
        match value {
            Some(value) => self.set(name, value),
            None => self,
        }
    }

    /// The pairs joined the way `paramsToString` joins them, each key and
    /// value encoded the way `encodeURIComponent` encodes it.
    pub fn rendered(&self) -> String {
        self.0
            .iter()
            .map(|(name, value)| {
                let name = percent_encoding::utf8_percent_encode(name, UNESCAPED);
                let value = percent_encoding::utf8_percent_encode(value, UNESCAPED);
                format!("{name}={value}")
            })
            .collect::<Vec<String>>()
            .join("&")
    }
}

/// The url `path` and `query` name against `base`.
pub fn url(base: &reqwest::Url, path: &str, query: &Query) -> reqwest::Url {
    let mut url = base.clone();
    if let Ok(mut segments) = url.path_segments_mut() {
        segments.pop_if_empty();
        segments.extend(path.split('/'));
    }
    let rendered = query.rendered();
    url.set_query((!rendered.is_empty()).then_some(rendered.as_str()));
    url
}

/// The answer's body, refusing any status at or above 400.
fn answered<R: serde::de::DeserializeOwned>(
    upstream: &Upstream,
    answer: jellyfin_api::RawResponse,
) -> Result<R, Failure> {
    let server = upstream.link().server();
    if answer.status == 401 || answer.status == 403 {
        return Err(Failure::TokenRejected);
    }
    if answer.status >= 400 {
        return Err(unreachable(server, format!("status {}", answer.status)));
    }
    serde_json::from_slice(&answer.body).map_err(|error| unreachable(server, error))
}

/// Reads what `path` and `query` name.
pub async fn got<R: serde::de::DeserializeOwned>(
    upstream: &Upstream,
    path: &str,
    query: &Query,
) -> Result<R, Failure> {
    let link = upstream.link();
    let target = url(link.base(), path, query);
    let answer = link
        .control()
        .raw_request(reqwest::Method::GET, target.as_str(), &[], None)
        .await
        .map_err(|error| unreachable(link.server(), error))?;
    answered(upstream, answer)
}

/// Posts `body`, serialized by `serde_json`, which preserves declaration
/// order, and reads the answer.
pub async fn posted<B: serde::Serialize, R: serde::de::DeserializeOwned>(
    upstream: &Upstream,
    path: &str,
    query: &Query,
    body: &B,
) -> Result<R, Failure> {
    let link = upstream.link();
    let target = url(link.base(), path, query);
    let rendered = serde_json::to_vec(body).map_err(|error| unreachable(link.server(), error))?;
    let answer = link
        .control()
        .raw_request(
            reqwest::Method::POST,
            target.as_str(),
            &[(
                reqwest::header::CONTENT_TYPE,
                "application/json".to_string(),
            )],
            Some(&rendered),
        )
        .await
        .map_err(|error| unreachable(link.server(), error))?;
    answered(upstream, answer)
}

/// The status alone, refusing any status at or above 400.
fn accepted(upstream: &Upstream, answer: &jellyfin_api::RawResponse) -> Result<(), Failure> {
    if answer.status == 401 || answer.status == 403 {
        return Err(Failure::TokenRejected);
    }
    if answer.status >= 400 {
        return Err(unreachable(
            upstream.link().server(),
            format!("status {}", answer.status),
        ));
    }
    Ok(())
}

/// Deletes what `path` and `query` name, reading no answer.
pub async fn deleted(upstream: &Upstream, path: &str, query: &Query) -> Result<(), Failure> {
    let link = upstream.link();
    let target = url(link.base(), path, query);
    let answer = link
        .control()
        .raw_request(reqwest::Method::DELETE, target.as_str(), &[], None)
        .await
        .map_err(|error| unreachable(link.server(), error))?;
    accepted(upstream, &answer)
}

/// Posts `body`, serialized by `serde_json`, to a route whose answer is a
/// status and nothing else, refusing any status at or above 400.
pub async fn told<B: serde::Serialize>(
    upstream: &Upstream,
    path: &str,
    query: &Query,
    body: &B,
) -> Result<(), Failure> {
    let link = upstream.link();
    let target = url(link.base(), path, query);
    let rendered = serde_json::to_vec(body).map_err(|error| unreachable(link.server(), error))?;
    let answer = link
        .control()
        .raw_request(
            reqwest::Method::POST,
            target.as_str(),
            &[(
                reqwest::header::CONTENT_TYPE,
                "application/json".to_string(),
            )],
            Some(&rendered),
        )
        .await
        .map_err(|error| unreachable(link.server(), error))?;
    accepted(upstream, &answer)
}

/// Posts nothing at all to a route whose answer is a status and nothing else,
/// refusing any status at or above 400.
pub async fn poked(upstream: &Upstream, path: &str, query: &Query) -> Result<(), Failure> {
    let link = upstream.link();
    let target = url(link.base(), path, query);
    let answer = link
        .control()
        .raw_request(reqwest::Method::POST, target.as_str(), &[], None)
        .await
        .map_err(|error| unreachable(link.server(), error))?;
    accepted(upstream, &answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_empty_value_is_dropped_and_a_reserved_one_is_encoded() {
        let query = Query::new()
            .set("UserId", "a b/c")
            .set("Empty", "")
            .maybe("Absent", None::<i32>)
            .maybe("Present", Some(7));
        assert_eq!(query.rendered(), "UserId=a%20b%2Fc&Present=7");
    }

    #[test]
    fn a_url_extends_the_servers_path_and_carries_no_query_when_the_query_is_empty() {
        let base = reqwest::Url::parse("https://example.test/jellyfin").expect("a base");
        assert_eq!(
            url(&base, "Items/1/PlaybackInfo", &Query::new()).as_str(),
            "https://example.test/jellyfin/Items/1/PlaybackInfo"
        );
        assert_eq!(
            url(&base, "LiveStreams/Open", &Query::new().set("UserId", "u")).as_str(),
            "https://example.test/jellyfin/LiveStreams/Open?UserId=u"
        );
    }
}
