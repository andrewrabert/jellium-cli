use std::rc::Rc;

use iced::widget::{button, checkbox, column, container, row, text};
use iced::{Element, Fill};
use jellyfin_api::types::{DayOfWeek, DayPattern, KeepUntil, SeriesTimerInfoDto};

use super::Action;
use crate::api::Api;
use crate::app::Message;
use crate::error::Answer;
use crate::text::{self as strings, Text};
use crate::theme;
use crate::widget;
use crate::window;

#[derive(Debug, Clone)]
pub struct State {
    /// Series timers by name.
    pub timers: Vec<SeriesTimerInfoDto>,
    pub window: window::Window,
}

/// The series options being edited, and whether confirming creates or updates.
#[derive(Debug, Clone)]
pub struct Editing {
    pub options: SeriesTimerInfoDto,
    /// True while the options prefill a series timer that does not exist yet.
    pub creating: bool,
}

/// One field of the series options, named as the Jellyfin server carries it.
#[derive(Debug, Clone, PartialEq)]
pub enum Field {
    DayPattern(Option<DayPattern>),
    Day(DayOfWeek, bool),
    RecordAnyChannel(bool),
    RecordAnyTime(bool),
    RecordNewOnly(bool),
    SkipEpisodesInLibrary(bool),
    KeepUntil(KeepUntil),
    KeepUpTo(i32),
    Priority(i32),
    PrePaddingSeconds(i32),
    PostPaddingSeconds(i32),
}

/// Every day a series timer can name, in week order.
const DAYS: [DayOfWeek; 7] = [
    DayOfWeek::Monday,
    DayOfWeek::Tuesday,
    DayOfWeek::Wednesday,
    DayOfWeek::Thursday,
    DayOfWeek::Friday,
    DayOfWeek::Saturday,
    DayOfWeek::Sunday,
];

/// Every day pattern the Jellyfin server offers, and the any-day absence.
const PATTERNS: [Option<DayPattern>; 4] = [
    None,
    Some(DayPattern::Daily),
    Some(DayPattern::Weekdays),
    Some(DayPattern::Weekends),
];

/// Every retention rule the Jellyfin server offers.
const KEEP: [KeepUntil; 4] = [
    KeepUntil::UntilDeleted,
    KeepUntil::UntilSpaceNeeded,
    KeepUntil::UntilWatched,
    KeepUntil::UntilDate,
];

impl Editing {
    /// Applies one edit.
    pub fn edited(&mut self, field: Field) {
        match field {
            Field::DayPattern(pattern) => self.options.day_pattern = pattern,
            Field::Day(day, wanted) => {
                let mut days = self.options.days.clone().unwrap_or_default();
                days.retain(|held| *held != day);
                if wanted {
                    days.push(day);
                }
                days.sort_by_key(|day| DAYS.iter().position(|known| known == day));
                self.options.days = Some(days);
            }
            Field::RecordAnyChannel(value) => self.options.record_any_channel = Some(value),
            Field::RecordAnyTime(value) => self.options.record_any_time = Some(value),
            Field::RecordNewOnly(value) => self.options.record_new_only = Some(value),
            Field::SkipEpisodesInLibrary(value) => {
                self.options.skip_episodes_in_library = Some(value);
            }
            Field::KeepUntil(keep) => self.options.keep_until = Some(keep),
            Field::KeepUpTo(count) => self.options.keep_up_to = Some(count),
            Field::Priority(priority) => self.options.priority = Some(priority),
            Field::PrePaddingSeconds(seconds) => self.options.pre_padding_seconds = Some(seconds),
            Field::PostPaddingSeconds(seconds) => {
                self.options.post_padding_seconds = Some(seconds);
            }
        }
    }
}

pub async fn load(api: Rc<Api>, height: f32) -> Answer<State> {
    Answer::of(async {
        Ok(State {
            timers: api.series_timers().await.bubbled()?,
            window: window::Window::new(window::Id::Series, theme::ROW_HEIGHT, height),
        })
    })
    .await
}

fn entry<'a>(timer: &'a SeriesTimerInfoDto) -> Element<'a, Message> {
    let controls: Element<'a, Message> = match timer.id.clone() {
        Some(id) => row![
            button(text(strings::lookup(Text::SeriesEdit)))
                .on_press(Message::LiveTvAction(Action::EditSeries(id.clone()))),
            button(text(strings::lookup(Text::SeriesCancel)))
                .on_press(Message::LiveTvAction(Action::CancelSeriesTimer(id))),
        ]
        .spacing(theme::CARD_SPACING)
        .into(),
        None => iced::widget::Space::new().into(),
    };

    container(
        row![
            text(timer.name.clone().unwrap_or_default())
                .size(15)
                .width(Fill),
            controls,
        ]
        .spacing(theme::CARD_SPACING)
        .align_y(iced::Center),
    )
    .height(theme::ROW_HEIGHT)
    .into()
}

/// A windowed list of series timers by name, each with a control that opens
/// its options and one that cancels it, and no sort control.
pub fn view<'a>(state: &'a State) -> Element<'a, Message> {
    if state.timers.is_empty() {
        return widget::banner(strings::lookup(Text::SeriesEmpty).to_string());
    }
    window::list(state.window, state.timers.len(), move |index| {
        entry(&state.timers[index])
    })
}

/// A number the user edits as text, kept as the number the server carries.
fn number<'a>(label: Text, value: i32, edited: impl Fn(i32) -> Field + 'a) -> Element<'a, Message> {
    row![
        text(strings::lookup(label)).width(theme::GUIDE_CHANNEL_WIDTH),
        iced::widget::text_input("", &value.to_string()).on_input(move |typed| {
            #[expect(
                clippy::disallowed_methods,
                reason = "a conversion that carries no cause beyond the value itself"
            )]
            let typed = typed.parse().unwrap_or(value);
            Message::LiveTvAction(Action::Edited(edited(typed)))
        }),
    ]
    .spacing(theme::CARD_SPACING)
    .align_y(iced::Center)
    .into()
}

fn switch<'a>(label: Text, value: bool, edited: fn(bool) -> Field) -> Element<'a, Message> {
    row![
        checkbox(value)
            .on_toggle(move |value| Message::LiveTvAction(Action::Edited(edited(value)))),
        text(strings::lookup(label)),
    ]
    .spacing(8)
    .align_y(iced::Center)
    .into()
}

/// The series options: day pattern, days, record-any-channel, record-any-time,
/// record-new-only, skip-episodes-in-library, keep-until, keep-up-to,
/// priority, and pre-roll and post-roll padding.
/// Priority and keep-until are shown as the Jellyfin server states them.
pub fn options<'a>(editing: &'a Editing) -> Element<'a, Message> {
    let held = &editing.options;
    let days = held.days.clone().unwrap_or_default();

    let patterns = row(PATTERNS.iter().map(|pattern| {
        let named = match pattern {
            None => strings::lookup(Text::SeriesDayPatternAny).to_string(),
            Some(pattern) => format!("{pattern:?}"),
        };
        button(text(named))
            .style(if held.day_pattern == *pattern {
                button::primary
            } else {
                button::secondary
            })
            .on_press(Message::LiveTvAction(Action::Edited(Field::DayPattern(
                *pattern,
            ))))
            .into()
    }))
    .spacing(8);

    let chosen = row(DAYS.iter().map(|day| {
        let wanted = days.contains(day);
        button(text(format!("{day:?}")))
            .style(if wanted {
                button::primary
            } else {
                button::secondary
            })
            .on_press(Message::LiveTvAction(Action::Edited(Field::Day(
                *day, !wanted,
            ))))
            .into()
    }))
    .spacing(8);

    let keep = row(KEEP.iter().map(|until| {
        button(text(format!("{until:?}")))
            .style(if held.keep_until == Some(*until) {
                button::primary
            } else {
                button::secondary
            })
            .on_press(Message::LiveTvAction(Action::Edited(Field::KeepUntil(
                *until,
            ))))
            .into()
    }))
    .spacing(8);

    let confirm = if editing.creating {
        Text::ProgramRecordSeries
    } else {
        Text::SeriesSave
    };

    container(
        column![
            text(held.name.clone().unwrap_or_default()).size(22),
            text(strings::lookup(Text::SeriesDayPattern)),
            patterns,
            text(strings::lookup(Text::SeriesDays)),
            chosen,
            switch(
                Text::SeriesAnyChannel,
                held.record_any_channel.unwrap_or(false),
                Field::RecordAnyChannel
            ),
            switch(
                Text::SeriesAnyTime,
                held.record_any_time.unwrap_or(false),
                Field::RecordAnyTime
            ),
            switch(
                Text::SeriesNewOnly,
                held.record_new_only.unwrap_or(false),
                Field::RecordNewOnly
            ),
            switch(
                Text::SeriesSkipInLibrary,
                held.skip_episodes_in_library.unwrap_or(false),
                Field::SkipEpisodesInLibrary
            ),
            text(strings::lookup(Text::SeriesKeepUntil)),
            keep,
            number(
                Text::SeriesKeepUpTo,
                held.keep_up_to.unwrap_or(0),
                Field::KeepUpTo
            ),
            number(
                Text::SeriesPriority,
                held.priority.unwrap_or(0),
                Field::Priority
            ),
            number(
                Text::SeriesPrePadding,
                held.pre_padding_seconds.unwrap_or(0),
                Field::PrePaddingSeconds
            ),
            number(
                Text::SeriesPostPadding,
                held.post_padding_seconds.unwrap_or(0),
                Field::PostPaddingSeconds
            ),
            row![
                button(text(strings::lookup(confirm)))
                    .on_press(Message::LiveTvAction(Action::ConfirmSeries)),
                button(text(strings::lookup(Text::SeriesClose)))
                    .on_press(Message::LiveTvAction(Action::CloseSeries)),
            ]
            .spacing(theme::CARD_SPACING),
        ]
        .spacing(theme::CARD_SPACING),
    )
    .style(theme::over_video)
    .padding(theme::CARD_SPACING)
    .width(Fill)
    .height(Fill)
    .into()
}
