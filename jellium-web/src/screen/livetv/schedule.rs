use std::rc::Rc;

use iced::widget::{button, column, container, row};
use iced::{Element, Fill};
use jellyfin_api::types::TimerInfoDto;

use super::Action;
use crate::api::Api;
use crate::app::Message;
use crate::error::Answer;
use crate::style::{self, Drawn, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget;
use crate::widget::{line, prose};
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
            window: window::Window::new(
                window::Id::Schedule,
                Drawn::of(style::drawn(space::LIST_ROW.drawn())),
                height,
            ),
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
        line(
            timer.name.clone().unwrap_or_default(),
            typeface::BODY,
            typeface::Weight::Regular
        ),
        line(
            format!(
                "{} — {}",
                timer.channel_name.clone().unwrap_or_default(),
                airtime(timer)
            ),
            typeface::SECONDARY,
            typeface::Weight::Regular,
        ),
    ]
    .spacing(style::drawn(space::BLOCK_GAP.drawn()))
    .width(Fill);
    if conflicted(timer) {
        named = named.push(line(
            strings::lookup(Text::ScheduleConflicted),
            typeface::SECONDARY,
            typeface::Weight::Regular,
        ));
    }

    let cancel: Element<'a, Message> = match timer.id.clone() {
        Some(id) => button(prose(strings::lookup(Text::ScheduleCancel), typeface::BODY))
            .on_press(Message::LiveTvAction(Action::CancelTimer(id)))
            .into(),
        None => iced::widget::Space::new().into(),
    };

    container(
        column![
            heading,
            row![named, cancel]
                .spacing(style::drawn(space::GUTTER.drawn()))
                .align_y(iced::Center),
        ]
        .spacing(style::drawn(space::BLOCK_GAP.drawn())),
    )
    .height(style::drawn(space::LIST_ROW.drawn()))
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
