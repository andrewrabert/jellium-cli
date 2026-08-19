use std::time::Duration;

use serde::Deserialize;

/// `value` as whole and fractional seconds.
pub fn serialize<S: serde::Serializer>(value: &Duration, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_f64(value.as_secs_f64())
}

/// Seconds as a span; a negative or non-finite number is no time at all.
pub fn span(seconds: f64) -> Duration {
    if seconds.is_finite() && seconds > 0.0 {
        Duration::from_secs_f64(seconds)
    } else {
        Duration::ZERO
    }
}

pub fn deserialize<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Duration, D::Error> {
    Ok(span(f64::deserialize(deserializer)?))
}
