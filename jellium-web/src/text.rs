use std::sync::OnceLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code, reason = "the boot page resolves some keys from JavaScript")]
pub enum Text {
    AppName,
    BootLoading,
    BootWasmFailed,
    BootRendererFailed,
    LoginTitle,
    LoginServer,
    LoginUsername,
    LoginPassword,
    LoginSubmit,
    LoginWorking,
    HomeLibraries,
    HomeContinueWatching,
    HomeNextUp,
    HomeEmpty,
    LibrarySort,
    SortName,
    SortNameDescending,
    SortDateAdded,
    SortReleaseDate,
    SortCommunityRating,
    SortRandom,
    PagePosition,
    PagePrevious,
    PageNext,
    SearchPlaceholder,
    SearchSubmit,
    SearchEmpty,
    DetailOverview,
    DetailSeasons,
    DetailEpisodes,
    DetailTracks,
    DetailAlbums,
    DetailMarkPlayed,
    DetailMarkUnplayed,
    DetailFavorite,
    DetailUnfavorite,
    NavBack,
    NavHome,
    NavSearch,
    NavLogout,
    StatusLoading,
    FailureServerUnreachable,
    FailureCredentialsRejected,
    FailureTokenRejected,
    FailureServerBelowMinimum,
    FailureRelay,
    FailureNotThisBrowser,
    FailureForeignOrigin,
    FailureNoSession,
    FailureNotRelayed,
    WarningOffSnapshot,
}

impl Text {
    pub fn key(self) -> &'static str {
        match self {
            Text::AppName => "appName",
            Text::BootLoading => "bootLoading",
            Text::BootWasmFailed => "bootWasmFailed",
            Text::BootRendererFailed => "bootRendererFailed",
            Text::LoginTitle => "loginTitle",
            Text::LoginServer => "loginServer",
            Text::LoginUsername => "loginUsername",
            Text::LoginPassword => "loginPassword",
            Text::LoginSubmit => "loginSubmit",
            Text::LoginWorking => "loginWorking",
            Text::HomeLibraries => "homeLibraries",
            Text::HomeContinueWatching => "homeContinueWatching",
            Text::HomeNextUp => "homeNextUp",
            Text::HomeEmpty => "homeEmpty",
            Text::LibrarySort => "librarySort",
            Text::SortName => "sortName",
            Text::SortNameDescending => "sortNameDescending",
            Text::SortDateAdded => "sortDateAdded",
            Text::SortReleaseDate => "sortReleaseDate",
            Text::SortCommunityRating => "sortCommunityRating",
            Text::SortRandom => "sortRandom",
            Text::PagePosition => "pagePosition",
            Text::PagePrevious => "pagePrevious",
            Text::PageNext => "pageNext",
            Text::SearchPlaceholder => "searchPlaceholder",
            Text::SearchSubmit => "searchSubmit",
            Text::SearchEmpty => "searchEmpty",
            Text::DetailOverview => "detailOverview",
            Text::DetailSeasons => "detailSeasons",
            Text::DetailEpisodes => "detailEpisodes",
            Text::DetailTracks => "detailTracks",
            Text::DetailAlbums => "detailAlbums",
            Text::DetailMarkPlayed => "detailMarkPlayed",
            Text::DetailMarkUnplayed => "detailMarkUnplayed",
            Text::DetailFavorite => "detailFavorite",
            Text::DetailUnfavorite => "detailUnfavorite",
            Text::NavBack => "navBack",
            Text::NavHome => "navHome",
            Text::NavSearch => "navSearch",
            Text::NavLogout => "navLogout",
            Text::StatusLoading => "statusLoading",
            Text::FailureServerUnreachable => "failureServerUnreachable",
            Text::FailureCredentialsRejected => "failureCredentialsRejected",
            Text::FailureTokenRejected => "failureTokenRejected",
            Text::FailureServerBelowMinimum => "failureServerBelowMinimum",
            Text::FailureRelay => "failureRelay",
            Text::FailureNotThisBrowser => "failureNotThisBrowser",
            Text::FailureForeignOrigin => "failureForeignOrigin",
            Text::FailureNoSession => "failureNoSession",
            Text::FailureNotRelayed => "failureNotRelayed",
            Text::WarningOffSnapshot => "warningOffSnapshot",
        }
    }
}

const TABLE_SOURCE: &str = include_str!("../strings/en-us.json");

fn table() -> &'static serde_json::Map<String, serde_json::Value> {
    static TABLE: OnceLock<serde_json::Map<String, serde_json::Value>> = OnceLock::new();
    TABLE.get_or_init(|| {
        serde_json::from_str(TABLE_SOURCE).expect("strings/en-us.json is not a JSON object")
    })
}

pub fn lookup(key: Text) -> &'static str {
    table()
        .get(key.key())
        .and_then(serde_json::Value::as_str)
        .unwrap_or_else(|| panic!("string table lacks the key {}", key.key()))
}

pub fn format(key: Text, args: &[&str]) -> String {
    let template = lookup(key);
    let mut out = String::with_capacity(template.len());
    let mut rest = template;
    while let Some(open) = rest.find('{') {
        let Some(close) = rest[open..].find('}').map(|i| open + i) else {
            break;
        };
        let Ok(index) = rest[open + 1..close].parse::<usize>() else {
            out.push_str(&rest[..close + 1]);
            rest = &rest[close + 1..];
            continue;
        };
        out.push_str(&rest[..open]);
        out.push_str(args.get(index).copied().unwrap_or_default());
        rest = &rest[close + 1..];
    }
    out.push_str(rest);
    out
}
