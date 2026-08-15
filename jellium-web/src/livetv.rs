use chrono::{DateTime, Utc};

pub use jellium_model::livetv::{Channel, Program};

/// The start and end of a program, as the display and the guide write them.
pub fn airtime(program: &Program) -> String {
    let format = |at: DateTime<Utc>| {
        chrono::DateTime::<chrono::Local>::from(at)
            .format("%H:%M")
            .to_string()
    };
    crate::text::format(
        crate::text::Text::ProgramAirtime,
        &[&format(program.start), &format(program.end)],
    )
}
