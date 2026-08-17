use std::rc::Rc;

use iced::widget::{button, column, container, row};
use iced::{Element, Fill};
use jellyfin_api::types::TimerInfoDto;

use super::Action;
use crate::api::Api;
use crate::app::Message;
use crate::error::Answer;
use crate::style::Drawn;
use crate::style::typeface;
use crate::text::{self as strings, Text};
use crate::theme;
use crate::widget;
use crate::widget::prose;
use crate::window;

#[derive(Debug, Clone)]
pub struct State {
    /// Upcoming timers ordered by start time.
    pub timers: Vec<TimerInfoDto>,
    pub window: window::Window,
}

pub async fn load(api: Rc<Api>, height: Drawn) -> Answer<State> {
    Answer::of(async {
        Ok(State {
            timers: api.timers().await.bubbled()?,
            window: window::Window::new(window::Id::Schedule, Drawn::of(theme::ROW_HEIGHT), height),
        })
    })
    .await
}

/// The day heading a timer falls under, and nothing when it repeats the day
/// the timer before it carried.
fn day(timers: &[TimerInfoDto], index: usize) -> Option<String> {
    let at = timers.get(index)?.start_date?;
    let named = chrono::DateTime::<chrono::Local>::from(at)
        .format("%A %d %B")
        .to_string();
    let before = index
        .checked_sub(1)
        .and_then(|before| timers.get(before))
        .and_then(|timer| timer.start_date)
        .map(|at| {
            chrono::DateTime::<chrono::Local>::from(at)
                .format("%A %d %B")
                .to_string()
        });
    (before.as_deref() != Some(named.as_str())).then_some(named)
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

fn entry<'a>(timers: &'a [TimerInfoDto], index: usize) -> Element<'a, Message> {
    let timer = &timers[index];
    let heading: Element<'a, Message> = match day(timers, index) {
        Some(named) => prose(
            crate::text::format(Text::ScheduleDay, &[&named]),
            typeface::BODY,
        ),
        None => iced::widget::Space::new().into(),
    };

    let mut named = column![
        prose(timer.name.clone().unwrap_or_default(), typeface::BODY),
        prose(
            format!(
                "{} — {}",
                timer.channel_name.clone().unwrap_or_default(),
                airtime(timer)
            ),
            typeface::SECONDARY
        ),
    ]
    .spacing(2)
    .width(Fill);
    if conflicted(timer) {
        named = named.push(prose(
            strings::lookup(Text::ScheduleConflicted).to_owned(),
            typeface::SECONDARY,
        ));
    }

    let cancel: Element<'a, Message> = match timer.id.clone() {
        Some(id) => button(prose(
            strings::lookup(Text::ScheduleCancel).to_owned(),
            typeface::BODY,
        ))
        .on_press(Message::LiveTvAction(Action::CancelTimer(id)))
        .into(),
        None => iced::widget::Space::new().into(),
    };

    container(
        column![
            heading,
            row![named, cancel]
                .spacing(theme::CARD_SPACING)
                .align_y(iced::Center),
        ]
        .spacing(2),
    )
    .height(theme::ROW_HEIGHT)
    .into()
}

/// A windowed list grouped by day, each row carrying its channel, program,
/// time and status, a timer the Jellyfin server reports conflicted shown as
/// conflicted, and a control that cancels it.
pub fn view<'a>(state: &'a State) -> Element<'a, Message> {
    if state.timers.is_empty() {
        return widget::banner(strings::lookup(Text::ScheduleEmpty).to_string());
    }
    window::list(state.window, state.timers.len(), move |index| {
        entry(&state.timers, index)
    })
}
