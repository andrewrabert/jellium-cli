//! `stopActiveEncodings`, which a stream change issues before the source is
//! swapped and again after it.

use jellium_protocol::Failure;

use crate::web::identity::Identity;
use crate::web::upstream::Upstream;
use crate::web::wire::{self, Query};

/// Stops the encodes `play_session` left running, naming the announced device
/// and then the session, in that order.
// reference: stop-active-encodings — apiClient.js:2043-2057
pub async fn stop(
    upstream: &Upstream,
    identity: &Identity,
    play_session: &str,
) -> Result<(), Failure> {
    let query = Query::new()
        .set("deviceId", identity.device_id())
        .set("PlaySessionId", play_session);
    wire::deleted(upstream, "Videos/ActiveEncodings", &query).await
}
