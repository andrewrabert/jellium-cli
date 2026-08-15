use std::num::NonZeroU32;
use std::time::{Duration, Instant};

use crate::web::upstream::Upstream;

/// The local server's measurement of its own link to Jellyfin, which is the
/// link that can be slow.
pub struct Bandwidth {
    held: tokio::sync::Mutex<Option<(Instant, i32)>>,
}

impl Bandwidth {
    /// Bytes pulled from the bitrate-test endpoint per measurement.
    pub const SAMPLE: NonZeroU32 = NonZeroU32::new(1_000_000).expect("a non-zero sample");

    /// A measurement older than this is taken again.
    pub const TTL: Duration = Duration::from_secs(300);

    /// The share of the measured rate that becomes the ceiling.
    pub const HEADROOM: f64 = 0.8;

    pub fn new() -> Bandwidth {
        Bandwidth {
            held: tokio::sync::Mutex::new(None),
        }
    }

    /// The ceiling in bits per second, remeasured when the held measurement is
    /// older than `TTL`; a link that cannot be measured reads as `None` and
    /// the request carries no ceiling.
    pub async fn ceiling(&self, upstream: &Upstream) -> Option<i32> {
        let mut held = self.held.lock().await;
        if let Some((taken, ceiling)) = *held
            && taken.elapsed() < Self::TTL
        {
            return Some(ceiling);
        }
        let bytes_per_second = upstream.measure(Self::SAMPLE).await.ok()?;
        let ceiling = (bytes_per_second * 8.0 * Self::HEADROOM).round();
        let ceiling = ceiling.clamp(1.0, f64::from(i32::MAX)) as i32;
        *held = Some((Instant::now(), ceiling));
        Some(ceiling)
    }
}
