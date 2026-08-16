use jellium_protocol::{AudioChoice, Chapter, Plan, PlayRequest, PlaybackRefused, StreamIndex};
use jellyfin_api::types::{ChapterInfo, MediaSourceInfo, MediaStream, MediaStreamType};

use super::negotiate::Negotiated;
use super::pointed::Pointed;
use super::{derive, subtitles};
use crate::web::identity::Identity;
use crate::web::route;
use crate::web::upstream::Upstream;

/// Every stream of `kind` the source carries, in the order the Jellyfin server
/// listed them.
pub fn streams(source: &MediaSourceInfo, kind: MediaStreamType) -> Vec<&MediaStream> {
    source
        .media_streams
        .as_deref()
        .unwrap_or_default()
        .iter()
        .filter(|stream| stream.type_ == Some(kind))
        .collect()
}

/// The label a stream is chosen by; a stream with no title of its own is named
/// by the number the Jellyfin server addresses it by.
pub fn label(stream: &MediaStream, index: StreamIndex) -> String {
    stream
        .display_title
        .clone()
        .or_else(|| stream.title.clone())
        .or_else(|| stream.language.clone())
        .unwrap_or_else(|| format!("Stream {}", index.number()))
}

/// What the Jellyfin server said about the item itself, which the negotiated
/// source says nothing of: its chapters, and the intros the entry door asked
/// for and every other door asked none of.
pub struct Described<'a> {
    pub chapters: &'a [ChapterInfo],
    pub intros: Vec<uuid::Uuid>,
}

/// The plan the browser plays from: the stream `derive::playable` chose, plus
/// the audio, subtitle, version and chapter choices.
/// A live plan carries no chapters, no run time and no subtitle stream.
/// The audio and subtitle streams the plan starts with are the ones the
/// negotiated source names as its defaults.
/// `described` carries what the Jellyfin server said about the item itself.
pub fn build(
    upstream: &Upstream,
    negotiated: &Negotiated,
    request: &PlayRequest,
    identity: &Identity,
    described: Described<'_>,
    seen: &route::Seen,
    pointed: &Pointed,
) -> Result<Plan, PlaybackRefused> {
    let base = upstream.link().base();
    let source_id = negotiated
        .source
        .id
        .clone()
        .ok_or(PlaybackRefused::NotRelayable)?;

    let playable = derive::playable(upstream, negotiated, request, identity, base, seen, pointed)?;

    let audio_streams = streams(&negotiated.source, MediaStreamType::Audio)
        .into_iter()
        .filter_map(|stream| {
            let index = StreamIndex::named(stream.index?)?;
            Some(AudioChoice {
                index,
                label: label(stream, index),
            })
        })
        .collect();

    let subtitle_streams = if negotiated.live {
        Vec::new()
    } else {
        subtitles::offered(&negotiated.source, base, seen, pointed)
    };

    Ok(Plan {
        play_session: negotiated.play_session.clone(),
        item: request.item,
        media_source: source_id,
        playable,
        start_ticks: request.start_ticks,
        run_time_ticks: (!negotiated.live)
            .then_some(negotiated.source.run_time_ticks)
            .flatten(),
        sources: negotiated.sources.clone(),
        audio_streams,
        subtitle_streams,
        audio_stream: negotiated
            .source
            .default_audio_stream_index
            .and_then(StreamIndex::named),
        subtitle_stream: (!negotiated.live)
            .then(|| {
                negotiated
                    .source
                    .default_subtitle_stream_index
                    .and_then(StreamIndex::named)
            })
            .flatten(),
        chapters: if negotiated.live {
            Vec::new()
        } else {
            described
                .chapters
                .iter()
                .map(|chapter| Chapter {
                    name: chapter.name.clone().unwrap_or_default(),
                    start_ticks: chapter.start_position_ticks.unwrap_or_default(),
                })
                .collect()
        },
        max_bitrate: negotiated.max_bitrate,
        live: negotiated.live,
        supports_transcoding: negotiated.source.supports_transcoding.unwrap_or(false),
        intros: described.intros,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::web::upstream::Upstream;
    use jellium_protocol::profile::DeviceProfile;
    use jellium_protocol::{HostGrants, Method, Quality, SubtitleDelivery, Subtitles};
    use jellyfin_api::types::MediaStreamProtocol;

    /// The media source the served source names itself by.
    const MEDIA_SOURCE: &str = "d0000000000000000000000000000000";

    fn upstream() -> Upstream {
        Upstream::stub("https://example.test")
    }

    fn device() -> Identity {
        Identity::of(jellium_protocol::Identity {
            device: "Firefox".to_owned(),
            device_id: uuid::Uuid::nil().to_string(),
        })
    }

    fn request() -> PlayRequest {
        PlayRequest {
            item: uuid::Uuid::nil(),
            media_source: None,
            audio_stream: None,
            subtitles: Subtitles::Default,
            start_ticks: 0,
            quality: Quality::Auto,
            profile: DeviceProfile::default(),
            always_burn_in_subtitle_when_transcoding: false,
            allow_direct_play: None,
            allow_direct_stream: None,
            allow_video_stream_copy: None,
            allow_audio_stream_copy: None,
            grants: HostGrants { remote_video: true },
            cinema_mode: true,
            fullscreen: true,
            start_index: None,
            reporting: jellium_protocol::report::Reporting {
                volume_level: 100,
                muted: false,
                repeat: jellium_protocol::Repeat::Off,
                shuffle: jellium_protocol::report::Shuffle::Sorted,
                playback_rate: 1.0,
                playlist_item_id: "playlistItem1".to_string(),
                queue: Vec::new(),
            },
        }
    }

    fn described(chapters: &[ChapterInfo]) -> Described<'_> {
        Described {
            chapters,
            intros: Vec::new(),
        }
    }

    fn planned(negotiated: &Negotiated) -> Result<Plan, PlaybackRefused> {
        build(
            &upstream(),
            negotiated,
            &request(),
            &device(),
            described(&[]),
            &route::Seen::new(),
            &Pointed::new(),
        )
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
            delivery_method: Some(jellyfin_api::types::SubtitleDeliveryMethod::External),
            delivery_url: Some(format!(
                "/Videos/{}/{MEDIA_SOURCE}/Subtitles/{index}/0/Stream.vtt",
                uuid::Uuid::nil()
            )),
            ..MediaStream::default()
        }
    }

    fn source(streams: Vec<MediaStream>) -> MediaSourceInfo {
        MediaSourceInfo {
            id: Some(MEDIA_SOURCE.to_string()),
            container: Some("mkv".to_string()),
            media_streams: Some(streams),
            run_time_ticks: Some(100),
            ..MediaSourceInfo::default()
        }
    }

    fn negotiated(source: MediaSourceInfo) -> Negotiated {
        Negotiated {
            play_session: "session".to_string(),
            source,
            stream_url: None,
            sources: Vec::new(),
            live_stream: None,
            enable_direct_play: false,
            max_bitrate: jellium_protocol::Bitrate::of(1_500_000),
            live: false,
        }
    }

    /// A source the Jellyfin server says may be played as it lies, which is
    /// what `createStreamInfo`'s static-stream branch answers.
    fn streamable(mut source: MediaSourceInfo) -> MediaSourceInfo {
        source.supports_direct_play = Some(true);
        source
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
            ..negotiated(source)
        }
    }

    #[test]
    fn a_live_stream_is_delivered_from_the_static_stream_path() {
        let plan = planned(&live_negotiated(streamable(live_source(
            "/LiveTv/LiveStreamFiles/tuner-1/stream.ts",
            false,
        ))))
        .expect("a plan");
        assert!(plan.live);
        assert_eq!(plan.playable.method, Method::DirectPlay);
        assert!(plan.playable.path.starts_with("/jellyfin/Videos/"));
        assert!(plan.playable.path.contains("stream.ts"));
    }

    #[test]
    fn a_live_source_direct_played_from_a_path_outside_the_table_names_its_shape() {
        let refused = planned(&Negotiated {
            enable_direct_play: true,
            ..live_negotiated(live_source("/LiveTv/Tuners/abcdef/stream.ts", false))
        })
        .expect_err("a refusal");
        let PlaybackRefused::LiveNotRelayable { shape } = refused else {
            panic!("a live source outside the table is not relayable");
        };
        assert_eq!(shape, "/LiveTv/Tuners/abcdef/stream.ts");
    }

    #[test]
    fn a_refused_live_shape_carries_no_id_and_no_query() {
        let id = uuid::Uuid::nil();
        let refused = planned(&Negotiated {
            enable_direct_play: true,
            ..live_negotiated(live_source(
                &format!("/LiveTv/Tuners/{id}/stream.ts?api_key=secret"),
                false,
            ))
        })
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
            &upstream(),
            &live_negotiated(streamable(live_source(
                "/LiveTv/LiveStreamFiles/tuner-1/stream.ts",
                false,
            ))),
            &request(),
            &device(),
            described(&[ChapterInfo {
                name: Some("One".to_string()),
                start_position_ticks: Some(0),
                ..ChapterInfo::default()
            }]),
            &route::Seen::new(),
            &Pointed::new(),
        )
        .expect("a plan");
        assert!(plan.chapters.is_empty());
        assert_eq!(plan.run_time_ticks, None);
        assert!(plan.subtitle_streams.is_empty());
        assert_eq!(plan.subtitle_stream, None);
    }

    #[test]
    fn a_direct_play_is_delivered_from_a_same_origin_static_stream_path() {
        let plan = planned(&negotiated(streamable(source(vec![video_stream(0)])))).expect("a plan");
        assert_eq!(plan.playable.method, Method::DirectPlay);
        assert!(plan.playable.path.starts_with("/jellyfin/Videos/"));
        assert!(plan.playable.path.contains("stream.mkv"));
    }

    #[test]
    fn a_static_stream_carries_the_reference_keys_and_no_play_session() {
        let mut tagged = streamable(source(vec![video_stream(0)]));
        tagged.e_tag = Some("etag-1".to_string());
        let plan = planned(&negotiated(tagged)).expect("a plan");
        let query = plan
            .playable
            .path
            .split_once('?')
            .expect("the static stream carries a query")
            .1;
        let names: Vec<&str> = query
            .split('&')
            .filter_map(|pair| pair.split('=').next())
            .collect();
        assert_eq!(names, ["Static", "mediaSourceId", "deviceId", "Tag"]);
    }

    #[test]
    fn a_source_that_only_direct_streams_is_a_direct_stream() {
        let mut streamed = source(vec![video_stream(0)]);
        streamed.supports_direct_stream = Some(true);
        let plan = planned(&negotiated(streamed)).expect("a plan");
        assert_eq!(plan.playable.method, Method::DirectStream);
    }

    #[test]
    fn a_transcode_maps_the_negotiated_url_to_a_same_origin_manifest_path() {
        let mut transcoding = source(vec![video_stream(0)]);
        transcoding.supports_transcoding = Some(true);
        transcoding.transcoding_url = Some(format!(
            "/Videos/{}/master.m3u8?api_key=secret&PlaySessionId=session",
            uuid::Uuid::nil()
        ));
        transcoding.transcoding_sub_protocol = Some(MediaStreamProtocol::Hls);
        let plan = planned(&negotiated(transcoding)).expect("a plan");
        assert_eq!(
            plan.playable.sub_protocol,
            Some(jellium_protocol::profile::Protocol::Hls)
        );
        assert!(plan.playable.path.starts_with("/jellyfin/Videos/"));
        assert!(!plan.playable.path.contains("secret"));
    }

    #[test]
    fn a_transcode_behind_a_server_path_prefix_maps_to_a_relay_path() {
        let mut transcoding = source(vec![video_stream(0)]);
        transcoding.supports_transcoding = Some(true);
        transcoding.transcoding_url = Some(format!(
            "/Videos/{}/master.m3u8?api_key=secret&PlaySessionId=session",
            uuid::Uuid::nil()
        ));
        transcoding.transcoding_sub_protocol = Some(MediaStreamProtocol::Hls);
        let plan = build(
            &Upstream::stub("https://example.test/jellyfin"),
            &negotiated(transcoding),
            &request(),
            &device(),
            described(&[]),
            &route::Seen::new(),
            &Pointed::new(),
        )
        .expect("a plan");
        assert!(plan.playable.path.starts_with("/jellyfin/Videos/"));
        assert!(!plan.playable.path.contains("secret"));
    }

    #[test]
    fn an_external_subtitle_carries_the_servers_delivery_url_and_a_burned_in_one_carries_none() {
        let mut burned = subtitle_stream(2, "pgssub");
        burned.delivery_method = Some(jellyfin_api::types::SubtitleDeliveryMethod::Encode);
        let plan = planned(&negotiated(streamable(source(vec![
            video_stream(0),
            subtitle_stream(1, "subrip"),
            burned,
        ]))))
        .expect("a plan");
        assert_eq!(plan.subtitle_streams.len(), 2);
        let expected = format!(
            "/jellyfin/Videos/{}/{MEDIA_SOURCE}/Subtitles/1/0/Stream.vtt",
            uuid::Uuid::nil()
        );
        assert_eq!(
            plan.subtitle_streams[0].delivery,
            SubtitleDelivery::External { path: expected }
        );
        assert_eq!(plan.subtitle_streams[1].delivery, SubtitleDelivery::Encode);
    }

    #[test]
    fn a_source_whose_default_stream_indexes_name_no_stream_leaves_the_plan_naming_none() {
        let mut sentinelled =
            streamable(source(vec![video_stream(0), subtitle_stream(1, "subrip")]));
        sentinelled.default_audio_stream_index = Some(-1);
        sentinelled.default_subtitle_stream_index = Some(-1);
        let plan = planned(&negotiated(sentinelled)).expect("a plan");
        assert_eq!(plan.audio_stream, None);
        assert_eq!(plan.subtitle_stream, None);
    }

    #[test]
    fn the_streams_a_plan_starts_with_are_the_ones_the_server_named_as_defaults() {
        let mut defaulted = streamable(source(vec![video_stream(0), subtitle_stream(1, "subrip")]));
        defaulted.default_audio_stream_index = Some(0);
        defaulted.default_subtitle_stream_index = Some(1);
        let plan = build(
            &upstream(),
            &negotiated(defaulted),
            &PlayRequest {
                subtitles: Subtitles::Off,
                ..request()
            },
            &device(),
            described(&[]),
            &route::Seen::new(),
            &Pointed::new(),
        )
        .expect("a plan");
        assert_eq!(plan.audio_stream, StreamIndex::named(0));
        assert_eq!(plan.subtitle_stream, StreamIndex::named(1));
    }

    #[test]
    fn no_path_a_plan_carries_leaves_the_local_server_and_none_carries_the_token() {
        let plan = planned(&negotiated(streamable(source(vec![
            video_stream(0),
            subtitle_stream(1, "subrip"),
        ]))))
        .expect("a plan");
        let rendered = serde_json::to_string(&plan).expect("serialized");
        assert!(!rendered.contains("example.test"));
        assert!(!rendered.contains("http"));
        assert!(!rendered.contains("ApiKey"));
        assert!(!rendered.contains("token"));
    }
}
