//! The stub upstream's playback half: the master playlist, the variant playlist
//! carrying an fMP4 initialization segment, the segments both name, and one
//! WebVTT subtitle track.

use axum::extract::RawQuery;
use axum::http::header::CONTENT_TYPE;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use uuid::Uuid;

/// The identities the stub upstream's playback routes serve, and the paths it
/// serves them at.
pub struct Stream;

impl Stream {
    /// The item the playback routes serve; a playback path naming another item
    /// matches no route here.
    const ITEM: Uuid = Uuid::from_u128(0x9006);

    /// The reference the master playlist names its one variant by.
    const VARIANT: &'static str = "main.m3u8";

    /// The reference the variant playlist names its initialization segment by.
    const INITIALIZATION: &'static str = "hls1/main/-1.mp4";

    /// The reference the variant playlist names its one media segment by.
    const SEGMENT: &'static str = "hls1/main/0.mp4";

    /// The one cue the subtitle track carries.
    pub const CUE: &'static str = "the relayed cue";

    /// The path a `reference` a playlist of the served item names resolves to,
    /// under the Jellyfin server's own root.
    fn under(reference: &str) -> String {
        format!("/Videos/{}/{reference}", Stream::ITEM)
    }

    /// The media source the playback routes serve: the item's own id without
    /// hyphens, the spelling a Jellyfin server gives a file source's id.
    fn source() -> String {
        Stream::ITEM.simple().to_string()
    }

    /// The path the master playlist is served at.
    pub fn master_playlist() -> String {
        Stream::under("master.m3u8")
    }

    /// The path the subtitle track is served at.
    pub fn subtitle_track() -> String {
        Stream::under(&format!("{}/Subtitles/2/0/Stream.vtt", Stream::source()))
    }
}

/// The content type a Jellyfin server answers an HLS playlist with.
const PLAYLIST_TYPE: &str = "application/vnd.apple.mpegurl";

fn suffixed(query: Option<String>) -> String {
    match query.filter(|query| !query.is_empty()) {
        Some(query) => format!("?{query}"),
        None => String::new(),
    }
}

fn playlist(body: String) -> Response {
    (
        axum::http::StatusCode::OK,
        [(CONTENT_TYPE, PLAYLIST_TYPE)],
        body,
    )
        .into_response()
}

async fn master(RawQuery(query): RawQuery) -> Response {
    playlist(format!(
        concat!(
            "#EXTM3U\n",
            "#EXT-X-STREAM-INF:BANDWIDTH=2000000\n",
            "{variant}{carried}\n",
        ),
        variant = Stream::VARIANT,
        carried = suffixed(query)
    ))
}

async fn variant(RawQuery(query): RawQuery) -> Response {
    let carried = suffixed(query);
    playlist(format!(
        concat!(
            "#EXTM3U\n",
            "#EXT-X-TARGETDURATION:6\n",
            "#EXT-X-MAP:URI=\"{initialization}{carried}\"\n",
            "#EXTINF:6,\n",
            "{segment}{carried}\n",
            "#EXT-X-ENDLIST\n",
        ),
        initialization = Stream::INITIALIZATION,
        segment = Stream::SEGMENT,
        carried = carried
    ))
}

/// One fMP4 segment, served as the bytes an mp4 route answers with.
async fn segment() -> Response {
    (
        axum::http::StatusCode::OK,
        [(CONTENT_TYPE, "video/mp4")],
        vec![0x00, 0x00, 0x00, 0x18],
    )
        .into_response()
}

async fn subtitles() -> Response {
    (
        axum::http::StatusCode::OK,
        [(CONTENT_TYPE, "text/vtt")],
        format!("WEBVTT\n\n00:00:00.000 --> 00:00:05.000\n{}\n", Stream::CUE),
    )
        .into_response()
}

pub fn router() -> axum::Router {
    axum::Router::new()
        .route(&Stream::master_playlist(), get(master))
        .route(&Stream::under(Stream::VARIANT), get(variant))
        .route(&Stream::under(Stream::INITIALIZATION), get(segment))
        .route(&Stream::under(Stream::SEGMENT), get(segment))
        .route(&Stream::subtitle_track(), get(subtitles))
}
