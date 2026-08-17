use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use chrono::{DateTime, Utc};
use iced::widget::{Space, button, column, container, row};
use iced::{Element, Fill, Subscription};
use uuid::Uuid;

use super::Action;
use crate::api::Api;
use crate::app::Message;
use crate::images::{self, Cache};
use crate::livetv::Program;
use crate::style::{self, Drawn, Viewport, space, typeface};
use crate::text::{self as strings, Text};
use crate::theme;
use crate::widget::prose;
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
        window: window::Window::new(
            window::Id::Guide,
            Drawn::of(style::drawn(space::LIST_ROW.drawn())),
            height,
        ),
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
    container(prose(
        strings::lookup(label).to_owned(),
        typeface::SECONDARY,
    ))
    .padding(style::drawn(space::BLOCK_GAP.drawn()))
    .into()
}

/// One cell: its title, its airtime, its badges and its record marker.
fn cell<'a>(program: &'a Program, focused: bool, viewport: Viewport) -> Element<'a, Message> {
    let mut marks = row![].spacing(style::drawn(space::BLOCK_GAP.drawn()));
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
        prose(program.title.clone(), typeface::BODY),
        prose(crate::livetv::airtime(program), typeface::SECONDARY),
        marks,
    ]
    .spacing(style::drawn(space::BLOCK_GAP.drawn()));

    button(body)
        .style(if focused {
            style::submit
        } else {
            style::raised
        })
        .width(minutes * style::drawn(space::guide_minute(viewport)))
        .height(style::drawn(space::LIST_ROW.drawn()))
        .on_press(Message::LiveTvAction(Action::Show(program.id.clone())))
        .into()
}

/// The time axis, ruled every `STEP`, with a marker at `now`.
fn axis<'a>(state: &'a State, now: DateTime<Utc>, viewport: Viewport) -> Element<'a, Message> {
    let minute = style::drawn(space::guide_minute(viewport));
    let channel = style::drawn(space::guide_channel(viewport));
    let steps = (SPAN.num_minutes() / STEP.num_minutes()).max(1);
    let ruled = (0..steps).map(|index| {
        let at = state.start + STEP * index as i32;
        container(prose(
            chrono::DateTime::<chrono::Local>::from(at)
                .format("%H:%M")
                .to_string(),
            typeface::SECONDARY,
        ))
        .width(STEP.num_minutes() as f32 * minute)
        .into()
    });

    let marker: Element<'a, Message> = if now >= state.start && now < state.start + SPAN {
        let across = (now - state.start).num_minutes() as f32 * minute;
        row![
            Space::new().width(across),
            container(Space::new())
                .width(style::drawn(
                    space::SLIDER_MARKER_WIDTH.drawn(viewport.band())
                ))
                .style(|theme: &iced::Theme| container::Style::default()
                    .background(theme.palette().danger)),
        ]
        .into()
    } else {
        Space::new().into()
    };

    column![
        row![Space::new().width(channel), row(ruled),],
        row![Space::new().width(channel), marker],
    ]
    .into()
}

/// The grid: the time axis ruled every `STEP`, a marker at `now`, the date
/// picker and the two screen steps, and one windowed row per channel.
/// A cell carries its title, its start and end, its live, new, premiere and
/// repeat badges, and a record marker that tells a single timer from a series
/// timer.
pub fn view<'a>(
    state: &'a State,
    now: DateTime<Utc>,
    images: &'a Cache,
    viewport: Viewport,
) -> Element<'a, Message> {
    let _ = images;

    let controls = row![
        button(prose(
            strings::lookup(Text::GuideEarlier).to_owned(),
            typeface::BODY
        ))
        .on_press(Message::LiveTvAction(Action::Step(Step::Back))),
        prose(
            chrono::DateTime::<chrono::Local>::from(state.start)
                .format("%a %d %b")
                .to_string(),
            typeface::BODY
        ),
        button(prose(
            strings::lookup(Text::GuideLater).to_owned(),
            typeface::BODY
        ))
        .on_press(Message::LiveTvAction(Action::Step(Step::Forward))),
        button(prose(
            strings::lookup(Text::GuideNow).to_owned(),
            typeface::BODY
        ))
        .on_press(Message::LiveTvAction(Action::Date(
            chrono::Local::now().date_naive()
        ))),
    ]
    .spacing(style::drawn(space::GUTTER.drawn()))
    .align_y(iced::Center);

    if let Some(Trouble::OutOfRange) = state.trouble {
        return column![
            controls,
            crate::widget::banner(strings::lookup(Text::FailureGuideOutOfRange).to_string()),
        ]
        .spacing(style::drawn(space::GUTTER.drawn()))
        .into();
    }

    if state.channels.is_empty() {
        return column![
            controls,
            crate::widget::banner(strings::lookup(Text::GuideEmpty).to_string()),
        ]
        .spacing(style::drawn(space::GUTTER.drawn()))
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
                    viewport,
                )
            })
            .collect::<Vec<_>>();
        row![
            container(prose(
                format!("{} {}", channel.number, channel.name),
                typeface::BODY
            ))
            .width(style::drawn(space::guide_channel(viewport)))
            .height(style::drawn(space::LIST_ROW.drawn())),
            row(cells).spacing(style::drawn(space::BLOCK_GAP.drawn())),
        ]
        .height(style::drawn(space::LIST_ROW.drawn()))
        .into()
    });

    column![controls, axis(state, now, viewport), grid]
        .spacing(style::drawn(space::GUTTER.drawn()))
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
