use std::sync::Arc;
use std::time::Duration;

use jellium_protocol::sync::Exchange;

use crate::web::AppState;
use crate::web::upstream::Upstream;

/// The Jellyfin server's clock as this machine sees it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Estimate {
    /// The Jellyfin server's clock minus this machine's, in milliseconds.
    pub offset: i64,
    /// The upstream round trip, in milliseconds.
    pub round_trip: i64,
}

/// Milliseconds since the unix epoch, on this machine's clock.
fn millis(when: chrono::DateTime<chrono::Utc>) -> i64 {
    when.timestamp_millis()
}

fn now() -> i64 {
    millis(chrono::Utc::now())
}

/// The offset held between measurements, and the task that refreshes it.
pub struct Clock {
    held: tokio::sync::RwLock<Option<Estimate>>,
    running: tokio::sync::Mutex<Option<tokio::task::JoinHandle<()>>>,
}

impl Clock {
    /// The wait between the samples taken while converging, which holds the
    /// upstream rate to two requests a second.
    pub const CONVERGING: Duration = Duration::from_millis(500);

    /// How many samples are taken before the cadence settles.
    pub const SAMPLES: u32 = 4;

    /// The wait between samples once converged.
    pub const SETTLED: Duration = Duration::from_secs(30);

    pub fn new() -> Clock {
        Clock {
            held: tokio::sync::RwLock::new(None),
            running: tokio::sync::Mutex::new(None),
        }
    }

    /// Starts measuring: `SAMPLES` taken `CONVERGING` apart, then one every
    /// `SETTLED` until `stop`.
    /// A start while it already runs changes nothing.
    pub async fn start(&self, state: Arc<AppState>) {
        let mut running = self.running.lock().await;
        if running.is_some() {
            return;
        }
        *running = Some(tokio::spawn(async move {
            let mut taken = 0u32;
            loop {
                if let Some(upstream) = state.session.signed().await
                    && let Some(estimate) = sample(&upstream).await
                {
                    *state.live.clock.held.write().await = Some(estimate);
                }
                taken = taken.saturating_add(1);
                let wait = if taken < Clock::SAMPLES {
                    Clock::CONVERGING
                } else {
                    Clock::SETTLED
                };
                tokio::time::sleep(wait).await;
            }
        }));
    }

    /// Stops measuring and leaves the last estimate standing.
    pub async fn stop(&self) {
        if let Some(task) = self.running.lock().await.take() {
            task.abort();
        }
    }

    /// The estimate held now, however old; a sample that failed leaves the one
    /// before it standing rather than clearing it.
    pub async fn estimate(&self) -> Option<Estimate> {
        *self.held.read().await
    }

    /// `when`, an instant on the Jellyfin server's clock, as milliseconds since
    /// the unix epoch on this machine's clock.
    /// With no estimate yet, the two clocks are taken as one.
    pub async fn locally(&self, when: chrono::DateTime<chrono::Utc>) -> i64 {
        let offset = self.estimate().await.map_or(0, |held| held.offset);
        millis(when) - offset
    }

    /// Now, on the Jellyfin server's clock, which is what a buffering or ready
    /// call is stamped with.
    pub async fn upstream_now(&self) -> chrono::DateTime<chrono::Utc> {
        let offset = self.estimate().await.map_or(0, |held| held.offset);
        chrono::Utc::now() + chrono::Duration::milliseconds(offset)
    }
}

/// One `/GetUtcTime` exchange, timed on this machine and read off the two
/// instants the answer carries.
pub async fn sample(upstream: &Upstream) -> Option<Estimate> {
    let sent = now();
    let answer = upstream.utc_time().await.ok()?;
    let returned = now();
    let exchange = Exchange {
        sent,
        received: millis(answer.request_reception_time?),
        answered: millis(answer.response_transmission_time?),
        returned,
    };
    Some(Estimate {
        offset: exchange.offset(),
        round_trip: exchange.round_trip(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::web::upstream::answering;

    fn scratch(name: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir()
            .join("jellium-cli-clock-tests")
            .join(name);
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("scratch directory");
        path.join("session.env")
    }

    #[tokio::test]
    async fn a_sample_against_a_skewed_clock_reads_the_skew() {
        let server = answering(204).await;
        server.skewed(5_000, Duration::ZERO);
        let upstream = Upstream::stub(&server.base);
        let estimate = sample(&upstream).await.expect("a sample");
        assert!(
            (estimate.offset - 5_000).abs() < 200,
            "the skew read as {}",
            estimate.offset
        );
        assert!(estimate.round_trip >= 0);
    }

    #[tokio::test]
    async fn a_scheduled_instant_converts_at_the_last_estimate_when_a_sample_fails() {
        let server = answering(204).await;
        server.skewed(5_000, Duration::ZERO);
        let upstream = Upstream::stub(&server.base);
        let clock = Clock::new();
        *clock.held.write().await = sample(&upstream).await;
        let estimate = clock.estimate().await.expect("an estimate");

        // a sample that fails leaves the estimate standing
        let refusing = Upstream::stub("http://127.0.0.1:1");
        assert_eq!(sample(&refusing).await, None);
        assert_eq!(clock.estimate().await, Some(estimate));

        let when = chrono::Utc::now() + chrono::Duration::milliseconds(5_000);
        let converted = clock.locally(when).await;
        assert!(
            (converted - now()).abs() < 500,
            "the conversion landed {} ms away",
            converted - now()
        );
    }

    #[tokio::test]
    async fn the_converging_cadence_issues_no_more_than_two_requests_a_second() {
        let server = answering(204).await;
        let state = Arc::new(AppState::stub(scratch("converging")));
        state.session.install(Upstream::stub(&server.base)).await;
        state.live.clock.start(state.clone()).await;
        tokio::time::sleep(Duration::from_millis(1_000)).await;
        state.live.clock.stop().await;
        assert!(
            server.asked("/GetUtcTime") <= 2,
            "{} requests in one second",
            server.asked("/GetUtcTime")
        );
    }

    #[tokio::test]
    async fn a_settled_clock_issues_no_more_than_one_request_every_thirty_seconds() {
        let server = answering(204).await;
        let state = Arc::new(AppState::stub(scratch("settled")));
        state.session.install(Upstream::stub(&server.base)).await;
        state.live.clock.start(state.clone()).await;
        tokio::time::sleep(Clock::CONVERGING * Clock::SAMPLES + Duration::from_millis(1_000)).await;
        state.live.clock.stop().await;
        assert_eq!(
            server.asked("/GetUtcTime"),
            Clock::SAMPLES as usize,
            "the cadence did not settle after {} samples",
            Clock::SAMPLES
        );
    }
}
