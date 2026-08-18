use std::rc::Rc;

use iced::widget::{button, checkbox, column, container, row};
use iced::{Element, Fill};
use jellyfin_api::types::{DayOfWeek, DayPattern, KeepUntil, SeriesTimerInfoDto};

use super::{Action, clock};
use crate::api::Api;
use crate::app::Message;
use crate::error::Answer;
use crate::icon::Icon;
use crate::style::space::Room;
use crate::style::{self, Viewport, card, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::{self, Control, Hovered, Poster, prose};
use crate::window;

#[derive(Debug, Clone)]
pub struct State {
    /// Series timers by name.
    pub timers: Vec<SeriesTimerInfoDto>,
    pub grid: window::Grid,
}

/// The series tab's own card, at the shape `shape: 'auto'` falls to for items
/// that report no aspect.
// reference: livetv-series-cards
// reference: card-auto-shape
pub const CARD: card::Drawing = card::Drawing {
    card: card::Card::Wall(card::Shape::Portrait),
    footer: card::Footer::SeriesTimer,
    backing: card::Backing::Padder,
    footing: card::Footing::Bare,
    setting: card::Setting::Centred,
    bottom: card::Bottom::Padded,
};

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

pub async fn load(api: Rc<Api>, room: Room) -> Answer<State> {
    Answer::of(async {
        Ok(State {
            timers: api.series_timers().await.bubbled()?,
            grid: window::Grid::new(
                window::Id::Series,
                CARD.card.width(room),
                CARD.row(room),
                room,
            ),
        })
    })
    .await
}

/// One series timer's card: its name over the time it records at and the
/// channel it records from. The reference keys its image by the timer's own
/// string id, which no image the cache holds is keyed by, so the card draws
/// none.
// reference: livetv-series-cards
fn entry<'a>(timer: &'a SeriesTimerInfoDto, room: Room) -> Element<'a, Message> {
    let name = timer.name.clone().unwrap_or_default();
    let at = match timer.record_any_time {
        Some(true) => strings::lookup(Text::SeriesTimerAnytime).to_string(),
        _ => timer.start_date.map(clock).unwrap_or_default(),
    };
    let channel = match timer.record_any_channel {
        Some(true) => strings::lookup(Text::SeriesTimerAllChannels).to_string(),
        _ => match timer.channel_name.clone() {
            Some(named) => named,
            None => strings::lookup(Text::SeriesTimerOneChannel).to_string(),
        },
    };
    widget::card(
        CARD,
        room,
        Poster {
            face: None,
            name: name.clone(),
            logo: None,
            timer: Some(crate::livetv::Recording::Series),
            elapsed: None,
            press: None,
            hovered: Hovered {
                plays: None,
                controls: match timer.id.clone() {
                    Some(id) => vec![
                        Control {
                            glyph: Icon::Edit,
                            label: Text::SeriesEdit,
                            press: Message::LiveTvAction(Action::EditSeries(id.clone())),
                        },
                        Control {
                            glyph: Icon::Delete,
                            label: Text::SeriesCancel,
                            press: Message::LiveTvAction(Action::CancelSeriesTimer(id)),
                        },
                    ],
                    None => Vec::new(),
                },
            },
        },
        move |line| match line {
            card::Line::Name => name.clone(),
            card::Line::SeriesTimerTime => at.clone(),
            card::Line::SeriesTimerChannel => channel.clone(),
            _ => String::new(),
        },
    )
}

/// A windowed wall of centred portrait cards, each writing its name, the time
/// it records at and the channel it records from.
pub fn view<'a>(state: &'a State, room: Room) -> Element<'a, Message> {
    if state.timers.is_empty() {
        return widget::centered(strings::lookup(Text::SeriesEmpty).to_string());
    }
    window::grid(
        state.grid,
        card::Wrap::Centred,
        state.timers.len(),
        move |index| entry(&state.timers[index], room),
    )
}

/// A number the user edits as text, kept as the number the server carries.
fn number<'a>(
    label: Text,
    value: i32,
    edited: impl Fn(i32) -> Field + 'a,
    viewport: Viewport,
) -> Element<'a, Message> {
    row![
        container(prose(strings::lookup(label), typeface::BODY))
            .width(style::drawn(space::guide_channel(viewport))),
        iced::widget::text_input("", &value.to_string())
            .style(style::input)
            .on_input(move |typed| {
                let read = match crate::failure::unraised::read::<i32>(typed.trim()) {
                    Ok(read) => read,
                    Err(_) => value,
                };
                Message::LiveTvAction(Action::Edited(edited(read)))
            }),
    ]
    .spacing(style::drawn(space::CONTROL_GAP.drawn()))
    .align_y(iced::Center)
    .into()
}

fn switch<'a>(label: Text, value: bool, edited: fn(bool) -> Field) -> Element<'a, Message> {
    row![
        checkbox(value)
            .on_toggle(move |value| Message::LiveTvAction(Action::Edited(edited(value)))),
        prose(strings::lookup(label), typeface::BODY),
    ]
    .spacing(style::drawn(space::CONTROL_GAP.drawn()))
    .align_y(iced::Center)
    .into()
}

/// The series options: day pattern, days, record-any-channel, record-any-time,
/// record-new-only, skip-episodes-in-library, keep-until, keep-up-to,
/// priority, and pre-roll and post-roll padding.
/// Priority and keep-until are shown as the Jellyfin server states them.
pub fn options<'a>(editing: &'a Editing, viewport: Viewport) -> Element<'a, Message> {
    let held = &editing.options;
    let days = held.days.clone().unwrap_or_default();

    let patterns = row(PATTERNS.iter().map(|pattern| {
        let named = match pattern {
            None => strings::lookup(Text::SeriesDayPatternAny).to_string(),
            Some(pattern) => format!("{pattern:?}"),
        };
        button(prose(named, typeface::BODY))
            .style(if held.day_pattern == *pattern {
                style::submit
            } else {
                style::raised
            })
            .on_press(Message::LiveTvAction(Action::Edited(Field::DayPattern(
                *pattern,
            ))))
            .into()
    }))
    .spacing(style::drawn(space::CONTROL_GAP.drawn()));

    let chosen = row(DAYS.iter().map(|day| {
        let wanted = days.contains(day);
        button(prose(format!("{day:?}"), typeface::BODY))
            .style(if wanted { style::submit } else { style::raised })
            .on_press(Message::LiveTvAction(Action::Edited(Field::Day(
                *day, !wanted,
            ))))
            .into()
    }))
    .spacing(style::drawn(space::CONTROL_GAP.drawn()));

    let keep = row(KEEP.iter().map(|until| {
        button(prose(format!("{until:?}"), typeface::BODY))
            .style(if held.keep_until == Some(*until) {
                style::submit
            } else {
                style::raised
            })
            .on_press(Message::LiveTvAction(Action::Edited(Field::KeepUntil(
                *until,
            ))))
            .into()
    }))
    .spacing(style::drawn(space::CONTROL_GAP.drawn()));

    let confirm = if editing.creating {
        Text::ProgramRecordSeries
    } else {
        Text::SeriesSave
    };

    container(
        column![
            prose(held.name.clone().unwrap_or_default(), typeface::HEADING_2),
            prose(strings::lookup(Text::SeriesDayPattern), typeface::BODY),
            patterns,
            prose(strings::lookup(Text::SeriesDays), typeface::BODY),
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
            prose(strings::lookup(Text::SeriesKeepUntil), typeface::BODY),
            keep,
            number(
                Text::SeriesKeepUpTo,
                held.keep_up_to.unwrap_or(0),
                Field::KeepUpTo,
                viewport
            ),
            number(
                Text::SeriesPriority,
                held.priority.unwrap_or(0),
                Field::Priority,
                viewport
            ),
            number(
                Text::SeriesPrePadding,
                held.pre_padding_seconds.unwrap_or(0),
                Field::PrePaddingSeconds,
                viewport
            ),
            number(
                Text::SeriesPostPadding,
                held.post_padding_seconds.unwrap_or(0),
                Field::PostPaddingSeconds,
                viewport
            ),
            row![
                button(prose(strings::lookup(confirm), typeface::BODY))
                    .style(style::submit)
                    .on_press(Message::LiveTvAction(Action::ConfirmSeries)),
                button(prose(strings::lookup(Text::SeriesClose), typeface::BODY))
                    .style(style::raised)
                    .on_press(Message::LiveTvAction(Action::CloseSeries)),
            ]
            .spacing(style::drawn(space::CONTROL_GAP.drawn())),
        ]
        .spacing(style::drawn(space::FIELD_GAP.drawn())),
    )
    .style(style::over_video)
    .padding(style::padding(space::PAGE_PAD))
    .width(Fill)
    .height(Fill)
    .into()
}
