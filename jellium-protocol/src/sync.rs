use serde::{Deserialize, Serialize};

/// One timing exchange between an asking clock and an answering one, in
/// milliseconds since the unix epoch on each side's own clock.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Exchange {
    /// The asking clock when the request left.
    pub sent: i64,
    /// The answering clock when the request arrived.
    pub received: i64,
    /// The answering clock when the answer left.
    pub answered: i64,
    /// The asking clock when the answer arrived.
    pub returned: i64,
}

impl Exchange {
    /// The answering clock minus the asking clock.
    pub fn offset(&self) -> i64 {
        ((self.received - self.sent) + (self.answered - self.returned)) / 2
    }

    /// The round trip, with the answering side's own delay taken out.
    pub fn round_trip(&self) -> i64 {
        (self.returned - self.sent) - (self.answered - self.received)
    }
}

/// What the group's schedule says the position is: `position_ticks` at `at`,
/// advancing in real time while `running`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
    pub position_ticks: i64,
    /// Milliseconds since the unix epoch, on the clock `now` is read from.
    pub at: i64,
    pub running: bool,
}

/// Ticks in one millisecond.
const TICKS_PER_MILLI: i64 = 10_000;

impl Schedule {
    /// The position the schedule gives at `now`.
    pub fn position_ticks(&self, now: i64) -> i64 {
        if self.running {
            self.position_ticks + (now - self.at) * TICKS_PER_MILLI
        } else {
            self.position_ticks
        }
    }
}

/// A drift inside this is left alone.
pub const DRIFT_TOLERANCE: i64 = 400;

/// A drift beyond this is corrected by seeking rather than by rate.
pub const DRIFT_SEEK: i64 = 2_000;

/// A rate correction absorbs the drift over this long.
pub const NUDGE: i64 = 1_000;

/// The slowest a rate correction plays.
pub const RATE_FLOOR: f64 = 0.5;

/// The fastest a rate correction plays.
pub const RATE_CEILING: f64 = 2.0;

/// What a drift from the group's schedule calls for.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Correction {
    /// Inside `DRIFT_TOLERANCE`; nothing is done.
    Hold,
    /// The rate to play at for `NUDGE`, clamped to `RATE_FLOOR` and
    /// `RATE_CEILING`.
    Rate(f64),
    /// Beyond `DRIFT_SEEK`; the element moves to the schedule's position.
    Seek,
}

/// How a drift from the group's schedule is corrected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SyncMethod {
    /// Nudges the rate up to `DRIFT_SEEK` and seeks beyond it.
    Auto,
    /// Never seeks.
    Rate,
    /// Never nudges.
    Seek,
}

/// What this installation's SyncPlay preferences ask of every correction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tuning {
    /// Added to the schedule's position, in milliseconds, before the drift is
    /// measured.
    pub extra_offset_ms: i64,
    pub method: SyncMethod,
    /// Consecutive rate corrections made before corrections are held.
    pub rate_attempts: u32,
    /// Consecutive seek corrections made before corrections are held.
    pub seek_attempts: u32,
}

impl Tuning {
    /// No offset, `SyncMethod::Auto`, and five attempts of each kind.
    pub const DEFAULT: Tuning = Tuning {
        extra_offset_ms: 0,
        method: SyncMethod::Auto,
        rate_attempts: 5,
        seek_attempts: 5,
    };
}

impl Default for Tuning {
    fn default() -> Tuning {
        Tuning::DEFAULT
    }
}

/// How many consecutive corrections of each kind have been made since the drift
/// was last inside the tolerance.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Attempts {
    pub rate: u32,
    pub seek: u32,
}

impl Attempts {
    /// Counts `correction`; a `Hold` clears both counts.
    pub fn made(&mut self, correction: Correction) {
        match correction {
            Correction::Hold => *self = Attempts::default(),
            Correction::Rate(_) => self.rate += 1,
            Correction::Seek => self.seek += 1,
        }
    }
}

/// The correction `drift` calls for under `tuning`, given the corrections
/// `attempts` counts; `drift` is how many milliseconds ahead of the schedule the
/// element is, negative when behind.
/// A drift inside `DRIFT_TOLERANCE` holds, whatever the method, and a kind whose
/// attempts are spent holds until the drift returns inside the tolerance.
pub fn correction(drift: i64, tuning: Tuning, attempts: Attempts) -> Correction {
    let magnitude = drift.abs();
    if magnitude <= DRIFT_TOLERANCE {
        return Correction::Hold;
    }
    let seeking = match tuning.method {
        SyncMethod::Auto => magnitude > DRIFT_SEEK,
        SyncMethod::Rate => false,
        SyncMethod::Seek => true,
    };
    if seeking {
        if attempts.seek >= tuning.seek_attempts {
            return Correction::Hold;
        }
        Correction::Seek
    } else {
        if attempts.rate >= tuning.rate_attempts {
            return Correction::Hold;
        }
        let rate = 1.0 - (drift as f64) / (NUDGE as f64);
        Correction::Rate(rate.clamp(RATE_FLOOR, RATE_CEILING))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Attempts, Correction, DRIFT_SEEK, DRIFT_TOLERANCE, Exchange, RATE_CEILING, RATE_FLOOR,
        Schedule, SyncMethod, Tuning, correction,
    };

    fn corrected(drift: i64) -> Correction {
        correction(drift, Tuning::DEFAULT, Attempts::default())
    }

    #[test]
    fn an_exchange_reads_the_offset_a_skewed_clock_carries() {
        let exchange = Exchange {
            sent: 1_000,
            received: 6_020,
            answered: 6_020,
            returned: 1_040,
        };
        assert_eq!(exchange.offset(), 5_000);
    }

    #[test]
    fn an_exchange_takes_the_answering_sides_delay_out_of_the_round_trip() {
        let exchange = Exchange {
            sent: 1_000,
            received: 2_000,
            answered: 2_300,
            returned: 1_500,
        };
        assert_eq!(exchange.round_trip(), 200);
    }

    #[test]
    fn a_running_schedule_advances_with_the_clock_and_a_stopped_one_does_not() {
        let running = Schedule {
            position_ticks: 0,
            at: 1_000,
            running: true,
        };
        assert_eq!(running.position_ticks(2_000), 10_000_000);
        let stopped = Schedule {
            running: false,
            ..running
        };
        assert_eq!(stopped.position_ticks(2_000), 0);
    }

    #[test]
    fn a_drift_inside_the_tolerance_is_held() {
        assert_eq!(corrected(0), Correction::Hold);
        assert_eq!(corrected(DRIFT_TOLERANCE), Correction::Hold);
        assert_eq!(corrected(-DRIFT_TOLERANCE), Correction::Hold);
    }

    #[test]
    fn a_drift_between_the_tolerance_and_the_seek_bar_nudges_the_rate() {
        assert_eq!(corrected(500), Correction::Rate(0.5));
        assert_eq!(corrected(-500), Correction::Rate(1.5));
    }

    #[test]
    fn a_nudge_is_clamped_to_the_floor_and_the_ceiling() {
        assert_eq!(corrected(DRIFT_SEEK), Correction::Rate(RATE_FLOOR));
        assert_eq!(corrected(-DRIFT_SEEK), Correction::Rate(RATE_CEILING));
    }

    #[test]
    fn a_drift_beyond_the_seek_bar_seeks() {
        assert_eq!(corrected(DRIFT_SEEK + 1), Correction::Seek);
        assert_eq!(corrected(-DRIFT_SEEK - 1), Correction::Seek);
    }

    #[test]
    fn the_rate_method_never_seeks_and_the_seek_method_never_nudges() {
        let rate = Tuning {
            method: SyncMethod::Rate,
            ..Tuning::DEFAULT
        };
        assert_eq!(
            correction(DRIFT_SEEK * 10, rate, Attempts::default()),
            Correction::Rate(RATE_FLOOR)
        );
        let seek = Tuning {
            method: SyncMethod::Seek,
            ..Tuning::DEFAULT
        };
        assert_eq!(
            correction(DRIFT_TOLERANCE + 1, seek, Attempts::default()),
            Correction::Seek
        );
    }

    #[test]
    fn a_kind_whose_attempts_are_spent_holds() {
        let spent = Attempts { rate: 5, seek: 5 };
        assert_eq!(correction(500, Tuning::DEFAULT, spent), Correction::Hold);
        assert_eq!(
            correction(DRIFT_SEEK + 1, Tuning::DEFAULT, spent),
            Correction::Hold
        );
        let none = Tuning {
            rate_attempts: 0,
            seek_attempts: 0,
            ..Tuning::DEFAULT
        };
        assert_eq!(correction(500, none, Attempts::default()), Correction::Hold);
    }

    #[test]
    fn a_hold_clears_both_counts_and_every_other_correction_counts_its_kind() {
        let mut attempts = Attempts::default();
        attempts.made(Correction::Rate(0.5));
        attempts.made(Correction::Rate(0.5));
        attempts.made(Correction::Seek);
        assert_eq!(attempts, Attempts { rate: 2, seek: 1 });
        attempts.made(Correction::Hold);
        assert_eq!(attempts, Attempts::default());
    }
}
