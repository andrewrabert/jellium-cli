//! Whether a negotiated version may be played from its own path, decided the
//! way jellyfin-web decides it.

use jellium_protocol::{Failure, HostGrants};
use jellyfin_api::types::{MediaProtocol, MediaSourceInfo, VideoType};

use crate::web::upstream::Upstream;

/// True when this session's connection can reach `source` where it lies: a
/// remote source always, an in-network source unless the connection is in
/// network without being on the server's own machine and the path names a
/// loopback host, and nothing out of network.
// reference: is-host-reachable — playbackmanager.js:576-597
pub async fn reachable(upstream: &Upstream, source: &MediaSourceInfo) -> Result<bool, Failure> {
    if source.is_remote == Some(true) {
        return Ok(true);
    }
    let endpoint = upstream.endpoint().await?;
    if endpoint.is_in_network != Some(true) {
        return Ok(false);
    }
    if endpoint.is_local != Some(true) {
        let path = source.path.clone().unwrap_or_default().to_lowercase();
        if path.contains("localhost") || path.contains("127.0.0.1") {
            return Ok(false);
        }
    }
    Ok(true)
}

/// True when the Jellyfin server sees this session's connection as one from
/// its own network, which is what the bitrate ladder keys its measurement by
/// and floors it on.
// reference: get-endpoint-info — apiClient.js:3864
pub async fn in_network(upstream: &Upstream) -> Result<bool, Failure> {
    Ok(upstream.endpoint().await?.is_in_network == Some(true))
}

/// True when the Jellyfin server reported a video type the stream builder does
/// not yet model, which the reference direct-plays regardless of
/// `SupportsDirectPlay`.
/// The reference also names `HdDvd`, which this Jellyfin version's `VideoType`
/// no longer carries, so no source can be answered under it.
fn folder_rip(source: &MediaSourceInfo) -> bool {
    matches!(source.video_type, Some(VideoType::BluRay | VideoType::Dvd))
}

/// True when `source` may be played from its own path.
/// A remote source is refused outright unless `grants` carries remote video.
// reference: supports-direct-play — playbackmanager.js:599-619
pub async fn direct_play(
    upstream: &Upstream,
    grants: &HostGrants,
    source: &MediaSourceInfo,
) -> Result<bool, Failure> {
    if source.supports_direct_play != Some(true) && !folder_rip(source) {
        return Ok(false);
    }
    if source.is_remote == Some(true) && !grants.remote_video {
        return Ok(false);
    }
    let headerless = source
        .required_http_headers
        .as_ref()
        .is_none_or(|headers| headers.is_empty());
    if source.protocol != Some(MediaProtocol::Http) || !headerless {
        return Ok(false);
    }
    if source.supports_direct_stream != Some(true) && source.supports_transcoding != Some(true) {
        return Ok(true);
    }
    reachable(upstream, source).await
}

/// The version this browser plays, and the direct-play answer the reference
/// writes onto it, which `create-stream-info`'s first branch reads back.
/// The first version that direct-plays, then the first that direct-streams,
/// then the first that transcodes, then the first offered.
// reference: get-optimal-media-source — playbackmanager.js:505-534
pub async fn optimal(
    upstream: &Upstream,
    grants: &HostGrants,
    sources: Vec<MediaSourceInfo>,
) -> Option<(MediaSourceInfo, bool)> {
    let mut answered = Vec::with_capacity(sources.len());
    for source in &sources {
        answered.push(direct_play(upstream, grants, source).await.unwrap_or(false));
    }
    let chosen = answered
        .iter()
        .position(|direct| *direct)
        .or_else(|| {
            sources
                .iter()
                .position(|source| source.supports_direct_stream == Some(true))
        })
        .or_else(|| {
            sources
                .iter()
                .position(|source| source.supports_transcoding == Some(true))
        })
        .unwrap_or(0);
    Some((sources.get(chosen)?.clone(), answered[chosen]))
}
