use std::collections::HashSet;
use std::rc::Rc;

use iced::widget::{column, container};
use iced::{Element, Fill};
use jellium_protocol::Session;
use jellyfin_api::types::{BaseItemDto, TimerInfoDto};

use super::clock;
use crate::api::Api;
use crate::app::Message;
use crate::error::Answer;
use crate::icon::Icon;
use crate::images::{self, Cache};
use crate::screen::overflow;
use crate::style::card::Aspect;
use crate::style::space::Room;
use crate::style::{self, card, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::{self, Control, Face, Hovered, Overlaid, Poster, prose};

#[derive(Debug, Clone)]
pub struct State {
    /// The recordings the server is writing now.
    pub active: Vec<BaseItemDto>,
    // the aspect those recordings share settles this
    pub drawing: card::Drawing,
    /// Upcoming timers ordered by start time.
    pub timers: Vec<TimerInfoDto>,
}

/// The card the active-recordings section draws.
// reference: livetv-schedule-recordings
// reference: livetv-schedule-active
// reference: card-auto-shape
fn active_card(recordings: &[BaseItemDto]) -> card::Drawing {
    let shared = Aspect::shared(
        recordings
            .iter()
            .filter_map(|item| item.primary_image_aspect_ratio)
            .map(Aspect::of),
    );
    card::Drawing {
        card: card::Card::Wall(card::Shape::fitting(shared, card::Shape::Backdrop)),
        footer: card::Footer::ActiveRecording,
        backing: card::Backing::Padder,
        footing: card::Footing::Bare,
        setting: card::Setting::Centred,
        bottom: card::Bottom::Padded,
        touch: card::Touch::Menu,
    }
}

/// The card a day group draws.
// reference: livetv-timer-cards
pub const TIMER_CARD: card::Drawing = card::Drawing {
    card: card::Card::Wall(card::Shape::Backdrop),
    footer: card::Footer::Timer,
    backing: card::Backing::Paper,
    footing: card::Footing::Padded,
    setting: card::Setting::Leading,
    bottom: card::Bottom::Flush,
    touch: card::Touch::Withheld,
};

pub async fn load(api: Rc<Api>) -> Answer<State> {
    Answer::of(async {
        let active = api.active_recordings().await.bubbled()?;
        Ok(State {
            drawing: active_card(&active),
            active,
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

/// The times a card writes for an airing that names both ends, and nothing
/// where it names neither.
// reference: card-air-time
fn airtime(
    start: Option<chrono::DateTime<chrono::Utc>>,
    end: Option<chrono::DateTime<chrono::Utc>>,
) -> String {
    let Some(start) = start else {
        return String::new();
    };
    match end {
        Some(end) => strings::format(Text::ProgramAirtime, &[&clock(start), &clock(end)]),
        None => clock(start),
    }
}

/// The image the Jellyfin server holds for `item`, asked at the width the card
/// it is drawn on wants.
fn key(id: uuid::Uuid, card: card::Card) -> images::Key {
    images::Key {
        item: id,
        kind: images::Kind::Primary,
        index: None,
        card,
    }
}

/// One active recording's card: its own image over the programme's name, its
/// episode title, the time it runs and the channel it came from.
// reference: livetv-schedule-active
fn active<'a>(
    item: &'a BaseItemDto,
    drawing: card::Drawing,
    room: Room,
    session: &Session,
    now: chrono::DateTime<chrono::Utc>,
    art: Option<iced::widget::image::Handle>,
) -> Element<'a, Message> {
    let name = item.name.clone().unwrap_or_default();
    let offered = overflow::commands(overflow::Subject::Item(item), session, None, now);
    let menu = (!offered.is_empty())
        .then_some(Message::OverflowAction(overflow::Action::Open { offered }));
    widget::card(
        drawing,
        room,
        Poster {
            face: art.map(Face::Image),
            name: name.clone(),
            logo: None,
            timer: crate::livetv::Recording::covering(item),
            elapsed: None,
            press: None,
            hovered: Hovered {
                plays: None,
                controls: menu
                    .clone()
                    .map(|press| Control {
                        glyph: Icon::MoreVert,
                        tint: style::Tint::Plain,
                        label: Text::OverflowOpen,
                        press,
                    })
                    .into_iter()
                    .collect(),
            },
            overlaid: Overlaid { plays: None, menu },
        },
        move |line| match line {
            card::Line::ParentTitle => name.clone(),
            // reference: card-display-name
            card::Line::Name => item.episode_title.clone().unwrap_or_default(),
            card::Line::AirTime => airtime(item.start_date, item.end_date),
            card::Line::ChannelName => item.channel_name.clone().unwrap_or_default(),
            _ => String::new(),
        },
    )
}

/// One scheduled timer's card: the programme's image over its name, its
/// episode title and the time it runs, with the channel's logo at the leading
/// edge of the footer.
// reference: livetv-timer-cards
fn timed<'a>(
    timer: &'a TimerInfoDto,
    room: Room,
    session: &Session,
    now: chrono::DateTime<chrono::Utc>,
    art: Option<iced::widget::image::Handle>,
    logo: Option<iced::widget::image::Handle>,
) -> Element<'a, Message> {
    let program = timer.program_info.as_ref();
    let name = program
        .and_then(|held| held.name.clone())
        .or(timer.name.clone())
        .unwrap_or_default();
    let episode = program.and_then(|held| held.episode_title.clone());
    let times = airtime(timer.start_date, timer.end_date);
    let offered = overflow::commands(overflow::Subject::Timer(timer), session, None, now);
    let menu = (!offered.is_empty())
        .then_some(Message::OverflowAction(overflow::Action::Open { offered }));
    widget::card(
        TIMER_CARD,
        room,
        Poster {
            face: art.map(Face::Image),
            name: name.clone(),
            logo,
            timer: Some(crate::livetv::Recording::scheduled(timer)),
            elapsed: None,
            press: None,
            hovered: Hovered {
                plays: None,
                controls: menu
                    .clone()
                    .map(|press| Control {
                        glyph: Icon::MoreVert,
                        tint: style::Tint::Plain,
                        label: Text::OverflowOpen,
                        press,
                    })
                    .into_iter()
                    .collect(),
            },
            overlaid: Overlaid { plays: None, menu },
        },
        move |line| match line {
            card::Line::ParentTitle => name.clone(),
            // reference: card-display-name
            card::Line::Name => episode.clone().unwrap_or_default(),
            card::Line::AirTime => times.clone(),
            _ => String::new(),
        },
    )
}

/// One group's heading over its wall of cards, and its cards alone where the
/// group has no heading.
// reference: schedule-groups
// reference: section-title-cards
fn grouped<'a>(
    day: Day<'a>,
    room: Room,
    images: &'a Cache,
    session: &'a Session,
    now: chrono::DateTime<chrono::Utc>,
) -> Element<'a, Message> {
    let cards = widget::wall(
        TIMER_CARD.card,
        room,
        card::Wrap::Leading,
        day.timers.iter().map(|timer| {
            timed(
                timer,
                room,
                session,
                now,
                program_art(timer, images),
                logo(timer, images),
            )
        }),
    );
    match day.named {
        None => cards,
        Some(named) => column![
            container(prose(named, typeface::HEADING_2))
                .padding(style::padding(space::GROUP_TITLE_PAD)),
            cards,
        ]
        .into(),
    }
}

/// The programme image a timer's card draws.
fn program_art(timer: &TimerInfoDto, images: &Cache) -> Option<iced::widget::image::Handle> {
    program_key(timer).and_then(|key| images.handle(key))
}

/// The channel logo a timer's card carries in its footer.
// reference: card-footer-logo
fn logo(timer: &TimerInfoDto, images: &Cache) -> Option<iced::widget::image::Handle> {
    timer
        .channel_id
        .and_then(|channel| images.handle(key(channel, TIMER_CARD.card)))
}

fn program_key(timer: &TimerInfoDto) -> Option<images::Key> {
    Some(key(timer.program_info.as_ref()?.id?, TIMER_CARD.card))
}

/// The active recordings under their own title, then the timers grouped by the
/// day they start on, all in one scroll.
// reference: livetv-tab-markup
pub fn view<'a>(
    state: &'a State,
    images: &'a Cache,
    room: Room,
    session: &'a Session,
    now: chrono::DateTime<chrono::Utc>,
) -> Element<'a, Message> {
    if state.active.is_empty() && state.timers.is_empty() {
        return widget::centered(strings::lookup(Text::ScheduleEmpty).to_string());
    }
    let mut page = column![];
    if !state.active.is_empty() {
        page = page.push(widget::section(
            strings::lookup(Text::ScheduleActive),
            widget::wall(
                state.drawing.card,
                room,
                card::Wrap::Centred,
                state.active.iter().map(|item| {
                    active(
                        item,
                        state.drawing,
                        room,
                        session,
                        now,
                        item.id
                            .and_then(|id| images.handle(key(id, state.drawing.card))),
                    )
                }),
            ),
        ));
    }
    for day in days(&state.timers) {
        page = page.push(grouped(day, room, images, session, now));
    }
    widget::scrolled(page).height(Fill).into()
}

/// Every active recording's own image, and every shown timer's programme image
/// and channel logo.
pub fn images(state: &State) -> HashSet<images::Key> {
    let active = state
        .active
        .iter()
        .filter_map(|item| item.id)
        .map(|id| key(id, state.drawing.card));
    let programs = state.timers.iter().filter_map(program_key);
    let logos = state
        .timers
        .iter()
        .filter_map(|timer| timer.channel_id)
        .map(|channel| key(channel, TIMER_CARD.card));
    active.chain(programs).chain(logos).collect()
}
