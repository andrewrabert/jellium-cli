//! The subtitle streams the negotiated source offers, and where each of them
//! reaches the browser from.

use jellium_protocol::{StreamIndex, SubtitleChoice, SubtitleDelivery};
use jellyfin_api::types::{MediaSourceInfo, MediaStream, MediaStreamType, SubtitleDeliveryMethod};

use super::plan::{label, streams};
use super::pointed::Pointed;
use crate::web::{manifest, route};

/// How the Jellyfin server said this stream is delivered: the method it named,
/// and external or embedded by the stream's own flag when it named none.
// reference: get-delivery-method — playbackmanager.js:1500-1507
fn method(stream: &MediaStream) -> SubtitleDeliveryMethod {
    stream.delivery_method.unwrap_or({
        if stream.is_external == Some(true) {
            SubtitleDeliveryMethod::External
        } else {
            SubtitleDeliveryMethod::Embed
        }
    })
}

/// Every subtitle stream `source` carries, each named by the delivery the
/// Jellyfin server decided for it.
/// Only an external stream carries a path, and that path is the server's own
/// `DeliveryUrl`: one inside the Jellyfin server maps to a relay path, and one
/// outside it is minted into `pointed`.
/// A stream the server marks external while sending no `DeliveryUrl` a path
/// can be made of is not offered, since external carries a path and there is
/// none to carry. Every other method, dropped among them, is offered carrying
/// no path.
// reference: get-text-tracks — playbackmanager.js:2908-2939
pub fn offered(
    source: &MediaSourceInfo,
    base: &reqwest::Url,
    seen: &route::Seen,
    pointed: &Pointed,
) -> Vec<SubtitleChoice> {
    streams(source, MediaStreamType::Subtitle)
        .into_iter()
        .filter_map(|stream| {
            let index = StreamIndex::named(stream.index?)?;
            let delivery = match method(stream) {
                SubtitleDeliveryMethod::External => SubtitleDelivery::External {
                    path: manifest::resolved(
                        stream.delivery_url.as_deref()?,
                        base,
                        base,
                        seen,
                        pointed,
                    )
                    .ok()?,
                },
                SubtitleDeliveryMethod::Embed => SubtitleDelivery::Embed,
                SubtitleDeliveryMethod::Encode => SubtitleDelivery::Encode,
                SubtitleDeliveryMethod::Hls => SubtitleDelivery::Hls,
                SubtitleDeliveryMethod::Drop => SubtitleDelivery::Drop,
            };
            Some(SubtitleChoice {
                index,
                label: label(stream, index),
                language: stream.language.clone(),
                codec: stream.codec.clone(),
                delivery,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The `DeliveryUrl` the Jellyfin server answers for stream `index`.
    fn subtitle_url(index: i32) -> String {
        format!(
            "/Videos/{}/d0000000000000000000000000000000/Subtitles/{index}/0/Stream.vtt",
            uuid::Uuid::nil()
        )
    }

    fn base() -> reqwest::Url {
        reqwest::Url::parse("https://example.test").expect("base")
    }

    fn stream(index: i32, method: Option<SubtitleDeliveryMethod>, url: &str) -> MediaStream {
        MediaStream {
            index: Some(index),
            type_: Some(MediaStreamType::Subtitle),
            codec: Some("subrip".to_string()),
            delivery_method: method,
            delivery_url: Some(url.to_string()),
            ..MediaStream::default()
        }
    }

    fn source(streams: Vec<MediaStream>) -> MediaSourceInfo {
        MediaSourceInfo {
            media_streams: Some(streams),
            ..MediaSourceInfo::default()
        }
    }

    #[test]
    fn only_an_external_stream_carries_a_path_and_it_is_the_delivery_url() {
        let offered = offered(
            &source(vec![
                stream(1, Some(SubtitleDeliveryMethod::External), &subtitle_url(1)),
                stream(2, Some(SubtitleDeliveryMethod::Embed), "/unused"),
                stream(3, Some(SubtitleDeliveryMethod::Encode), "/unused"),
                stream(4, Some(SubtitleDeliveryMethod::Hls), "/unused"),
            ]),
            &base(),
            &route::Seen::new(),
            &Pointed::new(),
        );
        assert_eq!(offered.len(), 4);
        assert_eq!(
            offered[0].delivery,
            SubtitleDelivery::External {
                path: format!("/jellyfin{}", subtitle_url(1)),
            }
        );
        assert_eq!(offered[1].delivery, SubtitleDelivery::Embed);
        assert_eq!(offered[2].delivery, SubtitleDelivery::Encode);
        assert_eq!(offered[3].delivery, SubtitleDelivery::Hls);
    }

    #[test]
    fn a_stream_the_server_named_no_method_for_reads_its_own_flag() {
        let mut external = stream(1, None, &subtitle_url(1));
        external.is_external = Some(true);
        let offered = offered(
            &source(vec![external, stream(2, None, "/unused")]),
            &base(),
            &route::Seen::new(),
            &Pointed::new(),
        );
        assert!(matches!(
            offered[0].delivery,
            SubtitleDelivery::External { .. }
        ));
        assert_eq!(offered[1].delivery, SubtitleDelivery::Embed);
    }

    #[test]
    fn an_external_url_outside_the_jellyfin_server_is_pointed_at() {
        let pointed = Pointed::new();
        let offered = offered(
            &source(vec![stream(
                1,
                Some(SubtitleDeliveryMethod::External),
                "https://elsewhere.test/a.vtt",
            )]),
            &base(),
            &route::Seen::new(),
            &pointed,
        );
        let SubtitleDelivery::External { path } = &offered[0].delivery else {
            panic!("an external stream carries a path");
        };
        let handle = path
            .strip_prefix(&format!("{}/", jellium_protocol::POINTED_PREFIX))
            .expect("a handle path");
        assert_eq!(
            pointed.resolve(handle).as_deref(),
            Some("https://elsewhere.test/a.vtt")
        );
    }

    #[test]
    fn a_source_with_twenty_eight_streams_offers_no_path_the_server_did_not_name() {
        let streams = (1..=28)
            .map(|index| stream(index, Some(SubtitleDeliveryMethod::Encode), "/unused"))
            .collect();
        let offered = offered(
            &source(streams),
            &base(),
            &route::Seen::new(),
            &Pointed::new(),
        );
        assert_eq!(offered.len(), 28);
        assert!(
            offered
                .iter()
                .all(|choice| choice.delivery == SubtitleDelivery::Encode)
        );
    }
}
