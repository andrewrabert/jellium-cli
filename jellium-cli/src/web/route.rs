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

    /// 1..=`max` bytes of ascii letters, digits, `-` and `_`, forwarded as
    /// received.
    Token { max: usize },

    /// 1..=`max` ascii digits, forwarded as received.
    Number { max: usize },

    /// `stem`, a `.`, and one of `extensions`, each matched without regard to
    /// ascii case and forwarded in the table's spelling.
    Suffixed {
        stem: &'static str,
        extensions: &'static [&'static str],
    },

    /// 1..=`max` ascii digits, a `.`, and one of `extensions`.
    Segmented {
        max: usize,
        extensions: &'static [&'static str],
    },

    /// 1..=`max` bytes of ascii digits and `.`, forwarded as received.
    Version { max: usize },

    /// A name `Seen` holds under `Observed`, forwarded as received.
    Seen(Observed),
}

/// The methods the route table admits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Verb {
    Get,
    Post,
    Delete,
}

impl Verb {
    fn method(self) -> Method {
        match self {
            Verb::Get => Method::GET,
            Verb::Post => Method::POST,
            Verb::Delete => Method::DELETE,
        }
    }
}

/// A name the local server admits only because it has itself seen it in a
/// listing during this run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Observed {
    /// A package name a repository listing carried.
    Package,
    /// A configuration page name a configuration-page listing carried.
    Page,
    /// A foreign image url a remote-search or remote-images answer carried.
    Image,
}

/// Where a handle the local server minted may cross a route on its way
/// upstream, declared per entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resolves {
    /// No value is resolved.
    None,
    /// The `imageUrl` query parameter carries a handle.
    Query,
    /// The request body's `ImageUrl` carries a handle.
    Body,
}

/// What a route does upstream and what it carries to do it, declared per entry
/// rather than derived from the method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Access {
    Read,
    ReadWithBody,
    Write,
    WriteWithBody,
}

impl Access {
    /// True when the route only reads upstream, which is what `--read-only`
    /// admits.
    pub fn read_only(self) -> bool {
        matches!(self, Access::Read | Access::ReadWithBody)
    }
}

/// The names the local server has itself seen in a listing during this run.
#[derive(Debug)]
pub struct Seen {
    packages: std::sync::RwLock<std::collections::HashSet<String>>,
    pages: std::sync::RwLock<std::collections::HashSet<String>>,
    /// The foreign image urls this run has itself seen in an answer.
    images: std::sync::RwLock<std::collections::HashSet<String>>,
    /// The foreign image urls this run has minted handles for, by handle and
    /// by url, so a url seen twice keeps the handle it was first given.
    foreign: std::sync::RwLock<Foreign>,
}

/// The foreign image urls minted this run.
#[derive(Debug, Default)]
struct Foreign {
    by_handle: std::collections::HashMap<String, String>,
    by_url: std::collections::HashMap<String, String>,
    minted: u64,
}

impl Seen {
    pub fn new() -> Seen {
        Seen {
            packages: std::sync::RwLock::new(std::collections::HashSet::new()),
            pages: std::sync::RwLock::new(std::collections::HashSet::new()),
            images: std::sync::RwLock::new(std::collections::HashSet::new()),
            foreign: std::sync::RwLock::new(Foreign::default()),
        }
    }

    /// Records every foreign image url `body` carries under `Observed::Image`
    /// and answers `body` with each replaced by the handle minted for it; a url
    /// seen before keeps the handle it was first given.
    /// A body that is not json is answered unchanged.
    pub fn foreign(&self, body: &str) -> String {
        let Ok(mut value) = serde_json::from_str::<serde_json::Value>(body) else {
            return body.to_owned();
        };
        self.minting(&mut value);
        serde_json::to_string(&value).unwrap_or_else(|_| body.to_owned())
    }

    /// Replaces every absolute http url `value` carries with its handle.
    fn minting(&self, value: &mut serde_json::Value) {
        match value {
            serde_json::Value::String(held) => {
                if is_foreign(held) {
                    *held = self.mint(held);
                }
            }
            serde_json::Value::Array(held) => {
                for entry in held {
                    self.minting(entry);
                }
            }
            serde_json::Value::Object(held) => {
                for (_, entry) in held.iter_mut() {
                    self.minting(entry);
                }
            }
            _ => {}
        }
    }

    /// The handle `url` is minted under, minting one when this run holds none.
    /// The url is recorded under `Observed::Image`, so `holds` answers whether
    /// this run has itself seen it.
    fn mint(&self, url: &str) -> String {
        match self.held(Observed::Image).write() {
            Ok(mut seen) => seen.insert(url.to_owned()),
            Err(poisoned) => poisoned.into_inner().insert(url.to_owned()),
        };
        let mut held = match self.foreign.write() {
            Ok(held) => held,
            Err(poisoned) => poisoned.into_inner(),
        };
        if let Some(handle) = held.by_url.get(url) {
            return handle.clone();
        }
        held.minted += 1;
        let handle = format!("f{:016x}", held.minted);
        held.by_url.insert(url.to_owned(), handle.clone());
        held.by_handle.insert(handle.clone(), url.to_owned());
        handle
    }

    /// The url `handle` was minted for, and `None` for a handle this run did
    /// not mint.
    pub fn observed(&self, handle: &str) -> Option<String> {
        match self.foreign.read() {
            Ok(held) => held.by_handle.get(handle).cloned(),
            Err(poisoned) => poisoned.into_inner().by_handle.get(handle).cloned(),
        }
    }

    fn held(&self, observed: Observed) -> &std::sync::RwLock<std::collections::HashSet<String>> {
        match observed {
            Observed::Package => &self.packages,
            Observed::Page => &self.pages,
            Observed::Image => &self.images,
        }
    }

    /// Records every name `body` carries, read as `observed` names it.
    /// A configuration-page listing names each entry's `Name` and a package
    /// listing its `name`, so both spellings are read.
    pub fn record(&self, observed: Observed, body: &str) {
        let Ok(value) = serde_json::from_str::<serde_json::Value>(body) else {
            return;
        };
        let Some(entries) = value.as_array() else {
            return;
        };
        let mut held = match self.held(observed).write() {
            Ok(held) => held,
            Err(poisoned) => poisoned.into_inner(),
        };
        for entry in entries {
            if let Some(name) = entry
                .get("Name")
                .or_else(|| entry.get("name"))
                .and_then(serde_json::Value::as_str)
            {
                held.insert(name.to_owned());
            }
        }
    }

    pub fn holds(&self, observed: Observed, name: &str) -> bool {
        match self.held(observed).read() {
            Ok(held) => held.contains(name),
            Err(poisoned) => poisoned.into_inner().contains(name),
        }
    }
}

impl Default for Seen {
    fn default() -> Seen {
        Seen::new()
    }
}

/// The query parameter names stripped from every browser-supplied query
/// string before it is forwarded, matched without regard to ascii case.
pub const STRIPPED: [&str; 3] = ["api_key", "ApiKey", "X-Emby-Token"];

/// The containers a direct-played or remuxed stream is served in.
static STREAM_CONTAINERS: &[&str] = &[
    "mp4", "m4v", "mkv", "webm", "mov", "ts", "avi", "mp3", "m4a", "flac", "ogg", "opus", "aac",
    "wav",
];

/// The containers an HLS segment is served in.
static SEGMENT_CONTAINERS: &[&str] = &["ts", "mp4", "m4s", "aac", "mp3"];

/// The name the Jellyfin server gives an HLS stream's fMP4 initialization
/// segment.
const INITIALIZATION: &str = "-1";

/// The item kinds an instant mix is built from.
static MIX_PARENTS: &[&str] = &["Items", "Albums", "Artists"];

/// What the relay does with a route's response body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Payload {
    /// The response can be an HLS manifest, so it is asked for undecoded and
    /// read and rewritten when its content type names one.
    Manifest,
    /// The response is streamed to the browser in the encoding it arrives in.
    Streamed,
    /// The body is read up to `OBSERVED_LIMIT`, every name it carries is
    /// recorded under `Observed`, and the bytes are forwarded unchanged.
    Observed(Observed),
    /// Only the last `TAIL_LIMIT` bytes are delivered, answered `206` with the
    /// full length in `Content-Range`.
    Tail,
    /// The body is read up to `OBSERVED_LIMIT`, every foreign image url it
    /// carries is recorded under `Observed::Image` and replaced by the handle
    /// the local server minted for it, and the rewritten bytes are forwarded.
    Foreign,
}

/// True when `value` is an absolute http or https url, which is what a
/// provider's image is pointed at by and what a handle is minted for.
fn is_foreign(value: &str) -> bool {
    let lowered = value.trim();
    (lowered.starts_with("http://") || lowered.starts_with("https://"))
        && reqwest::Url::parse(lowered).is_ok()
}

/// The largest body read whole so its names can be recorded.
pub const OBSERVED_LIMIT: usize = 1 << 20;

/// The most of a log file the relay delivers.
pub const TAIL_LIMIT: usize = 2 * 1024 * 1024;

/// The configuration sections the dashboard reads and writes by key.
static SECTIONS: &[&str] = &[
    "encoding",
    "network",
    "metadata",
    "trickplay",
    "livetv",
    "branding",
];

/// The cap every body-carrying route declared before the user settings
/// milestone, and the cap every one but the user image route declares now.
pub const BODY_LIMIT: usize = 64 * 1024;

/// The cap the user image and item image write routes declare.
pub const IMAGE_LIMIT: usize = 4 * 1024 * 1024;

/// The longest segment `shape` keeps.
pub const SHAPE_LITERAL: usize = 24;

/// What the relay does with a route's request body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Carried {
    /// No request body is forwarded.
    None,
    /// The request body is read up to `cap` and forwarded unchanged; a larger
    /// one is refused as `Refusal::BodyTooLarge` naming `cap`.
    Capped { cap: usize },
    /// The request body is read up to `cap` and forwarded base64-encoded,
    /// because Jellyfin's user image route takes base64; `cap` counts the bytes
    /// the browser sent.
    Encoded { cap: usize },
}

/// The stage a relayed route is admissible in, declared per entry rather than
/// derived from what the route does.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    /// Admissible only while the setup upstream is held.
    Setup,
    /// Admissible only while a session is held.
    Signed,
    /// Admissible while either is held.
    Either,
}

impl Stage {
    /// True when an entry declaring this stage may be asked for while the
    /// local server stands in `admits`; no stage admits `Admits::Login`.
    pub fn admits(self, admits: jellium_protocol::Admits) -> bool {
        if admits == jellium_protocol::Admits::Login {
            return false;
        }
        match self {
            Stage::Either => true,
            Stage::Setup => admits == jellium_protocol::Admits::Setup,
            Stage::Signed => admits == jellium_protocol::Admits::Signed,
        }
    }

    /// The one stage this entry admits, and `None` for `Either`; a refusal
    /// names it.
    pub fn only(self) -> Option<jellium_protocol::Admits> {
        match self {
            Stage::Either => None,
            Stage::Setup => Some(jellium_protocol::Admits::Setup),
            Stage::Signed => Some(jellium_protocol::Admits::Signed),
        }
    }
}

/// One Jellyfin route Jellium Web is relayed.
struct Route {
    verb: Verb,
    path: &'static [Segment],
    /// The stage this entry is admissible in.
    stage: Stage,
    /// What this route does upstream and what it carries to do it.
    access: Access,
    /// What this route does with its request body, declared per entry.
    carried: Carried,
    /// Where this entry admits a handle the local server minted.
    resolves: Resolves,
    /// `Payload::Manifest` on the master, variant and universal routes,
    /// `Payload::Observed` on the two listings whose names the local server
    /// admits later, `Payload::Tail` on the log body, and `Payload::Streamed`
    /// on every other route in `RELAYED`.
    payload: Payload,
}

impl Route {
    /// A route that reads upstream and carries no body.
    const fn read(verb: Verb, path: &'static [Segment]) -> Route {
        Route {
            verb,
            path,
            access: Access::Read,
            carried: Carried::None,
            payload: Payload::Streamed,
            resolves: Resolves::None,
            stage: Stage::Signed,
        }
    }

    /// A route that reads upstream and carries a body.
    const fn reads(verb: Verb, path: &'static [Segment]) -> Route {
        Route {
            access: Access::ReadWithBody,
            carried: Carried::Capped { cap: BODY_LIMIT },
            ..Route::read(verb, path)
        }
    }

    /// A route that writes upstream and carries no body.
    const fn write(verb: Verb, path: &'static [Segment]) -> Route {
        Route {
            access: Access::Write,
            ..Route::read(verb, path)
        }
    }

    /// A route that writes upstream and carries a body.
    const fn writes(verb: Verb, path: &'static [Segment]) -> Route {
        Route {
            access: Access::WriteWithBody,
            carried: Carried::Capped { cap: BODY_LIMIT },
            ..Route::read(verb, path)
        }
    }

    const fn manifest(self) -> Route {
        Route {
            payload: Payload::Manifest,
            ..self
        }
    }

    const fn observing(self, observed: Observed) -> Route {
        Route {
            payload: Payload::Observed(observed),
            ..self
        }
    }

    /// Reads and rewrites the response body's foreign image urls.
    const fn foreign(self) -> Route {
        Route {
            payload: Payload::Foreign,
            ..self
        }
    }

    /// Admits a minted handle where `resolves` names.
    const fn resolving(self, resolves: Resolves) -> Route {
        Route { resolves, ..self }
    }

    const fn tailed(self) -> Route {
        Route {
            payload: Payload::Tail,
            ..self
        }
    }

    /// Forwards the request body base64-encoded, under a cap of its own.
    const fn encoded(self, cap: usize) -> Route {
        Route {
            carried: Carried::Encoded { cap },
            ..self
        }
    }

    /// Admissible only during setup.
    const fn setup(self) -> Route {
        Route {
            stage: Stage::Setup,
            ..self
        }
    }

    /// Admissible in both stages.
    const fn either(self) -> Route {
        Route {
            stage: Stage::Either,
            ..self
        }
    }
}

/// The image kinds Jellium Web asks for, reads and writes.
static IMAGE_KINDS: &[&str] = &[
    "Primary", "Backdrop", "Thumb", "Logo", "Banner", "Art", "Chapter",
];

/// The item kinds the Jellyfin server offers a remote search for.
static SEARCHABLE: &[&str] = &[
    "Book",
    "BoxSet",
    "Movie",
    "MusicAlbum",
    "MusicArtist",
    "MusicVideo",
    "Person",
    "Series",
    "Trailer",
];

/// Every route Jellium Web calls, and no other.
static RELAYED: &[Route] = &[
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("Items"),
            Segment::Id,
            Segment::Literal("Images"),
            Segment::OneOf(IMAGE_KINDS),
            Segment::Number { max: 4 },
        ],
    ),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("Items"),
            Segment::Id,
            Segment::Literal("Images"),
        ],
    ),
    Route::writes(
        Verb::Post,
        &[
            Segment::Literal("Items"),
            Segment::Id,
            Segment::Literal("Images"),
            Segment::OneOf(IMAGE_KINDS),
        ],
    )
    .encoded(IMAGE_LIMIT),
    Route::writes(
        Verb::Post,
        &[
            Segment::Literal("Items"),
            Segment::Id,
            Segment::Literal("Images"),
            Segment::OneOf(IMAGE_KINDS),
            Segment::Number { max: 4 },
        ],
    )
    .encoded(IMAGE_LIMIT),
    Route::write(
        Verb::Delete,
        &[
            Segment::Literal("Items"),
            Segment::Id,
            Segment::Literal("Images"),
            Segment::OneOf(IMAGE_KINDS),
        ],
    ),
    Route::write(
        Verb::Delete,
        &[
            Segment::Literal("Items"),
            Segment::Id,
            Segment::Literal("Images"),
            Segment::OneOf(IMAGE_KINDS),
            Segment::Number { max: 4 },
        ],
    ),
    Route::write(
        Verb::Post,
        &[
            Segment::Literal("Items"),
            Segment::Id,
            Segment::Literal("Images"),
            Segment::OneOf(IMAGE_KINDS),
            Segment::Number { max: 4 },
            Segment::Literal("Index"),
        ],
    ),
    Route::writes(Verb::Post, &[Segment::Literal("Items"), Segment::Id]),
    Route::write(Verb::Delete, &[Segment::Literal("Items"), Segment::Id]),
    Route::write(
        Verb::Post,
        &[
            Segment::Literal("Items"),
            Segment::Id,
            Segment::Literal("ContentType"),
        ],
    ),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("Items"),
            Segment::Id,
            Segment::Literal("MetadataEditor"),
        ],
    ),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("Items"),
            Segment::Id,
            Segment::Literal("Similar"),
        ],
    ),
    Route::read(
        Verb::Get,
        &[Segment::Literal("Items"), Segment::Literal("Filters")],
    ),
    Route::read(
        Verb::Get,
        &[Segment::Literal("Items"), Segment::Literal("Latest")],
    ),
    Route::read(
        Verb::Get,
        &[Segment::Literal("Items"), Segment::Literal("Suggestions")],
    ),
    Route::reads(
        Verb::Post,
        &[
            Segment::Literal("Items"),
            Segment::Literal("RemoteSearch"),
            Segment::OneOf(SEARCHABLE),
        ],
    )
    .foreign(),
    Route::writes(
        Verb::Post,
        &[
            Segment::Literal("Items"),
            Segment::Literal("RemoteSearch"),
            Segment::Literal("Apply"),
            Segment::Id,
        ],
    )
    .resolving(Resolves::Body),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("Items"),
            Segment::Id,
            Segment::Literal("RemoteImages"),
        ],
    )
    .foreign(),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("Items"),
            Segment::Id,
            Segment::Literal("RemoteImages"),
            Segment::Literal("Providers"),
        ],
    ),
    Route::write(
        Verb::Post,
        &[
            Segment::Literal("Items"),
            Segment::Id,
            Segment::Literal("RemoteImages"),
            Segment::Literal("Download"),
        ],
    )
    .resolving(Resolves::Query),
    Route::read(Verb::Get, &[Segment::Literal("Genres")]),
    Route::read(Verb::Get, &[Segment::Literal("MusicGenres")]),
    Route::read(Verb::Get, &[Segment::Literal("Studios")]),
    Route::read(Verb::Get, &[Segment::Literal("Persons")]),
    Route::read(Verb::Get, &[Segment::Literal("Artists")]),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("Artists"),
            Segment::Literal("AlbumArtists"),
        ],
    ),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("Movies"),
            Segment::Literal("Recommendations"),
        ],
    ),
    Route::read(
        Verb::Get,
        &[Segment::Literal("Shows"), Segment::Literal("Upcoming")],
    ),
    Route::write(Verb::Post, &[Segment::Literal("Collections")]),
    Route::write(
        Verb::Post,
        &[
            Segment::Literal("Collections"),
            Segment::Id,
            Segment::Literal("Items"),
        ],
    ),
    Route::write(
        Verb::Delete,
        &[
            Segment::Literal("Collections"),
            Segment::Id,
            Segment::Literal("Items"),
        ],
    ),
    Route::writes(Verb::Post, &[Segment::Literal("Playlists")]),
    Route::read(Verb::Get, &[Segment::Literal("Playlists"), Segment::Id]),
    Route::writes(Verb::Post, &[Segment::Literal("Playlists"), Segment::Id]),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("Playlists"),
            Segment::Id,
            Segment::Literal("Items"),
        ],
    ),
    Route::write(
        Verb::Post,
        &[
            Segment::Literal("Playlists"),
            Segment::Id,
            Segment::Literal("Items"),
        ],
    ),
    Route::write(
        Verb::Delete,
        &[
            Segment::Literal("Playlists"),
            Segment::Id,
            Segment::Literal("Items"),
        ],
    ),
    Route::write(
        Verb::Post,
        &[
            Segment::Literal("Playlists"),
            Segment::Id,
            Segment::Literal("Items"),
            Segment::Id,
            Segment::Literal("Move"),
            Segment::Number { max: 6 },
        ],
    ),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("Playlists"),
            Segment::Id,
            Segment::Literal("Users"),
        ],
    ),
    Route::writes(
        Verb::Post,
        &[
            Segment::Literal("Playlists"),
            Segment::Id,
            Segment::Literal("Users"),
            Segment::Id,
        ],
    ),
    Route::write(
        Verb::Delete,
        &[
            Segment::Literal("Playlists"),
            Segment::Id,
            Segment::Literal("Users"),
            Segment::Id,
        ],
    ),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("Videos"),
            Segment::Id,
            Segment::Literal("Trickplay"),
            Segment::Number { max: 5 },
            Segment::Segmented {
                max: 10,
                extensions: &["jpg"],
            },
        ],
    ),
    Route::read(Verb::Get, &[Segment::Literal("Items")]),
    Route::read(Verb::Get, &[Segment::Literal("Items"), Segment::Id]),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("Items"),
            Segment::Id,
            Segment::Literal("Images"),
            Segment::OneOf(IMAGE_KINDS),
        ],
    ),
    Route::read(Verb::Get, &[Segment::Literal("UserViews")]),
    Route::read(
        Verb::Get,
        &[Segment::Literal("UserItems"), Segment::Literal("Resume")],
    ),
    Route::read(
        Verb::Get,
        &[Segment::Literal("Shows"), Segment::Literal("NextUp")],
    ),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("Shows"),
            Segment::Id,
            Segment::Literal("Seasons"),
        ],
    ),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("Shows"),
            Segment::Id,
            Segment::Literal("Episodes"),
        ],
    ),
    Route::write(
        Verb::Post,
        &[Segment::Literal("UserPlayedItems"), Segment::Id],
    ),
    Route::write(
        Verb::Delete,
        &[Segment::Literal("UserPlayedItems"), Segment::Id],
    ),
    Route::write(
        Verb::Post,
        &[Segment::Literal("UserFavoriteItems"), Segment::Id],
    ),
    Route::write(
        Verb::Delete,
        &[Segment::Literal("UserFavoriteItems"), Segment::Id],
    ),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("Videos"),
            Segment::Id,
            Segment::Literal("stream"),
        ],
    ),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("Videos"),
            Segment::Id,
            Segment::Suffixed {
                stem: "stream",
                extensions: STREAM_CONTAINERS,
            },
        ],
    ),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("Videos"),
            Segment::Id,
            Segment::Suffixed {
                stem: "master",
                extensions: &["m3u8"],
            },
        ],
    )
    .manifest(),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("Videos"),
            Segment::Id,
            Segment::Suffixed {
                stem: "main",
                extensions: &["m3u8"],
            },
        ],
    )
    .manifest(),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("Videos"),
            Segment::Id,
            Segment::Literal("hls1"),
            Segment::Token { max: 64 },
            Segment::Segmented {
                max: 10,
                extensions: SEGMENT_CONTAINERS,
            },
        ],
    ),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("Videos"),
            Segment::Id,
            Segment::Literal("hls1"),
            Segment::Token { max: 64 },
            Segment::Suffixed {
                stem: INITIALIZATION,
                extensions: SEGMENT_CONTAINERS,
            },
        ],
    ),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("Videos"),
            Segment::Id,
            Segment::Token { max: 64 },
            Segment::Literal("Subtitles"),
            Segment::Number { max: 4 },
            Segment::Suffixed {
                stem: "Stream",
                extensions: &["vtt"],
            },
        ],
    ),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("Videos"),
            Segment::Id,
            Segment::Token { max: 64 },
            Segment::Literal("Subtitles"),
            Segment::Number { max: 4 },
            Segment::Number { max: 20 },
            Segment::Suffixed {
                stem: "Stream",
                extensions: &["vtt"],
            },
        ],
    ),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("Audio"),
            Segment::Id,
            Segment::Literal("universal"),
        ],
    )
    .manifest(),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("Audio"),
            Segment::Id,
            Segment::Literal("stream"),
        ],
    ),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("Audio"),
            Segment::Id,
            Segment::Suffixed {
                stem: "stream",
                extensions: STREAM_CONTAINERS,
            },
        ],
    ),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("Audio"),
            Segment::Id,
            Segment::Suffixed {
                stem: "master",
                extensions: &["m3u8"],
            },
        ],
    )
    .manifest(),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("Audio"),
            Segment::Id,
            Segment::Suffixed {
                stem: "main",
                extensions: &["m3u8"],
            },
        ],
    )
    .manifest(),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("Audio"),
            Segment::Id,
            Segment::Literal("hls1"),
            Segment::Token { max: 64 },
            Segment::Segmented {
                max: 10,
                extensions: SEGMENT_CONTAINERS,
            },
        ],
    ),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("Audio"),
            Segment::Id,
            Segment::Literal("hls1"),
            Segment::Token { max: 64 },
            Segment::Suffixed {
                stem: INITIALIZATION,
                extensions: SEGMENT_CONTAINERS,
            },
        ],
    ),
    Route::read(
        Verb::Get,
        &[
            Segment::OneOf(MIX_PARENTS),
            Segment::Id,
            Segment::Literal("InstantMix"),
        ],
    ),
    Route::read(
        Verb::Get,
        &[Segment::Literal("LiveTv"), Segment::Literal("GuideInfo")],
    ),
    Route::read(
        Verb::Get,
        &[Segment::Literal("LiveTv"), Segment::Literal("Channels")],
    ),
    Route::reads(
        Verb::Post,
        &[Segment::Literal("LiveTv"), Segment::Literal("Programs")],
    ),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("LiveTv"),
            Segment::Literal("Programs"),
            Segment::Token { max: 64 },
        ],
    ),
    Route::read(
        Verb::Get,
        &[Segment::Literal("LiveTv"), Segment::Literal("Recordings")],
    ),
    Route::write(
        Verb::Delete,
        &[
            Segment::Literal("LiveTv"),
            Segment::Literal("Recordings"),
            Segment::Id,
        ],
    ),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("LiveTv"),
            Segment::Literal("Timers"),
            Segment::Literal("Defaults"),
        ],
    ),
    Route::read(
        Verb::Get,
        &[Segment::Literal("LiveTv"), Segment::Literal("Timers")],
    ),
    Route::writes(
        Verb::Post,
        &[Segment::Literal("LiveTv"), Segment::Literal("Timers")],
    ),
    Route::writes(
        Verb::Post,
        &[
            Segment::Literal("LiveTv"),
            Segment::Literal("Timers"),
            Segment::Token { max: 64 },
        ],
    ),
    Route::write(
        Verb::Delete,
        &[
            Segment::Literal("LiveTv"),
            Segment::Literal("Timers"),
            Segment::Token { max: 64 },
        ],
    ),
    Route::read(
        Verb::Get,
        &[Segment::Literal("LiveTv"), Segment::Literal("SeriesTimers")],
    ),
    Route::writes(
        Verb::Post,
        &[Segment::Literal("LiveTv"), Segment::Literal("SeriesTimers")],
    ),
    Route::writes(
        Verb::Post,
        &[
            Segment::Literal("LiveTv"),
            Segment::Literal("SeriesTimers"),
            Segment::Token { max: 64 },
        ],
    ),
    Route::write(
        Verb::Delete,
        &[
            Segment::Literal("LiveTv"),
            Segment::Literal("SeriesTimers"),
            Segment::Token { max: 64 },
        ],
    ),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("LiveTv"),
            Segment::Literal("LiveStreamFiles"),
            Segment::Token { max: 64 },
            Segment::Suffixed {
                stem: "stream",
                extensions: STREAM_CONTAINERS,
            },
        ],
    ),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("LiveTv"),
            Segment::Literal("LiveRecordings"),
            Segment::Token { max: 64 },
            Segment::Literal("stream"),
        ],
    ),
    Route::read(
        Verb::Get,
        &[Segment::Literal("System"), Segment::Literal("Info")],
    ),
    Route::write(
        Verb::Post,
        &[Segment::Literal("System"), Segment::Literal("Restart")],
    ),
    Route::write(
        Verb::Post,
        &[Segment::Literal("System"), Segment::Literal("Shutdown")],
    ),
    Route::read(
        Verb::Get,
        &[Segment::Literal("System"), Segment::Literal("Logs")],
    ),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("System"),
            Segment::Literal("Logs"),
            Segment::Literal("Log"),
        ],
    )
    .tailed(),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("System"),
            Segment::Literal("ActivityLog"),
            Segment::Literal("Entries"),
        ],
    ),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("System"),
            Segment::Literal("Configuration"),
        ],
    ),
    Route::writes(
        Verb::Post,
        &[
            Segment::Literal("System"),
            Segment::Literal("Configuration"),
        ],
    ),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("System"),
            Segment::Literal("Configuration"),
            Segment::OneOf(SECTIONS),
        ],
    ),
    Route::writes(
        Verb::Post,
        &[
            Segment::Literal("System"),
            Segment::Literal("Configuration"),
            Segment::OneOf(SECTIONS),
        ],
    ),
    Route::read(Verb::Get, &[Segment::Literal("Users")]),
    Route::read(Verb::Get, &[Segment::Literal("Users"), Segment::Id]),
    Route::writes(Verb::Post, &[Segment::Literal("Users"), Segment::Id]),
    Route::write(Verb::Delete, &[Segment::Literal("Users"), Segment::Id]),
    Route::writes(
        Verb::Post,
        &[Segment::Literal("Users"), Segment::Literal("New")],
    ),
    Route::writes(
        Verb::Post,
        &[
            Segment::Literal("Users"),
            Segment::Id,
            Segment::Literal("Policy"),
        ],
    ),
    Route::writes(
        Verb::Post,
        &[
            Segment::Literal("Users"),
            Segment::Id,
            Segment::Literal("Configuration"),
        ],
    ),
    Route::writes(
        Verb::Post,
        &[
            Segment::Literal("Users"),
            Segment::Id,
            Segment::Literal("Password"),
        ],
    ),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("Users"),
            Segment::Id,
            Segment::Literal("Images"),
            Segment::Literal("Primary"),
        ],
    ),
    Route::writes(
        Verb::Post,
        &[
            Segment::Literal("Users"),
            Segment::Id,
            Segment::Literal("Images"),
            Segment::Literal("Primary"),
        ],
    )
    .encoded(IMAGE_LIMIT),
    Route::write(
        Verb::Delete,
        &[
            Segment::Literal("Users"),
            Segment::Id,
            Segment::Literal("Images"),
            Segment::Literal("Primary"),
        ],
    ),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("Library"),
            Segment::Literal("VirtualFolders"),
        ],
    )
    .either(),
    Route::writes(
        Verb::Post,
        &[
            Segment::Literal("Library"),
            Segment::Literal("VirtualFolders"),
        ],
    )
    .either(),
    Route::write(
        Verb::Delete,
        &[
            Segment::Literal("Library"),
            Segment::Literal("VirtualFolders"),
        ],
    )
    .either(),
    Route::write(
        Verb::Post,
        &[
            Segment::Literal("Library"),
            Segment::Literal("VirtualFolders"),
            Segment::Literal("Name"),
        ],
    )
    .either(),
    Route::writes(
        Verb::Post,
        &[
            Segment::Literal("Library"),
            Segment::Literal("VirtualFolders"),
            Segment::Literal("Paths"),
        ],
    )
    .either(),
    Route::writes(
        Verb::Post,
        &[
            Segment::Literal("Library"),
            Segment::Literal("VirtualFolders"),
            Segment::Literal("Paths"),
            Segment::Literal("Update"),
        ],
    )
    .either(),
    Route::write(
        Verb::Delete,
        &[
            Segment::Literal("Library"),
            Segment::Literal("VirtualFolders"),
            Segment::Literal("Paths"),
        ],
    )
    .either(),
    Route::writes(
        Verb::Post,
        &[
            Segment::Literal("Library"),
            Segment::Literal("VirtualFolders"),
            Segment::Literal("LibraryOptions"),
        ],
    ),
    Route::write(
        Verb::Post,
        &[Segment::Literal("Library"), Segment::Literal("Refresh")],
    ),
    Route::write(
        Verb::Post,
        &[
            Segment::Literal("Items"),
            Segment::Id,
            Segment::Literal("Refresh"),
        ],
    ),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("Libraries"),
            Segment::Literal("AvailableOptions"),
        ],
    ),
    Route::read(
        Verb::Get,
        &[Segment::Literal("Environment"), Segment::Literal("Drives")],
    )
    .either(),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("Environment"),
            Segment::Literal("DirectoryContents"),
        ],
    )
    .either(),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("Environment"),
            Segment::Literal("DefaultDirectoryBrowser"),
        ],
    )
    .setup(),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("Environment"),
            Segment::Literal("ParentPath"),
        ],
    )
    .setup(),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("Localization"),
            Segment::Literal("Options"),
        ],
    )
    .setup(),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("Localization"),
            Segment::Literal("Cultures"),
        ],
    )
    .either(),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("Localization"),
            Segment::Literal("Countries"),
        ],
    )
    .setup(),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("DisplayPreferences"),
            Segment::Literal("usersettings"),
        ],
    ),
    Route::writes(
        Verb::Post,
        &[
            Segment::Literal("DisplayPreferences"),
            Segment::Literal("usersettings"),
        ],
    ),
    Route::write(
        Verb::Post,
        &[
            Segment::Literal("QuickConnect"),
            Segment::Literal("Authorize"),
        ],
    ),
    Route::read(Verb::Get, &[Segment::Literal("ScheduledTasks")]),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("ScheduledTasks"),
            Segment::Token { max: 64 },
        ],
    ),
    Route::write(
        Verb::Post,
        &[
            Segment::Literal("ScheduledTasks"),
            Segment::Literal("Running"),
            Segment::Token { max: 64 },
        ],
    ),
    Route::write(
        Verb::Delete,
        &[
            Segment::Literal("ScheduledTasks"),
            Segment::Literal("Running"),
            Segment::Token { max: 64 },
        ],
    ),
    Route::writes(
        Verb::Post,
        &[
            Segment::Literal("ScheduledTasks"),
            Segment::Token { max: 64 },
            Segment::Literal("Triggers"),
        ],
    ),
    Route::read(Verb::Get, &[Segment::Literal("Plugins")]),
    Route::write(Verb::Delete, &[Segment::Literal("Plugins"), Segment::Id]),
    Route::write(
        Verb::Delete,
        &[
            Segment::Literal("Plugins"),
            Segment::Id,
            Segment::Version { max: 32 },
        ],
    ),
    Route::write(
        Verb::Post,
        &[
            Segment::Literal("Plugins"),
            Segment::Id,
            Segment::Version { max: 32 },
            Segment::Literal("Enable"),
        ],
    ),
    Route::write(
        Verb::Post,
        &[
            Segment::Literal("Plugins"),
            Segment::Id,
            Segment::Version { max: 32 },
            Segment::Literal("Disable"),
        ],
    ),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("Plugins"),
            Segment::Id,
            Segment::Version { max: 32 },
            Segment::Literal("Image"),
        ],
    ),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("Plugins"),
            Segment::Id,
            Segment::Literal("Configuration"),
        ],
    ),
    Route::writes(
        Verb::Post,
        &[
            Segment::Literal("Plugins"),
            Segment::Id,
            Segment::Literal("Configuration"),
        ],
    ),
    Route::read(Verb::Get, &[Segment::Literal("Packages")]).observing(Observed::Package),
    Route::write(
        Verb::Post,
        &[
            Segment::Literal("Packages"),
            Segment::Literal("Installed"),
            Segment::Seen(Observed::Package),
        ],
    ),
    Route::write(
        Verb::Delete,
        &[
            Segment::Literal("Packages"),
            Segment::Literal("Installing"),
            Segment::Id,
        ],
    ),
    Route::read(Verb::Get, &[Segment::Literal("Repositories")]),
    Route::writes(Verb::Post, &[Segment::Literal("Repositories")]),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("web"),
            Segment::Literal("ConfigurationPages"),
        ],
    )
    .observing(Observed::Page),
    Route::read(Verb::Get, &[Segment::Literal("Devices")]),
    Route::write(Verb::Delete, &[Segment::Literal("Devices")]),
    Route::writes(
        Verb::Post,
        &[Segment::Literal("Devices"), Segment::Literal("Options")],
    ),
    Route::read(
        Verb::Get,
        &[Segment::Literal("Auth"), Segment::Literal("Keys")],
    ),
    Route::write(
        Verb::Post,
        &[Segment::Literal("Auth"), Segment::Literal("Keys")],
    ),
    Route::write(
        Verb::Delete,
        &[
            Segment::Literal("Auth"),
            Segment::Literal("Keys"),
            Segment::Token { max: 64 },
        ],
    ),
    Route::writes(
        Verb::Post,
        &[Segment::Literal("LiveTv"), Segment::Literal("TunerHosts")],
    ),
    Route::write(
        Verb::Delete,
        &[Segment::Literal("LiveTv"), Segment::Literal("TunerHosts")],
    ),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("LiveTv"),
            Segment::Literal("TunerHosts"),
            Segment::Literal("Types"),
        ],
    ),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("LiveTv"),
            Segment::Literal("Tuners"),
            Segment::Literal("Discover"),
        ],
    ),
    Route::write(
        Verb::Post,
        &[
            Segment::Literal("LiveTv"),
            Segment::Literal("Tuners"),
            Segment::Token { max: 64 },
            Segment::Literal("Reset"),
        ],
    ),
    Route::writes(
        Verb::Post,
        &[
            Segment::Literal("LiveTv"),
            Segment::Literal("ListingProviders"),
        ],
    ),
    Route::write(
        Verb::Delete,
        &[
            Segment::Literal("LiveTv"),
            Segment::Literal("ListingProviders"),
        ],
    ),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("LiveTv"),
            Segment::Literal("ListingProviders"),
            Segment::Literal("Default"),
        ],
    ),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("LiveTv"),
            Segment::Literal("ListingProviders"),
            Segment::Literal("Lineups"),
        ],
    ),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("LiveTv"),
            Segment::Literal("ListingProviders"),
            Segment::Literal("SchedulesDirect"),
            Segment::Literal("Countries"),
        ],
    ),
    Route::read(
        Verb::Get,
        &[
            Segment::Literal("LiveTv"),
            Segment::Literal("ChannelMappingOptions"),
        ],
    ),
    Route::writes(
        Verb::Post,
        &[
            Segment::Literal("LiveTv"),
            Segment::Literal("ChannelMappings"),
        ],
    ),
];

impl Segment {
    /// The spelling `decoded` is forwarded in when it matches, or `None`.
    fn admit(&self, decoded: &str, seen: &Seen) -> Option<String> {
        match self {
            Segment::Literal(literal) => decoded
                .eq_ignore_ascii_case(literal)
                .then(|| literal.to_string()),
            Segment::OneOf(options) => options
                .iter()
                .find(|option| decoded.eq_ignore_ascii_case(option))
                .map(|option| option.to_string()),
            Segment::Id => decoded.parse::<Uuid>().ok().map(|id| id.to_string()),
            Segment::Token { max } => (token(decoded, *max)).then(|| decoded.to_string()),
            Segment::Number { max } => (number(decoded, *max)).then(|| decoded.to_string()),
            Segment::Suffixed { stem, extensions } => {
                let (head, tail) = decoded.rsplit_once('.')?;
                head.eq_ignore_ascii_case(stem).then_some(())?;
                extensions
                    .iter()
                    .find(|extension| tail.eq_ignore_ascii_case(extension))
                    .map(|extension| format!("{stem}.{extension}"))
            }
            Segment::Segmented { max, extensions } => {
                let (head, tail) = decoded.rsplit_once('.')?;
                number(head, *max).then_some(())?;
                extensions
                    .iter()
                    .find(|extension| tail.eq_ignore_ascii_case(extension))
                    .map(|extension| format!("{head}.{extension}"))
            }
            Segment::Version { max } => version(decoded, *max).then(|| decoded.to_string()),
            Segment::Seen(observed) => seen.holds(*observed, decoded).then(|| decoded.to_string()),
        }
    }
}

/// True for 1..=`max` bytes of ascii digits and `.`.
fn version(decoded: &str, max: usize) -> bool {
    !decoded.is_empty()
        && decoded.len() <= max
        && decoded
            .bytes()
            .all(|byte| byte.is_ascii_digit() || byte == b'.')
}

/// True for 1..=`max` bytes of ascii letters, digits, `-` and `_`.
fn token(decoded: &str, max: usize) -> bool {
    !decoded.is_empty()
        && decoded.len() <= max
        && decoded
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_')
}

/// True for 1..=`max` ascii digits.
fn number(decoded: &str, max: usize) -> bool {
    !decoded.is_empty() && decoded.len() <= max && decoded.bytes().all(|byte| byte.is_ascii_digit())
}

/// `query` with every name in `STRIPPED` removed, each remaining pair kept in
/// the spelling it arrived in.
fn stripped(query: Option<&str>) -> Option<String> {
    let query = query?;
    let kept = query
        .split('&')
        .filter(|pair| !pair.is_empty())
        .filter(|pair| {
            let name = pair.split_once('=').map_or(*pair, |(name, _)| name);
            let name = percent_encoding::percent_decode_str(name)
                .decode_utf8()
                .map_or_else(|_| name.to_string(), |decoded| decoded.into_owned());
            !STRIPPED
                .iter()
                .any(|banned| name.eq_ignore_ascii_case(banned))
        })
        .collect::<Vec<_>>()
        .join("&");
    (!kept.is_empty()).then_some(kept)
}

impl Route {
    fn admit(&self, method: &Method, decoded: &[String], seen: &Seen) -> Option<Vec<String>> {
        if self.verb.method() != *method || self.path.len() != decoded.len() {
            return None;
        }
        self.path
            .iter()
            .zip(decoded)
            .map(|(segment, decoded)| segment.admit(decoded, seen))
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
    payload: Payload,
    access: Access,
    carried: Carried,
    resolves: Resolves,
    stage: Stage,
}

impl Target {
    /// Splits `path` on `/`, drops empty segments, percent-decodes each, and
    /// returns the first route in `RELAYED` whose method and segments all
    /// match.
    /// A segment that is not utf-8 once decoded matches nothing, and so does
    /// one holding a decoded `/`, `?` or `#`, since no `Segment` admits it.
    /// A `Segment::Seen` matches only a name `seen` holds.
    pub fn admit(method: &Method, path: &str, seen: &Seen) -> Option<Target> {
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
            .find_map(|route| Some((route.admit(method, &decoded, seen)?, route)))
            .map(|(segments, route)| Target {
                method: method.clone(),
                segments,
                payload: route.payload,
                access: route.access,
                carried: route.carried,
                resolves: route.resolves,
                stage: route.stage,
            })
    }

    pub fn method(&self) -> Method {
        self.method.clone()
    }

    /// True when this route only reads upstream, declared per entry rather
    /// than derived from the method.
    pub fn read_only(&self) -> bool {
        self.access.read_only()
    }

    /// What the relay does with this route's request body, declared per entry.
    pub fn body(&self) -> Carried {
        self.carried
    }

    /// What the relay does with this route's response body.
    pub fn payload(&self) -> Payload {
        self.payload
    }

    /// Where this route's entry admits a minted handle.
    pub fn resolves(&self) -> Resolves {
        self.resolves
    }

    /// The stage this route's entry declares.
    pub fn stage(&self) -> Stage {
        self.stage
    }

    /// `base` with this target's segments appended and `query` set with every
    /// name in `STRIPPED` removed, both encoded by the url parser rather than
    /// pasted into text it re-reads.
    pub fn url(&self, base: &reqwest::Url, query: Option<&str>) -> reqwest::Url {
        let mut url = base.clone();
        {
            let mut segments = url
                .path_segments_mut()
                .expect("the upstream base is a hierarchical url");
            segments.pop_if_empty();
            segments.extend(&self.segments);
        }
        url.set_query(stripped(query).as_deref());
        url
    }

    /// The relay path this target is reached at: `RELAY_PREFIX`, the admitted
    /// segments, and `query` with every name in `STRIPPED` removed.
    pub fn path(&self, query: Option<&str>) -> String {
        let prefix = jellium_protocol::RELAY_PREFIX;
        let segments = self.segments.join("/");
        match stripped(query) {
            Some(query) => format!("{prefix}/{segments}?{query}"),
            None => format!("{prefix}/{segments}"),
        }
    }
}

/// `path` named by its shape: the query dropped, and every segment that parses
/// as a uuid or runs longer than `SHAPE_LITERAL` bytes replaced by `*`.
/// It is how a live source the table refuses is reported, so no opaque id and
/// no token reaches the browser.
pub fn shape(path: &str) -> String {
    let path = path.split(['?', '#']).next().unwrap_or_default();
    let shaped = path
        .split('/')
        .filter(|segment| !segment.is_empty())
        .map(|segment| {
            if segment.parse::<Uuid>().is_ok() || segment.len() > SHAPE_LITERAL {
                "*".to_string()
            } else {
                segment.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("/");
    format!("/{shaped}")
}

#[cfg(test)]
mod tests {
    use super::*;

    const ID: &str = "0191b2f0-1c3d-4e5f-8a9b-0c1d2e3f4a5b";

    #[test]
    fn an_encoded_query_delimiter_is_not_relayed() {
        assert!(Target::admit(&Method::GET, "Items%3Fx=1", &Seen::new()).is_none());
    }

    #[test]
    fn an_encoded_fragment_delimiter_is_not_relayed() {
        assert!(Target::admit(&Method::GET, "Items%23x", &Seen::new()).is_none());
    }

    #[test]
    fn an_encoded_path_delimiter_is_not_relayed() {
        assert!(Target::admit(&Method::GET, "Items%2FBad", &Seen::new()).is_none());
    }

    #[test]
    fn a_lowercase_percent_escape_is_not_relayed() {
        assert!(Target::admit(&Method::GET, "Items%2fBad", &Seen::new()).is_none());
    }

    #[test]
    fn a_credential_minting_path_outside_the_table_is_not_relayed() {
        assert!(Target::admit(&Method::GET, "QuickConnect/Initiate", &Seen::new()).is_none());
        assert!(Target::admit(&Method::POST, "QuickConnect/Initiate", &Seen::new()).is_none());
        assert!(Target::admit(&Method::GET, "QuickConnect/Connect", &Seen::new()).is_none());
        assert!(
            Target::admit(
                &Method::POST,
                "Users/AuthenticateWithQuickConnect",
                &Seen::new()
            )
            .is_none()
        );
        assert!(Target::admit(&Method::POST, "Startup/User", &Seen::new()).is_none());
    }

    #[test]
    fn a_sign_in_path_is_not_relayed() {
        assert!(Target::admit(&Method::POST, "Users/AuthenticateByName", &Seen::new()).is_none());
    }

    #[test]
    fn the_search_hints_endpoint_is_not_relayed() {
        for path in ["Search/Hints", "Items/Search/Hints"] {
            assert!(
                Target::admit(&Method::GET, path, &Seen::new()).is_none(),
                "{path}"
            );
        }
    }

    #[test]
    fn a_revoke_path_is_not_relayed() {
        assert!(Target::admit(&Method::POST, "Sessions/Logout", &Seen::new()).is_none());
    }

    #[test]
    fn a_write_method_on_a_read_route_is_not_relayed() {
        let path = format!("Items/{ID}/Similar");
        assert!(Target::admit(&Method::GET, &path, &Seen::new()).is_some());
        assert!(Target::admit(&Method::POST, &path, &Seen::new()).is_none());
        assert!(Target::admit(&Method::DELETE, &path, &Seen::new()).is_none());
    }

    #[test]
    fn the_metadata_manager_saves_and_deletes_one_item_by_id() {
        let path = format!("Items/{ID}");
        assert!(Target::admit(&Method::GET, &path, &Seen::new()).is_some());
        let save = Target::admit(&Method::POST, &path, &Seen::new()).expect("the save route");
        assert!(!save.read_only());
        let delete = Target::admit(&Method::DELETE, &path, &Seen::new()).expect("the delete route");
        assert!(!delete.read_only());
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
                Target::admit(&method, &path, &Seen::new()).is_some(),
                "{method} {path} was not relayed"
            );
        }
    }

    #[test]
    fn an_admitted_target_forwards_the_table_spelling_and_a_canonical_id() {
        let target = Target::admit(
            &Method::GET,
            "items/0191B2F0-1C3D-4E5F-8A9B-0C1D2E3F4A5B",
            &Seen::new(),
        )
        .expect("a relayed route");
        assert_eq!(target.segments, vec!["Items".to_string(), ID.to_string()]);
    }

    #[test]
    fn every_playback_route_jellium_web_calls_is_relayed() {
        let calls = [
            format!("Videos/{ID}/stream"),
            format!("Videos/{ID}/stream.mp4"),
            format!("Videos/{ID}/master.m3u8"),
            format!("Videos/{ID}/main.m3u8"),
            format!("Videos/{ID}/hls1/main/0.ts"),
            format!("Videos/{ID}/{ID}/Subtitles/2/Stream.vtt"),
            format!("Videos/{ID}/{ID}/Subtitles/2/0/Stream.vtt"),
            format!("Audio/{ID}/universal"),
            format!("Audio/{ID}/stream"),
            format!("Audio/{ID}/stream.mp3"),
            format!("Audio/{ID}/master.m3u8"),
            format!("Audio/{ID}/main.m3u8"),
            format!("Audio/{ID}/hls1/main/1.aac"),
            format!("Items/{ID}/InstantMix"),
            format!("Albums/{ID}/InstantMix"),
            format!("Artists/{ID}/InstantMix"),
        ];
        for path in calls {
            assert!(
                Target::admit(&Method::GET, &path, &Seen::new()).is_some(),
                "GET {path} was not relayed"
            );
        }
    }

    #[test]
    fn the_initialization_segment_is_relayed_under_hls1_and_nowhere_else() {
        for path in [
            format!("Videos/{ID}/hls1/main/-1.mp4"),
            format!("Audio/{ID}/hls1/main/-1.mp4"),
        ] {
            assert!(
                Target::admit(&Method::GET, &path, &Seen::new()).is_some(),
                "GET {path} was not relayed"
            );
        }
        for path in [
            format!("Videos/{ID}/Trickplay/320/-1.jpg"),
            format!("Videos/{ID}/hls1/main/-2.mp4"),
            format!("Videos/{ID}/hls1/main/-1.exe"),
        ] {
            assert!(
                Target::admit(&Method::GET, &path, &Seen::new()).is_none(),
                "GET {path} was relayed"
            );
        }
    }

    #[test]
    fn every_route_that_can_answer_a_manifest_is_read_rather_than_streamed() {
        let manifests = [
            format!("Videos/{ID}/master.m3u8"),
            format!("Videos/{ID}/main.m3u8"),
            format!("Audio/{ID}/universal"),
            format!("Audio/{ID}/master.m3u8"),
            format!("Audio/{ID}/main.m3u8"),
        ];
        for path in manifests {
            let target = Target::admit(&Method::GET, &path, &Seen::new()).expect("a relayed route");
            assert_eq!(target.payload(), Payload::Manifest, "GET {path}");
        }
    }

    #[test]
    fn a_segment_route_and_an_item_route_are_streamed() {
        for path in [format!("Videos/{ID}/hls1/main/0.ts"), format!("Items/{ID}")] {
            let target = Target::admit(&Method::GET, &path, &Seen::new()).expect("a relayed route");
            assert_eq!(target.payload(), Payload::Streamed, "GET {path}");
        }
    }

    #[test]
    fn an_unknown_container_is_not_relayed() {
        assert!(
            Target::admit(
                &Method::GET,
                &format!("Videos/{ID}/stream.exe"),
                &Seen::new()
            )
            .is_none()
        );
        assert!(
            Target::admit(
                &Method::GET,
                &format!("Videos/{ID}/hls1/main/0.exe"),
                &Seen::new()
            )
            .is_none()
        );
    }

    #[test]
    fn a_token_segment_outside_its_character_class_is_not_relayed() {
        assert!(
            Target::admit(
                &Method::GET,
                &format!("Videos/{ID}/hls1/ma.in/0.ts"),
                &Seen::new()
            )
            .is_none()
        );
        assert!(
            Target::admit(
                &Method::GET,
                &format!("Videos/{ID}/hls1/{}/0.ts", "a".repeat(65)),
                &Seen::new()
            )
            .is_none()
        );
    }

    #[test]
    fn a_number_segment_admits_only_digits() {
        assert!(
            Target::admit(
                &Method::GET,
                &format!("Videos/{ID}/{ID}/Subtitles/x/Stream.vtt"),
                &Seen::new()
            )
            .is_none()
        );
    }

    #[test]
    fn a_credential_query_parameter_never_reaches_the_server() {
        let base = reqwest::Url::parse("https://example.test").expect("base");
        let target = Target::admit(&Method::GET, "Items", &Seen::new()).expect("route");
        let url = target.url(
            &base,
            Some("api_key=secret&ApiKey=secret&X-Emby-Token=secret&ids=1"),
        );
        assert_eq!(url.query(), Some("ids=1"));
        assert_eq!(target.path(Some("api_key=secret")), "/jellyfin/Items");
    }

    #[test]
    fn a_relay_path_carries_the_prefix_and_the_kept_query() {
        let target = Target::admit(
            &Method::GET,
            &format!("Videos/{ID}/master.m3u8"),
            &Seen::new(),
        )
        .expect("route");
        assert_eq!(
            target.path(Some("mediaSourceId=abc")),
            format!("/jellyfin/Videos/{ID}/master.m3u8?mediaSourceId=abc")
        );
    }

    #[test]
    fn no_route_carries_a_free_form_segment() {
        for route in RELAYED {
            for segment in route.path {
                match segment {
                    Segment::Literal(_) | Segment::OneOf(_) | Segment::Id => {}
                    Segment::Token { max } | Segment::Number { max } => assert!(*max > 0),
                    Segment::Suffixed { extensions, .. } => assert!(!extensions.is_empty()),
                    Segment::Segmented { max, extensions } => {
                        assert!(*max > 0 && !extensions.is_empty());
                    }
                    Segment::Version { max } => assert!(*max > 0),
                    Segment::Seen(_) => {}
                }
            }
        }
    }

    #[test]
    fn a_query_carrying_a_fragment_delimiter_is_encoded() {
        let base = reqwest::Url::parse("https://example.test").expect("base");
        let target = Target::admit(&Method::GET, "Items", &Seen::new()).expect("route");
        let url = target.url(&base, Some("term=a#b"));
        assert!(!url.as_str().contains('#'));
        assert_eq!(url.query(), Some("term=a%23b"));
    }

    #[test]
    fn every_live_tv_route_jellium_web_calls_is_relayed() {
        let calls = [
            (Method::GET, "LiveTv/GuideInfo".to_string()),
            (Method::GET, "LiveTv/Channels".to_string()),
            (Method::POST, "LiveTv/Programs".to_string()),
            (Method::GET, "LiveTv/Programs/prog-1".to_string()),
            (Method::GET, "LiveTv/Recordings".to_string()),
            (Method::DELETE, format!("LiveTv/Recordings/{ID}")),
            (Method::GET, "LiveTv/Timers/Defaults".to_string()),
            (Method::GET, "LiveTv/Timers".to_string()),
            (Method::POST, "LiveTv/Timers".to_string()),
            (Method::POST, "LiveTv/Timers/timer-1".to_string()),
            (Method::DELETE, "LiveTv/Timers/timer-1".to_string()),
            (Method::GET, "LiveTv/SeriesTimers".to_string()),
            (Method::POST, "LiveTv/SeriesTimers".to_string()),
            (Method::POST, "LiveTv/SeriesTimers/series-1".to_string()),
            (Method::DELETE, "LiveTv/SeriesTimers/series-1".to_string()),
            (
                Method::GET,
                "LiveTv/LiveStreamFiles/stream-1/stream.mp4".to_string(),
            ),
            (
                Method::GET,
                "LiveTv/LiveRecordings/rec-1/stream".to_string(),
            ),
        ];
        for (method, path) in calls {
            assert!(
                Target::admit(&method, &path, &Seen::new()).is_some(),
                "{method} {path} was not relayed"
            );
        }
    }

    #[test]
    fn a_live_tv_route_outside_the_table_is_not_relayed() {
        assert!(Target::admit(&Method::GET, "LiveTv/Info", &Seen::new()).is_none());
        assert!(Target::admit(&Method::GET, "LiveTv/Tuners", &Seen::new()).is_none());
        assert!(Target::admit(&Method::GET, "LiveTv/Recordings/Folders", &Seen::new()).is_none());
        assert!(
            Target::admit(
                &Method::GET,
                "LiveTv/LiveStreamFiles/s/stream.exe",
                &Seen::new()
            )
            .is_none()
        );
    }

    #[test]
    fn every_route_declares_whether_it_reads_or_writes() {
        for route in RELAYED {
            if route.verb == Verb::Get {
                assert!(
                    route.access.read_only(),
                    "a GET route declared itself a write"
                );
            }
            if !route.access.read_only() {
                assert_ne!(route.verb, Verb::Get, "a route declared a write over GET");
            }
        }
    }

    #[test]
    fn a_posted_program_query_is_declared_read_only() {
        let target =
            Target::admit(&Method::POST, "LiveTv/Programs", &Seen::new()).expect("a relayed route");
        assert!(target.read_only());
        assert_eq!(target.body(), Carried::Capped { cap: BODY_LIMIT });
    }

    #[test]
    fn every_body_carrying_route_but_the_user_image_route_caps_at_the_old_limit() {
        for route in RELAYED {
            match route.carried {
                Carried::Capped { cap } => assert_eq!(cap, BODY_LIMIT),
                Carried::Encoded { cap } => assert_eq!(cap, IMAGE_LIMIT),
                Carried::None => {}
            }
        }
    }

    #[test]
    fn the_user_image_route_declares_the_image_cap_and_is_forwarded_encoded() {
        let target = Target::admit(
            &Method::POST,
            &format!("Users/{}/Images/Primary", Uuid::nil()),
            &Seen::new(),
        )
        .expect("a relayed route");
        assert_eq!(target.body(), Carried::Encoded { cap: IMAGE_LIMIT });
        assert!(!target.read_only());
    }

    #[test]
    fn the_preference_bag_and_the_authorize_are_relayed_and_the_culture_list_stands_in_both_stages()
    {
        assert!(
            Target::admit(
                &Method::GET,
                "DisplayPreferences/usersettings",
                &Seen::new()
            )
            .is_some()
        );
        let saved = Target::admit(
            &Method::POST,
            "DisplayPreferences/usersettings",
            &Seen::new(),
        )
        .expect("a relayed route");
        assert!(!saved.read_only());
        assert_eq!(saved.body(), Carried::Capped { cap: BODY_LIMIT });
        let authorize = Target::admit(&Method::POST, "QuickConnect/Authorize", &Seen::new())
            .expect("a relayed route");
        assert!(!authorize.read_only());
        assert_eq!(authorize.body(), Carried::None);
        let cultures = Target::admit(&Method::GET, "Localization/Cultures", &Seen::new())
            .expect("a relayed route");
        assert_eq!(cultures.stage(), Stage::Either);
    }

    #[test]
    fn a_write_method_on_a_route_declared_read_only_is_not_relayed() {
        assert!(Target::admit(&Method::GET, "LiveTv/Channels", &Seen::new()).is_some());
        assert!(Target::admit(&Method::POST, "LiveTv/Channels", &Seen::new()).is_none());
        assert!(Target::admit(&Method::DELETE, "LiveTv/Channels", &Seen::new()).is_none());
        assert!(Target::admit(&Method::DELETE, "LiveTv/GuideInfo", &Seen::new()).is_none());
    }

    #[test]
    fn a_timer_id_outside_its_character_class_is_not_relayed() {
        assert!(Target::admit(&Method::DELETE, "LiveTv/Timers/ti.mer", &Seen::new()).is_none());
        assert!(
            Target::admit(&Method::POST, "LiveTv/SeriesTimers/ser%20ies", &Seen::new()).is_none()
        );
    }

    #[test]
    fn a_timer_id_longer_than_its_limit_is_not_relayed() {
        let long = "a".repeat(65);
        assert!(
            Target::admit(
                &Method::DELETE,
                &format!("LiveTv/Timers/{long}"),
                &Seen::new()
            )
            .is_none()
        );
        assert!(
            Target::admit(
                &Method::DELETE,
                &format!("LiveTv/SeriesTimers/{long}"),
                &Seen::new()
            )
            .is_none()
        );
    }

    #[test]
    fn a_route_carrying_no_body_declares_it() {
        for path in ["LiveTv/Channels", "LiveTv/Timers", "Items"] {
            let target = Target::admit(&Method::GET, path, &Seen::new()).expect("a relayed route");
            assert_eq!(target.body(), Carried::None, "GET {path}");
        }
        let target = Target::admit(&Method::DELETE, "LiveTv/Timers/t-1", &Seen::new())
            .expect("a relayed route");
        assert_eq!(target.body(), Carried::None);
    }

    #[test]
    fn a_shape_keeps_the_literals_and_drops_the_ids_and_the_query() {
        assert_eq!(
            shape(&format!(
                "/LiveTv/LiveRecordings/{ID}/stream?api_key=secret"
            )),
            "/LiveTv/LiveRecordings/*/stream"
        );
        assert_eq!(
            shape(&format!("/Videos/{}/stream.mp4", "a".repeat(40))),
            "/Videos/*/stream.mp4"
        );
        assert_eq!(shape("/LiveTv/Channels"), "/LiveTv/Channels");
    }

    #[test]
    fn a_server_url_carrying_a_path_keeps_it_under_the_relayed_segments() {
        let base = reqwest::Url::parse("https://example.test/jellyfin").expect("base");
        let target =
            Target::admit(&Method::GET, &format!("Items/{ID}"), &Seen::new()).expect("route");
        let url = target.url(&base, None);
        assert_eq!(url.path(), format!("/jellyfin/Items/{ID}"));
    }

    #[test]
    fn a_foreign_answer_carries_no_provider_url_once_it_is_rewritten() {
        let seen = Seen::new();
        let body = r#"[{"Name":"one","ImageUrl":"https://provider.example/a.jpg"},
                       {"Name":"two","ImageUrl":"http://provider.example/b.jpg"}]"#;
        let rewritten = seen.foreign(body);
        assert!(!rewritten.contains("provider.example"));
        assert!(!rewritten.contains("https://"));
        assert!(rewritten.contains("one"));
        assert!(rewritten.contains("two"));
    }

    #[test]
    fn a_minted_handle_resolves_to_the_url_it_was_minted_for() {
        let seen = Seen::new();
        let url = "https://provider.example/a.jpg";
        let rewritten = seen.foreign(&format!(r#"{{"ImageUrl":"{url}"}}"#));
        let value: serde_json::Value = serde_json::from_str(&rewritten).expect("json");
        let handle = value["ImageUrl"].as_str().expect("a handle");
        assert_ne!(handle, url);
        assert_eq!(seen.observed(handle).as_deref(), Some(url));
        assert!(seen.holds(Observed::Image, url));
    }

    #[test]
    fn a_handle_this_run_did_not_mint_resolves_to_nothing() {
        let seen = Seen::new();
        assert_eq!(seen.observed("f0000000deadbeef"), None);
        assert!(!seen.holds(Observed::Image, "https://provider.example/a.jpg"));
    }

    #[test]
    fn a_url_seen_twice_keeps_the_handle_it_was_first_given() {
        let seen = Seen::new();
        let url = "https://provider.example/a.jpg";
        let first = seen.foreign(&format!(r#"{{"ImageUrl":"{url}"}}"#));
        let again = seen.foreign(&format!(r#"{{"Other":"{url}"}}"#));
        let one: serde_json::Value = serde_json::from_str(&first).expect("json");
        let two: serde_json::Value = serde_json::from_str(&again).expect("json");
        assert_eq!(one["ImageUrl"], two["Other"]);
    }

    #[test]
    fn a_body_that_is_not_json_is_answered_unchanged() {
        let seen = Seen::new();
        assert_eq!(seen.foreign("not json at all"), "not json at all");
    }

    #[test]
    fn a_relative_reference_is_left_alone_because_it_names_no_provider() {
        let seen = Seen::new();
        let rewritten = seen.foreign(r#"{"ImageUrl":"/Items/1/Images/Primary"}"#);
        assert!(rewritten.contains("/Items/1/Images/Primary"));
    }

    #[test]
    fn the_foreign_routes_read_their_bodies_rather_than_streaming_them() {
        let seen = Seen::new();
        for path in [
            "/Items/RemoteSearch/Movie",
            "/Items/00000000000000000000000000000001/RemoteImages",
        ] {
            let method = if path.ends_with("RemoteImages") {
                Method::GET
            } else {
                Method::POST
            };
            let target = Target::admit(&method, path, &seen).expect(path);
            assert_eq!(target.payload(), Payload::Foreign, "{path}");
        }
    }

    #[test]
    fn only_the_two_download_routes_admit_a_minted_handle() {
        let seen = Seen::new();
        let query = Target::admit(
            &Method::POST,
            "/Items/00000000000000000000000000000001/RemoteImages/Download",
            &seen,
        )
        .expect("the download route");
        assert_eq!(query.resolves(), Resolves::Query);

        let body = Target::admit(
            &Method::POST,
            "/Items/RemoteSearch/Apply/00000000000000000000000000000001",
            &seen,
        )
        .expect("the apply route");
        assert_eq!(body.resolves(), Resolves::Body);

        let plain = Target::admit(&Method::GET, "/Genres", &seen).expect("the genre listing");
        assert_eq!(plain.resolves(), Resolves::None);
    }
}
