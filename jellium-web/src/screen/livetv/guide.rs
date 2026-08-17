use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use chrono::{DateTime, Utc};
use iced::widget::{Space, button, column, container, row, text};
use iced::{Element, Fill, Subscription};
use uuid::Uuid;

use super::Action;
use crate::api::Api;
use crate::app::Message;
use crate::images::{self, Cache};
use crate::livetv::Program;
use crate::style::Drawn;
use crate::text::{self as strings, Text};
use crate::theme;
use crate::window;

pub use jellium_model::guide::{Fetched, Focus, Move, SPAN, STEP, State, Step, Trouble, half_hour};

/// Loads the TV channels and the guide range, opening at the current half hour
/// with `SPAN` shown.
pub async fn load(api: Rc<Api>, height: Drawn) -> Result<State, crate::error::Bubble> {
    let range = api.guide_range().await.bubbled()?;
    let channels = api
        .channels(jellyfin_api::types::ChannelType::Tv, None)
        .await
        .bubbled()?;
    let start = half_hour(Utc::now()).max(range.start);
    Ok(State {
        channels,
        range,
        start,
        window: window::Window::new(window::Id::Guide, Drawn::of(theme::ROW_HEIGHT), height),
        programs: HashMap::new(),
        held: None,
        focus: Focus {
            channel: 0,
            at: start,
        },
        trouble: None,
    })
}

/// The programs `wanted` covers, in one request.
pub async fn fetch(
    api: Rc<Api>,
    wanted: Fetched,
    channels: Vec<Uuid>,
) -> crate::error::Answer<(Fetched, Vec<Program>)> {
    crate::error::Answer::of(async {
        let programs = api
            .programs(&channels, wanted.span.clone())
            .await
            .bubbled()?;
        Ok((wanted, programs))
    })
    .await
}

fn badge<'a>(label: Text) -> Element<'a, Message> {
    container(text(strings::lookup(label)).size(11))
        .padding(2)
        .into()
}

/// One cell: its title, its airtime, its badges and its record marker.
fn cell<'a>(program: &'a Program, focused: bool) -> Element<'a, Message> {
    let mut marks = row![].spacing(4);
    if program.live {
        marks = marks.push(badge(Text::GuideBadgeLive));
    }
    if program.new {
        marks = marks.push(badge(Text::GuideBadgeNew));
    }
    if program.premiere {
        marks = marks.push(badge(Text::GuideBadgePremiere));
    }
    if program.repeat {
        marks = marks.push(badge(Text::GuideBadgeRepeat));
    }
    if program.series_timer.is_some() {
        marks = marks.push(badge(Text::GuideRecordingSeries));
    } else if program.timer.is_some() {
        marks = marks.push(badge(Text::GuideRecording));
    }

    let minutes = (program.end - program.start).num_minutes().max(1) as f32;
    let body = column![
        text(program.title.clone()).size(14),
        text(crate::livetv::airtime(program)).size(12),
        marks,
    ]
    .spacing(2);

    button(body)
        .style(if focused {
            button::primary
        } else {
            button::secondary
        })
        .width(minutes * theme::GUIDE_MINUTE)
        .height(theme::ROW_HEIGHT)
        .on_press(Message::LiveTvAction(Action::Show(program.id.clone())))
        .into()
}

/// The time axis, ruled every `STEP`, with a marker at `now`.
fn axis<'a>(state: &'a State, now: DateTime<Utc>) -> Element<'a, Message> {
    let steps = (SPAN.num_minutes() / STEP.num_minutes()).max(1);
    let ruled = (0..steps).map(|index| {
        let at = state.start + STEP * index as i32;
        container(
            text(
                chrono::DateTime::<chrono::Local>::from(at)
                    .format("%H:%M")
                    .to_string(),
            )
            .size(12),
        )
        .width(STEP.num_minutes() as f32 * theme::GUIDE_MINUTE)
        .into()
    });

    let marker: Element<'a, Message> = if now >= state.start && now < state.start + SPAN {
        let across = (now - state.start).num_minutes() as f32 * theme::GUIDE_MINUTE;
        row![
            Space::new().width(across),
            container(Space::new())
                .width(theme::GUIDE_MARKER_WIDTH)
                .style(|theme: &iced::Theme| container::Style::default()
                    .background(theme.palette().danger)),
        ]
        .into()
    } else {
        Space::new().into()
    };

    column![
        row![Space::new().width(theme::GUIDE_CHANNEL_WIDTH), row(ruled),],
        row![Space::new().width(theme::GUIDE_CHANNEL_WIDTH), marker],
    ]
    .into()
}

/// The grid: the time axis ruled every `STEP`, a marker at `now`, the date
/// picker and the two screen steps, and one windowed row per channel.
/// A cell carries its title, its start and end, its live, new, premiere and
/// repeat badges, and a record marker that tells a single timer from a series
/// timer.
pub fn view<'a>(state: &'a State, now: DateTime<Utc>, images: &'a Cache) -> Element<'a, Message> {
    let _ = images;

    let controls = row![
        button(text(strings::lookup(Text::GuideEarlier)))
            .on_press(Message::LiveTvAction(Action::Step(Step::Back))),
        text(
            chrono::DateTime::<chrono::Local>::from(state.start)
                .format("%a %d %b")
                .to_string()
        ),
        button(text(strings::lookup(Text::GuideLater)))
            .on_press(Message::LiveTvAction(Action::Step(Step::Forward))),
        button(text(strings::lookup(Text::GuideNow))).on_press(Message::LiveTvAction(
            Action::Date(chrono::Local::now().date_naive())
        )),
    ]
    .spacing(theme::CARD_SPACING)
    .align_y(iced::Center);

    if let Some(Trouble::OutOfRange) = state.trouble {
        return column![
            controls,
            crate::widget::banner(strings::lookup(Text::FailureGuideOutOfRange).to_string()),
        ]
        .spacing(theme::CARD_SPACING)
        .into();
    }

    if state.channels.is_empty() {
        return column![
            controls,
            crate::widget::banner(strings::lookup(Text::GuideEmpty).to_string()),
        ]
        .spacing(theme::CARD_SPACING)
        .into();
    }

    let focus = state.focus;
    let grid = window::list(state.window, state.channels.len(), move |index| {
        let channel = &state.channels[index];
        let cells = state
            .cells(index)
            .into_iter()
            .map(|program| {
                cell(
                    program,
                    index == focus.channel && program.start <= focus.at && focus.at < program.end,
                )
            })
            .collect::<Vec<_>>();
        row![
            container(text(format!("{} {}", channel.number, channel.name)).size(14))
                .width(theme::GUIDE_CHANNEL_WIDTH)
                .height(theme::ROW_HEIGHT),
            row(cells).spacing(2),
        ]
        .height(theme::ROW_HEIGHT)
        .into()
    });

    column![controls, axis(state, now), grid]
        .spacing(theme::CARD_SPACING)
        .width(Fill)
        .height(Fill)
        .into()
}

pub fn images(state: &State) -> HashSet<images::Key> {
    state
        .window
        .shown(state.channels.len())
        .filter_map(|index| state.channels.get(index))
        .map(|channel| images::Key {
            item: channel.id,
            kind: images::Kind::Primary,
            index: None,
            width: theme::IMAGE_WIDTH,
        })
        .collect()
}

/// Left and right move one `STEP`, up and down one channel, and Enter opens
/// the focused cell.
pub fn keys() -> Subscription<Action> {
    use iced::keyboard::{self, Key, key::Named};
    keyboard::listen().with(()).filter_map(|((), event)| {
        let keyboard::Event::KeyPressed { key, .. } = event else {
            return None;
        };
        match key.as_ref() {
            Key::Named(Named::ArrowLeft) => Some(Action::Focus(Move::Earlier)),
            Key::Named(Named::ArrowRight) => Some(Action::Focus(Move::Later)),
            Key::Named(Named::ArrowUp) => Some(Action::Focus(Move::Up)),
            Key::Named(Named::ArrowDown) => Some(Action::Focus(Move::Down)),
            Key::Named(Named::Enter) => Some(Action::Open),
            _ => None,
        }
    })
}
