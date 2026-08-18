//! The span between two moments, in the words date-fns picks for it.

use chrono::{Datelike, TimeZone};

const MINUTES_IN_DAY: u32 = 1440;
const MINUTES_IN_ALMOST_TWO_DAYS: u32 = 2520;
const MINUTES_IN_MONTH: u32 = 43200;
const MINUTES_IN_TWO_MONTHS: u32 = 86400;

/// One phrase date-fns picks for the span between two moments, carrying the
/// count it writes into that phrase.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Distance {
    LessThanMinutes(u32),
    Minutes(u32),
    AboutHours(u32),
    Days(u32),
    AboutMonths(u32),
    Months(u32),
    AboutYears(u32),
    OverYears(u32),
    AlmostYears(u32),
}

impl Distance {
    // the span is read whichever way round the two moments stand
    // the months beyond two are counted in calendar months in the zone the
    // browser stands in, which is the zone date-fns counts them in
    // reference: date-fns-distance
    // reference: date-fns-distance-steps
    // reference: date-fns-distance-en-us
    pub fn between(
        from: chrono::DateTime<chrono::Utc>,
        to: chrono::DateTime<chrono::Utc>,
    ) -> Distance {
        let earlier = from.min(to);
        let later = from.max(to);
        let minutes = minutes(earlier, later);

        if minutes < 2 {
            return match minutes {
                0 => Distance::LessThanMinutes(1),
                count => Distance::Minutes(count),
            };
        }
        if minutes < 45 {
            return Distance::Minutes(minutes);
        }
        if minutes < 90 {
            return Distance::AboutHours(1);
        }
        if minutes < MINUTES_IN_DAY {
            return Distance::AboutHours(rounded(i64::from(minutes), 60));
        }
        if minutes < MINUTES_IN_ALMOST_TWO_DAYS {
            return Distance::Days(1);
        }
        if minutes < MINUTES_IN_MONTH {
            return Distance::Days(rounded(i64::from(minutes), i64::from(MINUTES_IN_DAY)));
        }
        if minutes < MINUTES_IN_TWO_MONTHS {
            return Distance::AboutMonths(rounded(i64::from(minutes), i64::from(MINUTES_IN_MONTH)));
        }

        let months = months(earlier, later);
        if months < 12 {
            return Distance::Months(rounded(i64::from(minutes), i64::from(MINUTES_IN_MONTH)));
        }
        let years = months / 12;
        match months % 12 {
            0..3 => Distance::AboutYears(years),
            3..9 => Distance::OverYears(years),
            _ => Distance::AlmostYears(years + 1),
        }
    }

    /// The count the phrase writes, which is what parts its singular from its
    /// plural.
    pub fn count(self) -> u32 {
        match self {
            Distance::LessThanMinutes(count)
            | Distance::Minutes(count)
            | Distance::AboutHours(count)
            | Distance::Days(count)
            | Distance::AboutMonths(count)
            | Distance::Months(count)
            | Distance::AboutYears(count)
            | Distance::OverYears(count)
            | Distance::AlmostYears(count) => count,
        }
    }
}

/// Which side of the moment it is measured from a distance falls on, which is
/// what decides the suffix date-fns writes around the phrase.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sense {
    Passed,
    Ahead,
}

impl Sense {
    // reference: date-fns-distance
    pub fn of(at: chrono::DateTime<chrono::Utc>, now: chrono::DateTime<chrono::Utc>) -> Sense {
        if at > now {
            Sense::Ahead
        } else {
            Sense::Passed
        }
    }
}

/// The whole minutes between the two moments, less what the zone the browser
/// stands in shifted between them.
// reference: date-fns-distance
fn minutes(earlier: chrono::DateTime<chrono::Utc>, later: chrono::DateTime<chrono::Utc>) -> u32 {
    let seconds = (later - earlier).num_seconds();
    let shift = i64::from(zoned(later).offset().local_minus_utc())
        - i64::from(zoned(earlier).offset().local_minus_utc());
    rounded(seconds - shift, 60)
}

/// The whole calendar months between the two moments, read in the zone the
/// browser stands in.
// reference: date-fns-months
// reference: date-fns-calendar-months
fn months(earlier: chrono::DateTime<chrono::Utc>, later: chrono::DateTime<chrono::Utc>) -> u32 {
    let right = zoned(earlier).naive_local();
    let left = zoned(later).naive_local();
    let difference = i64::from(left.year() - right.year()) * 12 + i64::from(left.month0())
        - i64::from(right.month0());
    if difference < 1 {
        return 0;
    }

    let raised = match left.month0() == 1 && left.day() > 27 {
        true => walked(left, 1, 30),
        false => left,
    };
    let shifted = walked(
        raised,
        i64::from(raised.month0()) - difference,
        i64::from(raised.day()),
    );
    let whole_month = last_day(left) && difference == 1 && left > right;
    u32::try_from(difference - i64::from(shifted < right && !whole_month)).unwrap_or(0)
}

/// The moment the given month and day name, counting a month outside the year
/// and a day outside the month on into the months and years around them, which
/// is what a `Date`'s own setters do.
fn walked(from: chrono::NaiveDateTime, month0: i64, day: i64) -> chrono::NaiveDateTime {
    let year = i64::from(from.year()) + month0.div_euclid(12);
    let month = month0.rem_euclid(12) as u32 + 1;
    let opening = i32::try_from(year)
        .ok()
        .and_then(|year| chrono::NaiveDate::from_ymd_opt(year, month, 1));
    match opening {
        Some(opening) => (opening + chrono::Duration::days(day - 1)).and_time(from.time()),
        None => from,
    }
}

/// Whether the moment stands on the last day of its own month.
// reference: date-fns-last-day
fn last_day(at: chrono::NaiveDateTime) -> bool {
    at.date()
        .succ_opt()
        .is_none_or(|next| next.month0() != at.month0())
}

/// The moment as the zone the browser stands in reads it.
fn zoned(at: chrono::DateTime<chrono::Utc>) -> chrono::DateTime<chrono::Local> {
    chrono::Local.from_utc_datetime(&at.naive_utc())
}

/// The quotient at the nearest whole number, halves reading upwards, which is
/// the rounding JavaScript's own does.
fn rounded(numerator: i64, denominator: i64) -> u32 {
    u32::try_from((numerator + denominator / 2).div_euclid(denominator)).unwrap_or(0)
}
