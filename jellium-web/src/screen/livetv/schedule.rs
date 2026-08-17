use std::borrow::Cow;
use std::rc::Rc;

use iced::widget::{button, column, container};
use iced::{Element, Fill};
use jellyfin_api::types::TimerInfoDto;

use super::Action;
use crate::api::Api;
use crate::app::Message;
use crate::error::Answer;
use crate::style::{self, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::{self, prose};

#[derive(Debug, Clone)]
pub struct State {
    /// Upcoming timers ordered by start time.
    pub timers: Vec<TimerInfoDto>,
}

/// Every row of the schedule: three lines and nothing before them.
const ROW: space::ListRow = space::ListRow::bare(space::Lines::Three);

pub async fn load(api: Rc<Api>) -> Answer<State> {
    Answer::of(async {
        Ok(State {
            timers: api.timers().await.bubbled()?,
        })
    })
    .await
}

/// One run of timers sharing a day: the heading the reference writes over the
/// run, and none where its timers carry no start.
struct Day<'a> {
    named: Option<String>,
    timers: &'a [TimerInfoDto],
}

/// The day a timer falls under, written as the reference writes it: the weekday
/// in full, the month short, the day of the month.
// reference: schedule-groups
fn named(at: chrono::DateTime<chrono::Utc>) -> String {
    chrono::DateTime::<chrono::Local>::from(at)
        .format("%A, %b %-d")
        .to_string()
}

/// The runs of consecutive timers sharing a day, in the order they arrive.
// reference: schedule-groups
fn days(timers: &[TimerInfoDto]) -> Vec<Day<'_>> {
    let mut days: Vec<Day<'_>> = Vec::new();
    let mut start = 0;
    for index in 0..timers.len() {
        let ends = timers
            .get(index + 1)
            .map(|next| next.start_date.map(named) != timers[index].start_date.map(named))
            .unwrap_or(true);
        if ends {
            days.push(Day {
                named: timers[start].start_date.map(named),
                timers: &timers[start..=index],
            });
            start = index + 1;
        }
    }
    days
}

/// True when the Jellyfin server reports this timer conflicted.
fn conflicted(timer: &TimerInfoDto) -> bool {
    timer
        .status
        .as_ref()
        .is_some_and(|status| format!("{status:?}").contains("Conflict"))
}

fn airtime(timer: &TimerInfoDto) -> String {
    let format = |at: Option<chrono::DateTime<chrono::Utc>>| {
        at.map(|at| {
            chrono::DateTime::<chrono::Local>::from(at)
                .format("%H:%M")
                .to_string()
        })
        .unwrap_or_default()
    };
    crate::text::format(
        Text::ProgramAirtime,
        &[&format(timer.start_date), &format(timer.end_date)],
    )
}

fn entry(timer: &TimerInfoDto) -> widget::list::Row<'_> {
    let mut secondary = vec![Cow::from(format!(
        "{} — {}",
        timer.channel_name.clone().unwrap_or_default(),
        airtime(timer)
    ))];
    if conflicted(timer) {
        secondary.push(strings::lookup(Text::ScheduleConflicted).into());
    }

    widget::list::Row {
        face: None,
        index: None,
        title: timer.name.clone().unwrap_or_default().into(),
        secondary,
        press: widget::list::Press::Inert,
        controls: match timer.id.clone() {
            Some(id) => vec![
                button(prose(strings::lookup(Text::ScheduleCancel), typeface::BODY))
                    .style(style::flat)
                    .on_press(Message::LiveTvAction(Action::CancelTimer(id)))
                    .into(),
            ],
            None => Vec::new(),
        },
    }
}

/// One group's heading over its rows, and its rows alone where the group has
/// none.
// reference: schedule-groups
// reference: section-title-cards
fn grouped<'a>(day: Day<'a>) -> Element<'a, Message> {
    let rows = widget::list::listed(ROW, day.timers.iter().map(entry));
    match day.named {
        None => rows,
        Some(named) => column![
            container(prose(named, typeface::HEADING_2))
                .padding(style::padding(space::GROUP_TITLE_PAD)),
            rows,
        ]
        .into(),
    }
}

/// The timers grouped by the day they start on, each group under the heading
/// the reference writes over it and its rows carrying the channel, program,
/// time and status, a timer the Jellyfin server reports conflicted shown as
/// conflicted, and a control that cancels it.
pub fn view<'a>(state: &'a State) -> Element<'a, Message> {
    if state.timers.is_empty() {
        return widget::centered(strings::lookup(Text::ScheduleEmpty).to_string());
    }
    widget::scrolled(column(days(&state.timers).into_iter().map(grouped)))
        .height(Fill)
        .into()
}
