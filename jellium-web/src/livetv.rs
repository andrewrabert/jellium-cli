use chrono::{DateTime, Utc};

pub use jellium_model::livetv::{Badge, Channel, Marque, Program, Recording};

use crate::style::card;
use crate::text::{self as strings, Template};

/// A time of day as `getDisplayTime` writes one.
// reference: card-air-time
pub fn clock(at: DateTime<Utc>) -> String {
    DateTime::<chrono::Local>::from(at)
        .format("%H:%M")
        .to_string()
}

/// The line `showAirTime` writes, as `getAirTimeText` writes it, and none where
/// the airing names no start.
// reference: card-air-time
pub fn aired(
    shape: card::AirTime,
    start: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
) -> Option<card::Caption> {
    let start = start?;
    let written = match (shape, end) {
        (card::AirTime::Ended, Some(end)) => {
            strings::format(Template::ProgramAirtime, &[&clock(start), &clock(end)])
        }
        (card::AirTime::Ended, None) => clock(start),
        (card::AirTime::Dated, _) => strings::format(
            Template::ProgramAirDate,
            &[
                &DateTime::<chrono::Local>::from(start)
                    .format("%a, %b %-d")
                    .to_string(),
                &clock(start),
            ],
        ),
    };
    card::Caption::of(written)
}

/// The start and end of a program, as the display and the guide write them.
pub fn airtime(program: &Program) -> String {
    strings::format(
        Template::ProgramAirtime,
        &[&clock(program.start), &clock(program.end)],
    )
}
