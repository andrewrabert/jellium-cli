//! The stub upstream's browsing half: collections, playlists with entry ids and
//! duplicate entries, the filter listing, the facet listings, latest media,
//! suggestions, trickplay tiles, chapter images, remote search results and
//! remote images.

use std::sync::{Arc, Mutex};

use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{delete, get, post};
use jellium_protocol::profile::MediaKind;
use jellyfin_api::types::{BaseItemDto, ChapterInfo, MediaType};
use uuid::Uuid;

/// One playlist entry: the item filed and the entry id distinguishing this copy
/// of it from another.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub item: Uuid,
    pub entry: String,
}

struct Held {
    /// Every playlist's entries in playlist order.
    playlists: std::collections::HashMap<Uuid, Vec<Entry>>,
    /// Every collection's items.
    collections: std::collections::HashMap<Uuid, Vec<Uuid>>,
    /// The origin the stub answers on, which the remote answers extend.
    base: String,
    minted: u64,
    /// What each id `GET /Items/{id}` answers as.
    described: std::collections::HashMap<Uuid, MediaKind>,
}

/// The stub upstream's library area.
#[derive(Clone)]
pub struct Library {
    held: Arc<Mutex<Held>>,
}

impl Library {
    /// The playlist the stub opens holding two copies of one item.
    pub const PLAYLIST: Uuid = Uuid::from_u128(0x9001);
    /// The collection the stub opens holding one item.
    pub const COLLECTION: Uuid = Uuid::from_u128(0x9002);
    /// The item both objects hold.
    pub const ITEM: Uuid = Uuid::from_u128(0x9003);
    /// The entry ids the playlist's two copies of `ITEM` are told apart by;
    /// Jellyfin's `PlaylistItemId` is a guid, so the stub mints guids too.
    pub const FIRST: Uuid = Uuid::from_u128(0x9004);
    pub const SECOND: Uuid = Uuid::from_u128(0x9005);
    /// The paths the stub serves a provider image at; the remote answers point
    /// at them absolutely.
    pub const PROVIDER: [&'static str; 2] = ["/provider/poster.jpg", "/provider/backdrop.jpg"];

    fn new() -> Library {
        let mut playlists = std::collections::HashMap::new();
        playlists.insert(
            Library::PLAYLIST,
            vec![
                Entry {
                    item: Library::ITEM,
                    entry: Library::FIRST.to_string(),
                },
                Entry {
                    item: Library::ITEM,
                    entry: Library::SECOND.to_string(),
                },
            ],
        );
        let mut collections = std::collections::HashMap::new();
        collections.insert(Library::COLLECTION, vec![Library::ITEM]);
        Library {
            held: Arc::new(Mutex::new(Held {
                playlists,
                collections,
                base: String::new(),
                minted: 0,
                described: std::collections::HashMap::new(),
            })),
        }
    }

    fn locked(&self) -> std::sync::MutexGuard<'_, Held> {
        match self.held.lock() {
            Ok(held) => held,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    /// The entries the playlist holds now, in playlist order, each with the
    /// entry id the stub minted for it.
    pub fn entries(&self, playlist: Uuid) -> Vec<Entry> {
        self.locked()
            .playlists
            .get(&playlist)
            .cloned()
            .unwrap_or_default()
    }

    /// The items the collection holds now.
    pub fn collected(&self, collection: Uuid) -> Vec<Uuid> {
        self.locked()
            .collections
            .get(&collection)
            .cloned()
            .unwrap_or_default()
    }

    /// Makes `id` answer as an item of `kind`; an id nothing describes answers
    /// as a video whose run time is one hour.
    pub fn describes(&self, id: Uuid, kind: MediaKind) {
        self.locked().described.insert(id, kind);
    }

    fn described(&self, id: Uuid) -> MediaKind {
        self.locked()
            .described
            .get(&id)
            .copied()
            .unwrap_or(MediaKind::Video)
    }

    /// Points the remote answers at `base`, which is the stub's own origin, so
    /// a foreign fetch reaches the stub and is recorded there.
    pub fn based(&self, base: &str) {
        self.locked().base = base.to_owned();
    }

    /// The absolute urls the stub's remote-search and remote-image answers
    /// carry, which is what a foreign fetch is admitted or refused against.
    pub fn foreign(&self) -> Vec<String> {
        let base = self.locked().base.clone();
        Library::PROVIDER
            .iter()
            .map(|path| format!("{base}{path}"))
            .collect()
    }
}

fn ids(raw: Option<&String>) -> Vec<Uuid> {
    raw.map(|raw| raw.split(',').filter_map(|id| id.parse().ok()).collect())
        .unwrap_or_default()
}

async fn playlist_items(
    State(library): State<Library>,
    Path(playlist): Path<Uuid>,
) -> Json<serde_json::Value> {
    let entries = library.entries(playlist);
    let items: Vec<serde_json::Value> = entries
        .iter()
        .map(|entry| {
            serde_json::json!({
                "Id": entry.item,
                "Name": "held",
                "PlaylistItemId": entry.entry,
            })
        })
        .collect();
    Json(serde_json::json!({
        "Items": items,
        "TotalRecordCount": items.len(),
        "StartIndex": 0,
    }))
}

async fn add_playlist_items(
    State(library): State<Library>,
    Path(playlist): Path<Uuid>,
    Query(query): Query<std::collections::HashMap<String, String>>,
) -> StatusCode {
    let mut held = library.locked();
    for item in ids(query.get("ids")) {
        held.minted += 1;
        let entry = Uuid::from_u128(0x9100 + u128::from(held.minted)).to_string();
        held.playlists
            .entry(playlist)
            .or_default()
            .push(Entry { item, entry });
    }
    StatusCode::NO_CONTENT
}

async fn remove_playlist_items(
    State(library): State<Library>,
    Path(playlist): Path<Uuid>,
    Query(query): Query<std::collections::HashMap<String, String>>,
) -> StatusCode {
    let named: Vec<String> = query
        .get("entryIds")
        .map(|raw| raw.split(',').map(str::to_owned).collect())
        .unwrap_or_default();
    let mut held = library.locked();
    if let Some(entries) = held.playlists.get_mut(&playlist) {
        entries.retain(|entry| !named.contains(&entry.entry));
    }
    StatusCode::NO_CONTENT
}

async fn move_playlist_item(
    State(library): State<Library>,
    Path((playlist, entry, to)): Path<(Uuid, String, usize)>,
) -> StatusCode {
    let mut held = library.locked();
    let Some(entries) = held.playlists.get_mut(&playlist) else {
        return StatusCode::NOT_FOUND;
    };
    let Some(at) = entries.iter().position(|held| held.entry == entry) else {
        return StatusCode::NOT_FOUND;
    };
    let moved = entries.remove(at);
    entries.insert(to.min(entries.len()), moved);
    StatusCode::NO_CONTENT
}

async fn add_collection_items(
    State(library): State<Library>,
    Path(collection): Path<Uuid>,
    Query(query): Query<std::collections::HashMap<String, String>>,
) -> StatusCode {
    let mut held = library.locked();
    let items = ids(query.get("ids"));
    held.collections
        .entry(collection)
        .or_default()
        .extend(items);
    StatusCode::NO_CONTENT
}

async fn remove_collection_items(
    State(library): State<Library>,
    Path(collection): Path<Uuid>,
    Query(query): Query<std::collections::HashMap<String, String>>,
) -> StatusCode {
    let named = ids(query.get("ids"));
    let mut held = library.locked();
    if let Some(items) = held.collections.get_mut(&collection) {
        items.retain(|item| !named.contains(item));
    }
    StatusCode::NO_CONTENT
}

/// The run time an item the stub describes carries: one hour, in ticks of a
/// hundred nanoseconds.
const RUN_TIME_TICKS: i64 = 36_000_000_000;

/// `GET /Items/{id}`, answering the item the playback chain reads its media
/// type, run time and chapters off.
pub async fn item(State(library): State<Library>, Path(id): Path<Uuid>) -> Response {
    let media_type = match library.described(id) {
        MediaKind::Audio => MediaType::Audio,
        MediaKind::Video => MediaType::Video,
    };
    Json(BaseItemDto {
        id: Some(id),
        media_type: Some(media_type),
        run_time_ticks: Some(RUN_TIME_TICKS),
        chapters: Some(vec![ChapterInfo {
            name: Some("opening".to_string()),
            start_position_ticks: Some(0),
            ..ChapterInfo::default()
        }]),
        ..BaseItemDto::default()
    })
    .into_response()
}

async fn listing() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "Items": [],
        "TotalRecordCount": 0,
        "StartIndex": 0,
    }))
}

async fn filters() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "Genres": ["Drama"],
        "OfficialRatings": ["PG"],
        "Tags": ["kept"],
        "Years": [1999],
    }))
}

async fn latest() -> Json<serde_json::Value> {
    Json(serde_json::json!([]))
}

async fn remote_search(State(library): State<Library>) -> Json<serde_json::Value> {
    let foreign = library.foreign();
    Json(serde_json::json!([{
        "Name": "candidate",
        "ProductionYear": 1999,
        "ImageUrl": foreign[0],
        "SearchProviderName": "provider",
    }]))
}

async fn remote_images(State(library): State<Library>) -> Json<serde_json::Value> {
    let foreign = library.foreign();
    Json(serde_json::json!({
        "Images": [{
            "ProviderName": "provider",
            "Url": foreign[0],
            "ThumbnailUrl": foreign[1],
            "Type": "Primary",
        }],
        "TotalRecordCount": 1,
        "StartIndex": 0,
    }))
}

/// One provider image, served as the bytes a jpeg route answers with.
async fn provider_image() -> Response {
    (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "image/jpeg")],
        vec![0xff, 0xd8, 0xff, 0xd9],
    )
        .into_response()
}

/// One trickplay tile sheet, served as the bytes a jpeg route answers with.
async fn trickplay_tile() -> Response {
    (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "image/jpeg")],
        vec![0xff, 0xd8, 0xff, 0xd9],
    )
        .into_response()
}

pub fn router() -> (axum::Router, Library) {
    let library = Library::new();
    let router = axum::Router::new()
        .route("/Playlists/{playlist}/Items", get(playlist_items))
        .route("/Playlists/{playlist}/Items", post(add_playlist_items))
        .route("/Playlists/{playlist}/Items", delete(remove_playlist_items))
        .route(
            "/Playlists/{playlist}/Items/{entry}/Move/{to}",
            post(move_playlist_item),
        )
        .route(
            "/Collections/{collection}/Items",
            post(add_collection_items),
        )
        .route(
            "/Collections/{collection}/Items",
            delete(remove_collection_items),
        )
        .route("/Items/Filters", get(filters))
        .route("/Items/Latest", get(latest))
        .route("/Items/Suggestions", get(listing))
        .route("/Items/{item}", get(item))
        .route("/Items/{item}/Similar", get(listing))
        .route("/Items/RemoteSearch/{kind}", post(remote_search))
        .route("/Items/{item}/RemoteImages", get(remote_images))
        .route("/Genres", get(listing))
        .route("/MusicGenres", get(listing))
        .route("/Studios", get(listing))
        .route("/Persons", get(listing))
        .route("/Artists", get(listing))
        .route("/Artists/AlbumArtists", get(listing))
        .route("/Movies/Recommendations", get(latest))
        .route("/Shows/Upcoming", get(listing))
        .route(
            "/Videos/{item}/Trickplay/{width}/{tile}",
            get(trickplay_tile),
        )
        .route("/provider/{name}", get(provider_image))
        .with_state(library.clone());
    (router, library)
}
