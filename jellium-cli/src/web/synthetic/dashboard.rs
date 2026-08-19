//! The second stub area serves everything the dashboard reads and one plugin
//! whose configuration page is HTML it hosts.

use std::sync::{Arc, Mutex};

use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{delete, get, post};
use uuid::Uuid;

/// The synthetic dashboard: `USERS` users, the configuration sections, the
/// virtual folders, `TASKS` scheduled tasks, `ACTIVITY` activity entries, the
/// installed plugins, `PACKAGES` catalog packages, the repositories, the Live
/// TV administration, and a log file `LOG_BYTES` long.
#[derive(Clone)]
pub struct Dashboard {
    held: Arc<Mutex<Held>>,
}

struct Held {
    /// The named configuration sections, by key.
    sections: std::collections::HashMap<String, serde_json::Value>,
    /// The user policies, by user.
    policies: std::collections::HashMap<Uuid, serde_json::Value>,
    /// The library options, by virtual folder name.
    options: std::collections::HashMap<String, serde_json::Value>,
    /// The display-preferences records, by client.
    preferences: std::collections::HashMap<String, serde_json::Value>,
    /// The user configurations, by user.
    configurations: std::collections::HashMap<Uuid, serde_json::Value>,
    /// The bytes the last user image upload carried.
    image: Option<Vec<u8>>,
    /// The Quick Connect codes authorized so far.
    authorized: std::collections::HashSet<String>,
    /// The configuration held for `Dashboard::PLUGIN`.
    plugin: serde_json::Value,
    /// The message the next write refuses with.
    refusing: Option<String>,
    /// Every write the stub took, as `METHOD path`.
    wrote: Vec<String>,
}

impl Dashboard {
    pub const USERS: usize = 12;
    pub const TASKS: usize = 24;
    pub const ACTIVITY: usize = 100_000;
    pub const PACKAGES: usize = 500;
    pub const LOG_BYTES: usize = 8 * 1024 * 1024;

    /// The Quick Connect code the stub holds a request for that is not yet
    /// authorized.
    pub const PENDING_CODE: &'static str = "123456";

    /// The Quick Connect code the stub holds an already-authorized request for.
    pub const AUTHORIZED_CODE: &'static str = "654321";

    /// The synthetic plugin whose configuration page is HTML this stub hosts.
    pub const PLUGIN: Uuid = Uuid::from_u128(0x5017_0000_0000_0000_0000_0000_0000_0001);
    pub const PLUGIN_PAGE: &'static str = "SyntheticPluginPage";

    fn new() -> Dashboard {
        let mut sections = std::collections::HashMap::new();
        for key in [
            "encoding",
            "network",
            "metadata",
            "trickplay",
            "livetv",
            "branding",
        ] {
            sections.insert(key.to_owned(), section_of(key));
        }
        Dashboard {
            held: Arc::new(Mutex::new(Held {
                sections,
                policies: std::collections::HashMap::new(),
                options: std::collections::HashMap::new(),
                preferences: std::collections::HashMap::new(),
                configurations: std::collections::HashMap::new(),
                image: None,
                authorized: std::collections::HashSet::new(),
                plugin: plugin_configuration(),
                refusing: None,
                wrote: Vec::new(),
            })),
        }
    }

    fn held(&self) -> std::sync::MutexGuard<'_, Held> {
        self.held.lock().expect("the synthetic dashboard")
    }

    /// The configuration held for `PLUGIN`, including the fields no control
    /// covers.
    pub fn plugin_configuration(&self) -> serde_json::Value {
        self.held().plugin.clone()
    }

    /// The named configuration section held now.
    pub fn section(&self, key: &str) -> serde_json::Value {
        self.held()
            .sections
            .get(key)
            .cloned()
            .unwrap_or(serde_json::Value::Null)
    }

    /// The user policy held for `user` now.
    pub fn policy(&self, user: Uuid) -> serde_json::Value {
        self.held()
            .policies
            .get(&user)
            .cloned()
            .unwrap_or_else(|| policy_of(user))
    }

    /// The display-preferences record held for `client` now, carrying the
    /// custom preference no control names.
    pub fn display_preferences(&self, client: &str) -> serde_json::Value {
        self.held()
            .preferences
            .get(client)
            .cloned()
            .unwrap_or_else(|| preferences_of(client))
    }

    /// The user configuration held for `user` now, carrying the fields no
    /// control covers.
    pub fn configuration(&self, user: Uuid) -> serde_json::Value {
        self.held()
            .configurations
            .get(&user)
            .cloned()
            .unwrap_or_else(configuration_of)
    }

    /// The bytes the last user image upload carried, decoded from the base64
    /// the relay forwarded.
    pub fn image(&self) -> Option<Vec<u8>> {
        self.held().image.clone()
    }

    /// The library options held for `name` now.
    pub fn options(&self, name: &str) -> serde_json::Value {
        self.held()
            .options
            .get(name)
            .cloned()
            .unwrap_or_else(|| options_of(name))
    }

    /// Makes the next write refuse with `message`, which is what an
    /// administrative write failure renders beneath its sentence.
    pub fn refuse_next(&self, message: &str) {
        self.held().refusing = Some(message.to_owned());
    }

    /// Every write the stub took, as `METHOD path`, in arrival order.
    pub fn wrote(&self) -> Vec<String> {
        self.held().wrote.clone()
    }

    /// Records one write, and answers the refusal it was told to make.
    fn writing(&self, what: &str) -> Option<Response> {
        let mut held = self.held();
        held.wrote.push(what.to_owned());
        let refusing = held.refusing.take()?;
        Some(
            (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "title": "Bad Request",
                    "detail": refusing,
                })),
            )
                .into_response(),
        )
    }

    /// The `id` of the user at `index`.
    pub fn user(index: usize) -> Uuid {
        Uuid::from_u128(0x1000_0000_0000_0000_0000_0000_0000_0000 + index as u128)
    }

    /// The `id` of the scheduled task at `index`.
    pub fn task(index: usize) -> String {
        format!("task-{index}")
    }
}

fn section_of(key: &str) -> serde_json::Value {
    serde_json::json!({
        "Key": key,
        "EnableThrottling": false,
        "TranscodingTempPath": format!("/var/lib/jellyfin/{key}"),
        "DownMixAudioBoost": 2,
        "UnnamedByAnyControl": {"kept": [1, 2, 3]},
    })
}

fn policy_of(user: Uuid) -> serde_json::Value {
    serde_json::json!({
        "IsAdministrator": user == Dashboard::user(0),
        "IsDisabled": false,
        "EnableAllFolders": true,
        "EnabledFolders": [],
        "MaxParentalRating": serde_json::Value::Null,
        "UnnamedByAnyControl": {"kept": true},
    })
}

fn options_of(name: &str) -> serde_json::Value {
    serde_json::json!({
        "Name": name,
        "EnablePhotos": true,
        "EnableRealtimeMonitor": true,
        "SaveLocalMetadata": false,
        "MetadataCountryCode": "US",
        "UnnamedByAnyControl": {"kept": "options"},
    })
}

fn plugin_configuration() -> serde_json::Value {
    serde_json::json!({
        "Greeting": "hello",
        "Retries": 3,
        "UnnamedByAnyControl": {"kept": ["a", "b"]},
    })
}

fn users() -> Vec<serde_json::Value> {
    (0..Dashboard::USERS)
        .map(|index| {
            let id = Dashboard::user(index);
            serde_json::json!({
                "Id": id,
                "Name": format!("user-{index}"),
                "Policy": policy_of(id),
                "Configuration": configuration_of(),
            })
        })
        .collect()
}

fn tasks() -> Vec<serde_json::Value> {
    (0..Dashboard::TASKS)
        .map(|index| {
            let running = index % 6 == 0;
            serde_json::json!({
                "Id": Dashboard::task(index),
                "Name": format!("Task {index}"),
                "Category": if index % 2 == 0 { "Library" } else { "Maintenance" },
                "Description": format!("What task {index} does."),
                "State": if running { "Running" } else { "Idle" },
                "CurrentProgressPercentage": running.then_some(42.5),
                "Triggers": [{"Type": "DailyTrigger", "TimeOfDayTicks": 36_000_000_000i64}],
                "LastExecutionResult": {
                    "Status": "Completed",
                    "StartTimeUtc": "2026-01-01T00:00:00Z",
                    "EndTimeUtc": "2026-01-01T00:05:00Z",
                },
            })
        })
        .collect()
}

fn activity(start: usize, limit: usize) -> Vec<serde_json::Value> {
    (start..(start + limit).min(Dashboard::ACTIVITY))
        .map(|index| {
            let named = index % 3 != 0;
            serde_json::json!({
                "Id": index as i64,
                "Name": format!("Entry {index}"),
                "ShortOverview": format!("What happened at {index}."),
                "Type": "SessionStarted",
                "Severity": "Information",
                "UserId": named.then(|| Dashboard::user(index % Dashboard::USERS)),
                "Date": "2026-01-01T00:00:00Z",
            })
        })
        .collect()
}

fn plugins() -> Vec<serde_json::Value> {
    vec![serde_json::json!({
        "Id": Dashboard::PLUGIN,
        "Name": "Synthetic Plugin",
        "Version": "1.2.3",
        "Status": "Active",
        "CanUninstall": true,
        "Description": "The one plugin the stub hosts.",
    })]
}

fn packages() -> Vec<serde_json::Value> {
    (0..Dashboard::PACKAGES)
        .map(|index| {
            serde_json::json!({
                "name": format!("Package {index}"),
                "description": format!("What package {index} offers."),
                "overview": "",
                "owner": "synthetic",
                "category": "General",
                "guid": Uuid::from_u128(0x2000_0000_0000_0000_0000_0000_0000_0000 + index as u128),
                "versions": [
                    {"version": "2.0.0", "repositoryName": "Synthetic", "repositoryUrl": "https://synthetic.test/manifest.json"},
                    {"version": "1.0.0", "repositoryName": "Synthetic", "repositoryUrl": "https://synthetic.test/manifest.json"},
                ],
            })
        })
        .collect()
}

fn virtual_folders() -> Vec<serde_json::Value> {
    ["Movies", "Shows", "Music"]
        .iter()
        .enumerate()
        .map(|(index, name)| {
            serde_json::json!({
                "Name": name,
                "CollectionType": if index == 2 { "music" } else { "movies" },
                "ItemId": Uuid::from_u128(0x3000_0000_0000_0000_0000_0000_0000_0000 + index as u128).to_string(),
                "Locations": [format!("/media/{}", name.to_lowercase())],
                "LibraryOptions": options_of(name),
            })
        })
        .collect()
}

/// The configuration page document the stub hosts: it reaches the host only
/// through the injected shim, and its one subresource is another configuration
/// page.
const PLUGIN_PAGE_HTML: &str = concat!(
    "<html><head><link rel=\"stylesheet\" href=\"ConfigurationPage?name=SyntheticPluginPage\">",
    "</head><body><h1>Synthetic</h1>",
    "<script>ApiClient.getPluginConfiguration().then(function (held) {",
    "document.body.dataset.greeting = held.Greeting; });</script>",
    "</body></html>",
);

async fn system_info() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "ServerName": "synthetic",
        "Version": "10.11.0",
        "Id": "synthetic-server",
        "OperatingSystem": "Linux",
        "HasUpdateAvailable": false,
    }))
}

async fn configuration() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "ServerName": "synthetic",
        "EnableMetrics": false,
        "LibraryScanFanoutConcurrency": 0,
        "UnnamedByAnyControl": {"kept": 1},
    }))
}

async fn named_configuration(
    State(state): State<Dashboard>,
    Path(key): Path<String>,
) -> Json<serde_json::Value> {
    Json(state.section(&key))
}

async fn save_named_configuration(
    State(state): State<Dashboard>,
    Path(key): Path<String>,
    Json(body): Json<serde_json::Value>,
) -> Response {
    if let Some(refused) = state.writing(&format!("POST /System/Configuration/{key}")) {
        return refused;
    }
    state.held().sections.insert(key, body);
    StatusCode::NO_CONTENT.into_response()
}

async fn logs() -> Json<serde_json::Value> {
    Json(serde_json::json!([
        {"Name": "jellyfin.log", "Size": Dashboard::LOG_BYTES, "DateCreated": "2026-01-01T00:00:00Z", "DateModified": "2026-01-02T00:00:00Z"},
    ]))
}

/// A log body `LOG_BYTES` long, which is what the relay must deliver only the
/// tail of.
async fn log_body(Query(query): Query<std::collections::HashMap<String, String>>) -> Response {
    if query.get("name").map(String::as_str) != Some("jellyfin.log") {
        return StatusCode::NOT_FOUND.into_response();
    }
    let line = "2026-01-01 00:00:00 [INF] a synthetic log line\n";
    let mut body = String::with_capacity(Dashboard::LOG_BYTES + line.len());
    while body.len() < Dashboard::LOG_BYTES {
        body.push_str(line);
    }
    body.truncate(Dashboard::LOG_BYTES);
    (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "text/plain")],
        body,
    )
        .into_response()
}

async fn activity_entries(
    Query(query): Query<std::collections::HashMap<String, String>>,
) -> Json<serde_json::Value> {
    let start = query
        .get("startIndex")
        .and_then(|held| held.parse::<usize>().ok())
        .unwrap_or(0);
    let limit = query
        .get("limit")
        .and_then(|held| held.parse::<usize>().ok())
        .unwrap_or(100);
    Json(serde_json::json!({
        "Items": activity(start, limit),
        "TotalRecordCount": Dashboard::ACTIVITY,
        "StartIndex": start,
    }))
}

async fn listed_users() -> Json<serde_json::Value> {
    Json(serde_json::Value::Array(users()))
}

/// The account a signed session resolves to, which is what a resume reads;
/// a request whose authorization carries an empty token is refused, which is
/// what a revoked saved credential meets.
async fn current_user(headers: axum::http::HeaderMap) -> Response {
    let tokened = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value.contains("Token=\"") && !value.contains("Token=\"\""));
    if !tokened {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    Json(serde_json::json!({
        "Id": Uuid::from_u128(1),
        "Name": "first",
        "Policy": {
            "AuthenticationProviderId": "",
            "PasswordResetProviderId": "",
            "EnableCollectionManagement": false,
            "EnableLyricManagement": false,
            "EnableSubtitleManagement": false,
            "IsAdministrator": true,
            "EnableLiveTvAccess": true,
        },
    }))
    .into_response()
}

async fn one_user(Path(id): Path<Uuid>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "Id": id,
        "Name": "user-0",
        "Policy": policy_of(id),
        "Configuration": configuration_of(),
    }))
}

/// The display-preferences record this stub answers for a client it holds none
/// for, carrying one custom preference no control names.
fn preferences_of(client: &str) -> serde_json::Value {
    serde_json::json!({
        "Client": client,
        "Id": "usersettings",
        "CustomPrefs": {"theirs": "kept"},
    })
}

/// The user configuration this stub answers for a user it holds none for,
/// carrying a field no control covers.
fn configuration_of() -> serde_json::Value {
    serde_json::json!({
        "AudioLanguagePreference": "eng",
        "GroupedFolders": ["theirs"],
    })
}

async fn display_preferences(
    State(state): State<Dashboard>,
    axum::extract::Query(query): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Json<serde_json::Value> {
    let client = query.get("client").cloned().unwrap_or_default();
    Json(state.display_preferences(&client))
}

async fn save_display_preferences(
    State(state): State<Dashboard>,
    axum::extract::Query(query): axum::extract::Query<std::collections::HashMap<String, String>>,
    Json(body): Json<serde_json::Value>,
) -> Response {
    if let Some(refused) = state.writing("POST /DisplayPreferences/usersettings") {
        return refused;
    }
    let client = query.get("client").cloned().unwrap_or_default();
    state.held().preferences.insert(client, body);
    StatusCode::NO_CONTENT.into_response()
}

async fn save_configuration(
    State(state): State<Dashboard>,
    Path(id): Path<Uuid>,
    Json(body): Json<serde_json::Value>,
) -> Response {
    if let Some(refused) = state.writing(&format!("POST /Users/{id}/Configuration")) {
        return refused;
    }
    state.held().configurations.insert(id, body);
    StatusCode::NO_CONTENT.into_response()
}

async fn save_user_image(
    State(state): State<Dashboard>,
    Path(id): Path<Uuid>,
    body: axum::body::Bytes,
) -> Response {
    if let Some(refused) = state.writing(&format!("POST /Users/{id}/Images/Primary")) {
        return refused;
    }
    use base64::Engine;
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(&body)
        .unwrap_or_default();
    state.held().image = Some(decoded);
    StatusCode::NO_CONTENT.into_response()
}

async fn authorize_quick_connect(
    State(state): State<Dashboard>,
    axum::extract::Query(query): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Response {
    let code = query.get("code").cloned().unwrap_or_default();
    if let Some(refused) = state.writing(&format!("POST /QuickConnect/Authorize?code={code}")) {
        return refused;
    }
    let mut held = state.held();
    if code == Dashboard::AUTHORIZED_CODE || !held.authorized.insert(code.clone()) {
        return (
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({"detail": "Request is already authorized"})),
        )
            .into_response();
    }
    if code != Dashboard::PENDING_CODE {
        held.authorized.remove(&code);
        return (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"detail": "not found"})),
        )
            .into_response();
    }
    StatusCode::NO_CONTENT.into_response()
}

async fn save_policy(
    State(state): State<Dashboard>,
    Path(id): Path<Uuid>,
    Json(body): Json<serde_json::Value>,
) -> Response {
    if let Some(refused) = state.writing(&format!("POST /Users/{id}/Policy")) {
        return refused;
    }
    state.held().policies.insert(id, body);
    StatusCode::NO_CONTENT.into_response()
}

async fn save_library_options(
    State(state): State<Dashboard>,
    Json(body): Json<serde_json::Value>,
) -> Response {
    if let Some(refused) = state.writing("POST /Library/VirtualFolders/LibraryOptions") {
        return refused;
    }
    let name = body
        .get("Id")
        .and_then(serde_json::Value::as_str)
        .unwrap_or_default()
        .to_owned();
    if let Some(options) = body.get("LibraryOptions") {
        state.held().options.insert(name, options.clone());
    }
    StatusCode::NO_CONTENT.into_response()
}

async fn plugin_configuration_read(State(state): State<Dashboard>) -> Json<serde_json::Value> {
    Json(state.plugin_configuration())
}

async fn plugin_configuration_write(
    State(state): State<Dashboard>,
    Json(body): Json<serde_json::Value>,
) -> Response {
    if let Some(refused) = state.writing("POST /Plugins/{id}/Configuration") {
        return refused;
    }
    state.held().plugin = body;
    StatusCode::NO_CONTENT.into_response()
}

async fn configuration_pages() -> Json<serde_json::Value> {
    Json(serde_json::json!([
        {
            "Name": Dashboard::PLUGIN_PAGE,
            "EnableInMainMenu": true,
            "PluginId": Dashboard::PLUGIN,
            "DisplayName": "Synthetic",
        },
    ]))
}

async fn configuration_page(
    Query(query): Query<std::collections::HashMap<String, String>>,
) -> Response {
    if query.get("name").map(String::as_str) != Some(Dashboard::PLUGIN_PAGE) {
        return StatusCode::NOT_FOUND.into_response();
    }
    (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "text/html")],
        PLUGIN_PAGE_HTML,
    )
        .into_response()
}

/// Every write with no state of its own: it is recorded, and refuses when the
/// stub was told to.
async fn wrote(State(state): State<Dashboard>, request: axum::extract::Request) -> Response {
    let what = format!("{} {}", request.method(), request.uri().path());
    match state.writing(&what) {
        Some(refused) => refused,
        None => StatusCode::NO_CONTENT.into_response(),
    }
}

async fn empty_list() -> Json<serde_json::Value> {
    Json(serde_json::json!([]))
}

/// The router serving `/System/Info`, `/System/Restart`, `/System/Shutdown`,
/// `/System/Configuration`, `/System/Configuration/{key}`, `/System/Logs`,
/// `/System/Logs/Log`, `/System/ActivityLog/Entries`, `/Users`, `/Users/{id}`
/// and its policy, configuration, password and image, `/Library/VirtualFolders`
/// and its name, paths and options, `/Libraries/AvailableOptions`,
/// `/Environment/Drives`, `/Environment/DirectoryContents`, `/ScheduledTasks`
/// and its running and trigger routes, `/Plugins` and its enable, disable,
/// image and configuration routes, `/Packages`, `/Repositories`,
/// `/web/ConfigurationPages`, `/web/ConfigurationPage`, `/Devices`,
/// `/Auth/Keys`, and the Live TV tuner, provider and mapping routes.
pub fn router() -> (axum::Router, Dashboard) {
    let state = Dashboard::new();
    let router = axum::Router::new()
        .route("/System/Info", get(system_info))
        .route("/System/Restart", post(wrote))
        .route("/System/Shutdown", post(wrote))
        .route("/System/Configuration", get(configuration).post(wrote))
        .route(
            "/System/Configuration/{key}",
            get(named_configuration).post(save_named_configuration),
        )
        .route("/System/Logs", get(logs))
        .route("/System/Logs/Log", get(log_body))
        .route("/System/ActivityLog/Entries", get(activity_entries))
        .route("/Users", get(listed_users))
        .route("/Users/New", post(wrote))
        .route("/Users/Me", get(current_user))
        .route("/Users/{id}", get(one_user).post(wrote).delete(wrote))
        .route("/Users/{id}/Policy", post(save_policy))
        .route("/Users/{id}/Configuration", post(save_configuration))
        .route("/Users/{id}/Password", post(wrote))
        .route(
            "/Users/{id}/Images/Primary",
            get(empty_list).post(save_user_image).delete(wrote),
        )
        .route(
            "/DisplayPreferences/{id}",
            get(display_preferences).post(save_display_preferences),
        )
        .route("/QuickConnect/Authorize", post(authorize_quick_connect))
        .route(
            "/Library/VirtualFolders",
            get(|| async { Json(serde_json::Value::Array(virtual_folders())) })
                .post(wrote)
                .delete(wrote),
        )
        .route("/Library/VirtualFolders/Name", post(wrote))
        .route(
            "/Library/VirtualFolders/Paths",
            post(wrote).delete(wrote),
        )
        .route("/Library/VirtualFolders/Paths/Update", post(wrote))
        .route(
            "/Library/VirtualFolders/LibraryOptions",
            post(save_library_options),
        )
        .route("/Library/Refresh", post(wrote))
        .route("/Items/{id}/Refresh", post(wrote))
        .route(
            "/Libraries/AvailableOptions",
            get(|| async {
                Json(serde_json::json!({
                    "MetadataSavers": [],
                    "MetadataReaders": [],
                    "SubtitleFetchers": [],
                    "TypeOptions": [],
                }))
            }),
        )
        .route(
            "/Environment/Drives",
            get(|| async {
                Json(serde_json::json!([{"Name": "media", "Path": "/media", "Type": "Directory"}]))
            }),
        )
        .route(
            "/Environment/DirectoryContents",
            get(|| async {
                Json(serde_json::json!([{"Name": "movies", "Path": "/media/movies", "Type": "Directory"}]))
            }),
        )
        .route(
            "/ScheduledTasks",
            get(|| async { Json(serde_json::Value::Array(tasks())) }),
        )
        .route(
            "/ScheduledTasks/{id}",
            get(|| async { Json(tasks().into_iter().next().expect("a synthetic task")) }),
        )
        .route(
            "/ScheduledTasks/Running/{id}",
            post(wrote).delete(wrote),
        )
        .route("/ScheduledTasks/{id}/Triggers", post(wrote))
        .route(
            "/Plugins",
            get(|| async { Json(serde_json::Value::Array(plugins())) }),
        )
        .route("/Plugins/{id}", delete(wrote))
        .route("/Plugins/{id}/{version}", delete(wrote))
        .route("/Plugins/{id}/{version}/Enable", post(wrote))
        .route("/Plugins/{id}/{version}/Disable", post(wrote))
        .route("/Plugins/{id}/{version}/Image", get(empty_list))
        .route(
            "/Plugins/{id}/Configuration",
            get(plugin_configuration_read).post(plugin_configuration_write),
        )
        .route(
            "/Packages",
            get(|| async { Json(serde_json::Value::Array(packages())) }),
        )
        .route("/Packages/Installed/{name}", post(wrote))
        .route("/Packages/Installing/{id}", delete(wrote))
        .route(
            "/Repositories",
            get(|| async {
                Json(serde_json::json!([
                    {"Name": "Synthetic", "Url": "https://synthetic.test/manifest.json", "Enabled": true},
                ]))
            })
            .post(wrote),
        )
        .route("/web/ConfigurationPages", get(configuration_pages))
        .route("/web/ConfigurationPage", get(configuration_page))
        .route(
            "/Devices",
            get(|| async {
                Json(serde_json::json!({
                    "Items": [{
                        "Id": "device-0",
                        "Name": "Device 0",
                        "AppName": "Jellium",
                        "LastUserName": "user-0",
                        "DateLastActivity": "2026-01-01T00:00:00Z",
                    }],
                    "TotalRecordCount": 1,
                }))
            })
            .delete(wrote),
        )
        .route("/Devices/Options", post(wrote))
        .route(
            "/Auth/Keys",
            get(|| async {
                Json(serde_json::json!({
                    "Items": [{"AccessToken": "key-0", "AppName": "Synthetic", "DateCreated": "2026-01-01T00:00:00Z"}],
                    "TotalRecordCount": 1,
                }))
            })
            .post(wrote),
        )
        .route("/Auth/Keys/{key}", delete(wrote))
        .route("/LiveTv/TunerHosts", post(wrote).delete(wrote))
        .route(
            "/LiveTv/TunerHosts/Types",
            get(|| async { Json(serde_json::json!([{"Name": "HD Homerun", "Id": "hdhomerun"}])) }),
        )
        .route(
            "/LiveTv/Tuners/Discover",
            get(|| async { Json(serde_json::json!([{"Id": "tuner-0", "Type": "hdhomerun", "Url": "http://tuner.test"}])) }),
        )
        .route("/LiveTv/Tuners/{id}/Reset", post(wrote))
        .route("/LiveTv/ListingProviders", post(wrote).delete(wrote))
        .route(
            "/LiveTv/ListingProviders/Default",
            get(|| async { Json(serde_json::json!({"Type": "SchedulesDirect"})) }),
        )
        .route(
            "/LiveTv/ListingProviders/Lineups",
            get(|| async { Json(serde_json::json!([{"Name": "Lineup", "Id": "lineup-0"}])) }),
        )
        .route(
            "/LiveTv/ListingProviders/SchedulesDirect/Countries",
            get(|| async { Json(serde_json::json!({"US": [{"shortName": "US", "fullName": "United States"}]})) }),
        )
        .route(
            "/LiveTv/ChannelMappingOptions",
            get(|| async {
                Json(serde_json::json!({
                    "TunerChannels": [],
                    "ProviderChannels": [],
                    "Mappings": [],
                }))
            }),
        )
        .route("/LiveTv/ChannelMappings", post(wrote))
        .with_state(state.clone());
    (router, state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_synthetic_dashboard_carries_what_the_load_rows_name() {
        assert_eq!(Dashboard::USERS, 12);
        assert_eq!(Dashboard::TASKS, 24);
        assert_eq!(Dashboard::ACTIVITY, 100_000);
        assert_eq!(Dashboard::PACKAGES, 500);
        assert_eq!(Dashboard::LOG_BYTES, 8 * 1024 * 1024);
        assert_eq!(users().len(), Dashboard::USERS);
        assert_eq!(tasks().len(), Dashboard::TASKS);
        assert_eq!(packages().len(), Dashboard::PACKAGES);
    }

    #[test]
    fn a_saved_section_keeps_what_it_was_given() {
        let dashboard = Dashboard::new();
        assert_eq!(dashboard.section("encoding")["DownMixAudioBoost"], 2);
        dashboard
            .held()
            .sections
            .insert("encoding".to_owned(), serde_json::json!({"A": 1}));
        assert_eq!(dashboard.section("encoding"), serde_json::json!({"A": 1}));
    }

    #[test]
    fn the_preference_bag_answers_a_custom_preference_no_control_names() {
        let dashboard = Dashboard::new();
        let read = dashboard.display_preferences("Jellium Web");
        assert_eq!(read["Id"], "usersettings");
        assert_eq!(read["Client"], "Jellium Web");
        assert_eq!(read["CustomPrefs"]["theirs"], "kept");
    }

    #[test]
    fn a_saved_bag_and_a_saved_configuration_keep_what_they_were_given() {
        let dashboard = Dashboard::new();
        let user = Uuid::nil();
        assert_eq!(dashboard.configuration(user)["GroupedFolders"][0], "theirs");
        dashboard
            .held()
            .preferences
            .insert("Jellium Web".to_owned(), serde_json::json!({"A": 1}));
        dashboard
            .held()
            .configurations
            .insert(user, serde_json::json!({"B": 2}));
        assert_eq!(
            dashboard.display_preferences("Jellium Web"),
            serde_json::json!({"A": 1})
        );
        assert_eq!(dashboard.configuration(user), serde_json::json!({"B": 2}));
    }

    #[test]
    fn the_stub_holds_the_bytes_an_upload_carried_once_they_are_decoded() {
        let dashboard = Dashboard::new();
        assert_eq!(dashboard.image(), None);
        dashboard.held().image = Some(vec![1, 2, 3]);
        assert_eq!(dashboard.image(), Some(vec![1, 2, 3]));
    }

    #[test]
    fn the_two_quick_connect_codes_are_six_digits_and_differ() {
        for code in [Dashboard::PENDING_CODE, Dashboard::AUTHORIZED_CODE] {
            assert_eq!(code.len(), 6);
            assert!(code.chars().all(|c| c.is_ascii_digit()));
        }
        assert_ne!(Dashboard::PENDING_CODE, Dashboard::AUTHORIZED_CODE);
    }

    #[test]
    fn a_refusal_is_made_once_and_carries_its_message() {
        let dashboard = Dashboard::new();
        dashboard.refuse_next("the server would not");
        assert!(dashboard.writing("POST /System/Restart").is_some());
        assert!(dashboard.writing("POST /System/Restart").is_none());
        assert_eq!(dashboard.wrote().len(), 2);
    }

    #[test]
    fn the_plugin_page_reaches_the_host_only_through_the_shim() {
        assert!(PLUGIN_PAGE_HTML.contains("ApiClient.getPluginConfiguration"));
        assert!(!PLUGIN_PAGE_HTML.contains("://"));
    }

    #[test]
    fn the_activity_log_pages_from_any_start() {
        let page = activity(99_900, 200);
        assert_eq!(page.len(), 100);
        assert_eq!(page[0]["Id"], 99_900);
    }
}
