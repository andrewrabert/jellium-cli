use jellium_protocol::{
    AudioChoice, Chapter, Delivery, Method, Plan, PlayRequest, PlaybackRefused, Refusal,
    SubtitleChoice,
};
use jellyfin_api::types::{
    ChapterInfo, MediaSourceInfo, MediaStream, MediaStreamProtocol, MediaStreamType,
};

use super::negotiate::Negotiated;
use super::profile;
use crate::web::{manifest, route};

fn streams(source: &MediaSourceInfo, kind: MediaStreamType) -> Vec<&MediaStream> {
    source
        .media_streams
        .as_deref()
        .unwrap_or_default()
        .iter()
        .filter(|stream| stream.type_ == Some(kind))
        .collect()
}

fn label(stream: &MediaStream) -> String {
    stream
        .display_title
        .clone()
        .or_else(|| stream.title.clone())
        .or_else(|| stream.language.clone())
        .unwrap_or_else(|| format!("Stream {}", stream.index.unwrap_or_default()))
}

/// True when the negotiated source carries a video stream, which is what
/// decides the endpoint family the browser is pointed at.
fn visual(source: &MediaSourceInfo) -> bool {
    !streams(source, MediaStreamType::Video).is_empty()
}

/// The first spelling of a comma-joined container list.
fn container(source: &MediaSourceInfo) -> Option<&str> {
    source
        .container
        .as_deref()
        .and_then(|container| container.split(',').next())
        .filter(|container| !container.is_empty())
}

fn relayed(
    base: &reqwest::Url,
    path: &str,
    query: &[(&str, String)],
    seen: &route::Seen,
) -> Result<String, Refusal> {
    let mut url = base.clone();
    {
        let mut segments = url
            .path_segments_mut()
            .map_err(|()| Refusal::ManifestNotRewritable)?;
        segments.pop_if_empty();
        segments.extend(path.split('/'));
    }
    url.query_pairs_mut().extend_pairs(query.iter().cloned());
    manifest::relay_path(&url, base, seen)
}

/// The same-origin path a direct-played or remuxed source is loaded from.
fn progressive(
    negotiated: &Negotiated,
    request: &PlayRequest,
    base: &reqwest::Url,
    seen: &route::Seen,
) -> Result<String, Refusal> {
    let family = if visual(&negotiated.source) {
        "Videos"
    } else {
        "Audio"
    };
    let item = request.item.to_string();
    let source_id = negotiated
        .source
        .id
        .clone()
        .ok_or(Refusal::ManifestNotRewritable)?;
    let query = [
        ("static", "true".to_string()),
        ("mediaSourceId", source_id),
        ("playSessionId", negotiated.play_session.clone()),
    ];
    if let Some(container) = container(&negotiated.source) {
        let suffixed = format!("{family}/{item}/stream.{container}");
        if let Ok(path) = relayed(base, &suffixed, &query, seen) {
            return Ok(path);
        }
    }
    relayed(base, &format!("{family}/{item}/stream"), &query, seen)
}

/// The same-origin path the negotiated transcoding url maps to.
fn transcoded(
    negotiated: &Negotiated,
    base: &reqwest::Url,
    seen: &route::Seen,
) -> Result<Delivery, Refusal> {
    let transcoding = negotiated
        .source
        .transcoding_url
        .as_deref()
        .ok_or(Refusal::ManifestNotRewritable)?;
    let path = manifest::resolved(transcoding, base, base, seen)?;
    if negotiated.source.transcoding_sub_protocol == Some(MediaStreamProtocol::Hls) {
        Ok(Delivery::Hls { path })
    } else {
        Ok(Delivery::Progressive { path })
    }
}

/// The same-origin path a live source is delivered from: the source's own path
/// mapped through the route table, as HLS when the source's transcoding
/// sub-protocol says so and progressively otherwise.
/// A path the table does not admit is `LiveNotRelayable` carrying
/// `route::shape` of it.
fn live(
    source: &MediaSourceInfo,
    base: &reqwest::Url,
    seen: &route::Seen,
) -> Result<Delivery, PlaybackRefused> {
    let reference = source.path.as_deref().unwrap_or_default();
    let refused = || PlaybackRefused::LiveNotRelayable {
        shape: route::shape(reference),
    };
    let path = manifest::resolved(reference, base, base, seen).map_err(|_| refused())?;
    if source.transcoding_sub_protocol == Some(MediaStreamProtocol::Hls) {
        Ok(Delivery::Hls { path })
    } else {
        Ok(Delivery::Progressive { path })
    }
}

/// The same-origin WebVTT path a text subtitle stream is fetched from, and
/// nothing for a bitmap stream the Jellyfin server burns in.
fn track(
    stream: &MediaStream,
    request: &PlayRequest,
    source_id: &str,
    base: &reqwest::Url,
    seen: &route::Seen,
) -> Option<String> {
    if stream.codec.as_deref().is_some_and(profile::bitmap) {
        return None;
    }
    let index = stream.index?;
    let item = request.item.to_string();
    relayed(
        base,
        &format!("Videos/{item}/{source_id}/Subtitles/{index}/0/Stream.vtt"),
        &[],
        seen,
    )
    .ok()
}

/// The plan the browser plays from: a live source delivered from the live
/// shapes, a direct play from a same-origin progressive path, and the
/// negotiated transcoding url mapped to a same-origin master manifest path
/// otherwise, plus the audio, subtitle, version and chapter choices.
/// A live plan carries no chapters, no run time and no subtitle stream.
/// Every text subtitle stream carries a same-origin WebVTT path; a bitmap
/// stream carries none.
pub fn build(
    negotiated: &Negotiated,
    request: &PlayRequest,
    chapters: &[ChapterInfo],
    base: &reqwest::Url,
    seen: &route::Seen,
) -> Result<Plan, PlaybackRefused> {
    let source_id = negotiated
        .source
        .id
        .clone()
        .ok_or(PlaybackRefused::NotRelayable)?;

    let delivery = if negotiated.live {
        live(&negotiated.source, base, seen)?
    } else {
        match negotiated.method {
            Method::DirectPlay | Method::DirectStream => Delivery::Progressive {
                path: progressive(negotiated, request, base, seen)
                    .map_err(|_| PlaybackRefused::NotRelayable)?,
            },
            Method::Transcode { .. } => {
                transcoded(negotiated, base, seen).map_err(|_| PlaybackRefused::NotRelayable)?
            }
        }
    };

    let default_audio = negotiated.source.default_audio_stream_index;
    let default_subtitle = negotiated.source.default_subtitle_stream_index;

    let audio_streams = streams(&negotiated.source, MediaStreamType::Audio)
        .into_iter()
        .filter_map(|stream| {
            Some(AudioChoice {
                index: stream.index?,
                label: label(stream),
                default: stream.index == default_audio,
            })
        })
        .collect();

    let subtitle_streams = if negotiated.live {
        Vec::new()
    } else {
        streams(&negotiated.source, MediaStreamType::Subtitle)
            .into_iter()
            .filter_map(|stream| {
                Some(SubtitleChoice {
                    index: stream.index?,
                    label: label(stream),
                    language: stream.language.clone(),
                    default: stream.index == default_subtitle,
                    track: track(stream, request, &source_id, base, seen),
                })
            })
            .collect()
    };

    Ok(Plan {
        play_session: negotiated.play_session.clone(),
        item: request.item,
        media_source: source_id,
        method: negotiated.method,
        delivery,
        start_ticks: request.start_ticks,
        run_time_ticks: (!negotiated.live)
            .then_some(negotiated.source.run_time_ticks)
            .flatten(),
        sources: negotiated.sources.clone(),
        audio_streams,
        subtitle_streams,
        audio_stream: request.audio_stream.or(default_audio),
        subtitle_stream: (!negotiated.live)
            .then(|| request.subtitle_stream.or(default_subtitle))
            .flatten(),
        chapters: if negotiated.live {
            Vec::new()
        } else {
            chapters
                .iter()
                .map(|chapter| Chapter {
                    name: chapter.name.clone().unwrap_or_default(),
                    start_ticks: chapter.start_position_ticks.unwrap_or_default(),
                })
                .collect()
        },
        max_bitrate: negotiated.max_bitrate,
        live: negotiated.live,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use jellium_protocol::{Capabilities, Decoding, Quality};

    fn base() -> reqwest::Url {
        reqwest::Url::parse("https://example.test").expect("base")
    }

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

    fn video_stream(index: i32) -> MediaStream {
        MediaStream {
            index: Some(index),
            type_: Some(MediaStreamType::Video),
            ..MediaStream::default()
        }
    }

    fn subtitle_stream(index: i32, codec: &str) -> MediaStream {
        MediaStream {
            index: Some(index),
            type_: Some(MediaStreamType::Subtitle),
            codec: Some(codec.to_string()),
            display_title: Some(format!("Subtitle {index}")),
            ..MediaStream::default()
        }
    }

    fn source(streams: Vec<MediaStream>) -> MediaSourceInfo {
        MediaSourceInfo {
            id: Some("d0000000000000000000000000000000".to_string()),
            container: Some("mkv".to_string()),
            media_streams: Some(streams),
            run_time_ticks: Some(100),
            ..MediaSourceInfo::default()
        }
    }

    fn negotiated(method: Method, source: MediaSourceInfo) -> Negotiated {
        Negotiated {
            play_session: "session".to_string(),
            source,
            sources: Vec::new(),
            live_stream: None,
            method,
            max_bitrate: None,
            live: false,
        }
    }

    fn live_source(path: &str, hls: bool) -> MediaSourceInfo {
        MediaSourceInfo {
            id: Some("live0000000000000000000000000000".to_string()),
            path: Some(path.to_string()),
            container: Some("ts".to_string()),
            media_streams: Some(vec![video_stream(0), subtitle_stream(1, "subrip")]),
            run_time_ticks: Some(100),
            transcoding_sub_protocol: hls.then_some(MediaStreamProtocol::Hls),
            ..MediaSourceInfo::default()
        }
    }

    fn live_negotiated(source: MediaSourceInfo) -> Negotiated {
        Negotiated {
            live: true,
            ..negotiated(Method::DirectPlay, source)
        }
    }

    #[test]
    fn a_live_stream_file_is_delivered_from_a_same_origin_path() {
        let plan = build(
            &live_negotiated(live_source(
                "/LiveTv/LiveStreamFiles/tuner-1/stream.ts",
                false,
            )),
            &request(),
            &[],
            &base(),
            &route::Seen::new(),
        )
        .expect("a plan");
        assert!(plan.live);
        let Delivery::Progressive { path } = plan.delivery else {
            panic!("a progressive live source is delivered progressively");
        };
        assert_eq!(path, "/jellyfin/LiveTv/LiveStreamFiles/tuner-1/stream.ts");
    }

    #[test]
    fn a_live_recording_is_delivered_from_a_same_origin_path() {
        let plan = build(
            &live_negotiated(live_source("/LiveTv/LiveRecordings/rec-1/stream", false)),
            &request(),
            &[],
            &base(),
            &route::Seen::new(),
        )
        .expect("a plan");
        let Delivery::Progressive { path } = plan.delivery else {
            panic!("a progressive live source is delivered progressively");
        };
        assert_eq!(path, "/jellyfin/LiveTv/LiveRecordings/rec-1/stream");
    }

    #[test]
    fn a_live_hls_source_is_delivered_over_hls() {
        let plan = build(
            &live_negotiated(live_source(
                "/LiveTv/LiveStreamFiles/tuner-1/stream.ts",
                true,
            )),
            &request(),
            &[],
            &base(),
            &route::Seen::new(),
        )
        .expect("a plan");
        let Delivery::Hls { path } = plan.delivery else {
            panic!("an hls live source is delivered over hls");
        };
        assert!(path.starts_with("/jellyfin/LiveTv/LiveStreamFiles/"));
    }

    #[test]
    fn a_live_source_outside_the_route_table_names_its_shape() {
        let refused = build(
            &live_negotiated(live_source("/LiveTv/Tuners/abcdef/stream.ts", false)),
            &request(),
            &[],
            &base(),
            &route::Seen::new(),
        )
        .expect_err("a refusal");
        let PlaybackRefused::LiveNotRelayable { shape } = refused else {
            panic!("a live source outside the table is not relayable");
        };
        assert_eq!(shape, "/LiveTv/Tuners/abcdef/stream.ts");
    }

    #[test]
    fn a_refused_live_shape_carries_no_id_and_no_query() {
        let id = uuid::Uuid::nil();
        let refused = build(
            &live_negotiated(live_source(
                &format!("/LiveTv/Tuners/{id}/stream.ts?api_key=secret"),
                false,
            )),
            &request(),
            &[],
            &base(),
            &route::Seen::new(),
        )
        .expect_err("a refusal");
        let PlaybackRefused::LiveNotRelayable { shape } = refused else {
            panic!("a live source outside the table is not relayable");
        };
        assert_eq!(shape, "/LiveTv/Tuners/*/stream.ts");
        assert!(!shape.contains("secret"));
    }

    #[test]
    fn a_live_plan_carries_no_chapters_and_no_run_time() {
        let plan = build(
            &live_negotiated(live_source(
                "/LiveTv/LiveStreamFiles/tuner-1/stream.ts",
                false,
            )),
            &request(),
            &[ChapterInfo {
                name: Some("One".to_string()),
                start_position_ticks: Some(0),
                ..ChapterInfo::default()
            }],
            &base(),
            &route::Seen::new(),
        )
        .expect("a plan");
        assert!(plan.chapters.is_empty());
        assert_eq!(plan.run_time_ticks, None);
        assert!(plan.subtitle_streams.is_empty());
        assert_eq!(plan.subtitle_stream, None);
    }

    #[test]
    fn a_direct_play_is_delivered_from_a_same_origin_path() {
        let plan = build(
            &negotiated(Method::DirectPlay, source(vec![video_stream(0)])),
            &request(),
            &[],
            &base(),
            &route::Seen::new(),
        )
        .expect("a plan");
        let Delivery::Progressive { path } = plan.delivery else {
            panic!("a direct play is progressive");
        };
        assert!(path.starts_with("/jellyfin/Videos/"));
        assert!(path.contains("stream.mkv"));
    }

    #[test]
    fn a_transcode_maps_the_negotiated_url_to_a_same_origin_manifest_path() {
        let mut transcoding = source(vec![video_stream(0)]);
        transcoding.transcoding_url = Some(format!(
            "/Videos/{}/master.m3u8?api_key=secret&PlaySessionId=session",
            uuid::Uuid::nil()
        ));
        transcoding.transcoding_sub_protocol = Some(MediaStreamProtocol::Hls);
        let plan = build(
            &negotiated(
                Method::Transcode {
                    subtitle_burn_in: false,
                },
                transcoding,
            ),
            &request(),
            &[],
            &base(),
            &route::Seen::new(),
        )
        .expect("a plan");
        let Delivery::Hls { path } = plan.delivery else {
            panic!("an hls transcode is delivered over hls");
        };
        assert!(path.starts_with("/jellyfin/Videos/"));
        assert!(!path.contains("secret"));
    }

    #[test]
    fn a_transcode_behind_a_server_path_prefix_maps_to_a_relay_path() {
        let base = reqwest::Url::parse("https://example.test/jellyfin").expect("base");
        let mut transcoding = source(vec![video_stream(0)]);
        transcoding.transcoding_url = Some(format!(
            "/Videos/{}/master.m3u8?api_key=secret&PlaySessionId=session",
            uuid::Uuid::nil()
        ));
        transcoding.transcoding_sub_protocol = Some(MediaStreamProtocol::Hls);
        let plan = build(
            &negotiated(
                Method::Transcode {
                    subtitle_burn_in: false,
                },
                transcoding,
            ),
            &request(),
            &[],
            &base,
            &route::Seen::new(),
        )
        .expect("a plan");
        let Delivery::Hls { path } = plan.delivery else {
            panic!("an hls transcode is delivered over hls");
        };
        assert!(path.starts_with("/jellyfin/Videos/"));
        assert!(!path.contains("secret"));
    }

    #[test]
    fn a_text_subtitle_carries_a_track_and_a_bitmap_subtitle_does_not() {
        let plan = build(
            &negotiated(
                Method::DirectPlay,
                source(vec![
                    video_stream(0),
                    subtitle_stream(1, "subrip"),
                    subtitle_stream(2, "pgssub"),
                ]),
            ),
            &request(),
            &[],
            &base(),
            &route::Seen::new(),
        )
        .expect("a plan");
        assert_eq!(plan.subtitle_streams.len(), 2);
        assert!(plan.subtitle_streams[0].track.is_some());
        assert!(plan.subtitle_streams[1].track.is_none());
    }

    #[test]
    fn no_path_a_plan_carries_leaves_the_local_server() {
        let plan = build(
            &negotiated(
                Method::DirectPlay,
                source(vec![video_stream(0), subtitle_stream(1, "subrip")]),
            ),
            &request(),
            &[],
            &base(),
            &route::Seen::new(),
        )
        .expect("a plan");
        let rendered = serde_json::to_string(&plan).expect("serialized");
        assert!(!rendered.contains("example.test"));
        assert!(!rendered.contains("http"));
    }
}
