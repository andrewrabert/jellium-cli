use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use chrono::{DateTime, TimeDelta, Utc};
use iced::widget::{Space, button, column, container, image, row};
use iced::{Element, Fill, Subscription};
use uuid::Uuid;

use super::Action;
use crate::api::Api;
use crate::app::Message;
use crate::images::{self, Cache};
use crate::livetv::{Badge, Channel, Marque, Program};
use crate::style::{self, Drawn, Viewport, scheme, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::{line, prose};
use crate::window;

pub use jellium_model::guide::{
    Fetched, Focus, Move, Placed, SPAN, STEP, Standing, State, Step, Trouble, half_hour,
};

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
        window: window::Window::new(window::Id::Guide, space::GUIDE_ROW.drawn(), height),
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

/// The face each badge writes.
// reference: guide-indicator-colors
fn face(badge: Badge) -> scheme::Color {
    match badge {
        Badge::Live => scheme::BADGE_LIVE,
        Badge::Premiere => scheme::BADGE_PREMIERE,
        Badge::New => scheme::BADGE_NEW,
    }
}

/// One `.guideProgramIndicator`: what it reads, on its own face, inside its own
/// padding and radius.
// reference: guide-program-indicator
fn badge<'a>(badge: Badge) -> Element<'a, Message> {
    let label = match badge {
        Badge::Live => Text::GuideBadgeLive,
        Badge::Premiere => Text::GuideBadgePremiere,
        Badge::New => Text::GuideBadgeNew,
    };
    let tint = face(badge);
    container(prose(strings::lookup(label), typeface::GUIDE_BADGE))
        .padding(style::padding(space::GUIDE_BADGE_PAD))
        .style(move |theme: &iced::Theme| style::badge(theme, tint))
        .into()
}

/// The rule the guide draws, as wide or as tall as it is laid.
// reference: scheme-guide-rule
fn rule<'a>(viewport: Viewport) -> iced::widget::Container<'a, Message> {
    container(Space::new())
        .width(style::drawn(space::GUIDE_RULE.drawn(viewport.band())))
        .style(style::guide_rule)
}

/// `.guide-channelHeaderCell`: the channel's number at the leading edge, the
/// primary image the Jellyfin server holds for the channel laid on the trailing
/// one, and the channel's own name in the image's place where it holds none.
/// The number goes on a narrow page.
// reference: guide-channel-header
// reference: guide-channel-header-markup
// reference: guide-channel-image
// reference: guide-channel-number
// reference: guide-channel-name
fn header<'a>(
    channel: &'a Channel,
    logo: Option<iced::widget::image::Handle>,
    viewport: Viewport,
) -> Element<'a, Message> {
    let standing = style::drawn(space::guide_standing(viewport.band()));
    let mut laid = row![].align_y(iced::Center).height(standing);
    if !viewport.matches(space::GUIDE_CHANNEL_NARROW_AT) {
        laid = laid.push(
            container(line(
                channel.number.clone(),
                typeface::BODY,
                typeface::Weight::Regular,
                typeface::LINE_HEIGHT,
            ))
            .max_width(style::drawn(space::guide_number(viewport)))
            .padding(iced::Padding::ZERO.left(style::drawn(space::GUIDE_CHANNEL_INSET.drawn()))),
        );
    }

    let trailing: Element<'a, Message> = match (channel.marque, logo) {
        (Marque::Logo, Some(handle)) => container(
            image(handle)
                .width(style::drawn(space::guide_logo(viewport)))
                .height(style::drawn(space::guide_logo_height())),
        )
        .padding(iced::Padding::ZERO.right(style::drawn(
            space::GUIDE_LOGO_INSET.of(space::guide_channel(viewport)),
        )))
        .into(),
        (Marque::Logo, None) => Space::new().into(),
        (Marque::Name, _) => container(line(
            channel.name.clone(),
            typeface::BODY,
            typeface::Weight::Regular,
            typeface::LINE_HEIGHT,
        ))
        .max_width(style::drawn(space::guide_name(viewport)))
        .padding(iced::Padding::ZERO.right(style::drawn(space::GUIDE_CHANNEL_INSET.drawn())))
        .into(),
    };

    row![
        laid.push(Space::new().width(Fill)).push(trailing),
        rule(viewport).height(standing),
    ]
    .width(style::drawn(space::guide_channel(viewport)))
    .height(standing)
    .into()
}

/// One `.programCell`: the rule down its leading edge, its name with its badge
/// beside it, its episode title beneath, and the timer's glyph after them.
// reference: guide-program-cell
// reference: guide-program-name
// reference: guide-episode-title
fn cell<'a>(program: &'a Program, standing: Standing, viewport: Viewport) -> Element<'a, Message> {
    let mut named = row![line(
        program.title.clone(),
        typeface::BODY,
        typeface::Weight::Regular,
        typeface::LINE_HEIGHT,
    )]
    .align_y(iced::Center);
    if let Some(worn) = program
        .badge()
        .filter(|_| !viewport.matches(space::GUIDE_BADGE_AT))
    {
        named = named
            .push(Space::new().width(style::drawn(space::GUIDE_BADGE_LEADING.drawn())))
            .push(badge(worn))
            .push(Space::new().width(style::drawn(space::GUIDE_BADGE_TRAILING.drawn())));
    }

    let mut body = column![named];
    if let Some(episode) = program.episode_title.as_ref() {
        body = body
            .push(Space::new().height(style::drawn(space::GUIDE_EPISODE_TOP.drawn())))
            .push(line(
                episode.clone(),
                typeface::SECONDARY,
                typeface::Weight::Regular,
                typeface::LINE_HEIGHT,
            ));
    }

    let mut inside = row![
        container(body)
            .padding(style::padding(space::GUIDE_PROGRAM_PAD))
            .width(Fill),
    ]
    .align_y(iced::Center);
    if let Some(recording) = program.recording() {
        inside = inside
            .push(Space::new().width(style::drawn(space::GUIDE_MARK_GAP.drawn())))
            .push(crate::widget::timer(recording, typeface::GUIDE_MARK));
    }

    button(row![rule(viewport).height(Fill), inside])
        .style(move |theme: &iced::Theme, status| style::program_cell(theme, status, standing))
        .padding(iced::Padding::ZERO)
        .width(Fill)
        .height(style::drawn(space::guide_standing(viewport.band())))
        .on_press(Message::LiveTvAction(Action::Show(program.id.clone())))
        .into()
}

/// `.timeslotHeaders`: the corner standing over the channel column, then one
/// `.timeslotHeader` every `STEP`, each as wide as `STEP` runs.
// reference: guide-timeslot
// reference: guide-timeslot-face
// reference: guide-timeslot-height
fn timeslots<'a>(state: &'a State, viewport: Viewport) -> Element<'a, Message> {
    let tall = style::drawn(space::GUIDE_TIMESLOT.drawn());
    let steps = (SPAN.num_minutes() / STEP.num_minutes()).max(1);
    let slots = (0..steps).map(|index| {
        let at = state.start + STEP * index as i32;
        container(line(
            chrono::DateTime::<chrono::Local>::from(at)
                .format("%H:%M")
                .to_string(),
            typeface::GUIDE_TIMESLOT,
            typeface::GUIDE_TIMESLOT_WEIGHT,
            typeface::LINE_HEIGHT,
        ))
        .width(style::drawn(space::guide_across(STEP, viewport)))
        .height(tall)
        .padding(iced::Padding::ZERO.left(style::drawn(space::GUIDE_TIMESLOT_INDENT.drawn())))
        .align_y(iced::Center)
        .into()
    });

    row![
        Space::new()
            .width(style::drawn(space::guide_channel(viewport)))
            .height(tall),
        row(slots),
    ]
    .into()
}

/// The strip of one channel's cells over the span shown, each cell beginning
/// where its programme begins and running as long as the span holds of it.
// reference: guide-cell-span
fn strip<'a>(
    state: &'a State,
    index: usize,
    now: DateTime<Utc>,
    viewport: Viewport,
) -> Element<'a, Message> {
    let mut laid = row![];
    let mut reached = TimeDelta::zero();
    for program in state.cells(index) {
        let Placed { begins, runs } = state.placed(program);
        if begins > reached {
            laid = laid.push(Space::new().width(style::drawn(space::guide_across(
                begins - reached,
                viewport,
            ))));
        }
        laid = laid.push(
            container(cell(program, state.standing(index, program, now), viewport))
                .width(style::drawn(space::guide_across(runs, viewport))),
        );
        reached = begins + runs;
    }
    laid.into()
}

/// The grid: the date controls, the time-slot strip, and one windowed row per
/// channel carrying that channel's header and the cells of the programmes it is
/// showing over the span shown.
pub fn view<'a>(
    state: &'a State,
    now: DateTime<Utc>,
    images: &'a Cache,
    viewport: Viewport,
) -> Element<'a, Message> {
    let controls = row![
        button(prose(strings::lookup(Text::GuideEarlier), typeface::BODY))
            .style(style::raised)
            .on_press(Message::LiveTvAction(Action::Step(Step::Back))),
        prose(
            chrono::DateTime::<chrono::Local>::from(state.start)
                .format("%a %d %b")
                .to_string(),
            typeface::BODY
        ),
        button(prose(strings::lookup(Text::GuideLater), typeface::BODY))
            .style(style::raised)
            .on_press(Message::LiveTvAction(Action::Step(Step::Forward))),
        button(prose(strings::lookup(Text::GuideNow), typeface::BODY))
            .style(style::raised)
            .on_press(Message::LiveTvAction(Action::Date(
                chrono::Local::now().date_naive()
            ))),
    ]
    .spacing(style::drawn(space::CONTROL_GAP.drawn()))
    .align_y(iced::Center);

    if let Some(Trouble::OutOfRange) = state.trouble {
        return column![
            controls,
            crate::widget::centered(strings::lookup(Text::FailureGuideOutOfRange).to_string()),
        ]
        .spacing(style::drawn(space::SECTION_GAP.drawn()))
        .into();
    }

    if state.channels.is_empty() {
        return column![
            controls,
            crate::widget::centered(strings::lookup(Text::GuideEmpty).to_string()),
        ]
        .spacing(style::drawn(space::SECTION_GAP.drawn()))
        .into();
    }

    let grid = window::list(state.window, state.channels.len(), move |index| {
        let channel = &state.channels[index];
        let logo = images.handle(images::Key {
            item: channel.id,
            kind: images::Kind::Primary,
            index: None,
        });
        column![
            row![
                header(channel, logo, viewport),
                strip(state, index, now, viewport),
            ]
            .height(style::drawn(space::guide_standing(viewport.band()))),
            rule(viewport).width(Fill),
        ]
        .height(style::drawn(space::GUIDE_ROW.drawn()))
        .into()
    });

    column![controls, timeslots(state, viewport), grid]
        .spacing(style::drawn(space::SECTION_GAP.drawn()))
        .width(Fill)
        .height(Fill)
        .into()
}

/// The primary image of every channel shown whose header draws one.
// reference: guide-channel-header-markup
pub fn images(state: &State) -> HashSet<images::Key> {
    state
        .window
        .shown(state.channels.len())
        .filter_map(|index| state.channels.get(index))
        .filter(|channel| channel.marque == Marque::Logo)
        .map(|channel| images::Key {
            item: channel.id,
            kind: images::Kind::Primary,
            index: None,
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
