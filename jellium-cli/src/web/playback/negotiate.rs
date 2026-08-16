use jellium_protocol::{Failure, Method, PlayRequest, PlaybackRefused, SourceChoice};
use jellyfin_api::types::{
    MediaSourceInfo, MediaStreamType, OpenLiveStreamDto, PlaybackErrorCode, PlaybackInfoDto,
};

use super::profile;
use crate::web::upstream::Upstream;

/// What the Jellyfin server settled on for one play request.
pub struct Negotiated {
    pub play_session: String,
    pub source: MediaSourceInfo,
    pub sources: Vec<SourceChoice>,
    pub live_stream: Option<String>,
    pub method: Method,
    pub max_bitrate: Option<i32>,
    /// True when the chosen source is a live stream this session opened.
    pub live: bool,
}

/// A negotiation outcome the browser is shown, or a transport failure.
pub enum Refused {
    Playback(PlaybackRefused),
    Upstream(Failure),
}

fn choices(sources: &[MediaSourceInfo]) -> Vec<SourceChoice> {
    sources
        .iter()
        .filter_map(|source| {
            let id = source.id.clone()?;
            let name = source.name.clone().unwrap_or_else(|| id.clone());
            Some(SourceChoice { id, name })
        })
        .collect()
}

/// The requested version, or the first the Jellyfin server offers.
fn chosen(sources: &[MediaSourceInfo], wanted: Option<&str>) -> Option<MediaSourceInfo> {
    match wanted {
        Some(wanted) => sources
            .iter()
            .find(|source| source.id.as_deref() == Some(wanted))
            .or_else(|| sources.first())
            .cloned(),
        None => sources.first().cloned(),
    }
}

/// True when the subtitle the request selects is a picture, which is what
/// forces a transcode to burn it in.
fn burns_in(source: &MediaSourceInfo, subtitle: Option<i32>) -> bool {
    let Some(subtitle) = subtitle else {
        return false;
    };
    source
        .media_streams
        .as_deref()
        .unwrap_or_default()
        .iter()
        .filter(|stream| stream.type_ == Some(MediaStreamType::Subtitle))
        .find(|stream| stream.index == Some(subtitle))
        .and_then(|stream| stream.codec.as_deref())
        .is_some_and(profile::bitmap)
}

fn refused(code: PlaybackErrorCode) -> PlaybackRefused {
    match code {
        PlaybackErrorCode::NoCompatibleStream => PlaybackRefused::NoPlayableSource,
        PlaybackErrorCode::NotAllowed => PlaybackRefused::TranscodeRefused {
            code: "NotAllowed".to_string(),
        },
        PlaybackErrorCode::RateLimitExceeded => PlaybackRefused::TranscodeRefused {
            code: "RateLimitExceeded".to_string(),
        },
    }
}

fn method(source: &MediaSourceInfo, allow_direct_play: bool, burn_in: bool) -> Method {
    if allow_direct_play && source.supports_direct_play == Some(true) {
        Method::DirectPlay
    } else if source.supports_direct_stream == Some(true) && source.transcoding_url.is_none() {
        Method::DirectStream
    } else {
        Method::Transcode {
            subtitle_burn_in: burn_in,
        }
    }
}

/// Posts `PlaybackInfo` with the profile built from `request`, picks the
/// requested source or the first one, and opens a live stream when the chosen
/// source requires opening.
/// A `NoCompatibleStream` error code reads as `NoPlayableSource`; `NotAllowed`
/// and `RateLimitExceeded` read as `TranscodeRefused`; an item with no source
/// reads as `NoMediaSource`.
/// A bitmap subtitle selection sets `always_burn_in_subtitle_when_transcoding`,
/// which is what `Method::Transcode::subtitle_burn_in` records.
/// A live source the Jellyfin server will not open reads as `NoTuner`, without
/// retry and naming nothing else.
/// A re-negotiation for the channel a held live session was playing that finds
/// `resuming` gone from the offered sources reads as `TunerGone`.
pub async fn negotiate(
    upstream: &Upstream,
    request: &PlayRequest,
    ceiling: Option<i32>,
    resuming: Option<&str>,
) -> Result<Negotiated, Refused> {
    let control = upstream.control();
    let user = upstream.user_id();

    let surveyed = control
        .get_playback_info(&request.item, Some(&user))
        .await
        .map_err(|e| Refused::Upstream(upstream.failed(e)))?;
    let previewed = chosen(&surveyed.media_sources, request.media_source.as_deref())
        .ok_or(Refused::Playback(PlaybackRefused::NoMediaSource))?;
    let burn_in = burns_in(&previewed, request.subtitle_stream);

    let body = PlaybackInfoDto {
        device_profile: Some(profile::build(&request.capabilities, ceiling)),
        user_id: Some(user),
        audio_stream_index: request.audio_stream,
        subtitle_stream_index: request.subtitle_stream,
        media_source_id: request.media_source.clone(),
        start_time_ticks: Some(request.start_ticks),
        max_streaming_bitrate: ceiling,
        enable_direct_play: Some(request.allow_direct_play),
        enable_direct_stream: Some(true),
        enable_transcoding: Some(true),
        allow_audio_stream_copy: Some(true),
        allow_video_stream_copy: Some(true),
        always_burn_in_subtitle_when_transcoding: Some(burn_in),
        auto_open_live_stream: Some(false),
        ..PlaybackInfoDto::default()
    };

    let negotiated = control
        .get_posted_playback_info(
            &request.item,
            &jellyfin_api::query::GetPostedPlaybackInfo::default(),
            &body,
        )
        .await
        .map_err(|e| Refused::Upstream(upstream.failed(e)))?;

    if let Some(code) = negotiated.error_code {
        return Err(Refused::Playback(refused(code)));
    }

    let play_session = negotiated
        .play_session_id
        .clone()
        .ok_or(Refused::Playback(PlaybackRefused::NoPlayableSource))?;
    if let Some(resuming) = resuming
        && !negotiated
            .media_sources
            .iter()
            .any(|source| source.id.as_deref() == Some(resuming))
    {
        return Err(Refused::Playback(PlaybackRefused::TunerGone));
    }

    let sources = choices(&negotiated.media_sources);
    let mut source = chosen(&negotiated.media_sources, request.media_source.as_deref())
        .ok_or(Refused::Playback(PlaybackRefused::NoMediaSource))?;

    let mut live_stream = source.live_stream_id.clone();
    let live = source.requires_opening == Some(true);
    if live {
        let opened = control
            .open_live_stream(
                &jellyfin_api::query::OpenLiveStream {
                    always_burn_in_subtitle_when_transcoding: Some(burn_in),
                    audio_stream_index: request.audio_stream,
                    enable_direct_play: Some(request.allow_direct_play),
                    enable_direct_stream: Some(true),
                    item_id: Some(&request.item),
                    max_streaming_bitrate: ceiling,
                    open_token: source.open_token.as_deref(),
                    play_session_id: Some(&play_session),
                    start_time_ticks: Some(request.start_ticks),
                    subtitle_stream_index: request.subtitle_stream,
                    user_id: Some(&user),
                    ..Default::default()
                },
                &OpenLiveStreamDto {
                    device_profile: Some(profile::build(&request.capabilities, ceiling)),
                    item_id: Some(request.item),
                    open_token: source.open_token.clone(),
                    play_session_id: Some(play_session.clone()),
                    user_id: Some(user),
                    ..OpenLiveStreamDto::default()
                },
            )
            .await
            .map_err(|_| Refused::Playback(PlaybackRefused::NoTuner))?;
        source = opened
            .media_source
            .ok_or(Refused::Playback(PlaybackRefused::NoTuner))?;
        live_stream = source.live_stream_id.clone();
    }

    let method = method(&source, request.allow_direct_play, burn_in);

    Ok(Negotiated {
        play_session,
        source,
        sources,
        live_stream,
        method,
        max_bitrate: ceiling,
        live,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::web::upstream::{Upstream, answering};
    use jellium_protocol::{Capabilities, Decoding, Quality};

    fn request() -> PlayRequest {
        PlayRequest {
            item: uuid::Uuid::nil(),
            media_source: None,
            audio_stream: None,
            subtitle_stream: None,
            start_ticks: 0,
            quality: Quality::Auto,
            capabilities: Capabilities {
                media_source: true,
                direct: Decoding::default(),
                adaptive: Decoding::default(),
            },
            allow_direct_play: true,
        }
    }

    #[tokio::test]
    async fn a_busy_tuner_reads_as_no_tuner() {
        let server = answering(204).await;
        server.live_tv.tuners_busy(true);
        let upstream = Upstream::stub(&server.base);

        let refused = negotiate(&upstream, &request(), None, None)
            .await
            .err()
            .expect("a refusal");
        let Refused::Playback(PlaybackRefused::NoTuner) = refused else {
            panic!("a busy tuner reads as no tuner");
        };
    }

    #[tokio::test]
    async fn a_resume_whose_source_is_gone_reads_as_tuner_gone() {
        let server = answering(204).await;
        let upstream = Upstream::stub(&server.base);

        let refused = negotiate(&upstream, &request(), None, Some("a-source-that-left"))
            .await
            .err()
            .expect("a refusal");
        let Refused::Playback(PlaybackRefused::TunerGone) = refused else {
            panic!("a resume whose source is gone reads as tuner gone");
        };

        negotiate(
            &upstream,
            &request(),
            None,
            Some("livesource00000000000000000000"),
        )
        .await
        .ok()
        .expect("a source still offered negotiates");
    }
}
