use axum::http::Method;
use uuid::Uuid;

/// One segment of a relayed route.
enum Segment {
    /// Matched without regard to ascii case, forwarded in this spelling.
    Literal(&'static str),

    /// Matched against each spelling without regard to ascii case, forwarded
    /// in the spelling that matched.
    OneOf(&'static [&'static str]),

    /// One segment parsing as a uuid, forwarded hyphenated and lowercase.
    Id,
}

/// One Jellyfin route Jellium Web is relayed.
struct Route {
    method: Method,
    path: &'static [Segment],
}

/// The image kinds Jellium Web asks for.
static IMAGE_KINDS: &[&str] = &["Primary", "Backdrop"];

/// Every route Jellium Web calls, and no other.
static RELAYED: &[Route] = &[
    Route {
        method: Method::GET,
        path: &[Segment::Literal("Items")],
    },
    Route {
        method: Method::GET,
        path: &[Segment::Literal("Items"), Segment::Id],
    },
    Route {
        method: Method::GET,
        path: &[
            Segment::Literal("Items"),
            Segment::Id,
            Segment::Literal("Images"),
            Segment::OneOf(IMAGE_KINDS),
        ],
    },
    Route {
        method: Method::GET,
        path: &[Segment::Literal("UserViews")],
    },
    Route {
        method: Method::GET,
        path: &[Segment::Literal("UserItems"), Segment::Literal("Resume")],
    },
    Route {
        method: Method::GET,
        path: &[Segment::Literal("Shows"), Segment::Literal("NextUp")],
    },
    Route {
        method: Method::GET,
        path: &[
            Segment::Literal("Shows"),
            Segment::Id,
            Segment::Literal("Seasons"),
        ],
    },
    Route {
        method: Method::GET,
        path: &[
            Segment::Literal("Shows"),
            Segment::Id,
            Segment::Literal("Episodes"),
        ],
    },
    Route {
        method: Method::POST,
        path: &[Segment::Literal("UserPlayedItems"), Segment::Id],
    },
    Route {
        method: Method::DELETE,
        path: &[Segment::Literal("UserPlayedItems"), Segment::Id],
    },
    Route {
        method: Method::POST,
        path: &[Segment::Literal("UserFavoriteItems"), Segment::Id],
    },
    Route {
        method: Method::DELETE,
        path: &[Segment::Literal("UserFavoriteItems"), Segment::Id],
    },
];

impl Segment {
    /// The spelling `decoded` is forwarded in when it matches, or `None`.
    fn admit(&self, decoded: &str) -> Option<String> {
        match self {
            Segment::Literal(literal) => decoded
                .eq_ignore_ascii_case(literal)
                .then(|| literal.to_string()),
            Segment::OneOf(options) => options
                .iter()
                .find(|option| decoded.eq_ignore_ascii_case(option))
                .map(|option| option.to_string()),
            Segment::Id => decoded.parse::<Uuid>().ok().map(|id| id.to_string()),
        }
    }
}

impl Route {
    fn admit(&self, method: &Method, decoded: &[String]) -> Option<Vec<String>> {
        if self.method != *method || self.path.len() != decoded.len() {
            return None;
        }
        self.path
            .iter()
            .zip(decoded)
            .map(|(segment, decoded)| segment.admit(decoded))
            .collect()
    }
}

/// A relayed request the route table admits, held in the form it is forwarded
/// in: every segment is a literal from the table or a uuid re-serialized from
/// the browser's, so no byte the browser chose reaches the upstream url as a
/// delimiter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Target {
    method: Method,
    segments: Vec<String>,
}

impl Target {
    /// Splits `path` on `/`, drops empty segments, percent-decodes each, and
    /// returns the first route in `RELAYED` whose method and segments all
    /// match.
    /// A segment that is not utf-8 once decoded matches nothing, and so does
    /// one holding a decoded `/`, `?` or `#`, since no `Segment` admits it.
    pub fn admit(method: &Method, path: &str) -> Option<Target> {
        let decoded = path
            .split('/')
            .filter(|segment| !segment.is_empty())
            .map(|segment| {
                percent_encoding::percent_decode_str(segment)
                    .decode_utf8()
                    .ok()
                    .map(|decoded| decoded.into_owned())
            })
            .collect::<Option<Vec<String>>>()?;

        RELAYED
            .iter()
            .find_map(|route| route.admit(method, &decoded))
            .map(|segments| Target {
                method: method.clone(),
                segments,
            })
    }

    pub fn method(&self) -> Method {
        self.method.clone()
    }

    /// `base` with this target's segments appended and `query` set, both
    /// encoded by the url parser rather than pasted into text it re-reads.
    pub fn url(&self, base: &reqwest::Url, query: Option<&str>) -> reqwest::Url {
        let mut url = base.clone();
        {
            let mut segments = url
                .path_segments_mut()
                .expect("the upstream base is a hierarchical url");
            segments.pop_if_empty();
            segments.extend(&self.segments);
        }
        url.set_query(query);
        url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ID: &str = "0191b2f0-1c3d-4e5f-8a9b-0c1d2e3f4a5b";

    #[test]
    fn an_encoded_query_delimiter_is_not_relayed() {
        assert!(Target::admit(&Method::GET, "Items%3Fx=1").is_none());
    }

    #[test]
    fn an_encoded_fragment_delimiter_is_not_relayed() {
        assert!(Target::admit(&Method::GET, "Items%23x").is_none());
    }

    #[test]
    fn an_encoded_path_delimiter_is_not_relayed() {
        assert!(Target::admit(&Method::GET, "Items%2FBad").is_none());
    }

    #[test]
    fn a_lowercase_percent_escape_is_not_relayed() {
        assert!(Target::admit(&Method::GET, "Items%2fBad").is_none());
    }

    #[test]
    fn an_api_key_path_is_not_relayed() {
        assert!(Target::admit(&Method::GET, "Auth/Keys").is_none());
        assert!(Target::admit(&Method::POST, "Auth/Keys").is_none());
    }

    #[test]
    fn a_sign_in_path_is_not_relayed() {
        assert!(Target::admit(&Method::POST, "Users/AuthenticateByName").is_none());
    }

    #[test]
    fn a_revoke_path_is_not_relayed() {
        assert!(Target::admit(&Method::POST, "Sessions/Logout").is_none());
    }

    #[test]
    fn a_write_method_on_a_read_route_is_not_relayed() {
        assert!(Target::admit(&Method::GET, &format!("Items/{ID}")).is_some());
        assert!(Target::admit(&Method::POST, &format!("Items/{ID}")).is_none());
        assert!(Target::admit(&Method::DELETE, &format!("Items/{ID}")).is_none());
    }

    #[test]
    fn every_route_jellium_web_calls_is_relayed() {
        let calls = [
            (Method::GET, "Items".to_string()),
            (Method::GET, format!("Items/{ID}")),
            (Method::GET, format!("Items/{ID}/Images/Primary")),
            (Method::GET, format!("Items/{ID}/Images/Backdrop")),
            (Method::GET, "UserViews".to_string()),
            (Method::GET, "UserItems/Resume".to_string()),
            (Method::GET, "Shows/NextUp".to_string()),
            (Method::GET, format!("Shows/{ID}/Seasons")),
            (Method::GET, format!("Shows/{ID}/Episodes")),
            (Method::POST, format!("UserPlayedItems/{ID}")),
            (Method::DELETE, format!("UserPlayedItems/{ID}")),
            (Method::POST, format!("UserFavoriteItems/{ID}")),
            (Method::DELETE, format!("UserFavoriteItems/{ID}")),
        ];
        for (method, path) in calls {
            assert!(
                Target::admit(&method, &path).is_some(),
                "{method} {path} was not relayed"
            );
        }
    }

    #[test]
    fn an_admitted_target_forwards_the_table_spelling_and_a_canonical_id() {
        let target = Target::admit(&Method::GET, "items/0191B2F0-1C3D-4E5F-8A9B-0C1D2E3F4A5B")
            .expect("a relayed route");
        assert_eq!(target.segments, vec!["Items".to_string(), ID.to_string()]);
    }

    #[test]
    fn a_query_carrying_a_fragment_delimiter_is_encoded() {
        let base = reqwest::Url::parse("https://example.test").expect("base");
        let target = Target::admit(&Method::GET, "Items").expect("route");
        let url = target.url(&base, Some("term=a#b"));
        assert!(!url.as_str().contains('#'));
        assert_eq!(url.query(), Some("term=a%23b"));
    }

    #[test]
    fn a_server_url_carrying_a_path_keeps_it_under_the_relayed_segments() {
        let base = reqwest::Url::parse("https://example.test/jellyfin").expect("base");
        let target = Target::admit(&Method::GET, &format!("Items/{ID}")).expect("route");
        let url = target.url(&base, None);
        assert_eq!(url.path(), format!("/jellyfin/Items/{ID}"));
    }
}
