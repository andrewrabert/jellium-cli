//! jellyfin-web's bitrate ladder, run from the relay against Jellyfin, since
//! the relay's own link is the one that can be slow.

use std::time::{Duration, Instant};

use jellium_protocol::{Bitrate, Failure, Quality, profile::MediaKind};

use super::reachable::in_network;
use crate::session::SessionFile;
use crate::web::link::unreachable;
use crate::web::upstream::Upstream;
use crate::web::wire::{self, Query};

/// One rung of the ladder: the sample it pulls, and the rate below which it is
/// the answer rather than a step toward the next.
struct Rung {
    bytes: u32,
    threshold: Bitrate,
}

// reference: detect-bitrate-with-endpoint-info — apiClient.js:4153-4180
const LADDER: [Rung; 3] = [
    Rung {
        bytes: 500_000,
        threshold: Bitrate::of(500_000),
    },
    Rung {
        bytes: 1_000_000,
        threshold: Bitrate::of(20_000_000),
    },
    Rung {
        bytes: 3_000_000,
        threshold: Bitrate::of(50_000_000),
    },
];

// reference: bitratetest-timeout — apiClient.js:16
const DEADLINE: Duration = Duration::from_secs(5);

// reference: max-bitrate — apiClient.js:12
const CEILING: Bitrate = Bitrate::of(2_147_483_647);

/// The floor a live measurement is raised to on an in-network link, applied at
/// `apiClient.js:4172-4173`.
// reference: lan-bitrate — apiClient.js:14
const IN_NETWORK_FLOOR: Bitrate = Bitrate::of(140_000_000);

// reference: max-streaming-bitrate-setting — appSettings.js:73-89
const DEFAULT: Bitrate = Bitrate::of(1_500_000);

/// `appSettings.js:83-85`, reached only when the saved value is read: the store
/// at `:76-77` is a no-op for in-network audio, not a short circuit.
const SAVED_IN_NETWORK_AUDIO: Bitrate = Bitrate::of(150_000_000);

/// What a measurement is keyed by and persisted under, the way `appSettings`
/// keys `maxbitrate-{mediaType}-{isInNetwork}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Keyed {
    pub media: MediaKind,
    pub in_network: bool,
}

impl Keyed {
    /// True where `appSettings.maxStreamingBitrate` neither stores what it is
    /// handed nor reads what is stored, answering a constant instead.
    fn constant(self) -> bool {
        self.media == MediaKind::Audio && self.in_network
    }

    /// The session-file key this measurement is held under.
    pub fn entry(self) -> String {
        let media = match self.media {
            MediaKind::Audio => "AUDIO",
            MediaKind::Video => "VIDEO",
        };
        let network = if self.in_network {
            "INNETWORK"
        } else {
            "OUTOFNETWORK"
        };
        format!("JELLYFIN_MAXBITRATE_{media}_{network}")
    }
}

/// The ladder's answers, held for `MEMO` the way `lastDetectedBitrate` is held.
pub struct Bandwidth {
    held: tokio::sync::Mutex<std::collections::HashMap<Keyed, (Instant, Bitrate)>>,
    /// The session file a measurement is persisted into and read back from.
    session: std::path::PathBuf,
}

impl Bandwidth {
    /// A measurement younger than this is answered rather than taken again.
    // reference: detect-bitrate-memo — apiClient.js:825-840
    pub const MEMO: Duration = Duration::from_secs(3600);

    pub fn new(session: std::path::PathBuf) -> Bandwidth {
        Bandwidth {
            held: tokio::sync::Mutex::new(std::collections::HashMap::new()),
            session,
        }
    }

    /// The ceiling this start goes upstream with.
    /// `Quality::Auto` runs the ladder and saves the result; `Quality::Limit`
    /// saves and returns the chosen ceiling; a failed or skipped run answers
    /// the saved value, which is `SAVED_IN_NETWORK_AUDIO` for in-network audio
    /// and the stored entry or `DEFAULT` otherwise.
    /// In-network audio measures like any other start: `appSettings.js:66-67`
    /// answers `enableAutomaticBitrateDetection` `true` unconditionally for it,
    /// which is the one case detection can never be turned off, so the ladder
    /// runs and its three `Playback/BitrateTest` requests go out.
    /// Every path ends at a measurement, the saved entry or `DEFAULT`, so a
    /// ceiling is always answered.
    // reference: detect-bitrate — playbackmanager.js:2575-2596
    pub async fn ceiling(
        &self,
        upstream: &Upstream,
        quality: Quality,
        media: MediaKind,
    ) -> Bitrate {
        let Ok(in_network) = in_network(upstream).await else {
            return saved(
                &self.session,
                Keyed {
                    media,
                    in_network: false,
                },
            )
            .await;
        };
        let keyed = Keyed { media, in_network };
        let chosen = match quality {
            Quality::Limit { bits_per_second } => Some(bits_per_second),
            Quality::Auto => self.detected(upstream, keyed).await,
        };
        match chosen {
            Some(bitrate) => {
                store(&self.session, keyed, bitrate).await;
                bitrate
            }
            None => saved(&self.session, keyed).await,
        }
    }

    /// The ladder's answer for `keyed`, normalized and floored, and `None` when
    /// no rung answered and no earlier measurement stands in.
    // reference: detect-bitrate-memo — apiClient.js:825-840
    async fn detected(&self, upstream: &Upstream, keyed: Keyed) -> Option<Bitrate> {
        let mut held = self.held.lock().await;
        if let Some((taken, bitrate)) = held.get(&keyed).copied()
            && taken.elapsed() <= Bandwidth::MEMO
        {
            return Some(bitrate);
        }
        let measured = ladder(upstream)
            .await
            .filter(|bitrate| bitrate.bits_per_second() > 0);
        let answered = match measured {
            Some(bitrate) => normalized(bitrate),
            None => held.get(&keyed).map(|(_, bitrate)| *bitrate)?,
        };
        let answered = if keyed.in_network {
            Bitrate::of(
                answered
                    .bits_per_second()
                    .max(IN_NETWORK_FLOOR.bits_per_second()),
            )
        } else {
            answered
        };
        held.insert(keyed, (Instant::now(), answered));
        Some(answered)
    }
}

/// The rung the ladder stops on, and `None` when the first rung failed with
/// nothing measured before it.
// reference: detect-bitrate-with-endpoint-info — apiClient.js:4153-4180
async fn ladder(upstream: &Upstream) -> Option<Bitrate> {
    let mut current = None;
    for rung_of in &LADDER {
        match rung(upstream, rung_of.bytes).await {
            Ok(bitrate) if bitrate.bits_per_second() < rung_of.threshold.bits_per_second() => {
                return Some(bitrate);
            }
            Ok(bitrate) => current = Some(bitrate),
            Err(_) => return current,
        }
    }
    current
}

/// One `GET Playback/BitrateTest`, timed from the response head and abandoned
/// after `DEADLINE`.
// reference: get-download-speed — apiClient.js:761-823
async fn rung(upstream: &Upstream, bytes: u32) -> Result<Bitrate, Failure> {
    let link = upstream.link();
    let server = link.server();
    let query = Query::new().set("Size", bytes);
    let target = wire::url(link.base(), "Playback/BitrateTest", &query);
    let mut response = upstream
        .streaming()
        .get(target)
        .header(reqwest::header::CACHE_CONTROL, "no-cache, no-store")
        .timeout(DEADLINE)
        .send()
        .await
        .map_err(|error| unreachable(server, error))?;
    let status = response.status().as_u16();
    if status >= 400 {
        return Err(unreachable(
            server,
            format!("BitrateTest failed with {status} status"),
        ));
    }
    let started = Instant::now();
    let mut read = 0u64;
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|error| unreachable(server, error))?
    {
        read += chunk.len() as u64;
    }
    let seconds = started.elapsed().as_secs_f64();
    Ok(Bitrate::of((read as f64 / seconds * 8.0).round() as i64))
}

/// `bitrate` with jellyfin-web's headroom taken off and `CEILING` applied.
// reference: normalize-return-bitrate — apiClient.js:4109-4132
fn normalized(bitrate: Bitrate) -> Bitrate {
    let taken = (bitrate.bits_per_second() as f64 * 0.7).round() as i64;
    Bitrate::of(taken.min(CEILING.bits_per_second()))
}

/// The value `keyed` reads as when no run measured one.
// reference: max-streaming-bitrate-setting — appSettings.js:73-89
async fn saved(session: &std::path::Path, keyed: Keyed) -> Bitrate {
    if keyed.constant() {
        return SAVED_IN_NETWORK_AUDIO;
    }
    SessionFile::load_async(session.to_path_buf())
        .await
        .ok()
        .and_then(|file| file.max_bitrate(keyed))
        .unwrap_or(DEFAULT)
}

/// Saves `bitrate` under `keyed`, which for in-network audio stores nothing,
/// the way `appSettings.js:76-77` stores nothing.
// reference: max-streaming-bitrate-setting — appSettings.js:73-89
async fn store(session: &std::path::Path, keyed: Keyed, bitrate: Bitrate) {
    if keyed.constant() {
        return;
    }
    if let Err(e) = SessionFile::update_async(session.to_path_buf(), move |file| {
        file.set_max_bitrate(keyed, bitrate);
    })
    .await
    {
        eprintln!("jellium-cli web: saving the measured bitrate: {e}");
    }
}
